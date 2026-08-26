//! 🔺️ Diff fragment yielded by `RenameWriter`.
use super::mutation::RenameWriter;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse `id`-only delta, built directly from the payload — real handcrafted construction,
/// never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &RenameWriter, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
    if base.id == payload.new_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Document is already named \"{}\".", payload.new_id));
    }
    protocol::MutationOutcome::new(WriterDiff { id: Some(payload.new_id.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
