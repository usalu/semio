//! ↩️ Inverse for SetSnapshot on En1999.
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

pub fn inverse(base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::SetSnapshot { snapshot: base.clone() }]
}
