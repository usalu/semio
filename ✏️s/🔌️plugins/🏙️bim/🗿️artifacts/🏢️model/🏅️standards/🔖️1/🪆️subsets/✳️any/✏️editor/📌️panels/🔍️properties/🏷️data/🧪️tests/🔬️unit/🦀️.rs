use super::super::render;
use crate::{ClassificationItem, ClassificationSystem, ModelInference, ModelSnapshot, PropertyDef, PropertyKind, PropertyTemplate, PropertyValue, TemplateTarget};
use semio_framework_ui_locale::Locale;
use std::collections::BTreeMap;

fn definition(name: &str, kind: PropertyKind, required: bool, default_value: Option<PropertyValue>) -> PropertyDef {
    PropertyDef { name: name.into(), kind, unit: Some("W/(m2.K)".into()).filter(|_| name == "ThermalTransmittance"), description: None, required, default_value, allowed: Vec::new(), minimum: None, maximum: None }
}

fn model() -> (ModelSnapshot, ModelInference) {
    let mut snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let properties = vec![
        definition("ThermalTransmittance", PropertyKind::Real, false, Some(PropertyValue::Real { value: 0.35 })),
        definition("FireRating", PropertyKind::Text, true, None),
    ];
    snapshot.property_templates.insert("pt-wall".into(), PropertyTemplate { name: "Pset_WallCommon".into(), applies_to: vec![TemplateTarget::Wall, TemplateTarget::WallType], properties });
    snapshot.properties.insert("wt-300".into(), BTreeMap::from([("Pset_WallCommon".to_string(), BTreeMap::from([("FireRating".to_string(), PropertyValue::Text { value: "EI60".into() })]))]));
    snapshot.classification_systems.insert("cs-uni".into(), ClassificationSystem { name: "Uniclass 2015".into(), edition: "2024".into(), source: None, entries: vec![ClassificationItem { code: "EF_25_10".into(), title: "Walls".into(), parent: None }] });
    snapshot.classifications.insert("w-south".into(), BTreeMap::from([("cs-uni".to_string(), "EF_25_10".to_string())]));
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(None, &snapshot, Clone::clone);
    (snapshot, inference)
}

fn text(snapshot: &ModelSnapshot, inference: &ModelInference, elements: &[&str], library: &[&str], locale: Locale) -> String {
    let own = |ids: &[&str]| ids.iter().map(|id| id.to_string()).collect::<Vec<_>>();
    let view = semio_framework_plugin::ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native);
    let node = render(snapshot, inference, &own(elements), &own(library), crate::editor::bim::terminology::bim_labels(&view)).expect("the properties panel renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("projects")
}

#[semio_framework_async_macros::async_test]
async fn a_wall_shows_what_it_inherits_what_a_template_defaults_and_what_is_still_missing() {
    let (snapshot, inference) = model();
    let rendered = text(&snapshot, &inference, &["w-south"], &[], Locale::En);
    for expected in ["Effective properties", "Pset_WallCommon · FireRating", "EI60 (from the type)", "Pset_WallCommon · ThermalTransmittance", "0.35 W/(m2.K) (default)"] {
        assert!(rendered.contains(expected), "the effective view shows '{expected}': {rendered}");
    }
    assert!(!rendered.contains("required, no value"), "the type states the required value: {rendered}");
    let german = text(&snapshot, &inference, &["w-south"], &[], Locale::De);
    assert!(german.contains("Wirksame Eigenschaften") && german.contains("vom Typ") && german.contains("Vorgabe"), "{german}");
}

#[semio_framework_async_macros::async_test]
async fn a_missing_required_property_is_a_finding_row_and_a_template_with_defaults_can_be_applied() {
    let (snapshot, inference) = model();
    let rendered = text(&snapshot, &inference, &["w-east"], &[], Locale::En);
    assert!(rendered.contains("Pset_WallCommon · FireRating") && rendered.contains("required, no value"), "{rendered}");
    assert!(rendered.contains("Apply template Pset_WallCommon") && rendered.contains("bim-properties.apply.pt-wall"), "{rendered}");
    let slab = text(&snapshot, &inference, &["st-first"], &[], Locale::En);
    assert!(!slab.contains("Apply template"), "a storey is not a kind the template applies to: {slab}");
}

#[semio_framework_async_macros::async_test]
async fn the_classification_of_a_holder_has_one_input_per_system_and_a_remove_row_where_it_has_a_code() {
    let (snapshot, inference) = model();
    let classified = text(&snapshot, &inference, &["w-south"], &[], Locale::En);
    for expected in ["Classification", "Uniclass 2015 (Walls)", "bim-properties.classification.cs-uni.input", "Remove classification Uniclass 2015"] {
        assert!(classified.contains(expected), "the classified wall shows '{expected}': {classified}");
    }
    let plain = text(&snapshot, &inference, &["w-east"], &[], Locale::En);
    assert!(plain.contains("bim-properties.classification.cs-uni.input") && !plain.contains("Remove classification"), "{plain}");
    let typed = text(&snapshot, &inference, &[], &["wt-300"], Locale::En);
    assert!(typed.contains("Property sets") && typed.contains("Pset_WallCommon · FireRating"), "a type carries and shows its properties: {typed}");
}

#[semio_framework_async_macros::async_test]
async fn a_template_lists_its_definitions_as_inputs_and_a_system_its_entry_commands() {
    let (snapshot, inference) = model();
    let template = text(&snapshot, &inference, &[], &["pt-wall"], Locale::En);
    for expected in ["1. ThermalTransmittance", "bim-properties.definition.0.name.input", "bim-properties.definition.0.default_value.input", "bim-properties.definition.1.required.input", "Remove definition FireRating", "bim-properties.definition.add.input"] {
        assert!(template.contains(expected), "the template shows '{expected}': {template}");
    }
    let system = text(&snapshot, &inference, &[], &["cs-uni"], Locale::En);
    for expected in ["Entry table", "bim-properties.entries.add.input", "bim-properties.entries.remove.input", "Edition"] {
        assert!(system.contains(expected), "the system shows '{expected}': {system}");
    }
    let german = text(&snapshot, &inference, &[], &["cs-uni"], Locale::De);
    assert!(german.contains("Eintragstabelle") && german.contains("Ausgabe"), "{german}");
}
