//! 🔺️ Diff fragment yielded by `EditText`.
use super::mutation::EditText;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Diff
/// 🔺️ Whole-body text replacement delta — delegates to the pre-existing sparse-diff builder in
/// `🔺️diff/📝️text` (real handcrafted construction, never apply-then-capture; that builder already
/// wraps the replacement in a `WriterTextDelta` rather than diffing character-by-character).
pub fn diff(payload: &EditText, _base: &WriterSnapshot) -> WriterDiff {
    crate::artifacts::writer::schema::diff::text::diff_set_text(&payload.text)
}
//#endregion 🔖️Diff
