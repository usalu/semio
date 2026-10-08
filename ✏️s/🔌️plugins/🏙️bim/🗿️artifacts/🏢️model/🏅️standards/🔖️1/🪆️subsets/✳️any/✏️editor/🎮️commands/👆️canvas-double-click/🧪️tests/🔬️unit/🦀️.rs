use super::*;
use crate::editor::bim::unit_tests::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_double_click_is_a_gesture_command_and_needs_its_retained_route() {
    let payload = CanvasDoubleClick { x: 1.0, y: 2.0, width: 3.0, height: 4.0, ..Default::default() };
    assert_eq!((payload.raw().x, payload.raw().height), (1.0, 4.0));
    assert!(run(&demo(), |doc, cfg| handle(&payload, doc, cfg, &mut ctx(&[]))).err().is_some());
}
