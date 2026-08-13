//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use flow::CameraJson;
use super::Procedural3dPreviewCamera;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.3d.config")]
pub struct Procedural3dConfig {
    #[state(config)] pub selected_node_ids: Vec<String>,
    #[state(config)] pub lod_mode: String,
    #[state(config)] pub show_mode: String,
    #[state(config)] pub selection_method: String,
    #[state(config)] pub hovered_node_id: Option<String>,
    #[state(config)] pub camera: CameraJson,
    #[state(config)] pub preview_camera: Procedural3dPreviewCamera,
    #[state(config)] pub sun_json: String,
    #[state(config)] pub selected_generation_id: Option<String>,
    #[state(config)] pub generation_preview_text: Option<String>,
    #[state(config)] pub active_utility_id: String,
    #[state(config)] pub locale: String,
    #[state(config)] pub contributions_json: String,
}

//region 📎 App-schema self-registration
/// 📎 Registers the `s.procedural.3d` app-schema descriptor (config + presence facets) into the
/// open [`::schema::AppSchemaRegistry`], mirroring the transplanted-from-framework closed-catalog
/// entry — see `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs::register_all_app_schema_descriptors()`.
pub fn register_app_schema() {
    ::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor {
        id: "s.procedural.3d",
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
    });
}
//endregion 📎 App-schema self-registration

