//! 🧮️ 🧵️ Flow play app commands command — `evaluate`.

use crate::editor::flow::commands::flow_eval_tick::{eval_tick_effect, may_rearm};
use crate::editor::flow::host_from_snapshot;
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::editor::flow::modes::edit::windows::main::FLOW_PLAY_WINDOW_MAIN;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Arm
/// 🧵️ Probes/arms `window_id`'s `flowEvalTick` chain — shared by `FlowCommand::Evaluate`, the
/// `auto-evaluate` extension effect, and `FlowPlayApp::pending_effects`. The probe host is retired,
/// never dropped: its layout `OrderedMap` aborts the guest on a bare drop.
///
/// 🔒️ `FlowEvalSession::sync` answers whether the graph has uncomputed nodes; the session's per-window
/// latch answers whether anything already owes a hop for them. BOTH are required: the probe used to be
/// the only gate, and because `pending_effects` rebuilt a throwaway session per poll the gate was
/// always open — every host refresh minted another hop for the same unchanged snapshot.
pub fn evaluate_result(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &mut FlowEvalSession, window_id: &str, window_kind_id: &str) -> Emit<FlowMutation, NoConfigMutation> {
    let host = host_from_snapshot(snapshot, config, session);
    let pending = session.sync(&host);
    let servable = may_rearm(&host.host_snapshot);
    host.retire_cold();
    if pending {
        session.note_window_tick_outcome(window_id, true);
    }
    if servable && session.arm_owed_window_tick(window_id) {
        Emit { effects: vec![eval_tick_effect(window_id, window_kind_id)], ..Default::default() }
    } else {
        Emit::default()
    }
}
//#endregion 🔖️Arm

//#region 🔖️Evaluate
//#endregion 🔖️Evaluate

//#region 🔖️FlowEvalTick
//#endregion 🔖️FlowEvalTick

//#region 🔖️FlowEvalResolve
//#endregion 🔖️FlowEvalResolve

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct Evaluate {}

pub fn handle(_payload: &Evaluate, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    Ok(evaluate_result(doc.snapshot, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session, FLOW_PLAY_WINDOW_MAIN, FLOW_PLAY_WINDOW_MAIN))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
