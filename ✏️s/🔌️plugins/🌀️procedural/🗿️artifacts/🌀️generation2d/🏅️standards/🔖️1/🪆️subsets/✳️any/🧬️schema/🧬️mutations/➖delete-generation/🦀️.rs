//! 🦠️ `➖delete-generation` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteGeneration {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_generation<Op: From<DeleteGeneration>>(id: String) -> Op {
    Op::from(DeleteGeneration { id })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for DeleteGeneration
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<crate::create_generation::CreateGeneration> + From<crate::select_generation::SelectGeneration>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "generation", kind: "delete-generation", record: "DeletedGeneration" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete generation \"{}\"", self.id), &format!("Erzeugung \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation2dSnapshot, Op> for DeleteGeneration
where
    Op: protocol::ColdMutation<Generation2dSnapshot, ReferenceDiff = Generation2dDiff> + From<crate::create_generation::CreateGeneration> + From<crate::select_generation::SelectGeneration>,
{
    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::delete_generation::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::delete_generation::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
