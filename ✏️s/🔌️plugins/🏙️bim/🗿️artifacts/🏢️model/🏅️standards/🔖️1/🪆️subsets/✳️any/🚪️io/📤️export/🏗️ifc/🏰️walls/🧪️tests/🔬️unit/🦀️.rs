use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, real, rows, string, tags, target};

#[test]
fn every_wall_is_written_with_its_id_as_tag_and_straight_unjoined_walls_are_standard_case() {
    let model = house();
    let document = document(&model);
    let mut written = tags(&document, "IFCWALLSTANDARDCASE");
    written.extend(tags(&document, "IFCWALL"));
    written.sort();
    assert_eq!(written, model.walls.keys().cloned().collect::<Vec<_>>());
    assert_eq!(tags(&document, "IFCWALLSTANDARDCASE"), ["w-first-south", "w-free"], "the two straight walls without a join");
    assert_eq!(count(&document, "IFCWALL"), 5, "four joined walls and the curved wall");
}

#[test]
fn a_standard_case_wall_is_a_rectangle_swept_up_to_its_height_along_a_two_point_axis() {
    let document = document(&house());
    let (_, wall) = rows(&document, "IFCWALLSTANDARDCASE").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("w-free")).expect("the free wall");
    let shape = target(&document, wall, 6).and_then(|instance| instance.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("the shape");
    let representations: Vec<_> = shape[2].as_list().expect("representations").iter().filter_map(|item| document.resolve(item)).filter_map(|item| item.entity("IFCSHAPEREPRESENTATION")).collect();
    assert_eq!(representations.iter().map(|args| string(args, 1).unwrap_or_default()).collect::<Vec<_>>(), ["Axis", "Body"]);
    let solid = document.resolve(&representations[1][3].as_list().expect("items")[0]).and_then(|item| item.entity("IFCEXTRUDEDAREASOLID")).expect("the solid");
    assert!((real(solid, 3) - 2.4).abs() < 1e-9, "Unconnected height 2.4");
    let profile = target(&document, solid, 0).and_then(|item| item.entity("IFCRECTANGLEPROFILEDEF")).expect("a rectangle");
    assert!((real(profile, 3) - 3.0).abs() < 1e-9 && (real(profile, 4) - 0.15).abs() < 1e-9, "length 3 by thickness 0.15");
}

#[test]
fn a_joined_or_curved_wall_extrudes_its_footprint_as_an_arbitrary_closed_profile() {
    let document = document(&house());
    let profiles = count(&document, "IFCARBITRARYCLOSEDPROFILEDEF");
    assert!(profiles >= 5, "one per IfcWall footprint, got {profiles}");
    assert!(count(&document, "IFCCOMPOSITECURVE") >= 1, "the curved wall has trimmed circles");
    assert!(count(&document, "IFCTRIMMEDCURVE") >= 2, "the arc axis and the curved faces");
}

#[test]
fn every_wall_gets_a_layer_set_usage_whose_offset_is_its_left_face_distance() {
    let model = house();
    let document = document(&model);
    let usages = rows(&document, "IFCMATERIALLAYERSETUSAGE");
    assert_eq!(usages.len(), model.walls.len() + model.slabs.len(), "walls plus slabs");
    let offsets: Vec<f64> = usages.iter().filter(|(_, args)| args[1].as_enum() == Some("AXIS2")).map(|(_, args)| real(args, 3)).collect();
    assert!(offsets.contains(&0.15), "a centred 300 wall: {offsets:?}");
    assert!(offsets.contains(&0.0), "the interior-located west wall");
    assert!(offsets.contains(&0.3), "the exterior-located east wall");
}

#[test]
fn walls_carry_base_quantities_that_match_the_layout() {
    let model = house();
    let document = document(&model);
    let (_, quantity) = rows(&document, "IFCELEMENTQUANTITY").into_iter().find(|(_, args)| string(args, 2).as_deref() == Some("Qto_WallBaseQuantities") && args[5].as_list().is_some_and(|items| items.iter().any(|item| document.resolve(item).and_then(|row| row.entity("IFCQUANTITYLENGTH")).is_some_and(|row| string(row, 0).as_deref() == Some("Length") && (real(row, 3) - 3.0).abs() < 1e-9)))).expect("the free wall quantities");
    let named = |name: &str| quantity[5].as_list().expect("quantities").iter().filter_map(|item| document.resolve(item)).find_map(|row| row.entities.iter().find(|(_, args)| string(args, 0).as_deref() == Some(name)).map(|(_, args)| real(args, 3))).unwrap_or(f64::NAN);
    assert!((named("Length") - 3.0).abs() < 1e-9 && (named("Height") - 2.4).abs() < 1e-9 && (named("Width") - 0.15).abs() < 1e-9);
    assert!((named("GrossVolume") - 3.0 * 0.15 * 2.4).abs() < 1e-9);
    assert!((named("NetVolume") - named("GrossVolume")).abs() < 1e-9, "no opening in the free wall");
}

#[test]
fn every_opening_voids_its_wall_and_every_window_and_door_fills_a_void() {
    let model = house();
    let document = document(&model);
    assert_eq!(count(&document, "IFCOPENINGELEMENT"), model.openings.len());
    assert_eq!(count(&document, "IFCRELVOIDSELEMENT"), 6);
    assert_eq!(count(&document, "IFCWINDOW"), 3);
    assert_eq!(count(&document, "IFCDOOR"), 2);
    assert_eq!(count(&document, "IFCRELFILLSELEMENT"), 5, "the plain void is not filled");
    assert_eq!(tags(&document, "IFCWINDOW"), ["o-win-1", "o-win-2", "o-win-arc"]);
}

#[test]
fn a_window_records_its_overall_size_and_a_door_flipped_to_the_other_facing_turns_around() {
    let document = document(&house());
    let (_, window) = rows(&document, "IFCWINDOW").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("o-win-2")).expect("window 2");
    assert!((real(window, 8) - 1.2).abs() < 1e-9 && (real(window, 9) - 1.0).abs() < 1e-9, "the type height and the overridden width");
    let placement = target(&document, window, 5).and_then(|instance| instance.entity("IFCLOCALPLACEMENT")).expect("the placement");
    let axis = document.resolve(&placement[1]).and_then(|instance| instance.entity("IFCAXIS2PLACEMENT3D")).expect("the axes");
    let direction = document.resolve(&axis[1]).and_then(|instance| instance.entity("IFCDIRECTION")).expect("the axis direction");
    let components: Vec<f64> = direction[0].as_list().expect("components").iter().filter_map(|item| item.as_real()).collect();
    assert_eq!(components, [0.0, 1.0, 0.0], "flip_facing turns the filling by half a turn");
}

#[test]
fn openings_reduce_the_net_wall_quantities() {
    let document = document(&house());
    let south = rows(&document, "IFCELEMENTQUANTITY");
    let net_side: Vec<f64> = south
        .iter()
        .filter(|(_, args)| string(args, 2).as_deref() == Some("Qto_WallBaseQuantities"))
        .flat_map(|(_, args)| args[5].as_list().expect("quantities").iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCQUANTITYAREA")).filter(|row| string(row, 0).as_deref() == Some("NetSideArea")).map(|row| real(row, 3)).collect::<Vec<f64>>())
        .collect();
    let gross_side = 8.0 * 3.0;
    let opened = 1.2 * 1.2 + 0.9 * 2.1;
    assert!(net_side.iter().any(|value| (value - (gross_side - opened)).abs() < 1e-9), "the south wall loses a window and a door: {net_side:?}");
}

//#region 🔖️Wall depth
use super::{envelope, ATTACH_SET, REVEAL_SET};
use crate::standards::v1::subsets::any::io::export::ifc::testkit::{attic, location, number_of, property_sets, text_of};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry;
use crate::{Opening, OpeningKind};
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Value};

fn layout(model: &crate::ModelSnapshot, id: &str) -> crate::standards::v1::subsets::any::schema::inferences::wall_layout::WallLayout {
    registry::try_with_inference(None, model, |inferred| inferred.wall_layout[id].clone()).expect("the model infers")
}

fn body_type(document: &Part21Document, args: &[Part21Value]) -> Option<String> {
    let shape = target(document, args, 6)?.entity("IFCPRODUCTDEFINITIONSHAPE")?;
    shape[2].as_list()?.iter().filter_map(|representation| document.resolve(representation)).filter_map(|representation| representation.entity("IFCSHAPEREPRESENTATION")).find(|representation| string(representation, 1).as_deref() == Some("Body")).and_then(|representation| string(representation, 2))
}

