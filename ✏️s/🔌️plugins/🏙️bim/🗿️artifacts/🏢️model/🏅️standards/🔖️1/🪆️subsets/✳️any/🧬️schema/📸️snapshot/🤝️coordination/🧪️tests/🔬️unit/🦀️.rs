//! 🧪️ The coordination rules: element classes, storeys and phases, selectors, scopes, moments and why a clash set, a rule, an issue or a comment cannot be written.

use super::*;
use crate::{Point2, Point3, SectionBox, ViewCamera};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️create-beam/✅️adds-below-the-storey-top/📸️snapshot/⬅️before/🔣️.json");

fn model() -> ModelSnapshot {
    let mut document: serde_json::Value = serde_json::from_str(HOUSE).expect("the committed house is JSON");
    document["window_types"]["win-1"] = serde_json::json!({ "name": "Window", "width": 1.2, "height": 1.2, "sill": 0.9, "frame_width": 0.05, "frame_depth": 0.08, "panes": 2, "material": "m-brick" });
    document["openings"]["o-window"] = serde_json::json!({ "host": "w-south", "kind": { "Window": { "window_type": "win-1" } }, "offset": 2.0, "flip_hand": false, "flip_facing": false, "name": "Window" });
    document["openings"]["o-void"] = serde_json::json!({ "host": "w-south", "kind": { "Void": { "width": 0.5, "height": 0.5 } }, "offset": 5.0, "flip_hand": false, "flip_facing": false, "name": "Void" });
    document["walls"]["w-south"]["phase"] = serde_json::json!("Existing");
    from_json_str(&document.to_string(), JsonMemberPolicy::Reject).expect("the house decodes")
}

fn camera() -> ViewCamera {
    ViewCamera { target: Point2 { x: 4.0, y: 3.0 }, target_height: 1.5, azimuth: 0.8, pitch: 0.5, distance: 25.0 }
}

fn issue() -> Issue {
    Issue {
        title: "Beam hits the wall".into(),
        description: String::new(),
        status: crate::IssueStatus::Open,
        priority: crate::IssuePriority::Normal,
        assignee: String::new(),
        author: "UG".into(),
        created: "2026-10-09T08:30:00Z".into(),
        labels: vec!["Clash".into()],
        elements: vec!["b-a".into(), "w-south".into()],
        clash: None,
        viewpoint: Some(IssueViewpoint { camera: camera(), section: None, isolate: Vec::new() }),
    }
}

#[test]
fn elements_have_a_class_a_storey_and_a_phase() {
    let model = model();
    assert_eq!(class_of(&model, "w-south"), Some(ElementClass::Wall));
    assert_eq!(class_of(&model, "b-a"), Some(ElementClass::Beam));
    assert_eq!(class_of(&model, "o-window"), Some(ElementClass::Window));
    assert_eq!(class_of(&model, "o-void"), None);
    assert_eq!(class_of(&model, "st-ground"), None);
    assert_eq!(storey_of(&model, "o-window").map(String::as_str), Some("st-ground"));
    assert_eq!(storey_of(&model, "b-a").map(String::as_str), Some("st-ground"));
    assert_eq!(storey_of(&model, "nothing"), None);
    assert_eq!(phase_of(&model, "w-south"), Phase::Existing);
    assert_eq!(phase_of(&model, "o-window"), Phase::Existing);
    assert_eq!(phase_of(&model, "b-a"), Phase::New);
    let ids: Vec<String> = classified(&model).into_iter().map(|(id, _)| id).collect();
    assert!(ids.contains(&"o-window".to_string()) && !ids.contains(&"o-void".to_string()));
    assert!(ids.windows(2).all(|pair| pair[0] <= pair[1]), "classified rows are in id order");
}

