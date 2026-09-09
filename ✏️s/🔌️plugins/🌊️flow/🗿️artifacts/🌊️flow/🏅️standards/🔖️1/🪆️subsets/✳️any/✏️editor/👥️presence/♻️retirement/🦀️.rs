//! 🫧️ Flow presence returns preview identifiers through the shared paged domain owner.

use super::{FlowPresence, FlowPresenceMutation};
use semio_framework_artifact_flow_flow::retained::{FlowOwner, FlowRetirement};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementStep};

const _: () = assert!(!std::mem::needs_drop::<semio_framework_artifact_flow_flow::CameraJson>());

/// 🌊️ Exact local and peer snapshot ownership, including variable-length UTF-8 identifiers.
pub struct FlowPresenceRetirementFactory;

impl SnapshotRetirementFactory<FlowPresence> for FlowPresenceRetirementFactory {
    fn retire(&self, root: Arc<FlowPresence>) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(FlowPresenceRetirement { root: ManuallyDrop::new(Some(root)), domain: FlowRetirement::default() })
    }
}

struct FlowPresenceRetirement {
    root: ManuallyDrop<Option<Arc<FlowPresence>>>,
    domain: FlowRetirement,
}

impl ErasedSnapshotRetirement for FlowPresenceRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if let Some(root) = self.root.take() {
            if let Some(value) = Arc::into_inner(root) {
                self.domain.push(FlowOwner::Strings(value.preview_off_node_ids));
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.domain.close_step(1, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none() && self.domain.is_empty()
    }
}

impl Drop for FlowPresenceRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.terminal_is_empty(), "Flow presence must return every retained owner before drop");
        }
    }
}

pub fn store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<FlowPresence, FlowPresenceMutation>>> {
    Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(Arc::new(FlowPresence::default()), |value| value.preview_off_node_ids.is_empty()).expect("default Flow presence is the empty domain terminal"))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
