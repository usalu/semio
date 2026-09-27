//! ↩️ Inverse for `SpliceText` — reads the BASE body, never the diff.
use super::SpliceText;
use crate::schema::mutations::WriterMutation;
use crate::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo is the located inverse splice (the removed run back in place of the inserted one, context of the new body), so
/// undoing one author's run relocates past every other author's text typed around it and never removes it.
pub fn inverse(payload: &SpliceText, base: &WriterSnapshot) -> Vec<WriterMutation> {
    let inverse = payload.splice().apply(&crate::writer_text(base), semio_framework_plugin::TEXT_SPLICE_CONTEXT_SCALARS).inverse;
    vec![super::splice_text(inverse)]
}
//#endregion 🔖️Inverse
