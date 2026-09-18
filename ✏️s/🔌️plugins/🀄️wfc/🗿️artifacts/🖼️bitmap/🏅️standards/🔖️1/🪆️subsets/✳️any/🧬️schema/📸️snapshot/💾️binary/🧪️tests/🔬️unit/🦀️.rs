//! 🧪️ Snapshot binary facet — the pack round trip and the bounded retirement ladder.

use super::*;
use crate::schema::snapshot::{encode_base64, BitmapColor, BitmapInput, BitmapPinnedPixel};

fn scene() -> BitmapSnapshot {
    BitmapSnapshot {
        seed: 3,
        input: BitmapInput { width: 2, height: 2, palette: vec![BitmapColor::opaque(1, 2, 3), BitmapColor::opaque(4, 5, 6)], pixels: encode_base64(&[0, 1, 1, 0]) },
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

#[test]
fn a_displaced_snapshot_retires_in_bounded_steps_before_it_drops() {
    let mut live = scene();
    let mut retirement = decode_into(&mut live, &encode(&BitmapSnapshot::default())).expect("decode into a live projection");
    assert_eq!(live, BitmapSnapshot::default(), "the projection is replaced in place");
    assert!(!retirement.terminal_is_empty(), "the displaced snapshot still owns its members");
    let mut steps = 0;
    while !retirement.terminal_is_empty() {
        retirement.close_step(4, 8_192).expect("a retirement step");
        steps += 1;
        assert!(steps < 16, "the retirement ladder terminates");
    }
}

#[test]
fn a_starved_retirement_step_makes_no_progress_rather_than_dropping_inline() {
    let mut live = scene();
    let mut retirement = decode_into(&mut live, &encode(&BitmapSnapshot::default())).expect("decode into a live projection");
    let step = retirement.close_step(0, 0).expect("a starved step");
    assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    while !retirement.terminal_is_empty() {
        retirement.close_step(4, 8_192).expect("a retirement step");
    }
}
