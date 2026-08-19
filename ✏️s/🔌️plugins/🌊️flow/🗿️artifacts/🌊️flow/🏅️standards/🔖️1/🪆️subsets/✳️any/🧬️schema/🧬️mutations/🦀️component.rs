//! 🧬️ Flow artifact — typed invertible semantic mutations over [`FlowSnapshot`]. Verbs drawn from
//! the closed taxonomy (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md`);
//! every variant wraps a `MutationKind<FlowSnapshot, FlowMutation>` payload from its own
//! `🧬️mutations/<kind>/` triad leaf. `impl Mutation`/`impl SemanticMutation` are
//! `#[derive(protocol::Mutations)]`-generated — never hand-written.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::flow::schema::diff::text::FlowDiff;
use crate::artifacts::flow::FlowSnapshot;
use protocol::{CollectionMutation, Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔹Operation
/// 🌊️ Typed, invertible flow-document semantic mutations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, protocol::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = FlowSnapshot, diff = FlowDiff, schema = "flow.flow")]
pub enum FlowMutation {
    CreateWidget(super::create_widget::mutation::CreateWidget),
    DeleteWidget(super::delete_widget::mutation::DeleteWidget),
    ReorderWidgets(super::reorder_widgets::mutation::ReorderWidgets),
    ReplaceWidget(super::replace_widget::mutation::ReplaceWidget),
    ConnectWidgets(super::connect_widgets::mutation::ConnectWidgets),
    DisconnectWidgets(super::disconnect_widgets::mutation::DisconnectWidgets),
    ReorderSynapses(super::reorder_synapses::mutation::ReorderSynapses),
    UpdateSynapseEndpoints(super::update_synapse_endpoints::mutation::UpdateSynapseEndpoints),
    MoveWidgets(super::move_widgets::mutation::MoveWidgets),
    DuplicateWidget(super::duplicate_widget::mutation::DuplicateWidget),
}

pub type FlowEnvelope = ArtifactEnvelope<FlowSnapshot, FlowMutation>;
pub type FlowStore = ArtifactStore<FlowSnapshot, FlowMutation>;

/// 🌈️ Applies a mutation onto a snapshot in place.
pub async fn apply_flow_mutation(snapshot: &mut FlowSnapshot, mutation: &FlowMutation) -> protocol::MutationApplyResult<()> {
    let next = <FlowMutation as Mutation<FlowSnapshot>>::diff(mutation, snapshot).diff().apply(snapshot)?;

    *snapshot = next;
    Ok(())
}

/// ↩️ Inverse mutations for undo.
pub async fn inverse_flow_mutation(snapshot: &FlowSnapshot, mutation: &FlowMutation) -> Vec<FlowMutation> {
    <FlowMutation as Mutation<FlowSnapshot>>::inverse(mutation, snapshot)
}
//#endregion 🔹Operation

//#region 🌉️FrameworkBridge
/// 🌎️ Converts a framework kernel mutation into this plugin's semantic mutation vocabulary.
/// `SetFixture` (whole-fixture replace) has no semantic-mutation representation — banned per the
/// taxonomy's `set-snapshot` ruling, "it has NO replacement mutation" — so it returns `None`;
/// callers route that case through `store::ArtifactStore::reset` instead of the `Mutation` enum.
/// The framework's own diffing helper (`flow::flow_fixture_operations`) never emits `SetFixture`
/// (only `Widgets`/`Synapses`/`SetLayout`), so this arm is unreachable on the live host-bridge path
/// and only matters for a hand-authored/decoded `flow.op` line.
/// ✏️ Runs a stateful host mutation and diffs the result back into granular `FlowMutation`s — pure
/// over two snapshots, so it lives here beside [`from_framework_mutation`] rather than under an app.
/// Returns an empty vec when the two fixtures are identical.
pub async fn snapshot_operations(before: &FlowSnapshot, after: &FlowSnapshot) -> Vec<FlowMutation> {
    flow::flow_fixture_operations(&before.to_fixture(), &after.to_fixture()).into_iter().filter_map(from_framework_mutation).collect()
}

pub async fn from_framework_mutation(mutation: flow::FlowMutation) -> Option<FlowMutation> {
    Some(match mutation {
        flow::FlowMutation::Widgets(operation) => match operation {
            CollectionMutation::Add { index, item } => FlowMutation::CreateWidget(super::create_widget::mutation::CreateWidget { index, widget: item }),
            CollectionMutation::Remove { id } => FlowMutation::DeleteWidget(super::delete_widget::mutation::DeleteWidget { id }),
            CollectionMutation::Move { id, to_index } => FlowMutation::ReorderWidgets(super::reorder_widgets::mutation::ReorderWidgets { id, to_index }),
            CollectionMutation::Patch { id, patch } => FlowMutation::ReplaceWidget(super::replace_widget::mutation::ReplaceWidget { id, widget: patch }),
        },
        flow::FlowMutation::Synapses(operation) => match operation {
            CollectionMutation::Add { index, item } => FlowMutation::ConnectWidgets(super::connect_widgets::mutation::ConnectWidgets {
                index,
                id: item.id,
                from: item.from,
                from_port: item.from_port,
                to: item.to,
                to_port: item.to_port,
            }),
            CollectionMutation::Remove { id } => FlowMutation::DisconnectWidgets(super::disconnect_widgets::mutation::DisconnectWidgets { id }),
            CollectionMutation::Move { id, to_index } => FlowMutation::ReorderSynapses(super::reorder_synapses::mutation::ReorderSynapses { id, to_index }),
            CollectionMutation::Patch { id, patch } => FlowMutation::UpdateSynapseEndpoints(super::update_synapse_endpoints::mutation::UpdateSynapseEndpoints {
                id,
                from: patch.from,
                from_port: patch.from_port,
                to: patch.to,
                to_port: patch.to_port,
            }),
        },
        flow::FlowMutation::SetLayout { entries } => FlowMutation::MoveWidgets(super::move_widgets::mutation::MoveWidgets { entries }),
        flow::FlowMutation::SetFixture { .. } => return None,
    })
}

/// 🌎️ Converts this plugin's semantic mutation into the framework kernel mutation enum — `None` for
/// `DuplicateWidget`: a composite folds to a SINGLE `FlowDiff`, but it is not itself a single
/// framework-generic op (it plans two: a `Widgets::Add` then a `Synapses::Add`), so there is no
/// framework-generic counterpart to bridge to — mirrors [`from_framework_mutation`]'s `SetFixture`
/// case, one direction over.
pub async fn to_framework_mutation(mutation: &FlowMutation) -> Option<flow::FlowMutation> {
    Some(match mutation {
        FlowMutation::CreateWidget(payload) => flow::FlowMutation::Widgets(CollectionMutation::Add { index: payload.index, item: payload.widget.clone() }),
        FlowMutation::DeleteWidget(payload) => flow::FlowMutation::Widgets(CollectionMutation::Remove { id: payload.id.clone() }),
        FlowMutation::ReorderWidgets(payload) => flow::FlowMutation::Widgets(CollectionMutation::Move { id: payload.id.clone(), to_index: payload.to_index }),
        FlowMutation::ReplaceWidget(payload) => flow::FlowMutation::Widgets(CollectionMutation::Patch { id: payload.id.clone(), patch: payload.widget.clone() }),
        FlowMutation::ConnectWidgets(payload) => flow::FlowMutation::Synapses(CollectionMutation::Add {
            index: payload.index,
            item: flow::SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() },
        }),
        FlowMutation::DisconnectWidgets(payload) => flow::FlowMutation::Synapses(CollectionMutation::Remove { id: payload.id.clone() }),
        FlowMutation::ReorderSynapses(payload) => flow::FlowMutation::Synapses(CollectionMutation::Move { id: payload.id.clone(), to_index: payload.to_index }),
        FlowMutation::UpdateSynapseEndpoints(payload) => flow::FlowMutation::Synapses(CollectionMutation::Patch {
            id: payload.id.clone(),
            patch: flow::SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() },
        }),
        FlowMutation::MoveWidgets(payload) => flow::FlowMutation::SetLayout { entries: payload.entries.clone() },
        FlowMutation::DuplicateWidget(_) => return None,
    })
}
//#endregion 🌉️FrameworkBridge

