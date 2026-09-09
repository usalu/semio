//! 🧬️ Generation2d artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload: eight live in the
//! `🧬️mutations/<slug>/` triad leaves wired by `🦀️.rs` (their directory/module names are
//! leftovers of the generic slots they were repurposed from — see this ticket's wave2 report for
//! the glue.rs rename that would align them), the rest — those with no pre-wired slot — live inline
//! below as `mod <slug> { 🦠️mutation / 🔺️diff / ↩️inverse }` regions, same shape, same file.
//! `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<Generation2dSnapshot>` and
//! `impl protocol::SemanticMutation<Generation2dSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::diff::Generation2dDiff;
use crate::{widget_id, Generation2dSnapshot};
use protocol::Mutation;
use semio_framework_artifact_flow_flow::FlowFixture;
#[cfg(test)]
use semio_framework_artifact_playbook_playbook::FormGeneration;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ArtifactEnvelope, ArtifactStore};
//#region 🔖️Addressing
/// 🌡️ Resolves a widget's stable id to its BASE-state index in the fixture's widget list.
pub fn widget_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.widgets.iter().position(|widget| widget_id(widget) == id)
}

/// 🌡️ Resolves a synapse's stable id to its BASE-state index in the fixture's synapse list.
pub fn synapse_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.synapses.iter().position(|synapse| synapse.id == id)
}
//#endregion 🔖️Addressing

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[mutations(snapshot = Generation2dSnapshot, diff = Generation2dDiff, schema = "generation.2d")]
pub enum Generation2dMutation {
    CreateWidget(super::create_widget::CreateWidget),
    ReplaceWidget(super::replace_widget::ReplaceWidget),
    DeleteWidget(super::delete_widget::DeleteWidget),
    ConnectSynapse(super::connect_synapse::ConnectSynapse),
    ReplaceSynapse(super::replace_synapse::ReplaceSynapse),
    DisconnectSynapse(super::disconnect_synapse::DisconnectSynapse),
    MoveWidget(super::move_widget::MoveWidget),
    ClearWidgetLayout(super::clear_widget_layout::ClearWidgetLayout),
    UpdateCamera(super::set_camera::UpdateCamera),
    ChangeSchema(super::change_schema::ChangeSchema),
    CreateGeneration(super::create_generation::CreateGeneration),
    DeleteGeneration(super::delete_generation::DeleteGeneration),
    RenameGeneration(super::rename_generation::RenameGeneration),
    ChangeGenerationValue(super::change_generation_value::ChangeGenerationValue),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Generation2dMutation`] variant, in declaration order — the exact
/// vocabulary the `procedural-2d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🌀️mutate-procedural-2d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-widget",
    "replace-widget",
    "delete-widget",
    "connect-synapse",
    "replace-synapse",
    "disconnect-synapse",
    "move-widget",
    "clear-widget-layout",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️GenerationBridge
/// 🌉️ Bridges one `semio_framework_artifact_playbook_playbook::GenerationMutation` (the framework's own generation-editing
/// vocabulary — `Add`/`Remove`/`Rename`/`UpdateValues`) onto this facet's semantic
/// `Generation2dMutation` variants, so app-layer callers that already hold a `GenerationMutation`
/// (from `semio_framework_artifact_playbook_playbook::generation_operations`) need only swap the mapping function at the call
/// site, not learn this facet's internal triad-leaf module paths. Twin of generation3d's
/// `generation_mutation_to_generation3d` — the two facets' generation payloads differ only in field
/// naming (`name`/`value` here, `new_name`/`new_value` there).
pub fn generation_mutation_to_generation2d(operation: GenerationMutation) -> Generation2dMutation {
    match operation {
        GenerationMutation::Add { generation } => Generation2dMutation::CreateGeneration(super::create_generation::CreateGeneration { generation }),
        GenerationMutation::Remove { id } => Generation2dMutation::DeleteGeneration(super::delete_generation::DeleteGeneration { id }),
        GenerationMutation::Rename { id, name } => Generation2dMutation::RenameGeneration(super::rename_generation::RenameGeneration { id, name }),
        GenerationMutation::UpdateValues { id, question_id, value } => Generation2dMutation::ChangeGenerationValue(super::change_generation_value::ChangeGenerationValue { id, question_id, value }),
    }
}
//#endregion 🔖️GenerationBridge

//#region 🔖️Builders
pub use super::change_generation_value::change_generation_value;
pub use super::change_schema::change_schema;
pub use super::clear_widget_layout::clear_widget_layout;
pub use super::connect_synapse::connect_synapse;
pub use super::create_generation::create_generation;
pub use super::create_widget::create_widget;
pub use super::delete_generation::delete_generation;
pub use super::delete_widget::delete_widget;
pub use super::disconnect_synapse::disconnect_synapse;
pub use super::move_widget::move_widget;
pub use super::rename_generation::rename_generation;
pub use super::replace_synapse::replace_synapse;
pub use super::replace_widget::replace_widget;
pub use super::set_camera::update_camera;
//#endregion 🔖️Builders

//#region 🔖️FixtureOperations
/// 🔀️ Diffs two fixtures into a minimal, invertible, mergeable semantic operation set:
/// created/replaced/deleted widgets and synapses (keyed by id), moved/cleared layout entries, and
/// a changed fixture schema. The canvas camera is ephemeral view state (app config), never a
/// document operation.
pub fn generation2d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Generation2dMutation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(delete_widget(widget_id(widget).to_string()));
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        match before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget)) {
            None => operations.push(create_widget(index, widget.clone())),
            Some(prior) if prior != widget => operations.push(replace_widget(widget.clone())),
            _ => {}
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(disconnect_synapse(synapse.id.clone()));
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        match before.synapses.iter().find(|entry| entry.id == synapse.id) {
            None => operations.push(connect_synapse(index, synapse.clone())),
            Some(prior) if prior != synapse => operations.push(replace_synapse(synapse.clone())),
            _ => {}
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(clear_widget_layout(id.clone()));
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(move_widget(id.clone(), layout.clone()));
        }
    }
    if before.schema != after.schema {
        operations.push(change_schema(after.schema.clone()));
    }
    operations
}
//#endregion 🔖️FixtureOperations

pub type Generation2dEnvelope = ArtifactEnvelope<Generation2dSnapshot, Generation2dMutation>;
pub type Generation2dStore = ArtifactStore<Generation2dSnapshot, Generation2dMutation>;

/// 🧬️ Applies a mutation to a projection — generic over every variant, so it never needs edits
/// when the semantic vocabulary grows.
pub fn apply_generation2d_mutation(projection: &mut Generation2dSnapshot, mutation: &Generation2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_generation2d_mutation(projection: &Generation2dSnapshot, mutation: &Generation2dMutation) -> Vec<Generation2dMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
