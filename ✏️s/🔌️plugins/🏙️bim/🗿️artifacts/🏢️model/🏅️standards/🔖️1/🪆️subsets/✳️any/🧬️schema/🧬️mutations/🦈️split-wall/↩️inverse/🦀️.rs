//! ↩️ Inverse of `SplitWall` in storage order, replayed reversed by the store: the original axis is restored first, then every handed-over
//! opening moves back onto the original wall at its base offset (`move-opening` re-hosts with the fit and overlap validation), and the
//! created wall, which hosts nothing by then, is deleted last.

use semio_s_artifact_bim_model_mut_any_61::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall;
use semio_s_artifact_bim_model_mut_any_79::standards::v1::subsets::any::schema::mutations::move_opening::MoveOpening;
use semio_s_artifact_bim_model_mut_any_161::standards::v1::subsets::any::schema::mutations::set_wall_axis::SetWallAxis;
use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry::split;
use crate::standards::v1::subsets::any::schema::mutations::split_wall::SplitWall;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SplitWall, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_61::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall> + From<semio_s_artifact_bim_model_mut_any_79::standards::v1::subsets::any::schema::mutations::move_opening::MoveOpening> + From<semio_s_artifact_bim_model_mut_any_161::standards::v1::subsets::any::schema::mutations::set_wall_axis::SetWallAxis>,
{
    let Some(wall) = base.walls.get(&payload.id) else {
        return Vec::new();
    };
    let Some(parts) = split(&wall.axis, payload.t) else {
        return Vec::new();
    };
    let handed = base.openings.iter().filter(|(_, opening)| opening.host == payload.id && opening.offset >= parts.at);
    std::iter::once(Op::from(DeleteWall { id: payload.new_id.clone() }))
        .chain(handed.map(|(id, opening)| Op::from(MoveOpening { id: id.clone(), offset: opening.offset, host: Some(payload.id.clone()) })))
        .chain(std::iter::once(Op::from(SetWallAxis { id: payload.id.clone(), axis: wall.axis.clone() })))
        .collect()
}
