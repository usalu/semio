//! 🧯️ Generation3d play app commands command — `flow-tessellate-cancel-resolve`: the acknowledgement
//! half of the kernel release, whose request half is `🔓️flow-eval-release`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::preview_eval;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

pub use crate::preview_eval::FlowTessellateCancelResolve;

/// 🧯️ Folds the geometry extension's `evaluateCancel` / `tessellateCancel` answer. It carries a
/// retirement COUNT and nothing the evaluation needs, so this command exists for two reasons a silent
/// drop could not serve: an [`semio_framework_plugin::ExtensionInvocation`] must name an app-owned
/// response action (an unknown id surfaces as a dispatch fault frame), and the release's own round
/// trip has to leave the window's latch exactly as quiescent as the closed run left it — which is why
/// this is NOT folded through `flowTessellateResolve`, whose settle would count a hop answer.
pub fn handle(payload: &FlowTessellateCancelResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    preview_eval::resolve_tessellate_cancel(payload, session);
    Ok(Emit::default())
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
