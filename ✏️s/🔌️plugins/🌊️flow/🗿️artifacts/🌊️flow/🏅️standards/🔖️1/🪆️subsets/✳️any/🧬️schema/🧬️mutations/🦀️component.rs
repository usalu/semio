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
pub fn apply_flow_mutation(snapshot: &mut FlowSnapshot, mutation: &FlowMutation) {
    *snapshot = <FlowMutation as Mutation<FlowSnapshot>>::diff(mutation, snapshot).apply(snapshot);
}

/// ↩️ Inverse mutations for undo.
pub fn inverse_flow_mutation(snapshot: &FlowSnapshot, mutation: &FlowMutation) -> Vec<FlowMutation> {
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
pub fn snapshot_operations(before: &FlowSnapshot, after: &FlowSnapshot) -> Vec<FlowMutation> {
    flow::flow_fixture_operations(&before.to_fixture(), &after.to_fixture()).into_iter().filter_map(from_framework_mutation).collect()
}

pub fn from_framework_mutation(mutation: flow::FlowMutation) -> Option<FlowMutation> {
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
pub fn to_framework_mutation(mutation: &FlowMutation) -> Option<flow::FlowMutation> {
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
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let FlowMutation::DuplicateWidget(payload) = self else {
            let framework_mutation = to_framework_mutation(self).expect("only DuplicateWidget has no framework-generic op");
            return protocol::OpBinary::encode_op(&framework_mutation);
        };
        let mut bytes = vec![DUPLICATE_WIDGET_OP_BINARY_TAG];
        bytes.extend(serde_json::to_vec(payload).map_err(|error| protocol::ProtocolError::Malformed { what: "flow.op", offset: 0, detail: format!("duplicate-widget: {error}") })?);
        Ok(bytes)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
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
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
    fn print_op(&self) -> String {
        let FlowMutation::DuplicateWidget(payload) = self else {
            let framework_mutation = to_framework_mutation(self).expect("only DuplicateWidget has no framework-generic op");
            return protocol::OpText::print_op(&framework_mutation);
        };
        format!("{DUPLICATE_WIDGET_OP_TEXT_KEYWORD}{}", serde_json::to_string(payload).expect("DuplicateWidget's all-String fields always serialize"))
    }
}
//#endregion 🔹WireCodecs
