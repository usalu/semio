//! Inverse for `insert-accidental-cases`.
use super::InsertAccidentalCases;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &InsertAccidentalCases, _base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::RemoveAccidentalCases(crate::mutations::remove_accidental_cases::RemoveAccidentalCases { index: payload.index })]
}
