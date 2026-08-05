//! 🧮️ GIS 3D play app — the view-state config artifact and its operation enum.
//!
//! Session-only but real, undoable config: panning and selecting never enter the document's undo
//! history, but they still round-trip through the config `DocumentStore` with a true `backwards`.
//! The terrain's one editable property (exaggeration) is document state and lives in
//! `crate::artifacts::gisterrain`.

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ gis3d's `DocumentApp::Config` — the free/live viewport camera and world selection, plus
/// `locale`. Mirrors `crate::apps::gis2d::config::Gis2dConfig`'s identical shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "gis3dcfg")]
#[dsl(layout = "lines")]
pub struct Gis3dConfig {
    /// 🎥️ The free/live world camera (`{position,target,up,fov}` JSON).
    pub camera_json: String,
    /// 👁️ Selected pin ids.
    pub selected_ids: Vec<String>,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

/// 🎥️ A default overview camera scaled for a real-world DEM tile patch (hundreds of meters to a
/// few kilometers wide) — the generic `world3d_default_camera()` (position `[4,-4,3]`) assumes an
/// object-scale scene and would sit inside the ground here.
fn default_gis3d_camera_json() -> String {
    serde_json::json!({ "position": [800.0, -800.0, 600.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string()
}

impl Default for Gis3dConfig {
    fn default() -> Self {
        Self { camera_json: default_gis3d_camera_json(), selected_ids: Vec::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(Gis3dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `Gis3dConfig`'s operation enum — one variant per settled interaction, plus a generic `Snapshot`
/// every variant's `backwards()` returns — mirrors `crate::apps::gis2d::config::Gis2dConfigOperation`'s
/// identical shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Gis3dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Gis3dConfig,
    },
    #[dsl(key = "camera")]
    SetCamera { camera_json: String },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<Gis3dConfig> for Gis3dConfigOperation {
    type Diff = Gis3dConfig;

    fn diff(&self, base: &Gis3dConfig) -> Gis3dConfig {
        let mut next = base.clone();
        match self {
            Gis3dConfigOperation::Snapshot { config } => return config.clone(),
            Gis3dConfigOperation::SetCamera { camera_json } => next.camera_json = camera_json.clone(),
            Gis3dConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            Gis3dConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &Gis3dConfig) -> Vec<Self> {
        vec![Gis3dConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gis3d_config_default_matches_the_pre_migration_view_defaults() {
        let config = Gis3dConfig::default();
        assert!(config.camera_json.contains("800"));
        assert!(config.selected_ids.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn gis3d_config_dsl_round_trips_default_and_populated() {
        store::test_support::assert_dsl_round_trip(&Gis3dConfig::default());
        let mut populated = Gis3dConfig::default();
        populated.selected_ids = vec!["p_institut_de_botanique_ulg_liege".into()];
        store::test_support::assert_dsl_round_trip(&populated);
        store::test_support::assert_dsl_pack_equivalence(&populated);
    }

    #[test]
    fn gis3d_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = Gis3dConfig::default();
        let operation = Gis3dConfigOperation::SetSelection { ids: vec!["p1".into()] };
        let next = operation.diff(&base);
        assert_eq!(next.selected_ids, vec!["p1".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![Gis3dConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&next), base);
    }

    #[test]
    fn gis3d_config_operation_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&Gis3dConfigOperation::SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() });
        store::test_support::assert_op_line_round_trip(&Gis3dConfigOperation::SetSelection { ids: vec!["p1".into()] });
        store::test_support::assert_op_line_round_trip(&Gis3dConfigOperation::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_line_round_trip(&Gis3dConfigOperation::Snapshot { config: Gis3dConfig::default() });
    }
}
//#endregion 🧪️Tests
