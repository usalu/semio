use crate::change_cement_type::ChangeCementType;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(_payload: &ChangeCementType, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cement_type::ChangeCementType> {
    Ok((|| {
    vec![Op::from(ChangeCementType { new_cement_type: base.cement_type.clone() })]

    })())
}
