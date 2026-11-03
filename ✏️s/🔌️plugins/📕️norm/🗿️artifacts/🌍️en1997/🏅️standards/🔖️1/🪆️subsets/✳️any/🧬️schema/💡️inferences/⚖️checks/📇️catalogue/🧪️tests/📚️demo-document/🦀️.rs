use super::*;
use semio_s_plugin_norm_compliance::design_case::DesignCase;

/// 📚️ Every catalogue entry is document-reachable: the full-catalogue demo document holds one case per entry and each case evaluates to results addressed to its own row.
#[test]
fn every_catalogue_entry_evaluates_from_the_demo_document() {
    let rows = catalogue_demo_cases();
    let entries = <En1997Cases as semio_s_plugin_norm_compliance::design_case::CaseCatalogue>::entries();
    assert_eq!(rows.len(), entries.len());
    let mut ids: Vec<&str> = rows.iter().map(|row| row.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), rows.len(), "case ids are unique");
    for (row, entry) in rows.iter().zip(&entries) {
        assert_eq!(row.entry_id, entry.id);
        let results = evaluate_design_case(&DesignCase::from(row), AnnexChoice::De);
        assert!(!results.is_empty(), "entry {} produced no result", entry.id);
        let prefix = format!("cases[id={}]", row.id);
        assert!(results.iter().all(|result| result.subject.path.starts_with(&prefix)), "entry {} results are addressed to {prefix}", entry.id);
    }
}

/// 📚️ The committed JSON demo document equals the seeded cases and covers the whole registry (the same registry-coverage test as the other codes).
#[test]
fn the_committed_demo_document_equals_the_seeded_cases_and_covers_the_registry() {
    let fixture = include_str!("../../../../../../🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json");
    let rows = catalogue_demo_cases();
    assert_eq!(fixture.matches("\"entryId\":").count(), rows.len(), "the demo document has a different number of cases than the seeded demo");
    for row in &rows {
        assert!(fixture.contains(&format!("\"id\": \"{}\"", row.id)) && fixture.contains(&format!("\"entryId\": \"{}\"", row.entry_id)), "{} is missing from the demo document", row.id);
    }
    let rows_with_values: usize = rows.iter().map(|row| row.numbers.len() + row.texts.len()).sum();
    assert_eq!(fixture.matches("\"value\":").count(), rows_with_values, "the demo document values drifted from the seeded demo");
    for entry in registry() {
        assert!(rows.iter().any(|row| row.entry_id == entry.id), "{} has no case in the demo document", entry.id);
    }
}

#[test]
fn every_demo_case_evaluates_its_entry_instead_of_reporting_missing_or_invalid_inputs() {
    let silent: Vec<String> = catalogue_demo_cases()
        .iter()
        .filter(|row| {
            let results = evaluate_design_case(&design_case::DesignCase::from(*row), AnnexChoice::De);
            results.iter().all(|result| result.status == semio_s_plugin_norm_compliance::CheckStatus::NotApplicable) || results.iter().any(|result| result.id.contains(".input."))
        })
        .map(|row| row.entry_id.clone())
        .collect();
    assert!(silent.is_empty(), "entries whose demo case only reports not-applicable or invalid inputs: {silent:?}");
}
