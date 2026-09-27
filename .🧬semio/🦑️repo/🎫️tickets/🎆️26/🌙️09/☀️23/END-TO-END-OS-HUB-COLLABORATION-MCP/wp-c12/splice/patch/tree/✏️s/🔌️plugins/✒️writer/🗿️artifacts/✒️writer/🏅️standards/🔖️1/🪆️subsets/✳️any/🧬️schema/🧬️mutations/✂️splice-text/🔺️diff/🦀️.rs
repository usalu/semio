//! 🔺️ Diff fragment yielded by `SpliceText`.
use super::SpliceText;
use crate::WriterDiff;
use crate::WriterSnapshot;

//#region 🔖️Diff
/// 🔺️ Relocates the splice in the BASE body by its context (`TextSplice::locate`) and hands the resulting body to the same
/// sparse `document` builder `EditText` uses (`diff_set_text`: a fresh content-addressed child handle with its local text owner).
/// An empty splice is a no-op; a splice whose `deleted` run is gone deletes nothing and is reported `mutation.clamped`.
pub fn diff(payload: &SpliceText, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
    let current = crate::writer_text(base);
    let applied = payload.splice().apply(&current, semio_framework_plugin::TEXT_SPLICE_CONTEXT_SCALARS);
    if applied.text == current {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Document text is unchanged.".to_string());
    }
    let outcome = protocol::MutationOutcome::new(crate::standards::v1::subsets::any::io::diff::text::diff_set_text(&applied.text, &base.id, &base.language_id));
    if applied.located.clamped {
        return outcome.warn("mutation.clamped", "The text this edit replaced had already changed; nothing was deleted.".to_string());
    }
    outcome
}
//#endregion 🔖️Diff
