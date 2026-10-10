use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, real, rows, string};
use super::report_json;
use crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3;
use crate::{AreaMeasure, AreaScheme, ModelInference, ModelSnapshot, Zone};
use protocol::Inference;
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Instance};
use std::collections::BTreeMap;

/// 🏘️ The house with two zones, two area schemes and finished spaces: the living room in the day zone, the bedroom in the night zone.
pub fn zoned() -> ModelSnapshot {
    let mut model = house();
    model.zones.insert("z-day".into(), Zone { name: "Day zone".into(), category: "Ventilation".into(), occupancy_density: 0.1 });
    model.zones.insert("z-night".into(), Zone { name: "Night zone".into(), category: "Fire compartment".into(), occupancy_density: 0.05 });
    model.zones.insert("z-empty".into(), Zone { name: "Empty zone".into(), category: "Tenant".into(), occupancy_density: 0.0 });
    let living = model.spaces.get_mut("sp-1").expect("the living room");
    (living.zone, living.floor_finish, living.wall_finish, living.ceiling_finish) = (Some("z-day".into()), Some("m-wood".into()), Some("m-brick".into()), Some("m-conc".into()));
    let bedroom = model.spaces.get_mut("sp-2").expect("the bedroom");
    (bedroom.zone, bedroom.floor_finish) = (Some("z-night".into()), Some("m-wood".into()));
    model.area_schemes.insert("as-nsa".into(), AreaScheme { name: "Net use area".into(), measure: AreaMeasure::Net, usages: vec!["living".into(), "bedroom".into()], zones: Vec::new() });
    model.area_schemes.insert("as-day".into(), AreaScheme { name: "Day zone area".into(), measure: AreaMeasure::Gross, usages: Vec::new(), zones: vec!["z-day".into()] });
    model
}

fn instance_of<'a>(document: &'a Part21Document, entity: &'a str, name: &str) -> &'a Part21Instance {
    rows(document, entity).into_iter().find(|(_, args)| string(args, 2).as_deref() == Some(name)).map(|(instance, _)| instance).unwrap_or_else(|| panic!("no {entity} named {name}"))
}

fn space_named<'a>(document: &'a Part21Document, id: &str) -> &'a Part21Instance {
    rows(document, "IFCSPACE").into_iter().find(|(_, args)| string(args, 4).as_deref() == Some(id)).map(|(instance, _)| instance).unwrap_or_else(|| panic!("no space {id}"))
}

fn sets_of<'a>(document: &'a Part21Document, element: u64, set: &'a str) -> impl Iterator<Item = &'a Part21Instance> {
    document
        .by_type("IFCRELDEFINESBYPROPERTIES")
        .filter_map(|relation| relation.entity("IFCRELDEFINESBYPROPERTIES"))
        .filter(move |args| args[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id() == Some(element))))
        .filter_map(|args| document.resolve(&args[5]))
        .filter(move |definition| definition.entity("IFCPROPERTYSET").or_else(|| definition.entity("IFCELEMENTQUANTITY")).is_some_and(|args| string(args, 2).as_deref() == Some(set)))
}

fn properties(document: &Part21Document, element: u64, set: &str) -> BTreeMap<String, String> {
    sets_of(document, element, set)
        .filter_map(|definition| definition.entity("IFCPROPERTYSET"))
        .flat_map(|args| args[4].as_list().unwrap_or_default().iter().filter_map(|row| document.resolve(row)).filter_map(|row| row.entity("IFCPROPERTYSINGLEVALUE")).map(|row| (string(row, 0).unwrap_or_default(), row[2].as_typed().and_then(|(_, items)| items.first()).and_then(|item| item.as_str().map(str::to_string).or_else(|| item.as_real().map(|value| value.to_string()))).unwrap_or_default())).collect::<Vec<_>>())
        .collect()
}

fn quantity(document: &Part21Document, element: u64, set: &str, name: &str) -> Option<f64> {
    sets_of(document, element, set).filter_map(|definition| definition.entity("IFCELEMENTQUANTITY")).find_map(|args| args[5].as_list()?.iter().filter_map(|row| document.resolve(row)).find_map(|row| row.entities.iter().find(|(_, values)| string(values, 0).as_deref() == Some(name)).map(|(_, values)| real(values, 3))))
}

