//! 🧮️ 🧵️ Flow play app commands command — `flow-eval-resolve`.

use crate::editor::flow::commands::flow_eval_tick::eval_tick_effect;
use crate::editor::flow::modes::edit::windows::main::FLOW_PLAY_WINDOW_MAIN;
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, ExtensionInvocation, Fault};
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
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[derive(semio_framework_value::RetireOwned)]
pub struct FlowEvalResolve {
    pub window_id: String,
    pub node_hash: u64,
    pub output_json: Option<String>,
}

/// ✅️ Folds one answer into `window_id`'s latch and owes the chain exactly one continuation: the
/// LAST answer of a fan-out re-arms, the earlier ones emit nothing, and an answer that cannot fold
/// gives the window up instead of asking for the same request again.
pub fn handle(payload: &FlowEvalResolve, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    if payload.window_id.is_empty() {
        return Err(Fault::from("flow-eval-resolve-window-required"));
    }
    let Some(output_json)=payload.output_json.as_deref()else{return Ok(Emit::default())};
    let given_up = match session.resolve_preview_eval_cold(payload.node_hash, output_json).map_err(|error|semio_framework_diagnostic::FaultFrom::into_fault(error))? {
        flow::PreviewEvalOutcome::Complete { output_json } => {
            if session.invocation_origin_cold(&payload.window_id,payload.node_hash).is_some_and(|origin|origin.outer_node_hash.is_some()){
                session.retain_invocation_reply_cold(&payload.window_id,payload.node_hash,output_json).map_err(|(error,source)|{flow::host::retire_invocation_reply_cold(source);semio_framework_diagnostic::FaultFrom::into_fault(error)})?;
                return Ok(Emit::default());
            }
            flow::host::io::evaluation_response::decode_flow_node_output_json(&output_json).map(|output|session.seed_node_cache(payload.node_hash,output)).is_err()
        },
        flow::PreviewEvalOutcome::Pending(pending)=>{
            let hash=pending.node_hash;
            if session.invocation_origin_cold(&payload.window_id,hash).is_none(){
                let origin=session.invocation_origin_cold(&payload.window_id,payload.node_hash).ok_or_else(||Fault::from("flow-eval-pending-original-context-required"))?.pending_origin_cold(pending);
                session.retain_invocation_origin_cold(flow::host::FlowInvocationOriginLease::from_cold(origin));
            }
            let origin=session.invocation_origin_cold(&payload.window_id,hash).ok_or_else(||Fault::from("flow-eval-pending-source-required"))?;
            let invocation=ExtensionInvocation::new(origin.extension_id.clone(),"evaluate",origin.request_json_cold(false),"flowEvalResolve");
            return Ok(Emit{extension_invocations:vec![invocation],..Default::default()});
        },
        flow::PreviewEvalOutcome::Cancelled => true,
        flow::PreviewEvalOutcome::Working => false,
    };
    if given_up {
        session.abandon_window_tick(&payload.window_id, crate::editor::flow::cold_grant()).map_err(crate::editor::flow::value_fault)?;
    }
    let armed = session.settle_window_extension(&payload.window_id, crate::editor::flow::cold_grant()).map_err(crate::editor::flow::value_fault)?.0 || session.arm_owed_window_tick(&payload.window_id, crate::editor::flow::cold_grant()).map_err(crate::editor::flow::value_fault)?.0;
    let effects = if armed { vec![eval_tick_effect(&payload.window_id, FLOW_PLAY_WINDOW_MAIN)] } else { Vec::new() };
    Ok(Emit { effects, ..Default::default() })
}
