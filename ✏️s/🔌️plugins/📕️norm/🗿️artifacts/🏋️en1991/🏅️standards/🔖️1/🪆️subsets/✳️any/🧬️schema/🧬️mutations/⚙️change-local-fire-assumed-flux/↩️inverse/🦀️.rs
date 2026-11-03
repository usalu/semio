//! ↩️ `change-local-fire-assumed-flux` inverse: restores the previous `assumed_flux`.

use crate::change_local_fire_assumed_flux::ChangeLocalFireAssumedFlux;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLocalFireAssumedFlux, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_local_fire_assumed_flux::ChangeLocalFireAssumedFlux> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLocalFireAssumedFlux, base: &En1991Snapshot) -> Option<ChangeLocalFireAssumedFlux> {
    let r0 = base.local_fires.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLocalFireAssumedFlux { id: payload.id.clone(), new_assumed_flux: r0.assumed_flux })
}
