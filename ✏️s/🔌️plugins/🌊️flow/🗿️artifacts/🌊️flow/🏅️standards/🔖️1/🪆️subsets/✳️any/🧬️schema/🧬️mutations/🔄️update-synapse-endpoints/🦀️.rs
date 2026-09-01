//! 🔄️ Atomically updates a synapse's endpoints (from/to widget + port) — an inseparable facet,
//! never meaningfully set one field at a time (taxonomy `update` verb).

use crate::artifacts::flow::FlowSnapshot;
use crate::artifacts::flow::schema::diff::text::FlowDiff;
use crate::artifacts::flow::schema::mutations::FlowMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔄️UpdateSynapseEndpoints
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSynapseEndpoints {
    pub id: String,
    pub from: String,
    pub from_port: String,
    pub to: String,
    pub to_port: String,
}

impl MutationKind<FlowSnapshot, FlowMutation> for UpdateSynapseEndpoints {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "synapse", kind: "update-synapse-endpoints", record: "UpdatedSynapseEndpoints" };

    fn diff(&self, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &FlowSnapshot) -> Vec<FlowMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Update synapse \"{}\" endpoints", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔄️UpdateSynapseEndpoints
