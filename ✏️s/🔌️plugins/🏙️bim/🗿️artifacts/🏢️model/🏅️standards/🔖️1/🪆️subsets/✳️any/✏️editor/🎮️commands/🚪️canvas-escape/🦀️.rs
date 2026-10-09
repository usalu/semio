//! 🚪️ `canvasEscape` (Escape): cancels the gesture in progress with zero trace: the chain, polygon, drag or marquee is dropped and the line typed into the window's entry field is emptied;
//! what was already written stays written.

use crate::editor::bim::gestures::{keep_entry, run_keyed};
use crate::editor::bim::gestures::session::ToolEvent;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "canvas-escape")]
pub struct CanvasEscape {}

pub fn handle(_payload: &CanvasEscape, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let emit = run_keyed(ctx, doc, ToolEvent::Escape)?;
    keep_entry(ctx, "");
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
