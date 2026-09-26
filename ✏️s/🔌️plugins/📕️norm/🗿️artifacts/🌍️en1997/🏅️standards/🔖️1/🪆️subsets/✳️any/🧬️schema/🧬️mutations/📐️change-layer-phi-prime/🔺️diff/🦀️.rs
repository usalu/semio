use super::ChangeLayerPhiPrime;
use crate::diff::{En1997Diff, En1997SoilLayerList};
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeLayerPhiPrime, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_phi_prime_deg.is_finite() || payload.new_phi_prime_deg < 0.0 || payload.new_phi_prime_deg > 50.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "phi' out of range", vec![payload.id.clone()]);
    }
    let Some(idx) = base.layers.iter().position(|f| f.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("layer {} missing", payload.id), vec![payload.id.clone()]);
    };
    let mut layers = base.layers.clone();
    layers[idx].phi_prime_deg = payload.new_phi_prime_deg;
    protocol::MutationOutcome::new(En1997Diff { layers: Some(En1997SoilLayerList { values: layers }), ..Default::default() })
}
