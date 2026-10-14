//! ↩️ `change-airtightness-n50` inverse.

use crate::change_airtightness_n50::ChangeAirtightnessN50;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(_payload: &ChangeAirtightnessN50, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_airtightness_n50::ChangeAirtightnessN50> {
    Ok((|| {
    vec![Op::from(ChangeAirtightnessN50 { new_airtightness_n50: base.airtightness_n50 })]

    })())
}
