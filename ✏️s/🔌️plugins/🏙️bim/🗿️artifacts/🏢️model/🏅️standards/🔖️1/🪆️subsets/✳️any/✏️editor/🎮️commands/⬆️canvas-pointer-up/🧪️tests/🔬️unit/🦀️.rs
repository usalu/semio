use super::*;
use crate::editor::bim::unit_tests::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_release_carries_the_cancel_flag_and_is_refused_outside_a_retained_route() {
    let payload = CanvasPointerUp { cancelled: true, ctrl: true, ..Default::default() };
    assert!(payload.cancelled && payload.raw().modifiers.ctrl);
    assert!(run(&demo(), |doc, cfg| handle(&payload, doc, cfg, &mut ctx(&[]))).err().is_some());
}
