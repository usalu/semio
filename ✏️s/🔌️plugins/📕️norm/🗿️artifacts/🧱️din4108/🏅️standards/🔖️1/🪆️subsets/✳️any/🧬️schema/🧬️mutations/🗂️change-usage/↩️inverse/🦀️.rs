//! ↩️ `change-usage` inverse.

use super::ChangeUsage;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeUsage, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeUsage(ChangeUsage { new_usage: base.usage.clone() })]
}
