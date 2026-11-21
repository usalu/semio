use super::*;

#[test]
fn every_catalogue_entry_evaluates_from_the_demo_document() {
    let rows = catalogue_demo_cases();
    assert_eq!(rows.len(), En1990Cases::entries().len());
    for row in &rows {
        let results = evaluate_design_case(&DesignCase::from(row), AnnexChoice::De);
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
            let results = evaluate_design_case(&DesignCase::from(*row), AnnexChoice::De);
            results.iter().all(|result| result.status == CheckStatus::NotApplicable) || results.iter().any(|result| result.id.contains(".input."))
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
    for entry in En1990Cases::entries() {
        assert!(rows.iter().any(|row| row.entry_id == entry.id), "{} has no case in the demo document", entry.id);
    }
}

#[test]
fn qualified_characteristic_value_bases() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🧪️tests/🧫️fixtures/📇️catalogue/characteristic-actions.json")).unwrap();
    let entry = design_case::entry::<En1990Cases>("E90.08").unwrap();
    for input in fixture.as_array().unwrap() {
        let id = input["id"].as_str().unwrap();
        let mut case = design_case::seed::<En1990Cases>(&entry, id);
        case.values.numbers.clear();
        for (key, value) in input["num"].as_object().unwrap() {
            case.values.numbers.insert(key.clone(), value.as_f64().unwrap());
        }
        for (key, value) in input["text"].as_object().unwrap() {
            case.values.texts.insert(key.clone(), value.as_str().unwrap().into());
        }
        let results = evaluate_design_case(&case, AnnexChoice::En);
        if input["valid"] == true {
            assert!(!results.is_empty(), "{id}");
            assert!(results.iter().all(|row| row.status == CheckStatus::Pass && row.limit.kind == QuantityKind::Force), "{id}");
        } else {
            assert_eq!(results.len(), 1, "{id}");
            assert_eq!(results[0].status, CheckStatus::NotApplicable, "{id}");
        }
    }
}
