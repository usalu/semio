use crate::*;
use semio_s_plugin_norm_compliance::{CheckResult, LocalizedCopy, SubjectRef};
use semio_s_plugin_norm_structural_scene::EntityKind;

fn check(id: &str, entity: &str, path: &str, computed: f64) -> CheckResult {
    CheckResult::assess(id, "p", ClauseId::new("EN", "1", "1"), SubjectRef::new(entity, path, LocalizedCopy::new(entity, entity)), LocalizedCopy::new(id, id))
        .utilization(Quantity::new(QuantityKind::Dimensionless, computed), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .explanation(LocalizedCopy::new("x", "x"))
        .build()
}

fn report() -> CheckReport {
    let mut report = CheckReport::default();
    report.push(check("b1.low", "b1", "members[id=b1].length", 0.4));
    report.push(check("b1.high", "b1", "members[id=b1].sectionId", 0.9));
    report.push(check("c2.u", "c2", "members[id=c2].length", 1.3));
    report
}

fn address(kind: &str, id: &str) -> String {
    crate::protocol::wire::encode_scoped_identity(&[kind, id]).expect("address")
}

#[test]
fn a_stored_check_selects_its_entity_and_field_path() {
    let selection = NormSelection::resolve(&report(), Some(2), &[]);
    assert_eq!(selection, NormSelection { entities: vec!["c2".into()], picked: Vec::new(), check: Some(2), paths: vec!["members[id=c2].length".into()], units: None });
    assert_eq!(selection.primary(&report()).as_deref(), Some("c2"));
    assert_eq!(selection.checks_selected(), Some(2));
    assert_eq!(selection.outliner_state().selected, ["c2", "members[id=c2].length"]);
}

#[test]
fn a_picked_entity_selects_its_governing_check() {
    let picked = [address("member", "b1"), "b1#face:2".to_string()];
    let selection = NormSelection::resolve(&report(), None, &picked);
    assert_eq!(selection.entities, ["b1"]);
    assert_eq!(selection.check, Some(1));
    assert_eq!(selection.paths, ["members[id=b1].sectionId"]);
}

#[test]
fn a_picked_check_row_selects_the_check_and_unions_with_other_picks() {
    let picked = [address("member", "c2"), address("check", "b1.low")];
    let selection = NormSelection::resolve(&report(), None, &picked);
    assert_eq!(selection.check, Some(0));
    assert_eq!(selection.entities, ["c2", "b1"]);
    assert_eq!(selection.primary(&report()).as_deref(), Some("b1"));
    assert!(selection.contains("c2") && !selection.contains("zz"));
}

#[test]
fn nothing_selected_or_a_stale_index_selects_nothing() {
    assert_eq!(NormSelection::resolve(&report(), None, &[]), NormSelection::default());
    assert_eq!(NormSelection::resolve(&report(), Some(9), &[]), NormSelection::default());
    assert_eq!(NormSelection::resolve(&CheckReport::default(), Some(0), &[]).primary(&CheckReport::default()), None);
}

#[test]
fn highlights_cover_the_entity_and_its_parts() {
    let selection = NormSelection::resolve(&report(), Some(0), &[]);
    let scene = [(EntityKind::Member, "b1"), (EntityKind::Fastener, "b1:bolt-1"), (EntityKind::Member, "c2")];
    let addresses = selection.highlights(scene);
    assert_eq!(addresses, vec![address("member", "b1"), address("fastener", "b1:bolt-1")]);
}

#[test]
fn highlights_match_the_entity_and_its_colon_parts_only() {
    let selection = NormSelection::resolve(&report(), Some(0), &[]);
    let scene = [(EntityKind::Member, "b10"), (EntityKind::Member, "b1"), (EntityKind::Fastener, "b1:bolt-1"), (EntityKind::Member, "b1-bolt"), (EntityKind::Member, "xb1:bolt"), (EntityKind::Member, "b")];
    assert_eq!(selection.highlights(scene), vec![address("member", "b1"), address("fastener", "b1:bolt-1")]);
}

#[test]
fn pending_progress_exists_only_while_slow_units_run_and_never_exceeds_the_total() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::EvalProgress;
    let progress = EvalProgress { done: 3, total: 5, ..EvalProgress::default() };
    assert_eq!(pending_of(true, &progress), None);
    assert_eq!(pending_of(false, &progress), Some(ChecksPending { completed: 3, total: 5 }));
    assert_eq!(pending_of(false, &EvalProgress { done: 4, total: 0, ..EvalProgress::default() }), Some(ChecksPending { completed: 4, total: 4 }));
    assert!(PAINT_BUDGET > DRAG_BUDGET);
}

