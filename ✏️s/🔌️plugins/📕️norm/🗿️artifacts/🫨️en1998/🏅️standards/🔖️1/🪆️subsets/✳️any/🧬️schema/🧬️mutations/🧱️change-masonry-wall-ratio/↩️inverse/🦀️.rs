//! Inverse for `change-masonry-wall-ratio`.
use crate::change_masonry_wall_ratio::ChangeMasonryWallRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMasonryWallRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_masonry_wall_ratio::ChangeMasonryWallRatio> {
    Ok((|| {
    match base.buildings.get(payload.building_index) {
        Some(b) => vec![Op::from(ChangeMasonryWallRatio { building_index: payload.building_index, new_masonry_wall_area_ratio: b.masonry_wall_area_ratio })],
        None => Vec::new(),
    }

    })())
}
