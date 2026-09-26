use super::ChangeWallBaseWidth;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(payload: &ChangeWallBaseWidth, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let w = base.retaining_walls.iter().find(|f| f.id == payload.id).map(|f| f.base_width).unwrap_or(payload.new_base_width);
    vec![En1997Mutation::ChangeWallBaseWidth(ChangeWallBaseWidth { id: payload.id.clone(), new_base_width: w })]
}
