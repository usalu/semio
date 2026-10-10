use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::{PropertyTemplate, PropertyValue, TemplateTarget};

fn edit(op: &str, index: &str, field: &str, value: &str) -> EditTemplate {
    EditTemplate { id: "pt-wall".into(), op: op.into(), index: index.into(), field: field.into(), value: value.into() }
}

fn definitions() -> Vec<PropertyDef> {
    apply(&[], &edit("add", "", "", "FireRating")).and_then(|list| apply(&list, &edit("add", "", "", "Mass:real"))).expect("two definitions")
}

#[semio_framework_async_macros::async_test]
async fn a_definition_is_added_by_name_and_kind_and_removed_and_moved_by_position() {
    let list = definitions();
    assert_eq!((list[0].name.as_str(), list[0].kind), ("FireRating", PropertyKind::Text));
    assert_eq!((list[1].name.as_str(), list[1].kind), ("Mass", PropertyKind::Real));
    let moved = apply(&list, &edit("up", "1", "", "")).expect("moves up");
    assert_eq!(moved.iter().map(|row| row.name.as_str()).collect::<Vec<_>>(), ["Mass", "FireRating"]);
    assert_eq!(apply(&moved, &edit("up", "0", "", "")), Err("bim.template.edit-invalid"));
    assert_eq!(apply(&moved, &edit("down", "1", "", "")), Err("bim.template.edit-invalid"));
    let rest = apply(&moved, &edit("remove", "0", "", "")).expect("removes");
    assert_eq!(rest.len(), 1);
    assert_eq!(apply(&rest, &edit("remove", "5", "", "")), Err("bim.template.edit-invalid"));
}

#[semio_framework_async_macros::async_test]
async fn every_field_of_a_definition_is_set_from_text_and_a_new_kind_clears_what_belonged_to_the_old_one() {
    let mut list = definitions();
    for (field, value) in [("unit", "kg"), ("description", "Mass per metre"), ("required", "true"), ("minimum", "1"), ("maximum", "2.5"), ("default_value", "1.5"), ("allowed", "1.5; 2")] {
        list = apply(&list, &edit("set", "1", field, value)).unwrap_or_else(|code| panic!("{field}: {code}"));
    }
    let mass = &list[1];
    assert_eq!((mass.unit.as_deref(), mass.description.as_deref(), mass.required, mass.minimum, mass.maximum), (Some("kg"), Some("Mass per metre"), true, Some(1.0), Some(2.5)));
    assert_eq!(mass.default_value, Some(PropertyValue::Real { value: 1.5 }));
    assert_eq!(mass.allowed, vec![PropertyValue::Real { value: 1.5 }, PropertyValue::Real { value: 2.0 }]);
    let text = apply(&list, &edit("set", "1", "kind", "text")).expect("changes the kind");
    assert_eq!((text[1].kind, text[1].default_value.clone(), text[1].allowed.len(), text[1].minimum, text[1].maximum), (PropertyKind::Text, None, 0, None, None));
    let cleared = apply(&list, &edit("set", "1", "unit", "  ")).expect("clears the unit");
    assert_eq!(cleared[1].unit, None);
}

#[semio_framework_async_macros::async_test]
async fn nonsense_edits_are_refused_with_their_fault_code() {
    let list = definitions();
    assert_eq!(apply(&list, &edit("set", "0", "colour", "red")), Err("bim.template.edit-invalid"));
    assert_eq!(apply(&list, &edit("frobnicate", "0", "", "")), Err("bim.template.edit-invalid"));
    assert_eq!(apply(&list, &edit("add", "", "", "  ")), Err("bim.template.edit-invalid"));
    assert_eq!(apply(&list, &edit("add", "", "", "Mass:weird")), Err("bim.template.value-invalid"));
    assert_eq!(apply(&list, &edit("set", "1", "minimum", "many")), Err("bim.template.value-invalid"));
    assert_eq!(apply(&list, &edit("set", "1", "default_value", "heavy")), Err("bim.template.value-invalid"));
    assert_eq!(apply(&list, &edit("set", "0", "required", "maybe")), Err("bim.template.value-invalid"));
}

#[semio_framework_async_macros::async_test]
async fn the_command_becomes_one_set_property_template_mutation() {
    let mut snapshot = demo();
    snapshot.property_templates.insert("pt-wall".into(), PropertyTemplate { name: "Pset_WallCommon".into(), applies_to: vec![TemplateTarget::Wall], properties: Vec::new() });
    let mut context = ctx(&[]);
    let emit = run(&snapshot, |doc, cfg| handle(&edit("add", "", "", "FireRating"), doc, cfg, &mut context)).expect("adds a definition");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetPropertyTemplate(_)]));
    assert_eq!(applied(&snapshot, &emit).property_templates["pt-wall"].properties.len(), 1);
    let missing = EditTemplate { id: "pt-none".into(), ..edit("add", "", "", "A") };
    let refused = run(&snapshot, |doc, cfg| handle(&missing, doc, cfg, &mut context));
    assert_eq!(refused.err().map(|fault| fault.code.0), Some("bim.template.target-missing".to_string()));
}
