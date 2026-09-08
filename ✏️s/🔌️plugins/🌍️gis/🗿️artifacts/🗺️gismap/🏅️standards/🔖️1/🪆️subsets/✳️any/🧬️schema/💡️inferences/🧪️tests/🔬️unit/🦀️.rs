
use super::*;
use crate::{MapFeature, gis_map_snapshot_with_derived_children};
use protocol::Inference;

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = GisMapSnapshot { positions: vec![MapFeature { id: "p1".into(), data: dsl::DslValue::from(serde_json::json!({ "lon": 1.0, "lat": 2.0 })) }], routes: Vec::new(), regions: Vec::new(), ..Default::default() };
    assert_eq!(GisMapInference::infer(&snapshot), GisMapInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(GisMapInference::infer(&GisMapSnapshot::default()), GisMapInference::default());
}

#[semio_framework_async_macros::async_test]
async fn map_create_region_group_work_stabilizes_parent_drawing_value_without_image() {
    use crate::mutations::apply_gis_map_mutation;
    use crate::schema::{gis_map_descriptor_json, gis_map_snapshot_to_drawing};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::mutations::apply_semio_drawing_mutation;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::mutations::apply_semio_value_mutation;

    let feature = |id: &str, data: serde_json::Value| MapFeature { id: id.into(), data: dsl::DslValue::from(data) };
    let snapshot = gis_map_snapshot_with_derived_children(GisMapSnapshot {
        positions: vec![feature("point-a", serde_json::json!({ "id": "point-a", "lon": 7, "lat": 47 }))],
        routes: vec![feature("route-a", serde_json::json!({ "id": "route-a", "points": [[8, 46], [9, 48]] }))],
        ..Default::default()
    });
    let inferred = GisMapInference::infer(&snapshot);
    let work = inferred.create_region_group_work(&snapshot, "11111111111111111111111111111111").expect("typed group work");
    assert_eq!(work.drawing_child.child_id, "gismap-drawing");
    assert_eq!(work.value_child.child_id, "gismap-value");
    assert!(snapshot.image.is_none());

    let mut parent_after = snapshot.clone();
    apply_gis_map_mutation(&mut parent_after, &work.parent).expect("parent applies");
    assert_eq!(parent_after.drawing, snapshot.drawing);
    assert_eq!(parent_after.value, snapshot.value);
    let before_drawing = gis_map_snapshot_to_drawing(&snapshot);
    let after_drawing = gis_map_snapshot_to_drawing(&parent_after);
    let mut projected_drawing = before_drawing.clone();
    apply_semio_drawing_mutation(&mut projected_drawing, &work.drawing);
    assert_eq!(projected_drawing, after_drawing);
    for inverse in &work.drawing_inverse {
        apply_semio_drawing_mutation(&mut projected_drawing, inverse);
    }
    assert_eq!(projected_drawing, before_drawing);

    let before_value = crate::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(&snapshot));
    let after_value = crate::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(&parent_after));
    let mut projected_value = before_value.clone();
    apply_semio_value_mutation(&mut projected_value, &work.value);
    assert_eq!(projected_value, after_value);
    for inverse in &work.value_inverse {
        apply_semio_value_mutation(&mut projected_value, inverse);
    }
    assert_eq!(projected_value, before_value);

    for inverse in &work.parent_inverse {
        apply_gis_map_mutation(&mut parent_after, inverse).expect("parent inverse applies");
    }
    assert_eq!(parent_after, snapshot);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧫️fixtures/🧩️map-create-region-group/🔣️.json")).expect("neutral Map membership corpus");
    for row in fixture["membershipCases"].as_array().expect("membership cases") {
        let mut candidate = snapshot.clone();
        candidate.drawing.child_id = row["drawingChildId"].as_str().unwrap().into();
        candidate.value.child_id = row["valueChildId"].as_str().unwrap().into();
        candidate.image = row["imageChildId"].as_str().map(|id| {
            store::ArtifactChild::new(id.to_owned(), store::os_io::ArtifactRef { artifact_id: id.to_owned(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } })
        });
        let supplied_image = candidate.image.clone();
        if row["deriveChildren"].as_bool().unwrap() {
            candidate = gis_map_snapshot_with_derived_children(candidate);
        }
        assert_eq!(candidate.image, supplied_image, "deriving children must preserve a supplied image");
        let unchanged = candidate.clone();
        let result = GisMapInference::infer(&candidate).create_region_group_work(&candidate, fixture["jobId"].as_str().unwrap());
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{}", row["name"]);
        if !row["accepted"].as_bool().unwrap() {
            assert_eq!(result.unwrap_err(), GisMapProposalError::Composition);
        }
        assert_eq!(candidate, unchanged, "planning must not change the supplied snapshot");
    }
}
//#endregion 🧪️InferenceLaws
