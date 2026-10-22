//! 🦠️ `🔢️change` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
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
pub fn change_generation_value<Op: From<ChangeGenerationValue>>(id: String, question_id: String, value: semio_framework_value::DslValue) -> Op {
    Op::from(ChangeGenerationValue { id, question_id, value })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for ChangeGenerationValue
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<ChangeGenerationValue>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "generation-value", kind: "change-generation-value", record: "ChangedGenerationValue" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change generation \"{}\" value \"{}\"", self.id, self.question_id), &format!("Erzeugung \"{}\": Wert \"{}\" ändern", self.id, self.question_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone(), self.question_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation2dSnapshot, Op> for ChangeGenerationValue
where
    Op: protocol::ColdMutation<Generation2dSnapshot, ReferenceDiff = Generation2dDiff> + From<ChangeGenerationValue>,
{
    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::change_generation_value::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_generation_value::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
