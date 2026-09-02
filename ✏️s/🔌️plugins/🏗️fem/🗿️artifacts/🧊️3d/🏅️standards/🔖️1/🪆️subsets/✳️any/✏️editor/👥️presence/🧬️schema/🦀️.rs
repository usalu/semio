//! 🧬️ schema leaf
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

/// 👥️ Empty presence — Fem3dPresence has no shareable live fields.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.fem.3d.presence")]
pub struct Fem3dPresence {}
