//! 🔩 `change-reinforcement-k` diff: patches `k` of the addressed reinforcement grade row.

use crate::change_reinforcement_k::ChangeReinforcementK;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992ReinforcementGradesPatch, En1992ReinforcementGradesRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeReinforcementK, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.reinforcement_grades.iter().find(|row| row.id == payload.grade_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Reinforcement grade {} does not exist.", payload.grade_id), [payload.grade_id.clone()]);
    };
    if r0.k == payload.new_k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The reinforcement grade already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { reinforcement_grades: Some(En1992ReinforcementGradesRows::modification(&payload.grade_id, En1992ReinforcementGradesPatch { k: Some(payload.new_k), ..Default::default() })), ..Default::default() })
}
