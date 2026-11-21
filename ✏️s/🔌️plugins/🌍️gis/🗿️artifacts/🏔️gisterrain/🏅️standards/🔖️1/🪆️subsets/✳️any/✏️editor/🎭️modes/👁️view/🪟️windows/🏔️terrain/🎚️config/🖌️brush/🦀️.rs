//! 🖌️ Persisted settings for one terrain window's sculpting tools.

use semio_framework_surface::terrain::{TerrainBrushAction, TerrainBrushArguments};
use semio_framework_value::{FromValue, ToValue, ValueError, ValueRefusalKind};

/// 🎚️ Brush radius in grid samples, raise or lower step in meters, and smoothing percent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromValue, ToValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TerrainBrushSettings {
    pub radius: u32,
    pub elevation_step: u32,
    pub smooth_strength: u32,
}

impl Default for TerrainBrushSettings {
    fn default() -> Self {
        Self { radius: 4, elevation_step: 25, smooth_strength: 50 }
    }
}

impl TerrainBrushSettings {
    /// 🎟️ Validates the values before persisting them in the addressed window.
    pub fn validate(self) -> Result<Self, ValueError> {
        if self.radius > 32 || !(1..=32767).contains(&self.elevation_step) || self.smooth_strength > 100 {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "terrain brush radius, elevation step or smoothing strength exceeds its range"));
        }
        Ok(self)
    }

    /// 🎯️ Supplies the terrain action for this window's selected utility.
    pub fn action(self, utility: &str) -> Option<TerrainBrushAction> {
        let settings = self.validate().ok()?;
        let (operation, value) = match utility {
            "terrainRaise" => ("raise-lower", settings.elevation_step as i32),
            "terrainLower" => ("raise-lower", -(settings.elevation_step as i32)),
            "terrainSmooth" => ("smooth", settings.smooth_strength as i32),
            _ => return None,
        };
        Some(TerrainBrushAction {
            action: "editElevation".into(),
            arguments: TerrainBrushArguments { operation: operation.into(), radius: settings.radius, value },
        })
    }
}
