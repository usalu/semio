use super::*;

#[test]
fn shooting_shot_field_values_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔢️field-values.json")).expect("neutral input vectors");
    let base = crate::standards::v1::subsets::any::schema::default_snapshot();
    let id = base.shots[0].id.clone();
    for vector in vectors["cases"].as_array().expect("cases") {
        let field = vector["field"].as_str().expect("field");
        let value = Value::String(vector["value"].as_str().expect("input string").into());
        let mutation = shot_mutation_for_field(id.clone(), field, &value);
        if vector["expected"].is_null() {
            assert!(mutation.is_none(), "invalid dimension must reject without truncation");
        } else {
            let (next, _) = store::apply_mutation(&base, &mutation.expect("valid input")).expect("apply shot field");
            let json: serde_json::Value = serde_json::from_str(&dsl::os_pack::to_json_string(&next)).expect("independent snapshot oracle");
            assert_eq!(json["shots"][0][field], vector["expected"]);
        }
    }
}
