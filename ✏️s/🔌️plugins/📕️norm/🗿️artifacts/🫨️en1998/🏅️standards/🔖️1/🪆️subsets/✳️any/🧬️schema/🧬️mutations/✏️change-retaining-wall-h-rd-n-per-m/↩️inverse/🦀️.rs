//! ↩️ `change-retaining-wall-h-rd-n-per-m` inverse: restores the previous `h_rd_n_per_m`.

use crate::change_retaining_wall_h_rd_n_per_m::ChangeRetainingWallHRdNPerM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeRetainingWallHRdNPerM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_retaining_wall_h_rd_n_per_m::ChangeRetainingWallHRdNPerM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRetainingWallHRdNPerM, base: &En1998Snapshot) -> Option<ChangeRetainingWallHRdNPerM> {
    let r0 = base.retaining_walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRetainingWallHRdNPerM { id: payload.id.clone(), new_h_rd_n_per_m: r0.h_rd_n_per_m })
}
