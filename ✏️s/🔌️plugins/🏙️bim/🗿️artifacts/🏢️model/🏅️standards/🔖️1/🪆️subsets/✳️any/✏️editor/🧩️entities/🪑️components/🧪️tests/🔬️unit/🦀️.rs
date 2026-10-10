use super::*;
use crate::editor::bim::entities::{kind_holding, kind_of};
use crate::editor::bim::terminology::BimLabels;
use crate::{ExprPoint3, Family, FamilyParameter, FamilySolid, ModelInference, ParameterKind, SolidShape};
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧬️families/🪑️table/📸️snapshot/🔣️.json");

/// 🛋️ The demo room (walls, storeys) with the families of the table fixture (a table, three profiles, a broken generic one) and a basin (plumbing, one parameter `width`, one cuboid).
pub(crate) fn furnished() -> ModelSnapshot {
    let mut snapshot = crate::editor::bim::gestures::tests::fixture::room();
    let table: ModelSnapshot = from_json_str(TABLE, JsonMemberPolicy::Reject).expect("the table decodes");
    snapshot.materials.extend(table.materials);
    snapshot.families = table.families;
    snapshot.family_parameters = table.family_parameters;
    snapshot.family_solids = table.family_solids;
    snapshot.families.insert("fam-basin".into(), Family { name: "Basin".into(), category: FamilyCategory::Plumbing });
    snapshot.family_parameters.insert("fam-basin.width".into(), FamilyParameter { family: "fam-basin".into(), name: "width".into(), kind: ParameterKind::Length, value: "0.5 m".into() });
    let offset = ExprPoint3 { x: "0 m".into(), y: "0 m".into(), z: "0 m".into() };
    let shape = SolidShape::Cuboid { x: "-width / 2".into(), y: "0 m".into(), z: "0 m".into(), width: "width".into(), depth: "0.4 m".into(), height: "0.2 m".into() };
    snapshot.family_solids.insert("s-basin".into(), FamilySolid { family: "fam-basin".into(), name: "Bowl".into(), shape, material: "\"m-steel\"".into(), visible: "true".into(), offset });
    snapshot
}

/// 🪑️ [`furnished`] with a table `c-table` and a basin `c-basin` mounted on the first wall, on the ground storey.
pub(crate) fn placed() -> ModelSnapshot {
    let mut snapshot = furnished();
    let wall = snapshot.walls.keys().next().cloned().expect("the room has walls");
    let storey = snapshot.walls[&wall].storey.clone();
    let component = |family: &str, host: Option<String>, name: &str| Component { storey: storey.clone(), family: family.into(), position: Point2 { x: 1.0, y: 1.0 }, elevation: 0.0, rotation: 0.0, mirrored: false, host, system: None, name: name.into() };
    snapshot.components.insert("c-table".into(), component("fam-table", None, "Table 1"));
    snapshot.components.insert("c-basin".into(), component("fam-basin", Some(wall), "Basin 1"));
    snapshot
}

fn write(kind: &str, key: &str, snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let field = kind_of(kind).expect("kind").fields.iter().find(|field| field.key == key).expect("field");
    (field.write.expect("editable"))(snapshot, id, value)
}

#[test]
fn only_families_that_are_no_profile_can_be_placed_and_they_come_by_category_then_name() {
    let snapshot = furnished();
    let order: Vec<_> = placeable(&snapshot).into_iter().map(|(id, category)| (id, category)).collect();
    assert_eq!(order, vec![("fam-table".to_string(), FamilyCategory::Furniture), ("fam-basin".to_string(), FamilyCategory::Plumbing), ("fam-broken".to_string(), FamilyCategory::Generic)]);
    assert!(wall_mounted(FamilyCategory::Plumbing) && wall_mounted(FamilyCategory::Lighting) && wall_mounted(FamilyCategory::Electrical) && wall_mounted(FamilyCategory::Casework));
    assert!(!wall_mounted(FamilyCategory::Furniture) && !wall_mounted(FamilyCategory::Equipment) && !wall_mounted(FamilyCategory::Mechanical) && !wall_mounted(FamilyCategory::Generic));
}

#[test]
fn a_search_matches_the_name_the_id_and_the_category_in_both_languages() {
    let snapshot = furnished();
    assert_eq!(matching(&snapshot, "", None), vec!["fam-table", "fam-basin", "fam-broken"]);
    assert_eq!(matching(&snapshot, "  BAS ", None), vec!["fam-basin"]);
    assert_eq!(matching(&snapshot, "plumb", None), vec!["fam-basin"]);
    assert_eq!(matching(&snapshot, "fam-br", None), vec!["fam-broken"]);
    assert_eq!(matching(&snapshot, "Sanitär", Some(&BimLabels::NATIVE_DE)), vec!["fam-basin"]);
    assert_eq!(matching(&snapshot, "Sanitär", Some(&BimLabels::NATIVE_EN)), Vec::<String>::new());
    assert!(matching(&snapshot, "HEA", None).is_empty(), "a profile is no search hit");
}

