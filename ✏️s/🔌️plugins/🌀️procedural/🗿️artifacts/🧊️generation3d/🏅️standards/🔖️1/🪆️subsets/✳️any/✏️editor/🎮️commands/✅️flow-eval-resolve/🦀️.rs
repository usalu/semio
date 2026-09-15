//! 🧮️ Generation3d play app commands command — `flow-eval-resolve`: the editor's binding of the
//! surface-neutral chain in `🧵️preview-eval`, which owns the payload shape and the fold itself.
//!
//! 🔁️ The fold is also where the chain CONTINUES. An `evaluate` answer is window-addressed, so once
//! the last answer of a wave lands this route holds the document, the config, the retained session
//! and the addressed preview window's transient — everything the next wave needs — and runs it
//! inline through [`flow_eval_tick::continue_inline`] instead of paying a whole
//! `flowEvalResolve` → `flowEvalTick` round-trip pair for it.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::editor::generation3d::commands::flow_eval_tick;
use crate::preview_eval;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::{FlowEvalPublication, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

pub use crate::preview_eval::FlowEvalResolve;

/// ✅️ One `evaluate` answer folded into the retained session, and the wave it unblocked run inline
/// when the turn still admits one. Answers what the calling surface owes the addressed window's
/// retained publication, exactly as the dispatched hop does.
pub fn resolve(
    payload: &FlowEvalResolve,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
    turn_started_us: Option<u64>,
) -> Result<(Emit<Generation3dMutation, Generation3dConfigMutation>, FlowEvalPublication), Fault> {
    preview_eval::resolve_eval(payload, session);
    flow_eval_tick::continue_inline(&payload.window_id, &payload.window_kind_id, doc, cfg, session, retained_eval, turn_started_us)
}

pub fn handle(payload: &FlowEvalResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    preview_eval::resolve_eval(payload, session);
    Ok(Emit { ui_scope: flow_eval_tick::chain_ui_scope(&payload.window_kind_id, true), ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️Budget
/// ⏱️ The BUDGET law of the `evaluate` capability — its own module because the law is about the
/// ENVELOPE and its continuation, not about the one finished answer `🔬️unit` pins
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(test)]
#[path = "🧪️tests/🔬️budget/🦀️.rs"]
mod budget;
//#endregion 🧪️Budget
