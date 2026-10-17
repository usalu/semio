//! 🧱️ Conformance_quick of the capabilities crate: Gateway capability declarations and the core catalog.

use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::Terminology;
#[cfg(any(test, feature = "testkit"))]
use crate::source_builders;
use semio_framework_os_mcp_catalog::catalog::{CapabilityKind, CapabilityOwner, CapabilityRef, Catalog, compile};
use semio_framework_os_mcp_catalog::conformance::{check, check_bilingual_labels, run_eval};

/// 🧪️ Reads a committed fixture from the package directory at run time instead of embedding it in the test binary.
fn fixture_text(relative: &str) -> &'static str {
    Box::leak(std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)).expect(relative).into_boxed_str())
}


fn compiled() -> Catalog {
    compile(&source_builders::note_and_cad_source(), Locale::En, Terminology::Native).expect("compiles")
}

#[test]
fn note_and_cad_fixtures_produce_zero_conformance_findings() {
    let catalog = compiled();
    let findings = check(&catalog);
    assert!(findings.is_empty(), "unexpected findings: {findings:?}");
}

#[test]
fn note_and_cad_fixtures_have_non_empty_bilingual_labels() {
    let findings = check_bilingual_labels(&source_builders::note_and_cad_source());
    assert!(findings.is_empty(), "unexpected bilingual findings: {findings:?}");
}

#[test]
fn mutation_without_writes_is_flagged() {
    let mut catalog = compiled();
    let mut capability = catalog.entries[0].clone();
    capability.kind = CapabilityKind::Mutation;
    capability.effects.writes.clear();
    catalog.entries[0] = capability;
    let findings = check(&catalog);
    assert!(findings.iter().any(|finding| finding.message.contains("declares no writes")));
}

#[test]
fn unknown_scope_is_flagged() {
    let mut catalog = compiled();
    let mut capability = catalog.entries[0].clone();
    capability.policy.scopes.push(semio_framework::manifest::kernel::CapabilityId("not.a.real.scope".to_string()));
    catalog.entries[0] = capability;
    let findings = check(&catalog);
    assert!(findings.iter().any(|finding| finding.message.contains("unknown policy scope")));
}

#[test]
fn bare_action_id_grammar_violation_is_flagged() {
    let mut catalog = compiled();
    let index = catalog.entries.iter().position(|entry| matches!(entry.owner, CapabilityOwner::Plugin { .. })).expect("the note+cad fixture always compiles plugin-owned capabilities");
    let mut capability = catalog.entries[index].clone();
    capability.id = CapabilityRef("bareActionId".to_string());
    catalog.entries[index] = capability;
    let findings = check(&catalog);
    assert!(findings.iter().any(|finding| finding.message.contains("must start with")));
}

#[test]
fn eval_harness_measures_top1_and_top3_accuracy_deterministically() {
    let catalog = compiled();
    let cases = source_builders::eval_cases(fixture_text("../../../🧪️tests/🧫️fixtures/🧠️conformance/🔣️.json"));
    let first = run_eval(&catalog, &cases);
    let second = run_eval(&catalog, &cases);
    assert_eq!(first, second, "eval must be fully deterministic");
    assert!(first.total >= 60);
    assert!(first.top1_accuracy() >= 0.0 && first.top1_accuracy() <= 1.0);
    assert!(first.top3_accuracy() >= first.top1_accuracy());
}
