use super::testkit::{document, document_in, house, psets, quantity, rows, string, tags};
use super::*;
use crate::standards::v1::subsets::any::io::import::ifc::{import_ifc2x3, import_ifc4};
use protocol::Inference;
use semio_s_artifact_stdio_ifc::part21::Part21Value;

fn house4() -> Part21Document {
    document_in(Schema::Ifc4, &house())
}

fn names(document: &Part21Document) -> BTreeSet<String> {
    document.instances.iter().filter_map(|instance| instance.primary().map(|(name, _)| name.to_string())).collect()
}

use std::collections::BTreeSet;

#[test]
fn the_house_exports_to_ifc4_without_skipping_anything() {
    let (bytes, notes) = export_ifc4(&house()).expect("the house exports");
    assert!(notes.is_empty(), "nothing is skipped: {notes:?}");
    assert!(bytes.starts_with(b"ISO-10303-21"));
    let decoded = codec::decode_ifc4(&bytes).expect("the reader decodes it");
    assert_eq!(decoded.instances, house4().instances);
    assert!(codec::decode_document(&bytes).is_err(), "the 2x3 reader refuses an IFC4 file");
}

#[test]
fn an_ifc4_file_declares_ifc4_and_writes_no_owner_history() {
    let document = house4();
    assert!(document.header.file_schema.iter().any(|value| value.as_list().is_some_and(|items| items.iter().any(|item| item.as_str() == Some("IFC4")))));
    assert_eq!(document.by_type("IFCOWNERHISTORY").count(), 0);
    assert_eq!(document.by_type("IFCPERSON").count(), 0);
    assert!(rows(&document, "IFCWALL").iter().all(|(_, args)| args[1].is_unset()));
}

#[test]
fn the_ifc4_export_is_deterministic_byte_for_byte_and_the_staged_job_writes_the_same_file() {
    let model = house();
    assert_eq!(export_ifc4(&model).expect("first").0, export_ifc4(&model).expect("second").0);
    let inferred = ModelInference::infer(&model).expect("the house infers");
    for schema in [Schema::Ifc2x3, Schema::Ifc4] {
        let mut staged = StagedExport::new(schema, &model);
        let mut steps = 0;
        let (document, notes) = loop {
            steps += 1;
            assert!(staged.fraction() < 1.0);
            if let Some(done) = staged.step(&model, &inferred) {
                break done;
            }
        };
        assert_eq!(steps, STAGES.len());
        assert_eq!((document, notes), inferred_to_part21(schema, &model, &inferred));
    }
}

#[test]
fn every_ifc4_product_has_the_attribute_count_of_its_schema_entity() {
    let document = house4();
    let widths = [
        ("IFCWALL", 9),
        ("IFCWALLSTANDARDCASE", 9),
        ("IFCSLAB", 9),
        ("IFCCOLUMN", 9),
        ("IFCBEAM", 9),
        ("IFCROOF", 9),
        ("IFCSTAIR", 9),
        ("IFCSTAIRFLIGHT", 13),
        ("IFCRAILING", 9),
        ("IFCCURTAINWALL", 9),
        ("IFCMEMBER", 9),
        ("IFCPLATE", 9),
        ("IFCOPENINGELEMENT", 9),
        ("IFCWINDOW", 13),
        ("IFCDOOR", 13),
        ("IFCSPACE", 11),
        ("IFCGRID", 11),
        ("IFCBUILDINGSTOREY", 10),
        ("IFCSITE", 14),
        ("IFCBUILDING", 12),
        ("IFCPROJECT", 9),
    ];
    for (entity, width) in widths {
        let found = rows(&document, entity);
        assert!(!found.is_empty() || matches!(entity, "IFCSTAIRFLIGHT" | "IFCMEMBER" | "IFCPLATE"), "{entity} is written");
        for (instance, args) in found {
            assert_eq!(args.len(), width, "{entity} #{}", instance.id);
        }
    }
}

