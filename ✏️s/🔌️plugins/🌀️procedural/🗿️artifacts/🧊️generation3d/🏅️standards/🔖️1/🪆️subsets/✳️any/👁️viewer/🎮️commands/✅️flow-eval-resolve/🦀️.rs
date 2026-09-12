//! ✅️ Generation3d viewer command — `flow-eval-resolve`: the read-only surface's binding of the
//! surface-neutral chain in `🧵️preview-eval`, which owns the payload shape and the fold itself.
//!
//! 🔒️ The fold seeds the retained evaluation session's node cache — runtime state, not a store
//! lane — so this route's publication contract is `HostOnly` and the `ViewEmit` fallback below is
//! empty (it is handed no session).

use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};

pub use crate::preview_eval::FlowEvalResolve;

pub fn handle(_payload: &FlowEvalResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    Ok(ViewEmit::default())
}
