//! 🧪️ View-step tests: every stepping shortcut as key, event and resulting view state. The independent oracles are hand-derived orders of the example report (list order worst entity first, governing order by utilisation) and `std` sorting.

use super::*;
use crate::accessibility::focus_order::{Chord, Scope};
use crate::{dispatch_shortcut, view_event, KeyCommand, NormTransient};
use semio_s_plugin_norm_compliance::{AnnexChoice, LocalizedCopy, SubjectRef};
use semio_s_plugin_norm_structural_scene::projection_kit::tracks::indexed;
use semio_s_plugin_norm_structural_scene::{Aabb, LocalizedCopy as Text, SceneRevision, Vec3, SCENE_SCHEMA_ID};

fn check(id: &str, entity: &str, utilization: f64, status: CheckStatus) -> CheckResult {
    CheckResult::assess(id, "ULS", ClauseId::new("EN 1993", "1-1", "6.2.9"), SubjectRef::new(entity, format!("members[{entity}].tM"), LocalizedCopy::new(format!("Member {entity}"), format!("Stab {entity}"))), LocalizedCopy::new(format!("check {id}"), format!("Nachweis {id}")))
        .utilization(Quantity::new(QuantityKind::Length, utilization * 0.0125), Quantity::new(QuantityKind::Length, 0.0125))
        .explanation(LocalizedCopy::new("explained", "erläutert"))
        .annex(AnnexChoice::De)
        .status(status)
        .build()
}

fn report() -> CheckReport {
    let mut report = CheckReport::default();
    for (id, entity, utilization, status) in [("a", "B1", 0.64, CheckStatus::Pass), ("b", "B2", 1.12, CheckStatus::Fail), ("c", "B1", 0.95, CheckStatus::Warning), ("d", "B3", 0.0, CheckStatus::NotApplicable), ("e", "B2", 0.3, CheckStatus::Pass)] {
        report.push(check(id, entity, utilization, status));
    }
    report
}

fn scene(combinations: usize) -> StructuralScene {
    let stops = (1..=combinations).map(|n| (format!("Combination {n}"), format!("Kombination {n}"), None, None)).collect();
    StructuralScene {
        schema: SCENE_SCHEMA_ID.into(),
        revision: SceneRevision { document_seq: 1, report_seq: 1 },
        bounds: Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
        entities: Vec::new(),
        annotations: Vec::new(),
        legends: Vec::new(),
        layers: Vec::new(),
        tracks: indexed("combination", TrackKind::Combination, ("Combination", "Kombination"), stops).into_iter().collect(),
        owners: Vec::new(),
        description: Text::new("scene", "Szene"),
    }
}

fn press(scope: Scope, key: &'static str, alt: bool, shift: bool, view: ViewLane, combinations: usize) -> ViewLane {
    let Some(KeyCommand::Host(action)) = dispatch_shortcut(scope, Chord { key, ctrl: false, alt, shift }) else { panic!("{key} is a host shortcut in {scope:?}") };
    let (act, direction) = shortcut_view_act(&shortcut_action_id(&action)).expect("a stepping shortcut");
    let event = view_event(act, None, Some(direction)).expect("a step intent");
    let events = resolve_view_steps(vec![event], &report(), || scene(combinations), &view);
    events.into_iter().fold(NormTransient::default().with_view(view), NormTransient::apply).view
}

fn at(check: u32) -> ViewLane {
    ViewLane { check: Some(check), ..ViewLane::default() }
}

#[test]
fn only_the_six_stepping_shortcuts_become_view_actions() {
    let stepping: Vec<_> = crate::accessibility::focus_order::shortcuts().into_iter().filter(|s| shortcut_view_act(&shortcut_action_id(&s.action)).is_some()).collect();
    assert_eq!(stepping.len(), 6);
    assert_eq!(shortcut_view_act("norm.shortcut.next-check"), Some((VIEW_ACTION_STEP_CHECK, 1.0)));
    assert_eq!(shortcut_view_act("norm.shortcut.previous-governing-check"), Some((VIEW_ACTION_STEP_GOVERNING, -1.0)));
    assert_eq!(shortcut_view_act("norm.shortcut.undo"), None);
}

