#[allow(unused_imports)]
use semio_s_artifact_bim_model_gestures::editor::bim::gestures::{keep_entry, run_typed};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use semio_s_artifact_bim_model_gestures::editor::bim::commands::engagement_submit::{handle, EngagementSubmit};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_submit_outside_the_retained_route_is_refused_because_no_gesture_can_hear_it() {
    let mut ctx = ctx(&[]);
    let result = run(&demo(), |doc, cfg| handle(&EngagementSubmit { value: "3, 4".into() }, doc, cfg, &mut ctx));
    assert_eq!(result.err().map(|fault| fault.code.0), Some(semio_s_artifact_bim_model_gestures::editor::bim::gestures::RETAINED_ROUTE.to_string()));
}
