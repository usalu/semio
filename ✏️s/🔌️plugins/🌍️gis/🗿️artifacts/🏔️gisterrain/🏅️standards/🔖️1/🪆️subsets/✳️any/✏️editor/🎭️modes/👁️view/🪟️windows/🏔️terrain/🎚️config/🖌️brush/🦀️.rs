//! 🖌️ Persisted settings for one terrain window's sculpting tools.

use semio_framework_surface::terrain::{TerrainBrushAction, TerrainBrushArguments};
use semio_framework_value::{FromValue, ToValue, ValueError, ValueRefusalKind};

/// 🎚️ Brush radius in grid samples, raise or lower step in millimetres, and smoothing percent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromValue, ToValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TerrainBrushSettings {
    pub radius: u32,
    pub elevation_step_millimetres: u32,
    pub smooth_strength: u32,
}

impl Default for TerrainBrushSettings {
    fn default() -> Self {
        Self { radius: 4, elevation_step_millimetres: 25_000, smooth_strength: 50 }
    }
}

impl TerrainBrushSettings {
    /// 📏️ Largest configured height step, stored in exact millimetres.
    pub const MAX_ELEVATION_STEP_MILLIMETRES: u32 = 32_767_000;

    /// 🎟️ Validates the values before persisting them in the addressed window.
    pub fn validate(self) -> Result<Self, ValueError> {
        if self.radius > 32 || !(1..=Self::MAX_ELEVATION_STEP_MILLIMETRES).contains(&self.elevation_step_millimetres) || self.smooth_strength > 100 {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "terrain brush radius, elevation step or smoothing strength exceeds its range"));
        }
        Ok(self)
    }

    /// 🎯️ Supplies the terrain action for this window's selected utility.
    pub fn action(self, utility: &str) -> Option<TerrainBrushAction> {
        let settings = self.validate().ok()?;
        let step = f64::from(settings.elevation_step_millimetres) / 1000.0;
        let (operation, value) = match utility {
            "terrainRaise" => ("raise-lower", step),
            "terrainLower" => ("raise-lower", -step),
            "terrainSmooth" => ("smooth", f64::from(settings.smooth_strength)),
            _ => return None,
        };
        Some(TerrainBrushAction {
            action: "editElevation".into(),
            arguments: TerrainBrushArguments { operation: operation.into(), radius: settings.radius, value },
        })
    }
}
