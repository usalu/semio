//! 🗿️ App-owned scene assembly delegates selected payload copying to the shared Flow cursor.

use super::{Owner, Retirement};
use super::super::FlowWorkingScene;
use flow::{neural, Widget};
use flow::retained::{FlowCopyAllocationBudget, FlowSynapseCopy, FlowWidgetCopy};
use std::mem::ManuallyDrop;
use std::sync::Arc;

#[path = "🧬️recipe/🦀️component.rs"]
pub(super) mod recipe;

#[path = "📸️snapshot/🦀️component.rs"]
pub(super) mod snapshot;

#[path = "📬️preparation/🦀️component.rs"]
pub(super) mod preparation;

//#region 🗿️SceneCursor
const FLOW_SCENE_COPY_ALLOCATION_BYTES: usize = 16 * 1024 * 1024;

pub(super) struct SceneCopyState {
    source: Option<Arc<FlowWorkingScene>>,
    widget: Option<FlowWidgetCopy<FlowWorkingScene>>,
    synapse: Option<FlowSynapseCopy<FlowWorkingScene>>,
    result: Option<FlowWorkingScene>,
    retirement: Retirement,
    phase: u8,
    index: usize,
    closing: bool,
}

pub(super) struct SceneCopy { state: ManuallyDrop<SceneCopyState> }
impl std::ops::Deref for SceneCopy { type Target = SceneCopyState; fn deref(&self) -> &Self::Target { &self.state } }
impl std::ops::DerefMut for SceneCopy { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }
impl Drop for SceneCopy {
    fn drop(&mut self) {
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.state); } }
        else if !std::thread::panicking() { panic!("Flow scene copy must close before drop"); }
    }
}

impl SceneCopy {
    pub(super) fn new(source: Arc<FlowWorkingScene>) -> Self {
        Self { state: ManuallyDrop::new(SceneCopyState {
            source: Some(source), widget: None, synapse: None, result: Some(FlowWorkingScene::default()),
            retirement: Retirement::default(), phase: 0, index: 0, closing: false,
        }) }
    }

    pub(super) fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<Option<usize>, String> {
        use store::SnapshotRetirementStep;
        if self.closing || maximum_items == 0 || maximum_bytes == 0 { return Ok(None); }
        let state = &mut *self.state;
        let source = state.source.as_ref().ok_or("Flow scene copier lost source")?;
        let result = state.result.as_mut().ok_or("Flow scene copier lost output")?;
        match state.phase {
            0 => {
                let count = source.widgets.len();
                if count > super::super::FLOW_STORE_MAX_SCENE_ITEMS { return Err("Flow scene widget count exceeds admitted envelope".into()); }
                let bytes = count.checked_mul(std::mem::size_of::<Widget>()).ok_or("Flow widget allocation overflow")?;
                if bytes > FLOW_SCENE_COPY_ALLOCATION_BYTES { return Err("Flow widget allocation exceeds admitted envelope".into()); }
                result.widgets.try_reserve_exact(count).map_err(|_| "Flow widget allocation failed")?;
                state.phase = 1;
            }
            1 => {
                if let Some(cursor) = state.widget.as_mut() {
                    if cursor.complete() {
                        result.widgets.push(cursor.take().unwrap()); cursor.begin_close(); state.phase = 2;
                    } else { return cursor.advance(1, maximum_bytes); }
                } else if state.index < source.widgets.len() {
                    state.widget = Some(FlowWidgetCopy::new(Arc::clone(source), state.index, |scene, index| scene.widgets.get(index), Arc::new(SceneRetirementFactory), FlowCopyAllocationBudget::new(FLOW_SCENE_COPY_ALLOCATION_BYTES, FLOW_SCENE_COPY_ALLOCATION_BYTES)));
                } else { state.index = 0; state.phase = 3; }
            }
            2 => {
                let cursor = state.widget.as_mut().unwrap();
                match cursor.close_step(1, maximum_bytes)? {
                    SnapshotRetirementStep::Complete => {
                        if !cursor.terminal_is_empty() { return Err("Flow selected widget retained an owner after close".into()); }
                        state.widget = None; state.index += 1; state.phase = 1;
                    }
                    SnapshotRetirementStep::Pending { released_bytes, .. } => return Ok(Some(released_bytes)),
                    SnapshotRetirementStep::Blocked => return Ok(None),
                }
            }
            3 => {
                let count = source.synapses.len();
                if count > super::super::FLOW_STORE_MAX_SCENE_ITEMS { return Err("Flow scene synapse count exceeds admitted envelope".into()); }
                let bytes = count.checked_mul(std::mem::size_of::<flow::SynapseSpec>()).ok_or("Flow synapse allocation overflow")?;
                if bytes > FLOW_SCENE_COPY_ALLOCATION_BYTES { return Err("Flow synapse allocation exceeds admitted envelope".into()); }
                result.synapses.try_reserve_exact(count).map_err(|_| "Flow synapse allocation failed")?;
                state.phase = 4;
            }
            4 => {
                if let Some(cursor) = state.synapse.as_mut() {
                    if cursor.complete() {
                        result.synapses.push(cursor.take().unwrap()); cursor.begin_close(); state.phase = 5;
                    } else { return cursor.advance(1, maximum_bytes); }
                } else if state.index < source.synapses.len() {
                    state.synapse = Some(FlowSynapseCopy::new(Arc::clone(source), state.index, |scene, index| scene.synapses.get(index), Arc::new(SceneRetirementFactory), FlowCopyAllocationBudget::new(FLOW_SCENE_COPY_ALLOCATION_BYTES, FLOW_SCENE_COPY_ALLOCATION_BYTES)));
                } else { state.phase = 6; }
            }
            5 => {
                let cursor = state.synapse.as_mut().unwrap();
                match cursor.close_step(1, maximum_bytes)? {
                    SnapshotRetirementStep::Complete => {
                        if !cursor.terminal_is_empty() { return Err("Flow selected synapse retained an owner after close".into()); }
                        state.synapse = None; state.index += 1; state.phase = 4;
                    }
                    SnapshotRetirementStep::Pending { released_bytes, .. } => return Ok(Some(released_bytes)),
                    SnapshotRetirementStep::Blocked => return Ok(None),
                }
            }
            6 => { result.layout = source.layout.clone(); state.phase = 7; }
            7 => {}
            _ => return Err("Flow scene copier phase is invalid".into()),
        }
        Ok(Some(0))
    }

