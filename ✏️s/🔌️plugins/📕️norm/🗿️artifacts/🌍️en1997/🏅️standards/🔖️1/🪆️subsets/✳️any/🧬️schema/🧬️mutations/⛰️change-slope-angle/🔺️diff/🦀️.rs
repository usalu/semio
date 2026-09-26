use super::ChangeSlopeAngle;
use crate::diff::{En1997Diff, En1997SlopeList};
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeSlopeAngle, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_angle_deg.is_finite() || payload.new_angle_deg <= 0.0 || payload.new_angle_deg >= 90.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "slope angle must be in (0,90)", vec![payload.id.clone()]);
    }
    let Some(idx) = base.slopes.iter().position(|f| f.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("slope {} missing", payload.id), vec![payload.id.clone()]);
    };
    let mut slopes = base.slopes.clone();
    slopes[idx].angle_deg = payload.new_angle_deg;
    protocol::MutationOutcome::new(En1997Diff { slopes: Some(En1997SlopeList { values: slopes }), ..Default::default() })
}
