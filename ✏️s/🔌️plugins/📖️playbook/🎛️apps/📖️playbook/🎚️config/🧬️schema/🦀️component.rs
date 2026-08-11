//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.playbook.playbook.config")]
pub struct PlaybookConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub contributions_json: String,
}

//#region 🔖️Register
/// 📌️ Self-registers this app owner's config + presence schema facets into the framework's open
/// `AppSchemaRegistry` (`schema::register_app_schema_descriptor`), replacing the entry framework's
/// closed `register_all_app_schema_descriptors()` still hardcodes for `"s.playbook.playbook"` — see
/// https://github.com/usalu/semio/issues/2543. Called from `artifacts::playbook::engine::register()`
/// (this app's real setup path); mirrors the parked `catalog-integration` call site's exact fn path.
pub fn register_app_schema() {
    schema::register_app_schema_descriptor(schema::AppSchemaDescriptor {
        id: "s.playbook.playbook",
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
    });
}
//#endregion 🔖️Register

