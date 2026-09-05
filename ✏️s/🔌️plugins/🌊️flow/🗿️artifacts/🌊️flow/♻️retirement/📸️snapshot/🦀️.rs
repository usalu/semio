//! 📸️ Exact Flow snapshot ownership handoff; local scene and wire strings retire separately.

use flow::retained::{FlowOwner, FlowRetirement};
use crate::artifacts::flow::{FlowSnapshot, FlowWorkingScene};
use std::{mem::ManuallyDrop, sync::Arc};

const _: () = assert!(!std::mem::needs_drop::<flow::CameraJson>());

//#region 🧹️SnapshotOwnership
pub struct SnapshotRetirementFactory;

impl store::SnapshotRetirementFactory<FlowSnapshot> for SnapshotRetirementFactory {
    fn retire(&self, snapshot: Arc<FlowSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(SnapshotRetirement { root: ManuallyDrop::new(Some(snapshot)), owned: ManuallyDrop::new(None), retirement: FlowRetirement::default(), phase: 0 })
    }
}

impl store::ArtifactOwnedValueRetirementFactory<FlowSnapshot> for SnapshotRetirementFactory {
    fn retire_owned(&self, snapshot: FlowSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(SnapshotRetirement { root: ManuallyDrop::new(None), owned: ManuallyDrop::new(Some(snapshot)), retirement: FlowRetirement::default(), phase: 0 })
    }
}

struct SnapshotRetirement {
    root: ManuallyDrop<Option<Arc<FlowSnapshot>>>, owned: ManuallyDrop<Option<FlowSnapshot>>,
    retirement: FlowRetirement, phase: u8,
}

impl store::ErasedSnapshotRetirement for SnapshotRetirement {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        use store::SnapshotRetirementStep as Step;
        if items == 0 || bytes == 0 { return Ok(Step::Blocked); }
        if !self.retirement.is_empty() { return store::ErasedSnapshotRetirement::close_step(&mut self.retirement, 1, bytes); }
        if let Some(root) = self.root.take() { self.owned = ManuallyDrop::new(Arc::into_inner(root)); return Ok(Step::Pending { released_items: 1, released_bytes: 0 }); }
        let Some(snapshot) = self.owned.as_mut() else { return Ok(Step::Complete); };
        let text = match self.phase {
            0 => {
                let scene = snapshot.content.take_local_owner::<FlowWorkingScene>().map_err(str::to_owned)?;
                if let Some(scene) = scene.and_then(Arc::into_inner) { self.retirement = super::retire_scene(scene); }
                self.phase = 1; return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
            }
            1 => std::mem::take(&mut snapshot.schema),
            2 => std::mem::take(&mut snapshot.content.child_id),
            3 => std::mem::take(&mut snapshot.content.target.artifact_id),
            4 => std::mem::take(&mut snapshot.content.target.dialect.artifact_kind),
            5 => std::mem::take(&mut snapshot.content.target.dialect.standard),
            6 => std::mem::take(&mut snapshot.content.target.dialect.subset),
            _ => { self.owned.take(); return Ok(Step::Pending { released_items: 1, released_bytes: 0 }); }
        };
        self.phase += 1; self.retirement.push(FlowOwner::Bytes(text.into_bytes()));
        Ok(Step::Pending { released_items: 1, released_bytes: 0 })
    }
    fn terminal_is_empty(&self) -> bool { self.root.is_none() && self.owned.is_none() && self.retirement.is_empty() }
}

impl Drop for SnapshotRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() { assert!(self.root.is_none() && self.owned.is_none() && self.retirement.is_empty(), "Flow snapshot retirement must close exactly"); }
    }
}
//#endregion 🧹️SnapshotOwnership
