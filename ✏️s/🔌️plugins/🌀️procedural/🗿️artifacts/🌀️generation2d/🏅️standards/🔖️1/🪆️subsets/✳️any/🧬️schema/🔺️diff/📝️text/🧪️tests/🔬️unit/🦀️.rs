
use super::*;
use crate::standards::v1::subsets::any::schema::empty_generation2d_snapshot;

#[test]
fn diff_absorb_prefers_incoming_fixture_and_scalars() {
    let base = empty_generation2d_snapshot();
    let mut first = diff_fixture_from_helpers(&base, WidgetsDiff { removed: vec!["w1".into()], set: vec![] }, SynapsesDiff::default(), LayoutDiff::default(), Some(CameraJson { x: 1.0, y: 1.0, zoom: 1.0 }), None);
    let second = Generation2dDiff { show_mode: Some("wire".into()), ..Generation2dDiff::default() };
    first.absorb(second);
    assert!(first.fixture.is_some());
    assert_eq!(first.show_mode.as_deref(), Some("wire"));
    assert_eq!(first.locale.as_deref(), Some("de-DE"));
}

#[test]
fn diff_apply_updates_fixture_widgets() {
    let snapshot = empty_generation2d_snapshot();
    let existing_id = widget_id(&snapshot.fixture.widgets[1]).to_string();
    let diff = diff_fixture_from_helpers(
        &snapshot,
        WidgetsDiff { removed: vec![], set: vec![(0, Widget::InputNote { id: existing_id.clone(), text: "replaced".into() }), (999, Widget::InputNote { id: "brand-new".into(), text: "new".into() })] },
        SynapsesDiff::default(),
        LayoutDiff::default(),
        None,
        None,
    );
    let next = diff.apply(&snapshot).expect("valid mutation diff");
    assert_eq!(next.fixture.widgets.len(), snapshot.fixture.widgets.len() + 1);
    let replaced = next.fixture.widgets.iter().find(|w| widget_id(w) == existing_id.as_str()).expect("replaced");
    assert_eq!(replaced, &Widget::InputNote { id: existing_id, text: "replaced".into() });
}
