use super::ChangeLayerPhiPrime;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(payload: &ChangeLayerPhiPrime, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let v = base.layers.iter().find(|f| f.id == payload.id).map(|f| f.phi_prime_deg).unwrap_or(payload.new_phi_prime_deg);
    vec![En1997Mutation::ChangeLayerPhiPrime(ChangeLayerPhiPrime { id: payload.id.clone(), new_phi_prime_deg: v })]
}
