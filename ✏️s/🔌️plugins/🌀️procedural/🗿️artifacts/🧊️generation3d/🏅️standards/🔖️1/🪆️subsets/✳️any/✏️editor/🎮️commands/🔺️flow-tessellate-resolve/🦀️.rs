//! 🧮️ Generation3d play app commands command — `flow-tessellate-resolve`: the editor's binding of
//! the surface-neutral chain in `🧵️preview-eval`, which owns the payload shape and the fold itself.
//!
//! 🔁️ A mesh body arrives one chunk per round trip, and each chunk used to cost a `flowEvalTick`
//! hop of its own to ask for the next one. This route is window-addressed, so it continues the
//! chain inline through [`flow_eval_tick::continue_inline`] on exactly the terms the `evaluate`
//! fold does.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::editor::generation3d::commands::flow_eval_tick;
use crate::preview_eval;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::{FlowEvalPublication, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

pub use crate::preview_eval::FlowTessellateResolve;

/// ✅️ One budgeted `tessellate` round trip folded into the retained session, and the hop it left
/// owed run inline when the turn still admits one. A step that neither finished the mesh nor
/// received its last body chunk leaves its window unfinished — a resumable tessellation the user can
/// watch and abort, now watched a whole round trip earlier per chunk.
pub fn resolve(
    payload: &FlowTessellateResolve,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
    turn_started_us: Option<u64>,
) -> Result<(Emit<Generation3dMutation, Generation3dConfigMutation>, FlowEvalPublication), Fault> {
    preview_eval::resolve_tessellate(payload, session);
    flow_eval_tick::continue_inline(&payload.window_id, &payload.window_kind_id, doc, cfg, session, retained_eval, turn_started_us)
}

pub fn handle(payload: &FlowTessellateResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    preview_eval::resolve_tessellate(payload, session);
    Ok(Emit::default())
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
