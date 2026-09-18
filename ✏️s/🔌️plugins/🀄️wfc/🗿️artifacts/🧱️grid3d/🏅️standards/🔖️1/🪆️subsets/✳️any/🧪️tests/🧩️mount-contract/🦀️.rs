//! 🧩️ Language-agnostic `s.wfc.grid3d` mount contract — apps, window kinds, codecs, examples, and the
//! subset declaration the plugin root binds.

use crate::examples::{blocks, pipes_3d};
use crate::schema::snapshot::text::{parse_dsl, print_dsl};
use crate::{Grid3dSnapshot, WFC_GRID3D_DIALECT};
use store::ArtifactPack;

#[test]
fn editor_and_viewer_share_the_grid3d_dialect() {
    assert_eq!(<crate::editor::grid3d::Grid3dEditor as semio_framework_plugin::ArtifactEditor>::DIALECT, WFC_GRID3D_DIALECT);
    assert_eq!(<crate::viewer::grid3d::Grid3dViewer as semio_framework_plugin::ArtifactViewer>::DIALECT, WFC_GRID3D_DIALECT);
}

#[test]
fn the_editor_declares_both_window_kinds_under_the_canonical_app_id() {
    let definition = crate::editor::grid3d::create_grid3d_editor();
    assert_eq!(definition.id, "s.wfc.grid3d@1/*#editor");
    assert!(definition.window_kinds.iter().any(|window| window.id == "wfc-grid3d-grid"));
    assert!(definition.window_kinds.iter().any(|window| window.id == "wfc-grid3d-preview"));
}

#[test]
fn examples_round_trip_dsl_and_pack() {
    for snapshot in [blocks::snapshot(), pipes_3d::snapshot()] {
        let text = print_dsl(&snapshot);
        assert!(!text.trim().is_empty());
        assert_eq!(parse_dsl(&text).expect("dsl"), snapshot);
        let pack = ArtifactPack::encode_pack(&snapshot);
        assert!(pack.len() > 64);
        assert_eq!(<Grid3dSnapshot as ArtifactPack>::decode_pack(&pack).expect("pack"), snapshot);
    }
}

#[test]
fn example_labels_are_localized_en_and_de() {
    assert_eq!(blocks::label(), semio_framework_plugin::LocalizedLabel::native("Building Blocks", "Bauklötze"));
    assert_eq!(pipes_3d::label(), semio_framework_plugin::LocalizedLabel::native("3D Pipes", "3D-Rohre"));
}

#[test]
fn the_schema_descriptor_carries_all_twenty_handcrafted_leaves() {
    let descriptor = crate::schema::grid3d_artifact_schema_descriptor();
    assert_eq!(descriptor.id, "s.wfc.grid3d");
    for facet in [&descriptor.artifact, &descriptor.snapshot, &descriptor.diff, &descriptor.mutations] {
        for leaf in [facet.rust, facet.typescript, facet.graphql, facet.json_schema, facet.proto] {
            assert!(!leaf.trim().is_empty(), "every schema leaf must be authored");
        }
    }
}

#[test]
fn the_subset_declares_its_native_codecs_and_its_solve_inference() {
    let io = crate::standards::v1::subsets::any::io();
    assert!(io.native.snapshot.text.is_some() && io.native.snapshot.binary.is_some());
    assert!(io.native.mutations.text.is_some() && io.native.mutations.binary.is_some());
    let descriptor = crate::schema::inferences::grid3d_artifact_inference_descriptor();
    assert_eq!(descriptor.id, "s.wfc.grid3d.solve");
}
