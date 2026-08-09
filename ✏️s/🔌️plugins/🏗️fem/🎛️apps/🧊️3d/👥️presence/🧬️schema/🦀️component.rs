//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

/// 👥️ Empty presence — Fem3dPresence has no shareable live fields.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.fem.3d.presence")]
pub struct Fem3dPresence {}
