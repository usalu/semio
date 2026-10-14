//! ↩️ `replace-elements` inverse.

use crate::replace_elements::ReplaceElements;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &ReplaceElements, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::replace_elements::ReplaceElements> {
    Ok((|| {
    vec![Op::from(ReplaceElements { new_elements: base.elements.clone() })]

    })())
}
