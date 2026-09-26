use super::ChangeMu;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeMu, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeMu(ChangeMu { index: payload.index, new_mu: base.walls[payload.index].mu })] }
}
