//! Inverse for `change-accidental-assumed-force`.
use crate::change_accidental_assumed_force::ChangeAccidentalAssumedForce;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &ChangeAccidentalAssumedForce, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accidental_assumed_force::ChangeAccidentalAssumedForce> {
    Ok((|| {
    if payload.index >= base.accidental_cases.len() { return Vec::new(); }
    vec![Op::from(ChangeAccidentalAssumedForce { index: payload.index, new_assumed_force: base.accidental_cases[payload.index].impact.first().map(|i| i.assumed_force).unwrap_or(0.0) })]

    })())
}
