//! 🧮️ Generation3d play app commands command — `flow-tessellate-resolve`: the editor's binding of
//! the surface-neutral chain in `🧵️preview-eval`, which owns the payload shape and the fold itself.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::preview_eval;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

pub use crate::preview_eval::FlowTessellateResolve;

/// ✅️ Folds one budgeted `tessellate` round trip into the retained session. A step that neither
/// finished the mesh nor received its last body chunk leaves its window unfinished, so the
/// `previewEval` run schedules the next hop — a resumable tessellation the user can watch and abort.
pub fn handle(payload: &FlowTessellateResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    preview_eval::resolve_tessellate(payload, session);
    Ok(Emit::default())
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
