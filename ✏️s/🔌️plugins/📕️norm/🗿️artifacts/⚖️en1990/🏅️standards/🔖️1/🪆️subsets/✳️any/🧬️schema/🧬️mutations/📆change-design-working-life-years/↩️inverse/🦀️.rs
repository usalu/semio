//! ↩️ `change-design-working-life-years` inverse.

use crate::change_design_working_life_years::ChangeDesignWorkingLifeYears;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeDesignWorkingLifeYears, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_design_working_life_years::ChangeDesignWorkingLifeYears> {
    Ok((|| {
    let _ = mutation;
    vec![Op::from(ChangeDesignWorkingLifeYears { new_design_working_life_years: base.design_working_life_years })]

    })())
}
