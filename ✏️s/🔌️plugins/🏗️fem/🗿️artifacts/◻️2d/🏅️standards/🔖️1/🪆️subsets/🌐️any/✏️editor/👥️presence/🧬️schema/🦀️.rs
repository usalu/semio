//! 🧬️ Empty app-presence schema; canonical window views and interaction own live state.
use ::semio_framework_schema::ArtifactSchema;
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(deny_unknown_fields)]
#[artifact_schema(id = "s.fem.2d.presence")]
pub struct NoPresence {}
