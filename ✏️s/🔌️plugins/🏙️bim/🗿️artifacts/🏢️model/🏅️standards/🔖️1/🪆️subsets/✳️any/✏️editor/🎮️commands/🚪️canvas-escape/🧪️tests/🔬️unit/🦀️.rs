#[allow(unused_imports)]
use semio_s_artifact_bim_model_gestures::editor::bim::gestures::{keep_entry, run_keyed};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::ToolEvent;
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
use semio_s_artifact_bim_model_gestures::editor::bim::commands::canvas_escape::{handle, CanvasEscape};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn escape_cancels_only_through_the_retained_route() {
    let fault = run(&demo(), |doc, cfg| handle(&CanvasEscape {}, doc, cfg, &mut ctx(&[]))).err().expect("no session owner");
    assert!(format!("{:?}", fault.code).contains(semio_s_artifact_bim_model_gestures::editor::bim::gestures::RETAINED_ROUTE));
}
