use super::*;
use super::super::tests::kit;
use crate::FamilyCategory;

fn table() -> ModelSnapshot {
    let mut snapshot = kit::model();
    kit::family(&mut snapshot, "fam-table", FamilyCategory::Furniture);
    snapshot
}

fn find<'a>(resolution: &'a Resolution, subject: &str) -> Vec<&'a FamilyIssue> {
    resolution.issues.iter().filter(|issue| issue.subject == subject).collect()
}

#[test]
fn parameters_resolve_in_dependency_order_whatever_their_names() {
    let mut snapshot = table();
    kit::parameter(&mut snapshot, "fam-table", "width", ParameterKind::Length, "2 * base + 40 mm");
    kit::parameter(&mut snapshot, "fam-table", "base", ParameterKind::Length, "500 mm");
    kit::parameter(&mut snapshot, "fam-table", "ratio", ParameterKind::Real, "width / base");
    kit::parameter(&mut snapshot, "fam-table", "tall", ParameterKind::Boolean, "width > 1 m");
    kit::parameter(&mut snapshot, "fam-table", "turn", ParameterKind::Angle, "90 deg");
    kit::parameter(&mut snapshot, "fam-table", "label", ParameterKind::Text, "\"oak\"");
    kit::parameter(&mut snapshot, "fam-table", "legs", ParameterKind::Integer, "round(width / 100 mm)");
    let resolution = resolve(&snapshot, "fam-table", &BTreeMap::new());
    assert!(resolution.issues.is_empty(), "{:?}", resolution.issues);
    let position = |name: &str| resolution.order.iter().position(|row| row == name).expect(name);
    assert!(position("base") < position("width") && position("width") < position("ratio"));
    let get = |name: &str| resolution.parameters[name].value.clone().expect(name);
    assert_eq!(get("base"), ParameterValue::Length { value: 0.5 });
    assert_eq!(get("width"), ParameterValue::Length { value: 1.04 });
    assert_eq!(get("tall"), ParameterValue::Boolean { value: true });
    assert_eq!(get("turn"), ParameterValue::Angle { value: std::f64::consts::FRAC_PI_2 });
    assert_eq!(get("label"), ParameterValue::Text { value: "oak".into() });
    assert_eq!(get("legs"), ParameterValue::Number { value: 10.0 });
    match get("ratio") {
        ParameterValue::Number { value } => assert!((value - 2.08).abs() < 1e-9),
        other => panic!("{other:?}"),
    }
    assert_eq!(resolution.parameters["width"].formula, "2 * base + 40 mm");
}

#[test]
fn a_cycle_gives_one_issue_per_member_and_the_rest_still_resolves() {
    let mut snapshot = table();
    kit::parameter(&mut snapshot, "fam-table", "a", ParameterKind::Length, "b + 1 m");
    kit::parameter(&mut snapshot, "fam-table", "b", ParameterKind::Length, "a + 1 m");
    kit::parameter(&mut snapshot, "fam-table", "c", ParameterKind::Length, "a");
    kit::parameter(&mut snapshot, "fam-table", "d", ParameterKind::Length, "2 m");
    let resolution = resolve(&snapshot, "fam-table", &BTreeMap::new());
    for member in ["a", "b"] {
        let issues = find(&resolution, member);
        assert_eq!(issues.len(), 1, "{member}");
        assert_eq!(issues[0].code, FamilyIssueCode::Cycle);
        assert_eq!(issues[0].names, vec!["a".to_string(), "b".to_string()]);
        assert!(resolution.parameters[member].value.is_none());
    }
    assert_eq!(find(&resolution, "c")[0].code, FamilyIssueCode::Dependency);
    assert_eq!(resolution.parameters["d"].value, Some(ParameterValue::Length { value: 2.0 }));
}

