//! ⚡️ Generation3d artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `Generation3dSnapshot`'s shape per `📓️derivation-rules.md`: an id-keyed widget
//! collection (`create`/`update`/`delete-widget`), a relationship/edge collection of synapses
//! (`connect`/`update`/`disconnect-synapse`), a per-widget position map (`move-widget` /
//! `delete-widget-position`), two document-level scalars (`update-camera`, `change-schema`), and an
//! id-keyed generation collection bridged from `semio_framework_artifact_playbook_playbook::GenerationMutation`
//! (`create`/`delete`/`rename-generation`, `change-generation-value`). Every variant wraps exactly
//! one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Generation3dSnapshot, Generation3dMutation>`; `#[derive(dsl::Mutations)]`
//! below generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.
//!
//! `SetWidget`/`RemoveWidget`/`SetSynapse`/`RemoveSynapse`/`SetLayout`/`RemoveLayout`/`SetCamera`/
//! `SetSchema`/`Generation(GenerationMutation)` — the pre-migration generic vocabulary — are gone.
//! Eight triad-leaf directories keep their pre-migration `➖remove-*`/`🎛set-*` names: glue.rs
//! path-includes those exact files and this facet's writable boundary excludes glue.rs, so the
//! directories couldn't be renamed alongside their content — see the migration report's
//! `sharedFileRequests` for the exact rename once a later pass can touch glue.rs.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::{widget_id, Generation3dSnapshot};
use semio_framework_artifact_flow_flow::FlowFixture;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔖️AddressHelpers
/// 🔎️ BASE-state widget index lookup by id — shared by every widget triad leaf's inverse.
pub(crate) fn widget_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.widgets.iter().position(|widget| widget_id(widget) == id)
}

/// 🔎️ BASE-state synapse index lookup by id — shared by every synapse triad leaf's inverse.
pub(crate) fn synapse_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.synapses.iter().position(|synapse| synapse.id == id)
}
//#endregion 🔖️AddressHelpers

//#region 🔖️NewLeaves
// 🌱️ Triad leaves that needed a fresh directory (no pre-migration slot to repurpose) — self-wired
// here since glue.rs is outside this facet's writable boundary; the eight leaves already carrying a
// semantic name (`delete_widget_position`/`disconnect_synapse`/`delete_widget`/`update_camera`/
// `move_widget`/`change_schema`/`update_synapse`/`update_widget`) stay wired by glue.rs's existing
// sibling `pub mod` blocks, unchanged — imported by those names just below.
#[path = "."]
pub mod create_widget {
    #[path = "🌱️create-widget/🦀️.rs"]
    mod component;
    #[path = "🌱️create-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌱️create-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🌱️create-widget/🧪️tests/📝️inserts-node-c-92255f/🦀️.rs"]
    mod tests_inserts_node_c_at_index_2;
}

#[path = "."]
pub mod connect_synapse {
    #[path = "🔗️connect-synapse/🦀️.rs"]
    mod component;
    #[path = "🔗️connect-synapse/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔗️connect-synapse/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🔗️connect-synapse/🧪️tests/🔌️wires-node-b-to-c90f7f/🦀️.rs"]
    mod tests_wires_node_b_to_node_c_at_index_1;
}

#[path = "."]
pub mod create_generation {
    #[path = "➕create-generation/🦀️.rs"]
    mod component;
    #[path = "➕create-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "➕create-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "➕create-generation/🧪️tests/🌱️appends-generatio-5c9205/🦀️.rs"]
    mod tests_appends_generation_2_and_moves_the_selection;
}

#[path = "."]
pub mod delete_generation {
    #[path = "🗑️delete-generation/🦀️.rs"]
    mod component;
    #[path = "🗑️delete-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗑️delete-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🗑️delete-generation/🧪️tests/🔬️t007/🦀️.rs"]
    mod tests_removes_the_selected_generation_2_and_falls_back;
}

#[path = "."]
pub mod rename_generation {
    #[path = "🏷️rename-generation/🦀️.rs"]
    mod component;
    #[path = "🏷️rename-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏷️rename-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🏷️rename-generation/🧪️tests/🏷️retitles-f090d2/🦀️.rs"]
    mod tests_retitles_generation_1_via_new_name;
}

#[path = "."]
pub mod change_generation_value {
    #[path = "🔧️change-generation-value/🦀️.rs"]
    mod component;
    #[path = "🔧️change-generation-value/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔧️change-generation-value/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🔧️change-generation-value/🧪️tests/🔬️t006/🦀️.rs"]
    mod tests_raises_the_storeys_answer_in_generation_1;
}
//#endregion 🔖️NewLeaves

