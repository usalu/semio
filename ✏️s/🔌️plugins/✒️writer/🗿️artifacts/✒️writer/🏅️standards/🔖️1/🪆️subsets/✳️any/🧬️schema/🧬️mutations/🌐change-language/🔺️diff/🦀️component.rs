//! 🔺️ Diff fragment yielded by `ChangeLanguage`.
use super::mutation::ChangeLanguage;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse `language_id`-only delta, built directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ChangeLanguage, _base: &WriterSnapshot) -> WriterDiff {
    WriterDiff { language_id: Some(payload.new_language_id.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
