//! 🧱️ Source_builders_quick of the capabilities crate: Gateway capability declarations and the core catalog.

#[cfg(any(test, feature = "testkit"))]
use crate::source_builders::{cad_actions, eval_cases, note_actions};

/// 🧪️ Reads a committed fixture from the package directory at run time instead of embedding it in the test binary.
fn fixture_text(relative: &str) -> &'static str {
    Box::leak(std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)).expect(relative).into_boxed_str())
}


#[test]
fn cad_descriptor_declares_forty_non_framework_actions() {
    assert_eq!(cad_actions().len(), 40);
}

#[test]
fn note_descriptor_declares_thirty_five_non_framework_actions() {
    assert_eq!(note_actions().len(), 35);
}

#[test]
fn note_actions_declare_zero_manifest_args_per_d2() {
    assert!(note_actions().iter().all(|action| action.args.is_empty()));
}

#[test]
fn eval_cases_has_at_least_sixty_entries_in_both_locales() {
    let cases = eval_cases(fixture_text("../../../🧪️tests/🧫️fixtures/🧠️conformance/🔣️.json"));
    assert!(cases.len() >= 60, "expected >= 60 eval cases, got {}", cases.len());
    assert!(cases.iter().any(|case| case.locale == "en"));
    assert!(cases.iter().any(|case| case.locale == "de"));
}
