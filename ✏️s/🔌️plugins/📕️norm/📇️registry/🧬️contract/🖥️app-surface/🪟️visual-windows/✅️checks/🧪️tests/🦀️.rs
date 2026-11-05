//! 🧪️ Unit tests of the checks window; scenarios in [`🥒️.feature`](🥒️.feature). The independent oracles are `serde_json` for the projected tree and `std` sorting for the order.

use crate::*;
use semio_s_plugin_norm_compliance::{AnnexChoice, LocalizedCopy, SubjectRef};

fn check(id: &str, part: &str, section: &str, entity: &str, utilization: f64, status: CheckStatus) -> CheckResult {
    let mut builder = CheckResult::assess(id, part, ClauseId::new("EN 1993", "1-1", section), SubjectRef::new(entity, format!("members[{entity}].tM"), LocalizedCopy::new(format!("Member {entity}"), format!("Stab {entity}"))), LocalizedCopy::new(format!("check {id}"), format!("Nachweis {id}")))
        .utilization(Quantity::new(QuantityKind::Length, utilization * 0.0125), Quantity::new(QuantityKind::Length, 0.0125))
        .explanation(LocalizedCopy::new("explained", "erläutert"))
        .annex(AnnexChoice::De);
    if status == CheckStatus::Fail {
        builder = builder.remedy(Remedy::at_least(SubjectRef::new(entity, format!("members[{entity}].tM"), LocalizedCopy::new("t", "t")), Quantity::length_m(0.012), Quantity::length_m(0.022), LocalizedCopy::new("Increase thickness", "Dicke erhöhen")));
    }
    builder.status(status).build()
}

fn report() -> CheckReport {
    let mut report = CheckReport::default();
    report.push(check("a", "ULS", "6.2.9", "B1", 0.64, CheckStatus::Pass));
    report.push(check("b", "ULS", "6.2.10", "B2", 1.12, CheckStatus::Fail));
    report.push(check("c", "SLS", "7.2", "B1", 0.95, CheckStatus::Warning));
    report.push(check("d", "SLS", "7.3", "B3", 0.0, CheckStatus::NotApplicable));
    report
}

fn project(node: BuiltNode) -> String {
    semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).expect("project the checks tree")
}

fn render(report: &CheckReport, state: &ChecksViewState, spec: &ChecksWindowSpec, locale: Locale) -> String {
    project(render_checks_with(report, state, spec, &TreeWindows::unhosted(), locale).expect("checks window"))
}

#[test]
fn the_bar_encodes_value_band_and_the_hundred_percent_tick_with_patterns() {
    assert_eq!(utilisation_bar_text(CheckStatus::Pass, 0.64), "██████░░░░┃░░");
    assert_eq!(utilisation_bar_text(CheckStatus::Warning, 0.9), "▓▓▓▓▓▓▓▓▓░┃░░");
    assert_eq!(utilisation_bar_text(CheckStatus::Fail, 5.0), "▒▒▒▒▒▒▒▒▒▒┃▒▒");
    assert_eq!(utilisation_bar_text(CheckStatus::NotApplicable, 0.0), "············");
    assert_eq!(UtilisationBand::of(CheckStatus::Pass, 1.01), UtilisationBand::Over);
    assert_eq!(UtilisationBand::of(CheckStatus::UnverifiedBasis, 0.5), UtilisationBand::Unverified);
    assert_eq!(status_glyph_text(CheckStatus::UnverifiedBasis, 0.5, Locale::En), "? Unverified basis");
    assert_eq!(status_glyph_text(CheckStatus::UnverifiedBasis, 0.5, Locale::De), "? Ungeprüfte Grundlage");
    let fills: std::collections::HashSet<char> = [UtilisationBand::Within, UtilisationBand::Warning, UtilisationBand::Over, UtilisationBand::NotApplicable, UtilisationBand::Unverified].iter().map(|band| band.fill()).collect();
    assert_eq!(fills.len(), 5, "every band has its own pattern");
}