    pub(super) fn complete(&self) -> bool { self.phase == 7 && self.result.is_some() }
    pub(super) fn take(&mut self) -> Option<FlowWorkingScene> { if self.phase == 7 { self.result.take() } else { None } }
    pub(super) fn begin_close(&mut self) {
        self.closing = true;
        if let Some(cursor) = self.widget.as_mut() { cursor.begin_close(); }
        if let Some(cursor) = self.synapse.as_mut() { cursor.begin_close(); }
    }

    pub(super) fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_job::InteractiveJobCloseStep, String> {
        use semio_framework_job::InteractiveJobCloseStep as Step;
        use store::SnapshotRetirementStep as SnapshotStep;
        if !self.closing || maximum_items == 0 || maximum_bytes == 0 { return Ok(Step::Blocked); }
        let state = &mut *self.state;
        if !state.retirement.is_empty() { return Ok(state.retirement.step(maximum_items, maximum_bytes)); }
        let step = if let Some(cursor) = state.widget.as_mut() {
            let step = cursor.close_step(1, maximum_bytes)?;
            if step == SnapshotStep::Complete {
                if !cursor.terminal_is_empty() { return Err("Flow selected widget close is nonterminal".into()); }
                state.widget = None;
            }
            Some(step)
        } else if let Some(cursor) = state.synapse.as_mut() {
            let step = cursor.close_step(1, maximum_bytes)?;
            if step == SnapshotStep::Complete {
                if !cursor.terminal_is_empty() { return Err("Flow selected synapse close is nonterminal".into()); }
                state.synapse = None;
            }
            Some(step)
        } else { None };
        if let Some(step) = step {
            return Ok(match step {
                SnapshotStep::Blocked => Step::Blocked,
                SnapshotStep::Pending { released_items, released_bytes } => Step::Pending { released_items, released_bytes },
                SnapshotStep::Complete => Step::Pending { released_items: 1, released_bytes: 0 },
            });
        }
        if let Some(result) = state.result.take() { state.retirement.push(Owner::Scene(result)); }
        else if let Some(source) = state.source.take() { if let Some(source) = Arc::into_inner(source) { state.retirement.push(Owner::Scene(source)); } }
        else { return Ok(Step::Complete); }
        Ok(Step::Pending { released_items: 1, released_bytes: 0 })
    }

