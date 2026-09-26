use super::ChangeUnitMaterial;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeUnitMaterial, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeUnitMaterial(ChangeUnitMaterial { index: payload.index, new_unit_material: base.walls[payload.index].unit_material })] }
}
