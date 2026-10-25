//! ✂️ `disconnect-synapse` payload — severs a [`SynapseSpec`] edge by id.

use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
//#region 🔖️DisconnectSynapse
/// ✂️ Severs the synapse edge with `id`; diff/inverse leaves capture the full removed edge from
/// `base` so undo is a real `connect-synapse`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DisconnectSynapse {
    pub id: String,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for DisconnectSynapse
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<crate::connect_synapse::ConnectSynapse>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "synapse", kind: "disconnect-synapse", record: "DisconnectedSynapse" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Disconnect synapse \"{}\"", self.id), &format!("Synapse \"{}\" trennen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for DisconnectSynapse
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<crate::connect_synapse::ConnectSynapse>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::disconnect_synapse::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::disconnect_synapse::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️DisconnectSynapse
