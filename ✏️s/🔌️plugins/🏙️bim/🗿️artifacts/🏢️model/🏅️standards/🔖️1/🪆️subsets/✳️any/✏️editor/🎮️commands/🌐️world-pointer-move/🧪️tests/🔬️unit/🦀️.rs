#[allow(unused_imports)]
use crate::editor::bim::gestures::session::ToolEvent;
#[allow(unused_imports)]
use crate::editor::bim::gestures::Raw;
#[allow(unused_imports)]
use crate::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use crate::editor::bim::commands::world_pointer_move::*;
use crate::editor::bim::fixture::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_move_without_a_ground_point_does_nothing() {
    let emit = run(&demo(), |doc, cfg| handle(&WorldPointerMove { pane: String::new(), position: Vec::new() }, doc, cfg, &mut ctx(&[]))).expect("nothing to do");
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty());
}
