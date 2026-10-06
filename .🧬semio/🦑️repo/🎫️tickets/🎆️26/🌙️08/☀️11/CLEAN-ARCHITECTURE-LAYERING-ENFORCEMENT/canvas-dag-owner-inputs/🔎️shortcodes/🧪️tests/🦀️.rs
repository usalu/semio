#[test]
fn icon_shortcode_precedence_trim_and_literal_keys_match_neutral_controls() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("closed shortcode controls");
    let kinds = ["emoji", "themed", "catalog"];
    let mut owned: [Vec<(String, String)>; 3] = std::array::from_fn(|index| fixture["tables"][kinds[index]].as_object().expect("closed table").iter().map(|(key, value)| (key.clone(), value.as_str().expect("literal value").to_owned())).collect());
    for table in &mut owned { table.sort_by(|left, right| left.0.cmp(&right.0)); }
    let tables: [Vec<(&str, &str)>; 3] = std::array::from_fn(|index| owned[index].iter().map(|(key, value)| (key.as_str(), value.as_str())).collect());
    let cases = fixture["cases"].as_array().expect("closed cases");
    for case in cases {
        let matched = super::resolve_icon_shortcode(case["code"].as_str().expect("literal code"), [&tables[0], &tables[1], &tables[2]]);
        let actual = matched.map(|value| match value {
            super::IconShortcodeMatch::Emoji(text) => ("emoji", text),
            super::IconShortcodeMatch::Themed(text) => ("themed", text),
            super::IconShortcodeMatch::Catalog(text) => ("catalog", text),
        });
        let expected = if case["expected"].is_null() { None } else { Some((case["expected"]["kind"].as_str().expect("literal kind"), case["expected"]["value"].as_str().expect("literal value"))) };
        assert_eq!(actual, expected, "{}", case["id"].as_str().expect("case id"));
    }
    println!("[DEBUG] shared icon shortcode controls {}", cases.len());
}
