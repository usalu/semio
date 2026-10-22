//! ➕ `create-generation` payload — brings a new id-keyed [`FormGeneration`] into existence.
//! Delegates to `semio_framework_artifact_playbook_playbook`'s existing `GenerationMutation::Add` engine (framework territory,
//! out of this facet's writable boundary) via the sibling `🔺️diff`/`↩️inverse` leaves.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_playbook_playbook::FormGeneration;
//#region 🔖️CreateGeneration
/// ➕ Full initial payload for a new generation.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateGeneration {
    pub generation: FormGeneration,
    /// 📍 Zero-based insertion position among the generations; `None` or past the end appends.
    pub index: Option<usize>,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for CreateGeneration
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<crate::delete_generation::DeleteGeneration>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "generation", kind: "create-generation", record: "CreatedGeneration" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create generation \"{}\"", self.generation.name), &format!("Erzeugung \"{}\" erstellen", self.generation.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.generation.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for CreateGeneration
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<crate::delete_generation::DeleteGeneration>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::create_generation::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::create_generation::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️CreateGeneration
