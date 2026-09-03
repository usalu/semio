//! 🔗 `connect-synapse` payload — brings a new [`SynapseSpec`] edge into existence between two
//! widget ports (relationship collection, per `📓️derivation-rules.md` rule 4:
//! `connect-<nouns>{endpoints,payload}` ↔ `disconnect-<noun>{id}`).

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use flow::SynapseSpec;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ConnectSynapse
/// 🔗 Full initial payload for a new synapse edge, placed at `index` (FINAL-state) if no edge with
/// the same id already exists.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ConnectSynapse {
    pub index: usize,
    pub synapse: SynapseSpec,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for ConnectSynapse {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "synapse", kind: "connect-synapse", record: "ConnectedSynapse" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::artifacts::generation3d::mutations::connect_synapse::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::artifacts::generation3d::mutations::connect_synapse::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Connect synapse \"{}\"", self.synapse.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.synapse.id.clone()]
    }
}
//#endregion 🔖️ConnectSynapse
