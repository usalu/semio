//! ➕️ `insert-member` — inserts one timber member at an index (clamped to the list end).
use semio_s_artifact_norm_en1995_model::{En1995Snapshot, TimberMember};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertMember { pub index: usize, pub member: TimberMember }
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for InsertMember where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_member::RemoveMember> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "member", kind: "insert-member", record: "InsertedMember" };
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1995Snapshot>>::Diff> { crate::insert_member::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::insert_member::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert member {} at #{}", self.member.id, self.index), &format!("Bauteil {} an #{} einfügen", self.member.id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.member.id.clone()] }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
