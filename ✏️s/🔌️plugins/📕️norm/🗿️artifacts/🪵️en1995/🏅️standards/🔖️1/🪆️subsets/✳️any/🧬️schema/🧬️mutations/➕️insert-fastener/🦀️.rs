//! ➕️ `insert-fastener` — inserts one fastener at an index (clamped to the list end).
use semio_s_artifact_norm_en1995_model::{En1995Snapshot, Fastener};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertFastener { pub index: usize, pub fastener: Fastener }
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for InsertFastener where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_fastener::RemoveFastener> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "fastener", kind: "insert-fastener", record: "InsertedFastener" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Insert fastener {} at #{}", self.fastener.id, self.index), &format!("Fastener {} an #{} einfügen", self.fastener.id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.fastener.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1995Snapshot, Op> for InsertFastener where Op: protocol::ColdMutation<En1995Snapshot, ReferenceDiff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_fastener::RemoveFastener> {
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1995Snapshot>>::ReferenceDiff> { crate::insert_fastener::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::insert_fastener::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
