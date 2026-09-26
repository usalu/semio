use super::ChangeMortarType;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeMortarType, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeMortarType(ChangeMortarType { index: payload.index, new_mortar_type: base.walls[payload.index].mortar_type })] }
}
