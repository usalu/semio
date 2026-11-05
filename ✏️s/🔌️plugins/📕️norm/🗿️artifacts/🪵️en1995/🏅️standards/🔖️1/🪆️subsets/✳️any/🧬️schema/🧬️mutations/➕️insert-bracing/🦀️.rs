//! ➕️ `insert-bracing` — inserts one bracing at an index (clamped to the list end).
use semio_s_artifact_norm_en1995_model::{En1995Snapshot, Bracing};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertBracing { pub index: usize, pub bracing: Bracing }
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for InsertBracing where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_bracing::RemoveBracing> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "bracing", kind: "insert-bracing", record: "InsertedBracing" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Insert bracing {} at #{}", self.bracing.id, self.index), &format!("Bracing {} an #{} einfügen", self.bracing.id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.bracing.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1995Snapshot, Op> for InsertBracing where Op: protocol::ColdMutation<En1995Snapshot, ReferenceDiff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_bracing::RemoveBracing> {
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1995Snapshot>>::ReferenceDiff> { crate::insert_bracing::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::insert_bracing::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
