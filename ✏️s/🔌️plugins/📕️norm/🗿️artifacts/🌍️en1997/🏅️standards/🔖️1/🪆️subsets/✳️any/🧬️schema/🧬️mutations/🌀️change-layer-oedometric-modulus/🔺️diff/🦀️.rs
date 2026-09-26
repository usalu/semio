use super::ChangeLayerOedometricModulus;
use crate::diff::{En1997Diff, En1997SoilLayerList};
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeLayerOedometricModulus, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_oedometric_modulus.is_finite() || payload.new_oedometric_modulus <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "E_oed must be positive", vec![payload.id.clone()]);
    }
    let Some(idx) = base.layers.iter().position(|f| f.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("layer {} missing", payload.id), vec![payload.id.clone()]);
    };
    let mut layers = base.layers.clone();
    layers[idx].oedometric_modulus = payload.new_oedometric_modulus;
    protocol::MutationOutcome::new(En1997Diff { layers: Some(En1997SoilLayerList { values: layers }), ..Default::default() })
}
