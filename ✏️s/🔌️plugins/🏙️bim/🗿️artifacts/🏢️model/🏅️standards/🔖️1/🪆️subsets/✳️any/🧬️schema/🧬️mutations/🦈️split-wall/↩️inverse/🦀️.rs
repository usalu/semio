//! ↩️ Inverse of `SplitWall` in storage order, replayed reversed by the store: the original axis is restored first, then every handed-over
//! opening moves back onto the original wall at its base offset (`move-opening` re-hosts with the fit and overlap validation), and the
//! created wall, which hosts nothing by then, is deleted last.

use super::super::delete_wall::DeleteWall;
use super::super::move_opening::MoveOpening;
use super::super::set_wall_axis::SetWallAxis;
use super::super::wall_geometry::split;
use super::SplitWall;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SplitWall, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return Vec::new();
    };
    let Some(parts) = split(&wall.axis, payload.t) else {
        return Vec::new();
    };
    let handed = base.openings.iter().filter(|(_, opening)| opening.host == payload.id && opening.offset >= parts.at);
    std::iter::once(ModelMutation::DeleteWall(DeleteWall { id: payload.new_id.clone() }))
        .chain(handed.map(|(id, opening)| ModelMutation::MoveOpening(MoveOpening { id: id.clone(), offset: opening.offset, host: Some(payload.id.clone()) })))
        .chain(std::iter::once(ModelMutation::SetWallAxis(SetWallAxis { id: payload.id.clone(), axis: wall.axis.clone() })))
        .collect()
}
