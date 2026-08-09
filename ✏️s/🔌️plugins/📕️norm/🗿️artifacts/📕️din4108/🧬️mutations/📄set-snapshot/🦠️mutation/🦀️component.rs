//! 📸️ Din4108 mutation — SetSnapshot payload + builder + apply.
use crate::artifacts::din4108::Din4108Snapshot;
use crate::artifacts::din4108::mutations::Din4108Mutation;

pub fn set_snapshot(snapshot: Din4108Snapshot) -> Din4108Mutation {
    Din4108Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut Din4108Snapshot, replacement: &Din4108Snapshot) {
    *base = replacement.clone();
}
