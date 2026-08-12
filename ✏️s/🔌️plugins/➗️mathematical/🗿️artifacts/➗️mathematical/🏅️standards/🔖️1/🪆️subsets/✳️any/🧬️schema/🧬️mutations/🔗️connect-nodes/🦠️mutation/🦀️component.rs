//! 🔗️ `connect-nodes` — creates an edge relationship between two graph nodes (the node-graph
//! canvas's `connect` edit op).

use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectNodes {
    pub id: String,
    pub source: String,
    pub target: String,
}

impl protocol::MutationKind<MathematicalSnapshot, MathematicalMutation> for ConnectNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "node", kind: "connect-nodes", record: "ConnectedNodes" };

    fn diff(&self, base: &MathematicalSnapshot) -> <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Connect \"{}\" to \"{}\"", self.source, self.target)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
