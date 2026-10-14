use crate::change_density::ChangeDensity;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeDensity, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_density::ChangeDensity> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else {
        vec![Op::from(ChangeDensity { index: payload.index, new_density_kg_m3: base.walls[payload.index].density_kg_m3 })]
    }

    })())
}
