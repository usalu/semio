//! 🧬️ schema leaf
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};
pub type LayerMap<V> = semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, V>;

//#region 🧬️Configuration
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gismap.windowconfig")]
pub struct MapWindowConfig {
    #[state(config)]
    pub layer_visibility: LayerMap<bool>,
    #[state(config)]
    pub camera_json: String,
    #[state(config)]
    pub render_mode: String,
    #[state(config)]
    pub vector_style: String,
    #[state(config)]
    pub lod_mode: String,
    #[state(config)]
    pub layer_stroke_scale: LayerMap<f64>,
}
//#endregion 🧬️Configuration
