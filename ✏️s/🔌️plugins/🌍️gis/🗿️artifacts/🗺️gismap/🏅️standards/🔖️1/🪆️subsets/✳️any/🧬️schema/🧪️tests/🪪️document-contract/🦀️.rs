//! 🧪️ Map codecs and mutations preserve exact durable child identities.
use crate::{GisMapSnapshot, MapFeature};
use crate::schema::{GisMapArtifact, diff::{GisMapDiff, GisMapFeaturesDelta}};
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn map_document_contract_preserves_all_children_and_dynamic_feature_payloads() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")).unwrap();
    let artifact: GisMapArtifact = dsl::json::from_json_str(&fixture["document"].to_string()).unwrap();
    let snapshot: GisMapSnapshot = dsl::json::from_json_str(&fixture["document"].to_string()).unwrap();
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(GisMapArtifact::from_snapshot(snapshot.clone()), artifact);
    assert_eq!(store::ChildRestoreProjection::from_snapshot(&snapshot).unwrap().len(), 3);
    assert_eq!(GisMapSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
    assert_eq!(GisMapSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
    assert_eq!(protocol::MutationDiff::apply(&GisMapDiff::default(), &snapshot).unwrap(), snapshot);
    let replacement = GisMapDiff { artifact: Some(Box::new(artifact)), ..Default::default() };
    assert_eq!(protocol::MutationDiff::apply(&replacement, &GisMapSnapshot::default()).unwrap(), snapshot);
    let delta = GisMapDiff { positions: Some(GisMapFeaturesDelta { added: vec![MapFeature { id: "scalar".into(), data: dsl::DslValue::Null }], ..Default::default() }), ..Default::default() };
    let changed = protocol::MutationDiff::apply(&delta, &snapshot).unwrap();
    assert_eq!((&changed.drawing, &changed.image, &changed.value), (&snapshot.drawing, &snapshot.image, &snapshot.value));
    let mut mutated = snapshot.clone();
    let mutation = crate::mutations::GisMapMutation::CreatePosition(crate::mutations::create_position::CreatePosition {
        index: snapshot.positions.len(),
        item: MapFeature { id: "created".into(), data: dsl::DslValue::Null },
    });
    crate::mutations::apply_gis_map_mutation(&mut mutated, &mutation).unwrap();
    assert_eq!(mutated.positions.len(), snapshot.positions.len() + 1);
    assert_eq!((&mutated.drawing, &mutated.image, &mutated.value), (&snapshot.drawing, &snapshot.image, &snapshot.value));
    for input in fixture["invalidDocuments"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<GisMapArtifact>(&input.to_string()).is_err(), "invalid artifact accepted: {input}");
        assert!(dsl::json::from_json_str::<GisMapSnapshot>(&input.to_string()).is_err(), "invalid snapshot accepted: {input}");
    }
    for input in fixture["invalidDiffs"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<GisMapDiff>(&input.to_string()).is_err(), "invalid diff accepted: {input}");
    }
    let actual: GisMapDiff = dsl::json::from_json_str(&fixture["diff"].to_string()).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&actual)).unwrap(), fixture["diff"]);
    eprintln!("[DEBUG] Map JSON/text/Pack and replacement retain drawing/image/value identities; dynamic feature payloads use the shared value schema");
}
