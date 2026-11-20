//! 🧪️ Snapshot binary facet — the pack round trip.

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
