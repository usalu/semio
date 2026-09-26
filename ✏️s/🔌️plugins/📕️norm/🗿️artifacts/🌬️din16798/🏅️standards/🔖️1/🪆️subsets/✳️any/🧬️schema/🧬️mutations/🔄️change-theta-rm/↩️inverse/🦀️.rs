//! ↩️ `change-theta-rm` inverse.
use super::ChangeThetaRm;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(_payload: &ChangeThetaRm, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeThetaRm(ChangeThetaRm { new_theta_rm_c: base.theta_rm_c })]
}
