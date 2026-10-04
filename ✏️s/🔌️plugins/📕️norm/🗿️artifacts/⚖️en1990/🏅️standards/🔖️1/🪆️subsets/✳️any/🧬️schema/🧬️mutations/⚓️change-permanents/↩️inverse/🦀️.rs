//! ↩️ `change-permanents` inverse.

use super::ChangePermanents;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangePermanents, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = mutation;
    vec![En1990Mutation::ChangePermanents(ChangePermanents { new_permanents: base.permanents.clone() })]

    })())
}
