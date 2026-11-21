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
            let annex = if matches!(row.entry_id.as_str(), "E12.02" | "E12.04") { AnnexChoice::En } else { AnnexChoice::De };
            let results = evaluate_design_case(&design_case::DesignCase::from(*row), annex);
            results.iter().all(|result| result.status == semio_s_plugin_norm_compliance::CheckStatus::NotApplicable) || results.iter().any(|result| result.id.contains(".input."))
        })
        .map(|row| row.entry_id.clone())
        .collect();
    assert!(silent.is_empty(), "entries whose demo case only reports not-applicable or invalid inputs: {silent:?}");
}

#[test]
fn the_vector_demo_values_reach_the_evaluator_input() {
    let rows = catalogue_demo_cases();
    let row = rows.iter().find(|row| row.entry_id == "E12.10").expect("E12.10 demo case");
    let input = case_input(AnnexChoice::De, &design_case::DesignCase::from(row));
    assert_eq!(input.vec.get("times_min").map(Vec::len), Some(5));
}

#[test]
fn annex_bb_reliability_agrees_with_the_verified_model() {
    use crate::annex_bb_reliability::{annual_failure_target, brigade_failure, ignition_model};

    for line in include_str!("annex-bb-probability.tsv").lines().skip(1) {
        let [occupancy, a, b, mean] = line.split('\t').collect::<Vec<_>>().try_into().unwrap();
        assert_eq!(ignition_model(occupancy), Some((a.parse().unwrap(), b.parse().unwrap(), mean.parse().unwrap())));
    }
    assert_eq!(brigade_failure("public", 17.5, 0.0), Some(0.35));
    assert!(brigade_failure("onsite_two", 10.0, 0.0).is_none());
    assert!(annual_failure_target("agricultural", "high", 0.0).is_none());

    let row = catalogue_demo_cases().into_iter().find(|row| row.entry_id == "E12.03.1").unwrap();
    let case = DesignCase::from(&row);
    let results = evaluate_design_case(&case, AnnexChoice::De);
    for (key, expected) in [
        ("p1", 0.0062), ("p2", 0.1), ("pfi", 0.00062), ("conditional_failure", 0.020_967_741_935_483_87),
        ("beta", 2.034_159_823_526_682_7), ("gamma_classified", 0.981_213_421_815_356_6),
        ("gamma_surveyed", 0.986_179_419_192_399_9),
    ] {
        let result = results.iter().find(|check| check.id == format!("en1991.e12.03.1.{key}.{}", row.id)).expect(key);
        assert!((result.computed.value - expected).abs() < 1e-10, "{key}: {}", result.computed.value);
    }
    assert!(evaluate_design_case(&case, AnnexChoice::En).iter().all(|check| check.status == semio_s_plugin_norm_compliance::CheckStatus::NotApplicable));
}
