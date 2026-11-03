use super::*;
use crate::{DragPreview, FieldWrite};

fn dragging(s: f64) -> NormPresence {
    let mut presence = NormPresence::new("ana");
    presence.drag = Some(DragPreview { handle: "h".into(), writes: vec![FieldWrite::number("a", s)], started_seq: 0, origin: 0.0, s, entry: String::new() });
    presence
}

#[test]
fn an_open_drag_is_published_at_most_every_34_ms_and_a_clear_at_once() {
    let mut clock = LaneClock::default();
    assert!(clock.admit_presence(&dragging(1.0), 1000));
    assert!(!clock.admit_presence(&dragging(2.0), 1010), "inside the 30 Hz gate");
    assert!(clock.admit_presence(&dragging(3.0), 1040));
    assert!(!clock.admit_presence(&dragging(3.0), 1100), "unchanged presence is not resent");
    assert!(clock.admit_presence(&NormPresence::new("ana"), 1101), "a cleared drag bypasses the gate");
}

#[test]
fn the_preview_report_is_due_60_ms_after_the_last_tick_and_at_once_on_commit() {
    let mut clock = LaneClock::default();
    clock.tick(1000);
    assert!(!clock.report_due(1030));
    assert!(clock.report_due(1060));
    assert!(!clock.report_due(1100), "a due tier fires once");
    clock.tick(2000);
    clock.commit();
    assert!(clock.report_due(2001));
}

#[test]
fn cached_preview_reports_are_dropped_on_commit() {
    let mut clock = LaneClock::default();
    clock.store_report("fam", Default::default());
    assert!(clock.cached_report("fam").is_some());
    clock.commit();
    assert!(clock.cached_report("fam").is_none());
}

#[test]
fn the_cached_preview_report_is_the_stored_handle_not_a_copy() {
    let mut clock = LaneClock::default();
    let stored = Arc::new(semio_s_plugin_norm_compliance::CheckReport::default());
    clock.store_report("fam", stored.clone());
    let first = clock.cached_report("fam").expect("stored");
    assert!(Arc::ptr_eq(&stored, &first) && Arc::ptr_eq(&first, &clock.cached_report("fam").expect("stored")), "every tick of a drag reads the one stored allocation");
}
