//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🧬️Configuration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
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
    #[state(config)]
    pub locale: String,
}
//#endregion 🧬️Configuration

//region 📎 App-schema descriptor
/// 📎 `s.gis.gis2d`'s config+presence schema descriptor — returned, not self-registered;
/// `ArtifactEditor::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to
/// `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.gis.gis2d",
        config: ::schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        presence: ::schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
