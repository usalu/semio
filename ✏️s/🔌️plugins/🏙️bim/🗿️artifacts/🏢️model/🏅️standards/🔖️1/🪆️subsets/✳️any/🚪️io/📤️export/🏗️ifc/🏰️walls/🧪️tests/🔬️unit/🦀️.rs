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
