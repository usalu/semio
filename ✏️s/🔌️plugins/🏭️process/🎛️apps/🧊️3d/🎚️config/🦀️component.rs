//! 🧮️ Process 3d play app — view state (`Process3dConfig`) and its operation enum
//! (`Process3dConfigOperation`), moved out of the old `⚙️engine`/`🔧️op` crates: this is APP state (view
//! state, never document content), so it belongs next to the app that owns it, not the artifact.
//!
//! B1: absorbs every field that used to live in the old UI crate's `Process3dRuntime` app-struct
//! `RefCell` (selection, hover, face pick, selection method, engagement input, camera, sun) plus the two
//! `ViewModel` fields process3d actually read (`active_utility_id`/`locale`) — session-only view state
//! now round-trips through the config `DocumentStore` exactly like document content, with a real
//! `backwards` per [`Process3dConfigOperation`], mirroring the `shooting_engine::ShootingConfig` pilot.
//! The camera (was `Process3dCamera`) and sun (was `WorldSunConfig`) are flattened into scalar fields
//! rather than embedded as DSL blocks — neither type derives `dsl::DslRecord`, and `WorldSunConfig` is
//! shared framework state out of scope for this migration (mirrors `lowpoly_engine::LowpolyConfig`'s
//! identical flattening of its own world camera/sun).

use protocol::Operation;
use serde::{Deserialize, Serialize};

/// 🧰️ The utility active when the config carries no explicit override.
pub const PROCESS3D_DEFAULT_UTILITY: &str = "select";

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "process3dcfg")]
#[dsl(layout = "lines")]
pub struct Process3dConfig {
    /// 👁️ Was `Process3dRuntime::selected_id`.
    pub selected_id: Option<String>,
    /// 👁️ Was `Process3dRuntime::hovered_id`.
    pub hovered_id: Option<String>,
    /// 🖱️ Was `Process3dRuntime::selected_face_id`.
    pub selected_face_id: Option<u32>,
    /// 👁️ Was `Process3dRuntime::selection_method`.
    pub selection_method: String,
    /// 👁️ Was `Process3dRuntime::engagement_input`.
    pub engagement_input: String,
    /// 🎥️ Was `Process3dRuntime::camera` (`Process3dCamera`), flattened.
    #[dsl(coord)]
    pub camera_position: [f64; 3],
    #[dsl(coord)]
    pub camera_target: [f64; 3],
    pub camera_fov: f64,
    /// 🌞️ Was `Process3dRuntime::sun` (`WorldSunConfig`), flattened.
    pub sun_enabled: bool,
    pub sun_azimuth: f64,
    pub sun_elevation: f64,
    pub sun_intensity: f64,
    pub sun_color: String,
    /// 🧰️ Was read off the host-pushed `ViewModel::active_utility_id` (deleted for migrated apps).
    pub active_utility_id: String,
    /// 🗣️ Was read off `ViewModel::locale`.
    pub locale: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `process.machines` hot-swap installs.
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
}

fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for Process3dConfig {
    fn default() -> Self {
        Self {
            selected_id: None,
            hovered_id: None,
            selected_face_id: None,
            selection_method: "rectangle".into(),
            engagement_input: String::new(),
            camera_position: [3.0, -3.0, 2.0],
            camera_target: [0.0, 0.0, 0.0],
            camera_fov: 45.0,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            active_utility_id: PROCESS3D_DEFAULT_UTILITY.into(),
            locale: "en-US".into(),
            contributions_json: default_contributions_json(),
        }
    }
}

impl Process3dConfig {
    /// 🧰️ Resolves the config-owned active utility, falling back to [`PROCESS3D_DEFAULT_UTILITY`] (only
    /// ever triggers if a config value somehow arrives empty).
    pub fn active_utility(&self) -> &str {
        if self.active_utility_id.is_empty() {
            PROCESS3D_DEFAULT_UTILITY
        } else {
            self.active_utility_id.as_str()
        }
    }
}

store::impl_whole_record_config!(Process3dConfig);

/// 🪆️ `Box<Process3dConfig>` needs its own `dsl::DslField` binding for
/// `Process3dConfigOperation::Snapshot` (boxed to fix clippy's `large_enum_variant` — the snapshot
/// variant was ~5x every other row's size) — `Box` is `#[fundamental]` in `std`, so implementing a
/// foreign trait (`dsl::DslField`) for `Box<Process3dConfig>` (a local type inside the foreign,
/// fundamental `Box` wrapper) is permitted by the orphan rules; this delegates entirely to
/// `Process3dConfig`'s own derive-generated `DslField` impl (from `DslDocument`), mirroring `cad`'s
/// identical `Box<CadProjection>` binding.
impl dsl::DslField for Box<Process3dConfig> {
    fn shape() -> dsl::Shape {
        <Process3dConfig as dsl::DslField>::shape()
    }
    fn to_value(&self) -> dsl::FieldValue {
        <Process3dConfig as dsl::DslField>::to_value(self)
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        <Process3dConfig as dsl::DslField>::from_value(value).map(Box::new)
    }
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`Process3dConfig`]'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `Process3dRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()` returns —
/// mirrors `shooting_op::ShootingConfigOperation`/`lowpoly_op::LowpolyConfigOperation`'s identical
/// pattern: a config-only dispatch is always a plain `Apply` (never `AmendLast`), so "undo this tick" =
/// "restore the whole-config snapshot from just before it", the simplest correct inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Process3dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Box<Process3dConfig>,
    },
    #[dsl(key = "selected-id")]
    SetSelectedId { value: Option<String> },
    #[dsl(key = "hovered-id")]
    SetHoveredId { value: Option<String> },
    #[dsl(key = "selected-face-id")]
    SetSelectedFaceId { value: Option<u32> },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(coord)]
        position: [f64; 3],
        #[dsl(coord)]
        target: [f64; 3],
        fov: f64,
    },
    #[dsl(key = "sun")]
    SetSun { enabled: bool, azimuth: f64, elevation: f64, intensity: f64, color: String },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

