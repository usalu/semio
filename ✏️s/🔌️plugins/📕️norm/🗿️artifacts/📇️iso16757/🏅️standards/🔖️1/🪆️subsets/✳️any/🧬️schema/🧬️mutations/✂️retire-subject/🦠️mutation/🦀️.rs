//! 🗑️ `retire-subject` — removes an id-keyed dictionary subject.

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RetireSubject {
    pub id: String,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for RetireSubject where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::introduce_subject::mutation::IntroduceSubject> {
    const SEMANTICS: protocol::SemanticDescriptor =

        protocol::SemanticDescriptor { verb: "remove", entity: "subject", kind: "retire-subject", record: "RetiredSubject" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete dictionary subject \"{}\"", self.id), &format!("Wörterbuchthema \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Iso16757Snapshot, Op> for RetireSubject where Op: protocol::ColdMutation<Iso16757Snapshot, ReferenceDiff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::introduce_subject::mutation::IntroduceSubject> {
    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Iso16757Snapshot>>::ReferenceDiff> {
        crate::retire_subject::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::retire_subject::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
