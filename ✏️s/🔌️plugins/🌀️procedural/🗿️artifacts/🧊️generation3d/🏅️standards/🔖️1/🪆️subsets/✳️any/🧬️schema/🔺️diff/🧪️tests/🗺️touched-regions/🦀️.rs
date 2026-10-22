//! 🗺️ Touched-region laws over every committed mutation quintet: the applied diff changes exactly the regions
//! `touches()` names, and the `topology` inference's reads are reached by exactly the leaves that move the topology.

use semio_s_artifact_procedural_3d_model::diff::{Generation3dDiff, Generation3dDiffRead};
use semio_s_artifact_procedural_3d_model::snapshot::{Generation3dSnapshot, Generation3dSnapshotRead};
use protocol::TouchedPaths;

/// 🧫️ One committed fixture of the subset, read at test runtime.
fn fixture(path: &str) -> String {
    std::fs::read_to_string(format!("{}/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/{path}", env!("CARGO_MANIFEST_DIR"))).unwrap_or_else(|error| panic!("fixture {path}: {error}"))
}

//#region 🗺️TouchedRegions
/// 🧬️ Every committed quintet of the mutation vocabulary: its kind, its before-snapshot and its mutation payload.
fn quintets() -> Vec<(&'static str, String, String)> {
    vec![
    ("create-widget", fixture("🧬️mutations/🌱️create-widget/📝️inserts/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🌱️create-widget/📝️inserts/🦠️mutation/🔣️.json")),
    ("update-widget", fixture("🧬️mutations/🩹update-widget/🎚️retunes/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🩹update-widget/🎚️retunes/🦠️mutation/🔣️.json")),
    ("delete-widget", fixture("🧬️mutations/❌delete-widget/🚫️removes/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/❌delete-widget/🚫️removes/🦠️mutation/🔣️.json")),
    ("connect-synapse", fixture("🧬️mutations/🔗️connect-synapse/🔌️wires/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🔗️connect-synapse/🔌️wires/🦠️mutation/🔣️.json")),
    ("update-synapse", fixture("🧬️mutations/🔄️update-synapse/📡️repoints/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🔄️update-synapse/📡️repoints/🦠️mutation/🔣️.json")),
    ("disconnect-synapse", fixture("🧬️mutations/✂️disconnect-synapse/✂️cuts/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/✂️disconnect-synapse/✂️cuts/🦠️mutation/🔣️.json")),
    ("move-widget", fixture("🧬️mutations/📍️move-widget/📍️repositions/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/📍️move-widget/📍️repositions/🦠️mutation/🔣️.json")),
    ("delete-widget-position", fixture("🧬️mutations/🧹️delete-widget-position/🧹️unpins/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🧹️delete-widget-position/🧹️unpins/🦠️mutation/🔣️.json")),
    ("update-camera", fixture("🧬️mutations/📷️update-camera/🔍️frames/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/📷️update-camera/🔍️frames/🦠️mutation/🔣️.json")),
    ("change-schema", fixture("🧬️mutations/🔤️change-schema/🏷️restamps/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🔤️change-schema/🏷️restamps/🦠️mutation/🔣️.json")),
    ("create-generation", fixture("🧬️mutations/➕create-generation/🌱️appends/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/➕create-generation/🌱️appends/🦠️mutation/🔣️.json")),
    ("delete-generation", fixture("🧬️mutations/🗑️delete-generation/🚫️removes/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🗑️delete-generation/🚫️removes/🦠️mutation/🔣️.json")),
    ("rename-generation", fixture("🧬️mutations/🏷️rename-generation/🏷️retitles/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🏷️rename-generation/🏷️retitles/🦠️mutation/🔣️.json")),
    ("change-generation-value", fixture("🧬️mutations/🔧️change-generation-value/🏢️raises/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🔧️change-generation-value/🏢️raises/🦠️mutation/🔣️.json")),
    ("change-slider-value", fixture("🧬️mutations/🎚️change-slider-value/🎚️sets/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🎚️change-slider-value/🎚️sets/🦠️mutation/🔣️.json")),
    ("drag-transforms", fixture("🧬️mutations/✋️drag-transforms/✋️drags/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/✋️drag-transforms/✋️drags/🦠️mutation/🔣️.json")),
    ("rotate-transforms", fixture("🧬️mutations/🔃️rotate-transforms/🔃️turns/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🔃️rotate-transforms/🔃️turns/🦠️mutation/🔣️.json")),
    ("scale-transforms", fixture("🧬️mutations/📏️scale-transforms/📏️scales/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/📏️scale-transforms/📏️scales/🦠️mutation/🔣️.json")),
    ("move-nodes", fixture("🧬️mutations/🚚️move-nodes/🚚️shifts/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🚚️move-nodes/🚚️shifts/🦠️mutation/🔣️.json")),
    ("change-widget-input", fixture("🧬️mutations/🎛️change-widget-input/🎛️sets/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/🎛️change-widget-input/🎛️sets/🦠️mutation/🔣️.json")),
    ("select-generation", fixture("🧬️mutations/👆️select-generation/👆️picks/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/👆️select-generation/👆️picks/🦠️mutation/🔣️.json")),
    ("change-generation-preview", fixture("🧬️mutations/📝️change-generation-preview/📝️retexts/📸️snapshot/⬅️before/🔣️.json"), fixture("🧬️mutations/📝️change-generation-preview/📝️retexts/🦠️mutation/🔣️.json"))
    ]
}

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
    assert_eq!(quintets().iter().map(|(kind, _, _)| *kind).collect::<Vec<_>>(), semio_s_artifact_procedural_3d_mutations::KINDS, "one quintet per leaf, in enum order");
    for (kind, before, mutation) in quintets() {
        let base = decode_snapshot(&before);
        let mutation: semio_s_artifact_procedural_3d_mutations::Generation3dMutation = semio_framework_pack_json::from_json_str(&mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation decodes");
        let (diff, _messages) = protocol::Mutation::diff(&mutation, &*base).into_parts();
        let diff = Generation3dDiffRead::new(diff);
        let next = Generation3dSnapshotRead::new(protocol::apply_diff_cold(&diff, &base).expect("the committed quintet's diff applies"));
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

/// 🧭️ The `topology` inference declares reads that no layout, camera, schema or generation edit reaches, and every leaf that
/// moves the topology reaches them — the tier-1 gate the cache skips on.
#[test]
fn topology_reads_are_sound_for_every_leaf() {
    use protocol::{DiffRegions, Inference, InferenceSpec};
    let reads = semio_s_artifact_procedural_3d_inferences::Generation3dInference::fields()[0].reads;
    for (kind, before, mutation) in quintets() {
        let base = decode_snapshot(&before);
        let mutation: semio_s_artifact_procedural_3d_mutations::Generation3dMutation = semio_framework_pack_json::from_json_str(&mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation decodes");
        let (diff, _messages) = protocol::Mutation::diff(&mutation, &*base).into_parts();
        let diff = Generation3dDiffRead::new(diff);
        let next = Generation3dSnapshotRead::new(protocol::apply_diff_cold(&diff, &base).expect("the committed quintet's diff applies"));
        let moved = semio_s_artifact_procedural_3d_inferences::Generation3dInference::infer(&crate::test_geometry::geometry_input(&base)).expect("topology infers") != semio_s_artifact_procedural_3d_inferences::Generation3dInference::infer(&crate::test_geometry::geometry_input(&next)).expect("topology infers");
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
