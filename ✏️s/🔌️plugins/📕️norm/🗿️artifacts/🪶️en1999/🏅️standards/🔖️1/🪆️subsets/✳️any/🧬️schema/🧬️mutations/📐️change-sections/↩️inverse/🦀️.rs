//! ↩️ `change-sections` inverse.

use crate::mutations::change_sections::ChangeSections;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(_payload: &ChangeSections, base: &En1999Snapshot) -> Result<Vec<En1999Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1999Mutation::ChangeSections(ChangeSections { sections: base.sections.clone() })]

    })())
}
