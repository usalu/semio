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
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::diff::Generation2dDiff;
use crate::{widget_id, Generation2dSnapshot};
use protocol::Mutation;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
#[cfg(test)]
use semio_framework_artifact_playbook_playbook::FormGeneration;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ArtifactEnvelope, ArtifactStore};
//#region 🔖️Addressing
/// 🌡️ Resolves a widget's stable id to its BASE-state index in the fixture's widget list.
pub fn widget_index(host_snapshot: &FlowHostSnapshot, id: &str) -> Option<usize> {
    host_snapshot.widgets.iter().position(|widget| widget_id(widget) == id)
}

/// 🌡️ Resolves a synapse's stable id to its BASE-state index in the fixture's synapse list.
pub fn synapse_index(host_snapshot: &FlowHostSnapshot, id: &str) -> Option<usize> {
    host_snapshot.synapses.iter().position(|synapse| synapse.id == id)
}
//#endregion 🔖️Addressing

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    UpdateCamera(super::update_camera::UpdateCamera),
    ChangeSchema(super::change_schema::ChangeSchema),
    CreateGeneration(super::create_generation::CreateGeneration),
    DeleteGeneration(super::delete_generation::DeleteGeneration),
    RenameGeneration(super::rename_generation::RenameGeneration),
    ChangeGenerationValue(super::change_generation_value::ChangeGenerationValue),
    ChangeSliderValue(super::change_slider_value::ChangeSliderValue),
    MoveNodes(super::move_nodes::MoveNodes),
    SelectGeneration(super::select_generation::SelectGeneration),
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
    "change-slider-value",
    "move-nodes",
    "select-generation",
];
//#endregion 🏷️Kinds

//#region 🔖️GestureLeaves
/// 🖊️ One number as the history labels print it: English with a decimal point, German with a decimal comma.
pub(crate) fn generation2d_label_number(value: f64) -> (String, String) {
    let english = format!("{}", (value * 1_000.0).round() / 1_000.0);
    let german = english.replace('.', ",");
    (english, german)
}

/// 🧱️ The payload-intrinsic target law every relative gesture leaf states in its schema: at least one id, each once.
pub(crate) fn generation2d_targets_invariant(targets: &[String]) -> Result<(), &'static str> {
    if targets.is_empty() || targets.iter().any(String::is_empty) {
        return Err("a gesture leaf names at least one non-empty target");
    }
    if targets.iter().enumerate().any(|(at, id)| targets[..at].contains(id)) {
        return Err("a gesture leaf names each target once");
    }
    Ok(())
}

/// 🩹️ The `mutation.partial` warning of the targets a relative leaf skipped, or nothing.
pub(crate) fn generation2d_partial(skipped: Vec<String>, total: usize, reason: &str) -> Option<protocol::MutationMessage> {
    (!skipped.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {total} target(s) skipped ({reason}): {}", skipped.len(), skipped.join(", "))).at(skipped))
}
//#endregion 🔖️GestureLeaves
//#endregion 🔖️Mutations

//#region 🧊️Retirement
impl Generation2dMutation {
    /// 🧊️ Explicit cold-only disposal of one owned mutation. `create-widget`/`replace-widget` carry a
    /// whole `Widget`, whose `neural::Dictionary` (and `OrderedSet`/`Tree`) fail-close on a bare drop,
    /// so a dropped mutation aborts the process. Every other variant drops freely.
    pub fn retire_cold(self) {
        match self {
            Self::CreateWidget(super::create_widget::CreateWidget { widget, .. }) | Self::ReplaceWidget(super::replace_widget::ReplaceWidget { widget }) => widget.retire_cold(),
            _ => {}
        }
    }
}
//#endregion 🧊️Retirement

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
        GenerationMutation::Add { generation } => Generation2dMutation::CreateGeneration(super::create_generation::CreateGeneration { generation, index: None }),
        GenerationMutation::Remove { id } => Generation2dMutation::DeleteGeneration(super::delete_generation::DeleteGeneration { id }),
        GenerationMutation::Rename { id, name } => Generation2dMutation::RenameGeneration(super::rename_generation::RenameGeneration { id, name }),
        GenerationMutation::UpdateValues { id, question_id, value } => Generation2dMutation::ChangeGenerationValue(super::change_generation_value::ChangeGenerationValue { id, question_id, value }),
    }
}
//#endregion 🔖️GenerationBridge

//#region 🔖️Builders
pub use super::change_generation_value::change_generation_value;
pub use super::change_schema::change_schema;
pub use super::change_slider_value::change_slider_value;
pub use super::clear_widget_layout::clear_widget_layout;
pub use super::connect_synapse::connect_synapse;
pub use super::create_generation::create_generation;
pub use super::create_widget::create_widget;
pub use super::delete_generation::delete_generation;
pub use super::delete_widget::delete_widget;
pub use super::disconnect_synapse::disconnect_synapse;
pub use super::move_nodes::move_nodes;
pub use super::move_widget::move_widget;
pub use super::rename_generation::rename_generation;
pub use super::select_generation::select_generation;
pub use super::replace_synapse::replace_synapse;
pub use super::replace_widget::replace_widget;
pub use super::update_camera::update_camera;
//#endregion 🔖️Builders

pub type Generation2dEnvelope = ArtifactEnvelope<Generation2dSnapshot, Generation2dMutation>;
pub type Generation2dStore = ArtifactStore<Generation2dSnapshot, Generation2dMutation>;


/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_generation2d_mutation(projection: &Generation2dSnapshot, mutation: &Generation2dMutation) -> Result<Vec<Generation2dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧪️gesture-leaves/🦀️.rs"]
mod gesture_leaves_tests;
//#endregion 🧪️Tests
