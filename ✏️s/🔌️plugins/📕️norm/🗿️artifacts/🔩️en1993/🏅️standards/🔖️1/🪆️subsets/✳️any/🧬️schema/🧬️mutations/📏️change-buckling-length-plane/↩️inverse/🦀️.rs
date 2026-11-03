//! ↩️ `change-buckling-length-plane` inverse: restores the previous `plane`.

use crate::change_buckling_length_plane::ChangeBucklingLengthPlane;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBucklingLengthPlane, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_buckling_length_plane::ChangeBucklingLengthPlane> {
    Ok(match base.buckling_lengths.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBucklingLengthPlane { id: payload.id.clone(), new_plane: row.plane })],
        None => Vec::new(),
    })
}
