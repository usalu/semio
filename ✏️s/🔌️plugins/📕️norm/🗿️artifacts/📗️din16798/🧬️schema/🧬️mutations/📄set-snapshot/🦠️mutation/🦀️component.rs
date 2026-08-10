//! 📸️ Din16798 mutation — SetSnapshot payload + builder + apply.
use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::mutations::Din16798Mutation;

pub fn set_snapshot(snapshot: Din16798Snapshot) -> Din16798Mutation {
    Din16798Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut Din16798Snapshot, replacement: &Din16798Snapshot) {
    *base = replacement.clone();
}
