//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use flow::CameraJson;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.2d.config")]
pub struct Procedural2dConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub camera: CameraJson,
    #[state(local_ui)] pub show_mode: String,
    #[state(local_ui)] pub selected_generation_id: Option<String>,
    #[state(local_ui)] pub generation_preview_text: Option<String>,
    #[state(local_ui)] pub locale: String,
}

//region 📎 App-schema self-registration
/// 📎 Registers the `s.procedural.2d` app-schema descriptor (config + presence facets) into the
/// open [`::schema::AppSchemaRegistry`], mirroring the transplanted-from-framework closed-catalog
/// entry — see `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs::register_all_app_schema_descriptors()`.
pub fn register_app_schema() {
    ::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor {
        id: "s.procedural.2d",
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

