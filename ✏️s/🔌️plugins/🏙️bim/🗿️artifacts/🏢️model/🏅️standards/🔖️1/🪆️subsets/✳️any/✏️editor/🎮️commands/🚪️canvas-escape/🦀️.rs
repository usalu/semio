//! 🚪️ `canvasEscape` (Escape): cancels the gesture in progress with zero trace: the chain, polygon, drag or marquee is dropped and the line typed into the window's entry field is emptied;
//! what was already written stays written.

use crate::editor::bim::gestures::{keep_entry, run_keyed};
use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::ToolEvent;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "canvas-escape")]
pub struct CanvasEscape {}

pub fn handle(_payload: &CanvasEscape, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let emit = run_keyed(ctx, doc, ToolEvent::Escape)?;
    keep_entry(ctx, "");
    Ok(emit)
}


