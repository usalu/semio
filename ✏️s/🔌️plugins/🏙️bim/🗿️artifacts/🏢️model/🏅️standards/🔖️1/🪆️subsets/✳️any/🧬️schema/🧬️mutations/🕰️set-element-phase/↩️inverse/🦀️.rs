//! ↩️ Inverse of `SetElementPhase`: an absolute `SetElementPhase` back to the phase the element was in on the base, none when the element is absent or carries no phase.

use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::set_element_phase::SetElementPhase;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetElementPhase, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_element_phase::SetElementPhase>,
{
    match elements::phase_of(base, &payload.id) {
        Some(phase) => vec![Op::from(SetElementPhase { id: payload.id.clone(), phase })],
        None => Vec::new(),
    }
}
