//! 🔧 `change-generation-value` payload — sets one answer value within a generation's form-values
//! map (single-field setter on a nested-addressed target, per `📓️taxonomy.md`'s `change` row).

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

//#region 🔖️ChangeGenerationValue
/// 🔧 Nested address: outermost `id` (the generation) then `question_id` (the form field).
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeGenerationValue {
    pub id: String,
    pub question_id: String,
    pub new_value: semio_framework_value::DslValue,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for ChangeGenerationValue
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<ChangeGenerationValue>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "generation-value", kind: "change-generation-value", record: "ChangedGenerationValue" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change generation \"{}\" value \"{}\"", self.id, self.question_id), &format!("Erzeugung \"{}\": Wert \"{}\" ändern", self.id, self.question_id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone(), self.question_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for ChangeGenerationValue
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<ChangeGenerationValue>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::change_generation_value::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_generation_value::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ChangeGenerationValue
