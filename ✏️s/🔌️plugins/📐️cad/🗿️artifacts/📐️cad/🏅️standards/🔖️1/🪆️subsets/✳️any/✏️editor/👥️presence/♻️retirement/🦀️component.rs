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
            return match Arc::try_unwrap(root) {
                Ok(owned) => {
                    *self.owned = Some(owned);
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                Err(root) => {
                    *self.root = Some(root);
                    Ok(SnapshotRetirementStep::Blocked)
                }
            };
        }
        if let Some(owned) = self.owned.as_mut() {
            let value = match self.field {
                0 => Some(std::mem::take(&mut owned.active_utility_id)),
                1 => Some(std::mem::take(&mut owned.engagement_step)),
                2 => owned.engagement_pane.take(),
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
    CadPresence { camera_position: [0.0; 3], camera_target: [0.0; 3], camera_zoom: 0.0, camera_fov: 0.0, active_utility_id: String::new(), engagement_step: String::new(), engagement_pane: None }
}

pub fn terminal_is_empty(value: &CadPresence) -> bool {
    value.active_utility_id.is_empty() && value.engagement_step.is_empty() && value.engagement_pane.is_none()
}

pub struct CadPresenceStoreDisposer {
    terminal: Option<Arc<CadPresence>>,
    active: Option<store::PresenceStoreRetirement<CadPresence>>,
}

impl CadPresenceStoreDisposer {
    pub fn new() -> Self {
        Self { terminal: Some(Arc::new(empty_terminal())), active: None }
    }
}

impl semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<CadPresence, super::CadPresenceMutation>> for CadPresenceStoreDisposer {
    fn close_step(
        &mut self,
        owner: &mut store::PresenceStore<CadPresence, super::CadPresenceMutation>,
        maximum_items: usize,
        maximum_bytes: usize,
    ) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework_plugin::Fault> {
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
        match owner.begin_retirement(terminal, terminal_is_empty, Arc::new(CadPresenceRetirementFactory)) {
            Ok(active) => self.active = Some(active),
            Err((reason, terminal)) => {
                self.terminal = Some(terminal);
                return Err(semio_framework_plugin::Fault::from(reason));
            }
        }
        Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self, owner: &store::PresenceStore<CadPresence, super::CadPresenceMutation>) -> bool {
        self.terminal.is_none() && self.active.as_ref().is_some_and(store::PresenceStoreRetirement::terminal_is_empty) && owner.retirement_started() && terminal_is_empty(owner.local_root().as_ref()) && owner.peers_root().is_empty()
    }
}
//#endregion 🏪️StoreRetirement

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn text(value: &serde_json::Value) -> String {
        value["unit"].as_str().unwrap().repeat(value["repeat"].as_u64().unwrap() as usize)
    }

    fn presence(case: &serde_json::Value) -> CadPresence {
        CadPresence { active_utility_id: text(&case["activeUtility"]), engagement_step: text(&case["engagementStep"]), engagement_pane: (!case["engagementPane"].is_null()).then(|| text(&case["engagementPane"])), ..CadPresence::default() }
    }

    #[test]
    fn retained_cad_presence_close_preserves_shared_roots_and_byte_grants() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️retirement.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let value = presence(case);
            let oracle = serde_json::to_value(&value).unwrap();
            let expected = ["activeUtilityId", "engagementStep", "engagementPane"].into_iter().map(|key| oracle[key].as_str().map_or(0, str::len)).sum::<usize>();
            assert_eq!(expected, case["expectedBytes"].as_u64().unwrap() as usize);
            let root = Arc::new(value);
            let reader = root.clone();
            let mut retirement = CadPresenceRetirementFactory.retire(root);
            assert_eq!(retirement.close_step(0, 4096).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!(retirement.close_step(1, 4096).unwrap(), SnapshotRetirementStep::Blocked);
            assert_eq!(serde_json::to_value(reader.as_ref()).unwrap(), oracle);
            drop(reader);
            let mut released = 0;
            for turn in 0..128 {
                match retirement.close_step(1, 4096).unwrap() {
                    SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1 && released_bytes <= 4096);
                        released += released_bytes;
                    }
                    SnapshotRetirementStep::Blocked => panic!("unshared CAD presence failed to progress"),
                    SnapshotRetirementStep::Complete => break,
                }
                assert!(turn < 127, "CAD presence exceeded its fixed lifecycle bound");
            }
            assert!(retirement.terminal_is_empty());
            assert_eq!(released, expected);
            eprintln!("[DEBUG] CAD presence close case={} retired_bytes={released}", case["name"]);
        }
    }

    /// 🧪️ Reclaims the one raw Arc ownership count preserved by the cursor's ManuallyDrop field during unwind.
    #[test]
    fn retained_cad_presence_close_worker_unwind_preserves_the_original_panic() {
        let root = Arc::new(CadPresence { active_utility_id: String::new(), engagement_step: String::new(), engagement_pane: None, ..CadPresence::default() });
        let pointer = Arc::into_raw(root);
        let result = std::panic::catch_unwind(|| {
            let mut retained = CadPresenceRetirement { root: ManuallyDrop::new(None), owned: ManuallyDrop::new(None), bytes: ManuallyDrop::new(None), field: 0 };
            *retained.root = Some(unsafe { Arc::from_raw(pointer) });
            panic!("CAD worker fixture failure");
        });
        let panic = result.expect_err("worker panic must remain observable");
        assert_eq!(panic.downcast_ref::<&str>(), Some(&"CAD worker fixture failure"));
        let mut retirement = CadPresenceRetirementFactory.retire(unsafe { Arc::from_raw(pointer) });
        for _ in 0..32 {
            if retirement.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete { break; }
        }
        assert!(retirement.terminal_is_empty());
        eprintln!("[DEBUG] CAD presence unwind preserved the original panic and retained its exact root for explicit retirement");
    }

    #[test]
    fn retained_cad_presence_close_nonempty_roster_retains_readers_and_domain_bytes() {
        use semio_framework_plugin::{ArtifactOwnedDisposer, PluginCloseStep};
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️retirement.json")).unwrap();
        let value = |name: &str| presence(fixture["cases"].as_array().unwrap().iter().find(|case| case["name"] == name).unwrap());
        for case in fixture["storeCases"].as_array().unwrap() {
            let mut owner = store::PresenceStore::<CadPresence, super::super::CadPresenceMutation>::new(value(case["local"].as_str().unwrap()));
            owner.install_peer_retirement_factory(Arc::new(CadPresenceRetirementFactory)).unwrap();
            let mut publication = owner.begin_peer_publication().unwrap();
            assert!(!publication.prune_one(|_| true).unwrap());
            for peer in case["peers"].as_array().unwrap() {
                assert!(publication.adopt(peer["actor"].as_str().unwrap().into(), value(peer["presence"].as_str().unwrap()), 0).is_ok());
            }
            while publication.release_created_one() {}
            let commit = publication.take_commit().unwrap();
            assert!(publication.terminal_is_empty());
            let mut initial_roster = owner.publish_peer_commit(commit).ok().unwrap().unwrap();
            for _ in 0..16 { if initial_roster.close_step(1, 4096).unwrap() == store::SnapshotRetirementStep::Complete { break; } }
            assert!(initial_roster.terminal_is_empty());
            let shared = case["sharedRoot"].as_bool().unwrap();
            let mut reader = shared.then(|| owner.peers_root());
            let mut disposer = CadPresenceStoreDisposer::new();
            let mut retired_bytes = 0;
            let mut observed_blocked = false;
            for turn in 0..512 {
                match disposer.close_step(&mut owner, 1, 4096).unwrap() {
                    PluginCloseStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= 4096); retired_bytes += released_bytes; }
                    PluginCloseStep::Blocked { .. } => {
                        let captured = reader.take().expect("only the declared captured reader may block CAD close");
                        let actual = captured.peers().map(|(actor, presence)| (actor.to_owned(), serde_json::to_value(presence).unwrap())).collect::<Vec<_>>();
                        let expected = case["peers"].as_array().unwrap().iter().map(|peer| (peer["actor"].as_str().unwrap().to_owned(), serde_json::to_value(value(peer["presence"].as_str().unwrap())).unwrap())).collect::<Vec<_>>();
                        assert_eq!(actual, expected);
                        observed_blocked = true;
                    }
                    PluginCloseStep::Complete => break,
                }
                assert!(turn < 511);
            }
            assert!(disposer.terminal_is_empty(&owner));
            assert!(terminal_is_empty(owner.local_root().as_ref()));
            assert_eq!(observed_blocked, shared);
            assert_eq!(retired_bytes, case["expectedBytes"].as_u64().unwrap() as usize);
            assert!(owner.begin_peer_publication().is_err());
            eprintln!("[DEBUG] CAD presence roster close case={} retired_bytes={retired_bytes} held_reader={observed_blocked}", case["name"]);
        }
    }
}
//#endregion 🧪️Tests
