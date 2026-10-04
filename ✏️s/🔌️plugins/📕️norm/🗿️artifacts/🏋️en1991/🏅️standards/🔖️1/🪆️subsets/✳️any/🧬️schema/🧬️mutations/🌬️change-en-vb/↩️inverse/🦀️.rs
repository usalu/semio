//! Inverse for `change-en-vb`.
use super::ChangeEnVb;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeEnVb, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeEnVb(ChangeEnVb { new_en_vb: base.en_vb })]

    })())
}
