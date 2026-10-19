//! ↩️ Inverse of `SetElementPhase`: an absolute `SetElementPhase` back to the phase the element was in on the base, none when the element is absent or carries no phase.

use crate::standards::v1::subsets::any::schema::mutations::set_element_phase::SetElementPhase;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetElementPhase, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_element_phase::SetElementPhase>,
{
    match semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::phase_of(base, &payload.id) {
        Some(phase) => vec![Op::from(SetElementPhase { id: payload.id.clone(), phase })],
        None => Vec::new(),
    }
}
