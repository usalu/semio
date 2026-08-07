//! 🧮️ CAD app — `DocumentApp::Config`: every field that used to live in the app struct's ephemeral
//! `CadPlayRuntime` (selection, hover, engagement session, per-pane cameras, sun, dislocate handles)
//! plus the locale/terminology/active-utility the shell used to push through the deleted `ViewModel`.
//! Session view state round-trips through the config `DocumentStore` exactly like document content,
//! with a real `backwards` via `CadConfigOperation` at the bottom of this file.

use crate::artifacts::cad::CadCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// @emoji 🎯️ Ephemeral World3d hover target — object + optional component (edge/face/vertex). Moved
/// out of `cad_ui` (was private there) so `CadConfig` can embed it as a `#[dsl(block)]` field; every
/// field stays optional so the whole record round-trips through a still-empty hover state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadHoverTarget {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub id: Option<u32>,
}

/// @emoji 🎯️ Which geometry kinds World3d may pick; edges stay enabled so B-rep lines hover/select.
/// Moved out of `cad_ui` alongside `CadHoverTarget`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadSelectionTargets {
    pub mesh: bool,
    pub vertex: bool,
    pub edge: bool,
    pub face: bool,
}

impl Default for CadSelectionTargets {
    fn default() -> Self {
        Self { mesh: true, vertex: false, edge: true, face: false }
    }
}

fn default_component_selection_mode() -> String {
    "mesh".into()
}

/// @emoji 🧩️ Component-level selection for World3d edge/face/vertex overlays. Moved out of `cad_ui`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadComponentSelection {
    #[serde(default)]
    #[dsl(block)]
    pub targets: CadSelectionTargets,
    #[serde(default = "default_component_selection_mode")]
    pub mode: String,
    #[serde(default)]
    pub ids: Vec<u32>,
}

impl Default for CadComponentSelection {
    fn default() -> Self {
        Self { targets: CadSelectionTargets::default(), mode: default_component_selection_mode(), ids: Vec::new() }
    }
}

/// 🎛️ Per-pane handle groups exposed by the Dislocate gumball utility — was keyed by an arbitrary
/// host-pushed `ViewModel.window_id` (`cad_ui::CadPlayRuntime::dislocate_options_by_window_id`); the
/// pure `DocumentApp::render`/`window_measures` surface has no per-window-instance parameter anymore
/// (only `body_key`, which already resolves 1:1 to one of the 4 fixed CAD panes), so `CadConfig` keys
/// this by PANE instead — one named field per pane, mirroring `camera`/`camera_building`/…
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadDislocateOptions {
    pub move_enabled: bool,
    pub rotate_enabled: bool,
}

impl Default for CadDislocateOptions {
    fn default() -> Self {
        Self { move_enabled: true, rotate_enabled: true }
    }
}

/// 🌞️ Local `dsl::DslRecord`-able mirror of `semio_framework_plugin::WorldSunConfig` (foreign,
/// out-of-scope crate — cannot gain a `dsl` derive there). `cad_sun_config_from_world`/
/// `cad_sun_config_to_world` convert at the boundary; field-for-field identical otherwise.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadSunConfig {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}

impl Default for CadSunConfig {
    fn default() -> Self {
        Self { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() }
    }
}

pub fn cad_sun_config_from_world(sun: &semio_framework_plugin::WorldSunConfig) -> CadSunConfig {
    CadSunConfig { enabled: sun.enabled, azimuth: sun.azimuth, elevation: sun.elevation, intensity: sun.intensity, color: sun.color.clone() }
}

pub fn cad_sun_config_to_world(sun: &CadSunConfig) -> semio_framework_plugin::WorldSunConfig {
    semio_framework_plugin::WorldSunConfig { enabled: sun.enabled, azimuth: sun.azimuth, elevation: sun.elevation, intensity: sun.intensity, color: sun.color.clone() }
}

