use super::*;
use crate::editor::bim::unit_tests::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn escape_cancels_only_through_the_retained_route() {
    let fault = run(&demo(), |doc, cfg| handle(&CanvasEscape {}, doc, cfg, &mut ctx(&[]))).err().expect("no session owner");
    assert!(format!("{:?}", fault.code).contains(crate::editor::bim::gestures::RETAINED_ROUTE));
}