#[test]
fn next_and_previous_check_walk_the_default_list_order_and_wrap() {
    let order = [1, 4, 2, 0, 3];
    assert_eq!(list_order(&report()), order);
    let mut view = ViewLane::default();
    for expected in order.iter().chain(&order[..1]) {
        view = press(Scope::Global, "]", false, false, view, 0);
        assert_eq!(view.check, Some(*expected as u32));
    }
    assert_eq!(press(Scope::Global, "[", false, false, at(1), 0).check, Some(3));
    assert_eq!(press(Scope::Global, "[", false, false, ViewLane::default(), 0).check, Some(3));
}

#[test]
fn governing_check_stepping_walks_from_the_highest_utilisation_down() {
    let mut order = Vec::new();
    let mut view = ViewLane::default();
    for _ in 0..5 {
        view = press(Scope::Global, "g", true, false, view, 0);
        order.push(view.check.expect("a check"));
    }
    assert_eq!(order, [1, 2, 0, 4, 3], "1.12 fail, 0.95, 0.64, 0.30, not applicable last");
    assert_eq!(press(Scope::Global, "g", true, false, at(3), 0).check, Some(1));
    assert_eq!(press(Scope::Global, "g", true, true, at(1), 0).check, Some(3));
    assert_eq!(press(Scope::Global, "g", true, true, at(0), 0).check, Some(2));
}

#[test]
fn combination_stepping_selects_track_value_and_index_and_wraps() {
    let mut view = ViewLane::default();
    for (expected_item, expected_value) in [(0, 1.0), (1, 2.0), (2, 3.0), (0, 1.0)] {
        view = press(Scope::Viewport, ".", false, false, view, 3);
        assert_eq!((view.item, view.value, view.track.as_deref()), (Some(expected_item), Some(expected_value), Some("combination")));
    }
    assert_eq!(press(Scope::Viewport, ",", false, false, ViewLane::default(), 3).item, Some(2));
    assert_eq!(press(Scope::Viewport, ",", false, false, view, 3).item, Some(2));
    let none = press(Scope::Viewport, ".", false, false, ViewLane::default(), 0);
    assert_eq!(none, ViewLane::default(), "a scene without a combination track has nothing to step through");
}

#[test]
fn a_step_never_touches_the_other_view_state() {
    let before = ViewLane { active_field: Some("stress".into()), units: Some("si".into()), speed: 2.0, ..ViewLane::default() };
    let after = press(Scope::Global, "]", false, false, before.clone(), 0);
    assert_eq!(ViewLane { check: None, ..after }, before);
}

#[test]
fn a_viewer_steps_checks_and_combinations_without_any_edit() {
    let order = list_order(&report());
    let mut view = ViewLane::default();
    let mut walked = Vec::new();
    for _ in 0..order.len() {
        view = step_view(&view, ViewStep::Check, true, &report(), || scene(3));
        walked.push(view.check.expect("a check") as usize);
    }
    assert_eq!(walked, order);
    let after_wrap = step_view(&view, ViewStep::Check, true, &report(), || scene(3));
    assert_eq!(after_wrap.check.map(|c| c as usize), Some(order[0]));
    let first = step_view(&ViewLane::default(), ViewStep::Combination, true, &report(), || scene(3));
    let second = step_view(&first, ViewStep::Combination, true, &report(), || scene(3));
    let back = step_view(&second, ViewStep::Combination, false, &report(), || scene(3));
    assert_eq!((first.item, second.item, back.item), (Some(0), Some(1), Some(0)));
    assert_eq!(first.track.as_deref(), Some("combination"));
    assert_eq!(step_view(&first, ViewStep::Combination, true, &report(), || scene(1)), first, "a scene without a combination track leaves the lane unchanged");
}
