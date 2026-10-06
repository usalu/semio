use super::*;
use serde_json::{json, Value};
fn source(rows: &Value) -> Vec<MutationInventoryLeaf<'_>> {
    rows.as_array().unwrap().iter().map(|row| {
        let classes = row["outcomes"].as_array().unwrap();
        let mut outcomes = [MutationInventoryOutcome::Applied; 5];
        for (index, class) in classes.iter().take(5).enumerate() {
            outcomes[index] = match class.as_str().unwrap() {
                "applied" => MutationInventoryOutcome::Applied, "no-op" => MutationInventoryOutcome::NoOp,
                "empty" => MutationInventoryOutcome::Empty, "disjoint" => MutationInventoryOutcome::Disjoint,
                "rejected" => MutationInventoryOutcome::Rejected, _ => panic!("unknown fixture outcome"),
            };
        }
        MutationInventoryLeaf { owner: row["owner"].as_str().unwrap(), id: row["id"].as_str().unwrap(), variant: row["variant"].as_str().unwrap(), outcomes, outcome_count: classes.len() }
    }).collect()
}
fn corpus() -> Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn coordinate(row: &Value) -> MutationInventoryCoordinate<'_> {
    MutationInventoryCoordinate { artifact: row["artifact"].as_str().unwrap(), standard: row["standard"].as_str().unwrap(), subset: row["subset"].as_str().unwrap(), surface: row["surface"].as_str(), owner: row["owner"].as_str().unwrap() }
}
#[test]
fn closed_corpus_matches_independent_serde_wire_values() {
    let fixture = corpus();
    for case in fixture["cases"].as_array().unwrap() {
        let sources = case["sources"].as_array().unwrap().iter().map(source).collect::<Vec<_>>();
        let sources = sources.iter().map(Vec::as_slice).collect::<Vec<_>>();
        let excluded = case["coordinate"]["excludedOwnerSegments"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>();
        let mut progress = Vec::new();
        let mut observe = |row: MutationInventoryProgress| { progress.push(row); Ok(()) };
        let output = encode_mutation_inventory(&coordinate(&case["coordinate"]), "sample-inventory", &sources, &excluded, MutationInventoryBudget { maximum_units: 1_000_000, maximum_owned_bytes: 65_536 }, &mut observe).unwrap();
        let oracle: Value = serde_json::from_slice(&output).unwrap();
        eprintln!("[DEBUG] Native inventory wire {}", json!({"name":case["name"],"output":oracle}));
        assert_eq!(oracle, case["expected"], "{}", case["name"]);
        assert_eq!(serde_json::from_slice::<Value>(&serde_json::to_vec(&oracle).unwrap()).unwrap(), oracle);
        assert!(progress.len() > output.len());
        assert!(progress.windows(2).all(|rows| rows[0].completed_units <= rows[1].completed_units && rows[0].emitted_bytes <= rows[1].emitted_bytes));
        assert_eq!(progress[0].owned_bytes, 0);
        assert!(progress.iter().all(|row| row.owned_bytes <= 65_536));
        assert_eq!(progress.last().unwrap().emitted_bytes, output.len());
    }
    eprintln!("[DEBUG] Eight native closed inventory cases match independent Serde values");
}
#[test]
fn work_bytes_and_cancellation_refuse_before_returning_output() {
    let fixture = corpus();
    let coordinate = MutationInventoryCoordinate { artifact: "sample.document", standard: "1", subset: "any", surface: None, owner: "domain/document" };
    for row in fixture["controls"].as_array().unwrap() {
        let limit = row["cancelAtCallback"].as_u64();
        let mut callbacks = 0;
        let mut first_owned = None;
        let mut observe = |progress: MutationInventoryProgress| {
            callbacks += 1;
            first_owned.get_or_insert(progress.owned_bytes);
            if limit == Some(callbacks) { Err(MutationInventoryError::Cancelled) } else { Ok(()) }
        };
        let result = encode_mutation_inventory(&coordinate, "sample-inventory", &[], &[], MutationInventoryBudget { maximum_units: row["maximumUnits"].as_u64().unwrap(), maximum_owned_bytes: row["maximumOwnedBytes"].as_u64().unwrap().try_into().unwrap() }, &mut observe);
        let actual = match result { Ok(_) => "Admitted".to_string(), Err(error) => format!("{error:?}") };
        assert_eq!(actual, row["expected"].as_str().unwrap(), "{}", row["name"]);
        if let Some(limit) = limit { assert_eq!(callbacks, limit); assert_eq!(first_owned, Some(0)); }
        eprintln!("[DEBUG] Native inventory control {}", json!({"name":row["name"],"actual":actual,"callbacks":callbacks,"firstOwned":first_owned}));
    }
    let mut early = |row: MutationInventoryProgress| { assert_eq!(row.owned_bytes, 0); Err(MutationInventoryError::Cancelled) };
    let huge = usize::try_from(9_007_199_254_740_991_u64).unwrap_or(usize::MAX);
    assert_eq!(encode_mutation_inventory(&coordinate, "sample-inventory", &[], &[], MutationInventoryBudget { maximum_units: 1_000_000, maximum_owned_bytes: huge }, &mut early).unwrap_err(), MutationInventoryError::Cancelled);
}
#[test]
fn exact_byte_and_work_frontiers_are_admitted_and_one_less_is_refused() {
    let coordinate = MutationInventoryCoordinate { artifact: "sample.document", standard: "1", subset: "any", surface: None, owner: "domain/document" };
    let mut terminal_units = 0;
    let mut observe = |row: MutationInventoryProgress| { terminal_units = row.completed_units; Ok(()) };
    let output = encode_mutation_inventory(&coordinate, "sample-inventory", &[], &[], MutationInventoryBudget { maximum_units: 1_000_000, maximum_owned_bytes: 65_536 }, &mut observe).unwrap();
    eprintln!("[DEBUG] Native inventory frontier {}", json!({"units":terminal_units,"bytes":output.len(),"callbacks":terminal_units+3}));
    let budget = MutationInventoryBudget { maximum_units: terminal_units, maximum_owned_bytes: output.len() };
    let mut observe = |_: MutationInventoryProgress| Ok(());
    assert_eq!(encode_mutation_inventory(&coordinate, "sample-inventory", &[], &[], budget, &mut observe).unwrap(), output);
    assert_eq!(encode_mutation_inventory(&coordinate, "sample-inventory", &[], &[], MutationInventoryBudget { maximum_units: terminal_units - 1, ..budget }, &mut observe).unwrap_err(), MutationInventoryError::WorkBudget);
    assert_eq!(encode_mutation_inventory(&coordinate, "sample-inventory", &[], &[], MutationInventoryBudget { maximum_owned_bytes: output.len() - 1, ..budget }, &mut observe).unwrap_err(), MutationInventoryError::ByteBudget);
}
#[test]
fn malformed_coordinates_selected_leafs_and_conflicting_duplicates_are_refused() {
    let coordinate = MutationInventoryCoordinate { artifact: "sample..document", standard: "1", subset: "any", surface: None, owner: "domain/document" };
    let mut observe = |_: MutationInventoryProgress| Ok(());
    let budget = MutationInventoryBudget { maximum_units: 1_000_000, maximum_owned_bytes: 65_536 };
    assert_eq!(encode_mutation_inventory(&coordinate, "sample-inventory", &[], &[], budget, &mut observe).unwrap_err(), MutationInventoryError::Coordinate);
    let coordinate = MutationInventoryCoordinate { artifact: "sample.document", ..coordinate };
    for outcomes in [json!([]), json!(["applied", "applied"]), json!(["applied", "no-op", "empty", "disjoint", "rejected", "applied"])] {
        let invalid = json!([{"owner":"domain/document/create","id":"create","variant":"Create","outcomes":outcomes}]);
        let rows = source(&invalid);
        assert_eq!(encode_mutation_inventory(&coordinate, "sample-inventory", &[&rows], &[], budget, &mut observe).unwrap_err(), MutationInventoryError::Descriptor);
    }
    for owner in ["domain/document/", "domain/document//create", "domain/document/../create"] {
        let invalid = json!([{"owner":owner,"id":"create","variant":"Create","outcomes":["applied"]}]);
        let rows = source(&invalid);
        assert_eq!(encode_mutation_inventory(&coordinate, "sample-inventory", &[&rows], &[], budget, &mut observe).unwrap_err(), MutationInventoryError::Descriptor);
    }
    let conflict = json!([{"owner":"domain/document/create","id":"create","variant":"First","outcomes":["applied"]},{"owner":"domain/document/other","id":"create","variant":"Other","outcomes":["no-op"]}]);
    let rows = source(&conflict);
    assert_eq!(encode_mutation_inventory(&coordinate, "sample-inventory", &[&rows], &[], budget, &mut observe).unwrap_err(), MutationInventoryError::Descriptor);
    for excluded in [&["edit", "edit"][..], &["edit/window"][..], &[""][..]] {
        assert_eq!(encode_mutation_inventory(&coordinate, "sample-inventory", &[], excluded, budget, &mut observe).unwrap_err(), MutationInventoryError::Coordinate);
    }
}
#[test]
fn command_refuses_a_withdrawn_contribution_ambiguity_and_missing_control_policy() {
    let fixture = corpus();
    let coordinate = MutationInventoryCoordinate { artifact: "sample.document", standard: "1", subset: "any", surface: None, owner: "domain/document" };
    for row in fixture["commands"].as_array().unwrap() {
        let args = row["arguments"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect::<Vec<_>>();
        let contributions = (0..row["contributions"].as_u64().unwrap()).map(|_| MutationInventoryContribution { coordinate, aggregates: &[&[]], excluded_owner_segments: &[] }).collect::<Vec<_>>();
        let mut observations = Vec::new();
        let mut observe = |progress: MutationInventoryProgress| { observations.push(progress); Ok(()) };
        let result = inventory_for_command("sample-inventory", &contributions, &args, &mut observe);
        let actual = match result { Ok(output) => { assert_eq!(serde_json::from_slice::<Value>(&output).unwrap()["mutations"], json!([])); "Admitted".to_string() }, Err(error) => format!("{error:?}") };
        assert_eq!(actual, row["expected"].as_str().unwrap());
        assert!(observations.windows(2).all(|rows| rows[0].completed_units <= rows[1].completed_units));
        eprintln!("[DEBUG] Native inventory command {}", json!({"name":row["name"],"actual":actual}));
    }
}
