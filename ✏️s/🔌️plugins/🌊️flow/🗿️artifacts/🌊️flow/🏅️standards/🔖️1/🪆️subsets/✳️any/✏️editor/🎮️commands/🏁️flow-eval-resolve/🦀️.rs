//! 🧮️ 🧵️ Flow play app commands command — `flow-eval-resolve`.

use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
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

/// 📬️ The reactor moves its native evaluation envelope together with the original window and node address.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[derive(semio_framework_value::RetireOwned)]
pub struct FlowEvalResolve {
    pub window_id: String,
    pub node_hash: u64,
    pub output: Option<semio_framework_value::DslValue>,
}

/// ✅️ Folds one answer into `window_id`'s latch and owes the chain exactly one continuation: the
/// LAST answer of a fan-out re-arms, the earlier ones emit nothing, and an answer that cannot fold
/// gives the window up instead of asking for the same request again.
/// 🎟️ Response publication enters through the registered retained factory and its original caller.
pub fn handle(_payload:&FlowEvalResolve,_doc:&ArtifactView<'_,FlowSnapshot>,_cfg:&ConfigView<'_,NoConfig>,_session:&mut FlowEvalSession)->Result<Emit<FlowMutation,NoConfigMutation>,Fault>{Err(Fault::from("flow-eval-resolve-requires-original-job"))}
