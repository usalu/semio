//! 🛑️ Generation3d play app commands command — `cancel-preview-eval`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🛑️ The user's explicit "stop computing this preview" gesture. Carries no argument: the retained
/// session owns every in-flight evaluation and tessellation, so the cancel is unambiguous.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "cancel-preview-eval")]
pub struct CancelPreviewEval {}

/// 🛑️ Retires every in-flight kernel tessellation job and forgets every pending round trip. Emits
/// no effect — the next render reads `phase: "cancelled"` off the status object, and any later edit
/// re-arms the tick chain from scratch.
pub fn handle(_payload: &CancelPreviewEval, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let _ = session.cancel_preview_evaluation();
    Ok(Emit::default())
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
