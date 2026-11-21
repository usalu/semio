//! 👁️ Settings commands for the current GIS Terrain window.

use crate::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        pub camera_json: String,
    }

    pub fn handle(_payload: &SetCamera, _doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️SetCamera

pub mod configure_terrain_brush {
    use super::*;
    use crate::editor::gis3d::modes::view::windows::terrain::config::{missing_window, TerrainBrushSettings, WINDOW_CONTEXT_REQUIRED};
    use semio_framework_value::DslValue;
    use semio_plugin::{ActionArgDef, ArgSchema, FaultCode, FaultOrigin};
    use semio_ui_locale::LocalizedLabel;
    const INVALID: &str = "gis.terrain.brush.settings-invalid";

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[value(rename_all = "camelCase", deny_unknown_fields)]
    #[dsl(keyword = "configure-terrain-brush")]
    pub struct ConfigureTerrainBrush {
        pub radius: u32,
        pub elevation_step: u32,
        pub smooth_strength: u32,
    }

    impl ConfigureTerrainBrush {
        pub fn from_action(args: &DslValue) -> Result<Self, Fault> {
            let read = |key: &str| {
                args.get(key).and_then(DslValue::as_f64)
                    .filter(|value| value.fract() == 0.0 && (0.0..=u32::MAX as f64).contains(value))
                    .map(|value| value as u32)
                    .ok_or_else(invalid)
            };
            let payload = Self { radius: read("radius")?, elevation_step: read("elevationStep")?, smooth_strength: read("smoothStrength")? };
            payload.settings()?;
            Ok(payload)
        }

        pub fn settings(&self) -> Result<TerrainBrushSettings, Fault> {
            TerrainBrushSettings { radius: self.radius, elevation_step: self.elevation_step, smooth_strength: self.smooth_strength }
                .validate().map_err(|_| invalid())
        }
    }

    pub fn action_arguments() -> Vec<ActionArgDef> {
        let defaults = TerrainBrushSettings::default();
        [
            ("radius", LocalizedLabel::native("Radius (samples)", "Radius (Rasterpunkte)"), 0, 32, defaults.radius),
            ("elevationStep", LocalizedLabel::native("Elevation Step (m)", "Höhenänderung (m)"), 1, 32767, defaults.elevation_step),
            ("smoothStrength", LocalizedLabel::native("Smoothing (%)", "Glättung (%)"), 0, 100, defaults.smooth_strength),
        ].into_iter().map(|(key, label, min, max, default)| {
            ActionArgDef::with_schema(key, label, ArgSchema::number(Some(f64::from(min)), Some(f64::from(max)), Some(1.0), true)).default_value(&default)
        }).collect()
    }

    pub fn fault_notices() -> [(&'static str, LocalizedLabel); 2] {
        [
            (INVALID, LocalizedLabel::native("Choose a whole-number radius, elevation step and smoothing strength within the shown ranges.", "Wählen Sie ganzzahlige Werte für Radius, Höhenänderung und Glättungsstärke innerhalb der angezeigten Bereiche.")),
            (WINDOW_CONTEXT_REQUIRED, LocalizedLabel::native("Choose the terrain window to configure.", "Wählen Sie das Geländefenster für die Einstellung.")),
        ]
    }

    fn invalid() -> Fault {
        Fault::new(FaultOrigin::App, FaultCode::new(INVALID), "Terrain brush settings are outside their ranges.")
    }

    pub fn handle(_payload: &ConfigureTerrainBrush, _doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
        Err(missing_window())
    }
}

//#region 🧪️Tests
//#endregion 🧪️Tests
