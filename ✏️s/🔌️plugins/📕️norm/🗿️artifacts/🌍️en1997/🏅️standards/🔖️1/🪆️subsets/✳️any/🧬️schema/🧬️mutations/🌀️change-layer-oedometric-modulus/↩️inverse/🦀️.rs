use super::ChangeLayerOedometricModulus;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(payload: &ChangeLayerOedometricModulus, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let v = base.layers.iter().find(|f| f.id == payload.id).map(|f| f.oedometric_modulus).unwrap_or(payload.new_oedometric_modulus);
    vec![En1997Mutation::ChangeLayerOedometricModulus(ChangeLayerOedometricModulus { id: payload.id.clone(), new_oedometric_modulus: v })]
}
