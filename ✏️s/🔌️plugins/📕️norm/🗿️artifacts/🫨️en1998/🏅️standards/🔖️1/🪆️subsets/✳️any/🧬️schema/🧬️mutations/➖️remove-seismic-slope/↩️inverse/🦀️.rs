//! ↩️ `remove-seismic-slope` inverse: inserts the removed row back at its position.

use crate::remove_seismic_slope::RemoveSeismicSlope;
use crate::insert_seismic_slope::InsertSeismicSlope;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveSeismicSlope, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_seismic_slope::InsertSeismicSlope> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveSeismicSlope, base: &En1998Snapshot) -> Option<InsertSeismicSlope> {
    let (index, row) = base.slopes.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertSeismicSlope { index: Some(index), seismic_slope: row.clone() })
}
