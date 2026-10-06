#[test]
fn canvas_color_input_matches_owned_words_and_original_rgba8_contract() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../🎨️color/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("closed color input fixture");
    for row in fixture["cases"].as_array().expect("color cases") {
        let input: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(row["json"].as_str().expect("source"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned literal input");
        let actual = input.as_array().and_then(super::color_from_value_rgba8).map(|color| { let value = color.to_rgba8(); [value.r, value.g, value.b, value.a] });
        let expected = row["expected"].as_array().map(|values| std::array::from_fn(|index| values[index].as_u64().expect("color byte") as u8));
        assert_eq!(actual, expected, "{}", row["id"].as_str().expect("case identity"));
    }
    eprintln!("[DEBUG] canonical owned color words 8");
}