fn members(document: &Part21Document, group: u64) -> Vec<String> {
    let mut found: Vec<String> = rows(document, "IFCRELASSIGNSTOGROUP")
        .into_iter()
        .filter(|(_, args)| args[6].as_ref_id() == Some(group))
        .flat_map(|(_, args)| args[4].as_list().unwrap_or_default().iter().filter_map(|item| document.resolve(item)).filter_map(|space| space.entity("IFCSPACE").and_then(|values| string(values, 4))).collect::<Vec<_>>())
        .collect();
    found.sort();
    found
}

#[test]
fn a_zone_is_an_ifc_zone_named_by_its_name_typed_by_its_category_with_pset_zone_common() {
    let document = document(&zoned());
    assert_eq!(count(&document, "IFCZONE"), 3);
    for (name, category, id) in [("Day zone", "Ventilation", "z-day"), ("Night zone", "Fire compartment", "z-night"), ("Empty zone", "Tenant", "z-empty")] {
        let zone = instance_of(&document, "IFCZONE", name);
        assert_eq!(string(zone.entity("IFCZONE").expect("a zone"), 4).as_deref(), Some(category), "the category is the object type of {name}");
        assert_eq!(properties(&document, zone.id, "Pset_ZoneCommon").get("Reference").map(String::as_str), Some(id));
    }
}

#[test]
fn a_zone_groups_exactly_its_spaces_through_one_assignment_and_an_empty_zone_has_none() {
    let document = document(&zoned());
    let (day, night, empty) = (instance_of(&document, "IFCZONE", "Day zone").id, instance_of(&document, "IFCZONE", "Night zone").id, instance_of(&document, "IFCZONE", "Empty zone").id);
    assert_eq!(members(&document, day), ["sp-1"]);
    assert_eq!(members(&document, night), ["sp-2"]);
    assert!(members(&document, empty).is_empty());
    assert_eq!(count(&document, "IFCRELASSIGNSTOGROUP"), 2, "an empty zone writes no relation");
}

#[test]
fn an_area_scheme_is_a_group_of_type_area_scheme_carrying_its_rule_and_no_members() {
    let document = document(&zoned());
    let schemes: Vec<_> = rows(&document, "IFCGROUP").into_iter().filter(|(_, args)| string(args, 4).as_deref() == Some("AreaScheme")).collect();
    assert_eq!(schemes.len(), 2);
    let (net, args) = schemes.iter().find(|(_, args)| string(args, 2).as_deref() == Some("Net use area")).expect("the net scheme");
    assert_eq!(string(args, 2).as_deref(), Some("Net use area"));
    let rule = properties(&document, net.id, "Semio_Authoring");
    assert_eq!((rule["Id"].as_str(), rule["Measure"].as_str(), rule["Usages"].as_str(), rule["Zones"].as_str()), ("as-nsa", "Net", "[\"living\",\"bedroom\"]", "[]"));
    let (day, _) = schemes.iter().find(|(_, args)| string(args, 2).as_deref() == Some("Day zone area")).expect("the day scheme");
    let rule = properties(&document, day.id, "Semio_Authoring");
    assert_eq!((rule["Measure"].as_str(), rule["Zones"].as_str()), ("Gross", "[\"z-day\"]"));
    assert!(members(&document, net.id).is_empty() && members(&document, day.id).is_empty(), "a scheme counts by rule, it groups nothing");
}

#[test]
fn a_zone_reports_the_derived_totals_and_its_authored_density() {
    let model = zoned();
    let document = document(&model);
    let totals = ModelInference::infer(&model).expect("infers").zone_totals;
    let zone = instance_of(&document, "IFCZONE", "Day zone");
    assert!(totals["z-day"].net_area > 0.0, "the living room is a resolved member");
    for (name, expected) in [("GrossFloorArea", totals["z-day"].area), ("NetFloorArea", totals["z-day"].net_area), ("GrossVolume", totals["z-day"].volume), ("FloorFinishArea", totals["z-day"].floor_finish_area), ("WallFinishArea", totals["z-day"].wall_finish_area), ("CeilingFinishArea", totals["z-day"].ceiling_finish_area)] {
        let written = quantity(&document, zone.id, "Semio_ZoneTotals", name).unwrap_or_else(|| panic!("no {name}"));
        assert!((written - expected).abs() < 1e-6 * expected.abs().max(1.0), "{name}: written {written}, inferred {expected}");
    }
    assert_eq!(properties(&document, zone.id, "Semio_Authoring").get("OccupancyDensity").map(|density| density.parse::<f64>().expect("a number")), Some(0.1));
}

