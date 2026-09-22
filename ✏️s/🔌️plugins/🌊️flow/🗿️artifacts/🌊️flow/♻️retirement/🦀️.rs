//! 🗃️ Shared document ownership for Flow scenes, snapshots and mutations.

use crate::op::FlowMutation;
use crate::{FlowSnapshot, FlowWorkingScene};
use semio_framework_artifact_flow_flow::retained::{FlowOwner, FlowRetirement};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{ErasedSnapshotRetirement, SnapshotRetirementStep};

#[path = "📸️snapshot/🦀️.rs"]
mod snapshot;
pub use snapshot::SnapshotRetirementFactory;

/// 🏪️ Exact document catalog shared by every surface that owns a Flow document.
pub fn store_owners() -> store::DocumentStoreOwners<FlowSnapshot, FlowMutation> {
    store::DocumentStoreOwners::new(Arc::new(SnapshotRetirementFactory), Arc::new(SnapshotRetirementFactory), Arc::new(MutationRetirementFactory), Box::new(store::ArtifactStoreCursorDisposer::<FlowSnapshot, FlowMutation>::new()))
}

pub(crate) fn retire_scene(scene: FlowWorkingScene) -> FlowRetirement {
    let mut retirement = FlowRetirement::default();
    let (widgets, synapses, layout) = scene.into_parts();
    retirement.push(FlowOwner::Widgets(widgets));
    retirement.push(FlowOwner::Specs(synapses));
    retirement.push(FlowOwner::Layouts(layout));
    retirement
}

pub(crate) fn retire_mutation(mutation: FlowMutation) -> FlowRetirement {
    let mut retirement = FlowRetirement::default();
    match mutation {
        FlowMutation::CreateWidget(value) => retirement.push(FlowOwner::Widget(value.widget)),
        FlowMutation::DeleteWidget(value) => retirement.text(value.id),
        FlowMutation::ReorderWidgets(value) => retirement.text(value.id),
        FlowMutation::ReplaceWidget(value) => {
            retirement.text(value.id);
            retirement.push(FlowOwner::Widget(value.widget));
        }
        FlowMutation::ConnectWidgets(value) => {
            retirement.text(value.id);
            retirement.text(value.from);
            retirement.text(value.to);
            retirement.text(value.from_port);
            retirement.text(value.to_port);
        }
        FlowMutation::DisconnectWidgets(value) => retirement.text(value.id),
        FlowMutation::ReorderSynapses(value) => retirement.text(value.id),
        FlowMutation::UpdateSynapseEndpoints(value) => {
            retirement.text(value.id);
            retirement.text(value.from);
            retirement.text(value.to);
            retirement.text(value.from_port);
            retirement.text(value.to_port);
        }
        FlowMutation::MoveWidgets(value) => retirement.push(FlowOwner::Layout(value.entries)),
        FlowMutation::DuplicateWidget(value) => {
            retirement.text(value.source_id);
            retirement.text(value.new_id);
            retirement.text(value.synapse_id);
            retirement.text(value.from_port);
            retirement.text(value.to_port);
        }
    }
    retirement
}

/// 📏️ Drives ONE bounded close step of a typed Flow frontier at the physical minimum that frontier
/// publishes, and hands the freed bytes back to the caller in page-sized instalments.
///
/// ⛔️ Since 2026-09-22 a `FlowRetirement` frees a heap allocation WHOLE or not at all and answers
/// `Blocked` for every grant below the demand it publishes through
/// [`FlowRetirement::next_close_byte_demand`]
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs`,
/// `release_root_backing`). A driver that forwarded its caller's fixed page therefore blocked
/// forever on any owner whose backing is larger than that page — which is every Flow document with
/// more than a page of text, and every registered fixture close through
/// `ArtifactDocumentStoreDisposer`.
///
/// 🎟️ The physical release is paid HERE, out of this driver's own admission, exactly as the
/// framework's own selected-copy cursor does. `debt` carries the part that does not fit in the
/// caller's page, so the caller's grant is never exceeded AND the total reported over the close
/// still equals the total physically freed — the accounting is amortized, the free is not delayed.
pub(crate) fn close_frontier_page(domain: &mut FlowRetirement, debt: &mut usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
    if *debt > 0 {
        let paid = (*debt).min(maximum_bytes);
        *debt -= paid;
        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: paid });
    }
    let demand = domain.next_close_byte_demand().map_err(str::to_owned)?;
    let step = domain.close_page(1, maximum_bytes.max(demand))?;
    let SnapshotRetirementStep::Pending { released_items, released_bytes } = step else {
        return Ok(step);
    };
    *debt = released_bytes.saturating_sub(maximum_bytes);
    Ok(SnapshotRetirementStep::Pending { released_items, released_bytes: released_bytes.min(maximum_bytes) })
}

struct RootRetirement<T> {
    root: ManuallyDrop<Option<Arc<T>>>,
    owned: ManuallyDrop<Option<T>>,
    domain: FlowRetirement,
    /// 🎟️ Bytes already freed above the caller's page, still owed to the caller's accounting.
    debt: usize,
    retire: fn(T) -> FlowRetirement,
}

impl<T> RootRetirement<T> {
    fn new(root: Option<Arc<T>>, owned: Option<T>, retire: fn(T) -> FlowRetirement) -> Self {
        Self { root: ManuallyDrop::new(root), owned: ManuallyDrop::new(owned), domain: FlowRetirement::default(), debt: 0, retire }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none() && self.owned.is_none() && self.domain.is_empty() && self.debt == 0
    }
}

impl<T: Send + Sync> ErasedSnapshotRetirement for RootRetirement<T> {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if items == 0 || bytes == 0 {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if self.debt == 0 {
            if let Some(root) = self.root.take() {
                *self.owned = Arc::into_inner(root);
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
        }
        if self.debt == 0 {
            if let Some(value) = self.owned.take() {
                self.domain = (self.retire)(value);
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
        }
        close_frontier_page(&mut self.domain, &mut self.debt, bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Drop for RootRetirement<T> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.is_empty(), "Flow document ownership must reach terminal emptiness");
        }
    }
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