/// 🧮️ B1/WORKFLOWS-END-TO-END-TYPED-PORTS: cad's real `DocumentApp::Config` — see the region doc
/// comment above for the full absorption story. `selected_object_ids` is a plain `Vec<String>` (not
/// `semio_framework_plugin::SelectionSet`, which is foreign and has no `dsl` derive); `cad_ui` still
/// uses the richer `SelectionSet` internally and converts at the boundary.
/// `engagement_session_json` is the pre-serialized JSON of `cad_document_engine::interaction::
/// CadEngagementScratch` — that type's `context: HashMap<String, Value>` field has no `dsl` shape
/// (arbitrary JSON), so it round-trips as an opaque string rather than a nested `#[dsl(block)]`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "cadcfg")]
#[dsl(layout = "lines")]
pub struct CadConfig {
    /// 👁️ Was `CadPlayRuntime::selected_object_ids` (`SelectionSet`).
    pub selected_object_ids: Vec<String>,
    /// 👁️ Was `CadPlayRuntime::selected_node_ids`.
    pub selected_node_ids: Vec<String>,
    /// 👁️ Marquee selection method (`"rectangle"`/…) — was `CadPlayRuntime::selection_method`.
    pub selection_method: String,
    /// 👁️ Was `CadPlayRuntime::hovered_object_id`.
    pub hovered_object_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::hovered_target`.
    #[dsl(block)]
    pub hovered_target: Option<CadHoverTarget>,
    /// 👁️ Was `CadPlayRuntime::active_object_id`.
    pub active_object_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::component_selection`.
    #[dsl(block)]
    pub component_selection: CadComponentSelection,
    /// 👁️ Was `CadPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 👁️ Was `CadPlayRuntime::engagement_step`.
    pub engagement_step: String,
    /// 👁️ Was `CadPlayRuntime::active_example_id`.
    pub active_example_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_reference_model_definition_id`.
    pub selected_reference_model_definition_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_reference_id`.
    pub selected_reference_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_primitive_id`.
    pub selected_primitive_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_primitive_kind`.
    pub selected_primitive_kind: Option<String>,
    /// 👁️ Was `CadPlayRuntime::engagement_pane`.
    pub engagement_pane: Option<String>,
    /// 👁️ Was `CadPlayRuntime::engagement_session` (`Option<CadEngagementScratch>`) — see the struct
    /// doc comment for why this is an opaque JSON string here.
    pub engagement_session_json: Option<String>,
    /// 👁️ Was `CadPlayRuntime::last_finalized_interaction_id`.
    pub last_finalized_interaction_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::sun` (`WorldSunConfig`).
    #[dsl(block)]
    pub sun: CadSunConfig,
    /// 🎥️ Per-pane camera pose — was `CadPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: CadCamera,
    /// 🎥️ Was `CadPlayRuntime::camera_building`.
    #[dsl(block)]
    pub camera_building: CadCamera,
    /// 🎥️ Was `CadPlayRuntime::camera_energy`.
    #[dsl(block)]
    pub camera_energy: CadCamera,
    /// 🎥️ Was `CadPlayRuntime::camera_structure_classic`.
    #[dsl(block)]
    pub camera_structure_classic: CadCamera,
    /// 🎛️ Was `CadPlayRuntime::dislocate_options_by_window_id.get(CAD_PLAY_WINDOW_SHAPE)` — see
    /// `CadDislocateOptions`'s doc comment for the per-window-id → per-pane simplification.
    #[dsl(block)]
    pub dislocate_shape: CadDislocateOptions,
    #[dsl(block)]
    pub dislocate_building: CadDislocateOptions,
    #[dsl(block)]
    pub dislocate_energy: CadDislocateOptions,
    #[dsl(block)]
    pub dislocate_structure_classic: CadDislocateOptions,
    /// 🧰️ The active transform-gumball utility — was read off `view_state.active_utility_id`
    /// (host-pushed `ViewModel`, deleted by B1).
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
    /// 🗣️ Terminology id (`"native"`/`"reuse"`) — was read off `view_state.terminology`.
    pub terminology: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `cad.computer` hot-swap installs.
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
}

fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for CadConfig {
    fn default() -> Self {
        Self {
            selected_object_ids: Vec::new(),
            selected_node_ids: Vec::new(),
            selection_method: "rectangle".into(),
            hovered_object_id: None,
            hovered_target: None,
            active_object_id: None,
            component_selection: CadComponentSelection::default(),
            engagement_input: String::new(),
            engagement_step: "Idle".into(),
            active_example_id: None,
            selected_reference_model_definition_id: None,
            selected_reference_id: None,
            selected_primitive_id: None,
            selected_primitive_kind: None,
            engagement_pane: None,
            engagement_session_json: None,
            last_finalized_interaction_id: None,
            sun: CadSunConfig::default(),
            camera: CadCamera::default(),
            camera_building: CadCamera::default(),
            camera_energy: CadCamera::default(),
            camera_structure_classic: CadCamera::default(),
            dislocate_shape: CadDislocateOptions::default(),
            dislocate_building: CadDislocateOptions::default(),
            dislocate_energy: CadDislocateOptions::default(),
            dislocate_structure_classic: CadDislocateOptions::default(),
            active_utility_id: "move".into(),
            locale: "en-US".into(),
            terminology: "native".into(),
            contributions_json: default_contributions_json(),
        }
    }
}

