//! 🔗️ Creates a synapse edge between two widget ports. Modeled as a relationship-collection verb
//! (taxonomy `derivation-rules.md` §4) rather than generic collection create, since a `SynapseSpec`
//! is literally an edge between two widget ports.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{FlowDiff, FlowSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔗️ConnectWidgets
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectWidgets {
    pub index: usize,
    pub id: String,
    pub from: String,
    pub from_port: String,
    pub to: String,
    pub to_port: String,
}

impl MutationKind<FlowSnapshot, FlowMutation> for ConnectWidgets {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "synapse", kind: "connect-widgets", record: "ConnectedWidgets" };

    fn diff(&self, base: &FlowSnapshot) -> FlowDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &FlowSnapshot) -> Vec<FlowMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Connect \"{}\" to \"{}\"", self.from, self.to)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔗️ConnectWidgets
