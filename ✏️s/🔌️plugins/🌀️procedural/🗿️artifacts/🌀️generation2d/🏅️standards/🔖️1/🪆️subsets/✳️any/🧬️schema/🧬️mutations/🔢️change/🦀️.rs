//! 🦠️ `🔢️change` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use crate::standards::v1::subsets::any::schema::diff::Generation2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeGenerationValue {
    pub id: String,
    pub question_id: String,
    pub value: semio_framework_value::DslValue,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_generation_value(id: String, question_id: String, value: semio_framework_value::DslValue) -> Generation2dMutation {
    Generation2dMutation::ChangeGenerationValue(ChangeGenerationValue { id, question_id, value })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for ChangeGenerationValue {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "generation-value", kind: "change-generation-value", record: "ChangedGenerationValue" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Generation2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change generation \"{}\" value \"{}\"", self.id, self.question_id), &format!("Erzeugung \"{}\": Wert \"{}\" ändern", self.id, self.question_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone(), self.question_id.clone()]
    }
}
//#endregion 🔖️Mutation
