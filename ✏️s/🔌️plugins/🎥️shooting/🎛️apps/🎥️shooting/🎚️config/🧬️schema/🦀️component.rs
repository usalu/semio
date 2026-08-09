//! 🧬️ schema leaf
use crate::artifacts::shooting::ShootingCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.shooting.shooting.config")]
pub struct ShootingConfig {
    #[state(local_ui)] pub default_shot_format: String,
    #[state(local_ui)] pub default_shot_shape: String,
    #[state(local_ui)] pub default_asset_format: String,
    #[state(local_ui)] pub selected_shot_ids: Vec<String>,
    #[state(local_ui)] pub selected_asset_ids: Vec<String>,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub hovered_asset_id: Option<String>,
    #[state(local_ui)] pub center_model: bool,
    #[state(local_ui)] pub fit_revision: u32,
    #[state(local_ui)] pub camera_draft_label: String,
    #[state(local_ui)] pub camera: ShootingCamera,
    #[state(local_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
}

//#region 🔖️AppSchemaDescriptor
/// 📚 Handcrafted app schema descriptor for this owner (config + presence facets).
pub fn app_schema_descriptor() -> schema::AppSchemaDescriptor {
    schema::AppSchemaDescriptor {
        id: "s.shooting.shooting",
        config: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        presence: schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️component.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️component.proto"),
        },
    }
}

/// 📎 Registers this owner's app schema into the OS-wide catalog.
pub fn register_app_schema() {
    schema::register_app_schema_descriptor(app_schema_descriptor());
}
//#endregion 🔖️AppSchemaDescriptor
