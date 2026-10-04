//! ↩️ `change-consequence-class` inverse.

use super::ChangeConsequenceClass;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeConsequenceClass, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = mutation;
    vec![En1990Mutation::ChangeConsequenceClass(ChangeConsequenceClass { new_consequence_class: base.consequence_class })]

    })())
}
