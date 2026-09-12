//! 🛑️ Generation3d play app commands command — `cancel-preview-eval`: the editor's binding of the
//! surface-neutral gesture in `🧵️preview-eval`, which owns the payload shape and the cancel law.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::preview_eval;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

pub use crate::preview_eval::CancelPreviewEval;

/// 🛑️ Retires every in-flight kernel tessellation, forgets every pending round trip, quiesces the
/// addressed window's arming latch and tells the geometry extension actor to drop the jobs it
/// retains. Emits no mutation — the next render reads `phase: "cancelled"` off the status object,
/// and any later gesture re-arms the tick chain from a clean state.
pub fn handle(payload: &CancelPreviewEval, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit { extension_invocations: preview_eval::cancel_preview_eval(payload, session), ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
