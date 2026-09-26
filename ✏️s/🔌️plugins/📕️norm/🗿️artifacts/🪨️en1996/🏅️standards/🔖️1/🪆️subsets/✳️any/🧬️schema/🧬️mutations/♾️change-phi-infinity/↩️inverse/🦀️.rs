use super::ChangePhiInfinity;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangePhiInfinity, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else {
        vec![En1996Mutation::ChangePhiInfinity(ChangePhiInfinity { index: payload.index, new_phi_infinity: base.walls[payload.index].phi_infinity })]
    }
}
