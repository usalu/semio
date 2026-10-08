use super::*;
use crate::editor::bim::unit_tests::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn enter_finishes_only_through_the_retained_route() {
    let fault = run(&demo(), |doc, cfg| handle(&CanvasCommitDraft {}, doc, cfg, &mut ctx(&[]))).err().expect("no session owner");
    assert!(format!("{:?}", fault.code).contains(crate::editor::bim::gestures::RETAINED_ROUTE));
}