#[test]
fn unknown_syntax_zero_and_kind_faults_are_issues_not_values() {
    let mut snapshot = table();
    kit::parameter(&mut snapshot, "fam-table", "ghosted", ParameterKind::Length, "ghost + 1 m");
    kit::parameter(&mut snapshot, "fam-table", "broken", ParameterKind::Length, "2 *");
    kit::parameter(&mut snapshot, "fam-table", "after", ParameterKind::Length, "broken + 1 m");
    kit::parameter(&mut snapshot, "fam-table", "zero", ParameterKind::Real, "1 / 0");
    kit::parameter(&mut snapshot, "fam-table", "mixed", ParameterKind::Length, "1 m + 30 deg");
    kit::parameter(&mut snapshot, "fam-table", "wrong", ParameterKind::Length, "2");
    let resolution = resolve(&snapshot, "fam-table", &BTreeMap::new());
    let code = |subject: &str| find(&resolution, subject).first().map(|issue| issue.code);
    assert_eq!(code("ghosted"), Some(FamilyIssueCode::Unknown));
    assert_eq!(find(&resolution, "ghosted")[0].names, vec!["ghost".to_string()]);
    assert_eq!(code("broken"), Some(FamilyIssueCode::Syntax));
    assert_eq!(code("after"), Some(FamilyIssueCode::Dependency));
    assert_eq!(find(&resolution, "after")[0].names, vec!["broken".to_string()]);
    assert_eq!(code("zero"), Some(FamilyIssueCode::DivisionByZero));
    assert_eq!(code("mixed"), Some(FamilyIssueCode::Kind));
    assert_eq!(code("wrong"), Some(FamilyIssueCode::Kind));
    assert!(resolution.parameters.values().all(|parameter| parameter.value.is_none()));
}

#[test]
fn integer_and_material_parameters_check_their_value() {
    let mut snapshot = table();
    kit::parameter(&mut snapshot, "fam-table", "legs", ParameterKind::Integer, "2.5");
    kit::parameter(&mut snapshot, "fam-table", "top", ParameterKind::Material, "\"m-ghost\"");
    kit::parameter(&mut snapshot, "fam-table", "frame", ParameterKind::Material, "\"m-steel\"");
    let resolution = resolve(&snapshot, "fam-table", &BTreeMap::new());
    assert_eq!(find(&resolution, "legs")[0].code, FamilyIssueCode::Kind);
    assert_eq!(find(&resolution, "top")[0].code, FamilyIssueCode::Unknown);
    assert_eq!(find(&resolution, "top")[0].names, vec!["m-ghost".to_string()]);
    assert!(find(&resolution, "frame").is_empty());
    assert!(resolution.parameters["frame"].value.is_some());
}

#[test]
fn overrides_replace_formulas_and_a_broken_override_is_a_syntax_issue_of_that_name() {
    let mut snapshot = table();
    kit::parameter(&mut snapshot, "fam-table", "base", ParameterKind::Length, "500 mm");
    kit::parameter(&mut snapshot, "fam-table", "width", ParameterKind::Length, "2 * base");
    let replaced = resolve(&snapshot, "fam-table", &BTreeMap::from([("base".to_string(), "0.6m".to_string())]));
    assert!(replaced.issues.is_empty());
    assert_eq!(replaced.parameters["base"].formula, "0.6 m");
    assert_eq!(replaced.parameters["width"].value, Some(ParameterValue::Length { value: 1.2 }));
    let broken = resolve(&snapshot, "fam-table", &BTreeMap::from([("base".to_string(), "(".to_string())]));
    assert_eq!(find(&broken, "base")[0].code, FamilyIssueCode::Syntax);
    assert_eq!(find(&broken, "width")[0].code, FamilyIssueCode::Dependency);
}

#[test]
fn only_the_parameters_of_the_family_count() {
    let mut snapshot = table();
    kit::family(&mut snapshot, "fam-other", FamilyCategory::Generic);
    kit::parameter(&mut snapshot, "fam-table", "width", ParameterKind::Length, "1 m");
    kit::parameter(&mut snapshot, "fam-other", "width", ParameterKind::Length, "9 m");
    kit::parameter(&mut snapshot, "fam-other", "extra", ParameterKind::Length, "width");
    assert_eq!(authored(&snapshot, "fam-table").len(), 1);
    assert_eq!(resolve(&snapshot, "fam-table", &BTreeMap::new()).parameters.len(), 1);
    assert_eq!(resolve(&snapshot, "fam-other", &BTreeMap::new()).parameters["extra"].value, Some(ParameterValue::Length { value: 9.0 }));
}

#[test]
fn areas_and_volumes_have_no_stored_form() {
    assert_eq!(stored(&Value::Area(2.0)), None);
    assert_eq!(stored(&Value::Volume(2.0)), None);
    assert_eq!(stored(&Value::Length(2.0)), Some(ParameterValue::Length { value: 2.0 }));
}
