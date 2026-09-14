//! 🔓️ Generation3d viewer command — `flow-eval-release`: the read-only surface's binding of the kernel
//! release hop in `🧵️preview-eval`. It retires EPHEMERAL runtime work in the geometry extension and
//! touches no store lane, so a viewer owns it exactly as the editor does.

use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};

pub use crate::preview_eval::FlowEvalRelease;

/// 🔓️ The session-free fallback. The served route (`Generation3dViewFlowResolveWork`) emits the
/// release invocations, because `ViewEmit` carries no extension invocation.
pub fn handle(_payload: &FlowEvalRelease, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    Ok(ViewEmit::default())
}
