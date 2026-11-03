//! ↩️ `remove-soil-layer` inverse: inserts the removed row back at its position.

use crate::remove_soil_layer::RemoveSoilLayer;
use crate::insert_soil_layer::InsertSoilLayer;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveSoilLayer, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_soil_layer::InsertSoilLayer> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveSoilLayer, base: &En1998Snapshot) -> Option<InsertSoilLayer> {
    let (index, row) = base.soil_layers.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertSoilLayer { index: Some(index), layer: row.clone() })
}
