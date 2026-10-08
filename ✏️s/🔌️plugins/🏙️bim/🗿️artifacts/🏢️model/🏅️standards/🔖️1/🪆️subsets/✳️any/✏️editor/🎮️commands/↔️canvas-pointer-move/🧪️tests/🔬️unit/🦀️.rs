use super::*;
use crate::editor::bim::unit_tests::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_move_outside_a_retained_route_is_refused_and_the_payload_reads_pixels_and_modifiers() {
    let payload = CanvasPointerMove { x: 440.0, y: 260.0, width: 800.0, height: 600.0, shift: true, ..Default::default() };
    let raw = payload.raw();
    assert_eq!((raw.x, raw.y, raw.width, raw.height, raw.modifiers.shift, raw.modifiers.ctrl, raw.ground), (440.0, 260.0, 800.0, 600.0, true, false, None));
    let fault = run(&demo(), |doc, cfg| handle(&payload, doc, cfg, &mut ctx(&[]))).err().expect("no session owner");
    assert!(format!("{:?}", fault.code).contains(crate::editor::bim::gestures::RETAINED_ROUTE));
}
