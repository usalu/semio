//! 🧪️ The coordination panels: the clash tree by set, group and clash with its four actions, the rules tree by rule and violation, the issues tree by status with comments and actions, in both languages.

use super::*;
use crate::standards::v1::subsets::any::schema::inferences::rule_results::RuleFinding;
use crate::{ClashRef, IssueComment, IssuePriority, RuleKind, RuleScope};
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_framework_plugin::ViewModel;
use semio_framework_ui_locale::{Locale, Terminology};

const FRAME: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧨️clash-sets/🏢️frame/📸️snapshot/🔣️.json");

fn frame() -> (ModelSnapshot, ModelInference) {
    let snapshot: ModelSnapshot = from_json_str(FRAME, JsonMemberPolicy::Reject).expect("the committed frame decodes");
    let inference = ModelInference::infer(&snapshot).expect("the frame infers");
    (snapshot, inference)
}

fn project(node: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("the tree projects")
}

fn clashes_text(snapshot: &ModelSnapshot, inference: &ModelInference, locale: Locale) -> String {
    let view = ViewModel::new(locale, Terminology::Native);
    project(render_clashes(snapshot, inference, crate::editor::bim::terminology::bim_labels(&view), &TreeWindows::for_body(&view, CLASHES_KEY)).expect("the clash panel renders"))
}

fn rules_text(snapshot: &ModelSnapshot, inference: &ModelInference, locale: Locale) -> String {
    let view = ViewModel::new(locale, Terminology::Native);
    project(render_rules(snapshot, inference, crate::editor::bim::terminology::bim_labels(&view), &TreeWindows::for_body(&view, RULES_KEY)).expect("the rules panel renders"))
}

fn issues_text(snapshot: &ModelSnapshot, locale: Locale) -> String {
    let view = ViewModel::new(locale, Terminology::Native);
    project(render_issues(snapshot, crate::editor::bim::terminology::bim_labels(&view), &TreeWindows::for_body(&view, ISSUES_KEY)).expect("the issues panel renders"))
}

#[semio_framework_async_macros::async_test]
async fn the_frame_lists_its_clash_sets_with_their_counts_in_both_languages() {
    let (snapshot, inference) = frame();
    let english = clashes_text(&snapshot, &inference, Locale::En);
    let german = clashes_text(&snapshot, &inference, Locale::De);
    for set in snapshot.clash_sets.values() {
        assert!(english.contains(&set.name) && german.contains(&set.name), "{}", set.name);
    }
    assert!(english.contains("Run clash detection") && german.contains("Kollisionsprüfung"));
    assert!(english.contains("hard") && german.contains("hart"));
}

#[semio_framework_async_macros::async_test]
async fn a_clash_row_opens_the_four_actions_on_its_pair() {
    let (snapshot, inference) = frame();
    let result = inference.clash_sets.values().find(|result| !result.clashes.is_empty()).expect("the frame has clashes");
    let rows = grouped(result);
    assert_eq!(rows.iter().map(|(_, clashes)| clashes.len()).sum::<usize>(), result.clashes.len(), "every clash is in exactly one group");
    let text = clashes_text(&snapshot, &inference, Locale::En);
    for label in ["Select the pair", "Zoom to the clash", "Isolate the clash", "Raise an issue"] {
        assert!(text.contains(label), "{label}");
    }
    assert!(text.contains("selectFindings") && text.contains("viewClash") && text.contains("raiseIssue"));
}

#[semio_framework_async_macros::async_test]
async fn a_model_without_clash_sets_says_so_in_both_languages() {
    let snapshot = ModelSnapshot::default();
    let inference = ModelInference::default();
    assert!(clashes_text(&snapshot, &inference, Locale::En).contains("No clash set"));
    assert!(clashes_text(&snapshot, &inference, Locale::De).contains("Kein Kollisionssatz"));
    assert!(rules_text(&snapshot, &inference, Locale::En).contains("No rule"));
    assert!(issues_text(&snapshot, Locale::De).contains("Kein Hinweis"));
}

fn with_rule(violating: bool) -> (ModelSnapshot, ModelInference) {
    let (mut snapshot, mut inference) = frame();
    snapshot.rules.insert("r-1".into(), Rule { name: "Door width".into(), kind: RuleKind::MinDoorWidth, limit: 0.9, severity: RuleSeverity::Warning, scope: RuleScope::all() });
    let violations = if violating { vec![RuleFinding { element: "w-1".into(), measured: 0.8, limit: 0.9, storey: "st-0".into() }] } else { Vec::new() };
    inference.rule_results.insert("r-1".into(), RuleResult { checked: 4, violations });
    (snapshot, inference)
}

