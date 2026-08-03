//! ⚡️ Flow app — operation type facade (constitutional: op).
//!
//! `FlowOperation`, its `protocol::Operation`/`OperationDiff` impls, and the private
//! `apply_flow_operation` fn all live in the shared flow kernel crate (`flow_core`,
//! `s/kernel/flow/core/rs`, `🔖️Operations` region) alongside the `FlowFixture` projection they mutate —
//! see `s/plugin/flow/app/rs/lib.rs` for why. Re-exported here so sibling constitutional crates
//! depend on the app-owned name instead of reaching into the kernel path directly.

use flow_core::CameraJson;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
pub use flow_core::FlowOperation;
//#endregion 🔖️Types

//#region 🔖️ConfigOperations
/// 🧮️ `flow_engine::FlowConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `FlowPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns: since a config-only "View" dispatch is a plain `Apply` (not an `AmendLast` — see
/// `FlowPlayApp::handle`), each tick is its own distinct, real config edit, and "undo this tick" is
/// exactly "restore the whole-config snapshot from just before it" — the simplest correct inverse,
/// needing no per-field reverse-patch bookkeeping. `Operation::Diff` is the WHOLE `FlowConfig` (not a
/// granular patch type): `diff()` returns "the full config after this op", and
/// `OperationDiff<FlowConfig>::apply` for `FlowConfig` itself (in `flow_engine`) just returns that
/// snapshot verbatim, ignoring `base` — the same "whole-record diff" shape
/// `shooting_op::ShootingConfigOperation`/`dag_op::DagConfigOperation`/
/// `procedural_3d_op::Procedural3dConfigOperation` already use.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum FlowConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: flow_engine::FlowConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { node_ids: Vec<String>, edge_ids: Vec<String>, handle_ids: Vec<String> },
    #[dsl(key = "preview-off")]
    SetPreviewOff { node_ids: Vec<String> },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: CameraJson,
    },
    #[dsl(key = "lod-mode")]
    SetLodMode { value: String },
    #[dsl(key = "proximity-distance")]
    SetProximityDistance { value: f64 },
    #[dsl(key = "grid-visible")]
    SetGridVisible { value: bool },
    #[dsl(key = "grid-snap")]
    SetGridSnapEnabled { value: bool },
    #[dsl(key = "grid-factor")]
    SetGridFactor { value: f64 },
    #[dsl(key = "catalogue-sections")]
    SetCatalogueSections { sections_json: String },
    #[dsl(key = "extension-enabled")]
    SetExtensionEnabled { json: String },
    #[dsl(key = "generation")]
    SetGeneration { json: String },
    #[dsl(key = "eval-driver")]
    SetEvalDriver { json: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<flow_engine::FlowConfig> for FlowConfigOperation {
    type Diff = flow_engine::FlowConfig;

    fn diff(&self, base: &flow_engine::FlowConfig) -> flow_engine::FlowConfig {
        let mut next = base.clone();
        match self {
            FlowConfigOperation::Snapshot { config } => return config.clone(),
            FlowConfigOperation::SetSelection { node_ids, edge_ids, handle_ids } => {
                next.selected_node_ids = node_ids.clone();
                next.selected_edge_ids = edge_ids.clone();
                next.selected_handle_ids = handle_ids.clone();
            }
            FlowConfigOperation::SetPreviewOff { node_ids } => next.preview_off_node_ids = node_ids.clone(),
            FlowConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            FlowConfigOperation::SetLodMode { value } => next.lod_mode = value.clone(),
            FlowConfigOperation::SetProximityDistance { value } => next.proximity_distance = *value,
            FlowConfigOperation::SetGridVisible { value } => next.grid_visible = *value,
            FlowConfigOperation::SetGridSnapEnabled { value } => next.grid_snap_enabled = *value,
            FlowConfigOperation::SetGridFactor { value } => next.grid_factor = *value,
            FlowConfigOperation::SetCatalogueSections { sections_json } => next.catalogue_sections_json = sections_json.clone(),
            FlowConfigOperation::SetExtensionEnabled { json } => next.extension_enabled_json = json.clone(),
            FlowConfigOperation::SetGeneration { json } => next.generation_json = json.clone(),
            FlowConfigOperation::SetEvalDriver { json } => next.eval_driver_json = json.clone(),
            FlowConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &flow_engine::FlowConfig) -> Vec<Self> {
        vec![FlowConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_engine::FlowConfig;

    #[test]
    fn flow_config_operation_text_binary_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::Snapshot { config: FlowConfig { selected_node_ids: vec!["n1".into()], locale: "de-DE".into(), ..FlowConfig::default() } });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetSelection { node_ids: vec!["n1".into(), "n2".into()], edge_ids: vec!["e1".into()], handle_ids: vec!["h1".into()] });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetSelection { node_ids: Vec::new(), edge_ids: Vec::new(), handle_ids: Vec::new() });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetPreviewOff { node_ids: vec!["n1".into()] });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetLodMode { value: "micro".into() });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetProximityDistance { value: 48.0 });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetGridVisible { value: true });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetGridSnapEnabled { value: false });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetGridFactor { value: 10.0 });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetCatalogueSections { sections_json: "[]".into() });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetExtensionEnabled { json: "{\"auto-layout\":true}".into() });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetGeneration { json: "{\"generations\":[]}".into() });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetEvalDriver { json: "{}".into() });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetLocale { value: "de-DE".into() });
    }

    #[test]
    fn flow_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = FlowConfig { selected_node_ids: vec!["n1".into()], locale: "en-US".into(), ..FlowConfig::default() };
        let operation = FlowConfigOperation::SetSelection { node_ids: vec!["n2".into()], edge_ids: Vec::new(), handle_ids: Vec::new() };
        let forward = operation.diff(&base);
        assert_eq!(forward.selected_node_ids, vec!["n2".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![FlowConfigOperation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
