//! 🧬️ Exact Flow main-window configuration schema.

use crate::schema::{FLOW_DEFAULT_GRID_FACTOR, FLOW_DEFAULT_PROXIMITY_DISTANCE};
use flow::FLOW_LOD_MODE_AUTOMATIC;
use semio_framework_artifact_flow_flow::CameraJson;

#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.flow.flow.mainwindowconfig", extension = "flowmainwindowcfg", layout = "lines")]
pub struct FlowMainWindowConfig {
    pub preview_off_node_ids: Vec<String>,
    #[dsl(block)]
    pub camera: CameraJson,
    pub lod_mode: String,
    pub proximity_distance: f64,
    pub grid_visible: bool,
    pub grid_snap_enabled: bool,
    pub grid_factor: f64,
    pub catalogue_sections_json: String,
    pub automation_enabled_json: String,
}

impl Default for FlowMainWindowConfig {
    fn default() -> Self {
        Self {
            preview_off_node_ids: Vec::new(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            lod_mode: FLOW_LOD_MODE_AUTOMATIC.into(),
            proximity_distance: FLOW_DEFAULT_PROXIMITY_DISTANCE,
            grid_visible: true,
            grid_snap_enabled: false,
            grid_factor: FLOW_DEFAULT_GRID_FACTOR,
            catalogue_sections_json: "[]".into(),
            automation_enabled_json: "{}".into(),
        }
    }
}
