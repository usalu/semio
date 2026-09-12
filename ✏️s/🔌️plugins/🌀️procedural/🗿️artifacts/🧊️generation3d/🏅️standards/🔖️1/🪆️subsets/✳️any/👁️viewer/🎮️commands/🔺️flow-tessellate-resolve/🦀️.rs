//! 🔺️ Generation3d viewer command — `flow-tessellate-resolve`: the read-only surface's binding of
//! the surface-neutral chain in `🧵️preview-eval`, which owns the payload shape and the fold itself.
//!
//! 🔒️ Same standing as `✅️flow-eval-resolve`: a resumable tessellation folds into the retained
//! session and re-arms the tick, touching no store lane at all.

use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};

pub use crate::preview_eval::FlowTessellateResolve;

pub fn handle(_payload: &FlowTessellateResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    Ok(ViewEmit::default())
}
