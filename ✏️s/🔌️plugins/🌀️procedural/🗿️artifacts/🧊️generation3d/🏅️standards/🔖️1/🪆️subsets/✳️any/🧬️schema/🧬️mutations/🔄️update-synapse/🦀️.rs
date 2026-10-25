//! 🔁 `update-synapse` payload — atomically replaces an EXISTING [`SynapseSpec`] edge's
//! endpoints/ports (cohesive multi-field facet, per `📓️taxonomy.md`'s `update` row).

use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::SynapseSpec;
//#region 🔖️UpdateSynapse
/// 🔁 The synapse's own `id` addresses the target.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateSynapse {
    pub synapse: SynapseSpec,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for UpdateSynapse
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<UpdateSynapse>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "synapse", kind: "update-synapse", record: "UpdatedSynapse" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update synapse \"{}\"", self.synapse.id), &format!("Synapse \"{}\" aktualisieren", self.synapse.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.synapse.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for UpdateSynapse
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<UpdateSynapse>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::update_synapse::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_synapse::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️UpdateSynapse