#[test]
fn the_ifc4_predefined_types_name_the_kind_of_every_element() {
    let document = house4();
    let predefined = |entity: &str, at: usize| -> BTreeSet<String> { rows(&document, entity).iter().filter_map(|(_, args)| args.get(at).and_then(Part21Value::as_enum)).map(str::to_string).collect() };
    assert_eq!(predefined("IFCWALL", 8), BTreeSet::from(["STANDARD".to_string()]));
    assert_eq!(predefined("IFCCOLUMN", 8), BTreeSet::from(["COLUMN".to_string()]));
    assert_eq!(predefined("IFCBEAM", 8), BTreeSet::from(["BEAM".to_string()]));
    assert_eq!(predefined("IFCOPENINGELEMENT", 8), BTreeSet::from(["OPENING".to_string()]));
    assert_eq!(predefined("IFCWINDOW", 10), BTreeSet::from(["WINDOW".to_string()]));
    assert_eq!(predefined("IFCDOOR", 10), BTreeSet::from(["DOOR".to_string()]));
}

#[test]
fn door_and_window_types_replace_the_styles_and_carry_the_operation_and_partitioning() {
    let model = house();
    let document = house4();
    assert_eq!(document.by_type("IFCDOORSTYLE").count() + document.by_type("IFCWINDOWSTYLE").count(), 0);
    assert_eq!(document.by_type("IFCDOORTYPE").count(), model.door_types.len());
    assert_eq!(document.by_type("IFCWINDOWTYPE").count(), model.window_types.len());
    for (id, kind) in &model.door_types {
        let (_, args) = rows(&document, "IFCDOORTYPE").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some(id.as_str())).expect("the door type");
        assert_eq!(args[10].as_enum(), Some(super::data::door_operation(kind)), "{id}");
        assert_eq!(args[9].as_enum(), Some("DOOR"));
    }
    for (id, kind) in &model.window_types {
        let (_, args) = rows(&document, "IFCWINDOWTYPE").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some(id.as_str())).expect("the window type");
        assert_eq!(args[10].as_enum(), Some(super::data::window_partitioning(kind.panes)), "{id}");
    }
    for (_, args) in rows(&document, "IFCDOOR") {
        let tag = string(args, 7).expect("a tag");
        let opening = &model.openings[&tag];
        let crate::OpeningKind::Door { door_type } = &opening.kind else { panic!("a door") };
        assert_eq!(args[11].as_enum(), Some(super::data::door_operation(&model.door_types[door_type])), "{tag}: the occurrence repeats the operation of its type");
    }
}

#[test]
fn roof_types_are_ifc_roof_types_in_ifc4_and_proxy_types_in_ifc2x3() {
    let model = house();
    assert_eq!(house4().by_type("IFCROOFTYPE").count(), model.roof_types.len());
    assert_eq!(house4().by_type("IFCBUILDINGELEMENTPROXYTYPE").count(), 0);
    assert_eq!(document(&model).by_type("IFCROOFTYPE").count(), 0);
    assert_eq!(document(&model).by_type("IFCBUILDINGELEMENTPROXYTYPE").count(), model.roof_types.len());
}

#[test]
fn ifc4_bodies_of_meshes_are_triangulated_face_sets_and_never_faceted_breps() {
    let document = house4();
    assert_eq!(document.by_type("IFCFACETEDBREP").count(), 0);
    assert!(document.by_type("IFCTRIANGULATEDFACESET").count() > 0);
    assert_eq!(document.by_type("IFCTRIANGULATEDFACESET").count(), document.by_type("IFCCARTESIANPOINTLIST3D").count());
    for (_, args) in rows(&document, "IFCTRIANGULATEDFACESET") {
        let triangles = args[3].as_list().expect("indices");
        assert!(triangles.iter().all(|triangle| triangle.as_list().is_some_and(|corners| corners.len() == 3)));
        assert!(!triangles.is_empty());
    }
    assert!(document.by_type("IFCFACETEDBREP").count() < document_in(Schema::Ifc2x3, &house()).by_type("IFCFACETEDBREP").count());
}

