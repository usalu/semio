#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::ToolEvent;
#[allow(unused_imports)]
use crate::editor::bim::commands::canvas_pointer_down::canvas_pointer;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use crate::editor::bim::commands::canvas_pointer_up::*;
use crate::editor::bim::fixture::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_release_carries_the_cancel_flag_and_is_refused_outside_a_retained_route() {
    let payload = CanvasPointerUp { cancelled: true, ctrl: true, ..Default::default() };
    assert!(payload.cancelled && payload.raw().modifiers.ctrl);
    assert!(run(&demo(), |doc, cfg| handle(&payload, doc, cfg, &mut ctx(&[]))).err().is_some());
}
