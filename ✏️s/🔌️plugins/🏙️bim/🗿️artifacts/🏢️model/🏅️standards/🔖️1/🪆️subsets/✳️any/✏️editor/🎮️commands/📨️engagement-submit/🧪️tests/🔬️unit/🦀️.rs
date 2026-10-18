#[allow(unused_imports)]
use crate::editor::bim::gestures::{keep_entry, run_typed};
#[allow(unused_imports)]
use crate::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use crate::editor::bim::commands::engagement_submit::*;
use crate::editor::bim::fixture::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_submit_outside_the_retained_route_is_refused_because_no_gesture_can_hear_it() {
    let mut ctx = ctx(&[]);
    let result = run(&demo(), |doc, cfg| handle(&EngagementSubmit { value: "3, 4".into() }, doc, cfg, &mut ctx));
    assert_eq!(result.err().map(|fault| fault.code.0), Some(crate::editor::bim::gestures::RETAINED_ROUTE.to_string()));
}
