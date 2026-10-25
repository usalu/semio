//! 🔗 `connect-synapse` payload — brings a new [`SynapseSpec`] edge into existence between two
//! widget ports (relationship collection, per `📓️derivation-rules.md` rule 4:
//! `connect-<nouns>{endpoints,payload}` ↔ `disconnect-<noun>{id}`).

use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::SynapseSpec;
//#region 🔖️ConnectSynapse
/// 🔗 Full initial payload for a new synapse edge, placed at `index` (FINAL-state) if no edge with
/// the same id already exists.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ConnectSynapse {
    pub index: usize,
    pub synapse: SynapseSpec,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for ConnectSynapse
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<crate::disconnect_synapse::DisconnectSynapse>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "synapse", kind: "connect-synapse", record: "ConnectedSynapse" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect synapse \"{}\"", self.synapse.id), &format!("Synapse \"{}\" verbinden", self.synapse.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.synapse.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for ConnectSynapse
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<crate::disconnect_synapse::DisconnectSynapse>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::connect_synapse::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::connect_synapse::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ConnectSynapse
