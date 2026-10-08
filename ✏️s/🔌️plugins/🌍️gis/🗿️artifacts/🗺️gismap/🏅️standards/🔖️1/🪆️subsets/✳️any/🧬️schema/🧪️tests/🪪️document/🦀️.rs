//! 🧪️ Map codecs and mutations preserve exact durable child identities.
use crate::{GisMapSnapshot, MapFeature};
use crate::schema::{GisMapArtifact, diff::{GisMapDiff, GisMapFeaturesDelta}};
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn map_document_contract_preserves_all_children_and_dynamic_feature_payloads() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document/🔣️.json")).unwrap();
    let artifact: GisMapArtifact = semio_framework_pack_json::from_json_str(&fixture["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let snapshot: GisMapSnapshot = semio_framework_pack_json::from_json_str(&fixture["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(GisMapArtifact::from_snapshot(snapshot.clone()), artifact);
    assert_eq!(store::ChildRestoreProjection::from_snapshot(&snapshot).unwrap().len(), 3);
    assert_eq!(GisMapSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
    assert_eq!(GisMapSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
    assert_eq!(protocol::apply_diff(&GisMapDiff::default(), &snapshot).unwrap(), snapshot);
    let delta = GisMapDiff { positions: Some(GisMapFeaturesDelta::insertion(snapshot.positions.len(), MapFeature { id: "scalar".into(), data: semio_framework_value::DslValue::Null })), ..Default::default() };
    let changed = protocol::apply_diff(&delta, &snapshot).unwrap();
    assert_eq!((&changed.drawing, &changed.image, &changed.value), (&snapshot.drawing, &snapshot.image, &snapshot.value));
    let mut mutated = snapshot.clone();
    let mutation = crate::mutations::GisMapMutation::CreatePosition(crate::mutations::create_position::CreatePosition {
        index: snapshot.positions.len(),
        item: MapFeature { id: "created".into(), data: semio_framework_value::DslValue::Null },
    });
    crate::standards::v1::subsets::any::io::text::mutations::apply_gis_map_mutation(&mut mutated, &mutation).unwrap();
    assert_eq!(mutated.positions.len(), snapshot.positions.len() + 1);
    assert_eq!((&mutated.drawing, &mutated.image, &mutated.value), (&snapshot.drawing, &snapshot.image, &snapshot.value));
    for input in fixture["invalidDocuments"].as_array().unwrap() {
        assert!(semio_framework_pack_json::from_json_str::<GisMapArtifact>(&input.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "invalid artifact accepted: {input}");
        assert!(semio_framework_pack_json::from_json_str::<GisMapSnapshot>(&input.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "invalid snapshot accepted: {input}");
    }
    for input in fixture["invalidDiffs"].as_array().unwrap() {
        assert!(semio_framework_pack_json::from_json_str::<GisMapDiff>(&input.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "invalid diff accepted: {input}");
    }
    let actual: GisMapDiff = semio_framework_pack_json::from_json_str(&fixture["diff"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&actual)).unwrap(), fixture["diff"]);
}

#[test]
fn default_document_boot_child_projection_and_genesis_packs_are_valid() {
    use crate::standards::v1::subsets::any::io::text::snapshot::default_document;
    let snapshot = default_document();
    let projection = store::ChildRestoreProjection::from_snapshot(&snapshot).expect("reuse-map default document child refs");
    assert_eq!(projection.len(), 2, "drawing and value; image is absent on the default map");
    assert!(crate::genesis_gis_map_child_pack(&snapshot, "drawing", &snapshot.drawing.child_id).is_some());
    assert!(crate::genesis_gis_map_child_pack(&snapshot, "value", &snapshot.value.child_id).is_some());
    assert!(crate::genesis_gis_map_child_pack(&snapshot, "drawing", "wrong-id").is_none());
}
