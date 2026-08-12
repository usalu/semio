//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gis2d.config")]
pub struct Gis2dConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub layer_visibility: BTreeMap<String, bool>,
    #[state(local_ui)] pub camera_json: String,
    #[state(local_ui)] pub render_mode: String,
    #[state(local_ui)] pub vector_style: String,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub feature_selection_json: String,
    #[state(local_ui)] pub hover_json: String,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub selection_mode: String,
    #[state(local_ui)] pub layer_stroke_scale: BTreeMap<String, f64>,
    #[state(local_ui)] pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 `s.gis.gis2d`'s config+presence schema descriptor — returned, not self-registered;
/// `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to
/// `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.gis.gis2d",
        config: ::schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        presence: ::schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️component.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️component.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor

