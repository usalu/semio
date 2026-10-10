use super::*;
use crate::standards::v1::subsets::any::io::export::ifc::model_to_part21;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::{document_in, psets, rows, string};
use crate::{template_problem, PropertyValue};
use std::collections::BTreeSet;

fn document() -> Part21Document {
    document_in(Schema::Ifc4, &psets())
}

fn count(document: &Part21Document, entity: &str) -> usize {
    document.by_type(entity).count()
}

fn definition(name: &str) -> PropertyDef {
    PropertyDef { name: name.to_string(), kind: PropertyKind::Real, unit: None, description: None, required: false, default_value: None, allowed: Vec::new(), minimum: None, maximum: None }
}

#[test]
fn every_target_names_one_distinct_ifc4_entity_and_type_targets_name_type_entities() {
    let names: BTreeSet<&str> = TemplateTarget::ALL.iter().map(|target| applicable_entity(*target)).collect();
    assert_eq!(names.len(), TemplateTarget::ALL.len());
    for target in TemplateTarget::ALL {
        let name = applicable_entity(target);
        assert!(name.starts_with("Ifc"), "{name}");
        assert_eq!(target.is_type(), name.ends_with("Type"), "{target:?} -> {name}");
    }
    assert_eq!((applicable_entity(TemplateTarget::Ceiling), applicable_entity(TemplateTarget::Storey), applicable_entity(TemplateTarget::Void)), ("IfcCovering", "IfcBuildingStorey", "IfcOpeningElement"));
}

#[test]
fn the_template_type_follows_the_kinds_a_template_applies_to() {
    use TemplateTarget::*;
    assert_eq!(template_type(&[]), "NOTDEFINED");
    assert_eq!(template_type(&[Wall, Slab]), "PSET_OCCURRENCEDRIVEN");
    assert_eq!(template_type(&[WallType, DoorType]), "PSET_TYPEDRIVENONLY");
    assert_eq!(template_type(&[Wall, WallType]), "PSET_TYPEDRIVENOVERRIDE");
}

#[test]
fn every_property_kind_has_its_own_ifc_measure_type() {
    let measures: BTreeSet<&str> = PropertyKind::ALL.iter().map(|kind| measure_type(*kind)).collect();
    assert_eq!(measures.len(), PropertyKind::ALL.len());
    assert_eq!((measure_type(PropertyKind::Text), measure_type(PropertyKind::Angle), measure_type(PropertyKind::Length)), ("IfcLabel", "IfcPlaneAngleMeasure", "IfcLengthMeasure"));
}

#[test]
fn a_definition_list_has_stable_keys_and_escapes_the_separators() {
    let plain = definition("A");
    assert_eq!(definition_text(&plain), "required=false;");
    let full = PropertyDef {
        unit: Some("W/(m2.K)".into()),
        description: Some("100% of a;b=c\nnext".into()),
        required: true,
        default_value: Some(PropertyValue::Real { value: 0.35 }),
        minimum: Some(0.0),
        maximum: Some(5.0),
        ..definition("U")
    };
    assert_eq!(definition_text(&full), "description=100%25 of a%3Bb%3Dc%0Anext;unit=W/(m2.K);required=true;default=0.35;minimum=0;maximum=5;");
    let flag = PropertyDef { kind: PropertyKind::Boolean, default_value: Some(PropertyValue::Boolean { value: false }), ..definition("F") };
    assert_eq!(definition_text(&flag), "required=false;default=false;");
    assert_eq!(escape("a%b;c=d"), "a%25b%3Bc%3Dd");
}

