use super::*;

fn text(value: &protocol::os_pack::json::Value) -> String {
    value["unit"].as_str().unwrap().repeat(value["repeat"].as_u64().unwrap() as usize)
}

fn presence(case: &protocol::os_pack::json::Value) -> CadPresence {
    CadPresence { engagement_step: text(&case["engagementStep"]), engagement_pane: (!case["engagementPane"].is_null()).then(|| text(&case["engagementPane"])), ..CadPresence::default() }
}

#[test]
fn retained_cad_presence_close_preserves_shared_roots_and_byte_grants() {
    let fixture: protocol::os_pack::json::Value = protocol::json::parse(include_str!("../../../🧫️fixtures/♻️retirement/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let value = presence(case);
        let oracle = value.clone();
        let expected = oracle.engagement_step.len() + oracle.engagement_pane.as_deref().map_or(0, str::len);
        assert_eq!(expected, case["expectedBytes"].as_u64().unwrap() as usize);
        let root = Arc::new(value);
        let reader = root.clone();
        let mut retirement = CadPresenceRetirementFactory.retire(root);
        assert_eq!(retirement.close_step(0, 4096).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(retirement.close_step(1, 4096).unwrap(), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        assert_eq!(retirement.close_step(1, 4096).unwrap(), SnapshotRetirementStep::Complete);
        assert!(retirement.terminal_is_empty());
        assert_eq!(reader.as_ref(), &oracle);
        let mut retirement = CadPresenceRetirementFactory.retire(reader);
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
    let root = Arc::new(CadPresence { engagement_step: String::new(), engagement_pane: None, ..CadPresence::default() });
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
        if retirement.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert!(retirement.terminal_is_empty());
    eprintln!("[DEBUG] CAD presence unwind preserved the original panic and retained its exact root for explicit retirement");
}

#[test]
fn retained_cad_presence_close_nonempty_roster_retains_readers_and_domain_bytes() {
    use semio_framework_plugin::{ArtifactOwnedDisposer, PluginCloseStep};
    let fixture: protocol::os_pack::json::Value = protocol::json::parse(include_str!("../../../🧫️fixtures/♻️retirement/🔣️.json")).unwrap();
    let value = |name: &str| presence(fixture["cases"].as_array().unwrap().iter().find(|case| case["name"] == name).unwrap());
    for case in fixture["storeCases"].as_array().unwrap() {
        let mut owner = store::PresenceStore::<CadPresence, super::super::CadPresenceMutation>::new(value(case["local"].as_str().unwrap()));
        owner.install_local_retirement_factory(Arc::new(CadPresenceRetirementFactory)).unwrap();
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
        for _ in 0..16 {
            if initial_roster.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(initial_roster.terminal_is_empty());
        let shared = case["sharedRoot"].as_bool().unwrap();
        let mut reader = shared.then(|| owner.peers_root());
        let mut disposer = CadPresenceStoreDisposer::new();
        let mut retired_bytes = 0;
        let mut observed_blocked = false;
        for turn in 0..512 {
            match disposer.close_step(&mut owner, 1, 4096).unwrap() {
                PluginCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= 4096);
                    retired_bytes += released_bytes;
                }
                PluginCloseStep::Blocked { .. } => {
                    let captured = reader.take().expect("only the declared captured reader may block CAD close");
                    let actual = captured.peers().map(|(actor, presence)| (actor.to_owned(), presence.clone())).collect::<Vec<_>>();
                    let expected = case["peers"].as_array().unwrap().iter().map(|peer| (peer["actor"].as_str().unwrap().to_owned(), value(peer["presence"].as_str().unwrap()))).collect::<Vec<_>>();
                    assert_eq!(actual, expected);
                    observed_blocked = true;
                }
                PluginCloseStep::AwaitingInput { reason } => panic!("store-only CAD fixture unexpectedly awaits external input: {reason}"),
                PluginCloseStep::Complete => break,
            }
            assert!(turn < 511);
        }
        assert!(disposer.terminal_is_empty(&owner));
        assert!(terminal_is_empty(owner.local()));
        assert_eq!(observed_blocked, shared);
        assert_eq!(retired_bytes, case["expectedBytes"].as_u64().unwrap() as usize);
        assert!(owner.begin_peer_publication().is_err());
        eprintln!("[DEBUG] CAD presence roster close case={} retired_bytes={retired_bytes} held_reader={observed_blocked}", case["name"]);
    }
}
