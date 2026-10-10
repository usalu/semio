use super::*;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, psets, real, rows, string, target};
use crate::standards::v1::subsets::any::io::export::ifc::{export_ifc2x3, model_to_part21};
use semio_s_artifact_stdio_ifc::part21::Part21Document;
use crate::PropertyValue;

#[test]
fn property_values_map_every_kind_to_its_ifc_measure() {
    let name = |value: PropertyValue| property_value(&value).as_typed().map(|(name, _)| name.to_string());
    assert_eq!(name(PropertyValue::Text { value: "a".into() }).as_deref(), Some("IFCLABEL"));
    assert_eq!(name(PropertyValue::Real { value: 1.0 }).as_deref(), Some("IFCREAL"));
    assert_eq!(name(PropertyValue::Integer { value: 2 }).as_deref(), Some("IFCINTEGER"));
    assert_eq!(name(PropertyValue::Boolean { value: true }).as_deref(), Some("IFCBOOLEAN"));
    assert_eq!(name(PropertyValue::Length { value: 1.0 }).as_deref(), Some("IFCLENGTHMEASURE"));
    assert_eq!(name(PropertyValue::Area { value: 1.0 }).as_deref(), Some("IFCAREAMEASURE"));
    assert_eq!(name(PropertyValue::Volume { value: 1.0 }).as_deref(), Some("IFCVOLUMEMEASURE"));
    assert_eq!(name(PropertyValue::Angle { value: 1.0 }).as_deref(), Some("IFCPLANEANGLEMEASURE"));
    assert_eq!(property_value(&PropertyValue::Boolean { value: true }).as_typed().map(|(_, items)| items[0].clone()), Some(V::Enum("T".into())));
}

#[test]
fn user_property_sets_become_typed_single_value_properties_of_the_element() {
    let model = house();
    let document = document(&model);
    let sets: Vec<_> = rows(&document, "IFCPROPERTYSET").into_iter().filter(|(_, args)| string(args, 2).as_deref() == Some("Pset_WallCommon")).collect();
    assert_eq!(sets.len(), 1);
    let properties: Vec<(String, String)> = sets[0].1[4].as_list().expect("properties").iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCPROPERTYSINGLEVALUE")).map(|args| (string(args, 0).unwrap_or_default(), args[2].as_typed().map(|(name, _)| name.to_string()).unwrap_or_default())).collect();
    assert_eq!(properties, [("FireRating".into(), "IFCLABEL".into()), ("IsExternal".into(), "IFCBOOLEAN".into()), ("ThermalTransmittance".into(), "IFCREAL".into())]);
    let (_, relation) = rows(&document, "IFCRELDEFINESBYPROPERTIES").into_iter().find(|(_, args)| args[5].as_ref_id() == Some(sets[0].0.id)).expect("the relation");
    let element = relation[4].as_list().and_then(|items| items.first()).and_then(|item| document.resolve(item)).and_then(|instance| instance.primary()).map(|(name, _)| name.to_string());
    assert_eq!(element.as_deref(), Some("IFCWALL"));
}

#[test]
fn layered_types_become_type_objects_with_a_layer_set_each_and_materials_with_their_physics() {
    let model = house();
    let document = document(&model);
    assert_eq!(count(&document, "IFCWALLTYPE"), model.wall_types.len());
    assert_eq!(count(&document, "IFCSLABTYPE"), model.slab_types.len());
    assert_eq!(count(&document, "IFCBUILDINGELEMENTPROXYTYPE"), model.roof_types.len());
    assert_eq!(count(&document, "IFCMATERIALLAYERSET"), model.wall_types.len() + model.slab_types.len() + model.roof_types.len());
    assert_eq!(count(&document, "IFCMATERIALLAYER"), 2 + 1 + 2 + 2);
    assert_eq!(count(&document, "IFCMATERIAL"), model.materials.len());
    assert_eq!(count(&document, "IFCEXTENDEDMATERIALPROPERTIES"), 3 * model.materials.len());
    let (_, wall_type) = rows(&document, "IFCWALLTYPE").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("wt-300")).expect("wt-300");
    assert_eq!(string(wall_type, 2).as_deref(), Some("Brick 300"));
    assert_eq!(wall_type[9].as_enum(), Some("STANDARD"));
}

#[test]
fn profiled_and_window_types_keep_their_parameters_in_the_semio_authoring_set() {
    let document = document(&house());
    let (_, style) = rows(&document, "IFCWINDOWSTYLE").into_iter().next().expect("a window style");
    let set = style[5].as_list().and_then(|items| items.first()).and_then(|item| document.resolve(item)).and_then(|instance| instance.entity("IFCPROPERTYSET")).expect("its pset");
    assert_eq!(string(set, 2).as_deref(), Some("Semio_Authoring"));
    let names: Vec<String> = set[4].as_list().expect("properties").iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCPROPERTYSINGLEVALUE")).filter_map(|args| string(args, 0)).collect();
    assert_eq!(names, ["Width", "Height", "Sill", "FrameWidth", "FrameDepth", "Panes", "Material", "MaterialName"]);
    let (_, door) = rows(&document, "IFCDOORSTYLE").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("dr-180")).expect("the double door");
    assert_eq!(door[8].as_enum(), Some("DOUBLE_DOOR_SINGLE_SWING"));
}

#[test]
fn occurrences_are_related_to_their_type_object() {
    let document = document(&house());
    let (_, relation) = rows(&document, "IFCRELDEFINESBYTYPE")
        .into_iter()
        .find(|(_, args)| target(&document, args, 5).and_then(|instance| instance.entity("IFCWALLTYPE")).is_some_and(|row| string(row, 7).as_deref() == Some("wt-300")))
        .expect("the wt-300 relation");
    assert_eq!(relation[4].as_list().map(<[V]>::len), Some(5), "south, east, north, west and first-south");
}

#[test]
fn classifications_share_one_classification_per_system_and_one_reference_per_code() {
    let document = document(&house());
    assert_eq!(count(&document, "IFCCLASSIFICATION"), 2);
    assert_eq!(count(&document, "IFCCLASSIFICATIONREFERENCE"), 2);
    let (_, relation) = rows(&document, "IFCRELASSOCIATESCLASSIFICATION")
        .into_iter()
        .find(|(_, args)| target(&document, args, 5).and_then(|instance| instance.entity("IFCCLASSIFICATIONREFERENCE")).is_some_and(|row| string(row, 1).as_deref() == Some("EF_25_10")))
        .expect("the wall classification");
    assert_eq!(relation[4].as_list().map(<[V]>::len), Some(2), "south and east share the code");
    let reference = target(&document, relation, 5).and_then(|instance| instance.entity("IFCCLASSIFICATIONREFERENCE")).expect("the reference");
    let source = document.resolve(&reference[3]).and_then(|instance| instance.entity("IFCCLASSIFICATION")).expect("the source");
    assert_eq!(string(source, 3).as_deref(), Some("Uniclass 2015"));
}

#[test]
fn quantities_are_written_as_element_quantities_named_after_the_element_family() {
    let document = document(&house());
    let names: std::collections::BTreeSet<String> = rows(&document, "IFCELEMENTQUANTITY").iter().filter_map(|(_, args)| string(args, 2)).collect();
    for name in ["Qto_WallBaseQuantities", "Qto_ColumnBaseQuantities", "Qto_BeamBaseQuantities", "Qto_SlabBaseQuantities", "Qto_WindowBaseQuantities", "Qto_DoorBaseQuantities", "Qto_BuildingStoreyBaseQuantities"] {
        assert!(names.contains(name), "{name} in {names:?}");
    }
    let volumes: Vec<f64> = rows(&document, "IFCQUANTITYVOLUME").iter().map(|(_, args)| real(args, 3)).collect();
    assert!(volumes.iter().all(|volume| *volume >= 0.0));
}

fn related(document: &Part21Document, relation: &[V]) -> Vec<String> {
    let mut tags: Vec<String> = relation[4].as_list().expect("related objects").iter().filter_map(|item| document.resolve(item)).filter_map(|instance| instance.primary()).filter_map(|(name, args)| string(args, if name == "IFCBUILDINGSTOREY" { 4 } else { 7 })).collect();
    tags.sort();
    tags
}

fn association(document: &Part21Document, system: &str, code: &str) -> Vec<String> {
    let (_, relation) = rows(document, "IFCRELASSOCIATESCLASSIFICATION")
        .into_iter()
        .find(|(_, args)| {
            target(document, args, 5).and_then(|instance| instance.entity("IFCCLASSIFICATIONREFERENCE")).is_some_and(|reference| {
                string(reference, 1).as_deref() == Some(code) && document.resolve(&reference[3]).and_then(|source| source.entity("IFCCLASSIFICATION")).is_some_and(|source| string(source, 3).as_deref() == Some(system))
            })
        })
        .unwrap_or_else(|| panic!("the association of {system} {code}"));
    related(document, relation)
}

#[test]
fn an_element_carries_one_association_per_system_and_a_type_carries_its_own() {
    let document = document(&psets());
    assert_eq!(association(&document, "Uniclass 2015", "EF_25_10"), ["w-east", "w-south"]);
    assert_eq!(association(&document, "DIN 276", "331"), ["w-south"]);
    assert_eq!(association(&document, "DIN 276", "330"), ["wt-300"], "the type object of the wall type");
    assert_eq!(association(&document, "Uniclass 2015", "EF_25"), ["wt-300"]);
    assert_eq!(association(&document, "DIN 276", "300"), ["dr-180", "st-ground"], "a door style and a storey share a code");
    assert_eq!(association(&document, "Omniclass", "21-02 10 10"), ["c-1"]);
    assert_eq!(count(&document, "IFCRELASSOCIATESCLASSIFICATION"), 6, "one per used (system, code)");
}

#[test]
fn every_table_row_is_a_reference_of_its_system_in_table_order_attached_or_not() {
    let model = psets();
    let document = document(&model);
    assert_eq!(count(&document, "IFCCLASSIFICATION"), 3);
    assert_eq!(count(&document, "IFCCLASSIFICATIONREFERENCE"), model.classification_systems.values().map(|system| system.entries.len()).sum::<usize>());
    let din = rows(&document, "IFCCLASSIFICATION").into_iter().find(|(_, args)| string(args, 3).as_deref() == Some("DIN 276")).expect("DIN 276");
    assert_eq!((string(din.1, 0).as_deref(), string(din.1, 1).as_deref()), (Some("https://www.din.de"), Some("2018-12")));
    let codes: Vec<String> = rows(&document, "IFCCLASSIFICATIONREFERENCE").into_iter().filter(|(_, args)| args[3].as_ref_id() == Some(din.0.id)).filter_map(|(_, args)| string(args, 1)).collect();
    assert_eq!(codes, ["300", "330", "331", "340"], "the unattached row 340 is written too");
    let uniclass = rows(&document, "IFCCLASSIFICATION").into_iter().find(|(_, args)| string(args, 3).as_deref() == Some("Uniclass 2015")).expect("Uniclass 2015");
    assert_eq!((string(uniclass.1, 0), string(uniclass.1, 1)), (Some(String::new()), Some(String::new())), "source and edition are mandatory in IFC 2x3 and written empty");
}

fn parents_set(document: &Part21Document) -> (u64, Vec<V>) {
    rows(document, "IFCPROPERTYSET").into_iter().find(|(_, args)| string(args, 2).as_deref() == Some(PARENTS_SET)).map(|(instance, args)| (instance.id, args.clone())).expect("the parents set")
}

#[test]
fn the_parent_column_travels_in_the_semio_classification_parents_set_of_the_project() {
    let document = document(&psets());
    let (id, set) = parents_set(&document);
    let parents: std::collections::BTreeMap<String, String> = set[4]
        .as_list()
        .expect("properties")
        .iter()
        .filter_map(|item| document.resolve(item))
        .filter_map(|row| row.entity("IFCPROPERTYSINGLEVALUE"))
        .map(|args| (string(args, 0).expect("a key"), args[2].as_typed().and_then(|(_, items)| items[0].as_str()).expect("a label").to_string()))
        .collect();
    assert_eq!(parents.len(), 6);
    assert_eq!(parents["DIN 276|2018-12|331"], "330");
    assert_eq!(parents["Uniclass 2015||EF_25_10"], "EF_25");
    assert!(!parents.contains_key("DIN 276|2018-12|300"), "a root has no parent row");
    let (_, relation) = rows(&document, "IFCRELDEFINESBYPROPERTIES").into_iter().find(|(_, args)| args[5].as_ref_id() == Some(id)).expect("the relation");
    assert!(document.resolve(&relation[4].as_list().expect("objects")[0]).is_some_and(|instance| instance.is_type("IFCPROJECT")));
}

#[test]
fn a_model_without_parents_writes_no_parents_set() {
    let mut model = psets();
    model.classification_systems.values_mut().flat_map(|system| system.entries.iter_mut()).for_each(|entry| entry.parent = None);
    assert!(rows(&document(&model), "IFCPROPERTYSET").iter().all(|(_, args)| string(args, 2).as_deref() != Some(PARENTS_SET)));
}

