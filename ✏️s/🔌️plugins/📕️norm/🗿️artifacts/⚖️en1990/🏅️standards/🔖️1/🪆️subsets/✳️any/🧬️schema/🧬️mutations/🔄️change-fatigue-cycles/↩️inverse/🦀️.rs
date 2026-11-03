//! ↩️ `change-fatigue-cycles` inverse: restores the previous `cycles`.

use crate::change_fatigue_cycles::ChangeFatigueCycles;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueCycles, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_cycles::ChangeFatigueCycles> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFatigueCycles, base: &En1990Snapshot) -> Option<ChangeFatigueCycles> {
    let r0 = base.fatigue_spectra.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFatigueCycles { id: payload.id.clone(), new_cycles: r0.cycles })
}
