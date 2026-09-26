use super::ChangeUnitFb;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeUnitFb, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeUnitFb(ChangeUnitFb { index: payload.index, new_f_b_pa: base.walls[payload.index].f_b_pa })] }
}
