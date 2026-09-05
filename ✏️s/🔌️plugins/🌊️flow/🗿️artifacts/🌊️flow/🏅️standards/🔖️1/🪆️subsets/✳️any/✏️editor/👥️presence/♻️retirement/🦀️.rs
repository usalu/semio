//! 🫧️ Flow presence returns preview identifiers through the shared paged domain owner.

use super::{FlowPresence, FlowPresenceMutation};
use flow::retained::{FlowOwner, FlowRetirement};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementStep};

const _: () = assert!(!std::mem::needs_drop::<flow::CameraJson>());

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
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        if let Some(root) = self.root.take() {
            if let Some(value) = Arc::into_inner(root) { self.domain.push(FlowOwner::Strings(value.preview_off_node_ids)); }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.domain.close_step(1, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool { self.root.is_none() && self.domain.is_empty() }
}

impl Drop for FlowPresenceRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(), "Flow presence must return every retained owner before drop"); } }
}

pub fn store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<FlowPresence, FlowPresenceMutation>>> {
    Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(Arc::new(FlowPresence::default()), |value| value.preview_off_node_ids.is_empty()).expect("default Flow presence is the empty domain terminal"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ArtifactApp, EditorApp, PluginCloseStep};

    fn presence(value: &serde_json::Value) -> FlowPresence {
        let text = value["unit"].as_str().unwrap().repeat(value["repeat"].as_u64().unwrap() as usize);
        FlowPresence { preview_off_node_ids: if text.is_empty() { vec![] } else { vec![text] }, ..Default::default() }
    }


    #[test]
    fn flow_presence_store_owners_preserve_readers_and_retire_neutral_byte_grants() {
        type App = EditorApp<crate::editor::flow::FlowPlayApp>;
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/👥️presence-owners/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            for grant in fixture["grants"].as_array().unwrap() {
                let maximum_bytes = grant.as_u64().unwrap() as usize;
                let mut owner = store::PresenceStore::<FlowPresence, FlowPresenceMutation>::new(presence(&row["local"]));
                owner.install_local_retirement_factory(App::build_presence_local_root_retirement_factory().unwrap()).unwrap();
                owner.install_peer_retirement_factory(App::build_presence_peer_retirement_factory().unwrap()).unwrap();
                let mut publication = owner.begin_peer_publication().unwrap();
                assert!(!publication.prune_one(|_| true).unwrap());
                for peer in row["peers"].as_array().unwrap() { assert!(publication.adopt(peer["actor"].as_str().unwrap().into(), presence(peer), 0).is_ok()); }
                while publication.release_created_one() {}
                let commit = publication.take_commit().unwrap();
                assert!(publication.terminal_is_empty());
                if let Some(mut retired) = owner.publish_peer_commit(commit).ok().unwrap() {
                    for _ in 0..100 { if retired.close_step(1, maximum_bytes).unwrap() == SnapshotRetirementStep::Complete { break; } }
                    assert!(retired.terminal_is_empty());
                }
                let shared = row["sharedRoot"].as_bool().unwrap();
                let mut reader = shared.then(|| owner.peers_root());
                let mut disposer = App::build_presence_store_disposer().unwrap();
                assert_eq!(disposer.close_step(&mut owner, 0, maximum_bytes).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                assert!(!owner.retirement_started());
                let mut released = 0;
                let mut observed_blocked = false;
                let mut completed = false;
                for _ in 0..100_000 {
                    match disposer.close_step(&mut owner, 1, maximum_bytes).unwrap() {
                        PluginCloseStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= maximum_bytes); released += released_bytes; }
                        PluginCloseStep::Blocked { .. } => {
                            let captured = reader.take().expect("only the declared captured roster may block Flow close");
                            let actual = captured.peers().map(|(actor, value)| (actor.to_owned(), value.clone())).collect::<Vec<_>>();
                            let expected = row["peers"].as_array().unwrap().iter().map(|peer| (peer["actor"].as_str().unwrap().to_owned(), presence(peer))).collect::<Vec<_>>();
                            assert_eq!(actual, expected);
                            observed_blocked = true;
                        }
                        PluginCloseStep::Complete => { completed = true; break; }
                    }
                }
                assert!(completed, "Flow presence must report completion within the fixed bound");
                assert!(disposer.terminal_is_empty(&owner));
                assert_eq!(released, row["expectedBytes"].as_u64().unwrap() as usize);
                assert_eq!(observed_blocked, shared);
                assert!(owner.begin_peer_publication().is_err());
                assert_eq!(disposer.close_step(&mut owner, 0, 0).unwrap(), PluginCloseStep::Complete);
                eprintln!("[DEBUG] Flow presence roster={} grant={maximum_bytes} retiredBytes={released} capturedReader={observed_blocked} terminal=true", row["id"]);
            }
        }
    }
}
