//! 🎨️ Playground editor command — `set-active-example`.

use crate::examples::demo;
use crate::standards::v1::subsets::any::schema::empty_playground_snapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_schema::ChangeSchema as ChangeSchemaMutation;
use crate::standards::v1::subsets::any::schema::mutations::PlaygroundMutation;
use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

/// 🎨️ The navbar example id. An empty id is the picker's cleared row.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🎨️ Loads the published `demo` document, or clears the schema when the id is empty.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, PlaygroundSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<PlaygroundMutation, NoConfigMutation>, Fault> {
    let schema = if payload.example_id.is_empty() {
        empty_playground_snapshot().schema
    } else if payload.example_id == demo::ID {
        <PlaygroundSnapshot as store::ArtifactDsl>::parse_dsl(demo::PRIMARY_TEXT).map_err(|error| Fault::from(error.to_string()))?.schema
    } else {
        return Ok(Emit::default());
    };
    Ok(Emit::mutations(vec![PlaygroundMutation::ChangeSchema(ChangeSchemaMutation { new_schema: schema })]))
}
