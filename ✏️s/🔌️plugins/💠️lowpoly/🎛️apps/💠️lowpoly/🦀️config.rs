//! 🧮️ Lowpoly play app — view state (`LowpolyConfig`) and its patch operations
//! (`LowpolyConfigOperation`). Absorbs every field that used to live in the old ui crate's
//! `LowpolyPlayRuntime` app-struct `RefCell` (selection, active object, paint utility/layer, selection
//! method/mode, hover, world camera, sun, show-edges) plus the two `ViewState` fields lowpoly actually
//! read (`active_utility_id`/`locale`) — session-only view state round-trips through the config
//! `DocumentStore` exactly like document content, with a real `backwards` per
//! `LowpolyConfigOperation`, mirroring the `shooting_engine::ShootingConfig` pilot. Nested value types
//! (`LowpolySelection`, the world camera, hover target, sun, paint color) are flattened into scalar
//! fields rather than embedded as DSL blocks — `LowpolySelection`/`WorldSunConfig` aren't
//! `dsl::DslField`-capable today and flattening avoids widening that surface just for this migration.

use protocol::Operation;
use semio_framework_plugin::WorldSunConfig;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "lowpolycfg")]
#[dsl(layout = "lines")]
pub struct LowpolyConfig {
    /// 👁️ Was `LowpolyPlayRuntime::active_object_id`.
    pub active_object_id: String,
    /// 👁️ Was `LowpolyPlayRuntime::selection` (`LowpolySelection`), flattened.
    pub selection_mode: String,
    pub selection_ids: Vec<u32>,
    pub selection_targets_mesh: bool,
    pub selection_targets_vertex: bool,
    pub selection_targets_edge: bool,
    pub selection_targets_face: bool,
    pub selection_keys: Vec<String>,
    /// 👁️ Was `LowpolyPlayRuntime::paint_utility`.
    pub paint_utility: String,
    /// 👁️ Was `LowpolyPlayRuntime::active_paint_layer`.
    pub active_paint_layer: u32,
    /// 👁️ Was `LowpolyPlayRuntime::selection_method`.
    pub selection_method: String,
    /// 👁️ Was `LowpolyPlayRuntime::selection_mode_default`.
    pub selection_mode_default: String,
    /// 👁️ Was `LowpolyPlayRuntime::selected_object_ids` (`SelectionSet`), flattened to its ordered ids.
    pub selected_object_ids: Vec<String>,
    /// 👁️ Was `LowpolyPlayRuntime::hovered_object_id`.
    pub hovered_object_id: Option<String>,
    /// 👁️ Was `LowpolyPlayRuntime::hovered_target` (`LowpolyHoverTarget`), flattened.
    pub hovered_target_object_id: Option<String>,
    pub hovered_target_mode: Option<String>,
    pub hovered_target_id: Option<u32>,
    /// 👁️ Was `LowpolyPlayRuntime::utility_params` (`serde_json::Value`) — carried as canonical JSON
    /// text since a raw `Value` field has no direct DSL binding.
    pub utility_params_json: String,
    /// 🎨️ Was `LowpolyPlayRuntime::paint_color` (`[u8; 4]`), flattened.
    pub paint_color_r: u8,
    pub paint_color_g: u8,
    pub paint_color_b: u8,
    pub paint_color_a: u8,
    /// 🎥️ Was `LowpolyPlayRuntime::world_camera` (`LowpolyWorldCamera`), flattened.
    #[dsl(coord)]
    pub world_camera_position: [f64; 3],
    #[dsl(coord)]
    pub world_camera_target: [f64; 3],
    #[dsl(angle = "deg")]
    pub world_camera_fov: f64,
    /// 👁️ Was `LowpolyPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 👁️ Was `LowpolyPlayRuntime::show_edges`.
    pub show_edges: bool,
    /// 🌞️ Was `LowpolyPlayRuntime::sun` (`WorldSunConfig`), flattened.
    pub sun_enabled: bool,
    pub sun_azimuth: f64,
    pub sun_elevation: f64,
    pub sun_intensity: f64,
    pub sun_color: String,
    /// 🧰️ Was read off the host-pushed `ViewState::active_utility_id` (deleted for migrated apps).
    pub active_utility_id: String,
    /// 🗣️ Was read off `ViewState::locale`.
    pub locale: String,
}

