//! ↩️ `insert-soil-layer` inverse: removes the inserted row again.

use crate::insert_soil_layer::InsertSoilLayer;
use crate::remove_soil_layer::RemoveSoilLayer;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertSoilLayer, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_soil_layer::RemoveSoilLayer> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertSoilLayer, base: &En1998Snapshot) -> Option<RemoveSoilLayer> {
    Some(RemoveSoilLayer { id: payload.layer.id.clone() })
}
