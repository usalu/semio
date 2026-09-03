//! ✂️ `disconnect-synapse` payload — severs a [`SynapseSpec`] edge by id.
//!
//! Directory kept at its pre-migration `➖remove-synapse` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️DisconnectSynapse
/// ✂️ Severs the synapse edge with `id`; diff/inverse leaves capture the full removed edge from
/// `base` so undo is a real `connect-synapse`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DisconnectSynapse {
    pub id: String,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for DisconnectSynapse {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "synapse", kind: "disconnect-synapse", record: "DisconnectedSynapse" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::artifacts::generation3d::mutations::disconnect_synapse::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::artifacts::generation3d::mutations::disconnect_synapse::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Disconnect synapse \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DisconnectSynapse
