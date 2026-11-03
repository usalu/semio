//! ↩️ `change-accelerogram-label` inverse: restores the previous `label`.

use crate::change_accelerogram_label::ChangeAccelerogramLabel;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAccelerogramLabel, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accelerogram_label::ChangeAccelerogramLabel> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccelerogramLabel, base: &En1998Snapshot) -> Option<ChangeAccelerogramLabel> {
    let r0 = base.accelerograms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAccelerogramLabel { id: payload.id.clone(), new_label: r0.label.clone() })
}
