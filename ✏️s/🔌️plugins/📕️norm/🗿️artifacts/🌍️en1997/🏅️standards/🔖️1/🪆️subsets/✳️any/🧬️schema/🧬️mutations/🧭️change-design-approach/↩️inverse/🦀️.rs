use crate::change_design_approach::ChangeDesignApproach;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(_payload: &ChangeDesignApproach, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_design_approach::ChangeDesignApproach> {
    Ok((|| {
    vec![Op::from(ChangeDesignApproach { new_design_approach: base.design_approach.clone() })]

    })())
}
