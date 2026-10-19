#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::ToolEvent;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_gestures::editor::bim::gestures::Raw;
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
use semio_s_artifact_bim_model_gestures::editor::bim::commands::world_pointer_move::{handle, WorldPointerMove};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_move_without_a_ground_point_does_nothing() {
    let emit = run(&demo(), |doc, cfg| handle(&WorldPointerMove { pane: String::new(), position: Vec::new() }, doc, cfg, &mut ctx(&[]))).expect("nothing to do");
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty());
}
