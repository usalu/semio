//! 🦠️ `🔄️replace` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_artifact_flow_flow::SynapseSpec;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReplaceSynapse {
    pub synapse: SynapseSpec,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_synapse<Op: From<ReplaceSynapse>>(synapse: SynapseSpec) -> Op {
    Op::from(ReplaceSynapse { synapse })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for ReplaceSynapse
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<ReplaceSynapse>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "synapse", kind: "replace-synapse", record: "ReplacedSynapse" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::replace_synapse::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::replace_synapse::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace synapse \"{}\"", self.synapse.id), &format!("Synapse \"{}\" ersetzen", self.synapse.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.synapse.id.clone()]
    }
}
//#endregion 🔖️Mutation