store::impl_whole_record_config!(CadConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ WORKFLOWS-END-TO-END-TYPED-PORTS config recipe: `CadConfig`'s
/// operation enum. Unlike `CadOperation` (many narrow document-mutating variants), this is a single
/// whole-record `Snapshot`: `cad_ui`'s pure `handle()` converts its (former `RefCell`-backed)
/// `CadPlayRuntime` scratch struct into the next `CadConfig` once per dispatch and diffs it against the
/// pre-command config, exactly like `CadOperation::SetScene`'s existing "whole-document replace"
/// pattern — session state (selection/hover/camera/engagement/…) mutates in tight clusters (e.g.
/// `worldSelect` touches 5+ fields together), so per-field variants would just be wide-argument
/// snapshots in miniature with none of a real granular diff's benefit. `backwards()` restores the
/// exact pre-command `CadConfig`, giving real, exact undo without any per-field reverse-patch
/// bookkeeping — the same justification `shooting_op::ShootingConfigOperation` documents for its own
/// `Snapshot` fallback, generalized here to the sole variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum CadConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: CadConfig,
    },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

impl Operation<CadConfig> for CadConfigOperation {
    type Diff = CadConfig;

    fn diff(&self, base: &CadConfig) -> CadConfig {
        match self {
            CadConfigOperation::Snapshot { config } => config.clone(),
            CadConfigOperation::SetContributions { json } => {
                let mut next = base.clone();
                next.contributions_json = json.clone();
                crate::artifacts::cad::engine::sync_cad_computer_contributions(json);
                next
            }
        }
    }

    fn backwards(&self, base: &CadConfig) -> Vec<Self> {
        vec![CadConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cad_config_default_matches_the_existing_runtime_defaults() {
        let config = CadConfig::default();
        assert_eq!(config.selection_method, "rectangle");
        assert_eq!(config.engagement_step, "Idle");
        assert_eq!(config.active_utility_id, "move");
        assert_eq!(config.locale, "en-US");
        assert!(config.dislocate_shape.move_enabled);
        assert!(config.dislocate_shape.rotate_enabled);
    }

    #[test]
    fn cad_config_dsl_round_trips_a_populated_record() {
        let mut config = CadConfig {
            selected_object_ids: vec!["object-1".into(), "object-2".into()],
            hovered_target: Some(CadHoverTarget { object_id: Some("object-1".into()), mode: Some("edge".into()), id: Some(3) }),
            engagement_session_json: Some("{\"interactionId\":\"box\"}".into()),
            active_utility_id: "rotate".into(),
            locale: "de-DE".into(),
            ..CadConfig::default()
        };
        config.component_selection.mode = "face".into();
        config.component_selection.ids = vec![1, 2, 3];
        config.camera.position = [1.0, 2.0, 3.0];
        let text = store::DocumentDsl::print_dsl(&config);
        let parsed = <CadConfig as store::DocumentDsl>::parse_dsl(&text).expect("cad config dsl parses");
        assert_eq!(parsed, config);
    }

    #[test]
    fn cad_config_pack_round_trips() {
        let mut config = CadConfig { selected_node_ids: vec!["node-1".into()], ..CadConfig::default() };
        config.dislocate_building.rotate_enabled = false;
        let bytes = store::DocumentPack::encode_pack(&config);
        let decoded = <CadConfig as store::DocumentPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, config);
    }

    #[test]
    fn cad_sun_config_round_trips_through_world_sun_config() {
        let world = semio_framework_plugin::WorldSunConfig { enabled: true, azimuth: 12.0, elevation: 34.0, intensity: 0.5, color: "#112233".into() };
        let cad_sun = cad_sun_config_from_world(&world);
        let back = cad_sun_config_to_world(&cad_sun);
        assert_eq!(back, world);
    }

    #[test]
    fn cad_config_operation_snapshot_round_trips_and_restores_exactly() {
        let base = CadConfig { selection_method: "lasso".into(), active_utility_id: "move".into(), ..CadConfig::default() };
        let next = CadConfig { selection_method: "lasso".into(), active_utility_id: "rotate".into(), selected_object_ids: vec!["object-1".into()], ..CadConfig::default() };
        let operation = CadConfigOperation::Snapshot { config: next.clone() };
        let forward = operation.diff(&base);
        assert_eq!(forward, next);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![CadConfigOperation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn cad_config_set_contributions_round_trips() {
        let base = CadConfig::default();
        let json = r#"[{"pluginId":"cad-extension-spatial-shape","contribution":{"kind":"cadComputer","appId":"cad-play","moduleId":"spatial-shape","label":"Spatial Shape","iconId":"box","computersJson":"{}"}}]"#;
        let operation = CadConfigOperation::SetContributions { json: json.into() };
        let next = operation.diff(&base);
        assert_eq!(next.contributions_json, json);
        store::test_support::assert_op_line_round_trip(&operation);
    }
}
//#endregion 🧪️Tests
