//! ↩️ Inverse for `SetText`.
use crate::artifacts::writer::mutations::WriterMutation;
use crate::artifacts::writer::WriterProjection;

//#region 🔖️Inverse
pub fn inverse(base: &WriterProjection, _text: &str) -> Vec<WriterMutation> {
    vec![WriterMutation::SetText { text: base.text.clone() }]
}
//#endregion 🔖️Inverse
