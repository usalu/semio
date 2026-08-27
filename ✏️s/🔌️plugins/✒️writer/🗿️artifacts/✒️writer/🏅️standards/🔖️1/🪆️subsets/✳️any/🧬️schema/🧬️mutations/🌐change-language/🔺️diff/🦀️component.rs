//! 🔺️ Diff fragment yielded by `ChangeLanguage`.
use super::ChangeLanguage;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse `language_id`-only delta, built directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ChangeLanguage, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
    if base.language_id == payload.new_language_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Document language is already \"{}\".", payload.new_language_id));
    }
    protocol::MutationOutcome::new(WriterDiff { language_id: Some(payload.new_language_id.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
