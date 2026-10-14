use crate::change_layer_phi_prime::ChangeLayerPhiPrime;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &ChangeLayerPhiPrime, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_phi_prime::ChangeLayerPhiPrime> {
    Ok((|| {
    let v = base.layers.iter().find(|f| f.id == payload.id).map(|f| f.phi_prime_deg).unwrap_or(payload.new_phi_prime_deg);
    vec![Op::from(ChangeLayerPhiPrime { id: payload.id.clone(), new_phi_prime_deg: v })]

    })())
}
