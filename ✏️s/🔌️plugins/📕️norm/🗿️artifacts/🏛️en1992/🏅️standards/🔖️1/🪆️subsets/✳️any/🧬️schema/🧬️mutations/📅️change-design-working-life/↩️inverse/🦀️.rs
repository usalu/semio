use crate::change_design_working_life::ChangeDesignWorkingLife;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(_payload: &ChangeDesignWorkingLife, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_design_working_life::ChangeDesignWorkingLife> {
    Ok((|| {
    vec![Op::from(ChangeDesignWorkingLife { new_years: base.design_working_life_years })]

    })())
}
