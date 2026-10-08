use super::*;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, real, rows, string, target};
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
