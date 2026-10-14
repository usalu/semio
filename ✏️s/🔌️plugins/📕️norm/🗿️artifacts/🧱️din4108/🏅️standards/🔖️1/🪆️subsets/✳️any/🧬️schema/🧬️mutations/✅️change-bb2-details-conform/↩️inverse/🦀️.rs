//! ↩️ `change-bb2-details-conform` inverse.

use crate::change_bb2_details_conform::ChangeBb2DetailsConform;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(_payload: &ChangeBb2DetailsConform, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bb2_details_conform::ChangeBb2DetailsConform> {
    Ok((|| {
    vec![Op::from(ChangeBb2DetailsConform { new_bb2_details_conform: base.bb2_details_conform })]

    })())
}
