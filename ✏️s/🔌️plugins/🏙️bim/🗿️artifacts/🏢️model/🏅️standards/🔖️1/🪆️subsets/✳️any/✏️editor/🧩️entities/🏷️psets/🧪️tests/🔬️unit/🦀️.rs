use super::super::{kind_holding, kind_of, EntityKind, ENTITIES};
use super::*;
use crate::{ClassificationItem, PropertyDef, PropertyKind};

fn model() -> ModelSnapshot {
    let mut snapshot = ModelSnapshot::default();
    let definition = PropertyDef { name: "FireRating".into(), kind: PropertyKind::Text, unit: None, description: None, required: true, default_value: None, allowed: Vec::new(), minimum: None, maximum: None };
    snapshot.property_templates.insert("pt-wall".into(), PropertyTemplate { name: "Pset_WallCommon".into(), applies_to: vec![TemplateTarget::Wall, TemplateTarget::WallType], properties: vec![definition] });
    snapshot.classification_systems.insert("cs-uni".into(), ClassificationSystem { name: "Uniclass 2015".into(), edition: "2024".into(), source: Some("https://www.thenbs.com".into()), entries: vec![ClassificationItem { code: "Pr".into(), title: "Products".into(), parent: None }] });
    snapshot.classifications.insert("w-1".into(), std::collections::BTreeMap::from([("cs-uni".to_string(), "Pr".to_string())]));
    snapshot
}

fn field(kind: &str, key: &str) -> &'static FieldRow {
    kind_of(kind).expect("the kind").fields.iter().find(|row| row.key == key).unwrap_or_else(|| panic!("no {key} row on {kind}"))
}

fn written(snapshot: &ModelSnapshot, kind: &str, key: &str, id: &str, value: &str) -> Option<ModelSnapshot> {
    let mutation = (field(kind, key).write.expect("editable"))(snapshot, id, value)?;
    crate::mutations::apply_model_mutation(snapshot, &mutation).ok()
}

#[semio_framework_async_macros::async_test]
async fn templates_and_systems_are_library_kinds_that_can_be_created_renamed_and_deleted() {
    let snapshot = model();
    for (kind, id) in [("property-template", "pt-wall"), ("classification-system", "cs-uni")] {
        let row = kind_of(kind).expect("the kind is declared");
        assert!(row.library && row.create.is_some() && row.delete.is_some() && row.rename.is_some(), "{kind} is a library kind with create, rename and delete");
        assert_eq!((row.ids)(&snapshot), [id]);
        assert!(kind_holding(&snapshot, id).is_some_and(|holder| holder.kind == kind));
    }
    assert_eq!(ENTITIES.iter().filter(|row| row.kind == "property-template" || row.kind == "classification-system").count(), 2);
}

#[semio_framework_async_macros::async_test]
async fn the_kinds_a_template_applies_to_read_and_write_as_comma_separated_names() {
    assert_eq!(parse_targets("wall, Curtain Wall,wall-type, wall"), Some(vec![TemplateTarget::Wall, TemplateTarget::CurtainWall, TemplateTarget::WallType]));
    assert_eq!(parse_targets("  "), Some(Vec::new()));
    assert_eq!(parse_targets("wall, spaceship"), None);
    assert_eq!(targets_text(&[TemplateTarget::Wall, TemplateTarget::DoorType]), "wall, door-type");
    let snapshot = model();
    assert_eq!((field("property-template", "applies_to").read)(&snapshot, "pt-wall").as_deref(), Some("wall, wall-type"));
    let retargeted = written(&snapshot, "property-template", "applies_to", "pt-wall", "slab").expect("retargets");
    assert_eq!(retargeted.property_templates["pt-wall"].applies_to, [TemplateTarget::Slab]);
    assert!((field("property-template", "applies_to").write.expect("editable"))(&snapshot, "pt-wall", "slab, spaceship").is_none(), "an unknown kind refuses the whole edit");
    let renamed = written(&snapshot, "property-template", "name", "pt-wall", "  Pset_Wall  ").expect("renames");
    assert_eq!(renamed.property_templates["pt-wall"].name, "Pset_Wall");
}

#[semio_framework_async_macros::async_test]
async fn a_classification_system_is_edited_by_name_edition_and_source_and_a_blank_source_removes_it() {
    let snapshot = model();
    let read = |key: &str| (field("classification-system", key).read)(&snapshot, "cs-uni");
    assert_eq!((read("name").as_deref(), read("edition").as_deref(), read("source").as_deref()), (Some("Uniclass 2015"), Some("2024"), Some("https://www.thenbs.com")));
    let edited = written(&snapshot, "classification-system", "edition", "cs-uni", "2025").expect("sets the edition");
    assert_eq!(edited.classification_systems["cs-uni"].edition, "2025");
    let without = written(&snapshot, "classification-system", "source", "cs-uni", "   ").expect("clears the source");
    assert_eq!(without.classification_systems["cs-uni"].source, None);
    let with = written(&without, "classification-system", "source", "cs-uni", " https://example.org ").expect("sets the source");
    assert_eq!(with.classification_systems["cs-uni"].source.as_deref(), Some("https://example.org"));
}

#[semio_framework_async_macros::async_test]
async fn the_inferred_rows_count_the_definitions_the_entries_and_the_holders_that_use_them() {
    let snapshot = model();
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(None, &snapshot, Clone::clone);
    let template = kind_of("property-template").expect("the kind");
    let system = kind_of("classification-system").expect("the kind");
    let read = |row: &'static EntityKind, key: &str, id: &str| row.inferred.iter().find(|inferred| inferred.key == key).and_then(|inferred| (inferred.read)(&snapshot, &inference, id));
    assert_eq!(read(template, "definitions", "pt-wall").as_deref(), Some("1"));
    assert_eq!(read(system, "entries", "cs-uni").as_deref(), Some("1"));
    assert_eq!(read(system, "classified", "cs-uni").as_deref(), Some("1"));
    assert_eq!(read(template, "reached", "pt-none"), None);
}

#[semio_framework_async_macros::async_test]
async fn a_new_template_and_a_new_system_are_empty_and_named() {
    let snapshot = model();
    let template = create_property_template(&snapshot, "pt-new", "", "Pset_New").expect("creates");
    assert!(matches!(&template, ModelMutation::CreatePropertyTemplate(row) if row.template.name == "Pset_New" && row.template.applies_to.is_empty() && row.template.properties.is_empty()));
    let system = create_classification_system(&snapshot, "cs-new", "", "DIN 276").expect("creates");
    assert!(matches!(&system, ModelMutation::CreateClassificationSystem(row) if row.system.name == "DIN 276" && row.system.entries.is_empty() && row.system.source.is_none()));
    assert!(crate::mutations::apply_model_mutation(&snapshot, &template).is_ok() && crate::mutations::apply_model_mutation(&snapshot, &system).is_ok());
}
