//! ↩️ `change-roof-assumed-sk` inverse: restores the previous `assumed_sk`.

use crate::change_roof_assumed_sk::ChangeRoofAssumedSk;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofAssumedSk, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_assumed_sk::ChangeRoofAssumedSk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofAssumedSk, base: &En1991Snapshot) -> Option<ChangeRoofAssumedSk> {
    let r0 = base.roofs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofAssumedSk { id: payload.id.clone(), new_assumed_sk: r0.assumed_sk })
}
