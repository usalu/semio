//! 🧬️ Forms app configuration schema leaf.

use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms.config")]
pub struct FormsConfig {
    #[state(config)]
    pub contributions_json: String,
}

pub fn app_schema_descriptor() -> ::framework_schema::AppSchemaDescriptor {
    ::framework_schema::AppSchemaDescriptor {
        id: "s.forms.forms",
        config: ::framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        presence: ::framework_schema::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },
    }
}
