use super::*;
use crate::default_generation3d_snapshot;
use crate::snapshot::Generation3dSnapshotRead;
use protocol::TouchedPaths;

fn camera_diff(oracle: &serde_json::Value, key: &str) -> (Generation3dDiff, semio_framework_artifact_flow_flow::CameraJson) {
    let camera: semio_framework_artifact_flow_flow::CameraJson = semio_framework_pack_json::from_json_str(&oracle[key].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    (Generation3dDiff { camera: Some(camera.clone()), ..Default::default() }, camera)
}

/// ➕️ Absorb keeps the incoming value of a scalar field, composes row deltas, and the absorbed diff applies exactly like its parts.
#[test]
fn diff_absorb_prefers_incoming_camera_and_preserves_other_rows() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🔺️diff/🧲️absorb/🔣️.json")).unwrap();
    let base = Generation3dSnapshotRead::new(default_generation3d_snapshot());
    let (first_camera, _) = camera_diff(&oracle, "firstCamera");
    let (incoming_camera, incoming) = camera_diff(&oracle, "incomingCamera");
    let selection = Generation3dDiff { selected_generation: Some(Generation3dSelectionChange { id: None }), ..Default::default() };
    let mut sum = first_camera;
    protocol::MutationDiff::absorb(&mut sum, selection);
    protocol::MutationDiff::absorb(&mut sum, incoming_camera);
    let sum = Generation3dDiffRead::new(sum);
    assert_eq!(sum.camera.as_ref(), Some(&incoming));
    assert_eq!(sum.selected_generation, Some(Generation3dSelectionChange { id: None }));
    let next = Generation3dSnapshotRead::new(protocol::apply_diff_cold(&*sum, &base).expect("absorbed diff applies"));
    assert_eq!(next.host_snapshot.camera, incoming);
    assert_eq!(next.generation.selected_generation_id, None);
    let camera: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&next.host_snapshot.camera)).unwrap();
    assert_eq!(camera, oracle["incomingCamera"]);
}

/// 🔁️ Added then removed rows vanish, removed then added rows become a replace, and the inverse of any single row diff restores the base.
#[test]
fn row_deltas_coalesce_and_invert() {
    let base = Generation3dSnapshotRead::new(default_generation3d_snapshot());
    let widget = base.host_snapshot.widgets[0].clone();
    let id = crate::widget_id(&widget).to_string();
    let removed = Generation3dDiff { widgets: Some(Generation3dWidgetsDelta { removed: vec![Generation3dWidgetRemoval { id: id.clone(), index: 0 }], ..Default::default() }), ..Default::default() };
    let readded = Generation3dDiff { widgets: Some(Generation3dWidgetsDelta { inserted: vec![Generation3dWidgetInsertion { index: 0, row: widget.clone() }], ..Default::default() }), ..Default::default() };
    let mut replaced = removed.clone();
    protocol::MutationDiff::absorb(&mut replaced, readded);
    let replaced = Generation3dDiffRead::new(replaced);
    let delta = replaced.widgets.as_ref().expect("widgets");
    assert_eq!((delta.removed.iter().map(|entry| entry.id.clone()).collect::<Vec<_>>(), delta.inserted.len()), (vec![id.clone()], 1));
    let inverse = Generation3dDiffRead::new(protocol::DiffAlgebra::inverse(&removed, &*base));
    let after = Generation3dSnapshotRead::new(protocol::apply_diff_cold(&removed, &base).expect("removal applies"));
    assert_eq!(protocol::apply_diff_cold(&*inverse, &after).expect("inverse applies").host_snapshot.widgets.len(), base.host_snapshot.widgets.len());
    removed.retire_cold();
}

/// 🗺️ A delta touches exactly the regions its rows name, and an absorbed pair touches the union of both sides.
#[test]
fn rows_name_their_regions_and_absorb_unions_them() {
    use protocol::DiffRegions;
    let widgets = Generation3dDiff { widgets: Some(Generation3dWidgetsDelta { removed: vec![Generation3dWidgetRemoval { id: "w/1".to_string(), index: 0 }], ..Default::default() }), ..Default::default() };
    assert_eq!(widgets.touches().paths, vec!["hostSnapshot/widgets/w~11".to_string()]);
    let mut first = Generation3dDiff { selected_generation: Some(Generation3dSelectionChange { id: None }), ..Default::default() };
    protocol::MutationDiff::absorb(&mut first, widgets);
    assert_eq!(first.touches().paths, vec!["generation/selected".to_string(), "hostSnapshot/widgets/w~11".to_string()]);
    assert!(Generation3dDiff::default().touches().paths.is_empty());
}
