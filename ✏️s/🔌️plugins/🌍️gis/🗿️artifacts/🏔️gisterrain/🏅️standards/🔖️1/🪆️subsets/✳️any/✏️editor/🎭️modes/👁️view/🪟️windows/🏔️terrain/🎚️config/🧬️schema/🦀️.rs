//! 🧬️ schema leaf
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gisterrain.window.config")]
pub struct GisTerrainWindowConfig {
    #[state(config)]
    pub camera_json: String,
}
