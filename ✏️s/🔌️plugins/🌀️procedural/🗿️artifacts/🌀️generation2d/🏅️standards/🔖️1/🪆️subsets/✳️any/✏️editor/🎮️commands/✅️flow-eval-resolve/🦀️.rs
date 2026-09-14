//! 🧮️ Generation2d play app commands command — `flow-eval-resolve`: the editor's binding of the
//! `previewEval` run's answer fold in `🧵️preview-eval`, which owns the payload shape and the fold itself.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::preview_eval;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

pub use crate::preview_eval::FlowEvalResolve;

/// ✅️ Folds the answer into the session it is handed; it arms nothing, because the run job schedules
/// the next hop off the window's latch.
pub fn handle(payload: &FlowEvalResolve, _doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    preview_eval::resolve_eval(payload, session);
    Ok(Emit::default())
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