    pub(super) fn terminal_is_empty(&self) -> bool { self.closing && self.source.is_none() && self.widget.is_none() && self.synapse.is_none() && self.result.is_none() && self.retirement.is_empty() }
}
//#endregion 🗿️SceneCursor

//#region 🪪️SceneIdentity
pub(super) struct SceneRetirementFactory;

impl store::SnapshotRetirementFactory<FlowWorkingScene> for SceneRetirementFactory {
    fn retire(&self, scene: Arc<FlowWorkingScene>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(SceneRetirement { scene: ManuallyDrop::new(Some(scene)), retirement: Retirement::default() })
    }
}

struct SceneRetirement { scene: ManuallyDrop<Option<Arc<FlowWorkingScene>>>, retirement: Retirement }

impl Drop for SceneRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.scene.is_none() && self.retirement.is_empty(), "Flow scene retirement must reach terminal emptiness"); } }
}

impl store::ErasedSnapshotRetirement for SceneRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        use store::SnapshotRetirementStep as Step;
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(Step::Blocked); }
        if !self.retirement.is_empty() { return store::ErasedSnapshotRetirement::close_step(&mut self.retirement, maximum_items, maximum_bytes); }
        if let Some(scene) = self.scene.take() {
            if let Some(scene) = Arc::into_inner(scene) { self.retirement.push(Owner::Scene(scene)); }
            return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(Step::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.scene.is_none() && self.retirement.is_empty() }
}

pub(super) struct SceneHash {
    reader: store::ArtifactCanonicalJsonReader<FlowWorkingScene>,
    hash: Option<semio_framework_hash::Sha256>,
    domain_offset: usize,
    digest: Option<[u8; 32]>,
}

impl SceneHash {
    pub(super) fn new(scene: Arc<FlowWorkingScene>) -> Self {
        Self { reader: store::ArtifactCanonicalJsonReader::new(scene, Arc::new(SceneRetirementFactory)), hash: Some(semio_framework_hash::Sha256::new()), domain_offset: 0, digest: None }
    }

    pub(super) fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<usize, String> {
        if !grant.permits_one() || self.digest.is_some() || self.hash.is_none() { return Ok(0); }
        let domain = crate::artifacts::flow::FLOW_CONTENT_ID_DOMAIN;
        if self.domain_offset < domain.len() {
            let end = (self.domain_offset + grant.maximum_bytes).min(domain.len());
            self.hash.as_mut().unwrap().update(&domain[self.domain_offset..end]);
            let bytes = end - self.domain_offset; self.domain_offset = end; return Ok(bytes);
        }
        let mut chunk = [0; 256];
        let bytes = self.reader.encode_chunk(grant, &mut chunk)?;
        self.hash.as_mut().unwrap().update(&chunk[..bytes]);
        if self.reader.is_complete() { self.digest = Some(self.hash.take().unwrap().finalize()); }
        Ok(bytes)
    }

    pub(super) fn complete(&self) -> bool { self.digest.is_some() }
    pub(super) fn take(&mut self) -> Option<(Arc<FlowWorkingScene>, [u8; 32])> { let digest = self.digest.take()?; Some((self.reader.take_root()?, digest)) }
    pub(super) fn cancel(&mut self) { self.reader.cancel(); }
    pub(super) fn begin_close(&mut self) { self.reader.begin_close(); self.hash = None; self.digest = None; }
    pub(super) fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> { self.reader.close_step(grant) }
    pub(super) fn terminal_is_empty(&self) -> bool { self.reader.terminal_is_empty() && self.hash.is_none() && self.digest.is_none() }
}
//#endregion 🪪️SceneIdentity

