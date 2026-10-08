use super::*;
use crate::standards::v1::subsets::any::schema::default_generation3d_snapshot;
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use protocol::TouchedPaths;

fn camera_diff(oracle: &serde_json::Value, key: &str) -> (Generation3dDiff, semio_framework_artifact_flow_flow::CameraJson) {
    let camera: semio_framework_artifact_flow_flow::CameraJson = semio_framework_pack_json::from_json_str(&oracle[key].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    (Generation3dDiff { camera: Some(camera.clone()), ..Default::default() }, camera)
}

/// ➕️ Absorb keeps the incoming value of a scalar field, composes row deltas, and the absorbed diff applies exactly like its parts.
#[test]
fn diff_absorb_prefers_incoming_camera_and_preserves_other_rows() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧲️absorb/🔣️.json")).unwrap();
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
    let next = Generation3dSnapshotRead::new(protocol::apply_diff(&*sum, &base).expect("absorbed diff applies"));
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
    let after = Generation3dSnapshotRead::new(protocol::apply_diff(&removed, &base).expect("removal applies"));
    assert_eq!(protocol::apply_diff(&*inverse, &after).expect("inverse applies").host_snapshot.widgets.len(), base.host_snapshot.widgets.len());
    removed.retire_cold();
}

//#region 🗺️TouchedRegions
/// 🧬️ Every committed quintet of the mutation vocabulary: its kind, its before-snapshot and its mutation payload.
const QUINTETS: [(&str, &str, &str); 22] = [
    ("create-widget", include_str!("../../../../🧫️fixtures/🧬️mutations/🌱️create-widget/📝️inserts/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🌱️create-widget/📝️inserts/🦠️mutation/🔣️.json")),
    ("update-widget", include_str!("../../../../🧫️fixtures/🧬️mutations/🩹update-widget/🎚️retunes/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🩹update-widget/🎚️retunes/🦠️mutation/🔣️.json")),
    ("delete-widget", include_str!("../../../../🧫️fixtures/🧬️mutations/❌delete-widget/🚫️removes/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/❌delete-widget/🚫️removes/🦠️mutation/🔣️.json")),
    ("connect-synapse", include_str!("../../../../🧫️fixtures/🧬️mutations/🔗️connect-synapse/🔌️wires/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🔗️connect-synapse/🔌️wires/🦠️mutation/🔣️.json")),
    ("update-synapse", include_str!("../../../../🧫️fixtures/🧬️mutations/🔄️update-synapse/📡️repoints/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🔄️update-synapse/📡️repoints/🦠️mutation/🔣️.json")),
    ("disconnect-synapse", include_str!("../../../../🧫️fixtures/🧬️mutations/✂️disconnect-synapse/✂️cuts/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/✂️disconnect-synapse/✂️cuts/🦠️mutation/🔣️.json")),
    ("move-widget", include_str!("../../../../🧫️fixtures/🧬️mutations/📍️move-widget/📍️repositions/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/📍️move-widget/📍️repositions/🦠️mutation/🔣️.json")),
    ("delete-widget-position", include_str!("../../../../🧫️fixtures/🧬️mutations/🧹️delete-widget-position/🧹️unpins/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🧹️delete-widget-position/🧹️unpins/🦠️mutation/🔣️.json")),
    ("update-camera", include_str!("../../../../🧫️fixtures/🧬️mutations/📷️update-camera/🔍️frames/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/📷️update-camera/🔍️frames/🦠️mutation/🔣️.json")),
    ("change-schema", include_str!("../../../../🧫️fixtures/🧬️mutations/🔤️change-schema/🏷️restamps/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🔤️change-schema/🏷️restamps/🦠️mutation/🔣️.json")),
    ("create-generation", include_str!("../../../../🧫️fixtures/🧬️mutations/➕create-generation/🌱️appends/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/➕create-generation/🌱️appends/🦠️mutation/🔣️.json")),
    ("delete-generation", include_str!("../../../../🧫️fixtures/🧬️mutations/🗑️delete-generation/🚫️removes/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🗑️delete-generation/🚫️removes/🦠️mutation/🔣️.json")),
    ("rename-generation", include_str!("../../../../🧫️fixtures/🧬️mutations/🏷️rename-generation/🏷️retitles/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🏷️rename-generation/🏷️retitles/🦠️mutation/🔣️.json")),
    ("change-generation-value", include_str!("../../../../🧫️fixtures/🧬️mutations/🔧️change-generation-value/🏢️raises/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🔧️change-generation-value/🏢️raises/🦠️mutation/🔣️.json")),
    ("change-slider-value", include_str!("../../../../🧫️fixtures/🧬️mutations/🎚️change-slider-value/🎚️sets/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🎚️change-slider-value/🎚️sets/🦠️mutation/🔣️.json")),
    ("drag-transforms", include_str!("../../../../🧫️fixtures/🧬️mutations/✋️drag-transforms/✋️drags/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/✋️drag-transforms/✋️drags/🦠️mutation/🔣️.json")),
    ("rotate-transforms", include_str!("../../../../🧫️fixtures/🧬️mutations/🔃️rotate-transforms/🔃️turns/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🔃️rotate-transforms/🔃️turns/🦠️mutation/🔣️.json")),
    ("scale-transforms", include_str!("../../../../🧫️fixtures/🧬️mutations/📏️scale-transforms/📏️scales/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/📏️scale-transforms/📏️scales/🦠️mutation/🔣️.json")),
    ("move-nodes", include_str!("../../../../🧫️fixtures/🧬️mutations/🚚️move-nodes/🚚️shifts/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🚚️move-nodes/🚚️shifts/🦠️mutation/🔣️.json")),
    ("change-widget-input", include_str!("../../../../🧫️fixtures/🧬️mutations/🎛️change-widget-input/🎛️sets/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/🎛️change-widget-input/🎛️sets/🦠️mutation/🔣️.json")),
    ("select-generation", include_str!("../../../../🧫️fixtures/🧬️mutations/👆️select-generation/👆️picks/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/👆️select-generation/👆️picks/🦠️mutation/🔣️.json")),
    ("change-generation-preview", include_str!("../../../../🧫️fixtures/🧬️mutations/📝️change-generation-preview/📝️retexts/📸️snapshot/⬅️before/🔣️.json"), include_str!("../../../../🧫️fixtures/🧬️mutations/📝️change-generation-preview/📝️retexts/🦠️mutation/🔣️.json"))
];

/// 🔎️ The regions where two snapshots differ, walked on their JSON forms by id — a second route to the answer
/// `host_regions` / `generation_regions` compute on the typed values.
fn changed_regions(before: &serde_json::Value, after: &serde_json::Value) -> std::collections::BTreeSet<String> {
    fn keyed(rows: &serde_json::Value) -> Vec<(String, &serde_json::Value)> {
        rows.as_array().map(|rows| rows.iter().map(|row| (row["id"].as_str().unwrap_or_default().to_string(), row)).collect()).unwrap_or_default()
    }
    fn collection(paths: &mut std::collections::BTreeSet<String>, prefix: &str, before: Vec<(String, &serde_json::Value)>, after: Vec<(String, &serde_json::Value)>) {
        for (id, row) in &before {
            if after.iter().find(|(other, _)| other == id).is_none_or(|(_, next)| next != row) {
                paths.insert(format!("{prefix}/{id}"));
            }
        }
        for (id, _) in &after {
            if !before.iter().any(|(other, _)| other == id) {
                paths.insert(format!("{prefix}/{id}"));
            }
        }
    }
    let mut paths = std::collections::BTreeSet::new();
    let (old, new) = (&before["hostSnapshot"], &after["hostSnapshot"]);
    collection(&mut paths, "hostSnapshot/widgets", keyed(&old["widgets"]), keyed(&new["widgets"]));
    collection(&mut paths, "hostSnapshot/synapses", keyed(&old["synapses"]), keyed(&new["synapses"]));
    fn layout(value: &serde_json::Value) -> Vec<(String, &serde_json::Value)> {
        value["layout"].as_object().map(|map| map.iter().map(|(id, row)| (id.clone(), row)).collect()).unwrap_or_default()
    }
    for (id, row) in layout(old) {
        if layout(new).iter().find(|(other, _)| *other == id).is_none_or(|(_, next)| *next != row) {
            paths.insert(format!("hostSnapshot/layout/{id}"));
        }
    }
    for (id, _) in layout(new) {
        if !layout(old).iter().any(|(other, _)| *other == id) {
            paths.insert(format!("hostSnapshot/layout/{id}"));
        }
    }
    for (member, path) in [("camera", "hostSnapshot/camera"), ("schema", "hostSnapshot/schema")] {
        if old[member] != new[member] {
            paths.insert(path.to_string());
        }
    }
    let (old, new) = (&before["generation"], &after["generation"]);
    collection(&mut paths, "generation", keyed(&old["generations"]), keyed(&new["generations"]));
    for (member, path) in [("selectedGenerationId", "generation/selected"), ("previewText", "generation/previewText")] {
        if old[member] != new[member] {
            paths.insert(path.to_string());
        }
    }
    paths
}

