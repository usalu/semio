//! 🔧️ Playground editor command — `change-schema`. The document's whole persistent snapshot is one
//! opaque `schema` metadata string (see `🧬️schema/🧬️mutations/🦀️.rs`'s own doc comment), so
//! this is the surface's only taxonomy command, dispatching straight through to the artifact's one
//! `PlaygroundMutation::ChangeSchema` variant.

use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::change_schema::ChangeSchema as ChangeSchemaMutation;
use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::PlaygroundMutation;
use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "change-schema")]
pub struct ChangeSchema {
    pub new_schema: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &ChangeSchema, _doc: &ArtifactView<'_, PlaygroundSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<PlaygroundMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![PlaygroundMutation::ChangeSchema(ChangeSchemaMutation { new_schema: payload.new_schema.clone() })]))
}
//#endregion 🔖️Handle
