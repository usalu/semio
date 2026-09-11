//! 🧮️ 🧮️ Generation3d play app commands command — `flow-eval-resolve`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-resolve")]
pub struct FlowEvalResolve {
    /// 🪟️ Echoed back from the tick's own invocation request by `reactor::extension_response_args`,
    /// so the re-armed tick keeps addressing the preview window that owns this evaluation.
    pub window_id: String,
    pub window_kind_id: String,
    pub node_hash: u64,
    pub output_json: String,
}

pub fn handle(payload: &FlowEvalResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    if session.seed_node_cache(payload.node_hash, &payload.output_json).is_err() {
        eprintln!("flowEvalResolve could not seed the node cache for nodeHash={} ({} output bytes)", payload.node_hash, payload.output_json.len());
    }
    Ok(Emit { effects: vec![super::flow_eval_tick::rearm(&payload.window_id, &payload.window_kind_id, 102)], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
