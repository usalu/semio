//! 🧪️ Snapshot text facet — the `.wfcbitmap` DSL round trip and the normative grammar's own shape.

use super::*;
use crate::schema::snapshot::{encode_base64, BitmapColor, BitmapInput, BitmapPinnedPixel};

fn scene() -> BitmapSnapshot {
    BitmapSnapshot {
        seed: 42,
        input: BitmapInput { width: 2, height: 2, palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)], pixels: encode_base64(&[0, 1, 1, 0]) },
        pinned: vec![BitmapPinnedPixel { x: 0, y: 0, color: 1 }],
        ..BitmapSnapshot::default()
    }
}

#[test]
fn the_dsl_round_trips_the_whole_document() {
    let snapshot = scene();
    let text = print_dsl(&snapshot);
    assert!(!text.trim().is_empty());
    assert_eq!(parse_dsl(&text).expect("printed text parses"), snapshot);
}

#[test]
fn an_empty_body_parses_to_the_default_document() {
    assert_eq!(parse_dsl("").expect("empty text parses"), BitmapSnapshot::default());
}

#[test]
fn the_normative_grammar_names_this_facets_dialect() {
    assert!(COMPONENT_GRAMMAR_SEMIO.starts_with("dialect grammar"));
    assert!(COMPONENT_GRAMMAR_SEMIO.contains("grammar wfcbitmap.snapshot"));
    assert!(COMPONENT_GRAMMAR_PATH.ends_with("📖️.grammar.semio"));
}

#[test]
fn the_extension_and_envelope_are_this_artifacts_own() {
    assert_eq!(<BitmapSnapshot as store::ArtifactDsl>::EXTENSION, "wfcbitmap");
    assert_eq!(<BitmapSnapshot as store::ArtifactDsl>::envelope_id(), "wfc.bitmap");
}
