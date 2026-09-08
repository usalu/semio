
use super::*;

fn owner_handle(text: &str) -> FlowContentChild {
    let target = store::os_io::ArtifactRef { artifact_id: "flow-content-reused".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() } };
    let scene = FlowWorkingScene { widgets: vec![Widget::InputNote { id: "note".into(), text: text.into() }], synapses: Vec::new(), layout: flow::OrderedMap::new() };
    FlowContentChild::new("flow-content-reused".into(), target).with_local_owner(Arc::new(scene))
}

fn owner_text(handle: &FlowContentChild) -> String {
    let owner = handle.local_owner::<FlowWorkingScene>().expect("typed Flow owner");
    let [Widget::InputNote { text, .. }] = owner.widgets.as_slice() else { panic!("one note fixture") };
    text.clone()
}

#[test]
fn flow_scene_owner_fixture_is_language_neutral_and_bounded() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/⚖️flow-scene-owner-law.json")).expect("language-neutral Flow owner fixture");
    assert_eq!(fixture["ownedSlots"], 1);
    assert_eq!(fixture["maximumCases"], 5);
    assert_eq!(fixture["cases"].as_array().map(Vec::len), Some(5));
}

#[test]
fn flow_scene_owner_holds_identity_isolation_aba_wire_omission_and_close() {
    let left = owner_handle("A");
    let clone = left.clone();
    let left_owner = left.local_owner::<FlowWorkingScene>().expect("left owner");
    let clone_owner = clone.local_owner::<FlowWorkingScene>().expect("clone owner");
    assert!(Arc::ptr_eq(&left_owner, &clone_owner));

    let right = owner_handle("B");
    assert_eq!(left.child_id, right.child_id, "hostile durable identity must collide");
    assert_eq!(owner_text(&left), "A");
    assert_eq!(owner_text(&right), "B");
    let stale_a = left.clone();
    drop(left);
    let reused_b = owner_handle("B");
    assert_eq!(owner_text(&stale_a), "A", "stale A must not resolve reused B");
    assert_eq!(owner_text(&reused_b), "B");

    let wire = serde_json::Value::from(dsl::ToValue::to_value(&stale_a));
    assert_eq!(wire.as_object().map(serde_json::Map::len), Some(2));
    assert!(wire.get("localOwner").is_none());
    let encoded = serde_json::to_vec(&wire).expect("independent JSON encoding");
    let decoded: FlowContentChild = flow::os_pack::json::from_json_str(std::str::from_utf8(&encoded).unwrap()).expect("child wire decode");
    assert!(decoded.local_owner::<FlowWorkingScene>().is_none());

    drop(left_owner);
    drop(clone_owner);
    drop(clone);
    drop(right);
    drop(stale_a);
    drop(reused_b);
    let terminal = owner_handle("close");
    let owner = terminal.local_owner::<FlowWorkingScene>().expect("close owner");
    let witness = Arc::downgrade(&owner);
    drop(owner);
    assert!(witness.upgrade().is_some());
    drop(terminal);
    assert!(witness.upgrade().is_none(), "one exact child slot must close its scene owner");
}

/// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("flow.artifact") is deliberately NOT
/// `FLOW_DOCUMENT_SCHEMA` ("flow.fixture") — the former names the artifact kind in the OS media
/// catalogue, the latter keys the store envelope. Pinned so a future edit can't silently merge them.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
    assert_eq!(artifact_kind().schema, "flow.artifact");
    assert_eq!(FLOW_DOCUMENT_SCHEMA, "flow.fixture");
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_widgets() {
    assert!(!FlowSnapshot::default().to_fixture().widgets.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn widget_content_round_trips_through_the_composed_child_snapshot() {
    let fixture = semio_framework_artifact_flow_flow::FlowFixture::default();
    let content = flow_content_snapshot_from_working(&fixture.widgets, &fixture.synapses, &fixture.layout);
    let (widgets, synapses, layout) = working_from_flow_content_snapshot(&content);
    assert_eq!(widgets, fixture.widgets);
    assert_eq!(synapses, fixture.synapses);
    for (id, entry) in &fixture.layout {
        assert_eq!(layout.get(id), Some(entry));
    }
}

#[test]
fn authored_slider_labels_survive_child_content_round_trip() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🏷️slider-labels.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let widget: Widget = dsl::FromValue::from_value(dsl::DslValue::from(row["widget"].clone())).unwrap();
        let content = flow_content_snapshot_from_working(&[widget.clone()], &[], &flow::OrderedMap::new());
        assert_eq!(content.nodes[0].label, row["expectedDagName"].as_str().unwrap());
        assert_eq!(working_from_flow_content_snapshot(&content).0, [widget]);
    }
}
