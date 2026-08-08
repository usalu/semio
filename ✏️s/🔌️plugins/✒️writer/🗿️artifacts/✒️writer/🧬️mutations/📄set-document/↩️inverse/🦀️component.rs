//! ↩️ Inverse for `SetDocument`.
use crate::artifacts::writer::mutations::WriterMutation;
use crate::artifacts::writer::WriterProjection;

//#region 🔖️Inverse
pub fn inverse(base: &WriterProjection, _replacement: &WriterProjection) -> Vec<WriterMutation> {
    vec![WriterMutation::SetDocument { document: base.clone() }]
}
//#endregion 🔖️Inverse
