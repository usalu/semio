//! ✅️ `canvasCommitDraft` (Enter): finishes the gesture in progress without a further point: a wall chain ends, a railing, slab or roof polygon is written.

use crate::editor::bim::gestures::run_keyed;
use crate::editor::bim::gestures::session::ToolEvent;
use crate::editor::bim::dispatch_context::BimDispatchCtx;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "canvas-commit-draft")]
pub struct CanvasCommitDraft {}

pub fn handle(_payload: &CanvasCommitDraft, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    run_keyed(ctx, doc, ToolEvent::Finish)
}


