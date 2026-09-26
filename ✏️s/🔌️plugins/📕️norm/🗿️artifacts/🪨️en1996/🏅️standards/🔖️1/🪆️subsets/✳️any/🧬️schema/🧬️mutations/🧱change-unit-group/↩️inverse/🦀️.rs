use super::ChangeUnitGroup;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeUnitGroup, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeUnitGroup(ChangeUnitGroup { index: payload.index, new_unit_group: base.walls[payload.index].unit_group })] }
}
