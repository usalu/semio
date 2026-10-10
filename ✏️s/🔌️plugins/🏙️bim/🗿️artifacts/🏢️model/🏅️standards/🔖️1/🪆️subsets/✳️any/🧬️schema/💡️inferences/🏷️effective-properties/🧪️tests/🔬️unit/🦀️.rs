use super::*;
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticCode;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use crate::{ClassificationItem, ClassificationSystem, Entry, ModelDiff, PropertyDef, PropertyKind, PropertySet, PropertyTemplatePatch};
use protocol::Inference;

fn text(value: &str) -> PropertyValue {
    PropertyValue::Text { value: value.into() }
}

fn real(value: f64) -> PropertyValue {
    PropertyValue::Real { value }
}

fn definition(name: &str, kind: PropertyKind) -> PropertyDef {
    PropertyDef { name: name.into(), kind, unit: None, description: None, required: false, default_value: None, allowed: Vec::new(), minimum: None, maximum: None }
}

fn set(pairs: &[(&str, PropertyValue)]) -> PropertySet {
    PropertySet::from([("Pset_WallCommon".to_string(), pairs.iter().map(|(name, value)| (name.to_string(), value.clone())).collect())])
}

fn model() -> ModelSnapshot {
    let mut snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let fire = PropertyDef { allowed: vec![text("EI30"), text("EI60")], ..definition("FireRating", PropertyKind::Text) };
    let thermal = PropertyDef { minimum: Some(0.0), maximum: Some(5.0), default_value: Some(real(0.35)), ..definition("ThermalTransmittance", PropertyKind::Real) };
    let external = PropertyDef { default_value: Some(PropertyValue::Boolean { value: false }), ..definition("IsExternal", PropertyKind::Boolean) };
    snapshot.property_templates.insert("pt-wall".into(), PropertyTemplate { name: "Pset_WallCommon".into(), applies_to: vec![TemplateTarget::Wall, TemplateTarget::WallType], properties: vec![fire, thermal, external] });
    let strict = PropertyDef { required: true, ..definition("Mass", PropertyKind::Real) };
    snapshot.property_templates.insert("pt-strict".into(), PropertyTemplate { name: "Pset_Strict".into(), applies_to: vec![TemplateTarget::Wall], properties: vec![strict] });
    snapshot.properties.insert("wt-300".into(), set(&[("FireRating", text("EI60"))]));
    snapshot.properties.insert("w-south".into(), set(&[("ThermalTransmittance", real(9.0)), ("IsExternal", PropertyValue::Boolean { value: true })]));
    snapshot.properties.insert("w-east".into(), set(&[("FireRating", text("EI90"))]));
    snapshot
}

fn infer(snapshot: &ModelSnapshot) -> ModelInference {
    ModelInference::infer(snapshot).expect("infers")
}

fn value<'a>(inference: &'a ModelInference, id: &str, property: &str) -> &'a EffectiveValue {
    inference.effective_properties[id].get("Pset_WallCommon", property).unwrap_or_else(|| panic!("{id} has no {property}"))
}

#[test]
fn an_instance_inherits_the_value_of_its_type_overrides_it_and_gets_the_defaults_of_its_kind() {
    let inference = infer(&model());
    let north = (value(&inference, "w-north", "FireRating"), value(&inference, "w-north", "ThermalTransmittance"), value(&inference, "w-north", "IsExternal"));
    assert_eq!((north.0.source, &north.0.value), (Source::Type, &text("EI60")));
    assert_eq!((north.1.source, &north.1.value), (Source::Default, &real(0.35)));
    assert_eq!((north.2.source, north.2.template.as_deref()), (Source::Default, Some("pt-wall")));
    let south = value(&inference, "w-south", "IsExternal");
    assert_eq!((south.source, &south.value), (Source::Own, &PropertyValue::Boolean { value: true }));
    assert_eq!(value(&inference, "w-south", "FireRating").source, Source::Type);
    let kind = &inference.effective_properties["wt-300"];
    assert_eq!(kind.get("Pset_WallCommon", "FireRating").map(|row| row.source), Some(Source::Own));
    assert_eq!(kind.get("Pset_WallCommon", "ThermalTransmittance").map(|row| row.source), Some(Source::Default), "a template of the type kind defaults its type too");
}

