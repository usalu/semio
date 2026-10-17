//! ↩️ Inverse for `SpliceText` — reads the BASE body, never the diff.
use super::SpliceText;
use crate::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo is the located inverse splice (the removed run back in place of the inserted one, context of the new body), so
/// undoing one author's run relocates past every other author's text typed around it and never removes it.
pub fn inverse<Op>(payload: &SpliceText, base: &WriterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::splice_text::SpliceText>,
{
    Ok((|| {
    let inverse = payload.splice().splice_into(&crate::writer_text(base), semio_framework_plugin::TEXT_SPLICE_CONTEXT_SCALARS).inverse;
    vec![super::splice_text(inverse)]

    })())
}
//#endregion 🔖️Inverse
