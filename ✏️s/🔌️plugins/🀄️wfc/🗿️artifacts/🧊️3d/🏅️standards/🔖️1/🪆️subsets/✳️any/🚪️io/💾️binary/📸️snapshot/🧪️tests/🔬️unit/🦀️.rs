//! 🧪️ The binary facet: pack round-trips, and a displaced document is released in BOUNDED steps
//! rather than dropped whole.

use crate::standards::v1::subsets::any::io::binary::snapshot::*;

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

/// 🎟️ The exact frame capacity one displaced snapshot's retirement is born with.
fn admission() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<Wfc3dSnapshot>(), maximum_depth: 1, ..Default::default() }
}

/// ♻️ Drains a retirement with grants quoted from its own next demand and returns the step count.
fn retire_all(retirement: &mut Box<dyn store::ErasedSnapshotRetirement>) -> usize {
    let mut steps = 0;
    while !retirement.terminal_is_empty() {
        let demand = retirement.next_demand(1 << 16).expect("a retained snapshot quotes its next demand");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = retirement.close_step(grant).expect("a funded step makes progress");
        assert!(step.progress().fits(grant));
        steps += 1;
        assert!(steps < 100_000, "the retirement ladder must terminate");
    }
    steps
}

/// ♻️ A decode-in-place hands back a retirement that makes progress only under a funded grant.
#[test]
fn a_displaced_document_retires_in_bounded_steps() {
    let mut live = crate::examples::tower_stack::snapshot();
    let replacement = encode(&crate::examples::two_room_corridor::snapshot());
    let (mut retirement, birth) = decode_into(&mut live, &replacement, admission()).expect("decode in place");
    assert!(birth.fits(admission()));
    assert_eq!(live, crate::examples::two_room_corridor::snapshot());
    let starved = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, ..admission() };
    assert_eq!(retirement.close_step(starved).expect("a starved step is not an error"), semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()));
    assert!(retire_all(&mut retirement) >= 1);
    let funded = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_depth: 1, ..Default::default() };
    assert!(matches!(retirement.close_step(funded).expect("a finished ladder is complete"), semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)));
}
