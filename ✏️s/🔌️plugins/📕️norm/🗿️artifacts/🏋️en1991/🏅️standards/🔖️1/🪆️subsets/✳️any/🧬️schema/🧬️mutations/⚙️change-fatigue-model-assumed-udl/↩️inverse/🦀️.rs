//! ↩️ `change-fatigue-model-assumed-udl` inverse: restores the previous `assumed_udl`.

use crate::change_fatigue_model_assumed_udl::ChangeFatigueModelAssumedUdl;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueModelAssumedUdl, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_model_assumed_udl::ChangeFatigueModelAssumedUdl> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFatigueModelAssumedUdl, base: &En1991Snapshot) -> Option<ChangeFatigueModelAssumedUdl> {
    let r0 = base.fatigue_models.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFatigueModelAssumedUdl { id: payload.id.clone(), new_assumed_udl: r0.assumed_udl })
}
