//! 🧬️ schema leaf
use schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.playbook.playbook.presence")]
pub struct PlaybookPresence {}
