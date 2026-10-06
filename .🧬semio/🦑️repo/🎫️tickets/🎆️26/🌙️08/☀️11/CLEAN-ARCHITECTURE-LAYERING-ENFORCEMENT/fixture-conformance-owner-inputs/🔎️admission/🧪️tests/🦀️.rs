//! 🧪️ Neutral readiness corpus and all cancellation frontiers.
use super::*;

fn status(value: &serde_json::Value) -> ConformanceInputStatus {
    match value.as_str().expect("declared input status") {
        "ready" => ConformanceInputStatus::Ready,
        "missing" => ConformanceInputStatus::Missing,
        "empty" => ConformanceInputStatus::Empty,
        "stub" => ConformanceInputStatus::Stub,
        "invalid" => ConformanceInputStatus::Invalid,
        _ => panic!("undeclared status"),
    }
}
fn outcome(value: ConformanceOutcome) -> String {
    let suffix = |status| match status { ConformanceInputStatus::Ready => "ready", ConformanceInputStatus::Missing => "missing", ConformanceInputStatus::Empty => "empty", ConformanceInputStatus::Stub => "stub", ConformanceInputStatus::Invalid => "invalid" };
    match value { ConformanceOutcome::Ready => "ready".into(), ConformanceOutcome::OwnerAbsent => "owner-absent".into(), ConformanceOutcome::KindMismatch => "kind-mismatch".into(), ConformanceOutcome::Specification(status) => format!("specification-{}", suffix(status)), ConformanceOutcome::Specimen(status) => format!("specimen-{}", suffix(status)) }
}
fn inputs(fixture: &serde_json::Value) -> Vec<ConformanceInput<'_>> {
    fixture["cases"].as_array().expect("portable cases").iter().map(|row| {
        let value=&row["input"];
        ConformanceInput { id: value["id"].as_str().expect("input id"), owner_present: value["ownerPresent"].as_bool().expect("contributed presence"), facet: match value["facet"].as_str().expect("facet") { "grammar" => ConformanceFacet::Grammar, "protocol" => ConformanceFacet::Protocol, _ => panic!("undeclared facet") }, specimen: match value["specimen"].as_str().expect("specimen") { "text" => ConformanceSpecimen::Text, "binary" => ConformanceSpecimen::Binary, _ => panic!("undeclared specimen") }, specification_status: status(&value["specificationStatus"]), specimen_status: status(&value["specimenStatus"]) }
    }).collect()
}
struct Control { frontier: Option<usize>, checkpoints: usize }
impl ConformanceOperationControl for Control {
    fn checkpoint(&mut self, completed: usize, total: usize) -> Result<(), ConformanceControlRefusal> {
        assert!(completed < total);
        if self.frontier == Some(completed) { return Err(ConformanceControlRefusal::Cancelled); }
        self.checkpoints += 1;
        Ok(())
    }
}
#[test]
fn neutral_input_corpus_has_exact_refusals() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("closed neutral corpus");
    let inputs=inputs(&fixture);
    for (input,row) in inputs.iter().zip(fixture["cases"].as_array().expect("portable cases")) { assert_eq!(outcome(admit_conformance_input(input).outcome), row["expected"].as_str().expect("expected outcome"), "{}", row["id"]); }
    eprintln!("[DEBUG] Neutral conformance admission: {} cases", inputs.len());
}
#[test]
fn caller_units_and_all_cancellation_frontiers_preserve_receipts() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("closed neutral corpus");
    let inputs=inputs(&fixture);
    for frontier in 0..inputs.len() {
        let mut cursor=ConformanceCursor::new(&inputs);
        let mut control=Control { frontier: Some(frontier), checkpoints: 0 };
        let mut received=Vec::new();
        let mut result=Ok(false);
        while result==Ok(false) { result=cursor.advance(2,&mut control,&mut |value| received.push(value)); }
        assert_eq!(result,Err(ConformanceControlRefusal::Cancelled));
        assert_eq!(cursor.completed(),frontier);
        assert_eq!(control.checkpoints,frontier);
        assert_eq!(received.len(),frontier);
        for (value,input) in received.iter().zip(inputs.iter()) { assert_eq!(value.input,input); }
    }
    let mut cursor=ConformanceCursor::new(&inputs);
    let mut control=Control { frontier: None, checkpoints: 0 };
    let mut received=Vec::new();
    assert_eq!(cursor.advance(0,&mut control,&mut |value| received.push(value)),Err(ConformanceControlRefusal::Budget));
    assert_eq!(cursor.completed(),0);
    assert!(received.is_empty());
    while !cursor.complete() { cursor.advance(1,&mut control,&mut |value| received.push(value)).expect("bounded owned admission"); }
    assert_eq!(received.len(),inputs.len());
    assert_eq!(control.checkpoints,inputs.len());
    eprintln!("[DEBUG] Neutral conformance cancellation: {} frontiers", inputs.len());
}
