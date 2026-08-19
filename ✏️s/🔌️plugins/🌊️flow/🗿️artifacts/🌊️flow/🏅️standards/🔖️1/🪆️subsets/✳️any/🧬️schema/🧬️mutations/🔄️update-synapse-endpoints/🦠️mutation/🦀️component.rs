//! 🔄️ Atomically updates a synapse's endpoints (from/to widget + port) — an inseparable facet,
//! never meaningfully set one field at a time (taxonomy `update` verb).
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{FlowDiff, FlowSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔄️UpdateSynapseEndpoints
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    async fn diff(&self, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &FlowSnapshot) -> Vec<FlowMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Update synapse \"{}\" endpoints", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔄️UpdateSynapseEndpoints