#[test]
fn ifc4_quantities_carry_the_formula_slot_and_equal_the_2x3_values() {
    let model = house();
    let (four, two) = (house4(), document(&model));
    for (_, args) in rows(&four, "IFCQUANTITYLENGTH") {
        assert_eq!(args.len(), 5);
        assert!(args[4].is_unset());
    }
    for (id, set, name) in [("w-south", "Qto_WallBaseQuantities", "NetVolume"), ("c-1", "Qto_ColumnBaseQuantities", "GrossVolume"), ("sl-1", "Qto_SlabBaseQuantities", "GrossArea")] {
        if model.walls.contains_key(id) || model.columns.contains_key(id) || model.slabs.contains_key(id) {
            assert_eq!(quantity(&four, id, set, name), quantity(&two, id, set, name), "{id}.{name}");
        }
    }
}

#[test]
fn ifc4_materials_carry_the_category_and_layers_the_function() {
    let model = house();
    let document = house4();
    assert_eq!(rows(&document, "IFCMATERIAL").len(), model.materials.len());
    for (_, args) in rows(&document, "IFCMATERIAL") {
        assert_eq!(args.len(), 3);
        assert!(args[2].as_str().is_some_and(|category| ["Concrete", "Masonry", "Wood", "Metal", "Glass", "Insulation", "Finish", "Membrane", "Other"].contains(&category)));
    }
    for (_, args) in rows(&document, "IFCMATERIALLAYER") {
        assert_eq!(args.len(), 7);
        assert!(args[5].as_str().is_some());
    }
    assert_eq!(document.by_type("IFCEXTENDEDMATERIALPROPERTIES").count(), 0);
    assert!(document.by_type("IFCMATERIALPROPERTIES").count() >= model.materials.len() * 3);
}

#[test]
fn the_psets_file_carries_the_templates_and_the_classification_chains_in_one_ifc4_file() {
    let document = document_in(Schema::Ifc4, &psets());
    assert_eq!(document.by_type("IFCPROJECTLIBRARY").count(), 1);
    assert_eq!(document.by_type("IFCPROPERTYSETTEMPLATE").count(), psets().property_templates.len());
    assert!(document.by_type("IFCRELDEFINESBYTEMPLATE").count() >= 1);
    for (_, args) in rows(&document, "IFCRELDEFINESBYTEMPLATE") {
        assert!(args[4].as_list().is_some_and(|sets| !sets.is_empty()));
        assert!(document.resolve(&args[5]).is_some_and(|template| template.is_type("IFCPROPERTYSETTEMPLATE")));
    }
    assert!(!names(&document).contains("IFCEXTENDEDMATERIALPROPERTIES"));
    assert_eq!(document.by_type("IFCPROPERTYSET").filter(|set| set.entity("IFCPROPERTYSET").is_some_and(|args| string(args, 2).as_deref() == Some("Semio_ClassificationParents"))).count(), 0, "IFC4 chains the parents instead");
}

fn model_after(schema: Schema, model: &crate::ModelSnapshot) -> crate::ModelSnapshot {
    match schema {
        Schema::Ifc2x3 => import_ifc2x3(&export_ifc2x3(model).expect("2x3 export").0).expect("2x3 import").0,
        Schema::Ifc4 => import_ifc4(&export_ifc4(model).expect("4 export").0).expect("4 import").0,
    }
}

#[test]
fn export_import_export_is_byte_stable_for_the_whole_house_in_both_schemas() {
    let model = house();
    for schema in [Schema::Ifc2x3, Schema::Ifc4] {
        let first = encode(schema, model_to_part21(schema, &model).expect("first").0).expect("first bytes");
        let back = model_after(schema, &model);
        let second = encode(schema, model_to_part21(schema, &back).expect("second").0).expect("second bytes");
        assert_eq!(String::from_utf8_lossy(&first).lines().count(), String::from_utf8_lossy(&second).lines().count(), "{schema:?}");
        assert!(first == second, "{schema:?}: the second export differs from the first");
    }
}