#[test]
fn the_library_declares_one_project_library_with_a_template_per_property_set_template() {
    let model = psets();
    let document = document();
    assert!(document.header.file_schema.iter().any(|value| value.as_list().is_some_and(|items| items.iter().any(|item| item.as_str() == Some("IFC4")))));
    let counts: Vec<usize> = COUNTED.iter().take(8).map(|name| count(&document, name)).collect();
    assert_eq!(counts, [1, 1, 2, 4, 9, 2, 3, 9], "{COUNTED:?}");
    assert!(count(&document, "IFCRELASSOCIATESCLASSIFICATION") > 3, "the systems and every holder of a code are associated");
    assert!(count(&document, "IFCRELDEFINESBYTEMPLATE") >= 1, "the property sets named like a template are related to it");
    assert_eq!(count(&document, "IFCPROPERTYSETTEMPLATE"), model.property_templates.len());
    assert_eq!(count(&document, "IFCSIMPLEPROPERTYTEMPLATE"), model.property_templates.values().map(|template| template.properties.len()).sum::<usize>());
    assert_eq!(count(&document, "IFCPROPERTYENUMERATION"), model.property_templates.values().flat_map(|template| template.properties.iter()).filter(|definition| !definition.allowed.is_empty()).count());
    assert_eq!(count(&document, "IFCOWNERHISTORY"), 0, "an IFC4 file writes no owner history");
}

fn template(document: &Part21Document, name: &str) -> Vec<Part21Value> {
    rows(document, "IFCPROPERTYSETTEMPLATE").into_iter().find(|(_, args)| string(args, 2).as_deref() == Some(name)).map(|(_, args)| args.clone()).unwrap_or_else(|| panic!("the template {name}"))
}

fn properties(document: &Part21Document, template: &[Part21Value]) -> Vec<Vec<Part21Value>> {
    template[6].as_list().expect("templates").iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCSIMPLEPROPERTYTEMPLATE")).cloned().collect()
}

#[test]
fn templates_carry_their_type_and_the_ifc_entities_of_their_targets() {
    let document = document();
    let wall = template(&document, "Pset_WallCommon");
    assert_eq!((wall[4].as_enum(), string(&wall, 5).as_deref()), (Some("PSET_TYPEDRIVENOVERRIDE"), Some("IfcWall,IfcWallType")));
    let window = template(&document, "Pset_WindowTypeCommon");
    assert_eq!((window[4].as_enum(), string(&window, 5).as_deref()), (Some("PSET_TYPEDRIVENONLY"), Some("IfcWindowType")));
    let space = template(&document, "Pset_SpaceCommon");
    assert_eq!((space[4].as_enum(), string(&space, 5).as_deref()), (Some("PSET_OCCURRENCEDRIVEN"), Some("IfcSpace")));
}

#[test]
fn simple_property_templates_keep_the_order_the_measure_type_and_the_definition_list() {
    let document = document();
    let door = properties(&document, &template(&document, "Pset_DoorCommon"));
    assert_eq!(door.iter().filter_map(|row| string(row, 2)).collect::<Vec<_>>(), ["FireExit", "Width", "Leaves"]);
    assert_eq!(door.iter().filter_map(|row| string(row, 5)).collect::<Vec<_>>(), ["IfcBoolean", "IfcLengthMeasure", "IfcInteger"]);
    assert_eq!(door.iter().map(|row| row[4].as_enum().unwrap_or_default()).collect::<Vec<_>>(), ["P_SINGLEVALUE", "P_SINGLEVALUE", "P_ENUMERATEDVALUE"]);
    assert_eq!(string(&door[1], 3).as_deref(), Some("description=Clear width%3B keep %3D and %3B apart;required=true;minimum=0.6;maximum=3;"));
    assert_eq!(string(&door[2], 3).as_deref(), Some("required=false;default=1;"));
}

