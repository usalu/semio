//! ↩️ `change-accelerogram-samples` inverse: restores the previous `samples`.

use crate::change_accelerogram_samples::ChangeAccelerogramSamples;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAccelerogramSamples, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accelerogram_samples::ChangeAccelerogramSamples> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccelerogramSamples, base: &En1998Snapshot) -> Option<ChangeAccelerogramSamples> {
    let r0 = base.accelerograms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAccelerogramSamples { id: payload.id.clone(), new_samples: r0.samples.clone() })
}
