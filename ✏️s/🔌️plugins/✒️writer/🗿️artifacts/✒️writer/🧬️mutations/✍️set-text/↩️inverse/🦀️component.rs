//! ↩️ Inverse for `SetText`.
use crate::artifacts::writer::mutations::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &WriterSnapshot, _text: &str) -> Vec<WriterMutation> {
    vec![WriterMutation::SetText { text: base.text.clone() }]
}
//#endregion 🔖️Inverse
