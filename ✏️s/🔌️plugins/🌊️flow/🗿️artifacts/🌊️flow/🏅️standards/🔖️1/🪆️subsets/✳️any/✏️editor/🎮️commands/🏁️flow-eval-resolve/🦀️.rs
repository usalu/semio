//! 🧮️ 🧵️ Flow play app commands command — `flow-eval-resolve`.

use crate::editor::flow::commands::flow_eval_tick::eval_tick_effect;
use crate::editor::flow::modes::edit::windows::main::FLOW_PLAY_WINDOW_MAIN;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Arm
// 🧵️ The arm probe is owned by `commands::evaluate::evaluate_result`; the hop effect by
// `commands::flow_eval_tick::eval_tick_effect`.
//#endregion 🔖️Arm

//#region 🔖️Evaluate
//#endregion 🔖️Evaluate

//#region 🔖️FlowEvalTick
//#endregion 🔖️FlowEvalTick

//#region 🔖️FlowEvalResolve
//#endregion 🔖️FlowEvalResolve

/// ✅️ One `evaluate` answer, echoed back onto the response action by
/// `reactor::extension_response_args` together with the window address the request carried.
///
/// 📏️ THREE fields is the whole budget: this route's wire witness is the fixed
/// `store::os_pack::ScalarRecordView` (`🎒️pack/🔎️scalar-witness`), which holds three slots and at most
/// two of them text. So the answer carries the window it belongs to, the node it answers and the
/// answer — and NOT generation2d's `extension_id`/`ok`/`fault_*`, which its own non-scalar route can
/// afford. A faulted answer is still bounded here: it folds as an envelope the session cannot seed,
/// which abandons the window rather than re-parking the identical request.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct FlowEvalResolve {
    pub window_id: String,
    pub node_hash: u64,
    pub output_json: String,
}

/// ✅️ Folds one answer into `window_id`'s latch and owes the chain exactly one continuation: the
/// LAST answer of a fan-out re-arms, the earlier ones emit nothing, and an answer that cannot fold
/// gives the window up instead of asking for the same request again.
pub fn handle(payload: &FlowEvalResolve, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    if payload.window_id.is_empty() {
        return Err(Fault::from("flow-eval-resolve-window-required"));
    }
    let given_up = match session.resolve_preview_eval(payload.node_hash, &payload.output_json) {
        flow::PreviewEvalOutcome::Complete { output_json } => session.seed_node_cache(payload.node_hash, &output_json).is_err(),
        flow::PreviewEvalOutcome::Cancelled => true,
        flow::PreviewEvalOutcome::Working => false,
    };
    if given_up {
        session.abandon_window_tick(&payload.window_id);
    }
    let armed = session.settle_window_extension(&payload.window_id) || session.arm_owed_window_tick(&payload.window_id);
    let effects = if armed { vec![eval_tick_effect(&payload.window_id, FLOW_PLAY_WINDOW_MAIN)] } else { Vec::new() };
    Ok(Emit { effects, ..Default::default() })
}
