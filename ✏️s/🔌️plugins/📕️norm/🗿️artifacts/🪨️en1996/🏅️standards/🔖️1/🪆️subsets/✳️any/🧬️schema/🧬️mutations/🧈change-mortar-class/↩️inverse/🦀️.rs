use super::ChangeMortarClass;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeMortarClass, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeMortarClass(ChangeMortarClass { index: payload.index, new_mortar_class: base.walls[payload.index].mortar_class })] }
}