#[test]
fn an_allowed_list_becomes_an_ifc_property_enumeration_of_typed_values() {
    let document = document();
    let wall = properties(&document, &template(&document, "Pset_WallCommon"));
    let enumeration = document.resolve(&wall[0][7]).and_then(|instance| instance.entity("IFCPROPERTYENUMERATION")).expect("the enumeration of FireRating");
    assert_eq!(string(enumeration, 0).as_deref(), Some("FireRating"));
    let values: Vec<(String, String)> = enumeration[1].as_list().expect("values").iter().filter_map(|value| value.as_typed()).map(|(name, items)| (name.to_string(), items[0].as_str().unwrap_or_default().to_string())).collect();
    assert_eq!(values, [("IFCLABEL".to_string(), "EI30".to_string()), ("IFCLABEL".to_string(), "EI60".to_string()), ("IFCLABEL".to_string(), "EI90".to_string())]);
    assert!(wall[1][7].is_unset(), "a property without allowed values has no enumeration");
    let leaves = properties(&document, &template(&document, "Pset_DoorCommon"));
    let numbers = document.resolve(&leaves[2][7]).and_then(|instance| instance.entity("IFCPROPERTYENUMERATION")).expect("the enumeration of Leaves");
    assert!(numbers[1].as_list().expect("values").iter().all(|value| value.as_typed().is_some_and(|(name, _)| name == "IFCINTEGER")));
}

#[test]
fn every_reference_chains_to_its_parent_reference_or_its_classification_and_keeps_its_table_position() {
    let model = psets();
    let document = document();
    for (id, system) in &model.classification_systems {
        let source = rows(&document, "IFCCLASSIFICATION").into_iter().find(|(_, args)| string(args, 3).as_deref() == Some(system.name.as_str())).unwrap_or_else(|| panic!("{id}"));
        assert_eq!((string(source.1, 0).unwrap_or_default(), string(source.1, 1).unwrap_or_default()), (system.source.clone().unwrap_or_default(), system.edition.clone()));
        for (index, entry) in system.entries.iter().enumerate() {
            let (_, reference) = rows(&document, "IFCCLASSIFICATIONREFERENCE")
                .into_iter()
                .find(|(_, args)| string(args, 1).as_deref() == Some(entry.code.as_str()) && chain_root(&document, args) == Some(source.0.id))
                .unwrap_or_else(|| panic!("{id} {}", entry.code));
            assert_eq!(string(reference, 2).as_deref(), Some(entry.title.as_str()));
            assert_eq!(string(reference, 5), Some(format!("{index:06}")));
            let above = reference[3].as_ref_id().and_then(|target| document.instance(target)).expect("a source");
            let parent = above.entity("IFCCLASSIFICATIONREFERENCE").and_then(|args| string(args, 1));
            assert_eq!(parent, entry.parent, "{id} {}", entry.code);
            if entry.parent.is_none() {
                assert_eq!(above.id, source.0.id, "a root hangs below its classification");
            }
            let mut codes = Vec::new();
            let mut at = reference[3].as_ref_id();
            while let Some(args) = at.and_then(|target| document.instance(target)).and_then(|instance| instance.entity("IFCCLASSIFICATIONREFERENCE")) {
                codes.push(string(args, 1).unwrap_or_default());
                at = args[3].as_ref_id();
            }
            codes.reverse();
            let expected: Vec<String> = system.lineage(&entry.code).iter().take(system.lineage(&entry.code).len() - 1).map(|row| row.code.clone()).collect();
            assert_eq!(codes, expected, "{id} {}", entry.code);
        }
    }
}

fn chain_root(document: &Part21Document, args: &[Part21Value]) -> Option<u64> {
    let mut at = args[3].as_ref_id();
    for _ in 0..document.instances.len() {
        let instance = document.instance(at?)?;
        match instance.entity("IFCCLASSIFICATIONREFERENCE") {
            Some(above) => at = above[3].as_ref_id(),
            None => return Some(instance.id),
        }
    }
    None
}

