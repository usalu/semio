//! 🏷️ `change-case-label-en` — changes `labelEn` (Label (EN)) on one addressed catalogue case.
use semio_s_artifact_norm_en1997_model::{En1997Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeCaseLabelEn { pub case_id: String, pub new_value: String }
impl<Op> protocol::MutationKind<En1997Snapshot, Op> for ChangeCaseLabelEn where Op: protocol::Mutation<En1997Snapshot, Diff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::change_case_label_en::ChangeCaseLabelEn> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "case-labelEn", kind: "change-case-label-en", record: "ChangedCaseLabelEn" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Change Label (EN) of case {}", self.case_id), &format!("Bezeichnung (EN) von Fall {} ändern", self.case_id)) }
    fn target(&self) -> Vec<String> { vec![self.case_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1997Snapshot, Op> for ChangeCaseLabelEn where Op: protocol::ColdMutation<En1997Snapshot, ReferenceDiff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::change_case_label_en::ChangeCaseLabelEn> {
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1997Snapshot>>::ReferenceDiff> { crate::change_case_label_en::diff::diff(self, base) }
    fn inverse(&self, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_case_label_en::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
