//! 🔗 Generation2d mutation — `ConnectSynapse`: creates a new edge between two widget ports at a
//! FINAL-state insertion index. Wired module name (`set_synapse`) is a leftover of the pre-semantic
//! generic slot this triad was repurposed from — see `sharedFileRequests` in this ticket's wave2
//! report for the glue.rs rename that would align the directory/module with the verb.

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_artifact_flow_flow::SynapseSpec;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ConnectSynapse
/// 🔗 `connect-synapse` payload — the full new edge plus a FINAL-state insertion index.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ConnectSynapse {
    pub index: usize,
    pub synapse: SynapseSpec,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_synapse<Op: From<ConnectSynapse>>(index: usize, synapse: SynapseSpec) -> Op {
    Op::from(ConnectSynapse { index, synapse })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for ConnectSynapse
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<crate::disconnect_synapse::DisconnectSynapse>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "synapse", kind: "connect-synapse", record: "ConnectedSynapse" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::connect_synapse::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::connect_synapse::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect synapse \"{}\" ({} → {})", self.synapse.id, self.synapse.from, self.synapse.to), &format!("Synapse \"{}\" ({} → {}) verbinden", self.synapse.id, self.synapse.from, self.synapse.to))
    }
    fn target(&self) -> Vec<String> {
        vec![self.synapse.id.clone()]
    }
}
//#endregion 🔖️ConnectSynapse