#[test]
fn the_project_declares_the_library_which_declares_the_templates_and_is_associated_with_every_system() {
    let document = document();
    let declares: Vec<(String, usize)> = rows(&document, "IFCRELDECLARES")
        .into_iter()
        .map(|(_, args)| (document.resolve(&args[4]).and_then(|instance| instance.primary()).map(|(name, _)| name.to_string()).unwrap_or_default(), args[5].as_list().map_or(0, <[Part21Value]>::len)))
        .collect();
    assert_eq!(declares, [("IFCPROJECT".to_string(), 1), ("IFCPROJECTLIBRARY".to_string(), 4)]);
    let associated: Vec<(String, String)> = rows(&document, "IFCRELASSOCIATESCLASSIFICATION")
        .into_iter()
        .map(|(_, args)| (document.resolve(&args[4].as_list().expect("objects")[0]).and_then(|instance| instance.primary()).map(|(name, _)| name.to_string()).unwrap_or_default(), document.resolve(&args[5]).and_then(|instance| instance.entity("IFCCLASSIFICATION")).and_then(|source| string(source, 3)).unwrap_or_default()))
        .filter(|(holder, _)| holder == "IFCPROJECTLIBRARY")
        .collect();
    assert_eq!(associated.len(), 3);
    assert_eq!(associated.iter().map(|(_, name)| name.as_str()).collect::<BTreeSet<_>>(), BTreeSet::from(["DIN 276", "Omniclass", "Uniclass 2015"]));
}

#[test]
fn the_bytes_are_an_ifc4_file_that_the_stdio_reader_decodes_back_to_the_same_graph() {
    let (bytes, notes) = export_ifc4(&psets()).expect("the file exports");
    assert!(notes.is_empty(), "{notes:?}");
    assert!(bytes.starts_with(b"ISO-10303-21"));
    let decoded = codec::decode_ifc4(&bytes).expect("the reader decodes it");
    assert_eq!(decoded.instances, document().instances);
    assert!(codec::decode_document(&bytes).is_err(), "the IFC2X3 reader refuses it");
    assert!(codec::decode_ifc4(&crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3(&psets()).expect("the 2x3 file").0).is_err(), "and the IFC4 reader refuses an IFC2X3 file");
}

#[test]
fn the_library_is_deterministic_and_every_reference_resolves() {
    let model = psets();
    assert_eq!(export_ifc4(&model).expect("first").0, export_ifc4(&model).expect("second").0);
    let document = document();
    let ids: BTreeSet<u64> = document.instances.iter().map(|instance| instance.id).collect();
    assert_eq!(ids.len(), document.instances.len());
    fn walk(value: &Part21Value, ids: &BTreeSet<u64>) {
        match value {
            Part21Value::Ref(id) => assert!(ids.contains(id), "#{id} is missing"),
            Part21Value::List(items) | Part21Value::Typed { items, .. } => items.iter().for_each(|item| walk(item, ids)),
            _ => {}
        }
    }
    document.instances.iter().flat_map(|instance| instance.entities.iter()).flat_map(|(_, args)| args.iter()).for_each(|arg| walk(arg, &ids));
}

#[test]
fn what_cannot_be_written_is_noted_and_the_rest_still_is() {
    let mut model = psets();
    model.property_templates.insert("pt-empty".into(), PropertyTemplate { name: "Pset_Empty".into(), applies_to: vec![TemplateTarget::Wall], properties: Vec::new() });
    let din = model.classification_systems.get_mut("cs-din-276").expect("DIN 276");
    din.entries.push(ClassificationItem { code: "399".into(), title: "Orphan".into(), parent: Some("missing".into()) });
    din.entries.push(ClassificationItem { code: "300".into(), title: "Twice".into(), parent: None });
    let (document, notes) = model_to_part21(Schema::Ifc4, &model).expect("the model exports");
    assert!(notes.iter().any(|note| note == "template pt-empty: it defines no property"), "{notes:?}");
    assert!(notes.iter().any(|note| note.contains("the parent of 399")), "{notes:?}");
    assert!(notes.iter().any(|note| note.contains("the code 300 is listed twice")), "{notes:?}");
    assert_eq!(count(&document, "IFCPROPERTYSETTEMPLATE"), 4);
    assert_eq!(count(&document, "IFCCLASSIFICATIONREFERENCE"), 10, "the orphan is written below its classification, the repeated code is not");
}

