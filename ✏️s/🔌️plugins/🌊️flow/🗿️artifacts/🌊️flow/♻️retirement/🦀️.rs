//! 🗃️ Shared document ownership for Flow scenes, snapshots and mutations.

use crate::artifacts::flow::{FlowSnapshot, FlowWorkingScene};
use crate::artifacts::flow::op::FlowMutation;
use flow::retained::{FlowOwner, FlowRetirement};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{ErasedSnapshotRetirement, SnapshotRetirementStep};

#[path = "📸️snapshot/🦀️.rs"]
mod snapshot;
pub use snapshot::SnapshotRetirementFactory;

/// 🏪️ Exact document catalog shared by every surface that owns a Flow document.
pub fn store_owners() -> store::MemberStoreOwners<FlowSnapshot, FlowMutation> {
    store::MemberStoreOwners::new(
        Arc::new(SnapshotRetirementFactory), Arc::new(SnapshotRetirementFactory), Arc::new(MutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<FlowSnapshot, FlowMutation>::new()),
    )
}

pub(crate) fn retire_scene(scene: FlowWorkingScene) -> FlowRetirement {
    let mut retirement = FlowRetirement::default();
    retirement.push(FlowOwner::Widgets(scene.widgets));
    retirement.push(FlowOwner::Specs(scene.synapses));
    retirement.push(FlowOwner::Layouts(scene.layout));
    retirement
}

pub(crate) fn retire_mutation(mutation: FlowMutation) -> FlowRetirement {
    let mut retirement = FlowRetirement::default();
    match mutation {
        FlowMutation::CreateWidget(value) => retirement.push(FlowOwner::Widget(value.widget)),
        FlowMutation::DeleteWidget(value) => retirement.text(value.id),
        FlowMutation::ReorderWidgets(value) => retirement.text(value.id),
        FlowMutation::ReplaceWidget(value) => { retirement.text(value.id); retirement.push(FlowOwner::Widget(value.widget)); }
        FlowMutation::ConnectWidgets(value) => { retirement.text(value.id); retirement.text(value.from); retirement.text(value.to); retirement.text(value.from_port); retirement.text(value.to_port); }
        FlowMutation::DisconnectWidgets(value) => retirement.text(value.id),
        FlowMutation::ReorderSynapses(value) => retirement.text(value.id),
        FlowMutation::UpdateSynapseEndpoints(value) => { retirement.text(value.id); retirement.text(value.from); retirement.text(value.to); retirement.text(value.from_port); retirement.text(value.to_port); }
        FlowMutation::MoveWidgets(value) => retirement.push(FlowOwner::Layout(value.entries)),
        FlowMutation::DuplicateWidget(value) => { retirement.text(value.source_id); retirement.text(value.new_id); retirement.text(value.synapse_id); retirement.text(value.from_port); retirement.text(value.to_port); }
    }
    retirement
}

struct RootRetirement<T> {
    root: ManuallyDrop<Option<Arc<T>>>,
    owned: ManuallyDrop<Option<T>>,
    domain: FlowRetirement,
    retire: fn(T) -> FlowRetirement,
}

impl<T> RootRetirement<T> {
    fn new(root: Option<Arc<T>>, owned: Option<T>, retire: fn(T) -> FlowRetirement) -> Self {
        Self { root: ManuallyDrop::new(root), owned: ManuallyDrop::new(owned), domain: FlowRetirement::default(), retire }
    }

    fn is_empty(&self) -> bool { self.root.is_none() && self.owned.is_none() && self.domain.is_empty() }
}

impl<T: Send + Sync> ErasedSnapshotRetirement for RootRetirement<T> {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if items == 0 || bytes == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        if let Some(root) = self.root.take() { *self.owned = Arc::into_inner(root); return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
        if let Some(value) = self.owned.take() { self.domain = (self.retire)(value); return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
        self.domain.close_step(1, bytes)
    }

    fn terminal_is_empty(&self) -> bool { self.is_empty() }
}

impl<T> Drop for RootRetirement<T> {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.is_empty(), "Flow document ownership must reach terminal emptiness"); } }
}

/// 🌐️ Returns a captured scene through its exact domain owner, preserving other readers.
pub struct SceneRetirementFactory;

impl store::SnapshotRetirementFactory<FlowWorkingScene> for SceneRetirementFactory {
    fn retire(&self, scene: Arc<FlowWorkingScene>) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(RootRetirement::new(Some(scene), None, retire_scene))
    }
}

/// 🧬️ A mutation is retained intact until a granted domain-retirement step.
pub struct MutationRetirementFactory;

impl store::SnapshotRetirementFactory<FlowMutation> for MutationRetirementFactory {
    fn retire(&self, mutation: Arc<FlowMutation>) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(RootRetirement::new(Some(mutation), None, retire_mutation))
    }
}

impl store::ArtifactOwnedValueRetirementFactory<FlowMutation> for MutationRetirementFactory {
    fn retire_owned(&self, mutation: FlowMutation) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(RootRetirement::new(None, Some(mutation), retire_mutation))
    }
}
