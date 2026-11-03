//! 🔺️ Diff for `change-stud-height-m`.
use crate::change_stud_height_m::ChangeStudHeightM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994StudsRows, En1994StudsPatch};

pub fn diff(payload: &ChangeStudHeightM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.studs.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shear stud {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_height_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [index.to_string()]);
    }
    let Some(stud) = base.studs.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "Shear stud missing", [index.to_string()]);
    };
    if (stud.height_m - payload.new_height_m).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        studs: Some(En1994StudsRows::modification(&payload.id, En1994StudsPatch { height_m: Some(payload.new_height_m), ..Default::default() })),
        ..Default::default()
    })
}
