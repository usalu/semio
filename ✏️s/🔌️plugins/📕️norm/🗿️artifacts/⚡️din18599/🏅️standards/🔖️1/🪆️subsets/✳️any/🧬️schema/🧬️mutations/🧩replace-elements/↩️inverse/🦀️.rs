//! ↩️ `replace-elements` inverse.

use crate::mutations::replace_elements::ReplaceElements;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ReplaceElements, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ReplaceElements(ReplaceElements { new_elements: base.elements.clone() })]
}
