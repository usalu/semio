//! 🔁 `update-synapse` payload — atomically replaces an EXISTING [`SynapseSpec`] edge's
//! endpoints/ports (cohesive multi-field facet, per `📓️taxonomy.md`'s `update` row).
//!
//! Directory kept at its pre-migration `🎛set-synapse` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use flow::SynapseSpec;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️UpdateSynapse
/// 🔁 The synapse's own `id` addresses the target.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateSynapse {
    pub synapse: SynapseSpec,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for UpdateSynapse {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "synapse", kind: "update-synapse", record: "UpdatedSynapse" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::artifacts::generation3d::mutations::update_synapse::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::artifacts::generation3d::mutations::update_synapse::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Update synapse \"{}\"", self.synapse.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.synapse.id.clone()]
    }
}
//#endregion 🔖️UpdateSynapse
