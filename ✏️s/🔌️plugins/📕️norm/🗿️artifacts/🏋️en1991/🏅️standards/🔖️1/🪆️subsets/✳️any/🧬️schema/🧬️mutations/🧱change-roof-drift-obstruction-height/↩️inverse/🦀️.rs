//! ↩️ `change-roof-drift-obstruction-height` inverse: restores the previous `drift_obstruction_height`.

use crate::change_roof_drift_obstruction_height::ChangeRoofDriftObstructionHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofDriftObstructionHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_drift_obstruction_height::ChangeRoofDriftObstructionHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofDriftObstructionHeight, base: &En1991Snapshot) -> Option<ChangeRoofDriftObstructionHeight> {
    let r0 = base.roofs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofDriftObstructionHeight { id: payload.id.clone(), new_drift_obstruction_height: r0.drift_obstruction_height })
}
