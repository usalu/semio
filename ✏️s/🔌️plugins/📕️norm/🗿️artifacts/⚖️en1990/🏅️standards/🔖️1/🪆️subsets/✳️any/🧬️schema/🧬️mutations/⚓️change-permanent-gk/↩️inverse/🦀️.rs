//! ↩️ `change-permanent-gk` inverse: restores the previous `gk`.

use crate::change_permanent_gk::ChangePermanentGk;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangePermanentGk, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_permanent_gk::ChangePermanentGk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePermanentGk, base: &En1990Snapshot) -> Option<ChangePermanentGk> {
    let r0 = base.permanents.iter().find(|row| row.id == payload.id)?;
    Some(ChangePermanentGk { id: payload.id.clone(), new_gk: r0.gk })
}
