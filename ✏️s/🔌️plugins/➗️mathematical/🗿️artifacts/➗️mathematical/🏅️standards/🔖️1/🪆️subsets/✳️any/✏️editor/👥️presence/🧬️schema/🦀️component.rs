//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.mathematical.presence")]
pub struct MathematicalPresence {}
