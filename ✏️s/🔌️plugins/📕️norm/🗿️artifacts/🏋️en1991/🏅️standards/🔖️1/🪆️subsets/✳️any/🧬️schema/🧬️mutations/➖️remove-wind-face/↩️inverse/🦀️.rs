//! ↩️ `remove-wind-face` inverse: inserts the removed row back at its position.

use crate::remove_wind_face::RemoveWindFace;
use crate::insert_wind_face::InsertWindFace;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveWindFace, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_wind_face::InsertWindFace> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveWindFace, base: &En1991Snapshot) -> Option<InsertWindFace> {
    let (index, row) = base.wind_faces.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertWindFace { index, item: row.clone() })
}
