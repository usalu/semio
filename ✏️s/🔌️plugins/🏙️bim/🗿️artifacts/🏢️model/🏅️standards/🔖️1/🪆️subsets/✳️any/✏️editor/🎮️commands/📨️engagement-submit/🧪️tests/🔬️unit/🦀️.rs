use super::*;
use crate::editor::bim::unit_tests::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_submit_outside_the_retained_route_is_refused_because_no_gesture_can_hear_it() {
    let mut ctx = ctx(&[]);
    let result = run(&demo(), |doc, cfg| handle(&EngagementSubmit { value: "3, 4".into() }, doc, cfg, &mut ctx));
    assert_eq!(result.err().map(|fault| fault.code.0), Some(crate::editor::bim::gestures::RETAINED_ROUTE.to_string()));
}