#[test]
fn the_view_lane_chooses_the_unit_system_and_the_picker_events_validate() {
    use crate::{view_event, LaneEvent, NormTransient, VIEW_ACTION_UNITS};
    assert_eq!(unit_system_of(&ViewLane::default()), None);
    let chosen = NormTransient::default().apply(view_event(VIEW_ACTION_UNITS, Some("si"), None).expect("event"));
    assert_eq!(unit_system_of(&chosen.view), Some(UnitSystem::Si));
    assert_eq!(unit_system_of(&chosen.apply(LaneEvent::ViewUnits(Some("engineering".into()))).view), Some(UnitSystem::Engineering));
    assert_eq!(view_event(VIEW_ACTION_UNITS, Some("furlong"), None), Some(LaneEvent::ViewUnits(None)), "an unknown system clears the choice");
    let options = NormSelection { units: Some(UnitSystem::Si), ..NormSelection::default() }.inspector_options();
    assert!(options.preset == UnitSystem::Si && options.unit_picker);
}

#[test]
fn a_viewport_pick_narrows_the_checks_list_to_the_picked_entities_and_selects_the_governing_check() {
    let report = report();
    let selection = NormSelection::resolve(&report, None, &[address("member", "b1:flange")]);
    assert_eq!(selection.picked, ["b1"], "a part pick narrows to its base entity");
    assert_eq!(selection.check, Some(1), "the governing check of the picked entity is the selected row");
    let state = ChecksViewState { owners: selection.picked.clone(), ..ChecksViewState::default() };
    assert_eq!(plan_checks(&report, &state, Locale::En).iter().flat_map(|group| group.rows.clone()).collect::<std::collections::BTreeSet<_>>(), [0, 1].into());
    assert!(NormSelection::resolve(&report, Some(2), &[]).picked.is_empty(), "a stepped check does not narrow the list");
    let unowned = ChecksViewState { owners: vec!["sections".into()], ..ChecksViewState::default() };
    assert_eq!(plan_checks(&report, &unowned, Locale::En).iter().map(|group| group.rows.len()).sum::<usize>(), 3, "a pick without checks leaves the list whole");
}

#[test]
fn interaction_picks_and_stored_checks_share_one_consistent_focus() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️cases.json")).expect("selection fixture");
    let mut report = CheckReport::default();
    for row in fixture["checks"].as_array().expect("checks") {
        report.push(check(row["id"].as_str().unwrap(), row["entity"].as_str().unwrap(), row["path"].as_str().unwrap(), row["utilization"].as_f64().unwrap()));
    }
    for case in fixture["cases"].as_array().expect("cases") {
        let picks: Vec<String> = case["picks"].as_array().unwrap().iter().map(|pick| address(pick[0].as_str().unwrap(), pick[1].as_str().unwrap())).collect();
        let stored = case["stored"].as_u64().map(|value| value as u32);
        let selection = if case["interaction"].as_bool().unwrap_or(false) { NormSelection::from_interaction(&report, stored, &picks) } else { NormSelection::resolve(&report, stored, &picks) };
        let actual = serde_json::json!({"entities": selection.entities, "picked": selection.picked, "check": selection.check, "primary": selection.primary(&report), "paths": selection.paths});
        let expected = serde_json::json!({"entities": case["entities"], "picked": case["picked"], "check": case["check"], "primary": case["primary"], "paths": case["paths"]});
        assert_eq!(actual, expected, "{}", case["name"]);
        let owners = selection.checks_owners(&report);
        assert_eq!(serde_json::json!(owners), case["checkOwners"], "{}: checks filter", case["name"]);
        let state = ChecksViewState { owners, ..ChecksViewState::default() };
        if let Some(selected) = selection.check {
            assert!(plan_checks(&report, &state, Locale::En).iter().any(|group| group.rows.contains(&selected)), "{}: selected check remains visible", case["name"]);
        }
    }
}