#[test]
fn export_import_export_is_byte_stable_for_the_psets_model_with_templates_and_classification_chains() {
    let model = psets();
    let first = export_ifc4(&model).expect("first").0;
    let (back, _) = import_ifc4(&first).expect("the import");
    assert_eq!(back.property_templates, model.property_templates);
    assert_eq!(back.classification_systems, model.classification_systems);
    assert_eq!(back.classifications, model.classifications);
    assert_eq!(first, export_ifc4(&back).expect("second").0);
}

#[test]
fn the_ifc4_import_of_the_house_gives_the_authored_families_back() {
    let model = house();
    let back = model_after(Schema::Ifc4, &model);
    assert_eq!(back.walls.keys().collect::<Vec<_>>(), model.walls.keys().collect::<Vec<_>>());
    assert_eq!((&back.roofs, &back.stairs, &back.railings, &back.curtain_walls), (&model.roofs, &model.stairs, &model.railings, &model.curtain_walls));
    assert_eq!(back.door_types, model.door_types);
    assert_eq!(back.window_types, model.window_types);
    assert_eq!(back.roof_types, model.roof_types);
    assert_eq!(back.project.author, model.project.author);
    assert_eq!(back.project.organization, model.project.organization);
}

#[test]
fn the_written_length_of_a_column_is_the_vertical_extent_of_its_inferred_solid() {
    let model = house();
    let column = model.columns.iter().find(|(_, row)| row.tilt.is_none()).map(|(id, _)| id.clone()).expect("a plumb column");
    let inferred = ModelInference::infer(&model).expect("infers");
    let solid = &inferred.element_solids[&column];
    let (base, top) = (solid.bounds.min.z, solid.bounds.max.z);
    let document = document(&model);
    let (_, args) = rows(&document, "IFCCOLUMN").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some(column.as_str())).expect("the column");
    let length = quantity(&document, &column, "Qto_ColumnBaseQuantities", "Length").expect("the length");
    assert!((length - (top - base)).abs() < 1e-9, "{length} against {base}..{top}");
    assert!(args[5].as_ref_id().is_some());
}

#[test]
fn tags_survive_in_both_schemas() {
    for schema in [Schema::Ifc2x3, Schema::Ifc4] {
        assert_eq!(tags(&document_in(schema, &house()), "IFCCOLUMN"), tags(&document(&house()), "IFCCOLUMN"));
    }
}

//#region 🔖️Fixtures
const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures");

const ASSETS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");

fn decoded(file: &str) -> crate::ModelSnapshot {
    let text = std::fs::read_to_string(file).unwrap_or_else(|error| panic!("{file}: {error}"));
    semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{file} decodes: {error:?}"))
}

fn snapshot_at(path: &str) -> crate::ModelSnapshot {
    decoded(&format!("{FIXTURES}/{path}/🔣️.json"))
}

/// 🗂️ The IFC4 fixture of a case: its folder below `🏢️ifc4` and the snapshot it is written from.
fn case(name: &str) -> (&'static str, crate::ModelSnapshot) {
    match name {
        "house" => ("🏠️house", house()),
        "psets" => ("🏷️psets", psets()),
        "ceilings" => ("🔲️ceilings", snapshot_at("🏗️ifc/🔲️ceilings/📸️snapshot")),
        "notated" => ("🪧️notated", snapshot_at("💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot")),
        "ramps" => ("🛝️ramps", snapshot_at("💡️inferences/🛝️ramp-runs/🏞️ramps/📸️snapshot")),
        "wall-depth" => ("🧗️wall-depth", snapshot_at("💡️inferences/🧗️wall-depth/🏠️attic/📸️snapshot")),
        "example-house" => ("🏡️example-house", decoded(&format!("{ASSETS}/🏡️house/📸️snapshot.json"))),
        "example-office" => ("🏢️example-office", decoded(&format!("{ASSETS}/🏢️office/📸️snapshot.json"))),
        other => panic!("no IFC4 fixture {other}"),
    }
}