#[test]
fn own_values_are_checked_against_the_definition_and_a_required_value_without_source_is_missing() {
    let inference = infer(&model());
    let issues = |id: &str| inference.effective_properties[id].findings.iter().map(|row| (row.set.as_str(), row.property.as_str(), row.issue)).collect::<Vec<_>>();
    assert_eq!(issues("w-south"), [("Pset_Strict", "Mass", Issue::Missing), ("Pset_WallCommon", "ThermalTransmittance", Issue::AboveMaximum)]);
    assert_eq!(issues("w-east"), [("Pset_Strict", "Mass", Issue::Missing), ("Pset_WallCommon", "FireRating", Issue::NotAllowed)]);
    assert_eq!(issues("w-north"), [("Pset_Strict", "Mass", Issue::Missing)], "an inherited value is not re-reported at the instance");
    assert!(issues("wt-300").is_empty(), "the type is no wall: the required property is the instance's duty");
}

#[test]
fn the_findings_become_diagnostics_in_both_languages_and_a_clean_model_has_none() {
    let inference = infer(&model());
    let codes = |code: DiagnosticCode| inference.diagnostics.iter().filter(|finding| finding.code == code).map(|finding| finding.elements[0].as_str()).collect::<Vec<_>>();
    assert_eq!(codes(DiagnosticCode::PropertyRequiredMissing), ["w-east", "w-north", "w-south", "w-west"]);
    assert_eq!(codes(DiagnosticCode::PropertyOutOfRange), ["w-south"]);
    assert_eq!(codes(DiagnosticCode::PropertyNotAllowed), ["w-east"]);
    let missing = inference.diagnostics.iter().find(|finding| finding.code == DiagnosticCode::PropertyRequiredMissing).expect("a finding");
    assert_eq!(missing.text("en").as_deref(), Some("w-east lacks the required property Pset_Strict.Mass."));
    assert!(missing.text("de").is_some_and(|text| text.contains("Pset_Strict.Mass")));
    let clean = infer(&crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot());
    assert!(clean.effective_properties.is_empty() && clean.diagnostics.iter().all(|finding| finding.code.category() != "property"));
}

#[test]
fn classifications_are_checked_against_the_table_of_their_system() {
    let mut snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    snapshot.classification_systems.insert("cs-uni".into(), ClassificationSystem { name: "Uniclass".into(), edition: String::new(), source: None, entries: vec![ClassificationItem { code: "EF_25_10".into(), title: "Walls".into(), parent: None }] });
    snapshot.classifications.insert("w-south".into(), std::collections::BTreeMap::from([("cs-uni".to_string(), "EF_25_10".to_string())]));
    snapshot.classifications.insert("w-east".into(), std::collections::BTreeMap::from([("cs-uni".to_string(), "Zz_99".to_string())]));
    snapshot.classifications.insert("w-north".into(), std::collections::BTreeMap::from([("cs-none".to_string(), "EF_25_10".to_string())]));
    let found = infer(&snapshot).diagnostics;
    let by = |code: DiagnosticCode| found.iter().filter(|finding| finding.code == code).map(|finding| (finding.elements[0].clone(), finding.missing[0].clone())).collect::<Vec<_>>();
    assert_eq!(by(DiagnosticCode::ClassificationUnknownCode), [("w-east".to_string(), "Zz_99".to_string())]);
    assert_eq!(by(DiagnosticCode::RefClassificationSystem), [("w-north".to_string(), "cs-none".to_string())]);
}

