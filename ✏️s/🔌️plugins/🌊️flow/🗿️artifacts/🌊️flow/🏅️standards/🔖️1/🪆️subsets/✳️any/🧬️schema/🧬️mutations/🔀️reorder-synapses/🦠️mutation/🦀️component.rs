//! 🔀️ Repositions a synapse within the ordered synapse list.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{FlowDiff, FlowSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔀️ReorderSynapses
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderSynapses {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<FlowSnapshot, FlowMutation> for ReorderSynapses {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "synapse", kind: "reorder-synapses", record: "ReorderedSynapses" };

    fn diff(&self, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &FlowSnapshot) -> Vec<FlowMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Reorder synapse \"{}\" to {}", self.id, self.to_index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔀️ReorderSynapses
