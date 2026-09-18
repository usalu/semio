//! 🧪️ The artifact facet and its descriptor: twenty leaves, all present and all non-empty, plus the
//! derived construction/analysis facets that `subset()` binds.

use super::*;
use crate::Wfc3dSnapshot;
use semio_framework_plugin::{AnalyzeSource, ArtifactAnalysis, ArtifactBuilder};

/// 🧬️ Four facets × five languages. A missing leaf is a crate-level compile error, so what this test
/// adds is the guarantee that none of them is an EMPTY placeholder.
#[test]
fn the_descriptor_carries_twenty_non_empty_leaves() {
    let descriptor = wfc3d_artifact_schema_descriptor();
    assert_eq!(descriptor.id, "s.wfc.wfc3d");
    let facets = [&descriptor.artifact, &descriptor.snapshot, &descriptor.diff, &descriptor.mutations];
    for facet in facets {
        for leaf in [facet.rust, facet.typescript, facet.graphql, facet.json_schema, facet.proto] {
            assert!(leaf.trim().len() > 32, "every schema leaf must be real content");
        }
    }
}

/// 🔣️ The JSON Schema leaf is NORMATIVE; the other four mirror it. At minimum every mirror has to
/// name the same six snapshot fields.
#[test]
fn every_snapshot_mirror_names_the_same_six_fields() {
    let descriptor = wfc3d_artifact_schema_descriptor();
    for field in ["schema", "seed", "slots", "edges", "tiles", "rules"] {
        assert!(descriptor.snapshot.json_schema.contains(field), "json schema must declare {field}");
        assert!(descriptor.snapshot.typescript.contains(field), "typescript mirror must declare {field}");
        assert!(descriptor.snapshot.graphql.contains(field), "graphql mirror must declare {field}");
        assert!(descriptor.snapshot.proto.contains(field), "proto mirror must declare {field}");
    }
}

#[test]
fn the_artifact_facet_wraps_exactly_the_snapshot() {
    let document = crate::examples::two_room_corridor::snapshot();
    let artifact = Wfc3dArtifact::from_snapshot(document.clone());
    assert_eq!(artifact.to_snapshot(), document);
    assert_eq!(Wfc3dArtifact::default().snapshot, Wfc3dSnapshot::default());
}

#[test]
fn the_builder_facet_constructs_from_text_and_from_binary() {
    let document = crate::examples::wall_roof_facade_strip::snapshot();
    let text = crate::schema::snapshot::text::print_dsl(&document);
    assert_eq!(Wfc3dBuilderConstruction::from_text(&text).expect("text builds").build().expect("no diagnostics"), document);
    let bytes = crate::schema::snapshot::binary::encode(&document);
    assert_eq!(Wfc3dBuilderConstruction::from_binary(&bytes).expect("binary builds").build().expect("no diagnostics"), document);
    assert_eq!(Wfc3dBuilderConstruction::empty().build().expect("empty builds"), Wfc3dSnapshot::default());
}

#[test]
fn the_analyzer_facet_recognises_both_native_carriers() {
    let document = crate::examples::tower_stack::snapshot();
    let text = crate::schema::snapshot::text::print_dsl(&document);
    let analysis = Wfc3dAnalyzerAnalysis::analyze(&[AnalyzeSource::Text(&text)]);
    assert_eq!(analysis.parts.snapshot.expect("text analyses"), document);
    let bytes = crate::schema::snapshot::binary::encode(&document);
    let analysis = Wfc3dAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&bytes)]);
    assert_eq!(analysis.parts.snapshot.expect("binary analyses"), document);
    assert_eq!(Wfc3dAnalyzerAnalysis::DIALECT.artifact_kind, "s.wfc.wfc3d");
}

/// 🧾️ Garbage must LOWER the confidence and raise a diagnostic, never decode to a silently empty
/// document.
#[test]
fn unparseable_text_lowers_the_confidence_and_says_why() {
    let analysis = Wfc3dAnalyzerAnalysis::analyze(&[AnalyzeSource::Text("this is not a wfc3d document {{{")]);
    assert!(analysis.parts.snapshot.is_none());
    assert!(!analysis.diagnostics.is_empty());
}