//#region 🔹WireCodecs
/// 🏷️ First byte of a `DuplicateWidget` op's binary encoding — reserved so it can never collide with
/// `store::os_dsl::variants_binary::OP_BINARY_FORMAT` (always `1`), the format every framework-bridged
/// leaf op decodes through. Any composite's own bytes are canonical-JSON of its payload (the same
/// idiom `HistoryOpMeta.origin` uses for a structured, non-hot-path field), not a `flow::FlowMutation`
/// bridge — see [`to_framework_mutation`]'s doc comment for why one cannot exist.
const DUPLICATE_WIDGET_OP_BINARY_TAG: u8 = 0xD0;
const DUPLICATE_WIDGET_OP_TEXT_KEYWORD: &str = "duplicate-widget ";

impl protocol::OpBinary for FlowMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let FlowMutation::DuplicateWidget(payload) = self else {
            let framework_mutation = to_framework_mutation(self).expect("only DuplicateWidget has no framework-generic op");
            return protocol::OpBinary::encode_op(&framework_mutation);
        };
        let mut bytes = vec![DUPLICATE_WIDGET_OP_BINARY_TAG];
        bytes.extend(serde_json::to_vec(payload).map_err(|error| protocol::ProtocolError::Malformed { what: "flow.op", offset: 0, detail: format!("duplicate-widget: {error}") })?);
        Ok(bytes)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        if bytes.first() == Some(&DUPLICATE_WIDGET_OP_BINARY_TAG) {
            let payload: super::duplicate_widget::mutation::DuplicateWidget = serde_json::from_slice(&bytes[1..])
                .map_err(|error| protocol::ProtocolError::Malformed { what: "flow.op", offset: 1, detail: format!("duplicate-widget: {error}") })?;
            return Ok(FlowMutation::DuplicateWidget(payload));
        }
        let framework_mutation = <flow::FlowMutation as protocol::OpBinary>::decode_op(bytes)?;
        from_framework_mutation(framework_mutation).ok_or_else(|| protocol::ProtocolError::Malformed {
            what: "flow.op",
            offset: 0,
            detail: "set-fixture has no semantic mutation representation (whole-document replace is banned; route through ArtifactStore::reset)".into(),
        })
    }
}
impl protocol::OpText for FlowMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        if let Some(rest) = line.strip_prefix(DUPLICATE_WIDGET_OP_TEXT_KEYWORD) {
            let payload: super::duplicate_widget::mutation::DuplicateWidget =
                serde_json::from_str(rest).map_err(|error| store::TextError::new(format!("duplicate-widget: {error}"), store::TextSpan::at(1, 1)))?;
            return Ok(FlowMutation::DuplicateWidget(payload));
        }
        let framework_mutation = <flow::FlowMutation as protocol::OpText>::parse_op(line)?;
        from_framework_mutation(framework_mutation).ok_or_else(|| {
            store::TextError::new(
                "set-fixture has no semantic mutation representation (whole-document replace is banned; route through ArtifactStore::reset)",
                store::TextSpan::at(1, 1),
            )
        })
    }
    async fn print_op(&self) -> String {
        let FlowMutation::DuplicateWidget(payload) = self else {
            let framework_mutation = to_framework_mutation(self).expect("only DuplicateWidget has no framework-generic op");
            return protocol::OpText::print_op(&framework_mutation);
        };
        format!("{DUPLICATE_WIDGET_OP_TEXT_KEYWORD}{}", serde_json::to_string(payload).expect("DuplicateWidget's all-String fields always serialize"))
    }
}
//#endregion 🔹WireCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::flow::schema::mutations::connect_widgets::mutation::ConnectWidgets;
    use crate::artifacts::flow::schema::mutations::create_widget::mutation::CreateWidget;
    use crate::artifacts::flow::schema::mutations::delete_widget::mutation::DeleteWidget;
    use crate::artifacts::flow::schema::mutations::disconnect_widgets::mutation::DisconnectWidgets;
    use crate::artifacts::flow::schema::mutations::move_widgets::mutation::MoveWidgets;
    use crate::artifacts::flow::schema::mutations::replace_widget::mutation::ReplaceWidget;
    use crate::artifacts::flow::schema::mutations::reorder_synapses::mutation::ReorderSynapses;
    use crate::artifacts::flow::schema::mutations::reorder_widgets::mutation::ReorderWidgets;
    use crate::artifacts::flow::schema::mutations::update_synapse_endpoints::mutation::UpdateSynapseEndpoints;
    use flow::{FlowLayoutEntry, Widget, WidgetLayout};
    use protocol::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};

    async fn widget_note(id: &str) -> Widget {
        Widget::InputNote { id: id.into(), text: String::new() }
    }
    async fn widget_slider(id: &str) -> Widget {
        Widget::InputSlider { id: id.into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 }
    }

    async fn apply(base: &FlowSnapshot, mutation: &FlowMutation) -> FlowSnapshot {
        <FlowMutation as Mutation<FlowSnapshot>>::diff(mutation, base).diff().apply(base).expect("valid mutation diff")
    }

    async fn base_with_two_widgets() -> FlowSnapshot {
        let base = apply(&FlowSnapshot::default(), &FlowMutation::CreateWidget(CreateWidget { index: 0, widget: widget_note("w1") }));
        apply(&base, &FlowMutation::CreateWidget(CreateWidget { index: 1, widget: widget_slider("w2") }))
    }

    async fn base_with_synapse() -> FlowSnapshot {
        let base = base_with_two_widgets();
        apply(&base, &FlowMutation::ConnectWidgets(ConnectWidgets { index: 0, id: "s1".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }))
    }

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
    /// one `assert_missing_target_is_error`/Fatal check per verb family this facet implements
    /// (create/delete/connect/disconnect/move/replace/reorder/update).
    #[test]
    async fn create_widget_duplicate_id_is_fatal() {
        let base = base_with_two_widgets();
        let outcome = FlowMutation::CreateWidget(CreateWidget { index: 0, widget: widget_note("w1") }).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[test]
    async fn delete_widget_missing_target_is_error() {
        let base = FlowSnapshot::default();
        assert_missing_target_is_error(&base, &FlowMutation::DeleteWidget(DeleteWidget { id: "ghost".into() }));
    }

    #[test]
    async fn delete_widget_cascades_severed_synapses() {
        let base = base_with_synapse();
        let outcome = FlowMutation::DeleteWidget(DeleteWidget { id: "w1".into() }).diff(&base);
        assert!(outcome.messages().iter().any(|message| message.level == protocol::Severity::Info && message.code.0 == "mutation.cascade"), "deleting a widget that severs a synapse must carry an Info mutation.cascade message, got {:?}", outcome.messages());
        assert!(outcome.diff().apply(&base).is_ok());
    }

    #[test]
    async fn connect_widgets_missing_endpoint_is_error() {
        let base = base_with_two_widgets();
        assert_missing_target_is_error(&base, &FlowMutation::ConnectWidgets(ConnectWidgets { index: 0, id: "edge-99".into(), from: "ghost".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }));
    }

    #[test]
    async fn connect_widgets_duplicate_id_is_fatal() {
        let base = base_with_synapse();
        let outcome = FlowMutation::ConnectWidgets(ConnectWidgets { index: 0, id: "s1".into(), from: "w2".into(), from_port: "out".into(), to: "w1".into(), to_port: "in".into() }).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[test]
    async fn connect_widgets_parallel_is_no_op() {
        let base = base_with_synapse();
        let outcome = FlowMutation::ConnectWidgets(ConnectWidgets { index: 1, id: "s2".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
        assert_eq!(outcome.diff(), &FlowDiff::default());
    }

    #[test]
    async fn disconnect_widgets_missing_target_is_error() {
        let base = base_with_two_widgets();
        assert_missing_target_is_error(&base, &FlowMutation::DisconnectWidgets(DisconnectWidgets { id: "ghost".into() }));
    }

    #[test]
    async fn move_widgets_missing_target_is_error() {
        let base = base_with_two_widgets();
        assert_missing_target_is_error(&base, &FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "ghost".into(), layout: Some(WidgetLayout { x: 1.0, y: 1.0 }) }] }));
    }

    #[test]
    async fn move_widgets_non_finite_is_fatal() {
        let base = base_with_two_widgets();
        let outcome = FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: f64::NAN, y: 0.0 }) }] }).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[test]
    async fn move_widgets_unchanged_is_no_op() {
        let base = base_with_two_widgets();
        let moved = apply(&base, &FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: 5.0, y: 5.0 }) }] }));
        let outcome = FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: 5.0, y: 5.0 }) }] }).diff(&moved);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    }

    #[test]
    async fn replace_widget_missing_target_is_error() {
        let base = base_with_two_widgets();
        assert_missing_target_is_error(&base, &FlowMutation::ReplaceWidget(ReplaceWidget { id: "ghost".into(), widget: widget_note("ghost") }));
    }

    #[test]
    async fn replace_widget_unchanged_is_no_op() {
        let base = base_with_two_widgets();
        let outcome = FlowMutation::ReplaceWidget(ReplaceWidget { id: "w1".into(), widget: widget_note("w1") }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    }

    #[test]
    async fn reorder_widgets_missing_target_is_error() {
        let base = base_with_two_widgets();
        assert_missing_target_is_error(&base, &FlowMutation::ReorderWidgets(ReorderWidgets { id: "ghost".into(), to_index: 0 }));
    }

    #[test]
    async fn reorder_widgets_already_current_is_no_op() {
        let base = base_with_two_widgets();
        let outcome = FlowMutation::ReorderWidgets(ReorderWidgets { id: "w1".into(), to_index: 0 }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    }

    #[test]
    async fn reorder_synapses_missing_target_is_error() {
        let base = base_with_synapse();
        assert_missing_target_is_error(&base, &FlowMutation::ReorderSynapses(ReorderSynapses { id: "ghost".into(), to_index: 0 }));
    }

    #[test]
    async fn reorder_synapses_already_current_is_no_op() {
        let base = base_with_synapse();
        let outcome = FlowMutation::ReorderSynapses(ReorderSynapses { id: "s1".into(), to_index: 0 }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    }

    #[test]
    async fn update_synapse_endpoints_missing_target_is_error() {
        let base = base_with_synapse();
        assert_missing_target_is_error(&base, &FlowMutation::UpdateSynapseEndpoints(UpdateSynapseEndpoints { id: "ghost".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }));
    }

    #[test]
    async fn update_synapse_endpoints_missing_endpoint_is_error() {
        let base = base_with_synapse();
        assert_missing_target_is_error(&base, &FlowMutation::UpdateSynapseEndpoints(UpdateSynapseEndpoints { id: "s1".into(), from: "ghost".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }));
    }

    #[test]
    async fn update_synapse_endpoints_unchanged_is_no_op() {
        let base = base_with_synapse();
        let outcome = FlowMutation::UpdateSynapseEndpoints(UpdateSynapseEndpoints { id: "s1".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests
