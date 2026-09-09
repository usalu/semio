//! 🧪️ Terrain codecs preserve the exact durable child owner.
use crate::{GisTerrainDiff, GisTerrainSnapshot};
use crate::schema::GisTerrainArtifact;
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn terrain_document_contract_preserves_mesh_identity_through_conversion_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")).unwrap();
    for row in fixture["diffCases"].as_array().unwrap() {
        let before: GisTerrainSnapshot = dsl::json::from_json_str(&row["before"].to_string()).unwrap();
        let diff: GisTerrainDiff = dsl::json::from_json_str(&row["diff"].to_string()).unwrap();
        let after = protocol::MutationDiff::apply(&diff, &before).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&after)).unwrap(), row["after"], "{}", row["name"]);
    }
    let artifact: GisTerrainArtifact = dsl::json::from_json_str(&fixture["document"].to_string()).unwrap();
    let snapshot: GisTerrainSnapshot = dsl::json::from_json_str(&fixture["document"].to_string()).unwrap();
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(GisTerrainArtifact::from_snapshot(snapshot.clone()), artifact);
    assert_eq!(protocol::MutationDiff::apply(&GisTerrainDiff::default(), &snapshot).unwrap(), snapshot);
    assert_eq!(GisTerrainDiff::default().apply_to_artifact(&artifact).unwrap(), artifact);
    assert_eq!(store::ChildRestoreProjection::from_snapshot(&snapshot).unwrap().len(), 1);
    assert_eq!(GisTerrainSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
    assert_eq!(GisTerrainSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
    let replacement = GisTerrainDiff { artifact: Some(Box::new(artifact)), ..Default::default() };
    assert_eq!(protocol::MutationDiff::apply(&replacement, &GisTerrainSnapshot::default()).unwrap(), snapshot);
    let generated = crate::gis_terrain_mesh_child_handle("2.5|{}");
    assert_eq!(generated.child_id, generated.target.artifact_id);
    for input in fixture["invalidDocuments"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<GisTerrainArtifact>(&input.to_string()).is_err());
        assert!(dsl::json::from_json_str::<GisTerrainSnapshot>(&input.to_string()).is_err());
    }
    for input in fixture["invalidDiffs"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<GisTerrainDiff>(&input.to_string()).is_err());
    }
    let actual: GisTerrainDiff = dsl::json::from_json_str(&fixture["diff"].to_string()).unwrap();
    let mut edited = snapshot.clone();
    let mutation = crate::schema::mutations::GisTerrainMutation::ChangeExaggeration(crate::schema::mutations::ChangeExaggeration { new_exaggeration: 3.0 });
    crate::mutations::apply_gis_terrain_mutation(&mut edited, &mutation).unwrap();
    assert_eq!(edited.mesh, snapshot.mesh);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&actual)).unwrap(), fixture["diff"]);
    eprintln!("[DEBUG] Terrain JSON/text/Pack, four neutral deltas and real scalar mutation preserve exact mesh identity");
}
