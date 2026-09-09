//! 🧮️ 🧮️ Generation2d play app commands command — `flow-eval-resolve`.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-resolve")]
pub struct FlowEvalResolve {
    pub node_hash: u64,
    pub output_json: String,
}

/// 🔁️ The response action of `flowEvalTick`'s `Emit::extension_invocations` entry: the SDK's request
/// registry minted the `req`, parked the continuation and dispatched this command with the extension's
/// own answer merged onto the request object it sent — so `nodeHash` here is the very node the
/// evaluation is waiting on. Seeds the retained session's node cache and re-arms the tick chain.
pub fn handle(payload: &FlowEvalResolve, _doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let _ = session.seed_node_cache(payload.node_hash, &payload.output_json);
    Ok(Emit { effects: vec![Effect::DispatchAction { req: semio_framework_plugin::RequestId(102), action: "flowEvalTick".into(), args: None, delay_ms: 0 }], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
