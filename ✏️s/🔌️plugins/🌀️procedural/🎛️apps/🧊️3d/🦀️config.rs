//! 🧮️ Procedural3d play app — view state (`Procedural3dConfig`) and its operation enum
//! (`Procedural3dConfigOperation`).
//!
//! This is APP state, not document state: selection, cameras, sun/LOD/show-mode display options, and
//! the derived generation preview live here rather than under `🗿️artifacts/`, since none of it survives
//! into the `.procedural3d` document.

use flow_core::CameraJson;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️PreviewCamera
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dPreviewCamera {
    #[serde(default = "default_preview_cam_pos")]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[serde(default = "default_preview_cam_target")]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[serde(default = "default_preview_fov")]
    pub fov: f64,
}

impl Default for Procedural3dPreviewCamera {
    fn default() -> Self {
        Self { position: default_preview_cam_pos(), target: default_preview_cam_target(), fov: default_preview_fov() }
    }
}

pub fn default_preview_cam_pos() -> [f64; 3] {
    [4.0, -4.0, 3.0]
}

pub fn default_preview_cam_target() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

pub fn default_preview_fov() -> f64 {
    45.0
}

pub fn default_show_mode() -> String {
    "shaded".into()
}

pub fn default_selection_method() -> String {
    "rectangle".into()
}

/// 🌞️ Serialized default [`semio_framework_plugin::WorldSunConfig`] — the sun toggle/azimuth/
/// elevation/intensity display options, stored as raw JSON since `WorldSunConfig` is a framework type
/// without a `dsl::DslRecord` impl (see [`Procedural3dConfig::sun`]).
pub fn default_sun_json() -> String {
    serde_json::to_string(&semio_framework_plugin::WorldSunConfig::default()).unwrap_or_default()
}

fn default_contributions_json() -> String {
    "[]".into()
}
//#endregion 🔖️PreviewCamera

//#region 🔖️Config
/// 🧮️ `Procedural3dPlayApp`'s real `DocumentApp::Config` — the pure-trait config artifact. Absorbs
/// selection, hover, selection method, LOD/show display options, flow-graph + preview cameras, sun
/// display options, active generation selection/preview, the active transform-gumball utility, and
/// locale — session-only view state round-trips through the config `DocumentStore` exactly like
/// document content, with a real `backwards` per [`Procedural3dConfigOperation`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "procedural3dcfg")]
#[dsl(layout = "lines")]
pub struct Procedural3dConfig {
    /// 👁️ Selected flow-graph widget ids.
    pub selected_node_ids: Vec<String>,
    /// 🎚️ Level-of-detail tessellation deflection.
    pub lod_mode: String,
    /// 👁️ Preview shading mode.
    pub show_mode: String,
    /// 🖱️ Marquee selection method.
    pub selection_method: String,
    /// 👁️ Hovered flow-graph widget id.
    pub hovered_node_id: Option<String>,
    /// 📷️ The flow-graph node canvas camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 📷️ The 3D preview viewport camera.
    #[dsl(block)]
    pub preview_camera: Procedural3dPreviewCamera,
    /// 🌞️ JSON-encoded `semio_framework_plugin::WorldSunConfig`.
    #[serde(default = "default_sun_json")]
    pub sun_json: String,
    /// 🧬️ The selected generation id.
    pub selected_generation_id: Option<String>,
    /// 🧬️ The evaluated preview text for the selected generation.
    pub generation_preview_text: Option<String>,
    /// 🧰️ The active transform-gumball utility for the preview window.
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `flow.extension` hot-swap installs.
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
}

impl Default for Procedural3dConfig {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            lod_mode: String::new(),
            show_mode: default_show_mode(),
            selection_method: default_selection_method(),
            hovered_node_id: None,
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: Procedural3dPreviewCamera::default(),
            sun_json: default_sun_json(),
            selected_generation_id: None,
            generation_preview_text: None,
            active_utility_id: "move".into(),
            locale: "en-US".into(),
            contributions_json: default_contributions_json(),
        }
    }
}

impl Procedural3dConfig {
    /// 🌞️ Parses `sun_json` — falls back to `WorldSunConfig::default()` on any malformed/legacy value.
    pub fn sun(&self) -> semio_framework_plugin::WorldSunConfig {
        serde_json::from_str(&self.sun_json).unwrap_or_default()
    }
}

store::impl_whole_record_config!(Procedural3dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`Procedural3dConfig`]'s operation enum — one variant per settled interaction, plus a generic
/// `Snapshot` every variant's `backwards()` returns.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Procedural3dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Procedural3dConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { node_ids: Vec<String> },
    #[dsl(key = "hover")]
    SetHover { node_id: Option<String> },
    #[dsl(key = "selection-method")]
    SetSelectionMethod { method: String },
    #[dsl(key = "lod-mode")]
    SetLodMode { value: String },
    #[dsl(key = "show-mode")]
    SetShowMode { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: CameraJson,
    },
    #[dsl(key = "preview-camera")]
    SetPreviewCamera {
        #[dsl(block)]
        camera: Procedural3dPreviewCamera,
    },
    #[dsl(key = "sun")]
    SetSun { json: String },
    #[dsl(key = "generation")]
    SetGeneration { selected_generation_id: Option<String>, generation_preview_text: Option<String> },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

