//! 🔁 `update-synapse` payload — atomically replaces an EXISTING [`SynapseSpec`] edge's
//! endpoints/ports (cohesive multi-field facet, per `📓️taxonomy.md`'s `update` row).
//!
//! Directory kept at its pre-migration `🎛set-synapse` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::SynapseSpec;
use serde::{Deserialize, Serialize};

//#region 🔖️UpdateSynapse
/// 🔁 The synapse's own `id` addresses the target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSynapse {
    pub synapse: SynapseSpec}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for UpdateSynapse {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "synapse", kind: "update-synapse", record: "UpdatedSynapse" };

    async fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::update_synapse::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::update_synapse::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Update synapse \"{}\"", self.synapse.id)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.synapse.id.clone()]
    }
}
//#endregion 🔖️UpdateSynapse
