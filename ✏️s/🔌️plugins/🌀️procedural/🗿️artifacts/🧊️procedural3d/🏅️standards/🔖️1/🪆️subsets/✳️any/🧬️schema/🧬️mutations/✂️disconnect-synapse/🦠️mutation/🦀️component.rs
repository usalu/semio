//! ✂️ `disconnect-synapse` payload — severs a [`SynapseSpec`] edge by id.
//!
//! Directory kept at its pre-migration `➖remove-synapse` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️DisconnectSynapse
/// ✂️ Severs the synapse edge with `id`; diff/inverse leaves capture the full removed edge from
/// `base` so undo is a real `connect-synapse`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectSynapse {
    pub id: String}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for DisconnectSynapse {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "synapse", kind: "disconnect-synapse", record: "DisconnectedSynapse" };

    fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::disconnect_synapse::diff::diff(self, base)
    }

    fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::disconnect_synapse::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Disconnect synapse \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DisconnectSynapse