fn tag_of(document: &Part21Document, value: &Part21Value) -> String {
    document.resolve(value).and_then(|instance| instance.primary()).and_then(|(_, args)| string(args, 7)).unwrap_or_default()
}

#[test]
fn a_wall_under_a_roof_or_on_a_slab_is_the_faceted_brep_of_its_envelope_and_the_flat_ones_stay_swept() {
    let document = document(&attic());
    assert_eq!(tags(&document, "IFCWALLSTANDARDCASE"), ["w-door", "w-rail"], "a flat wall on a free top stays a swept standard case");
    for (_, args) in rows(&document, "IFCWALL") {
        let tag = string(args, 7).unwrap_or_default();
        let expected = if ["w-hip", "w-base", "w-east", "w-west"].contains(&tag.as_str()) { "Brep" } else { "SweptSolid" };
        assert_eq!(body_type(&document, args).as_deref(), Some(expected), "{tag}");
    }
}

#[test]
fn the_envelope_of_an_attached_wall_has_the_volume_of_its_layout() {
    let model = attic();
    for id in ["w-hip", "w-base", "w-east"] {
        let layout = layout(&model, id);
        let wall = &model.walls[id];
        let mesh = envelope(wall, &layout, 0.0);
        assert!(mesh.triangle_count() > 0, "{id}");
        assert!((mesh.signed_volume() - layout.volume).abs() < 1e-6, "{id}: mesh {}, layout {}", mesh.signed_volume(), layout.volume);
    }
}

#[test]
fn an_attached_wall_carries_its_authored_top_and_base_in_the_attach_set() {
    let document = document(&attic());
    let hip = &property_sets(&document, "w-hip", ATTACH_SET)[0];
    assert_eq!((text_of(hip, "TopKind").as_deref(), text_of(hip, "TopTarget").as_deref(), number_of(hip, "TopOffset")), (Some("Roof"), Some("r-hip"), Some(0.0)));
    assert_eq!(text_of(hip, "BaseSlab").as_deref(), Some(""));
    let layout = layout(&attic(), "w-hip");
    assert!((number_of(hip, "Height").expect("a height") - layout.height).abs() < 1e-9);
    let slope = &property_sets(&document, "w-base", ATTACH_SET)[0];
    assert_eq!((text_of(slope, "TopKind").as_deref(), text_of(slope, "BaseSlab").as_deref(), number_of(slope, "BaseOffset")), (Some("StoreyTop"), Some("sl-slope"), Some(0.0)));
    assert!(property_sets(&document, "w-rail", ATTACH_SET).is_empty(), "a free wall carries no attach set");
}

#[test]
fn every_attach_is_an_ifc_rel_connects_elements_from_the_wall_to_its_target() {
    let document = document(&attic());
    let mut described: Vec<(String, String, String)> = rows(&document, "IFCRELCONNECTSELEMENTS").iter().map(|(_, args)| (string(args, 3).unwrap_or_default(), tag_of(&document, &args[5]), tag_of(&document, &args[6]))).collect();
    described.sort();
    let top = |wall: &str, roof: &str| ("TopAttach".to_string(), wall.to_string(), roof.to_string());
    assert_eq!(described, [("BaseAttach".to_string(), "w-base".to_string(), "sl-slope".to_string()), top("w-east", "r-gable"), top("w-hip", "r-hip"), top("w-north", "r-gable"), top("w-south", "r-gable"), top("w-west", "r-gable")]);
}

#[test]
fn an_opening_in_a_wall_on_a_slope_is_placed_at_the_inferred_sill() {
    let mut model = attic();
    model.openings.insert("o-slope".into(), Opening { host: "w-base".into(), kind: OpeningKind::Window { window_type: "win-1".into() }, offset: 4.0, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, name: String::new(), reveal_depth: None, reveal_material: None });
    let (expected, base) = registry::try_with_inference(None, &model, |inferred| (inferred.opening_frames["o-slope"].local.origin.z, inferred.wall_layout["w-base"].base_at(4.0) - inferred.storey_levels[&model.walls["w-base"].storey].elevation)).expect("the model infers");
    let document = document(&model);
    let (_, args) = rows(&document, "IFCOPENINGELEMENT").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("o-slope")).expect("the opening element");
    let at = location(&document, args).expect("a location");
    assert!((at[2] - expected).abs() < 1e-9, "placed {} for the frame at {expected}", at[2]);
    let sill = model.window_types["win-1"].sill;
    assert!(base.abs() > 1e-3 && (expected - (sill + base)).abs() < 1e-9, "the sill follows the slope: frame at {expected}, sill {sill} over the base {base}");
}

