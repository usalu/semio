use super::ChangeDensity;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeDensity, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else {
        vec![En1996Mutation::ChangeDensity(ChangeDensity { index: payload.index, new_density_kg_m3: base.walls[payload.index].density_kg_m3 })]
    }
}
