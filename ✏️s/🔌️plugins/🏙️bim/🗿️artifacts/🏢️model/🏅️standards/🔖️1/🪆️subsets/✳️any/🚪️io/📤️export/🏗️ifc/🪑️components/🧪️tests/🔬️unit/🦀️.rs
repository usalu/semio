use super::*;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::{components, count, document, document_in, quantity, rows, string, tags, target};
use crate::standards::v1::subsets::any::io::export::ifc::Schema;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

#[test]
fn every_component_is_the_class_of_its_family_category_in_ifc_2x3() {
    let document = document(&components());
    assert_eq!(tags(&document, "IFCFURNISHINGELEMENT"), ["cmp-chair-free", "cmp-chair-mirrored", "cmp-table", "cmp-table-first"]);
    assert_eq!(tags(&document, "IFCFLOWTERMINAL"), ["cmp-basin", "cmp-diffuser", "cmp-lamp", "cmp-panel", "cmp-wc"]);
    assert_eq!(tags(&document, "IFCBUILDINGELEMENTPROXY"), ["cmp-fan", "cmp-kitchen", "cmp-pillar"]);
}

#[test]
fn every_component_is_the_class_of_its_family_category_in_ifc4() {
    let document = document_in(Schema::Ifc4, &components());
    assert_eq!(count(&document, "IFCFURNITURE"), 4);
    assert_eq!(tags(&document, "IFCSANITARYTERMINAL"), ["cmp-basin", "cmp-wc"]);
    assert_eq!(tags(&document, "IFCLIGHTFIXTURE"), ["cmp-lamp"]);
    assert_eq!(tags(&document, "IFCFLOWTERMINAL"), ["cmp-diffuser", "cmp-panel"]);
    assert_eq!(count(&document, "IFCBUILDINGELEMENTPROXY"), 3);
}

#[test]
fn a_family_becomes_one_type_object_per_kind_of_occurrence_that_carries_the_authored_family() {
    let document = document(&components());
    assert_eq!(tags(&document, "IFCFURNITURETYPE"), ["fam-chair", "fam-table"]);
    assert_eq!(tags(&document, "IFCSANITARYTERMINALTYPE"), ["fam-basin", "fam-wc"]);
    assert_eq!(tags(&document, "IFCLIGHTFIXTURETYPE"), ["fam-lamp"]);
    assert_eq!(tags(&document, "IFCFLOWTERMINALTYPE"), ["fam-diffuser", "fam-panel"]);
    assert_eq!(tags(&document, "IFCBUILDINGELEMENTPROXYTYPE"), ["fam-kitchen", "fam-pillar", "fam-rig"]);
    let (_, table) = rows(&document, "IFCFURNITURETYPE").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("fam-table")).expect("the table type");
    assert_eq!(string(table, 2).as_deref(), Some("Table"));
    let typed: Vec<_> = rows(&document, "IFCRELDEFINESBYTYPE").into_iter().filter(|(_, relation)| target(&document, relation, 5).and_then(|kind| kind.entity("IFCFURNITURETYPE")).is_some_and(|kind| string(kind, 7).as_deref() == Some("fam-table"))).collect();
    assert_eq!(typed.len(), 1);
    assert_eq!(typed[0].1[4].as_list().map(<[_]>::len), Some(2));
}

#[test]
fn the_instance_carries_its_record_its_overrides_and_its_base_quantities() {
    let model = components();
    let document = document(&model);
    let (_, args) = rows(&document, "IFCFURNISHINGELEMENT").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("cmp-table")).expect("the table");
    assert!(args[6].as_ref_id().is_some(), "the table carries a body");
    let volume = quantity(&document, "cmp-table", QUANTITY_SET, "GrossVolume").expect("a volume");
    let top = 1.8 * 0.9 * 0.03;
    let legs = 4.0 * 0.06 * 0.06 * (0.74 - 0.03);
    assert!((volume - (top + legs)).abs() < 1e-9, "{volume}");
    assert_eq!(quantity(&document, "cmp-table", QUANTITY_SET, "NetVolume"), Some(volume));
    let footprint = quantity(&document, "cmp-table", QUANTITY_SET, "GrossFootprintArea").expect("a footprint");
    assert!((footprint - 1.8 * 0.9).abs() < 1e-9, "{footprint}");
    let text = semio_s_artifact_stdio_ifc::part21::write_part21(&document);
    assert!(text.contains("Semio_ComponentOverrides"));
    assert!(text.contains("'1.8 m'") && text.contains("'0.9 m'"));
    let record = semio_framework_pack_json::to_json_string(&model.components["cmp-table"]);
    assert!(text.contains(&record.replace('\'', "''")), "the authored record is written once");
}

