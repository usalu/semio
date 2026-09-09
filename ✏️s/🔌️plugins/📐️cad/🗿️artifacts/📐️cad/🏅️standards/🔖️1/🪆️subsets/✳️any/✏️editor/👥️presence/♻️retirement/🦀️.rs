//! 🧹️ Exact CAD presence ownership retirement, including variable-length engagement identifiers.

use super::CadPresence;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use store::{ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementStep};

//#region 🧹️SnapshotRetirement
pub struct CadPresenceRetirementFactory;

impl SnapshotRetirementFactory<CadPresence> for CadPresenceRetirementFactory {
    fn retire(&self, root: Arc<CadPresence>) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(CadPresenceRetirement { root: ManuallyDrop::new(Some(root)), owned: ManuallyDrop::new(None), bytes: ManuallyDrop::new(None), field: 0 })
    }
}

struct CadPresenceRetirement {
    root: ManuallyDrop<Option<Arc<CadPresence>>>,
    owned: ManuallyDrop<Option<CadPresence>>,
    bytes: ManuallyDrop<Option<Vec<u8>>>,
    field: u8,
}

impl ErasedSnapshotRetirement for CadPresenceRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(bytes) = self.bytes.as_mut() {
            if bytes.is_empty() {
                drop(self.bytes.take());
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let released_bytes = bytes.len().min(maximum_bytes);
            bytes.truncate(bytes.len() - released_bytes);
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
        }
        if let Some(root) = self.root.take() {
            *self.owned = Arc::into_inner(root);
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(owned) = self.owned.as_mut() {
            let value = match self.field {
                0 => Some(std::mem::take(&mut owned.engagement_step)),
                1 => owned.engagement_pane.take(),
                _ => {
                    drop(self.owned.take());
                    return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                }
            };
            self.field += 1;
            *self.bytes = value.map(String::into_bytes);
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none() && self.owned.is_none() && self.bytes.is_none()
    }
}

impl Drop for CadPresenceRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.terminal_is_empty(), "CAD presence retirement requires its exact terminal-empty witness");
        }
    }
}
//#endregion 🧹️SnapshotRetirement

//#region 🏪️StoreRetirement
pub fn empty_terminal() -> CadPresence {
    CadPresence { camera_position: [0.0; 3], camera_target: [0.0; 3], camera_zoom: 0.0, camera_fov: 0.0, engagement_step: String::new(), engagement_pane: None }
}

pub fn terminal_is_empty(value: &CadPresence) -> bool {
    value.engagement_step.is_empty() && value.engagement_pane.is_none()
}

pub struct CadPresenceStoreDisposer {
    terminal: Option<Arc<CadPresence>>,
    active: Option<store::PresenceStoreRetirement<CadPresence>>,
}

impl Default for CadPresenceStoreDisposer {
    fn default() -> Self {
        Self::new()
    }
}

impl CadPresenceStoreDisposer {
    pub fn new() -> Self {
        Self { terminal: Some(Arc::new(empty_terminal())), active: None }
    }
}

impl semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<CadPresence, super::CadPresenceMutation>> for CadPresenceStoreDisposer {
    fn close_step(&mut self, owner: &mut store::PresenceStore<CadPresence, super::CadPresenceMutation>, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework_plugin::Fault> {
        if maximum_items == 0 {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(active) = self.active.as_mut() {
            return active.close_step(1, maximum_bytes).map_err(semio_framework_plugin::Fault::from).map(|step| match step {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes },
                SnapshotRetirementStep::Blocked => semio_framework_plugin::PluginCloseStep::Blocked { reason: "CAD presence retains captured local or peer readers" },
                SnapshotRetirementStep::Complete => semio_framework_plugin::PluginCloseStep::Complete,
            });
        }
        let terminal = self.terminal.take().expect("CAD presence close owns its exact empty terminal root");
        match owner.begin_retirement(terminal, terminal_is_empty) {
            Ok(active) => self.active = Some(active),
            Err((reason, terminal)) => {
                self.terminal = Some(terminal);
                return Err(semio_framework_plugin::Fault::from(reason));
            }
        }
        Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self, owner: &store::PresenceStore<CadPresence, super::CadPresenceMutation>) -> bool {
        self.terminal.is_none() && self.active.as_ref().is_some_and(store::PresenceStoreRetirement::terminal_is_empty) && owner.retirement_started() && terminal_is_empty(owner.local()) && owner.peers_root().is_empty()
    }
}
//#endregion 🏪️StoreRetirement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
