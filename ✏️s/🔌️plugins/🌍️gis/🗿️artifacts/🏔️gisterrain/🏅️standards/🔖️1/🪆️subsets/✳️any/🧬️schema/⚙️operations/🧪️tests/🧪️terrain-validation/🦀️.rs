//! 🛂️ Terrain mutation diagnostics and canonical authored elevation.

use crate::diff::GisTerrainDiff;
use crate::mutations::{ChangeExaggeration, ChangeImportedFeatures, GisTerrainMutation, ReplaceElevationSource};
use crate::schema::{imported_map::ImportedMap, snapshot::ElevationSource};
use crate::GisTerrainSnapshot;
use protocol::ColdMutation;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonTree};
use semio_framework_value::DslValue;
use serde_json::{json, Value};

#[test]
fn terrain_validation_refuses_invalid_fields_with_diagnostics() {
    let mutations = [
        GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: f64::NAN }),
        GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures {
            new_imported_map: Some(ImportedMap { positions: vec![DslValue::Null], ..Default::default() }),
        }),
        GisTerrainMutation::ReplaceElevationSource(ReplaceElevationSource {
            new_elevation_source: Some(ElevationSource { missing_samples: None, bounds: [7.1, 46.0, 7.2, 46.1], rows_millimetres: vec![] }),
        }),
    ];
    let base = GisTerrainSnapshot::default();
    let actual: Vec<Value> = mutations.iter().map(|mutation| {
        let outcome = mutation.diff(&base);
        assert_eq!(outcome.diff(), &GisTerrainDiff::default());
        let protocol::MutationAdmission::Refused(outcome) = outcome.admit() else { panic!("invalid terrain mutation was admitted") };
        let [message] = outcome.messages() else { panic!("expected one targeted terrain diagnostic") };
        serde_json::from_str(&semio_framework_pack_json::to_json_string(message)).expect("diagnostic JSON")
    }).collect();
    let expected: Value = serde_json::from_str(include_str!("🔣️.json")).expect("terrain diagnostic fixture");
    assert_eq!(actual.as_slice(), expected["diagnostics"].as_array().expect("ordered diagnostics").as_slice());
}

#[test]
fn terrain_validation_projects_authored_elevation_into_canonical_json() {
    let fixture = semio_framework_test_fixtures::fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🏔️replace-elevation-source/🏔️authors/🦠️mutation/🔣️.json");
    let mutation: GisTerrainMutation = semio_framework_pack_json::from_json_str(fixture, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored elevation decodes");
    let GisTerrainMutation::ReplaceElevationSource(mutation) = mutation else { panic!("expected elevation replacement") };
    let source = mutation.canonical_tree_child(0).expect("newElevationSource is the canonical mutation field");
    assert!(matches!(source.canonical_tree_node().unwrap(), Node::Object(2)));
    let oracle: Value = serde_json::from_str(fixture).unwrap();
    let expected = &oracle["ReplaceElevationSource"]["newElevationSource"];
    let bounds = source.canonical_tree_child(1).unwrap();
    assert!(matches!(bounds.canonical_tree_node().unwrap(), Node::Array(4)));
    for (index, bound) in expected["bounds"].as_array().unwrap().iter().enumerate() {
        let Node::F64(actual) = bounds.canonical_tree_child(index).unwrap().canonical_tree_node().unwrap() else { panic!("expected floating-point bound") };
        assert_eq!(json!(actual), *bound);
    }
    let rows = source.canonical_tree_child(0).unwrap();
    let expected_rows = expected["rowsMillimetres"].as_array().unwrap();
    assert!(matches!(rows.canonical_tree_node().unwrap(), Node::Array(count) if count == expected_rows.len()));
    for (index, row) in expected_rows.iter().enumerate() {
        let samples = rows.canonical_tree_child(index).unwrap();
        assert!(matches!(samples.canonical_tree_node().unwrap(), Node::Array(count) if count == row.as_array().unwrap().len()));
        for (sample_index, sample) in row.as_array().unwrap().iter().enumerate() {
            let Node::I64(actual) = samples.canonical_tree_child(sample_index).unwrap().canonical_tree_node().unwrap() else { panic!("expected integer sample") };
            assert_eq!(json!(actual), *sample);
        }
    }
}
