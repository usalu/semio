//! ↩️ `change-masonry-wall-ratio` inverse: restores the previous `masonry_wall_area_ratio`.

use crate::change_masonry_wall_ratio::ChangeMasonryWallRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMasonryWallRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_masonry_wall_ratio::ChangeMasonryWallRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMasonryWallRatio, base: &En1998Snapshot) -> Option<ChangeMasonryWallRatio> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMasonryWallRatio { id: payload.id.clone(), new_masonry_wall_area_ratio: r0.masonry_wall_area_ratio })
}
