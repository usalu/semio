//! 📸️ Iso16757 mutation — SetSnapshot payload + builder + apply.
use crate::artifacts::iso16757::Iso16757Snapshot;
use crate::artifacts::iso16757::mutations::Iso16757Mutation;

pub fn set_snapshot(snapshot: Iso16757Snapshot) -> Iso16757Mutation {
    Iso16757Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut Iso16757Snapshot, replacement: &Iso16757Snapshot) {
    *base = replacement.clone();
}