impl Default for LowpolyConfig {
    fn default() -> Self {
        Self {
            active_object_id: String::new(),
            selection_mode: "mesh".into(),
            selection_ids: Vec::new(),
            selection_targets_mesh: true,
            selection_targets_vertex: false,
            selection_targets_edge: false,
            selection_targets_face: false,
            selection_keys: Vec::new(),
            paint_utility: "brush".into(),
            active_paint_layer: 0,
            selection_method: "rectangle".into(),
            selection_mode_default: "default".into(),
            selected_object_ids: Vec::new(),
            hovered_object_id: None,
            hovered_target_object_id: None,
            hovered_target_mode: None,
            hovered_target_id: None,
            utility_params_json: default_utility_params_json(),
            paint_color_r: 255,
            paint_color_g: 64,
            paint_color_b: 64,
            paint_color_a: 255,
            world_camera_position: [18.0, -18.0, 12.0],
            world_camera_target: [0.0, 0.0, 0.0],
            world_camera_fov: 45.0,
            engagement_input: String::new(),
            show_edges: true,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            active_utility_id: "move".into(),
            locale: "en-US".into(),
        }
    }
}

/// 🧰️ `LowpolyConfig::default`'s `utility_params_json` — mirrors the pre-B1
/// `LowpolyPlayRuntime::utility_params`'s default JSON object verbatim.
pub fn default_utility_params_json() -> String {
    serde_json::json!({
        "extrudeDistance": 0.25,
        "insetAmount": 0.1,
        "bevelAmount": 0.05,
        "bevelSegments": 1,
        "loopCuts": 1,
        "decimateRatio": 0.5,
        "snapGrid": 0.25,
        "mirrorAxis": 0,
        "brushSize": 16,
        "brushOpacity": 1,
        "brushHardness": 0.5,
    })
    .to_string()
}

store::impl_whole_record_config!(LowpolyConfig);

