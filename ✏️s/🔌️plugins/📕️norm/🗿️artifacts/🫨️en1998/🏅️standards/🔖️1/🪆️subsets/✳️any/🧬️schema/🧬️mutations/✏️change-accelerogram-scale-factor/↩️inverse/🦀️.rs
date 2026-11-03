//! ↩️ `change-accelerogram-scale-factor` inverse: restores the previous `scale_factor`.

use crate::change_accelerogram_scale_factor::ChangeAccelerogramScaleFactor;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAccelerogramScaleFactor, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accelerogram_scale_factor::ChangeAccelerogramScaleFactor> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccelerogramScaleFactor, base: &En1998Snapshot) -> Option<ChangeAccelerogramScaleFactor> {
    let r0 = base.accelerograms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAccelerogramScaleFactor { id: payload.id.clone(), new_scale_factor: r0.scale_factor })
}
