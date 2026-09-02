//! 🧬️ schema leaf
use schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.forms.forms.presence")]
pub struct FormsPresence {}