#[test]
fn the_cache_is_transparent_and_a_property_edit_recomputes_one_node_and_no_geometry() {
    let snapshot = model();
    let uncached = infer(&snapshot);
    let mut session = ModelInferenceSession::new();
    let cold = session.refresh(&snapshot).clone();
    assert_eq!(cold, uncached);
    assert_eq!(session.report().computed_by_kind.get("properties"), Some(&5), "wt-300 and the four walls: {:?}", session.report());
    let warm = session.refresh(&snapshot).clone();
    assert_eq!(session.report().computed, 0, "a second refresh is all cache hits");
    assert_eq!(warm, uncached);
    let edit = ModelDiff::properties("w-north", Entry::Created(set(&[("FireRating", text("EI30"))])));
    let after = protocol::apply_diff(&edit, &snapshot).expect("applies");
    let incremental = session.update(&after, &edit).clone();
    let report = session.report().clone();
    assert_eq!(report.computed_by_kind.get("properties"), Some(&1), "{report:?}");
    assert_eq!(report.computed_by_kind.get("wall-layout"), None, "a property edit touches no layout");
    assert_eq!(incremental, infer(&after));
    assert_eq!(value(&incremental, "w-north", "FireRating").source, Source::Own);
}

#[test]
fn a_template_edit_reaches_every_holder_it_applies_to_and_a_type_edit_its_instances() {
    let snapshot = model();
    let mut session = ModelInferenceSession::new();
    session.refresh(&snapshot);
    let edit = ModelDiff::property_templates("pt-wall", Entry::Patched(PropertyTemplatePatch { applies_to: Some(vec![TemplateTarget::Wall]), ..Default::default() }));
    let after = protocol::apply_diff(&edit, &snapshot).expect("applies");
    let incremental = session.update(&after, &edit).clone();
    assert_eq!(incremental, infer(&after));
    assert!(incremental.effective_properties["wt-300"].get("Pset_WallCommon", "ThermalTransmittance").is_none(), "the type fell out of the template");
    let retype = ModelDiff::properties("wt-300", Entry::Patched(crate::PropertySetPatch { assigned: std::collections::BTreeMap::from([("Pset_WallCommon".to_string(), std::collections::BTreeMap::from([("FireRating".to_string(), Some(text("EI30")))]))]) }));
    let next = protocol::apply_diff(&retype, &after).expect("applies");
    let served = session.update(&next, &retype).clone();
    assert_eq!(served, infer(&next));
    assert_eq!(value(&served, "w-west", "FireRating").value, text("EI30"), "the instances follow their type");
}

#[test]
fn the_holders_are_the_records_with_data_or_under_a_template_types_first() {
    let snapshot = model();
    let rows = holders(&snapshot);
    assert_eq!(rows[0], ("wt-300".to_string(), None));
    assert!(rows.iter().any(|(id, parent)| id == "w-north" && parent.as_deref() == Some("wt-300")));
    assert_eq!(rows.len(), 5);
    assert!(holders(&crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()).is_empty());
    assert_eq!((target_of(&snapshot, "w-south"), target_of(&snapshot, "wt-300"), target_of(&snapshot, "nobody")), (Some(TemplateTarget::Wall), Some(TemplateTarget::WallType), None));
    assert_eq!(type_of(&snapshot, "w-south").as_deref(), Some("wt-300"));
}

#[test]
fn a_template_of_the_type_kind_and_of_the_kind_it_types_reports_a_bad_type_value_once() {
    let mut snapshot = model();
    snapshot.properties.insert("wt-300".into(), set(&[("FireRating", text("EI90"))]));
    let inference = infer(&snapshot);
    let issues: Vec<_> = inference.effective_properties["wt-300"].findings.iter().map(|row| (row.set.as_str(), row.property.as_str(), row.issue)).collect();
    assert_eq!(issues, [("Pset_WallCommon", "FireRating", Issue::NotAllowed)]);
    assert_eq!(inference.effective_properties["w-north"].findings.iter().filter(|row| row.property == "FireRating").count(), 0, "the instance does not report the value of its type again");
}
