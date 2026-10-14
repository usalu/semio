//! ✂️ Generation2d mutation — `DisconnectSynapse`: removes an id-keyed edge (captures the removed
//! synapse for its inverse). Wired module name (`remove_synapse`) is a leftover of the pre-semantic
//! generic slot this triad was repurposed from — see `sharedFileRequests` in this ticket's wave2
//! report for the glue.rs rename that would align the directory/module with the verb.

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️DisconnectSynapse
/// ✂️ `disconnect-synapse` payload — removes the edge with `id`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DisconnectSynapse {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_synapse<Op: From<DisconnectSynapse>>(id: String) -> Op {
    Op::from(DisconnectSynapse { id })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for DisconnectSynapse
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<crate::connect_synapse::ConnectSynapse>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "disconnect", entity: "synapse", kind: "disconnect-synapse", record: "DisconnectedSynapse" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::disconnect_synapse::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::disconnect_synapse::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Disconnect synapse \"{}\"", self.id), &format!("Synapse \"{}\" trennen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DisconnectSynapse
