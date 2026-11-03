//! ↩️ `change-bridge-natural-frequency` inverse: restores the previous `natural_frequency`.

use crate::change_bridge_natural_frequency::ChangeBridgeNaturalFrequency;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeNaturalFrequency, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_natural_frequency::ChangeBridgeNaturalFrequency> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeNaturalFrequency, base: &En1990Snapshot) -> Option<ChangeBridgeNaturalFrequency> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeNaturalFrequency { id: payload.id.clone(), new_natural_frequency: r0.natural_frequency })
}
