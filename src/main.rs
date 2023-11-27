use colors_transform::Color;
use image::{get_rectangle_pos_procentage, Rectangle};
use position::pick_up_from_conveyor;
use robotproject::{
    self,
    cbinding::{self, close_port, read, write},
    protocol::{self, homing, ptp, queue, sensor, FloatCustom, GetPoseR, IntCustom, SuctionCup},
};
use std::{
    cmp,
    ffi::CString,
    fs,
    process::Command,
    thread::{self, Thread},
    time::Duration,
};

mod image;
mod position;

pub fn pickup_cube(fd: i32) {
    let procentage = get_rectangle_pos_procentage();
    pick_up_from_conveyor(fd, procentage);
}

// 3280x2464 pixels
fn main() {
    unsafe {
        //  take_picture();

        // getob("src/tyy.jpg", "yeppers.jpg", 1.5);
        // let s = String::from("HalloWelt!");
        // let cs = CString::new(s).unwrap();
        // let cv: Vec<u8> = cs.into_bytes_with_nul();
        // let mut tmp: Vec<i8> = cv.into_iter().map(|c| c as i8).collect::<_>(); // line 7
        // let _cptr: *mut i8 = tmp.as_mut_ptr();

        // cbinding::bindings::takee_pic(_cptr);

         let fd = cbinding::serial_open();
        // pickup_cube(fd);

        //        move_to_pos_in_grid(fd, 3, 4);
        queue::StopExec::send_immediate_command(fd);
        queue::ClearExec::send_immediate_command(fd);

            // homing::Param::send_queue_command(
            //     fd,
            //     &FloatCustom::new(100.0),
            //     &FloatCustom::new(0.0),
            //     &FloatCustom::new(0.0),
            //     &FloatCustom::new(0.0),
            // );
            // homing::Cmd::send_queue_command(fd, &0);
            pickup_cube(fd);

        //     let pos = GetPoseR::send_immediate_command(fd).unwrap();
        //     let x = pos.x.to_float();
        //     let y = pos.y.to_float();
        //     let z = pos.z.to_float();
        //     let r = pos.r.to_float();

        //      println!("X: {}, Y: {}, Z: {}, R: {}", x, y, z, r);

        // // // // for e in &pos.y.hex_float {
        // // // //     println!("hex: Y: {:#02x}", e);
        // // // // }
        //     thread::sleep(Duration::from_millis(2000));
        //    // let d_d = ptp::Cmd::send_queue_command(fd, &ptp::PTPMode::MovlXYZ,  &FloatCustom::new(100.0), &FloatCustom::new(-170.0), &FloatCustom::new(20.0), &FloatCustom::new(0.0));
        //     let d_d = ptp::Cmd::send_queue_command(fd, &ptp::PTPMode::MovlXYZ,  &FloatCustom::new(25.0), &FloatCustom::new(-200.0), &FloatCustom::new(20.0), &FloatCustom::new(0.0));
         queue::StartExec::send_immediate_command(fd);
        // println!("({},{},{}, {})", x, y, pos.z.to_float(), pos.r.to_float());
        // // 120, -85, -30, 0, first row
        // //x, 120 -> 215
        // //y, -85-> -125,

        // X:X: -3.374155, Y: -100.01023, Z: 21.952965, R: -91.93231
        //X: -3.835488, Y: -180.45583, Z: 21.87461, R: -91.217606
        // DIFF, -0.461333, -80, 0, 0,
        // thread::sleep(Duration::from_millis(2000));
        // ptp::Cmd::send_queue_command(
        //     fd,
        //     &ptp::PTPMode::MovlXYZ,
        //     &FloatCustom::new(-3.374155),
        //     &FloatCustom::new(-100.01023),
        //     &FloatCustom::new(21.952965),
        //     &FloatCustom::new(-91.93231),
        // );

        // ptp::Cmd::send_queue_command(
        //     fd,
        //     &ptp::PTPMode::MovlXYZ,
        //     &FloatCustom::new(-3.374155 - 0.23),
        //     &FloatCustom::new(-100.01023 - 40.0),
        //     &FloatCustom::new(21.952965),
        //     &FloatCustom::new(-91.93231),
        // );
        // queue::StartExec::send_immediate_command(fd);
        // // let new_x = 120.0 + (((215.0 - 120.0) / 4.0) * 1.0);
        // // let new_y = -85.0 + (((-125.0 + 85.0) / 4.0) * 1.0);
        // // homing::Cmd::send_immediate_command(fd, &0);
        // thread::sleep(Duration::from_secs(1));
        // homing::Cmd::send_immediate_command(fd, &0);
        //  thread::sleep(Duration::from_secs(1));
        // ptp::Cmd::send_immediate_command(
        //     fd,
        //     &ptp::PTPMode::MovlXYZ,
        //     &FloatCustom::new(210.0),
        //     &FloatCustom::new(-125.0),
        //     &FloatCustom::new(-30.0),
        //     &FloatCustom::new(0.0),
        // );
        // protocol::ptp::Cmd::send_immediate_command(fd, ptp_mode, x, y, z, r)

        //  sensor::set_infrared_immediate(fd, 1, sensor::Port::GP4);

        //  protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));

        // // thread::sleep(Duration::from_millis(2000));
        //     println!("yeppers");
        //     //thread::sleep(Duration::from_millis(1000));
        //     //protocol::GetPoseR::send_immediate_command(fd);
        //     protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));
        //     sensor::set_infrared_immediate(fd,1, sensor::Port::GP4);
        //     // sensor::get_infrared_state(fd, 0);
        //  //   println!("yeppers2");
        //     loop {
        //         if sensor::get_infrared_state(fd, 0) == 1 {
        //             println!("yeppers2");
        //             break;

        //         }
        //     }

        //     protocol::EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(0));
        //  protocol::EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(0));
        // protocol::EMotor::send_immediate_command(fd, &1, &1, &IntCustom::new(10000));
        // // sensor::get_infrared_state(fd, 0);
        // loop {
        //     if sensor::get_infrared_state(fd, 0) == 1 {
        //         break;
        //     }
        // }

        // protocol::EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(0));
        // protocol::SuctionCup::send_immediate_command(fd, &1, &1);

        // thread::sleep(Duration::from_millis(2000));
        // protocol::SuctionCup::send_immediate_command(fd, &0, &0);

        // protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));

        // thread::sleep(Duration::from_millis(3000));
        // protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(0));

        // queue::StopExec::send_immediate_command(fd);
        // queue::ClearExec::send_immediate_command(fd);
        // let mut last_index = 0;
        // let mut f = FloatCustom::new(0.0);
        // let mut y_add = 10.0;

        // for i in 0..5 {
        //     y_add += 20.0;
        //     last_index = protocol::ptp::Cmd::send_queue_command(
        //         fd,
        //         &protocol::ptp::PTPMode::MovlXYZ,
        //         &FloatCustom::new(175.0),
        //         &FloatCustom::new(0.0),
        //         &FloatCustom::new(0.0),
        //         &pos.r,
        //     )
        //     .unwrap();
        // }
        // let mut pos = protocol::GetPoseR::send_immediate_command(fd).unwrap();

        // queue::ClearExec::send_immediate_command(fd);
        // queue::StopExec::send_immediate_command(fd);

        // protocol::homing::Cmd::send_queue_command(fd, &0);
        // println!("x: {}", &pos.x.to_float());
        //  println!("y: {}", &pos.y.to_float());
        //   println!("z: {}", &pos.z.to_float());
        // protocol::ptp::Cmd::send_queue_command(
        //     fd,
        //     &protocol::ptp::PTPMode::MovlXYZ,
        //     &FloatCustom::new(100.0),
        //     &FloatCustom::new(0.0),
        //     &FloatCustom::new(0.0),
        //     &pos.r,
        // );

        // queue::CurrentIndex::send_immediate_command(fd, current_index)
        // protocol::queue::StartExec::send_immediate_command(fd);

        // queue::StartExec::send_immediate_command(fd);

        // while protocol::queue::CurrentIndex::send_get_command(fd)
        //     .unwrap()
        //     .current_index
        //     >= last_index
        // {}
         cbinding::close_port(fd);
    }
}
