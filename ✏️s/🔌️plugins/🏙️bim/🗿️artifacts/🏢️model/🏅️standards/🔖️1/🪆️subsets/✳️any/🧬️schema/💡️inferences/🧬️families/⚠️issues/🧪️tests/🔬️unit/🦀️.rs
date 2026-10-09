use super::*;
use semio_framework_expression::{evaluate, parse};
use std::collections::BTreeMap;

fn issue_of(text: &str, env: &BTreeMap<String, semio_framework_expression::Value>, declared: &[&str]) -> FamilyIssue {
    let error = evaluate(&parse(text).expect("parses"), env).expect_err("fails");
    FamilyIssue::of_error(IssueOwner::Parameter, "p", "value", &error, &|name| declared.contains(&name))
}

#[test]
fn every_error_of_the_expression_crate_maps_to_a_code() {
    let env = BTreeMap::new();
    assert_eq!(issue_of("ghost + 1 m", &env, &[]).code, FamilyIssueCode::Unknown);
    assert_eq!(issue_of("ghost + 1 m", &env, &["ghost"]).code, FamilyIssueCode::Dependency);
    assert_eq!(issue_of("1 m + 30 deg", &env, &[]).code, FamilyIssueCode::Kind);
    assert_eq!(issue_of("1 / 0", &env, &[]).code, FamilyIssueCode::DivisionByZero);
    assert_eq!(issue_of("sqrt(-1)", &env, &[]).code, FamilyIssueCode::Domain);
    assert_eq!(issue_of("if 1 then 2 else 3", &env, &[]).code, FamilyIssueCode::Kind);
}

#[test]
fn the_path_of_a_fault_addresses_the_node() {
    let issue = issue_of("1 m + (2 m * ghost)", &BTreeMap::new(), &[]);
    assert_eq!(issue.path, vec![1, 1]);
    assert_eq!(issue.names, vec!["ghost".to_string()]);
    assert!(!issue.detail.is_empty());
}

#[test]
fn a_parse_fault_is_a_syntax_issue_with_the_english_fallback() {
    let error = parse("2 *").expect_err("fails");
    let issue = FamilyIssue::of_parse(IssueOwner::Solid, "s-1", "height", &error);
    assert_eq!((issue.code, issue.owner, issue.subject.as_str(), issue.field.as_str()), (FamilyIssueCode::Syntax, IssueOwner::Solid, "s-1", "height"));
    assert!(issue.path.is_empty());
}

#[test]
fn every_code_has_a_distinct_slug_and_a_message_in_both_languages() {
    let slugs: std::collections::BTreeSet<&str> = FamilyIssueCode::ALL.iter().map(|code| code.slug()).collect();
    assert_eq!(slugs.len(), FamilyIssueCode::ALL.len());
    for code in FamilyIssueCode::ALL {
        for owner in [IssueOwner::Parameter, IssueOwner::Solid, IssueOwner::Family] {
            let issue = FamilyIssue::new(code, owner, "width", "height", "detail", vec!["a".into(), "b".into()]);
            let (en, de) = (message(&issue, "en"), message(&issue, "de"));
            assert!(!en.is_empty() && !de.is_empty());
            assert_ne!(en, de, "{code:?} {owner:?}");
            assert!(en.contains("a, b") || !matches!(code, FamilyIssueCode::Cycle | FamilyIssueCode::Unknown | FamilyIssueCode::Dependency), "{en}");
        }
    }
    let issue = FamilyIssue::new(FamilyIssueCode::Cycle, IssueOwner::Parameter, "a", "value", "", vec!["a".into(), "b".into()]);
    assert_eq!(message(&issue, "en"), "parameter \"a\": circular reference between a, b");
    assert_eq!(message(&issue, "de"), "Parameter \"a\": Zirkelbezug zwischen a, b");
}
