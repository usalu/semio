//! 🛑️ Generation3d viewer command — `cancel-preview-eval`: the read-only surface's binding of the
//! surface-neutral cancel gesture in `🧵️preview-eval`, which owns the payload shape, the local
//! retirement and the one `tessellateCancel` invocation.
//!
//! 🔒️ A cancel is a `View` action, not a mutation: it retires EPHEMERAL runtime work (the retained
//! session's tessellation ledger and the geometry extension's own kernel jobs) and touches no store
//! lane at all, so a viewer may own it exactly as the editor does. The published status contract is
//! what tells the shell the affordance exists (`cancelAction`), and a surface that publishes the
//! contract must also declare the verb or `ShellHost`'s `declaredAction` gate drops the click
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};

pub use crate::preview_eval::CancelPreviewEval;

/// 🛑️ The session-free fallback. The real work runs on the retained route
/// (`Generation3dViewFlowCancelWork`), which is the only place holding the app instance's session.
pub fn handle(_payload: &CancelPreviewEval, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    Ok(ViewEmit::default())
}