#[test]
fn the_pickers_name_families_walls_and_systems_by_their_localized_labels() {
    let snapshot = placed();
    let families = family_choices(&snapshot, &BimLabels::NATIVE_DE);
    assert_eq!(families[1], ("fam-basin".to_string(), "Basin · Sanitär".to_string()));
    let hosts = host_choices(&snapshot, &BimLabels::NATIVE_EN);
    assert_eq!(hosts.len(), snapshot.walls.len());
    assert!(hosts[0].1.contains('·'));
    let systems = system_choices(&snapshot, &BimLabels::NATIVE_DE);
    assert_eq!((systems.len(), systems[0].clone(), systems[3].clone()), (9, ("Supply".to_string(), "Zuluft".to_string()), ("DomesticWater".to_string(), "Trinkwasser".to_string())));
    assert_eq!((system_of("water"), system_of(" gas "), system_of("lighting"), system_of("steam")), (Some(MepSystem::DomesticWater), Some(MepSystem::Gas), Some(MepSystem::Lighting), None));
}

#[test]
fn a_component_is_a_placed_entity_under_its_storey_and_its_override_hangs_under_the_component() {
    let mut snapshot = placed();
    let storey = snapshot.components["c-table"].storey.clone();
    snapshot.component_overrides.insert("c-table.width".into(), crate::ComponentOverride { component: "c-table".into(), name: "width".into(), value: "2 m".into() });
    let (component, overridden) = (kind_of("component").expect("component"), kind_of("component-override").expect("override"));
    assert!(!component.library && !overridden.library);
    assert_eq!((component.name)(&snapshot, "c-table").as_deref(), Some("Table 1"));
    assert_eq!((component.parent)(&snapshot, "c-table"), Some(storey));
    assert_eq!((overridden.name)(&snapshot, "c-table.width").as_deref(), Some("width"));
    assert_eq!((overridden.parent)(&snapshot, "c-table.width").as_deref(), Some("c-table"));
    assert_eq!(kind_holding(&snapshot, "c-table.width").map(|row| row.kind), Some("component-override"));
    assert_eq!(kind_holding(&snapshot, "c-basin").map(|row| row.kind), Some("component"));
    assert!(component.create.is_some() && overridden.create.is_none() && overridden.rename.is_none());
}

#[test]
fn the_component_fields_read_the_authored_values() {
    let snapshot = placed();
    let component = kind_of("component").expect("component");
    let read = |key: &str, id: &str| (component.fields.iter().find(|field| field.key == key).expect("field").read)(&snapshot, id);
    assert_eq!((read("family", "c-table").as_deref(), read("position", "c-table").as_deref(), read("elevation", "c-table").as_deref()), (Some("fam-table"), Some("1, 1"), Some("0")));
    assert_eq!((read("host", "c-table").as_deref(), read("host", "c-basin").is_some_and(|host| !host.is_empty()), read("system", "c-table").as_deref()), (Some(""), true, Some("")));
    assert_eq!((read("mirrored", "c-table").as_deref(), read("rotation", "c-table").as_deref()), (Some("false"), Some("0")));
    assert_eq!(read("name", "missing"), None);
}

#[test]
fn an_edited_component_value_is_exactly_one_sparse_set_component() {
    let snapshot = placed();
    let sparse = |mutation: Option<ModelMutation>| match mutation {
        Some(ModelMutation::SetComponent(leaf)) => leaf,
        other => panic!("one set-component expected, got {other:?}"),
    };
    let leaf = sparse(write("component", "elevation", &snapshot, "c-table", " 0.8 "));
    assert_eq!((leaf.id.as_str(), leaf.elevation), ("c-table", Some(0.8)));
    assert!(leaf.rotation.is_none() && leaf.host.is_none() && leaf.system.is_none() && leaf.name.is_none());
    let leaf = sparse(write("component", "rotation", &snapshot, "c-table", "90"));
    assert!((leaf.rotation.expect("rotation") - std::f64::consts::FRAC_PI_2).abs() < 1e-12, "degrees become radians");
    let leaf = sparse(write("component", "position", &snapshot, "c-table", "2.5, -1"));
    assert_eq!(leaf.position, Some(Point2 { x: 2.5, y: -1.0 }));
    assert_eq!(sparse(write("component", "mirrored", &snapshot, "c-table", "true")).mirrored, Some(true));
    assert_eq!(sparse(write("component", "family", &snapshot, "c-table", " fam-basin ")).family.as_deref(), Some("fam-basin"));
}