//#region 🧪️SceneOwnership
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_job::InteractiveJobCloseStep;

    fn close(cursor: &mut SceneCopy, grant: usize) -> usize {
        cursor.begin_close();
        let mut bytes = 0;
        for _ in 0..200_000 {
            match cursor.close_step(1, grant).unwrap() {
                InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= grant);
                    bytes += released_bytes;
                }
                InteractiveJobCloseStep::Complete => { assert!(cursor.terminal_is_empty()); return bytes; }
                InteractiveJobCloseStep::Blocked => panic!("positive byte grant must advance scene close"),
            }
        }
        panic!("scene close failed to terminate")
    }

    #[test]
    fn sixteen_kib_authored_label_copies_and_retires_at_actual_grants() {
        let frontier: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🎯️grant-frontier.json")).unwrap();
        for row in frontier["cases"].as_array().unwrap() {
            let label = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
            let grant = row["grantBytes"].as_u64().unwrap() as usize;
            let bytes = label.len() + "slider".len();
            let source = Arc::new(FlowWorkingScene { widgets: vec![Widget::InputSlider { id: "slider".into(), label, value: 6.0, min: 0.0, max: 10.0, step: 0.5 }], ..Default::default() });
            let expected = serde_json::to_value(&*source).unwrap();
            let weak = Arc::downgrade(&source);
            let mut cursor = SceneCopy::new(source);
            let mut copied = 0;
            for _ in 0..100_000 {
                if cursor.complete() { break; }
                let completed = cursor.advance(1, grant).unwrap().unwrap();
                assert!(completed <= grant);
                copied += completed;
            }
            assert!(cursor.complete());
            assert_eq!(copied, bytes);
            assert_eq!(serde_json::to_value(cursor.result.as_ref().unwrap()).unwrap(), expected);
            assert_eq!(close(&mut cursor, grant), bytes * 2);
            assert!(weak.upgrade().is_none());
        }
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn cancelled_nested_map_cursor_keeps_source_alive_across_worker_transfer() {
        let params = neural::Dictionary::new().insert("🌊".repeat(2048), neural::Value::Dictionary(neural::Dictionary::new().insert("value", neural::Value::Atom(neural::Atom::String("x".repeat(8192))))));
        let source = Arc::new(FlowWorkingScene { widgets: vec![Widget::Neuron { id: "node".into(), neuron_kind: "nested".into(), params, input_ports: vec![], output_ports: vec![], preview: false }], ..Default::default() });
        let weak = Arc::downgrade(&source);
        let mut cursor = SceneCopy::new(source);
        for _ in 0..5 { cursor.advance(1, 1).unwrap(); }
        assert!(weak.upgrade().is_some());
        let mut cursor = std::thread::spawn(move || { for _ in 0..5 { cursor.advance(1, 1).unwrap(); } cursor }).join().unwrap();
        assert!(weak.upgrade().is_some());
        assert!(!cursor.complete());
        assert!(close(&mut cursor, 1) >= 16384);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn scene_identity_matches_node_crypto_and_adopts_the_exact_root() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🪪️content-identity.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let canonical = row["canonicalJson"].as_str().unwrap();
            let (widgets, synapses, layout) = crate::artifacts::flow::schema::mutations::decode_flow_scene_json(canonical).unwrap();
            let root = Arc::new(FlowWorkingScene { widgets, synapses, layout });
            let expected_id = format!("flow-content-sha256-{}", row["expectedSha256"].as_str().unwrap());
            assert_eq!(serde_json::to_string(&*root).unwrap(), canonical);
            assert_eq!(crate::artifacts::flow::flow_content_child_handle(&root.widgets, &root.synapses, &root.layout).child_id, expected_id);
            for maximum_bytes in [1, 64, 4096] {
                let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes };
                let mut cursor = SceneHash::new(Arc::clone(&root));
                let mut bytes = 0;
                for _ in 0..100_000 {
                    if cursor.complete() { break; }
                    let completed = cursor.advance(grant).unwrap(); assert!(completed <= maximum_bytes); bytes += completed;
                }
                assert!(cursor.complete());
                assert_eq!(bytes, crate::artifacts::flow::FLOW_CONTENT_ID_DOMAIN.len() + canonical.len());
                let (scene, digest) = cursor.take().unwrap(); assert!(Arc::ptr_eq(&scene, &root));
                let child = crate::artifacts::flow::flow_content_child_from_digest(digest, scene);
                assert_eq!(child.child_id, expected_id);
                assert!(Arc::ptr_eq(&child.local_owner::<FlowWorkingScene>().unwrap(), &root));
                cursor.begin_close();
                for _ in 0..100_000 {
                    if cursor.close_step(grant).unwrap() == store::SnapshotRetirementStep::Complete { break; }
                }
                assert!(cursor.terminal_is_empty());
            }
            let mut retirement = store::SnapshotRetirementFactory::retire(&SceneRetirementFactory, root);
            for _ in 0..100_000 { if retirement.close_step(1, 4096).unwrap() == store::SnapshotRetirementStep::Complete { break; } }
            assert!(retirement.terminal_is_empty());
        }
    }
}
//#endregion 🧪️SceneOwnership
