//! 🦠️ `➕create-generation` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_artifact_playbook_playbook::FormGeneration;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateGeneration {
    pub generation: FormGeneration,
    /// 📍 Zero-based insertion position among the generations; `None` or past the end appends.
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_generation<Op: From<CreateGeneration>>(generation: FormGeneration) -> Op {
    Op::from(CreateGeneration { generation, index: None })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for CreateGeneration
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<crate::delete_generation::DeleteGeneration>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "generation", kind: "create-generation", record: "CreatedGeneration" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::create_generation::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::create_generation::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create generation \"{}\"", self.generation.name), &format!("Erzeugung \"{}\" erstellen", self.generation.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.generation.id.clone()]
    }
}
//#endregion 🔖️Mutation
