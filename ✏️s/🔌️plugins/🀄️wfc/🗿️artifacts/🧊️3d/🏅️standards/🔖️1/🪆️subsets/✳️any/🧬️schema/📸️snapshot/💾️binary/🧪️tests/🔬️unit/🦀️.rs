//! 🧪️ The binary facet: pack round-trips, and a displaced document is released in BOUNDED steps
//! rather than dropped whole.

use super::*;
use store::SnapshotRetirementStep;

#[test]
fn every_example_round_trips_through_the_pack_codec() {
    for document in [crate::examples::two_room_corridor::snapshot(), crate::examples::wall_roof_facade_strip::snapshot(), crate::examples::tower_stack::snapshot()] {
        let bytes = encode(&document);
        assert!(bytes.len() > 64);
        assert_eq!(decode(&bytes).expect("pack decodes"), document);
    }
}

#[test]
fn empty_bytes_decode_to_the_default_document() {
    assert_eq!(decode(&[]).expect("empty pack is the default document"), Wfc3dSnapshot::default());
}

/// ♻️ A decode-in-place hands back a retirement that makes progress one collection at a time and
/// refuses to advance under a budget it cannot honour.
#[test]
fn a_displaced_document_retires_in_bounded_steps() {
    let mut live = crate::examples::tower_stack::snapshot();
    let replacement = encode(&crate::examples::two_room_corridor::snapshot());
    let mut retirement = decode_into(&mut live, &replacement).expect("decode in place");
    assert_eq!(live, crate::examples::two_room_corridor::snapshot());
    assert!(matches!(retirement.close_step(1, 0).expect("a starved step is pending"), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    let mut steps = 0;
    while !retirement.terminal_is_empty() {
        retirement.close_step(64, 1 << 20).expect("a funded step makes progress");
        steps += 1;
        assert!(steps <= 8, "the ladder must terminate");
    }
    assert!(matches!(retirement.close_step(64, 1 << 20).expect("a finished ladder is complete"), SnapshotRetirementStep::Complete));
}
