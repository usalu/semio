//! ↩️ `change-design-working-life-category` inverse.

use super::ChangeDesignWorkingLifeCategory;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeDesignWorkingLifeCategory, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = mutation;
    vec![En1990Mutation::ChangeDesignWorkingLifeCategory(ChangeDesignWorkingLifeCategory { new_design_working_life_category: base.design_working_life_category })]

    })())
}
