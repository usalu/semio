//! 🧬️ Flow artifact — typed invertible mutations over [`FlowSnapshot`].

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::flow::schema::diff::text::{
    diff_set_snapshot, synapses_delta_from_collection_mutation, widgets_delta_from_collection_mutation, FlowDiff,
    FlowLayoutMapDelta,
};
use crate::artifacts::flow::FlowSnapshot;
use flow::{FlowLayoutEntry, SynapseSpec, Widget};
use protocol::{inverse_collection_mutation, CollectionMutation, Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔹Operation
/// 🌊️ Typed, invertible flow-document operation owned by this plugin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum FlowMutation {
    Widgets(CollectionMutation<String, Widget, Widget>),
    Synapses(CollectionMutation<String, SynapseSpec, SynapseSpec>),
    SetLayout { entries: Vec<FlowLayoutEntry> },
    SetSnapshot { snapshot: FlowSnapshot },
}

impl Mutation<FlowSnapshot> for FlowMutation {
    type Diff = FlowDiff;

    fn diff(&self, snapshot: &FlowSnapshot) -> FlowDiff {
        match self {
            FlowMutation::Widgets(operation) => FlowDiff {
                widgets: Some(widgets_delta_from_collection_mutation(&snapshot.widgets, operation)),
                ..Default::default()
            },
            FlowMutation::Synapses(operation) => FlowDiff {
                synapses: Some(synapses_delta_from_collection_mutation(&snapshot.synapses, operation)),
                ..Default::default()
            },
            FlowMutation::SetLayout { entries } => FlowDiff {
                layout: Some(FlowLayoutMapDelta {
                    entries: entries
                        .iter()
                        .map(|entry| (entry.id.clone(), entry.layout.clone()))
                        .collect(),
                }),
                ..Default::default()
            },
            FlowMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &FlowSnapshot) -> Vec<Self> {
        match self {
            FlowMutation::Widgets(operation) => {
                vec![FlowMutation::Widgets(inverse_collection_mutation(&snapshot.widgets, operation))]
            }
            FlowMutation::Synapses(operation) => {
                vec![FlowMutation::Synapses(inverse_collection_mutation(&snapshot.synapses, operation))]
            }
            FlowMutation::SetLayout { entries } => vec![FlowMutation::SetLayout {
                entries: entries
                    .iter()
                    .map(|entry| FlowLayoutEntry {
                        id: entry.id.clone(),
                        layout: snapshot.layout.get(&entry.id).cloned(),
                    })
                    .collect(),
            }],
            FlowMutation::SetSnapshot { .. } => {
                vec![FlowMutation::SetSnapshot {
                    snapshot: snapshot.clone(),
                }]
            }
        }
    }
}

pub type FlowEnvelope = DocumentEnvelope<FlowSnapshot, FlowMutation>;
pub type FlowStore = DocumentStore<FlowSnapshot, FlowMutation>;

/// 🌈️ Applies a mutation onto a snapshot in place.
pub fn apply_flow_mutation(snapshot: &mut FlowSnapshot, mutation: &FlowMutation) {
    *snapshot = <FlowMutation as Mutation<FlowSnapshot>>::diff(mutation, snapshot).apply(snapshot);
}

/// ↩️ Inverse mutations for undo.
pub fn inverse_flow_mutation(snapshot: &FlowSnapshot, mutation: &FlowMutation) -> Vec<FlowMutation> {
    <FlowMutation as Mutation<FlowSnapshot>>::inverse(mutation, snapshot)
}

/// 🌎️ Converts a framework kernel mutation into this plugin's mutation enum.
pub fn from_framework_mutation(mutation: flow::FlowMutation) -> FlowMutation {
    match mutation {
        flow::FlowMutation::Widgets(op) => FlowMutation::Widgets(op),
        flow::FlowMutation::Synapses(op) => FlowMutation::Synapses(op),
        flow::FlowMutation::SetLayout { entries } => FlowMutation::SetLayout { entries },
        flow::FlowMutation::SetFixture { fixture } => FlowMutation::SetSnapshot {
            snapshot: FlowSnapshot::from_fixture(fixture),
        },
    }
}

/// 🌎️ Converts this plugin mutation into the framework kernel mutation enum.
pub fn to_framework_mutation(mutation: &FlowMutation) -> flow::FlowMutation {
    match mutation {
        FlowMutation::Widgets(op) => flow::FlowMutation::Widgets(op.clone()),
        FlowMutation::Synapses(op) => flow::FlowMutation::Synapses(op.clone()),
        FlowMutation::SetLayout { entries } => flow::FlowMutation::SetLayout {
            entries: entries.clone(),
        },
        FlowMutation::SetSnapshot { snapshot } => flow::FlowMutation::SetFixture {
            fixture: snapshot.to_fixture(),
        },
    }
}
//#endregion 🔹Operation


//#region 🔹WireCodecs
impl protocol::OpBinary for FlowMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        protocol::OpBinary::encode_op(&to_framework_mutation(self))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        <flow::FlowMutation as protocol::OpBinary>::decode_op(bytes).map(from_framework_mutation)
    }
}
impl protocol::OpText for FlowMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        <flow::FlowMutation as protocol::OpText>::parse_op(line).map(from_framework_mutation)
    }
    fn print_op(&self) -> String {
        protocol::OpText::print_op(&to_framework_mutation(self))
    }
}
//#endregion 🔹WireCodecs