#[test]
fn a_component_without_overrides_writes_no_override_set() {
    let text = semio_s_artifact_stdio_ifc::part21::write_part21(&document(&components()));
    assert_eq!(text.matches("Semio_ComponentOverrides").count(), 3, "the table, the mirrored chair and the kitchen unit");
}

#[test]
fn the_body_is_in_the_frame_of_the_instance_and_the_placement_is_the_inferred_one() {
    let model = components();
    let document = document(&model);
    let (_, args) = rows(&document, "IFCFURNISHINGELEMENT").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("cmp-chair-free")).expect("the chair");
    let placement = target(&document, args, 5).and_then(|place| place.entity("IFCLOCALPLACEMENT")).expect("a placement");
    let axis = target(&document, placement, 1).and_then(|axis| axis.entity("IFCAXIS2PLACEMENT3D")).expect("an axis placement");
    let origin = target(&document, axis, 0).and_then(|point| point.entity("IFCCARTESIANPOINT")).expect("a point");
    let at: Vec<f64> = origin[0].as_list().expect("coordinates").iter().filter_map(|value| value.as_real()).collect();
    assert_eq!(at.len(), 3);
    assert!((at[0] - 3.0).abs() < 1e-9 && (at[1] - 3.0).abs() < 1e-9 && at[2].abs() < 1e-9, "{at:?}");
    let reference = target(&document, axis, 2).and_then(|direction| direction.entity("IFCDIRECTION")).expect("a reference direction");
    let direction: Vec<f64> = reference[0].as_list().expect("ratios").iter().filter_map(|value| value.as_real()).collect();
    assert!(direction[0].abs() < 1e-9 && (direction[1] - 1.0).abs() < 1e-9, "a quarter turn: {direction:?}");
}

#[test]
fn every_routed_element_is_a_flow_segment_typed_by_its_section() {
    let document = document(&components());
    assert_eq!(tags(&document, "IFCFLOWSEGMENT"), ["mep-gas", "mep-lighting", "mep-return", "mep-supply", "mep-tray", "mep-waste", "mep-water"]);
    assert_eq!(count(&document, "IFCDUCTSEGMENTTYPE"), 2);
    assert_eq!(count(&document, "IFCPIPESEGMENTTYPE"), 3);
    assert_eq!(count(&document, "IFCCABLECARRIERSEGMENTTYPE"), 2);
    let ifc4 = document_in(Schema::Ifc4, &components());
    assert_eq!(tags(&ifc4, "IFCDUCTSEGMENT"), ["mep-return", "mep-supply"]);
    assert_eq!(tags(&ifc4, "IFCPIPESEGMENT"), ["mep-gas", "mep-waste", "mep-water"]);
    assert_eq!(tags(&ifc4, "IFCCABLECARRIERSEGMENT"), ["mep-lighting", "mep-tray"]);
}

#[test]
fn a_segment_carries_the_centre_line_the_quantities_and_the_record() {
    let model = components();
    let document = document(&model);
    assert!((quantity(&document, "mep-supply", "Qto_DuctSegmentBaseQuantities", "Length").expect("a length") - 5.3).abs() < 1e-9);
    assert!((quantity(&document, "mep-supply", "Qto_DuctSegmentBaseQuantities", "CrossSectionArea").expect("an area") - 0.1).abs() < 1e-9);
    assert!((quantity(&document, "mep-supply", "Qto_DuctSegmentBaseQuantities", "GrossVolume").expect("a volume") - 0.53).abs() < 1e-9);
    assert!((quantity(&document, "mep-water", "Qto_PipeSegmentBaseQuantities", "Length").expect("a length") - 2.75).abs() < 1e-9);
    let (_, args) = rows(&document, "IFCFLOWSEGMENT").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("mep-supply")).expect("the duct");
    let shape = target(&document, args, 6).and_then(|shape| shape.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("a shape");
    let axis = shape[2].as_list().expect("representations").iter().filter_map(|value| document.resolve(value)).filter_map(|instance| instance.entity("IFCSHAPEREPRESENTATION")).find(|args| string(args, 1).as_deref() == Some("Axis")).expect("an axis");
    assert_eq!(string(axis, 2).as_deref(), Some("Curve3D"));
    let polyline = axis[3].as_list().and_then(|items| items.first()).and_then(|item| document.resolve(item)).and_then(|item| item.entity("IFCPOLYLINE")).expect("a polyline");
    assert_eq!(polyline[0].as_list().map(<[_]>::len), Some(3));
    let record = semio_framework_pack_json::to_json_string(&model.mep_elements["mep-supply"]);
    let text = semio_s_artifact_stdio_ifc::part21::write_part21(&document);
    assert!(text.contains(&record.replace('\'', "''")));
    let back: crate::MepElement = from_json_str(&record, JsonMemberPolicy::Reject).expect("the record decodes");
    assert_eq!(back, model.mep_elements["mep-supply"]);
}

