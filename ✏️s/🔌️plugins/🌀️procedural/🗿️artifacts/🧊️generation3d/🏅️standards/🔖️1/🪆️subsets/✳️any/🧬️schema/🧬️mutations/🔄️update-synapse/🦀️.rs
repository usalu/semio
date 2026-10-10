//! 🔁 `update-synapse` payload — atomically replaces an EXISTING [`SynapseSpec`] edge's
//! endpoints/ports (cohesive multi-field facet, per `📓️taxonomy.md`'s `update` row).

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::SynapseSpec;
//#region 🔖️UpdateSynapse
/// 🔁 The synapse's own `id` addresses the target.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateSynapse {
    pub synapse: SynapseSpec,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for UpdateSynapse {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "synapse", kind: "update-synapse", record: "UpdatedSynapse" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::update_synapse::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::update_synapse::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update synapse \"{}\"", self.synapse.id), &format!("Synapse \"{}\" aktualisieren", self.synapse.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.synapse.id.clone()]
    }
}
//#endregion 🔖️UpdateSynapse
