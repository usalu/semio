//! 🧪️ Language-agnostic bitmap mount contract — the identities, window kinds, codecs, mutation
//! vocabulary and examples every other language's mirror of this subset measures itself against.
//! The committed `🧫️fixtures/🧩️mount-contract/🔣️.json` is the shared statement; this file is its
//! Rust half.

use crate::examples::{flowers_24, rooms_16};
use crate::schema::snapshot::text::{parse_dsl, print_dsl};
use crate::{BitmapSnapshot, WFC_BITMAP_DIALECT};
use store::ArtifactPack;

const CONTRACT: &str = include_str!("../../🧫️fixtures/🧩️mount-contract/🔣️.json");

fn contract() -> serde_json::Value {
    serde_json::from_str(CONTRACT).expect("the mount contract decodes")
}

fn strings(value: &serde_json::Value, key: &str) -> Vec<String> {
    value[key].as_array().unwrap_or_else(|| panic!("the contract declares '{key}'")).iter().map(|entry| entry.as_str().expect("a text entry").to_string()).collect()
}

#[test]
fn editor_and_viewer_share_the_bitmap_dialect() {
    assert_eq!(<crate::editor::bitmap::BitmapEditor as semio_framework_plugin::ArtifactEditor>::DIALECT, WFC_BITMAP_DIALECT);
    assert_eq!(<crate::viewer::bitmap::BitmapViewer as semio_framework_plugin::ArtifactViewer>::DIALECT, WFC_BITMAP_DIALECT);
}

#[test]
fn the_declared_app_ids_and_kind_id_are_the_committed_ones() {
    let contract = contract();
    assert_eq!(crate::editor::bitmap::create_bitmap_editor().id, contract["editorAppId"].as_str().expect("editorAppId"));
    assert_eq!(crate::viewer::bitmap::create_bitmap_viewer().id, contract["viewerAppId"].as_str().expect("viewerAppId"));
    assert_eq!(crate::artifact_kind().id, contract["artifactKindId"].as_str().expect("artifactKindId"));
    assert_eq!(crate::inferences::BITMAP_INFERENCE_TOOL_ID, contract["inferenceToolId"].as_str().expect("inferenceToolId"));
}

#[test]
fn both_surfaces_declare_exactly_the_committed_window_kinds() {
    let declared = strings(&contract(), "windowKindIds");
    for definition in [crate::editor::bitmap::create_bitmap_editor(), crate::viewer::bitmap::create_bitmap_viewer()] {
        let ids: Vec<String> = definition.window_kinds.iter().map(|window| window.id.clone()).collect();
        assert_eq!(ids, declared, "surface {} declares the wrong window kinds", definition.id);
    }
}

#[test]
fn the_mutation_vocabulary_is_the_committed_one() {
    assert_eq!(crate::mutations::KINDS.to_vec(), strings(&contract(), "mutationKinds"));
}

#[test]
fn examples_round_trip_dsl_and_pack() {
    for snapshot in [rooms_16::snapshot(), flowers_24::snapshot()] {
        let text = print_dsl(&snapshot);
        assert!(!text.trim().is_empty());
        assert_eq!(parse_dsl(&text).expect("dsl"), snapshot);
        let pack = ArtifactPack::encode_pack(&snapshot);
        assert!(pack.len() > 64);
        assert_eq!(<BitmapSnapshot as ArtifactPack>::decode_pack(&pack).expect("pack"), snapshot);
    }
}

#[test]
fn every_committed_example_row_matches_its_builder() {
    let contract = contract();
    let rows = contract["examples"].as_array().expect("examples");
    let built = [("rooms-16", rooms_16::snapshot()), ("flowers-24", flowers_24::snapshot())];
    assert_eq!(rows.len(), built.len());
    for (row, (id, snapshot)) in rows.iter().zip(built) {
        assert_eq!(row["id"].as_str(), Some(id));
        assert_eq!(row["inputWidth"].as_u64(), Some(u64::from(snapshot.input.width)));
        assert_eq!(row["inputHeight"].as_u64(), Some(u64::from(snapshot.input.height)));
        assert_eq!(row["paletteSize"].as_u64(), Some(snapshot.input.palette.len() as u64));
    }
}

#[test]
fn example_labels_are_localized_en_and_de() {
    assert_eq!(rooms_16::label(), semio_framework_plugin::LocalizedLabel::native("Rooms 16", "Räume 16"));
    assert_eq!(flowers_24::label(), semio_framework_plugin::LocalizedLabel::native("Flowers 24", "Blumen 24"));
}
