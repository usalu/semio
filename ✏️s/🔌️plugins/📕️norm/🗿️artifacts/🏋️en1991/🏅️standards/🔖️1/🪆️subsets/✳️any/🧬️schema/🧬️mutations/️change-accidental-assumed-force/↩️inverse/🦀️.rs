//! Inverse for `change-accidental-assumed-force`.
use super::ChangeAccidentalAssumedForce;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &ChangeAccidentalAssumedForce, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    if payload.index >= base.accidental_cases.len() { return Vec::new(); }
    vec![En1991Mutation::ChangeAccidentalAssumedForce(ChangeAccidentalAssumedForce { index: payload.index, new_assumed_force: base.accidental_cases[payload.index].impact.first().map(|i| i.assumed_force).unwrap_or(0.0) })]
}