#[test]
fn percent_and_quantities_use_the_locale_decimal_separator() {
    assert_eq!(utilisation_percent_text(0.64, Locale::En), "64 %");
    assert_eq!(utilisation_percent_text(0.056, Locale::En), "5.6 %");
    assert_eq!(utilisation_percent_text(0.056, Locale::De), "5,6 %");
    assert_eq!(quantity_text(&Quantity::length_m(0.0123), Locale::En), "12.3 mm");
    assert_eq!(quantity_text(&Quantity::length_m(0.0123), Locale::De), "12,3 mm");
}

#[test]
fn view_state_round_trips_and_rejects_unknown_wire_values() {
    let state = ChecksViewState { grouping: ChecksGrouping::Clause, sort: ChecksSort::Title, filter: ChecksFilter { pass: false, query: "b1".into(), ..ChecksFilter::default() }, ..ChecksViewState::default() };
    assert_eq!(ChecksViewState::decode(&state.encode()), Some(state));
    assert_eq!(ChecksViewState::decode("grouping=zone"), None);
    assert_eq!(ChecksViewState::decode("status=x"), None);
    assert_eq!(ChecksViewState::decode("nonsense"), None);
}

#[test]
fn clauses_order_naturally_and_read_in_both_languages() {
    let (nine, ten) = (ClauseId::new("EN 1993", "1-1", "6.2.9"), ClauseId::new("EN 1993", "1-1", "6.2.10"));
    assert_eq!(clause_order(&nine, &ten), std::cmp::Ordering::Less);
    assert_eq!(clause_text(&nine, Locale::En), "EN 1993 1-1 §6.2.9");
    assert_eq!(clause_text(&nine, Locale::De), "EN 1993 1-1 Abschn. 6.2.9");
}

#[test]
fn worst_first_plan_matches_a_std_sort_and_groups_by_entity() {
    let report = report();
    let groups = plan_checks(&report, &ChecksViewState::default(), Locale::En);
    let keys: Vec<&str> = groups.iter().map(|group| group.key.as_str()).collect();
    assert_eq!(keys, ["B2", "B1", "B3"]);
    let mut expected: Vec<usize> = vec![0, 2];
    expected.sort_by(|a, b| report.checks[*b].utilization.partial_cmp(&report.checks[*a].utilization).unwrap());
    assert_eq!(groups[1].rows, expected);
    assert_eq!(groups[1].counts, [1, 1, 0, 0]);
}

#[test]
fn filters_by_status_and_query_and_groups_by_part_or_clause() {
    let report = report();
    let failing = ChecksViewState { filter: ChecksFilter { pass: false, warning: false, not_applicable: false, ..ChecksFilter::default() }, ..ChecksViewState::default() };
    assert_eq!(plan_checks(&report, &failing, Locale::En).iter().flat_map(|group| group.rows.clone()).collect::<Vec<_>>(), [1]);
    let query = ChecksViewState { filter: ChecksFilter { query: "stab b3".into(), ..ChecksFilter::default() }, ..ChecksViewState::default() };
    assert_eq!(plan_checks(&report, &query, Locale::De).iter().flat_map(|group| group.rows.clone()).collect::<Vec<_>>(), [3]);
    let by_part = ChecksViewState { grouping: ChecksGrouping::Part, sort: ChecksSort::ReportOrder, ..ChecksViewState::default() };
    assert_eq!(plan_checks(&report, &by_part, Locale::En).iter().map(|group| group.key.clone()).collect::<Vec<_>>(), ["ULS", "SLS"]);
    let by_clause = ChecksViewState { grouping: ChecksGrouping::Clause, sort: ChecksSort::ClauseOrder, ..ChecksViewState::default() };
    assert_eq!(plan_checks(&report, &by_clause, Locale::En)[0].rows, [0, 1, 2, 3]);
}

