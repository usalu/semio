//! 🧬️ schema leaf
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🧬️Configuration
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gis2d.config")]
pub struct Gis2dConfig {
    #[state(config)]
    pub layer_visibility: BTreeMap<String, bool>,
    #[state(config)]
    pub camera_json: String,
    #[state(config)]
    pub render_mode: String,
    #[state(config)]
    pub vector_style: String,
    #[state(config)]
    pub lod_mode: String,
    #[state(config)]
    pub layer_stroke_scale: BTreeMap<String, f64>,
}
//#endregion 🧬️Configuration

//region 📎 App-schema descriptor
/// 📎 `s.gis.gis2d`'s config+presence schema descriptor — returned, not self-registered;
/// `ArtifactEditor::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to
/// `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::semio_framework_schema::AppSchemaDescriptor {
    ::semio_framework_schema::AppSchemaDescriptor {
        id: "s.gis.gis2d",
        config: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        presence: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🧫️fixtures/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
