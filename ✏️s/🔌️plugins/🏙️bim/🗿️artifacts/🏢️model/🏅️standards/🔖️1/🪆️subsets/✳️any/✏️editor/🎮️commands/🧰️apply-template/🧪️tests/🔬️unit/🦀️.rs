use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::{PropertyDef, PropertyKind, PropertyTemplate, PropertyValue, TemplateTarget};

fn definition(name: &str, kind: PropertyKind, default_value: Option<PropertyValue>) -> PropertyDef {
    PropertyDef { name: name.into(), kind, unit: None, description: None, required: false, default_value, allowed: Vec::new(), minimum: None, maximum: None }
}

fn with_template() -> ModelSnapshot {
    let mut snapshot = demo();
    let properties = vec![definition("FireRating", PropertyKind::Text, Some(PropertyValue::Text { value: "EI30".into() })), definition("Mass", PropertyKind::Real, None), definition("External", PropertyKind::Boolean, Some(PropertyValue::Boolean { value: false }))];
    snapshot.property_templates.insert("pt-wall".into(), PropertyTemplate { name: "Pset_WallCommon".into(), applies_to: vec![TemplateTarget::Wall, TemplateTarget::WallType], properties });
    snapshot
}

fn apply(snapshot: &ModelSnapshot, ids: &[&str], template: &str, selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut ctx = ctx(selected);
    run(snapshot, |doc, cfg| handle(&ApplyTemplate { ids: ids.iter().map(|id| id.to_string()).collect(), template: template.into() }, doc, cfg, &mut ctx))
}

#[semio_framework_async_macros::async_test]
async fn applying_gives_the_defaults_a_holder_does_not_state_and_nothing_twice() {
    let snapshot = with_template();
    let emit = apply(&snapshot, &["w-south"], "pt-wall", &[]).expect("applies");
    assert_eq!(emit.artifact_mutations.len(), 2, "the two defaults, not the property without one");
    let after = applied(&snapshot, &emit);
    let set = &after.properties["w-south"]["Pset_WallCommon"];
    assert_eq!(set["FireRating"], PropertyValue::Text { value: "EI30".into() });
    assert!(!set.contains_key("Mass"));
    assert!(apply(&after, &["w-south"], "pt-wall", &[]).expect("nothing left").artifact_mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_stated_value_is_kept_and_a_type_is_a_holder_the_instances_inherit_from() {
    let mut snapshot = with_template();
    snapshot.properties.insert("w-south".into(), std::collections::BTreeMap::from([("Pset_WallCommon".to_string(), std::collections::BTreeMap::from([("FireRating".to_string(), PropertyValue::Text { value: "EI90".into() })]))]));
    let emit = apply(&snapshot, &[], "pt-wall", &["w-south", "wt-300"]).expect("applies to the selection");
    let after = applied(&snapshot, &emit);
    assert_eq!(after.properties["w-south"]["Pset_WallCommon"]["FireRating"], PropertyValue::Text { value: "EI90".into() });
    assert_eq!(after.properties["wt-300"]["Pset_WallCommon"]["FireRating"], PropertyValue::Text { value: "EI30".into() });
}

#[semio_framework_async_macros::async_test]
async fn a_missing_template_a_missing_target_and_a_kind_the_template_does_not_cover_are_refused() {
    let snapshot = with_template();
    let code = |result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>| result.err().map(|fault| fault.code.0);
    assert_eq!(code(apply(&snapshot, &["w-south"], "pt-none", &[])), Some("bim.template.target-missing".to_string()));
    assert_eq!(code(apply(&snapshot, &["nobody"], "pt-wall", &[])), Some("bim.property.target-missing".to_string()));
    assert_eq!(code(apply(&snapshot, &["st-first"], "pt-wall", &[])), Some("bim.template.not-applicable".to_string()));
}
