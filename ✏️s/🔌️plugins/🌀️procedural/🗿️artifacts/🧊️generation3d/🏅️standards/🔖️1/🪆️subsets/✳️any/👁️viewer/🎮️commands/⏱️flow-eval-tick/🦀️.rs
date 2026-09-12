//! ⏱️ Generation3d viewer command — `flow-eval-tick`: the read-only surface's binding of the
//! surface-neutral chain in `🧵️preview-eval`, which owns the payload shape, the addressing law and
//! the tick core.
//!
//! 🔒️ Read-only by construction: the tick publishes ONLY into the addressed preview window's own
//! ephemeral transient, never onto the document or draft lane — which is why the served route
//! (`Generation3dViewFlowEvalWindowWork`) builds its own `Emit`, and the `ViewEmit` `handle` below
//! is the session-free fallback with no window to publish into and no retained session to advance.

use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};

pub use crate::preview_eval::FlowEvalTick;

pub fn handle(_payload: &FlowEvalTick, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    Ok(ViewEmit::default())
}
