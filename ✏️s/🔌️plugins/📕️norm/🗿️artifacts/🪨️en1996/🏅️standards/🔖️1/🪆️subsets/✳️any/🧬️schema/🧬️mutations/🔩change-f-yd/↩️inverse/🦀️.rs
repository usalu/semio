use super::ChangeFYd;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeFYd, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeFYd(ChangeFYd { index: payload.index, new_f_yd_pa: base.walls[payload.index].f_yd_pa })] }
}
