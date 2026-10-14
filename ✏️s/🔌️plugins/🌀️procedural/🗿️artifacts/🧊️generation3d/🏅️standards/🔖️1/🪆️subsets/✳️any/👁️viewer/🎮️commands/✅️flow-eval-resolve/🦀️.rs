//! ✅️ Generation3d viewer command — `flow-eval-resolve`: the read-only surface's binding of the
//! surface-neutral chain in `🧵️preview-eval`, which owns the payload shape and the fold itself.
//!
//! 🔒️ The fold seeds the retained evaluation session's node cache — runtime state, not a store
//! lane — so this route's publication contract is `HostOnly` and the `ViewEmit` fallback below is
//! empty (it is handed no session).

use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};



pub fn handle(_payload: &FlowEvalResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    Ok(ViewEmit::default())
}
