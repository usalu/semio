//! ↩️ `change-method` inverse.

use crate::mutations::change_method::ChangeMethod;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ChangeMethod, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeMethod(ChangeMethod { new_method: base.method })]
}
