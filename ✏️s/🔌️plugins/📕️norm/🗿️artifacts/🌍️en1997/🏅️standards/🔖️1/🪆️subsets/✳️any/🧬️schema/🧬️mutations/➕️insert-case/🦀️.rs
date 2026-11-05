//! ➕️ `insert-case` — inserts one catalogue case into the case table.
use semio_s_artifact_norm_en1997_model::{CaseRow, En1997Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertCase { pub index: usize, pub case: CaseRow }
impl<Op> protocol::MutationKind<En1997Snapshot, Op> for InsertCase where Op: protocol::Mutation<En1997Snapshot, Diff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::remove_case::RemoveCase> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "case", kind: "insert-case", record: "InsertedCase" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Insert catalogue case {} at #{}", self.case.id, self.index), &format!("Katalogfall {} an #{} einfügen", self.case.id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.case.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1997Snapshot, Op> for InsertCase where Op: protocol::ColdMutation<En1997Snapshot, ReferenceDiff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::remove_case::RemoveCase> {
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1997Snapshot>>::ReferenceDiff> { crate::insert_case::diff::diff(self, base) }
    fn inverse(&self, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::insert_case::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
