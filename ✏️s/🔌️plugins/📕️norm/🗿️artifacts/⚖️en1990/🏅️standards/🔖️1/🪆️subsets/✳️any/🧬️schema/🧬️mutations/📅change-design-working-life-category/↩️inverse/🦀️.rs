//! ↩️ `change-design-working-life-category` inverse.

use crate::change_design_working_life_category::ChangeDesignWorkingLifeCategory;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeDesignWorkingLifeCategory, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_design_working_life_category::ChangeDesignWorkingLifeCategory> {
    Ok((|| {
    let _ = mutation;
    vec![Op::from(ChangeDesignWorkingLifeCategory { new_design_working_life_category: base.design_working_life_category })]

    })())
}
