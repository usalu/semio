//! 🧪️ Snapshot text facet — the `.wfcbitmap` DSL round trip and the normative grammar's own shape.

use crate::standards::v1::subsets::any::io::text::snapshot::*;
use crate::schema::snapshot::{BitmapColor, BitmapInput, BitmapPinnedPixel};
use crate::standards::v1::subsets::any::io::text::snapshot::{encode_base64};

fn scene() -> BitmapSnapshot {
    BitmapSnapshot {
        seed: 42,
        input: BitmapInput { width: 2, height: 2, palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)], pixels: ([0, 1, 1, 0]).to_vec() },
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

#[test]
fn intrinsic_pixels_have_physical_byte_fixtures() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/byte-fields/🔣️.json")).unwrap();
 for vector in fixture["vectors"].as_array().unwrap(){let bytes:Vec<u8>=vector["bytes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();let text=vector["base64"].as_str().unwrap();assert_eq!(super::encode_base64(&bytes),text);assert_eq!(super::decode_base64(text),Some(bytes.clone()));let region=crate::diff::BitmapPixelRegion{pixels:bytes.clone(),..Default::default()};let physical=crate::standards::v1::subsets::any::io::text::bitmap_json_encode(&region);let oracle:serde_json::Value=serde_json::from_str(&physical).unwrap();assert_eq!(oracle["pixels"],text);assert!(<crate::diff::BitmapPixelRegion as semio_framework_value::FromValue>::from_value(oracle.clone().into()).is_err());let admitted:crate::diff::BitmapPixelRegion=crate::standards::v1::subsets::any::io::text::bitmap_json_decode(&physical).unwrap();assert_eq!(admitted.pixels,bytes);}
 for text in fixture["refused"].as_array().unwrap(){assert!(super::decode_base64(text.as_str().unwrap()).is_none());}
 eprintln!("[DEBUG] bitmap intrinsic pixel fixtures=5 physical JSON serde oracle=agree refusals=6");
}
