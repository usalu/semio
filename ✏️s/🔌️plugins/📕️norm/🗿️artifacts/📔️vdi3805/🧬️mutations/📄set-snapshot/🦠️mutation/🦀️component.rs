//! 📸️ Vdi3805 mutation — SetSnapshot payload + builder + apply.
use crate::artifacts::vdi3805::Vdi3805Snapshot;
use crate::artifacts::vdi3805::mutations::Vdi3805Mutation;

pub fn set_snapshot(snapshot: Vdi3805Snapshot) -> Vdi3805Mutation {
    Vdi3805Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut Vdi3805Snapshot, replacement: &Vdi3805Snapshot) {
    *base = replacement.clone();
}
