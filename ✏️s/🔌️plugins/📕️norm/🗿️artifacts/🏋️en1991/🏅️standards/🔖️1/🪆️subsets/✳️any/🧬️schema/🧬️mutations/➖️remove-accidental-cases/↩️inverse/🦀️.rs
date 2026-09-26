//! Inverse for `remove-accidental-cases`.
use super::RemoveAccidentalCases;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &RemoveAccidentalCases, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    if payload.index >= base.accidental_cases.len() { return Vec::new(); }
    let item = base.accidental_cases[payload.index].clone();
    vec![En1991Mutation::InsertAccidentalCases(crate::mutations::insert_accidental_cases::InsertAccidentalCases { index: payload.index, item })]
}
