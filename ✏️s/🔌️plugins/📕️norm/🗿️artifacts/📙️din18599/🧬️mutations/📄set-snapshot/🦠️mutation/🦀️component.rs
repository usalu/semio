//! 📸️ Din18599 mutation — SetSnapshot payload + builder + apply.
use crate::artifacts::din18599::Din18599Snapshot;
use crate::artifacts::din18599::mutations::Din18599Mutation;

pub fn set_snapshot(snapshot: Din18599Snapshot) -> Din18599Mutation {
    Din18599Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut Din18599Snapshot, replacement: &Din18599Snapshot) {
    *base = replacement.clone();
}
