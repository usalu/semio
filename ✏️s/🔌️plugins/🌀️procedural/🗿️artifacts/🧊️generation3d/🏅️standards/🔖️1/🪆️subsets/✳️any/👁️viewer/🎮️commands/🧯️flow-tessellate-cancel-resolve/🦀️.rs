//! 🧯️ Generation3d viewer command — `flow-tessellate-cancel-resolve`: the read-only surface's
//! binding of the cancel's own response action.
//!
//! 🔁️ It needs a response action of its own rather than reusing `flowTessellateResolve`, whose fold
//! is "one budgeted tessellate step" and whose settle path may RE-ARM — restarting the very chain
//! the user just stopped. This one arms nothing and publishes nothing.

use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};

pub use crate::preview_eval::FlowTessellateCancelResolve;

pub fn handle(_payload: &FlowTessellateCancelResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    Ok(ViewEmit::default())
}
