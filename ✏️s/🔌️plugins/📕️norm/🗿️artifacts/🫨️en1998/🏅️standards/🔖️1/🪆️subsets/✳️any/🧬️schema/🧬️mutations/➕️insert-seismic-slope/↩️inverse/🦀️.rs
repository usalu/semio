//! ↩️ `insert-seismic-slope` inverse: removes the inserted row again.

use crate::insert_seismic_slope::InsertSeismicSlope;
use crate::remove_seismic_slope::RemoveSeismicSlope;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertSeismicSlope, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_seismic_slope::RemoveSeismicSlope> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertSeismicSlope, base: &En1998Snapshot) -> Option<RemoveSeismicSlope> {
    Some(RemoveSeismicSlope { id: payload.seismic_slope.id.clone() })
}
