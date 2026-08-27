//! 🔺️ Diff fragment yielded by `EditText`.
use super::EditText;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Diff
/// 🔺️ Whole-body text replacement delta — delegates to the pre-existing sparse-diff builder in
/// `🔺️diff/📝️text` (real handcrafted construction, never apply-then-capture; that builder mints a
/// new content-addressed `document` child handle and attaches its local text owner with the real
/// text, rather than diffing character-by-character — see that file's `🔖️Builders` doc comment).
pub fn diff(payload: &EditText, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
    if crate::artifacts::writer::writer_text(base) == payload.text {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Document text is unchanged.".to_string());
    }
    protocol::MutationOutcome::new(crate::artifacts::writer::standards::v1::subsets::any::io::diff::text::diff_set_text(&payload.text, &base.id, &base.language_id))
}
//#endregion 🔖️Diff
