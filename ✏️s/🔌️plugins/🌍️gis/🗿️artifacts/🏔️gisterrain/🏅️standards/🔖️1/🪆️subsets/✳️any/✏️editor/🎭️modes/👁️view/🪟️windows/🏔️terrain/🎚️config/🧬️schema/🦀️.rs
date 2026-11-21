//! 🧬️ schema leaf
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gisterrain.window.config")]
pub struct GisTerrainWindowConfig {
    #[state(config)]
    pub camera_json: String,
    #[state(config)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub brush: Option<TerrainBrushSettings>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(Serialize, Deserialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TerrainBrushSettings {
    pub radius: u32,
    pub elevation_step: u32,
    pub smooth_strength: u32,
}
