#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::import::ifc::data::{label, type_authoring};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::import::ifc::frames::Rigid;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::import::ifc::reader::{opt_text, real, text, Doc, Section};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::import::ifc::spatial::{authoring_of, set_of, string_of};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::import::ifc::Import;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::export::ifc::components::{FAMILY_ROWS, RECORD_ROW};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::authored::formula;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::{Component, ComponentOverride, ExprPoint3, Family, FamilyCategory, FamilyParameter, FamilySolid, MepSystem, ParameterKind, Point2, SolidShape};
#[allow(unused_imports)]
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
#[allow(unused_imports)]
use semio_s_artifact_stdio_contract::part21::Part21Value;
#[allow(unused_imports)]
use std::collections::BTreeMap;
use crate::standards::v1::subsets::any::io::import::ifc::components::*;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::components;
use crate::standards::v1::subsets::any::io::export::ifc::{export_ifc2x3, export_ifc4};
use crate::standards::v1::subsets::any::io::import::ifc::{import_ifc2x3, import_ifc4};

#[test]
fn the_components_survive_the_round_trip_exactly_in_both_schemas() {
    let model = components();
    for (bytes, importer) in [(export_ifc2x3(&model).expect("2x3 export").0, import_ifc2x3 as fn(&[u8]) -> Result<(crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot, Vec<String>), String>), (export_ifc4(&model).expect("4 export").0, import_ifc4)] {
        let (back, notes) = importer(&bytes).expect("the file imports");
        assert!(notes.iter().all(|note| !note.contains("cmp-") && !note.contains("mep-")), "{notes:?}");
        assert_eq!(back.components, model.components);
        assert_eq!(back.component_overrides, model.component_overrides);
        for id in ["fam-table", "fam-chair", "fam-kitchen", "fam-wc", "fam-basin", "fam-lamp", "fam-diffuser", "fam-panel", "fam-pillar", "fam-rig"] {
            assert_eq!(back.families[id], model.families[id], "{id}");
        }
        assert_eq!(back.family_parameters, model.family_parameters);
        assert_eq!(back.family_solids, model.family_solids);
    }
}

#[test]
fn export_import_export_of_components_is_byte_stable() {
    let model = components();
    let (first, _) = export_ifc2x3(&model).expect("first export");
    let (back, _) = import_ifc2x3(&first).expect("the import");
    let (second, _) = export_ifc2x3(&back).expect("second export");
    assert_eq!(first, second);
}

#[test]
fn a_foreign_product_becomes_a_component_of_a_generic_cuboid_family() {
    let model = components();
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let text = String::from_utf8(bytes).expect("a text file").replace("'Component'", "'Other'").replace("'Family'", "'Other family'");
    let (back, notes) = import_ifc2x3(text.as_bytes()).expect("the file imports");
    assert_eq!(back.components.keys().collect::<Vec<_>>(), model.components.keys().collect::<Vec<_>>(), "{notes:?}");
    let table = &back.components["cmp-table"];
    let family = &back.families[&table.family];
    assert_eq!(family.name, "Table");
    assert_eq!(family.category, crate::standards::v1::subsets::any::schema::snapshot::FamilyCategory::Furniture);
    assert!(matches!(back.family_solids.values().find(|solid| solid.family == table.family).map(|solid| &solid.shape), Some(crate::standards::v1::subsets::any::schema::snapshot::SolidShape::Cuboid { .. })));
    assert!((table.position.x - 1.5).abs() < 1e-6 && (table.position.y - 1.5).abs() < 1e-6 && (table.rotation - 0.5).abs() < 1e-6, "{table:?}");
    assert_eq!(table.storey, "st-ground");
    assert_eq!(back.components["cmp-lamp"].system, Some(crate::standards::v1::subsets::any::schema::snapshot::MepSystem::Lighting));
    assert_eq!(back.components["cmp-diffuser"].system, Some(crate::standards::v1::subsets::any::schema::snapshot::MepSystem::Supply));
    assert_eq!(back.components["cmp-kitchen"].storey, "st-ground");
}

#[test]
fn the_system_of_a_group_is_read_from_its_name_or_predefined_type() {
    assert_eq!(system_named("Supply"), Some(crate::standards::v1::subsets::any::schema::snapshot::MepSystem::Supply));
    assert_eq!(system_named("Domestic Water"), Some(crate::standards::v1::subsets::any::schema::snapshot::MepSystem::DomesticWater));
    assert_eq!(system_named("WASTEWATER"), Some(crate::standards::v1::subsets::any::schema::snapshot::MepSystem::Waste));
    assert_eq!(system_named("ELECTRICAL"), Some(crate::standards::v1::subsets::any::schema::snapshot::MepSystem::Power));
    assert_eq!(system_named("unknown"), None);
}

#[test]
fn a_component_outside_a_storey_is_skipped() {
    let model = components();
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let text = String::from_utf8(bytes).expect("a text file").replace("'Component'", "'Other'");
    let (back, _) = import_ifc2x3(text.as_bytes()).expect("the file imports");
    assert!(back.components.values().all(|row| back.storeys.contains_key(&row.storey)));
}
