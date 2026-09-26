use super::*;
use crate::editor::din4108::unit_tests::context;
use crate::field_meta::din4108_field_meta;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_INPUTS);
    assert_eq!(definition().id, WINDOW_INPUTS);
}

#[semio_framework_async_macros::async_test]
async fn renders_structured_document_editor() {
    let mut app = context::app_with_registry().await;
    let body = context::render(&mut app, BODY_INPUTS).await;
    assert!(!body.is_empty(), "structured inputs body renders");
    let climate = din4108_field_meta("climateZone").expect("climateZone meta");
    assert!(!climate.label_en.is_empty() && !climate.label_de.is_empty());
    assert!(climate.choices.is_some_and(|c| c.iter().any(|x| !x.label_en.is_empty() && !x.label_de.is_empty())));
    let bb2 = din4108_field_meta("thermalBridges[].bb2Type").expect("bb2Type meta");
    let bb2_choices = bb2.choices.expect("bb2 choices");
    assert!(bb2_choices.len() >= 3);
    assert!(bb2_choices.iter().any(|c| c.label_en.contains("category") || c.label_de.contains("Kategorie")));
    let app_t = din4108_field_meta("elements[].layers[].applicationType").expect("applicationType meta");
    let app_choices = app_t.choices.expect("application choices");
    assert!(app_choices.iter().any(|c| c.value == "WAP" && c.label_de.contains("WDVS")));
    context::close(&mut app);
}
