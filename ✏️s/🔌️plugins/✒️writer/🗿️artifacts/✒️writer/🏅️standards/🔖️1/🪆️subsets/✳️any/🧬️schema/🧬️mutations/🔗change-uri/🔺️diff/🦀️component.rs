//! 🔺️ Diff fragment yielded by `ChangeUri`.
use super::mutation::ChangeUri;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse `uri`-only delta, built directly from the payload — real handcrafted construction,
/// never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ChangeUri, _base: &WriterSnapshot) -> WriterDiff {
    WriterDiff { uri: Some(payload.new_uri.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