#[test]
fn a_selector_picks_when_every_restriction_that_is_set_holds() {
    let model = model();
    let beams = ElementSelector::of_classes(&[ElementClass::Beam]);
    assert!(beams.picks(&model, "b-a", ElementClass::Beam));
    assert!(!beams.picks(&model, "w-south", ElementClass::Wall));
    assert!(ElementSelector::all().picks(&model, "w-south", ElementClass::Wall));
    let existing = ElementSelector { phases: vec![Phase::Existing], ..ElementSelector::all() };
    assert!(existing.picks(&model, "w-south", ElementClass::Wall) && existing.picks(&model, "o-window", ElementClass::Window));
    assert!(!existing.picks(&model, "b-a", ElementClass::Beam));
    let upstairs = ElementSelector { storeys: vec!["st-first".into()], ..ElementSelector::all() };
    assert!(!upstairs.picks(&model, "w-south", ElementClass::Wall));
    let named = ElementSelector { ids: vec!["w-east".into()], ..ElementSelector::all() };
    assert!(named.picks(&model, "w-east", ElementClass::Wall) && !named.picks(&model, "w-south", ElementClass::Wall));
}

#[test]
fn a_scope_covers_what_it_names_and_a_zone_stands_on_no_storey() {
    let model = model();
    let scope = RuleScope { storeys: vec!["st-ground".into()], ..RuleScope::all() };
    assert!(scope.covers(&model, "b-a", Some(&"st-ground".to_string())));
    assert!(!scope.covers(&model, "b-a", Some(&"st-first".to_string())));
    assert!(scope.covers(&model, "zone-1", None));
    let phased = RuleScope { phases: vec![Phase::Existing], ..RuleScope::all() };
    assert!(phased.covers(&model, "w-south", None) && !phased.covers(&model, "b-a", None));
}

#[test]
fn moments_are_dates_with_an_optional_time() {
    for good in ["2026-10-09", "2026-10-09T08:30:00", "2026-10-09T08:30:00Z", "2026-10-09T08:30:00.125+02:00", "2026-10-09T23:59:60-05:30"] {
        assert!(valid_timestamp(good), "{good}");
    }
    for bad in ["", "9 October", "2026-13-01", "2026-10-09T25:00:00Z", "2026-10-09T08:30Z", "2026-10-09T08:30:00+2", "2026-10-09T08:30:00.Z", "2026-10-09 08:30:00"] {
        assert!(!valid_timestamp(bad), "{bad}");
    }
}

#[test]
fn a_clash_set_needs_a_name_sane_distances_and_selectors_that_exist() {
    let model = model();
    let set = ClashSet::standard("Beams against walls", ElementSelector::of_classes(&[ElementClass::Beam]), ElementSelector::of_classes(&[ElementClass::Wall]));
    assert_eq!(clash_set_problem(&model, "cs-1", &set), None);
    assert_eq!(set.tolerance, DEFAULT_TOLERANCE);
    let problem = |set: ClashSet| clash_set_problem(&model, "cs-1", &set).expect("a problem");
    assert_eq!(problem(ClashSet { name: " ".into(), ..set.clone() }).field, "name");
    assert_eq!(problem(ClashSet { tolerance: -0.1, ..set.clone() }).field, "tolerance");
    assert_eq!(problem(ClashSet { tolerance: f64::NAN, ..set.clone() }).field, "tolerance");
    assert_eq!(problem(ClashSet { tolerance: 2.0, ..set.clone() }).field, "tolerance");
    assert_eq!(problem(ClashSet { clearance: 11.0, ..set.clone() }).field, "clearance");
    let missing = problem(ClashSet { a: ElementSelector { storeys: vec!["st-attic".into()], ..ElementSelector::all() }, ..set.clone() });
    assert!(missing.missing && missing.field == "a");
    let missing = problem(ClashSet { b: ElementSelector { ids: vec!["w-9".into()], ..ElementSelector::all() }, ..set.clone() });
    assert!(missing.missing && missing.field == "b");
    let twice = problem(ClashSet { a: ElementSelector { classes: vec![ElementClass::Beam, ElementClass::Beam], ..ElementSelector::all() }, ..set });
    assert!(!twice.missing && twice.field == "a");
}