#[test]
fn a_parent_that_comes_later_in_the_table_is_still_written_before_its_child() {
    let mut model = psets();
    let uniclass = model.classification_systems.get_mut("cs-uniclass-2015").expect("Uniclass");
    uniclass.entries.reverse();
    let (document, notes) = model_to_part21(Schema::Ifc4, &model).expect("the model exports");
    assert!(notes.is_empty(), "{notes:?}");
    for (instance, args) in rows(&document, "IFCCLASSIFICATIONREFERENCE") {
        assert!(args[3].as_ref_id().is_some_and(|target| target < instance.id), "#{} refers to #{:?}", instance.id, args[3].as_ref_id());
    }
    let sorts: Vec<String> = rows(&document, "IFCCLASSIFICATIONREFERENCE").into_iter().filter(|(_, args)| string(args, 1).as_deref().is_some_and(|code| code.starts_with("EF"))).filter_map(|(_, args)| string(args, 5)).collect();
    assert_eq!(sorts, ["000003", "000000", "000002", "000001"], "the table position of the reversed rows, parents first");
}

#[test]
fn the_definitions_the_library_describes_are_sound() {
    for template in psets().property_templates.values() {
        assert!(template_problem(&template.name, &template.applies_to, &template.properties).is_none(), "{}", template.name);
    }
}

#[test]
fn the_report_lists_templates_systems_declarations_and_associations() {
    let report: serde_json::Value = serde_json::from_str(&library_report(&document())).expect("valid JSON");
    assert_eq!(report["schema"], "IFC4");
    assert_eq!(report["counts"]["IfcPropertySetTemplate"], 4);
    assert_eq!(report["templates"]["Pset_WallCommon"]["type"], "PSET_TYPEDRIVENOVERRIDE");
    assert_eq!(report["templates"]["Pset_WallCommon"]["properties"][0]["values"][1], serde_json::json!(["IFCLABEL", "EI60"]));
    assert_eq!(report["systems"]["DIN 276|2018-12"]["entries"][2], serde_json::json!({"code": "331", "title": "Load-bearing exterior walls", "parent": "330", "sort": "000002"}));
    assert_eq!(report["systems"]["DIN 276|2018-12"]["source"], "https://www.din.de");
    assert_eq!(report["declares"], serde_json::json!(["IFCPROJECT:IFC Psets>IFCPROJECTLIBRARY:IFC Psets Library", "IFCPROJECTLIBRARY:IFC Psets Library>IFCPROPERTYSETTEMPLATE:Pset_DoorCommon,IFCPROPERTYSETTEMPLATE:Pset_SpaceCommon,IFCPROPERTYSETTEMPLATE:Pset_WallCommon,IFCPROPERTYSETTEMPLATE:Pset_WindowTypeCommon"]));
    assert!(report["associated"].as_array().expect("associations").iter().any(|row| row == "IFCPROJECTLIBRARY:IFC Psets Library>DIN 276|2018-12"));
}

#[semio_framework_async_macros::async_test]
async fn the_serializer_returns_the_file_as_a_binary_payload_with_a_diagnostic_per_skipped_item() {
    let clean = ModelIntoIfc4::serialize(&psets(), &ArchiveChildren::empty()).await.expect("the psets model serializes");
    assert!(matches!(clean.value, IoPayload::Binary(ref bytes) if bytes.starts_with(b"ISO-10303-21")));
    assert!(clean.diagnostics.is_empty());
    assert_eq!(<ModelIntoIfc4 as Serializer<ModelSnapshot>>::INTO.standard.0, "4");
    let mut broken = psets();
    broken.property_templates.insert("pt-empty".into(), PropertyTemplate { name: "Pset_Empty".into(), applies_to: Vec::new(), properties: Vec::new() });
    let outcome = ModelIntoIfc4::serialize(&broken, &ArchiveChildren::empty()).await.expect("it still serializes");
    assert!(outcome.diagnostics.iter().any(|diagnostic| diagnostic.message.contains("pt-empty")));
}