#[test]
fn the_host_and_the_system_can_be_set_and_cleared() {
    let snapshot = placed();
    let sparse = |mutation: Option<ModelMutation>| match mutation {
        Some(ModelMutation::SetComponent(leaf)) => leaf,
        other => panic!("one set-component expected, got {other:?}"),
    };
    assert_eq!(sparse(write("component", "host", &snapshot, "c-table", "wall-1")).host, Some(Assigned::new(Some("wall-1".to_string()))));
    assert_eq!(sparse(write("component", "host", &snapshot, "c-basin", "")).host, Some(Assigned::new(None)), "an empty host releases the component");
    assert_eq!(sparse(write("component", "system", &snapshot, "c-table", "supply")).system, Some(Assigned::new(Some(MepSystem::Supply))));
    assert_eq!(sparse(write("component", "system", &snapshot, "c-table", "")).system, Some(Assigned::new(None)), "an empty system makes it a plain component again");
    assert!(write("component", "system", &snapshot, "c-table", "steam").is_none());
}

#[test]
fn text_that_names_no_value_is_no_write() {
    let snapshot = placed();
    for (key, text) in [("elevation", "high"), ("rotation", "x"), ("position", "1"), ("position", "a, b"), ("mirrored", "maybe"), ("family", "  ")] {
        assert!(write("component", key, &snapshot, "c-table", text).is_none(), "{key} {text:?} writes nothing");
    }
}

#[test]
fn an_override_is_written_canonical_and_deleted_by_its_key() {
    let mut snapshot = placed();
    snapshot.component_overrides.insert("c-table.width".into(), crate::ComponentOverride { component: "c-table".into(), name: "width".into(), value: "2 m".into() });
    let leaf = write("component-override", "value", &snapshot, "c-table.width", "depth*2");
    assert!(matches!(leaf, Some(ModelMutation::SetComponentOverride(leaf)) if leaf.component == "c-table" && leaf.name == "width" && leaf.value == "depth * 2"));
    assert!(write("component-override", "value", &snapshot, "c-table.width", "1 m +").is_none());
    assert!(write("component-override", "value", &snapshot, "c-table.gone", "1 m").is_none());
    let delete = kind_of("component-override").and_then(|row| row.delete).expect("delete");
    assert!(matches!(delete("c-table.width"), ModelMutation::RemoveComponentOverride(leaf) if leaf.component == "c-table" && leaf.name == "width"));
    assert!(kind_of("component-override").expect("override").fields.iter().filter(|field| field.write.is_none()).count() == 2, "the component and the parameter are fixed");
}

#[test]
fn a_new_component_is_the_first_placeable_family_beside_the_ones_on_its_storey() {
    let snapshot = placed();
    let storey = snapshot.components["c-table"].storey.clone();
    let Ok(ModelMutation::CreateComponent(create)) = create_component(&snapshot, "c-new", &storey, "Table 2") else { panic!("a create-component") };
    assert_eq!((create.id.as_str(), create.component.family.as_str(), create.component.name.as_str()), ("c-new", "fam-table", "Table 2"));
    assert_eq!((create.component.position, create.component.elevation, create.component.host, create.component.system), (Point2 { x: 2.0, y: 0.0 }, 0.0, None, None));
    assert_eq!(create_component(&snapshot, "c-new", "nowhere", "x").err(), Some("bim.create.storey-missing"));
    let mut bare = snapshot.clone();
    bare.families.retain(|_, family| family.category == FamilyCategory::Profile);
    assert_eq!(create_component(&bare, "c-new", &storey, "x").err(), Some("bim.create.component-family-missing"));
}

#[test]
fn what_a_component_shows_is_inferred_and_a_component_counts_its_overrides() {
    let mut snapshot = placed();
    snapshot.component_overrides.insert("c-table.width".into(), crate::ComponentOverride { component: "c-table".into(), name: "width".into(), value: "2 m".into() });
    let inference = ModelInference::infer(&snapshot).expect("infers");
    let read = |rows: &[InferredRow], key: &str, id: &str| (rows.iter().find(|row| row.key == key).expect("row").read)(&snapshot, &inference, id);
    assert_eq!(read(COMPONENT_INFERRED, "category", "c-table").as_deref(), Some("Furniture"));
    assert_eq!((read(COMPONENT_INFERRED, "overrides", "c-table").as_deref(), read(COMPONENT_INFERRED, "overrides", "c-basin").as_deref(), read(COMPONENT_INFERRED, "overrides", "ghost")), (Some("1"), Some("0"), None));
    assert!(read(COMPONENT_INFERRED, "issues", "c-table").is_some());
    assert_eq!(read(COMPONENT_OVERRIDE_INFERRED, "family_formula", "c-table.width").as_deref(), Some("1.6 m"));
}

#[test]
fn the_angle_is_shown_in_degrees_without_the_noise_of_the_conversion() {
    assert_eq!(degrees_text(0.0), "0");
    assert_eq!(degrees_text(std::f64::consts::PI / 12.0), "15");
    assert_eq!(degrees_text(-std::f64::consts::FRAC_PI_2), "-90");
}
