//! 🔺️ Diff for `change-stud-fu-pa`.
use crate::change_stud_f_u_pa::ChangeStudFUPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994StudsRows, En1994StudsPatch};

pub fn diff(payload: &ChangeStudFUPa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.studs.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shear stud {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_f_u_pa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [index.to_string()]);
    }
    let Some(stud) = base.studs.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "Shear stud missing", [index.to_string()]);
    };
    if (stud.f_u_pa - payload.new_f_u_pa).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        studs: Some(En1994StudsRows::modification(&payload.id, En1994StudsPatch { f_u_pa: Some(payload.new_f_u_pa), ..Default::default() })),
        ..Default::default()
    })
}
