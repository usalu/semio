//! ↩️ `change-footbridge-natural-frequency` inverse: restores the previous `natural_frequency`.

use crate::change_footbridge_natural_frequency::ChangeFootbridgeNaturalFrequency;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFootbridgeNaturalFrequency, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_footbridge_natural_frequency::ChangeFootbridgeNaturalFrequency> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFootbridgeNaturalFrequency, base: &En1991Snapshot) -> Option<ChangeFootbridgeNaturalFrequency> {
    let r0 = base.footbridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFootbridgeNaturalFrequency { id: payload.id.clone(), new_natural_frequency: r0.natural_frequency })
}