#[test]
fn a_finished_space_names_its_materials_in_pset_space_covering_requirements_and_keeps_their_ids() {
    let document = document(&zoned());
    let living = space_named(&document, "sp-1");
    let covering = properties(&document, living.id, "Pset_SpaceCoveringRequirements");
    assert_eq!((covering["FloorCovering"].as_str(), covering["WallCovering"].as_str(), covering["CeilingCovering"].as_str()), ("Oak", "Brick", "Concrete"));
    let authored = properties(&document, living.id, "Semio_Authoring");
    assert_eq!((authored["FloorFinish"].as_str(), authored["WallFinish"].as_str(), authored["CeilingFinish"].as_str()), ("m-wood", "m-brick", "m-conc"));
    let bedroom = properties(&document, space_named(&document, "sp-2").id, "Pset_SpaceCoveringRequirements");
    assert_eq!(bedroom.keys().map(String::as_str).collect::<Vec<_>>(), ["FloorCovering"], "an unfinished surface writes no row");
}

#[test]
fn a_model_without_zones_finishes_or_schemes_writes_none_of_it() {
    let mut model = house();
    model.zones.clear();
    model.area_schemes.clear();
    let document = document(&model);
    assert_eq!((count(&document, "IFCZONE"), count(&document, "IFCRELASSIGNSTOGROUP")), (0, 0));
    assert!(rows(&document, "IFCGROUP").iter().all(|(_, args)| string(args, 4).as_deref() != Some("AreaScheme")));
    assert!(rows(&document, "IFCPROPERTYSET").iter().all(|(_, args)| string(args, 2).as_deref() != Some("Pset_SpaceCoveringRequirements")));
    assert_eq!(count(&document, "IFCSPACE"), 2, "the rest of the house is still written");
}

/// 📁️ The committed zoned export: the snapshot, the IFC file written from it and the table IfcOpenShell measured from the file.
pub const ZONED_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🏗️ifc/🏘️zoned");

fn pretty(model: &ModelSnapshot) -> String {
    let value: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(model)).expect("the snapshot is JSON");
    serde_json::to_string_pretty(&value).expect("pretty JSON") + "
"
}

fn committed(name: &str) -> Vec<u8> {
    std::fs::read(format!("{ZONED_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the zoning export case."))
}

#[test]
fn the_committed_zoned_files_are_the_current_snapshot_and_export() {
    let model = zoned();
    let (text, file) = (pretty(&model), export_ifc2x3(&model).expect("the model exports").0);
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::create_dir_all(format!("{ZONED_DIR}/📸️snapshot")).expect("the fixture directory");
        std::fs::write(format!("{ZONED_DIR}/📸️snapshot/🔣️.json"), text.as_bytes()).expect("the snapshot is written");
        std::fs::write(format!("{ZONED_DIR}/🏘️zoned.ifc"), &file).expect("the file is written");
    }
    assert_eq!(String::from_utf8(committed("📸️snapshot/🔣️.json")).expect("UTF-8"), text, "the committed snapshot drifted: rewrite it with BIM_BLESS=1");
    assert_eq!(committed("🏘️zoned.ifc"), file, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_report_names_every_zone_scheme_and_covering_the_file_holds() {
    let report: serde_json::Value = serde_json::from_str(&report_json(&zoned()).expect("reports")).expect("JSON");
    assert_eq!(report["zones"]["z-day"]["members"], serde_json::json!(["sp-1"]));
    assert_eq!(report["zones"]["z-empty"]["members"], serde_json::json!([]));
    assert_eq!(report["schemes"]["as-nsa"]["usages"], serde_json::json!(["living", "bedroom"]));
    assert_eq!(report["coverings"]["sp-1"]["CeilingCovering"], "Concrete");
    assert!(report["coverings"].get("sp-3").is_none());
    assert!(report["zones"]["z-day"]["totals"]["NetFloorArea"].as_f64().is_some_and(|area| area > 0.0));
}