impl Operation<Process3dConfig> for Process3dConfigOperation {
    type Diff = Process3dConfig;

    fn diff(&self, base: &Process3dConfig) -> Process3dConfig {
        let mut next = base.clone();
        match self {
            Process3dConfigOperation::Snapshot { config } => return config.as_ref().clone(),
            Process3dConfigOperation::SetSelectedId { value } => next.selected_id = value.clone(),
            Process3dConfigOperation::SetHoveredId { value } => next.hovered_id = value.clone(),
            Process3dConfigOperation::SetSelectedFaceId { value } => next.selected_face_id = *value,
            Process3dConfigOperation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            Process3dConfigOperation::SetCamera { position, target, fov } => {
                next.camera_position = *position;
                next.camera_target = *target;
                next.camera_fov = *fov;
            }
            Process3dConfigOperation::SetSun { enabled, azimuth, elevation, intensity, color } => {
                next.sun_enabled = *enabled;
                next.sun_azimuth = *azimuth;
                next.sun_elevation = *elevation;
                next.sun_intensity = *intensity;
                next.sun_color = color.clone();
            }
            Process3dConfigOperation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            Process3dConfigOperation::SetLocale { value } => next.locale = value.clone(),
            Process3dConfigOperation::SetContributions { json } => {
                next.contributions_json = json.clone();
                crate::artifacts::process3d::engine::sync_process_machine_contributions(json);
            }
        }
        next
    }

    fn backwards(&self, base: &Process3dConfig) -> Vec<Self> {
        vec![Process3dConfigOperation::Snapshot { config: Box::new(base.clone()) }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process3d_config_dsl_and_pack_round_trip() {
        use store::DocumentPack;
        let config = Process3dConfig { selected_id: Some("stock".into()), hovered_id: Some("step-0".into()), selected_face_id: Some(3), sun_enabled: true, active_utility_id: "cut".into(), ..Process3dConfig::default() };
        store::test_support::assert_dsl_round_trip(&config);
        let bytes = config.encode_pack();
        assert_eq!(Process3dConfig::decode_pack(&bytes).expect("decode"), config);
    }

    #[test]
    fn process3d_config_operation_backwards_is_always_a_snapshot_of_base() {
        let base = Process3dConfig::default();
        let operation = Process3dConfigOperation::SetSelectedId { value: Some("step-0".into()) };
        let inverse = operation.backwards(&base);
        assert_eq!(inverse, vec![Process3dConfigOperation::Snapshot { config: Box::new(base) }]);
    }

    #[test]
    fn process3d_config_operation_diff_applies_expected_fields() {
        let base = Process3dConfig::default();
        let next = Process3dConfigOperation::SetCamera { position: [1.0, 2.0, 3.0], target: [0.1, 0.2, 0.3], fov: 60.0 }.diff(&base);
        assert_eq!(next.camera_position, [1.0, 2.0, 3.0]);
        assert_eq!(next.camera_target, [0.1, 0.2, 0.3]);
        assert_eq!(next.camera_fov, 60.0);

        let next = Process3dConfigOperation::SetSun { enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#123456".into() }.diff(&base);
        assert!(next.sun_enabled);
        assert_eq!(next.sun_azimuth, 10.0);
        assert_eq!(next.sun_elevation, 20.0);
        assert_eq!(next.sun_intensity, 0.5);
        assert_eq!(next.sun_color, "#123456");
    }

    #[test]
    fn process3d_config_op_text_round_trips_every_variant() {
        let config = Process3dConfig { selected_id: Some("stock".into()), hovered_id: Some("step-0".into()), selected_face_id: Some(2), active_utility_id: "cut".into(), ..Process3dConfig::default() };
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::Snapshot { config: Box::new(config) });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetSelectedId { value: Some("stock".into()) });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetSelectedId { value: None });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetHoveredId { value: Some("step-0".into()) });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetSelectedFaceId { value: Some(3) });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetSelectedFaceId { value: None });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetEngagementInput { value: "cut".into() });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetCamera { position: [1.0, 2.0, 3.0], target: [0.1, 0.2, 0.3], fov: 60.0 });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetSun { enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#123456".into() });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetActiveUtility { utility_id: "cut".into() });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetContributions { json: "[]".into() });
    }

    #[test]
    fn process3d_config_default_matches_the_existing_runtime_defaults() {
        let config = Process3dConfig::default();
        assert_eq!(config.selection_method, "rectangle");
        assert_eq!(config.camera_position, [3.0, -3.0, 2.0]);
        assert_eq!(config.camera_target, [0.0, 0.0, 0.0]);
        assert_eq!(config.camera_fov, 45.0);
        assert!(!config.sun_enabled);
        assert_eq!(config.active_utility_id, "select");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.active_utility(), "select");
    }
}
//#endregion 🧪️Tests
