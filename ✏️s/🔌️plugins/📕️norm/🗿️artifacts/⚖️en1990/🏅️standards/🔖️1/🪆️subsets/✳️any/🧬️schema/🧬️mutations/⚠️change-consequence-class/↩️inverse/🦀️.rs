//! ↩️ `change-consequence-class` inverse.

use crate::change_consequence_class::ChangeConsequenceClass;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeConsequenceClass, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_consequence_class::ChangeConsequenceClass> {
    Ok((|| {
    let _ = mutation;
    vec![Op::from(ChangeConsequenceClass { new_consequence_class: base.consequence_class })]

    })())
}