fn decode_snapshot(text: &str) -> Generation3dSnapshotRead {
    Generation3dSnapshotRead::new(semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed snapshot decodes"))
}

fn json(snapshot: &Generation3dSnapshot) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(snapshot)).expect("snapshot encodes")
}

/// ⚖️ Coverage soundness, the law of `protocol::DiffRegions`: over every committed quintet, each region where the applied diff
/// changes the document is covered by a path of `touches()` — and the claim is honest, every path of `touches()` names a
/// region that really changed. A stale cached inference is the price of the first failing; a needless recompute that of the
/// second.
#[test]
fn every_leafs_applied_diff_changes_only_covered_regions_and_covers_every_change() {
    use protocol::DiffRegions;
    assert_eq!(QUINTETS.iter().map(|(kind, _, _)| *kind).collect::<Vec<_>>(), crate::standards::v1::subsets::any::schema::mutations::KINDS, "one quintet per leaf, in enum order");
    for (kind, before, mutation) in QUINTETS {
        let base = decode_snapshot(before);
        let mutation: crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation = semio_framework_pack_json::from_json_str(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation decodes");
        let (diff, _messages) = protocol::Mutation::diff(&mutation, &*base).into_parts();
        let diff = Generation3dDiffRead::new(diff);
        let next = Generation3dSnapshotRead::new(protocol::apply_diff(&diff, &base).expect("the committed quintet's diff applies"));
        let changed = changed_regions(&json(&base), &json(&next));
        let touches = diff.touches();
        assert!(!changed.is_empty(), "{kind}: the committed vector moves the document");
        for region in &changed {
            assert!(touches.intersects_prefix(region), "{kind}: the applied diff changes {region}, which touches() {:?} does not cover", touches.paths);
        }
        for path in &touches.paths {
            assert!(changed.iter().any(|region| TouchedPaths::new([path.clone()]).intersects_prefix(region)), "{kind}: touches() claims {path}, which the applied diff leaves alone (changed: {changed:?})");
        }
        mutation.retire_cold();
    }
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

/// 🧭️ The `topology` inference declares reads that no layout, camera, schema or generation edit reaches, and every leaf that
/// moves the topology reaches them — the tier-1 gate the cache skips on.
#[test]
fn topology_reads_are_sound_for_every_leaf() {
    use protocol::{DiffRegions, Inference, InferenceSpec};
    let reads = crate::standards::v1::subsets::any::schema::inferences::Generation3dInference::fields()[0].reads;
    for (kind, before, mutation) in QUINTETS {
        let base = decode_snapshot(before);
        let mutation: crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation = semio_framework_pack_json::from_json_str(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation decodes");
        let (diff, _messages) = protocol::Mutation::diff(&mutation, &*base).into_parts();
        let diff = Generation3dDiffRead::new(diff);
        let next = Generation3dSnapshotRead::new(protocol::apply_diff(&diff, &base).expect("the committed quintet's diff applies"));
        let moved = crate::standards::v1::subsets::any::schema::inferences::Generation3dInference::infer(&crate::test_serial::geometry_input(&base)).expect("topology infers") != crate::standards::v1::subsets::any::schema::inferences::Generation3dInference::infer(&crate::test_serial::geometry_input(&next)).expect("topology infers");
        if moved {
            assert!(diff.touches().intersects_any(reads), "{kind}: the topology moves but touches() {:?} misses the reads {reads:?}", diff.touches().paths);
        }
        if ["move-widget", "delete-widget-position", "update-camera", "change-schema", "change-slider-value", "drag-transforms", "move-nodes", "select-generation", "change-generation-preview"].contains(&kind) {
            assert!(!moved && !diff.touches().intersects_any(&["hostSnapshot/synapses"]), "{kind}: a layout, camera, schema, value or generation edit leaves the wiring alone");
        }
        mutation.retire_cold();
    }
}
//#endregion 🗺️TouchedRegions
