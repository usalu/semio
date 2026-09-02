//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.playbook.playbook.config")]
pub struct PlaybookConfig {
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub contributions_json: String,
}

//#region 🔖️Register
/// 📌️ This app owner's config + presence schema facets, replacing the entry framework's closed
/// `register_all_app_schema_descriptors()` still hardcodes for `"s.playbook.playbook"` — see
/// https://github.com/usalu/semio/issues/2543. Returned, not self-registered; `ArtifactEditor::app_schema`
/// (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to `register_document_app`, which
/// replaces the narrowed `.setup()` this used to run through — `.setup()` is gone from this plugin root.
pub fn app_schema_descriptor() -> schema::AppSchemaDescriptor {
    schema::AppSchemaDescriptor {
        id: "s.playbook.playbook",
        config: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        presence: schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Register
