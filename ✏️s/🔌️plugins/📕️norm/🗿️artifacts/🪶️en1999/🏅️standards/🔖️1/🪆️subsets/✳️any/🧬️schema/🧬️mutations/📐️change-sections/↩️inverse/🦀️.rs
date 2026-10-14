//! ↩️ `change-sections` inverse.

use crate::change_sections::ChangeSections;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(_payload: &ChangeSections, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sections::ChangeSections> {
    Ok((|| {
    vec![Op::from(ChangeSections { sections: base.sections.clone() })]

    })())
}
