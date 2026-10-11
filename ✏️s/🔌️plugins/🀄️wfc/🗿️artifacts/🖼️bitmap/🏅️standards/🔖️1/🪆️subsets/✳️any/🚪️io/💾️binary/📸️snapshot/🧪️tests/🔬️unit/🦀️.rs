//! 🧪️ Snapshot binary facet — the pack round trip and the bounded retirement ladder.

use crate::standards::v1::subsets::any::io::binary::snapshot::*;
use crate::schema::snapshot::{BitmapColor, BitmapInput, BitmapPinnedPixel};

fn scene() -> BitmapSnapshot {
    BitmapSnapshot {
        seed: 3,
        input: BitmapInput { width: 2, height: 2, palette: vec![BitmapColor::opaque(1, 2, 3), BitmapColor::opaque(4, 5, 6)], pixels: ([0, 1, 1, 0]).to_vec() },
        pinned: vec![BitmapPinnedPixel { x: 1, y: 1, color: 0 }],
        ..BitmapSnapshot::default()
    }
}

#[test]
fn the_pack_codec_round_trips_the_document() {
    let snapshot = scene();
    let bytes = encode(&snapshot);
    assert!(bytes.len() > 16);
    assert_eq!(decode(&bytes).expect("pack decodes"), snapshot);
}

#[test]
fn the_normative_protocol_names_this_facets_dialect() {
    assert!(COMPONENT_PROTOCOL_SEMIO.starts_with("dialect protocol"));
    assert!(COMPONENT_PROTOCOL_SEMIO.contains("protocol wfcbitmap.snapshot"));
    assert!(COMPONENT_PROTOCOL_PATH.ends_with("📡️.protocol.semio"));
}

/// 🎟️ The exact frame capacity one displaced snapshot's retirement is born with.
fn admission() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<BitmapSnapshot>(), maximum_depth: 1, ..Default::default() }
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

#[test]
fn a_displaced_snapshot_retires_in_bounded_steps_before_it_drops() {
    let mut live = scene();
    let (mut retirement, birth) = decode_into(&mut live, &encode(&BitmapSnapshot::default()), admission()).expect("decode into a live projection");
    assert!(birth.fits(admission()));
    assert_eq!(live, BitmapSnapshot::default(), "the projection is replaced in place");
    assert!(!retirement.terminal_is_empty(), "the displaced snapshot still owns its members");
    assert!(retire_all(&mut retirement) >= 1, "the retirement ladder releases the displaced members");
}

#[test]
fn a_starved_retirement_step_makes_no_progress_rather_than_dropping_inline() {
    let mut live = scene();
    let (mut retirement, _) = decode_into(&mut live, &encode(&BitmapSnapshot::default()), admission()).expect("decode into a live projection");
    let starved = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, ..admission() };
    assert_eq!(retirement.close_step(starved).expect("a starved step is not an error"), semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()));
    retire_all(&mut retirement);
}
