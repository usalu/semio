use super::ChangeFm;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeFm, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeFm(ChangeFm { index: payload.index, new_mortar_strength_pa: base.walls[payload.index].mortar_strength_pa })] }
}