#[test]
fn a_check_selects_its_entity_and_field_path() {
    let report = report();
    assert_eq!(check_selection(&report.checks[1]), Some(CheckSelection { entity_id: "B2".into(), path: "members[B2].tM".into() }));
    let mut whole = report.checks[0].clone();
    whole.subject = SubjectRef::whole(LocalizedCopy::new("All", "Alle"));
    assert_eq!(check_selection(&whole), None);
}

#[test]
fn rows_carry_glyph_text_bar_and_status_beyond_colour_in_both_languages() {
    let report = report();
    let en = render(&report, &ChecksViewState::default(), &ChecksWindowSpec::default(), Locale::En);
    let de = render(&report, &ChecksViewState::default(), &ChecksWindowSpec::default(), Locale::De);
    for glyph in ["✓", "⚠", "✗", "∅", "┃"] {
        assert!(en.contains(glyph), "{glyph}: {en}");
    }
    assert!(en.contains("Does not comply") && de.contains("Nicht erfüllt"), "{en}\n{de}");
    assert!(en.contains("EN 1993 1-1 §6.2.10") && de.contains("Abschn. 6.2.10"), "{de}");
    assert!(en.contains("Increase thickness") && de.contains("Dicke erhöhen"));
    assert!(de.contains("1,12") || de.contains("112 %"), "{de}");
    assert!(en.contains("rogress"), "worst-utilisation progress node: {en}");
    let parsed: serde_json::Value = serde_json::from_str(&en).expect("valid json");
    assert!(parsed.is_object());
}

#[test]
fn selection_and_remedy_actions_are_bound_only_for_an_editor_controller() {
    let report = report();
    let viewer = render(&report, &ChecksViewState::default(), &ChecksWindowSpec::default(), Locale::En);
    assert!(!viewer.contains("setSelectedCheckIndex") && !viewer.contains("applyRemedy"), "{viewer}");
    let editor = render(&report, &ChecksViewState { selected: Some(1), ..ChecksViewState::default() }, &ChecksWindowSpec { controller_id: Some("s.norm.x@1/*#editor"), ..ChecksWindowSpec::default() }, Locale::En);
    assert!(editor.contains("setSelectedCheckIndex") && editor.contains("applyRemedy"), "{editor}");
}

#[test]
fn entity_groups_pick_the_raw_entity_id_in_the_interaction_domain() {
    let spec = ChecksWindowSpec { controller_id: Some("s.norm.x@1/*#editor"), interaction_domain: Some("norm.x"), ..ChecksWindowSpec::default() };
    let json = render(&report(), &ChecksViewState::default(), &spec, Locale::En);
    assert!(json.contains("\"B2\"") && json.contains("entity") && json.contains("norm.x"), "{json}");
    let by_part = render(&report(), &ChecksViewState { grouping: ChecksGrouping::Part, ..ChecksViewState::default() }, &spec, Locale::En);
    assert!(!by_part.contains("\"granularity\""), "{by_part}");
}

#[test]
fn optional_controls_cancel_and_clause_links_appear_only_when_declared() {
    let plain = ChecksWindowSpec { controller_id: Some("s.norm.x@1/*#editor"), ..ChecksWindowSpec::default() };
    let declared = ChecksWindowSpec { actions: ChecksActions { view_controls: true, clause_links: true, cancel_evaluation: true }, ..plain };
    let state = ChecksViewState { pending: Some(ChecksPending { completed: 2, total: 9 }), ..ChecksViewState::default() };
    let without = render(&report(), &state, &plain, Locale::En);
    let with = render(&report(), &state, &declared, Locale::En);
    for action in ["setChecksView", "cancelEvaluation", "openClause"] {
        assert!(!without.contains(action) && with.contains(action), "{action}");
    }
    assert!(with.contains("2 / 9"));
}

