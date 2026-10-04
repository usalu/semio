//! ↩️ `change-cold-formed` inverse.

use crate::mutations::change_cold_formed::ChangeColdFormed;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(_payload: &ChangeColdFormed, base: &En1999Snapshot) -> Result<Vec<En1999Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1999Mutation::ChangeColdFormed(ChangeColdFormed { cold_formed: base.cold_formed.clone() })]

    })())
}
