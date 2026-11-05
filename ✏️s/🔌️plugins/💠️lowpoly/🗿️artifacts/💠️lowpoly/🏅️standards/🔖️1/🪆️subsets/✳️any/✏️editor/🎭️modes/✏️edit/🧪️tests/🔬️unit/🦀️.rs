use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_model_window_only() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(model::LOWPOLY_PLAY_WINDOW_MAIN));
}

#[semio_framework_async_macros::async_test]
async fn localized_engagement_and_failure_labels_follow_neutral_cases() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("locales.json")).expect("locale contract");
    assert_eq!(crate::editor::lowpoly::modes::paint::layout().label, contract["sharedPresetLabels"]["paint"].as_str().expect("shared bilingual name"));
    for case in contract["cases"].as_array().expect("cases") {
        let labels = match case["locale"].as_str().expect("explicit locale") {
            "en" => &LowpolyLabels::NATIVE_EN,
            "de" => &LowpolyLabels::NATIVE_DE,
            other => panic!("unsupported contract locale {other}"),
        };
        let snapshot = semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot::default();
        let config = LowpolyConfig::default();
        let select = semio_s_artifact_lowpoly_lowpoly_options::select::SelectState { granularity: case["granularity"].as_str().unwrap().into(), ..Default::default() };
        let engagement = lowpoly_window_engagement_with_selection(LowpolyView { snapshot: &snapshot, config: &config }, case["utility"].as_str().unwrap(), labels, &select, case["count"].as_u64().unwrap() as usize);
        let wire = serde_json::to_value(&engagement).expect("independent engagement serializer");
        let observed = serde_json::json!({
            "status": wire["status"][0]["text"],
            "placeholder": wire["input"]["placeholder"],
            "failedModel": labels.failed_model.as_str(),
            "failedUv": labels.failed_uv.as_str()
        });
        assert_eq!(observed, case["expect"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn built_in_layouts_leave_window_titles_to_localized_window_definitions() {
    let layouts = serde_json::json!([layout(), crate::editor::lowpoly::modes::paint::layout().layout]);
    fn check(value: &serde_json::Value) {
        match value {
            serde_json::Value::Object(fields) => {
                if fields.get("kind").and_then(serde_json::Value::as_str) == Some("window") {
                    assert!(fields.get("title").is_none(), "locale-specific window title must resolve from its window kind");
                }
                for child in fields.values() { check(child); }
            }
            serde_json::Value::Array(items) => { for child in items { check(child); } }
            _ => {}
        }
    }
    check(&layouts);
}
