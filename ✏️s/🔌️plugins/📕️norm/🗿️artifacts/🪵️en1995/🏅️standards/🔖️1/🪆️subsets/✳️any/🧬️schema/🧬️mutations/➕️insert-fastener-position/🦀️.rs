//! ➕️ `insert-fastener-position` — inserts one position into the positions of one fastener at an index (clamped to the list end).
use semio_s_artifact_norm_en1995_model::{En1995Snapshot, FastenerPosition};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertFastenerPosition { pub fastener_id: String, pub index: usize, pub position: FastenerPosition }
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for InsertFastenerPosition where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_fastener_position::RemoveFastenerPosition> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "fastener-position", kind: "insert-fastener-position", record: "InsertedFastenerPosition" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert position {} into fastener {} at #{}", self.position.id, self.fastener_id, self.index), &format!("Lage {} in Verbindungsmittelbild {} an #{} einfügen", self.position.id, self.fastener_id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.fastener_id.clone(), self.position.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1995Snapshot, Op> for InsertFastenerPosition where Op: protocol::ColdMutation<En1995Snapshot, ReferenceDiff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_fastener_position::RemoveFastenerPosition> {
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1995Snapshot>>::ReferenceDiff> { crate::insert_fastener_position::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::insert_fastener_position::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