#[test]
fn a_holder_outside_the_export_a_system_outside_the_library_and_a_code_outside_the_table_are_noted() {
    let mut model = psets();
    model.classifications.insert("ghost".into(), [("cs-din-276".to_string(), "300".to_string())].into());
    model.classifications.insert("w-east".into(), [("cs-missing".to_string(), "1".to_string()), ("cs-din-276".to_string(), "999".to_string())].into());
    let (document, notes) = model_to_part21(crate::standards::v1::subsets::any::io::export::ifc::Schema::Ifc2x3, &model).expect("the model exports");
    assert!(notes.iter().any(|note| note.starts_with("classification ghost: the holder")), "{notes:?}");
    assert!(notes.iter().any(|note| note.starts_with("classification w-east: the system cs-missing")), "{notes:?}");
    assert!(notes.iter().any(|note| note.starts_with("classification w-east: the code 999")), "{notes:?}");
    assert_eq!(association(&document, "DIN 276", "999"), ["w-east"], "an unknown code is still written, without a title");
}

fn type_sets(document: &Part21Document, entity: &str, id: &str) -> Vec<(String, Vec<(String, String)>)> {
    let (_, kind) = rows(document, entity).into_iter().find(|(_, args)| string(args, 7).as_deref() == Some(id)).unwrap_or_else(|| panic!("{entity} {id}"));
    kind[5]
        .as_list()
        .expect("HasPropertySets")
        .iter()
        .filter_map(|item| document.resolve(item))
        .filter_map(|instance| instance.entity("IFCPROPERTYSET"))
        .map(|set| {
            let properties = set[4].as_list().expect("properties").iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCPROPERTYSINGLEVALUE")).map(|row| (string(row, 0).expect("a name"), row[2].as_typed().map(|(name, _)| name.to_string()).expect("typed"))).collect();
            (string(set, 2).expect("a set name"), properties)
        })
        .collect()
}

#[test]
fn a_type_object_lists_its_authored_property_sets_after_its_authoring_set() {
    let document = document(&psets());
    let sets = type_sets(&document, "IFCWALLTYPE", "wt-300");
    assert_eq!(sets.iter().map(|(name, _)| name.as_str()).collect::<Vec<_>>(), ["Semio_Authoring", "Custom", "Pset_WallCommon"]);
    assert_eq!(sets[2].1, [("FireRating".to_string(), "IFCLABEL".to_string()), ("IsExternal".to_string(), "IFCBOOLEAN".to_string()), ("ThermalTransmittance".to_string(), "IFCREAL".to_string())]);
    let door = type_sets(&document, "IFCDOORSTYLE", "dr-180");
    assert_eq!(door[1], ("Pset_DoorCommon".to_string(), vec![("FireExit".to_string(), "IFCBOOLEAN".to_string()), ("Leaves".to_string(), "IFCINTEGER".to_string()), ("Width".to_string(), "IFCLENGTHMEASURE".to_string())]));
    assert_eq!(type_sets(&document, "IFCWINDOWSTYLE", "wnd-120").len(), 2);
    assert_eq!(type_sets(&document, "IFCCOLUMNTYPE", "ct-rect").len(), 2);
    assert_eq!(type_sets(&document, "IFCSLABTYPE", "st-floor").len(), 1, "a type without authored properties lists only its authoring set");
}

#[test]
fn type_properties_are_written_once_on_the_type_and_never_on_its_instances() {
    let document = document(&psets());
    let on_instances = |name: &str| {
        rows(&document, "IFCRELDEFINESBYPROPERTIES")
            .into_iter()
            .filter(|(_, args)| document.resolve(&args[5]).and_then(|set| set.entity("IFCPROPERTYSET")).is_some_and(|set| string(set, 2).as_deref() == Some(name)))
            .count()
    };
    assert_eq!(on_instances("Pset_WallCommon"), 1, "only w-south states its own set; the inherited and default values are inferred, never written");
    assert_eq!(on_instances("Pset_DoorCommon"), 0);
    assert_eq!(on_instances("Custom"), 1, "the column's own custom set");
}

#[test]
fn the_export_with_classifications_and_type_properties_is_deterministic() {
    let model = psets();
    let first = export_ifc2x3(&model).expect("first").0;
    assert_eq!(first, export_ifc2x3(&model).expect("second").0);
    assert!(String::from_utf8_lossy(&first).contains("Semio_ClassificationParents"));
}

#[test]
fn the_parent_key_names_the_system_edition_and_code() {
    let model = psets();
    assert_eq!(parent_key(&model.classification_systems["cs-din-276"], "331"), "DIN 276|2018-12|331");
    assert_eq!(parent_key(&model.classification_systems["cs-uniclass-2015"], "EF_25"), "Uniclass 2015||EF_25");
}
