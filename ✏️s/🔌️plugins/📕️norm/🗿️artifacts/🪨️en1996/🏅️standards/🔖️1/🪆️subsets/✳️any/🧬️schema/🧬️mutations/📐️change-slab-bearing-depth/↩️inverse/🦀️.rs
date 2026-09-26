use super::ChangeSlabBearingDepth;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeSlabBearingDepth, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeSlabBearingDepth(ChangeSlabBearingDepth { index: payload.index, new_slab_bearing_depth_m: base.walls[payload.index].slab_bearing_depth_m })] }
}