#[semio_framework_async_macros::async_test]
async fn a_rule_lists_the_members_that_break_it_with_the_measure_against_the_limit() {
    let (snapshot, inference) = with_rule(true);
    let english = rules_text(&snapshot, &inference, Locale::En);
    assert!(english.contains("Door width (1 / 4)") && english.contains("0.8") && english.contains("0.9") && english.contains("selectFindings"), "{english}");
    let german = rules_text(&snapshot, &inference, Locale::De);
    assert!(german.contains("Door width (1 / 4)") && german.contains("Warnung"));
}

#[semio_framework_async_macros::async_test]
async fn a_rule_that_holds_says_so() {
    let (snapshot, inference) = with_rule(false);
    assert!(rules_text(&snapshot, &inference, Locale::En).contains("All members keep this rule"));
    assert!(rules_text(&snapshot, &inference, Locale::De).contains("Alle Bauteile halten diese Regel ein"));
}

fn with_issues() -> ModelSnapshot {
    let (mut snapshot, _) = frame();
    let issue = |title: &str, status: IssueStatus, elements: &[&str], viewpoint: bool| Issue {
        title: title.into(),
        description: String::new(),
        status,
        priority: IssuePriority::High,
        assignee: "AB".into(),
        author: "UG".into(),
        created: "2026-10-09".into(),
        labels: Vec::new(),
        elements: elements.iter().map(|id| id.to_string()).collect(),
        clash: Some(ClashRef { set: "cs-frame".into(), first: "b-1".into(), second: "c-1".into() }).filter(|_| viewpoint),
        viewpoint: viewpoint.then(|| crate::IssueViewpoint { camera: crate::ViewCamera { target: crate::Point2 { x: 0.0, y: 0.0 }, target_height: 1.0, azimuth: 0.0, pitch: 0.3, distance: 10.0 }, section: None, isolate: Vec::new() }),
    };
    snapshot.issues.insert("i-1".into(), issue("Beam hits column", IssueStatus::Open, &["b-1", "c-1"], true));
    snapshot.issues.insert("i-2".into(), issue("Slab edge", IssueStatus::Closed, &[], false));
    snapshot.issue_comments.insert("ic-1".into(), IssueComment { issue: "i-1".into(), author: "AB".into(), date: "2026-10-09T09:00:00Z".into(), text: "Looking at it.".into() });
    snapshot
}

#[semio_framework_async_macros::async_test]
async fn issues_are_listed_by_status_with_their_actions_and_comments() {
    let snapshot = with_issues();
    let english = issues_text(&snapshot, Locale::En);
    assert!(english.contains("Open (1)") && english.contains("Closed (1)") && english.contains("Beam hits column") && english.contains("Slab edge"), "{english}");
    assert!(english.contains("Restore the viewpoint") && english.contains("Capture the viewpoint") && english.contains("AB: Looking at it."), "{english}");
    assert!(english.contains("restoreViewpoint") && english.contains("captureViewpoint") && english.contains("exportModel"), "{english}");
    let german = issues_text(&snapshot, Locale::De);
    assert!(german.contains("Offen (1)") && german.contains("Abgeschlossen (1)") && german.contains("BCF exportieren"), "{german}");
}

#[semio_framework_async_macros::async_test]
async fn an_issue_without_a_viewpoint_or_elements_offers_only_capture_and_comment() {
    let snapshot = with_issues();
    let closed = issues_with(&snapshot, IssueStatus::Closed);
    assert_eq!(closed, ["i-2"]);
    let english = issues_text(&snapshot, Locale::En);
    let slab = &english[english.find("Slab edge").expect("the closed issue")..];
    assert!(slab.contains("captureViewpoint") && !slab.contains("restoreViewpoint"));
}

#[semio_framework_async_macros::async_test]
async fn every_status_and_the_names_of_elements_have_a_word_in_both_languages() {
    let snapshot = with_issues();
    for status in STATUSES {
        assert_ne!(status_label(&BimLabels::NATIVE_EN, status), status_label(&BimLabels::NATIVE_DE, status));
    }
    assert_eq!(name_of(&snapshot, "b-1"), "B1");
    assert_eq!(name_of(&snapshot, "nothing"), "nothing");
    assert_eq!(clip(&"x".repeat(500)).chars().count(), CLIP);
}
