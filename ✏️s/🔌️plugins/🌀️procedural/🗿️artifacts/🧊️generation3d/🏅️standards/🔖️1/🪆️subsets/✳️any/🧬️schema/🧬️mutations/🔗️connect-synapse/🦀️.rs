//! 🔗 `connect-synapse` payload — brings a new [`SynapseSpec`] edge into existence between two
//! widget ports (relationship collection, per `📓️derivation-rules.md` rule 4:
//! `connect-<nouns>{endpoints,payload}` ↔ `disconnect-<noun>{id}`).

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::SynapseSpec;
//#region 🔖️ConnectSynapse
/// 🔗 Full initial payload for a new synapse edge, placed at `index` (FINAL-state) if no edge with
/// the same id already exists.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ConnectSynapse {
    pub index: usize,
    pub synapse: SynapseSpec,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for ConnectSynapse {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "synapse", kind: "connect-synapse", record: "ConnectedSynapse" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::connect_synapse::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::connect_synapse::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect synapse \"{}\"", self.synapse.id), &format!("Synapse \"{}\" verbinden", self.synapse.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.synapse.id.clone()]
    }
}
//#endregion 🔖️ConnectSynapse