#[test]
fn every_service_in_use_is_one_system_that_groups_its_segments_and_terminals() {
    let model = components();
    let document = document(&model);
    assert_eq!(count(&document, "IFCSYSTEM"), 7);
    let mut grouped: Vec<(String, Vec<String>)> = Vec::new();
    for (_, relation) in rows(&document, "IFCRELASSIGNSTOGROUP") {
        let group = target(&document, relation, 6).and_then(|group| group.entity("IFCSYSTEM")).and_then(|group| string(group, 2)).expect("a named system");
        let mut members: Vec<String> = relation[4].as_list().expect("members").iter().filter_map(|member| document.resolve(member)).filter_map(|member| member.primary()).filter_map(|(_, args)| string(args, 7)).collect();
        members.sort();
        grouped.push((group, members));
    }
    grouped.sort();
    let wanted: Vec<(String, Vec<String>)> = [
        ("DomesticWater", vec!["mep-water"]),
        ("Gas", vec!["mep-gas"]),
        ("Lighting", vec!["cmp-lamp", "mep-lighting"]),
        ("Power", vec!["cmp-panel", "mep-tray"]),
        ("Return", vec!["mep-return"]),
        ("Supply", vec!["cmp-diffuser", "mep-supply"]),
        ("Waste", vec!["cmp-basin", "mep-waste"]),
    ]
    .into_iter()
    .map(|(name, members)| (name.to_string(), members.into_iter().map(str::to_string).collect()))
    .collect();
    assert_eq!(grouped, wanted);
    let ifc4 = document_in(Schema::Ifc4, &model);
    assert_eq!(count(&ifc4, "IFCDISTRIBUTIONSYSTEM"), 7);
}

#[test]
fn products_are_contained_in_their_storey_and_the_export_is_deterministic() {
    let model = components();
    let (first, _) = crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3(&model).expect("the export");
    let (second, _) = crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3(&model).expect("the second export");
    assert_eq!(first, second);
    let document = document(&model);
    let contained: Vec<(String, Vec<String>)> = rows(&document, "IFCRELCONTAINEDINSPATIALSTRUCTURE")
        .into_iter()
        .map(|(_, relation)| {
            let storey = target(&document, relation, 5).and_then(|storey| storey.entity("IFCBUILDINGSTOREY")).and_then(|storey| string(storey, 4)).unwrap_or_default();
            let mut members: Vec<String> = relation[4].as_list().expect("members").iter().filter_map(|member| document.resolve(member)).filter_map(|member| member.primary()).filter_map(|(_, args)| string(args, 7)).collect();
            members.sort();
            (storey, members)
        })
        .collect();
    let upper = contained.iter().find(|(storey, _)| storey == "st-first");
    assert_eq!(upper.map(|(_, members)| members.clone()), Some(vec!["cmp-table-first".to_string()]));
}

#[test]
fn a_component_of_a_missing_family_is_noted_and_not_written() {
    let mut model = components();
    model.families.remove("fam-pillar");
    model.family_parameters.retain(|_, row| row.family != "fam-pillar");
    model.family_solids.retain(|_, row| row.family != "fam-pillar");
    let (document, notes) = crate::standards::v1::subsets::any::io::export::ifc::model_to_part21(Schema::Ifc2x3, &model).expect("the export");
    assert!(notes.iter().any(|note| note.contains("cmp-pillar")), "{notes:?}");
    assert!(!tags(&document, "IFCBUILDINGELEMENTPROXY").contains(&"cmp-pillar".to_string()));
}

const COMPONENTS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🏗️ifc/🪑️components");

fn read_components(name: &str) -> Vec<u8> {
    std::fs::read(format!("{COMPONENTS_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."))
}

#[test]
fn the_committed_components_file_is_the_current_export() {
    let (bytes, notes) = crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3(&components()).expect("the components export");
    assert!(notes.is_empty(), "every component and run is written: {notes:?}");
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::write(format!("{COMPONENTS_DIR}/🪑️components.ifc"), &bytes).expect("the file is written");
    }
    assert_eq!(read_components("🪑️components.ifc"), bytes, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_components_subject_report_equals_the_table_the_ifcopenshell_oracle_measured_from_the_committed_file() {
    use crate::standards::v1::subsets::any::io::export::ifc::projection::projection;
    let oracle: serde_json::Value = serde_json::from_slice(&read_components("🔬️measure/🔣️.json")).expect("the oracle table");
    let ours: serde_json::Value = serde_json::from_str(&projection(&components()).to_json()).expect("the report parses");
    assert_eq!(oracle["schema"], ours["schema"]);
    assert_eq!(oracle["counts"], ours["counts"]);
    assert_eq!(oracle["containment"], ours["containment"]);
    assert_eq!(oracle["volumes"], ours["volumes"]);
}
