//! 🚗 `change-accidental-assumed-pressure` diff: patches `assumed_pressure` of the explosion block of the addressed accidental case.

use crate::change_accidental_assumed_pressure::ChangeAccidentalAssumedPressure;
use semio_s_artifact_norm_en1991_diff::{En1991AccidentalCaseDelta, En1991AccidentalCasePatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeAccidentalAssumedPressure, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_pressure.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.accidental_cases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Accidental case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some(current) = r0.explosion.first() else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The accidental case has no explosion block.", [payload.id.clone()]);
    };
    if current.assumed_pressure == payload.new_assumed_pressure {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The accidental case already has this value.");
    }
    let mut explosion = r0.explosion.clone();
    explosion[0].assumed_pressure = payload.new_assumed_pressure;
    protocol::MutationOutcome::new(En1991Diff { accidental_cases: En1991AccidentalCaseDelta::modification(&payload.id, En1991AccidentalCasePatch { explosion: Some(explosion), ..Default::default() }), ..Default::default() })
}
