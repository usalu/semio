use super::*;

#[test]
fn every_catalogue_entry_evaluates_from_the_demo_document() {
    let rows = catalogue_demo_cases();
    assert_eq!(rows.len(), En1991Cases::entries().len());
    for row in &rows {
        let results = evaluate_design_case(&design_case::DesignCase::from(row), AnnexChoice::De);
        assert!(!results.is_empty(), "entry {} yields no result", row.entry_id);
        let prefix = format!("cases[id={}]", row.id);
        assert!(results.iter().all(|result| result.subject.path.starts_with(&prefix)), "entry {} is not addressed to its case row", row.entry_id);
    }
}

#[test]
fn demo_case_ids_are_unique_and_addressable() {
    let rows = catalogue_demo_cases();
    let ids: std::collections::BTreeSet<&str> = rows.iter().map(|row| row.id.as_str()).collect();
    assert_eq!(ids.len(), rows.len());
    assert!(ids.iter().all(|id| !id.contains(['.', ']', '='])));
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

#[test]
fn the_committed_demo_document_equals_the_seeded_cases_and_covers_the_registry() {
    let fixture = include_str!("../../../../../../../🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json");
    let rows = catalogue_demo_cases();
    assert_eq!(fixture.matches("\"entryId\":").count(), rows.len(), "the demo document has a different number of cases than the seeded demo");
    for row in &rows {
        assert!(fixture.contains(&format!("\"id\": \"{}\"", row.id)) && fixture.contains(&format!("\"entryId\": \"{}\"", row.entry_id)), "{} is missing from the demo document", row.id);
    }
    let rows_with_values: usize = rows.iter().map(|row| row.numbers.len() + row.texts.len()).sum();
    assert_eq!(fixture.matches("\"value\":").count(), rows_with_values, "the demo document values drifted from the seeded demo");
    for entry in En1991Cases::entries() {
        assert!(rows.iter().any(|row| row.entry_id == entry.id), "{} has no case in the demo document", entry.id);
    }
}

#[test]
fn the_vector_demo_values_reach_the_evaluator_input() {
    let rows = catalogue_demo_cases();
    let row = rows.iter().find(|row| row.entry_id == "E12.10").expect("E12.10 demo case");
    let input = case_input(AnnexChoice::De, &design_case::DesignCase::from(row));
    assert_eq!(input.vec.get("times_min").map(Vec::len), Some(5));
}
