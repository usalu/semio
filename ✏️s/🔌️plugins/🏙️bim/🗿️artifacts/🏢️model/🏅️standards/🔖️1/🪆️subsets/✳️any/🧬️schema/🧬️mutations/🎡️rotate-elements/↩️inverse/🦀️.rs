//! ↩️ Inverse of `RotateElements`: one `PlaceElements` carrying the exact base placement of every element the rotation changes (rotation
//! fields included), read from the base through the shared element vocabulary, so no floating-point residue of a back rotation remains.
//! Empty when the pivot or angle is not finite, the angle is zero or no placement changes.

use semio_s_artifact_bim_model_mut_any_84::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements;
use crate::standards::v1::subsets::any::schema::mutations::rotate_elements::RotateElements;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &RotateElements, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_84::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    let (pivot, angle) = (payload.pivot, payload.angle);
    if !(pivot.x.is_finite() && pivot.y.is_finite() && angle.is_finite()) || angle == 0.0 {
        return Vec::new();
    }
    match semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::changes(base, &payload.ids, |placement| placement.rotated(pivot, angle)) {
        Ok(change) if !change.before.is_empty() => vec![Op::from(PlaceElements { placements: change.before })],
        _ => Vec::new(),
    }
}
