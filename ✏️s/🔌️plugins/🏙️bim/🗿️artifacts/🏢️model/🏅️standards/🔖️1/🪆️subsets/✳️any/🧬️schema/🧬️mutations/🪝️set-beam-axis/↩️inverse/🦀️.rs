//! ↩️ Inverse of `SetBeamAxis`: an absolute `SetBeamAxis` back to the base axis, none when the beam is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_beam_axis::SetBeamAxis;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetBeamAxis, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_beam_axis::SetBeamAxis>,
{
    match base.beams.get(&payload.id) {
        Some(beam) => vec![Op::from(SetBeamAxis { id: payload.id.clone(), axis: beam.axis.clone() })],
        None => Vec::new(),
    }
}