//#region 🔖️RepurposedLeaves
// 🌱️ Triad leaves that repurpose a pre-migration `➖remove-*`/`🎛set-*` directory glue.rs already
// path-includes as a sibling of `component` (this file) under `pub mod mutations { ... }` — brought
// into this file's own scope the same way `cad`'s already-migrated `🧬️mutations/🦀️.rs`
// reaches its own siblings (`use super::create_object;` etc.): `pub use component::*` only lifts
// `component`'s items UP into `mutations`, it doesn't inject `mutations`'s OTHER children back down.
use super::change_schema;
use super::delete_widget;
use super::delete_widget_position;
use super::disconnect_synapse;
use super::move_widget;
use super::update_camera;
use super::update_synapse;
use super::update_widget;
//#endregion 🔖️RepurposedLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the generation3d document, derived per
/// `📓️derivation-rules.md` from `Generation3dSnapshot`'s shape.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Generation3dSnapshot, diff = Generation3dDiff, schema = "generation.3d")]
pub enum Generation3dMutation {
    CreateWidget(create_widget::CreateWidget),
    UpdateWidget(update_widget::UpdateWidget),
    DeleteWidget(delete_widget::DeleteWidget),
    ConnectSynapse(connect_synapse::ConnectSynapse),
    UpdateSynapse(update_synapse::UpdateSynapse),
    DisconnectSynapse(disconnect_synapse::DisconnectSynapse),
    MoveWidget(move_widget::MoveWidget),
    DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition),
    UpdateCamera(update_camera::UpdateCamera),
    ChangeSchema(change_schema::ChangeSchema),
    CreateGeneration(create_generation::CreateGeneration),
    DeleteGeneration(delete_generation::DeleteGeneration),
    RenameGeneration(rename_generation::RenameGeneration),
    ChangeGenerationValue(change_generation_value::ChangeGenerationValue),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Generation3dMutation`] variant, in declaration order — the exact
/// vocabulary the `procedural-3d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🧊️mutate-procedural-3d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-widget",
    "update-widget",
    "delete-widget",
    "connect-synapse",
    "update-synapse",
    "disconnect-synapse",
    "move-widget",
    "delete-widget-position",
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
/// `Generation3dMutation` variants, so app-layer callers that already hold a `GenerationMutation`
/// (from `semio_framework_artifact_playbook_playbook::generation_operations`) need only swap the mapping function at the call
/// site, not learn this facet's internal triad-leaf module paths.
pub fn generation_mutation_to_generation3d(operation: GenerationMutation) -> Generation3dMutation {
    match operation {
        GenerationMutation::Add { generation } => Generation3dMutation::CreateGeneration(create_generation::CreateGeneration { generation }),
        GenerationMutation::Remove { id } => Generation3dMutation::DeleteGeneration(delete_generation::DeleteGeneration { id }),
        GenerationMutation::Rename { id, name } => Generation3dMutation::RenameGeneration(rename_generation::RenameGeneration { id, new_name: name }),
        GenerationMutation::UpdateValues { id, question_id, value } => Generation3dMutation::ChangeGenerationValue(change_generation_value::ChangeGenerationValue { id, question_id, new_value: value }),
    }
}
//#endregion 🔖️GenerationBridge

//#region 🔖️FixtureDiffing
/// 🔀️ Diffs two fixtures into a minimal, invertible, mergeable semantic mutation set — signature
/// preserved from the pre-migration generic-vocabulary version (`🏗️builder`/app callers reach this
/// via `crate::standards::v1::subsets::any::schema::commit_fixture`, unchanged) but every pushed
/// mutation is now a real semantic variant.
pub fn generation3d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Generation3dMutation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(Generation3dMutation::DeleteWidget(delete_widget::DeleteWidget { id: widget_id(widget).to_string() }));
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let prior = before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget));
        match prior {
            Some(previous) if previous != widget => operations.push(Generation3dMutation::UpdateWidget(update_widget::UpdateWidget { widget: widget.clone() })),
            None => operations.push(Generation3dMutation::CreateWidget(create_widget::CreateWidget { index, widget: widget.clone() })),
            _ => {}
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(Generation3dMutation::DisconnectSynapse(disconnect_synapse::DisconnectSynapse { id: synapse.id.clone() }));
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        let prior = before.synapses.iter().find(|entry| entry.id == synapse.id);
        match prior {
            Some(previous) if previous != synapse => operations.push(Generation3dMutation::UpdateSynapse(update_synapse::UpdateSynapse { synapse: synapse.clone() })),
            None => operations.push(Generation3dMutation::ConnectSynapse(connect_synapse::ConnectSynapse { index, synapse: synapse.clone() })),
            _ => {}
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(Generation3dMutation::DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition { id: id.clone() }));
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(Generation3dMutation::MoveWidget(move_widget::MoveWidget { id: id.clone(), layout: layout.clone() }));
        }
    }
    if before.schema != after.schema {
        operations.push(Generation3dMutation::ChangeSchema(change_schema::ChangeSchema { new_schema: after.schema.clone() }));
    }
    operations
}
//#endregion 🔖️FixtureDiffing

pub type Generation3dEnvelope = ArtifactEnvelope<Generation3dSnapshot, Generation3dMutation>;
pub type Generation3dStore = ArtifactStore<Generation3dSnapshot, Generation3dMutation>;

//#region 🔖️Apply
/// 🎬️ Fallible in-place `vcs::apply_mutation` boundary.
pub fn apply_generation3d_mutation(projection: &mut Generation3dSnapshot, mutation: &Generation3dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

pub fn inverse_generation3d_mutation(projection: &Generation3dSnapshot, mutation: &Generation3dMutation) -> Vec<Generation3dMutation> {
    protocol::Mutation::inverse(mutation, projection)
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
