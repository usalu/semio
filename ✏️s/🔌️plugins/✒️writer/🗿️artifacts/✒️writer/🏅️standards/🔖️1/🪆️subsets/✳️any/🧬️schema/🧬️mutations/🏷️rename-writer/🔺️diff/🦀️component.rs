//! 🔺️ Diff fragment yielded by `RenameWriter`.
use super::mutation::RenameWriter;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse `id`-only delta, built directly from the payload — real handcrafted construction,
/// never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &RenameWriter, _base: &WriterSnapshot) -> WriterDiff {
    WriterDiff { id: Some(payload.new_id.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
