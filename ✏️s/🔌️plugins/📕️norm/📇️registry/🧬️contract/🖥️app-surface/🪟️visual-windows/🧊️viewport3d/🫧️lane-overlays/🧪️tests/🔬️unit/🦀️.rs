use super::*;
use crate::accessibility::A11yPrefs;
use semio_plugin::World3dMarkerShape;
use crate::{DragPreview, FieldWrite, NormTransient, PeerDrag};
use semio_plugin::World3dAnnotation;
use semio_s_plugin_norm_structural_scene::{Affine, EntityRef, FieldBinding, GripStyle, Interval, LocalizedCopy, SnapRule};

fn handle() -> Handle {
    Handle {
        id: "b1/length/end".into(),
        entity: EntityRef::new(EntityKind::Member, "b1", "members[id=b1]"),
        kind: HandleKind::ScalarAlong { axis: Vec3::new(1.0, 0.0, 0.0) },
        anchor: Vec3::new(4.0, 0.0, 0.0),
        grip: GripStyle::Arrow,
        writes: vec![FieldBinding::new("members[id=b1].length", Affine::IDENTITY, "m")],
        snap: SnapRule { grid: None, increment: Some(0.05), targets: vec![], catalogue: vec![], tolerance: 0.0 },
        limits: Some(Interval::new(0.5, 12.0)),
        accessible_name: LocalizedCopy::new("Length", "Länge"),
    }
}

fn dragging(entry: &str) -> LaneFrame {
    let drag = DragPreview { handle: "b1/length/end".into(), writes: vec![FieldWrite::number("members[id=b1].length", 5.5)], started_seq: 0, origin: 4.0, s: 5.5, entry: entry.into() };
    LaneFrame { transient: NormTransient::default().with_drag(Some(drag)), peers: Vec::new() }
}

fn text_of(annotation: &World3dAnnotation) -> (String, String) {
    match annotation {
        World3dAnnotation::Dimension(d) => (d.text.en.clone(), d.text.de.clone()),
        World3dAnnotation::Marker(m) => (m.text.en.clone(), m.text.de.clone()),
        _ => panic!("unexpected annotation"),
    }
}

#[test]
fn the_open_drag_gets_an_inline_dimension_line_with_the_previewed_value() {
    let line = dimension_entry(&[handle()], &dragging("")).expect("a dimension line");
    match &line {
        World3dAnnotation::Dimension(d) => {
            assert_eq!(d.id, "entry:b1/length/end");
            assert_eq!(d.from, [4.0, 0.0, 0.0]);
            assert!((d.to[0] - 5.5).abs() < 1e-12);
        }
        other => panic!("expected a dimension, got {other:?}"),
    }
    assert_eq!(text_of(&line).0, "5.5 m");
    assert_eq!(text_of(&dimension_entry(&[handle()], &dragging("")).expect("de")).1, "5,5 m");
}

#[test]
fn typed_entry_shows_the_text_and_a_caret() {
    let line = dimension_entry(&[handle()], &dragging("7,")).expect("a dimension line");
    assert_eq!(text_of(&line).0, "7,\u{258F} m");
}

#[test]
fn no_open_drag_means_no_dimension_line() {
    assert!(dimension_entry(&[handle()], &LaneFrame::default()).is_none());
}

#[test]
fn grips_mark_every_handle_and_tone_the_dragged_one() {
    let marks = grips(&[handle()], &GripView { dragging: Some("b1/length/end"), ..GripView::default() });
    assert_eq!(marks.len(), 1);
    assert!(matches!(&marks[0], World3dAnnotation::Marker(m) if m.id == "handle:b1/length/end" && m.tone == World3dTone::Primary));
    assert!(matches!(&grips(&[handle()], &GripView::default())[0], World3dAnnotation::Marker(m) if m.tone == World3dTone::Info));
}

#[test]
fn the_focused_grip_gets_a_cross_so_focus_is_not_told_by_colour_alone_and_the_preferences_are_honoured() {
    let focused = GripView { focused: Some("b1/length/end"), ..GripView::default() };
    let marks = grips(&[handle()], &focused);
    assert_eq!(marks.len(), 2);
    assert!(matches!(&marks[1], World3dAnnotation::Marker(m) if m.id == "focus:b1/length/end" && m.shape == World3dMarkerShape::Cross && m.position == [4.0, 0.0, 0.0]));
    assert_eq!(text_of(&marks[1]), ("Focused: Length".to_string(), "Fokussiert: Länge".to_string()));
    let calm = A11yPrefs::default();
    for contrast in [A11yPrefs { forced_colors: true, ..calm }, A11yPrefs { prefers_contrast_more: true, ..calm }] {
        let marks = grips(&[handle()], &GripView { prefs: contrast, ..GripView::default() });
        assert!(matches!(&marks[0], World3dAnnotation::Marker(m) if m.tone == World3dTone::Primary), "more contrast draws every grip in the strongest tone");
    }
    assert_eq!(format!("{:?}", grips(&[handle()], &focused)), format!("{:?}", grips(&[handle()], &GripView { prefs: A11yPrefs { prefers_reduced_motion: true, ..calm }, ..focused })), "grips never animate, so reduced motion changes nothing");
}

#[test]
fn the_insert_ghost_follows_the_cursor_in_both_languages() {
    let ghost = insert_ghost(EntityKind::Member, &Vec3::new(1.0, 2.0, 3.0));
    assert!(matches!(&ghost, World3dAnnotation::Marker(m) if m.position == [1.0, 2.0, 3.0] && m.id == "insert:ghost"));
    assert_eq!(text_of(&ghost), ("Insert member".to_string(), "Bauteil einfügen".to_string()));
}

#[test]
fn the_drags_of_other_users_are_drawn_with_their_name() {
    let drag = DragPreview { handle: "b1/length/end".into(), writes: Vec::new(), started_seq: 0, origin: 4.0, s: 6.0, entry: String::new() };
    let frame = LaneFrame { transient: NormTransient::default().with_peers(vec![PeerDrag { user: "ana".into(), drag }]), peers: Vec::new() };
    let lines = peer_drags(&[handle()], &frame);
    assert_eq!(lines.len(), 1);
    assert_eq!(text_of(&lines[0]).0, "ana: 6 m");
    assert_eq!(text_of(&lines[0]).1, "ana: 6 m");
    assert!(peer_drags(&[], &frame).is_empty(), "a grip that is not known is not drawn");
}

#[test]
fn the_overlays_of_a_paint_carry_the_keyboard_focus_and_the_platform_preferences_the_shell_reported() {
    let domain = "norm.test-lane-overlays";
    crate::set_focused_grip(domain, Some("b1/length/end".into()));
    let lanes = LaneOverlays::of_frame(&LaneFrame::default(), domain);
    assert_eq!(lanes.focused.as_deref(), Some("b1/length/end"));
    assert_eq!(LaneOverlays::of_frame(&LaneFrame::default(), "norm.test-lane-overlays-other").focused, None, "the focus is per artifact");
    let marks = grips(&[handle()], &GripView { focused: lanes.focused.as_deref(), prefs: A11yPrefs { forced_colors: true, ..A11yPrefs::default() }, ..GripView::default() });
    assert!(marks.iter().any(|m| matches!(m, World3dAnnotation::Marker(m) if m.id == "focus:b1/length/end")), "the focused grip is marked in the painted overlay");
    crate::set_focused_grip(domain, None);
    assert_eq!(LaneOverlays::of_frame(&LaneFrame::default(), domain).focused, None);
}
