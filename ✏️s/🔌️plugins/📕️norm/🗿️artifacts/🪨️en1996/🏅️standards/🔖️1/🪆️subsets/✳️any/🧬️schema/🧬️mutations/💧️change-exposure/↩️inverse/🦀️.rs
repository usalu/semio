use super::ChangeExposure;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeExposure, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeExposure(ChangeExposure { index: payload.index, new_exposure: base.walls[payload.index].exposure })] }
}