#[test]
fn a_rule_needs_a_name_a_limit_the_kind_accepts_and_a_scope_that_exists() {
    let model = model();
    let rule = Rule { name: "Riser".into(), kind: RuleKind::MaxRiser, limit: 0.19, severity: crate::RuleSeverity::Error, scope: RuleScope::all() };
    assert_eq!(rule_problem(&model, "r-1", &rule), None);
    let problem = |rule: Rule| rule_problem(&model, "r-1", &rule).expect("a problem");
    assert_eq!(problem(Rule { name: "".into(), ..rule.clone() }).field, "name");
    assert_eq!(problem(Rule { limit: 0.0, ..rule.clone() }).field, "limit");
    assert_eq!(problem(Rule { limit: f64::INFINITY, ..rule.clone() }).field, "limit");
    assert_eq!(problem(Rule { kind: RuleKind::MaxRampSlope, limit: 1.5, ..rule.clone() }).field, "limit");
    assert_eq!(rule_problem(&model, "r-1", &Rule { kind: RuleKind::MaxRampSlope, limit: 0.0833, ..rule.clone() }), None);
    assert!(problem(Rule { scope: RuleScope { storeys: vec!["st-attic".into()], ..RuleScope::all() }, ..rule.clone() }).missing);
    assert!(!problem(Rule { scope: RuleScope { ids: vec!["b-a".into(), "b-a".into()], ..RuleScope::all() }, ..rule }).missing);
}

#[test]
fn rule_kinds_know_their_direction_and_their_name() {
    assert!(RuleKind::MaxRiser.is_maximum() && RuleKind::MaxRampSlope.is_maximum() && RuleKind::MaxCompartmentArea.is_maximum());
    assert!(!RuleKind::MinTread.is_maximum() && !RuleKind::MinDoorWidth.is_maximum() && !RuleKind::MinClearHeight.is_maximum());
    for kind in RULE_KINDS {
        assert_eq!(RuleKind::parse(kind.name()), Some(kind));
    }
    for class in CLASSES {
        assert_eq!(ElementClass::parse(class.name()), Some(class));
    }
    assert_eq!(RuleKind::parse("nonsense"), None);
}

#[test]
fn an_issue_needs_a_title_an_author_a_moment_and_references_that_exist() {
    let model = model();
    assert_eq!(issue_problem(&model, "i-1", &issue()), None);
    let problem = |issue: Issue| issue_problem(&model, "i-1", &issue).expect("a problem");
    assert_eq!(problem(Issue { title: "".into(), ..issue() }).field, "title");
    assert_eq!(problem(Issue { author: " ".into(), ..issue() }).field, "author");
    assert_eq!(problem(Issue { created: "yesterday".into(), ..issue() }).field, "created");
    assert_eq!(problem(Issue { labels: vec!["A".into(), "A".into()], ..issue() }).field, "labels");
    assert!(problem(Issue { elements: vec!["w-9".into()], ..issue() }).missing);
    assert!(!problem(Issue { elements: vec!["b-a".into(), "b-a".into()], ..issue() }).missing);
    let clash = |set: &str, first: &str, second: &str| Issue { clash: Some(ClashRef { set: set.into(), first: first.into(), second: second.into() }), ..issue() };
    assert!(problem(clash("cs-9", "b-a", "w-south")).missing);
    let mut with_set = model.clone();
    with_set.clash_sets.insert("cs-1".into(), ClashSet::standard("Set", ElementSelector::all(), ElementSelector::all()));
    assert_eq!(issue_problem(&with_set, "i-1", &clash("cs-1", "b-a", "w-south")), None);
    assert!(issue_problem(&with_set, "i-1", &clash("cs-1", "b-a", "w-9")).expect("a problem").missing);
    assert!(!issue_problem(&with_set, "i-1", &clash("cs-1", "b-a", "b-a")).expect("a problem").missing);
}