#[test]
fn a_reveal_is_written_as_a_set_and_sets_the_filling_back_from_the_front_face() {
    let document = document(&attic());
    let reveal = &property_sets(&document, "o-reveal", REVEAL_SET)[0];
    assert_eq!((number_of(reveal, "RevealDepth"), text_of(reveal, "RevealMaterial").as_deref()), (Some(0.1), Some("m-paint")));
    assert!(property_sets(&document, "o-plain", REVEAL_SET).is_empty());
    let filling = |tag: &str| rows(&document, "IFCWINDOW").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some(tag)).map(|(_, args)| location(&document, args).expect("a location")).expect("a window");
    assert!((filling("o-reveal")[1] - 0.01).abs() < 1e-9, "front 0.15, reveal 0.1 and half the frame depth 0.04 leave the frame 0.01 in front of the centre: {:?}", filling("o-reveal"));
    assert!(filling("o-plain")[1].abs() < 1e-12, "a window without a reveal stays centred");
}

#[test]
fn the_attic_export_is_deterministic_and_skips_nothing() {
    let model = attic();
    assert_eq!(document(&model), document(&model));
    let (_, notes) = crate::standards::v1::subsets::any::io::export::ifc::model_to_part21(crate::standards::v1::subsets::any::io::export::ifc::Schema::Ifc2x3, &model).expect("the attic exports");
    assert!(notes.is_empty(), "{notes:?}");
}
//#endregion 🔖️Wall depth

//#region 🔖️Committed attic
use crate::standards::v1::subsets::any::io::export::ifc::{export_ifc2x3, projection::projection};

const ATTIC_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🏗️ifc/🧗️wall-depth");

fn read_attic(name: &str) -> Vec<u8> {
    std::fs::read(format!("{ATTIC_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."))
}

#[test]
fn the_committed_attic_file_is_the_current_export() {
    let (bytes, notes) = export_ifc2x3(&attic()).expect("the attic export");
    assert!(notes.is_empty(), "{notes:?}");
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::create_dir_all(ATTIC_DIR).expect("the fixture directory");
        std::fs::write(format!("{ATTIC_DIR}/🧗️wall-depth.ifc"), &bytes).expect("the file is written");
    }
    assert_eq!(read_attic("🧗️wall-depth.ifc"), bytes, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_report_counts_the_attached_walls_the_sweep_runs_and_the_connections_and_measures_only_the_swept_walls() {
    let report = projection(&attic());
    assert_eq!((report.counts["IfcWall"], report.counts["IfcWallStandardCase"], report.counts["IfcMember"], report.counts["IfcRelConnectsElements"]), (6, 2, 4, 6));
    let sweeps = |storey: &str| report.containment[storey].iter().filter(|tag| tag.starts_with("ws-")).cloned().collect::<Vec<_>>();
    assert_eq!((sweeps("st-0"), sweeps("st-1")), (vec!["ws-door".to_string(), "ws-door".to_string(), "ws-slope".to_string()], vec!["ws-rail".to_string()]), "every run is contained in the storey of its wall");
    assert_eq!(report.volumes.keys().collect::<Vec<_>>(), ["w-door", "w-rail"], "an attached wall is a brep measured against the wall layout, not here");
}

#[test]
fn the_attic_subject_report_equals_the_table_the_ifcopenshell_oracle_measured_from_the_committed_file() {
    let oracle: serde_json::Value = serde_json::from_slice(&read_attic("🔬️measure/🔣️.json")).expect("the oracle table");
    let ours: serde_json::Value = serde_json::from_str(&projection(&attic()).to_json()).expect("the report parses");
    for key in ["schema", "counts", "containment"] {
        assert_eq!(oracle[key], ours[key], "{key}");
    }
    assert_eq!(oracle["volumes"].as_object().expect("volumes").len(), ours["volumes"].as_object().expect("volumes").len());
    for (tag, volume) in ours["volumes"].as_object().expect("volumes") {
        let (measured, written) = (oracle["volumes"][tag].as_f64().expect("a kernel volume"), volume.as_f64().expect("a written volume"));
        assert!((measured - written).abs() < 1e-9 * written.abs().max(1.0), "{tag}: kernel {measured}, written {written}");
    }
}
//#endregion 🔖️Committed attic
