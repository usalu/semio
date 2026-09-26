//! ↩️ `change-use-class` inverse.

use crate::mutations::change_use_class::ChangeUseClass;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ChangeUseClass, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeUseClass(ChangeUseClass { new_use_class: base.use_class })]
}
