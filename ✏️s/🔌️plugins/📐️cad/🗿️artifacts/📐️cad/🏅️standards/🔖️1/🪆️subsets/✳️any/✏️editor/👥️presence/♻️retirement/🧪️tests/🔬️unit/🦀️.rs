use super::*;
use semio_framework_value::retained_clone::RetainedCloneStep;
use store::SnapshotRetirementFactory;

fn text(value: &semio_framework_pack_json::Value) -> String {
    value["unit"].as_str().unwrap().repeat(value["repeat"].as_u64().unwrap() as usize)
}

fn presence(case: &semio_framework_pack_json::Value) -> CadPresence {
    CadPresence { engagement_step: text(&case["engagementStep"]), engagement_pane: (!case["engagementPane"].is_null()).then(|| text(&case["engagementPane"])), ..CadPresence::default() }
}

fn drive(retirement: &mut Box<dyn store::ErasedSnapshotRetirement>, turns: usize) -> RetainedCloneProgress {
    let mut total = RetainedCloneProgress::default();
    for turn in 0..turns {
        let demand = retirement.next_demand(usize::MAX).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = retirement.close_step(grant).unwrap();
        let progress = step.progress();
        assert!(progress.fits(grant));
        total.copied_items += progress.copied_items;
        total.copied_bytes += progress.copied_bytes;
        total.released_bytes += progress.released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(retirement.terminal_is_empty());
            return total;
        }
        assert!(turn + 1 < turns, "CAD presence exceeded its fixed lifecycle bound");
    }
    unreachable!()
}

#[test]
fn retained_cad_presence_close_preserves_shared_roots_and_exact_grants() {
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::parse(include_str!("../../../🧫️fixtures/♻️retirement/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let value = presence(case);
        let oracle = value.clone();
        let expected = oracle.engagement_step.len() + oracle.engagement_pane.as_deref().map_or(0, str::len);
        assert_eq!(expected, case["expectedBytes"].as_u64().unwrap() as usize);
        let root = Arc::new(value);
        let reader = root.clone();
        let birth = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: CadPresenceRetirementFactory.retirement_birth_bytes(&root), maximum_release_bytes: 0, maximum_depth: 1 };
        let (mut first, receipt) = CadPresenceRetirementFactory.retire(root, birth).unwrap_or_else(|(error, _)| panic!("CAD presence birth refused: {error}"));
        assert!(receipt.fits(birth));
        drive(&mut first, 4096);
        assert_eq!(reader.as_ref(), &oracle);
        let weak = Arc::downgrade(&reader);
        let birth = RetainedCloneGrant { maximum_capacity_bytes: CadPresenceRetirementFactory.retirement_birth_bytes(&reader), ..birth };
        let (mut last, _) = CadPresenceRetirementFactory.retire(reader, birth).unwrap_or_else(|(error, _)| panic!("CAD presence birth refused: {error}"));
        let total = drive(&mut last, 4096);
        assert!(total.released_bytes >= expected);
        assert!(weak.upgrade().is_none());
    }
}

#[test]
fn cad_presence_empty_terminal_is_its_own_empty_witness() {
    assert!(terminal_is_empty(&empty_terminal()));
    assert!(!terminal_is_empty(&CadPresence::default()));
    drop(store_disposer());
}
