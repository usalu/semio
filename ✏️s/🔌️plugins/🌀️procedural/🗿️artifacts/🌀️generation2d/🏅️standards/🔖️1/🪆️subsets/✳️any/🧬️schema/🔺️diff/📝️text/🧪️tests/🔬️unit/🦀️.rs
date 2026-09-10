use super::*;
use crate::standards::v1::subsets::any::schema::empty_generation2d_snapshot;

#[test]
fn diff_absorb_prefers_incoming_fixture_and_preserves_generation() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧲️absorb/🔣️.json")).unwrap();
    let base = empty_generation2d_snapshot();
    let mut first_fixture = base.fixture.clone();
    first_fixture.camera = pack::from_json_str(&oracle["firstCamera"].to_string()).unwrap();
    let mut incoming = base.fixture.clone();
    incoming.camera = pack::from_json_str(&oracle["incomingCamera"].to_string()).unwrap();
    let mut first = Generation2dDiff { fixture: Some(first_fixture), generation: Some(base.generation.clone()), ..Default::default() };
    first.absorb(Generation2dDiff { fixture: Some(incoming.clone()), ..Default::default() });
    assert_eq!(first.fixture.as_ref(), Some(&incoming));
    assert_eq!(first.generation.as_ref(), Some(&base.generation));
    let mut expected = base.clone();
    expected.fixture = incoming;
    let next = first.apply(&base).expect("absorbed diff applies");
    assert_eq!(next, expected);
    let camera: serde_json::Value = serde_json::from_str(&pack::to_json_string(&next.fixture.camera)).unwrap();
    assert_eq!(camera, oracle["incomingCamera"]);
}

#[test]
fn diff_apply_updates_fixture_widgets() {
    let snapshot = empty_generation2d_snapshot();
    let existing_id = widget_id(&snapshot.fixture.widgets[1]).to_string();
    let diff = diff_fixture_from_helpers(&snapshot, &WidgetsDiff { removed: vec![], set: vec![(0, Widget::InputNote { id: existing_id.clone(), text: "replaced".into() }), (999, Widget::InputNote { id: "brand-new".into(), text: "new".into() })] }, &SynapsesDiff::default(), &LayoutDiff::default(), None, None);
    let next = diff.apply(&snapshot).expect("valid mutation diff");
    assert_eq!(next.fixture.widgets.len(), snapshot.fixture.widgets.len() + 1);
    let replaced = next.fixture.widgets.iter().find(|w| widget_id(w) == existing_id.as_str()).expect("replaced");
    assert_eq!(replaced, &Widget::InputNote { id: existing_id, text: "replaced".into() });
}

/// 🔐️ LAW: the generic replay seams reach this artifact through the `MutationDiff` CONTRACT, never
/// through the inherent helper, so both cold-retirement hooks must be overridden — an inhabited
/// `fixture` owns an `OrderedMap<WidgetLayout>` root whose bare drop aborts the process, which is
/// exactly what `os_vcs::apply_mutation` did on every undone/redone 2d operation.
#[test]
fn the_mutation_diff_contract_retires_an_inhabited_layout_delta_and_its_scratch_projection() {
    let base = empty_generation2d_snapshot();
    let mut fixture = base.fixture.clone();
    fixture.layout.insert("laid-out".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 3.0, y: 4.0 });
    let diff = Generation2dDiff { fixture: Some(fixture), generation: Some(base.generation.clone()), ..Default::default() };
    let scratch = <Generation2dDiff as MutationDiff<Generation2dSnapshot>>::apply(&diff, &base).expect("inhabited layout delta applies");
    assert!(scratch.fixture.layout.contains_key("laid-out"));
    <Generation2dDiff as MutationDiff<Generation2dSnapshot>>::retire_projection(scratch);
    <Generation2dDiff as MutationDiff<Generation2dSnapshot>>::retire_cold(diff);
    base.retire_cold();
}
