//! ↩️ Inverse for SetSnapshot on En1997.
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

pub fn inverse(base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::SetSnapshot { snapshot: base.clone() }]
}
