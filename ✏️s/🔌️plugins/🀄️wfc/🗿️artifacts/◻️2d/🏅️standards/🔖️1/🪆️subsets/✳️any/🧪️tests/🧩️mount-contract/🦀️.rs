//! 🪝️ Language-agnostic wfc2d mount contract — apps, window kinds, codecs, examples, inference route.
//!
//! Every `#[path]` mount in the crate root is exercised transitively by naming the module it binds;
//! a stale mount is a compile error before it can become a silent gap.

use crate::examples::{hex_ring, terrain_ring, two_room_corridor, wall_roof_facade_strip};
use crate::standards::v1::subsets::any::io::snapshot::text::{parse_dsl, print_dsl};
use crate::{Wfc2dSnapshot, WFC_2D_DIALECT};
use store::ArtifactPack;

#[test]
fn editor_and_viewer_share_the_wfc2d_dialect() {
    assert_eq!(<crate::editor::wfc2d::Wfc2dEditor as semio_framework_plugin::ArtifactEditor>::DIALECT, WFC_2D_DIALECT);
    assert_eq!(<crate::viewer::wfc2d::Wfc2dViewer as semio_framework_plugin::ArtifactViewer>::DIALECT, WFC_2D_DIALECT);
}

#[test]
fn the_editor_declares_both_window_kinds() {
    let definition = crate::editor::wfc2d::create_wfc2d_editor();
    assert_eq!(definition.id, "s.wfc.wfc2d@1/*#editor");
    assert!(definition.window_kinds.iter().any(|window| window.id == "wfc-graph"));
    assert!(definition.window_kinds.iter().any(|window| window.id == "wfc-2d-preview"));
}

#[test]
fn every_example_round_trips_dsl_and_pack() {
    for document in crate::examples::documents() {
        let text = print_dsl(&document);
        assert!(!text.trim().is_empty());
        assert_eq!(parse_dsl(&text).expect("dsl"), document);
        let pack = ArtifactPack::encode_pack(&document);
        assert!(pack.len() > 64);
        assert_eq!(<Wfc2dSnapshot as ArtifactPack>::decode_pack(&pack).expect("pack"), document);
    }
}

#[test]
fn example_labels_are_localized_en_and_de() {
    assert_eq!(two_room_corridor::label(), semio_framework_plugin::LocalizedLabel::native("Two Rooms And A Corridor", "Zwei Räume und ein Korridor"));
    assert_eq!(wall_roof_facade_strip::label(), semio_framework_plugin::LocalizedLabel::native("Wall And Roof Facade Strip", "Wand-Dach-Fassadenstreifen"));
    assert_eq!(hex_ring::label(), semio_framework_plugin::LocalizedLabel::native("Hexagonal Ring", "Sechseckiger Ring"));
    assert_eq!(terrain_ring::label(), semio_framework_plugin::LocalizedLabel::native("Terrain Ring", "Gelände-Ring"));
}

/// 🧭️ The routed inference metadata names the owner, the artifact and the tool id the plugin root
/// advertises — one string, spelled once, on both sides of the roster.
#[test]
fn the_inference_route_is_declared_consistently() {
    let metadata = crate::schema::inferences::wfc2d_inference_metadata();
    assert_eq!(metadata.owner, "wfc");
    assert_eq!(metadata.artifact_kind, crate::WFC_2D_DOCUMENT_SCHEMA);
    assert_eq!(metadata.inference_schema, crate::schema::inferences::WFC_2D_INFERENCE_TOOL_ID);
    assert_eq!(crate::schema::inferences::WFC_2D_INFERENCE_TOOL_ID, "s.wfc.wfc2d.solve");
    assert_eq!(crate::schema::inferences::WFC_2D_INFERENCE_JOB_KIND, "semio.infer");
    assert_eq!(crate::schema::inferences::wfc2d_artifact_inference_descriptor().id, "s.wfc.wfc2d.solve");
}

/// 🧬️ The schema descriptor carries twenty non-empty leaves — four facets × five languages.
#[test]
fn the_schema_descriptor_carries_every_leaf() {
    let descriptor = crate::schema::wfc2d_artifact_schema_descriptor();
    assert_eq!(descriptor.id, "s.wfc.wfc2d");
    for facet in [&descriptor.artifact, &descriptor.snapshot, &descriptor.diff, &descriptor.mutations] {
        for leaf in [facet.rust, facet.typescript, facet.graphql, facet.json_schema, facet.proto] {
            assert!(leaf.len() > 32, "a schema leaf is empty or a stub");
        }
    }
}

/// 🚪️ The io declaration registers all five native languages and no foreign entries.
#[test]
fn the_io_declaration_registers_the_native_languages() {
    let io = crate::standards::v1::subsets::any::io::io();
    assert!(io.native.snapshot.text.is_some() && io.native.snapshot.binary.is_some());
    assert!(io.native.mutations.text.is_some() && io.native.mutations.binary.is_some());
    assert!(io.native.diff.text.is_some());
    assert!(io.entries.is_empty(), "wfc2d declares no foreign interchange hop");
}