impl Operation<Procedural3dConfig> for Procedural3dConfigOperation {
    type Diff = Procedural3dConfig;

    fn diff(&self, base: &Procedural3dConfig) -> Procedural3dConfig {
        let mut next = base.clone();
        match self {
            Procedural3dConfigOperation::Snapshot { config } => return config.clone(),
            Procedural3dConfigOperation::SetSelection { node_ids } => next.selected_node_ids = node_ids.clone(),
            Procedural3dConfigOperation::SetHover { node_id } => next.hovered_node_id = node_id.clone(),
            Procedural3dConfigOperation::SetSelectionMethod { method } => next.selection_method = method.clone(),
            Procedural3dConfigOperation::SetLodMode { value } => next.lod_mode = value.clone(),
            Procedural3dConfigOperation::SetShowMode { value } => next.show_mode = value.clone(),
            Procedural3dConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            Procedural3dConfigOperation::SetPreviewCamera { camera } => next.preview_camera = camera.clone(),
            Procedural3dConfigOperation::SetSun { json } => next.sun_json = json.clone(),
            Procedural3dConfigOperation::SetGeneration { selected_generation_id, generation_preview_text } => {
                next.selected_generation_id = selected_generation_id.clone();
                next.generation_preview_text = generation_preview_text.clone();
            }
            Procedural3dConfigOperation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            Procedural3dConfigOperation::SetLocale { value } => next.locale = value.clone(),
            Procedural3dConfigOperation::SetContributions { json } => {
                next.contributions_json = json.clone();
                crate::artifacts::procedural3d::engine::sync_flow_extension_contributions(json);
            }
        }
        next
    }

    fn backwards(&self, base: &Procedural3dConfig) -> Vec<Self> {
        vec![Procedural3dConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn procedural3d_config_default_matches_the_former_runtime_defaults() {
        let config = Procedural3dConfig::default();
        assert_eq!(config.show_mode, "shaded");
        assert_eq!(config.selection_method, "rectangle");
        assert_eq!(config.active_utility_id, "move");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.sun(), semio_framework_plugin::WorldSunConfig::default());
    }

    fn config_round_trip(base: &Procedural3dConfig, operation: &Procedural3dConfigOperation) -> Procedural3dConfig {
        let forward = operation.diff(base);
        let backwards = operation.backwards(base);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored);
        }
        assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_set_selection_round_trips() {
        let base = Procedural3dConfig::default();
        let next = config_round_trip(&base, &Procedural3dConfigOperation::SetSelection { node_ids: vec!["a".into(), "b".into()] });
        assert_eq!(next.selected_node_ids, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn config_set_hover_round_trips() {
        let base = Procedural3dConfig::default();
        let next = config_round_trip(&base, &Procedural3dConfigOperation::SetHover { node_id: Some("extrude".into()) });
        assert_eq!(next.hovered_node_id, Some("extrude".to_string()));
    }

    #[test]
    fn config_set_camera_and_preview_camera_round_trip() {
        let base = Procedural3dConfig::default();
        let next = config_round_trip(&base, &Procedural3dConfigOperation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        assert_eq!(next.camera, CameraJson { x: 1.0, y: 2.0, zoom: 3.0 });
        let camera = Procedural3dPreviewCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 };
        let next2 = config_round_trip(&next, &Procedural3dConfigOperation::SetPreviewCamera { camera: camera.clone() });
        assert_eq!(next2.preview_camera, camera);
    }

    #[test]
    fn config_set_sun_round_trip_as_raw_json() {
        let base = Procedural3dConfig::default();
        let next = config_round_trip(&base, &Procedural3dConfigOperation::SetSun { json: "{\"enabled\":true}".into() });
        assert_eq!(next.sun_json, "{\"enabled\":true}");
    }

    #[test]
    fn config_set_generation_round_trips() {
        let base = Procedural3dConfig::default();
        let next = config_round_trip(&base, &Procedural3dConfigOperation::SetGeneration { selected_generation_id: Some("generation-1".into()), generation_preview_text: Some("42".into()) });
        assert_eq!(next.selected_generation_id, Some("generation-1".to_string()));
        assert_eq!(next.generation_preview_text, Some("42".to_string()));
    }

    #[test]
    fn config_set_active_utility_and_locale_round_trip() {
        let base = Procedural3dConfig::default();
        let next = config_round_trip(&base, &Procedural3dConfigOperation::SetActiveUtility { utility_id: "rotate".into() });
        assert_eq!(next.active_utility_id, "rotate");
        let next2 = config_round_trip(&next, &Procedural3dConfigOperation::SetLocale { value: "de-DE".into() });
        assert_eq!(next2.locale, "de-DE");
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::SetSelection { node_ids: vec!["a".into()] });
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::SetHover { node_id: None });
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::SetSelectionMethod { method: "lasso".into() });
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::SetLodMode { value: "coarse".into() });
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::SetShowMode { value: "wireframe".into() });
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::SetPreviewCamera { camera: Procedural3dPreviewCamera { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], fov: 45.0 } });
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::SetSun { json: "{}".into() });
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::SetGeneration { selected_generation_id: Some("g1".into()), generation_preview_text: None });
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::SetActiveUtility { utility_id: "scale".into() });
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_line_round_trip(&Procedural3dConfigOperation::Snapshot { config: Procedural3dConfig::default() });
    }
}
//#endregion 🧪️Tests
