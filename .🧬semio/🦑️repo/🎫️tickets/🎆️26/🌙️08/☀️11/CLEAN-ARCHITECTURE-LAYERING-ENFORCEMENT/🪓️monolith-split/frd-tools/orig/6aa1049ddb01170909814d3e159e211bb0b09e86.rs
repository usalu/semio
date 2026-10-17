use crate::GisMapSnapshot;
use semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation;
use semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation;
#[allow(unused_imports)]
use crate::{standards::v1::subsets::any::schema::inferences::bounds::all_lon_lat_pairs, standards::v1::subsets::any::schema::inferences::bounds::lon_lat_bounds};
#[allow(unused_imports)]
use crate::{standards::v1::subsets::any::map_composition::GisMapDrawingChild, standards::v1::subsets::any::map_composition::GisMapValueChild};
#[allow(unused_imports)]
use ::semio_framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_drawing::schema::diff::NodePath;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_drawing::{schema::mutations::create_node, schema::mutations::inverse_semio_drawing_mutation};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::DrawNode;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_value_mutations::schema::mutations::inverse_semio_value_mutation;
use crate::standards::v1::subsets::any::schema::inferences::*;
use crate::{MapFeature};
use crate::standards::v1::subsets::any::map_composition::{gis_map_snapshot_with_derived_children};
use protocol::Inference;

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = GisMapSnapshot { positions: vec![MapFeature { id: "p1".into(), data: semio_framework_value::DslValue::from(serde_json::json!({ "lon": 1.0, "lat": 2.0 })) }], routes: Vec::new(), regions: Vec::new(), ..Default::default() };
    assert_eq!(GisMapInference::infer(&snapshot).expect("valid materialized inference fixture"), GisMapInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(GisMapInference::infer(&GisMapSnapshot::default()).expect("valid materialized inference fixture"), GisMapInference::default());
}

#[semio_framework_async_macros::async_test]
async fn map_create_region_group_work_stabilizes_parent_drawing_value_without_image() {
    use crate::standards::v1::subsets::any::io::text::mutations::apply_gis_map_mutation;
    use crate::schema::drawing_bridge::gis_map_snapshot_to_drawing;
use crate::standards::v1::subsets::any::io::text::snapshot::gis_map_descriptor_json;
    use {semio_s_artifact_stdio_semio_drawing::schema::mutations::{diff_semio_drawing_mutation, SemioDrawingMutation as AppliedDrawingMutation}, semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot as AppliedDrawingSnapshot};
    use {semio_s_artifact_stdio_semio_value_mutations::schema::mutations::{diff_semio_value_mutation, SemioValueMutation as AppliedValueMutation}, semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot as AppliedValueSnapshot};
        let apply_semio_drawing_mutation = |content: &mut AppliedDrawingSnapshot, mutation: &AppliedDrawingMutation| {
            let outcome = diff_semio_drawing_mutation(mutation, content);
            *content = semio_framework_os_kernel::apply_diff(outcome.diff(), content).expect("a drawing diff applies to the snapshot it was computed from");
        };
        let apply_semio_value_mutation = |content: &mut AppliedValueSnapshot, mutation: &AppliedValueMutation| {
            let outcome = diff_semio_value_mutation(mutation, content);
            *content = semio_framework_os_kernel::apply_diff(outcome.diff(), content).expect("a value diff applies to the snapshot it was computed from");
        };

    let feature = |id: &str, data: serde_json::Value| MapFeature { id: id.into(), data: semio_framework_value::DslValue::from(data) };
    let snapshot = gis_map_snapshot_with_derived_children(GisMapSnapshot {
        positions: vec![feature("point-a", serde_json::json!({ "id": "point-a", "lon": 7, "lat": 47 }))],
        routes: vec![feature("route-a", serde_json::json!({ "id": "route-a", "points": [[8, 46], [9, 48]] }))],
        ..Default::default()
    });
    let inferred = GisMapInference::infer(&snapshot).expect("valid materialized inference fixture");
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
    for inverse in work.drawing_inverse.iter().rev() {
        apply_semio_drawing_mutation(&mut projected_drawing, inverse);
    }
    assert_eq!(projected_drawing, before_drawing);

    let before_value = crate::standards::v1::subsets::any::map_composition::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(&snapshot));
    let after_value = crate::standards::v1::subsets::any::map_composition::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(&parent_after));
    let mut projected_value = before_value.clone();
    apply_semio_value_mutation(&mut projected_value, &work.value);
    assert_eq!(projected_value, after_value);
    for inverse in work.value_inverse.iter().rev() {
        apply_semio_value_mutation(&mut projected_value, inverse);
    }
    assert_eq!(projected_value, before_value);

    for inverse in &work.parent_inverse {
        apply_gis_map_mutation(&mut parent_after, inverse).expect("parent inverse applies");
    }
    assert_eq!(parent_after, snapshot);
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../../../../../../../🧪️tests/🧫️fixtures/🧩️map-create-region-group/🔣️.json")).expect("neutral Map membership corpus");
    for row in fixture["membershipCases"].as_array().expect("membership cases") {
        let mut candidate = snapshot.clone();
        candidate.drawing.child_id = row["drawingChildId"].as_str().unwrap().into();
        candidate.value.child_id = row["valueChildId"].as_str().unwrap().into();
        candidate.image = row["imageChildId"].as_str().map(|id| {
            store::os_store::artifact_child::ArtifactChild::new(id.to_owned(), semio_framework_artifact_reference::ArtifactRef { artifact_id: id.to_owned(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } })
        });
        let supplied_image = candidate.image.clone();
        if row["deriveChildren"].as_bool().unwrap() {
            candidate = gis_map_snapshot_with_derived_children(candidate);
        }
        assert_eq!(candidate.image, supplied_image, "deriving children must preserve a supplied image");
        let unchanged = candidate.clone();
        let result = GisMapInference::infer(&candidate).expect("valid materialized inference fixture").create_region_group_work(&candidate, fixture["jobId"].as_str().unwrap());
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{}", row["name"]);
        if !row["accepted"].as_bool().unwrap() {
            assert_eq!(result.unwrap_err(), GisMapProposalError::Composition);
        }
        assert_eq!(candidate, unchanged, "planning must not change the supplied snapshot");
    }
}
//#endregion 🧪️InferenceLaws

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
