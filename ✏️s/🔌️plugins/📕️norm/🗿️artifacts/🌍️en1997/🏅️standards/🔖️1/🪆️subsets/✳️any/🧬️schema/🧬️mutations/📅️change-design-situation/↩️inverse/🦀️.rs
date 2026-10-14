use crate::change_design_situation::ChangeDesignSituation;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(_payload: &ChangeDesignSituation, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_design_situation::ChangeDesignSituation> {
    Ok((|| {
    vec![Op::from(ChangeDesignSituation { new_design_situation: base.design_situation.clone() })]

    })())
}
