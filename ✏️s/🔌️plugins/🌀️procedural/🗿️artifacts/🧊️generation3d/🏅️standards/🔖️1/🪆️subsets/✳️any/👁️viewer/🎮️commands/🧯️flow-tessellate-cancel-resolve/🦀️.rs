//! 🧯️ Generation3d viewer command — `flow-tessellate-cancel-resolve`: the read-only surface's
//! binding of the cancel's own response action.
//!
//! 🔁️ It needs a response action of its own rather than reusing `flowTessellateResolve`, whose fold
//! is "one budgeted tessellate step" and whose settle path may RE-ARM — restarting the very chain
//! the user just stopped. This one arms nothing and publishes nothing.

use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};



pub fn handle(_payload: &FlowTessellateCancelResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    Ok(ViewEmit::default())
}
