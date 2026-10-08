/// 🔎️ Compares the current typed catalog JSON with neutral rows and the independent serde_json tree.
pub(super) fn verify(catalog_id: &str, json: &str) {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected = cases.as_array().unwrap().iter().find(|row| row["catalogId"] == catalog_id).unwrap();
    let independent: serde_json::Value = serde_json::from_str(json).unwrap();
    let rows = independent.as_array().unwrap();
    let ids: Vec<&str> = rows.iter().map(|row| row["id"].as_str().unwrap()).collect();
    let counts: Vec<usize> = rows.iter().map(|row| row["capabilities"].as_array().unwrap().len()).collect();
    assert_eq!(serde_json::to_value(&ids).unwrap(), expected["machineIds"]);
    assert_eq!(serde_json::to_value(&counts).unwrap(), expected["capabilityCounts"]);
    let owned = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let first_party: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&owned)).unwrap();
    assert_eq!(first_party, independent);
    eprintln!("[DEBUG] {catalog_id} catalog JSON matches neutral machine order/counts and serde_json exact tree: {} machines", rows.len());
}