#[test]
fn the_table_twin_lines_up_with_the_checks() {
    let report = report();
    assert_eq!(report_table_rows(&report, Locale::En).len(), report.checks.len());
    assert_eq!(report_table_columns(Locale::De).len(), report_table_rows(&report, Locale::De)[0].len());
}

#[test]
fn remedy_chips_of_live_editors_trigger_ghost_previews_and_a_clear_row() {
    let live = render(&report(), &ChecksViewState { selected: Some(1), ..ChecksViewState::default() }, &ChecksWindowSpec { controller_id: Some("s.norm.en1993@1/*#editor"), ..ChecksWindowSpec::default() }, Locale::En);
    assert!(live.contains("previewRemedy") && live.contains("clearPreview") && live.contains("Preview"), "{live}");
    let german = render(&report(), &ChecksViewState { selected: Some(1), ..ChecksViewState::default() }, &ChecksWindowSpec { controller_id: Some("s.norm.en1993@1/*#editor"), ..ChecksWindowSpec::default() }, Locale::De);
    assert!(german.contains("Vorschau"), "{german}");
    let other = render(&report(), &ChecksViewState { selected: Some(1), ..ChecksViewState::default() }, &ChecksWindowSpec { controller_id: Some("s.norm.x@1/*#editor"), ..ChecksWindowSpec::default() }, Locale::En);
    assert!(!other.contains("previewRemedy") && other.contains("applyRemedy"), "{other}");
    assert!(controller_is_live("s.norm.en1990@1/*#editor") && !controller_is_live("s.norm.din4108@1/*#editor"));
}

#[test]
fn the_checks_window_publishes_a_table_and_one_row_node_per_listed_check_in_both_languages() {
    use crate::accessibility::published_nodes::published;
    let report = report();
    let state = ChecksViewState { selected: Some(1), ..ChecksViewState::default() };
    let spec = ChecksWindowSpec { interaction_domain: Some("norm.checks-nodes-test"), ..ChecksWindowSpec::default() };
    for locale in [Locale::En, Locale::De] {
        render_checks_with(&report, &state, &spec, &TreeWindows::unhosted(), locale).expect("checks window");
        let nodes = published("checks", "norm.checks-nodes-test");
        assert_eq!(nodes, check_window_nodes(&report, &state, locale));
        assert_eq!(nodes.len(), 1 + report.checks.len());
        assert_eq!((nodes[0].node.role, nodes[0].node.name.as_str()), ("table", if locale == Locale::De { "Nachweise" } else { "Checks" }));
        let rows: Vec<_> = nodes[1..].iter().collect();
        assert!(rows.iter().all(|n| n.node.role == "row" && n.depth == 1 && n.node.missing_required().is_empty()));
        assert_eq!(rows.iter().map(|n| n.node.position).collect::<Vec<_>>(), (1..=report.checks.len()).map(|p| Some((p, report.checks.len()))).collect::<Vec<_>>());
        for check in &report.checks {
            let node = nodes.iter().find(|n| n.key == format!("check:{}", check.id)).expect("a row per check");
            let title = if locale == Locale::De { &check.title.de } else { &check.title.en };
            assert!(node.node.name.starts_with(title.as_str()), "{}", node.node.name);
            assert_eq!(node.node.description.as_deref(), Some(clause_text(&check.clause, locale).as_str()));
        }
        assert_eq!(nodes.iter().filter(|n| n.node.selected == Some(true)).count(), 1);
        assert_eq!(nodes.iter().find(|n| n.node.selected == Some(true)).map(|n| n.key.as_str()), Some("check:b"));
    }
    let filtered = ChecksViewState { filter: ChecksFilter { fail: false, ..ChecksFilter::default() }, ..ChecksViewState::default() };
    assert_eq!(check_window_nodes(&report, &filtered, Locale::En).len(), 1 + report.checks.len() - 1, "a filtered row is not announced");
}
