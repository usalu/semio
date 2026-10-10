//! 🧪️ The coordination rows: selector and scope text round trips, field reads and writes, the pickers, the create defaults and the inferred counts.

use super::*;
use crate::editor::bim::terminology::BimLabels;
use crate::{ClashRef, IssueViewpoint, Point2, Point3, SectionBox, ViewCamera};

fn model() -> ModelSnapshot {
    let mut model = ModelSnapshot::default();
    model.project.author = "UG".into();
    model.clash_sets.insert("cs-1".into(), ClashSet { name: "Beams against walls".into(), a: ElementSelector { classes: vec![ElementClass::Beam], storeys: vec!["st-0".into()], phases: vec![Phase::New], ids: Vec::new() }, b: ElementSelector::of_classes(&[ElementClass::Wall]), tolerance: 0.002, clearance: 0.05 });
    model.rules.insert("r-1".into(), Rule { name: "Riser".into(), kind: RuleKind::MaxRiser, limit: 0.19, severity: RuleSeverity::Error, scope: RuleScope { storeys: vec!["st-0".into()], phases: Vec::new(), ids: Vec::new(), filter: "Corridor".into() } });
    let camera = ViewCamera { target: Point2 { x: 1.0, y: 2.0 }, target_height: 1.5, azimuth: 0.5, pitch: 0.4, distance: 12.0 };
    let viewpoint = IssueViewpoint { camera, section: Some(SectionBox { min: Point3 { x: 0.0, y: 0.0, z: 0.0 }, max: Point3 { x: 1.0, y: 1.0, z: 1.0 } }), isolate: vec!["w-1".into()] };
    model.issues.insert("i-1".into(), Issue { title: "Beam hits wall".into(), description: String::new(), status: IssueStatus::Open, priority: IssuePriority::High, assignee: "AB".into(), author: "UG".into(), created: "2026-10-09T08:00:00Z".into(), labels: vec!["Clash".into()], elements: vec!["b-1".into(), "w-1".into()], clash: Some(ClashRef { set: "cs-1".into(), first: "b-1".into(), second: "w-1".into() }), viewpoint: Some(viewpoint) });
    model.issue_comments.insert("c-1".into(), IssueComment { issue: "i-1".into(), author: "AB".into(), date: "2026-10-09T09:00:00Z".into(), text: "On it, will check the bearing before Friday.".into() });
    model
}

#[test]
fn a_selector_reads_back_the_text_it_is_shown_as() {
    let model = model();
    let selector = &model.clash_sets["cs-1"].a;
    assert_eq!(selector_text(selector), "classes=beam; storeys=st-0; phases=New");
    assert_eq!(parse_selector(&selector_text(selector)).as_ref(), Some(selector));
    assert_eq!(selector_text(&ElementSelector::all()), "");
    assert_eq!(parse_selector(""), Some(ElementSelector::all()));
    assert_eq!(parse_selector("classes=Wall , curtain-wall; ids=w-1").map(|s| s.classes), Some(vec![ElementClass::Wall, ElementClass::CurtainWall]));
    assert_eq!(parse_selector("classes=teapot"), None);
    assert_eq!(parse_selector("phases=Someday"), None);
    assert_eq!(parse_selector("colour=red"), None);
    assert_eq!(parse_selector("classes"), None);
}

#[test]
fn a_scope_reads_back_the_text_it_is_shown_as() {
    let scope = &model().rules["r-1"].scope;
    assert_eq!(scope_text(scope), "storeys=st-0; filter=Corridor");
    assert_eq!(parse_scope(&scope_text(scope)).as_ref(), Some(scope));
    assert_eq!(parse_scope("phases=Existing,New").map(|s| s.phases), Some(vec![Phase::Existing, Phase::New]));
    assert_eq!(parse_scope("classes=wall"), None);
}

#[test]
fn every_field_with_a_mutation_writes_the_value_its_own_read_shows() {
    let model = model();
    for (row, id) in [(&CLASH_SET, "cs-1"), (&RULE, "r-1"), (&ISSUE, "i-1"), (&ISSUE_COMMENT, "c-1")] {
        for field in row.fields.iter().filter(|field| field.write.is_some()) {
            let shown = (field.read)(&model, id).expect("a field reads off its record");
            assert!(field.write.expect("a writer")(&model, id, &shown).is_some(), "{}.{} must decode what it shows: {shown:?}", row.kind, field.key);
        }
    }
}

