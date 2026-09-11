//! 🧮️ 🧮️ Generation3d play app commands command — `flow-tessellate-resolve`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-tessellate-resolve")]
pub struct FlowTessellateResolve {
    /// 🪟️ Echoed back from the tessellate invocation's own request, so a resumable tessellation
    /// re-arms the tick on the preview window that owns the mesh it is building.
    pub window_id: String,
    pub window_kind_id: String,
    pub node_hash: u64,
    pub output_json: String,
}

/// ✅️ Folds one budgeted `tessellate` round trip into the retained session. A step that neither
/// finished the mesh nor received its last body chunk re-arms the tick chain, which is what turns
/// the formerly one-shot synchronous tessellation into a resumable job the user can watch and stop.
pub fn handle(payload: &FlowTessellateResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let outcome = session.resolve_preview_tessellate(payload.node_hash, &payload.output_json);
    let effects = if outcome.needs_another_round_trip() { vec![super::flow_eval_tick::rearm(&payload.window_id, &payload.window_kind_id, 107)] } else { Vec::new() };
    Ok(Emit { effects, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
