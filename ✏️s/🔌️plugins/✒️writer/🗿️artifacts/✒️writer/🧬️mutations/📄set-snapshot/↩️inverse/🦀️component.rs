//! ↩️ Inverse for `SetSnapshot`.
use crate::artifacts::writer::mutations::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &WriterSnapshot, _replacement: &WriterSnapshot) -> Vec<WriterMutation> {
    vec![WriterMutation::SetSnapshot { snapshot: base.clone() }]
}
//#endregion 🔖️Inverse
