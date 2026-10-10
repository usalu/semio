//! 🔧 `change-generation-value` payload — sets one answer value within a generation's form-values
//! map (single-field setter on a nested-addressed target, per `📓️taxonomy.md`'s `change` row).

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

//#region 🔖️ChangeGenerationValue
/// 🔧 Nested address: outermost `id` (the generation) then `question_id` (the form field).
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeGenerationValue {
    pub id: String,
    pub question_id: String,
    pub new_value: semio_framework_value::DslValue,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for ChangeGenerationValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "generation-value", kind: "change-generation-value", record: "ChangedGenerationValue" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::change_generation_value::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::change_generation_value::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change generation \"{}\" value \"{}\"", self.id, self.question_id), &format!("Erzeugung \"{}\": Wert \"{}\" ändern", self.id, self.question_id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone(), self.question_id.clone()]
    }
}
//#endregion 🔖️ChangeGenerationValue