#[test]
fn a_viewpoint_needs_a_finite_camera_a_proper_box_and_elements_that_exist() {
    let model = model();
    let with = |viewpoint: IssueViewpoint| issue_problem(&model, "i-1", &Issue { viewpoint: Some(viewpoint), ..issue() });
    let base = IssueViewpoint { camera: camera(), section: None, isolate: Vec::new() };
    assert_eq!(with(base.clone()), None);
    assert!(with(IssueViewpoint { camera: ViewCamera { distance: 0.0, ..camera() }, ..base.clone() }).is_some());
    assert!(with(IssueViewpoint { camera: ViewCamera { pitch: 2.0, ..camera() }, ..base.clone() }).is_some());
    assert!(with(IssueViewpoint { camera: ViewCamera { azimuth: f64::NAN, ..camera() }, ..base.clone() }).is_some());
    let corner = |x: f64, y: f64, z: f64| Point3 { x, y, z };
    assert_eq!(with(IssueViewpoint { section: Some(SectionBox { min: corner(0.0, 0.0, 0.0), max: corner(8.0, 6.0, 3.0) }), ..base.clone() }), None);
    assert!(with(IssueViewpoint { section: Some(SectionBox { min: corner(0.0, 0.0, 3.0), max: corner(8.0, 6.0, 3.0) }), ..base.clone() }).is_some());
    assert!(with(IssueViewpoint { isolate: vec!["w-9".into()], ..base }).expect("a problem").missing);
}

#[test]
fn a_comment_needs_its_issue_an_author_text_and_a_moment() {
    let mut model = model();
    model.issues.insert("i-1".into(), issue());
    let comment = IssueComment { issue: "i-1".into(), author: "AB".into(), date: "2026-10-09T09:00:00Z".into(), text: "On it.".into() };
    assert_eq!(comment_problem(&model, "c-1", &comment), None);
    assert!(comment_problem(&model, "c-1", &IssueComment { issue: "i-9".into(), ..comment.clone() }).expect("a problem").missing);
    assert_eq!(comment_problem(&model, "c-1", &IssueComment { author: "".into(), ..comment.clone() }).expect("a problem").field, "author");
    assert_eq!(comment_problem(&model, "c-1", &IssueComment { text: " ".into(), ..comment.clone() }).expect("a problem").field, "text");
    assert_eq!(comment_problem(&model, "c-1", &IssueComment { date: "soon".into(), ..comment }).expect("a problem").field, "date");
}

#[test]
fn comments_are_listed_in_writing_order() {
    let mut model = model();
    model.issues.insert("i-1".into(), issue());
    let write = |date: &str| IssueComment { issue: "i-1".into(), author: "AB".into(), date: date.into(), text: "Text".into() };
    model.issue_comments.insert("c-b".into(), write("2026-10-09T10:00:00Z"));
    model.issue_comments.insert("c-a".into(), write("2026-10-09T11:00:00Z"));
    model.issue_comments.insert("c-c".into(), IssueComment { issue: "i-2".into(), ..write("2026-10-08") });
    let ids: Vec<&String> = comments_of(&model, "i-1").into_iter().map(|(id, _)| id).collect();
    assert_eq!(ids, ["c-b", "c-a"]);
}

#[test]
fn an_orbit_camera_and_its_eye_and_target_describe_each_other() {
    let (eye, target) = camera().eye_and_target();
    assert_eq!(target, [4.0, 3.0, 1.5]);
    let back = ViewCamera::from_eye_and_target(eye, target);
    assert!((back.azimuth - 0.8).abs() < 1e-12 && (back.pitch - 0.5).abs() < 1e-12 && (back.distance - 25.0).abs() < 1e-9);
    assert!(eye[2] > target[2], "a positive pitch looks down on the target");
    let down = ViewCamera::from_eye_and_target([1.0, 1.0, 11.0], [1.0, 1.0, 1.0]);
    assert!((down.pitch - std::f64::consts::FRAC_PI_2).abs() < 1e-12 && down.azimuth == 0.0 && (down.distance - 10.0).abs() < 1e-12);
    assert_eq!(ViewCamera::from_eye_and_target([2.0, 2.0, 2.0], [2.0, 2.0, 2.0]).distance, 1.0);
}
