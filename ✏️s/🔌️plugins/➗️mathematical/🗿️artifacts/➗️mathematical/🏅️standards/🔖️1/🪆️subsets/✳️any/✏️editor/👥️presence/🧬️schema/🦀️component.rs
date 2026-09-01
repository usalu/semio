//! 🧬️ schema leaf
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.mathematical.presence")]
pub struct MathematicalPresence {}