#[test]
fn a_changed_field_becomes_the_set_mutation_of_exactly_that_field() {
    let model = model();
    let write = |row: &EntityKind, id: &str, key: &str, value: &str| row.fields.iter().find(|field| field.key == key).and_then(|field| field.write).and_then(|write| write(&model, id, value));
    assert!(matches!(write(&CLASH_SET, "cs-1", "tolerance", "0.01"), Some(ModelMutation::SetClashSet(set)) if set.tolerance == Some(0.01) && set.clearance.is_none() && set.name.is_none()));
    assert!(matches!(write(&RULE, "r-1", "kind", "MinTread"), Some(ModelMutation::SetRule(set)) if set.kind == Some(RuleKind::MinTread)));
    assert!(matches!(write(&RULE, "r-1", "kind", "min-door-width"), Some(ModelMutation::SetRule(set)) if set.kind == Some(RuleKind::MinDoorWidth)));
    assert!(matches!(write(&ISSUE, "i-1", "status", "Resolved"), Some(ModelMutation::SetIssue(set)) if set.status == Some(IssueStatus::Resolved)));
    assert!(matches!(write(&ISSUE, "i-1", "labels", "A, B,,C"), Some(ModelMutation::SetIssue(set)) if set.labels == Some(vec!["A".into(), "B".into(), "C".into()])));
    assert_eq!(write(&ISSUE, "i-1", "status", "Sleeping"), None);
    assert_eq!(write(&RULE, "r-1", "limit", "plenty"), None);
    assert!(ISSUE.fields.iter().filter(|field| matches!(field.key, "clash" | "viewpoint")).all(|field| field.write.is_none()), "the clash and the viewpoint are set by their commands");
}

#[test]
fn issues_show_their_clash_and_viewpoint_and_comments_nest_under_their_issue() {
    let model = model();
    let read = |row: &EntityKind, key: &str| row.fields.iter().find(|field| field.key == key).and_then(|field| (field.read)(&model, if row.kind == "issue" { "i-1" } else { "c-1" }));
    assert_eq!(read(&ISSUE, "clash").as_deref(), Some("cs-1: b-1 ↔ w-1"));
    assert_eq!(read(&ISSUE, "viewpoint").as_deref(), Some("camera 12 m, section box, 1 isolated"));
    assert_eq!((ISSUE_COMMENT.parent)(&model, "c-1").as_deref(), Some("i-1"));
    assert_eq!((ISSUE_COMMENT.name)(&model, "c-1").as_deref(), Some("On it, will check the bearing before Frida"));
}

#[test]
fn the_pickers_offer_every_value_in_both_languages() {
    let model = model();
    for labels in [&BimLabels::NATIVE_EN, &BimLabels::NATIVE_DE] {
        assert_eq!(kind_choices(&model, labels).len(), 8);
        assert_eq!(severity_choices(&model, labels).len(), 3);
        assert_eq!(status_choices(&model, labels).len(), 4);
        assert_eq!(priority_choices(&model, labels).len(), 4);
        for (stored, label) in kind_choices(&model, labels).into_iter().chain(status_choices(&model, labels)).chain(priority_choices(&model, labels)) {
            assert!(!label.is_empty() && !stored.is_empty());
        }
    }
    assert_ne!(kind_choices(&model, &BimLabels::NATIVE_EN)[0].1, kind_choices(&model, &BimLabels::NATIVE_DE)[0].1);
}

#[test]
fn new_records_start_with_the_conventions_and_the_latest_moment_the_model_knows() {
    let model = model();
    assert!(latest_moment(&model).starts_with("2026-10-09"));
    assert_eq!(latest_moment(&ModelSnapshot::default()), "1970-01-01");
    let Ok(ModelMutation::CreateIssue(create)) = create_issue(&model, "i-2", "", "Door swings into the corridor") else { panic!("an issue") };
    assert_eq!((create.issue.author.as_str(), create.issue.status, create.issue.priority), ("UG", IssueStatus::Open, IssuePriority::Normal));
    assert!(crate::valid_timestamp(&create.issue.created));
    let Ok(ModelMutation::CreateRule(create)) = create_rule(&model, "r-2", "", "Riser") else { panic!("a rule") };
    assert_eq!((create.rule.kind, create.rule.limit), (RuleKind::MaxRiser, 0.19));
    let Ok(ModelMutation::CreateClashSet(create)) = create_clash_set(&model, "cs-2", "", "Everything") else { panic!("a clash set") };
    assert_eq!((create.clash_set.tolerance, create.clash_set.a), (DEFAULT_TOLERANCE, ElementSelector::all()));
    assert!(matches!(create_comment(&model, "c-2", "i-1", "Thanks"), Ok(ModelMutation::CreateIssueComment(_))));
    assert_eq!(create_comment(&model, "c-2", "i-9", "Thanks").err(), Some("bim.create.issue-missing"));
    assert_eq!(create_issue(&ModelSnapshot::default(), "i-1", "", "x").ok().map(|mutation| matches!(mutation, ModelMutation::CreateIssue(c) if c.issue.author == "unknown")), Some(true));
}

#[test]
fn the_inferred_rows_read_the_clash_and_rule_results() {
    let model = model();
    let mut inference = crate::ModelInference::default();
    inference.clash_sets.insert("cs-1".into(), Default::default());
    inference.rule_results.insert("r-1".into(), crate::standards::v1::subsets::any::schema::inferences::rule_results::RuleResult { checked: 3, violations: Vec::new() });
    let read = |rows: &'static [InferredRow], key: &str, id: &str| rows.iter().find(|row| row.key == key).and_then(|row| (row.read)(&model, &inference, id));
    assert_eq!(read(CLASH_SET_INFERRED, "hard", "cs-1").as_deref(), Some("0"));
    assert_eq!(read(CLASH_SET_INFERRED, "hard", "cs-9"), None);
    assert_eq!(read(RULE_INFERRED, "checked", "r-1").as_deref(), Some("3"));
    assert_eq!(read(RULE_INFERRED, "violations", "r-1").as_deref(), Some("0"));
}
