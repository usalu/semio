//! 🔺️ Diff fragment yielded by `EditText`.
use super::mutation::EditText;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Diff
/// 🔺️ Whole-body text replacement delta — delegates to the pre-existing sparse-diff builder in
/// `🔺️diff/📝️text` (real handcrafted construction, never apply-then-capture; that builder mints a
/// new content-addressed `document` child handle and seeds the working-scene cache with the real
/// text, rather than diffing character-by-character — see that file's `🔖️Builders` doc comment).
pub fn diff(payload: &EditText, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
    if crate::artifacts::writer::writer_text(base) == payload.text {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Document text is unchanged.".to_string());
    }
    protocol::MutationOutcome::new(crate::artifacts::writer::schema::diff::text::diff_set_text(&payload.text, &base.id, &base.language_id))
}
//#endregion 🔖️Diff