/// 🌞️ Reads `LowpolyConfig`'s flattened sun fields back into a `WorldSunConfig` — the boundary where
/// the framework's shared sun toggle/slider helper (`apply_world3d_sun_action`) can operate on it.
pub fn lowpoly_sun_config(config: &LowpolyConfig) -> WorldSunConfig {
    WorldSunConfig { enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() }
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `LowpolyConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `LowpolyPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — mirrors `shooting_op::ShootingConfigOperation`'s identical pattern: a config-only dispatch
/// is always a plain `Apply` (never `AmendLast`), so "undo this tick" = "restore the whole-config
/// snapshot from just before it", the simplest correct inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum LowpolyConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: LowpolyConfig,
    },
    #[dsl(key = "active-object")]
    SetActiveObject { object_id: String },
    #[dsl(key = "selection")]
    SetSelection { mode: String, ids: Vec<u32> },
    #[dsl(key = "selection-targets")]
    SetSelectionTargets { mesh: bool, vertex: bool, edge: bool, face: bool },
    #[dsl(key = "selection-keys")]
    SetSelectionKeys { keys: Vec<String> },
    #[dsl(key = "paint-utility")]
    SetPaintUtility { value: String },
    #[dsl(key = "active-paint-layer")]
    SetActivePaintLayer { value: u32 },
    #[dsl(key = "selection-method")]
    SetSelectionMethod { value: String },
    #[dsl(key = "selection-mode-default")]
    SetSelectionModeDefault { value: String },
    #[dsl(key = "selected-objects")]
    SetSelectedObjectIds { ids: Vec<String> },
    #[dsl(key = "hovered-object")]
    SetHoveredObject { object_id: Option<String> },
    #[dsl(key = "hovered-target")]
    SetHoveredTarget { object_id: Option<String>, mode: Option<String>, id: Option<u32> },
    #[dsl(key = "utility-params")]
    SetUtilityParams { json: String },
    #[dsl(key = "paint-color")]
    SetPaintColor { r: u8, g: u8, b: u8, a: u8 },
    #[dsl(key = "world-camera")]
    SetWorldCamera {
        #[dsl(coord)]
        position: [f64; 3],
        #[dsl(coord)]
        target: [f64; 3],
        fov: f64,
    },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "show-edges")]
    SetShowEdges { value: bool },
    #[dsl(key = "sun")]
    SetSun { enabled: bool, azimuth: f64, elevation: f64, intensity: f64, color: String },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<LowpolyConfig> for LowpolyConfigOperation {
    type Diff = LowpolyConfig;

    fn diff(&self, base: &LowpolyConfig) -> LowpolyConfig {
        let mut next = base.clone();
        match self {
            LowpolyConfigOperation::Snapshot { config } => return config.clone(),
            LowpolyConfigOperation::SetActiveObject { object_id } => next.active_object_id = object_id.clone(),
            LowpolyConfigOperation::SetSelection { mode, ids } => {
                next.selection_mode = mode.clone();
                next.selection_ids = ids.clone();
            }
            LowpolyConfigOperation::SetSelectionTargets { mesh, vertex, edge, face } => {
                next.selection_targets_mesh = *mesh;
                next.selection_targets_vertex = *vertex;
                next.selection_targets_edge = *edge;
                next.selection_targets_face = *face;
            }
            LowpolyConfigOperation::SetSelectionKeys { keys } => next.selection_keys = keys.clone(),
            LowpolyConfigOperation::SetPaintUtility { value } => next.paint_utility = value.clone(),
            LowpolyConfigOperation::SetActivePaintLayer { value } => next.active_paint_layer = *value,
            LowpolyConfigOperation::SetSelectionMethod { value } => next.selection_method = value.clone(),
            LowpolyConfigOperation::SetSelectionModeDefault { value } => next.selection_mode_default = value.clone(),
            LowpolyConfigOperation::SetSelectedObjectIds { ids } => next.selected_object_ids = ids.clone(),
            LowpolyConfigOperation::SetHoveredObject { object_id } => next.hovered_object_id = object_id.clone(),
            LowpolyConfigOperation::SetHoveredTarget { object_id, mode, id } => {
                next.hovered_target_object_id = object_id.clone();
                next.hovered_target_mode = mode.clone();
                next.hovered_target_id = *id;
            }
            LowpolyConfigOperation::SetUtilityParams { json } => next.utility_params_json = json.clone(),
            LowpolyConfigOperation::SetPaintColor { r, g, b, a } => {
                next.paint_color_r = *r;
                next.paint_color_g = *g;
                next.paint_color_b = *b;
                next.paint_color_a = *a;
            }
            LowpolyConfigOperation::SetWorldCamera { position, target, fov } => {
                next.world_camera_position = *position;
                next.world_camera_target = *target;
                next.world_camera_fov = *fov;
            }
            LowpolyConfigOperation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            LowpolyConfigOperation::SetShowEdges { value } => next.show_edges = *value,
            LowpolyConfigOperation::SetSun { enabled, azimuth, elevation, intensity, color } => {
                next.sun_enabled = *enabled;
                next.sun_azimuth = *azimuth;
                next.sun_elevation = *elevation;
                next.sun_intensity = *intensity;
                next.sun_color = color.clone();
            }
            LowpolyConfigOperation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            LowpolyConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &LowpolyConfig) -> Vec<Self> {
        vec![LowpolyConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use store::DocumentPack;

    #[test]
    fn lowpoly_config_dsl_round_trips_default() {
        store::test_support::assert_dsl_round_trip(&LowpolyConfig::default());
    }

    #[test]
    fn lowpoly_config_dsl_round_trips_non_default() {
        let config = LowpolyConfig {
            active_object_id: "obj-2".into(),
            selection_mode: "face".into(),
            selection_ids: vec![1, 2, 3],
            selected_object_ids: vec!["obj-2".into(), "obj-3".into()],
            hovered_object_id: Some("obj-4".into()),
            hovered_target_object_id: Some("obj-4".into()),
            hovered_target_mode: Some("mesh".into()),
            hovered_target_id: Some(7),
            locale: "de-DE".into(),
            ..LowpolyConfig::default()
        };
        store::test_support::assert_dsl_round_trip(&config);
    }

    #[test]
    fn lowpoly_config_pack_round_trips() {
        let config = LowpolyConfig { active_object_id: "obj-9".into(), sun_enabled: true, ..LowpolyConfig::default() };
        let bytes = config.encode_pack();
        let restored = LowpolyConfig::decode_pack(&bytes).expect("decode");
        assert_eq!(restored, config);
    }

    #[test]
    fn config_op_backwards_always_snapshots_prior_state() {
        let base = LowpolyConfig { active_object_id: "obj-1".into(), ..LowpolyConfig::default() };
        let operation = LowpolyConfigOperation::SetActiveObject { object_id: "obj-2".into() };
        let after = operation.diff(&base);
        assert_eq!(after.active_object_id, "obj-2");
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![LowpolyConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&after), base);
    }

    #[test]
    fn config_op_text_round_trip_set_selection() {
        store::test_support::assert_op_line_round_trip(&LowpolyConfigOperation::SetSelection { mode: "face".into(), ids: vec![1, 2, 3] });
    }

    #[test]
    fn config_op_text_round_trip_world_camera() {
        store::test_support::assert_op_line_round_trip(&LowpolyConfigOperation::SetWorldCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 });
    }

    #[test]
    fn config_op_text_round_trip_snapshot() {
        store::test_support::assert_op_line_round_trip(&LowpolyConfigOperation::Snapshot { config: LowpolyConfig::default() });
    }

    #[test]
    fn config_op_binary_round_trips_and_agrees_with_text() {
        let operation = LowpolyConfigOperation::SetHoveredTarget { object_id: Some("obj-1".into()), mode: Some("mesh".into()), id: Some(3) };
        store::test_support::assert_op_text_binary_equivalence(&operation);
    }
}
//#endregion 🧪️Tests