fn file_of(folder: &str) -> String {
    let stem = folder.trim_start_matches(|c: char| !c.is_ascii());
    let emoji = &folder[..folder.len() - stem.len()];
    format!("{FIXTURES}/🏢️ifc4/{folder}/{emoji}{stem}.ifc")
}

fn committed_export(name: &str) {
    let (folder, model) = case(name);
    let (bytes, notes) = export_ifc4(&model).unwrap_or_else(|error| panic!("{name}: {error}"));
    assert!(notes.iter().all(|note| !note.is_empty()), "{name}: {notes:?}");
    let file = file_of(folder);
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::create_dir_all(std::path::Path::new(&file).parent().expect("a folder")).expect("the fixture folder");
        std::fs::write(&file, &bytes).expect("the file is written");
    }
    let committed = std::fs::read(&file).unwrap_or_else(|error| panic!("{file}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export-bim-1-ifc4 case."));
    assert!(committed == bytes, "{name}: the committed IFC4 export drifted: rewrite it with BIM_BLESS=1");
}

fn oracle_table(name: &str) {
    use crate::standards::v1::subsets::any::io::export::ifc::projection::projection_in;
    let (folder, model) = case(name);
    let measured = format!("{FIXTURES}/🏢️ifc4/{folder}/🔬️measure/🔣️.json");
    let oracle: serde_json::Value = serde_json::from_slice(&std::fs::read(&measured).unwrap_or_else(|error| panic!("{measured}: {error}. Run `python 🐍️.py write` of the export-bim-1-ifc4 case."))).expect("the oracle table");
    let report: serde_json::Value = serde_json::from_str(&projection_in(Schema::Ifc4, &model).to_json()).expect("the report parses");
    assert_eq!(oracle["schema"], "IFC4", "{name}");
    for key in ["counts", "containment", "classifications", "type_properties", "annotations"] {
        assert_eq!(oracle[key], report[key], "{name}: {key}");
    }
    let (kernel, written) = (oracle["volumes"].as_object().expect("volumes"), report["volumes"].as_object().expect("volumes"));
    assert_eq!(kernel.keys().collect::<Vec<_>>(), written.keys().collect::<Vec<_>>(), "{name}");
    for (tag, volume) in written {
        let (a, b) = (kernel[tag].as_f64().expect("a number"), volume.as_f64().expect("a number"));
        assert!((a - b).abs() < 1e-8 * b.abs().max(1.0), "{name} {tag}: kernel {a}, written {b}");
    }
}

macro_rules! fixture_tests {
    ($($export:ident, $table:ident => $case:literal;)+) => {
        $(
            #[test]
            fn $export() {
                committed_export($case);
            }

            #[test]
            fn $table() {
                oracle_table($case);
            }
        )+
    };
}

fixture_tests! {
    the_committed_ifc4_house_is_the_current_export, the_ifc4_house_report_equals_the_ifcopenshell_table => "house";
    the_committed_ifc4_psets_file_is_the_current_export, the_ifc4_psets_report_equals_the_ifcopenshell_table => "psets";
    the_committed_ifc4_ceilings_file_is_the_current_export, the_ifc4_ceilings_report_equals_the_ifcopenshell_table => "ceilings";
    the_committed_ifc4_notated_file_is_the_current_export, the_ifc4_notated_report_equals_the_ifcopenshell_table => "notated";
    the_committed_ifc4_ramps_file_is_the_current_export, the_ifc4_ramps_report_equals_the_ifcopenshell_table => "ramps";
    the_committed_ifc4_wall_depth_file_is_the_current_export, the_ifc4_wall_depth_report_equals_the_ifcopenshell_table => "wall-depth";
    the_committed_ifc4_example_house_is_the_current_export, the_ifc4_example_house_report_equals_the_ifcopenshell_table => "example-house";
    the_committed_ifc4_example_office_is_the_current_export, the_ifc4_example_office_report_equals_the_ifcopenshell_table => "example-office";
}
//#endregion 🔖️Fixtures
