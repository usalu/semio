//! ↩️ `change-seismic-a-ek` inverse: restores the previous `a_ek`.

use crate::change_seismic_a_ek::ChangeSeismicAEk;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeSeismicAEk, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_seismic_a_ek::ChangeSeismicAEk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSeismicAEk, base: &En1990Snapshot) -> Option<ChangeSeismicAEk> {
    let r0 = base.seismics.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSeismicAEk { id: payload.id.clone(), new_a_ek: r0.a_ek })
}
