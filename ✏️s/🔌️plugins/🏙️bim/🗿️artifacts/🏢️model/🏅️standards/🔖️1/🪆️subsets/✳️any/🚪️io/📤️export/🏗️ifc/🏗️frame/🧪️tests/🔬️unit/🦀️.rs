use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, real, rows, string, tags, target};
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Value};

fn solid_of<'a>(document: &'a Part21Document, args: &[Part21Value]) -> &'a Vec<Part21Value> {
    let shape = target(document, args, 6).and_then(|instance| instance.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("the shape");
    let representation = document.resolve(&shape[2].as_list().expect("representations")[0]).and_then(|instance| instance.entity("IFCSHAPEREPRESENTATION")).expect("the representation");
    document.resolve(&representation[3].as_list().expect("items")[0]).and_then(|instance| instance.entity("IFCEXTRUDEDAREASOLID")).expect("the solid")
}

#[test]
fn columns_extrude_their_profile_over_the_resolved_height() {
    let document = document(&house());
    assert_eq!(tags(&document, "IFCCOLUMN"), ["c-1", "c-2"]);
    let (_, rectangle) = rows(&document, "IFCCOLUMN").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("c-1")).expect("c-1");
    let solid = solid_of(&document, rectangle);
    assert!((real(solid, 3) - 3.0).abs() < 1e-9, "the ground storey height");
    let profile = target(&document, solid, 0).and_then(|instance| instance.entity("IFCRECTANGLEPROFILEDEF")).expect("a rectangle profile");
    assert_eq!((real(profile, 3), real(profile, 4)), (0.3, 0.3));
    let (_, round) = rows(&document, "IFCCOLUMN").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("c-2")).expect("c-2");
    assert!((real(solid_of(&document, round), 3) - 2.5).abs() < 1e-9, "the unconnected height");
    let circle = target(&document, solid_of(&document, round), 0).and_then(|instance| instance.entity("IFCCIRCLEPROFILEDEF")).expect("a circle");
    assert!((real(circle, 3) - 0.2).abs() < 1e-9);
}

#[test]
fn a_rotated_column_turns_its_placement_reference_direction() {
    let document = document(&house());
    let (_, rotated) = rows(&document, "IFCCOLUMN").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("c-1")).expect("c-1");
    let placement = target(&document, rotated, 5).and_then(|instance| instance.entity("IFCLOCALPLACEMENT")).expect("the placement");
    let axes = document.resolve(&placement[1]).and_then(|instance| instance.entity("IFCAXIS2PLACEMENT3D")).expect("the axes");
    let direction = document.resolve(&axes[2]).and_then(|instance| instance.entity("IFCDIRECTION")).expect("the reference direction");
    let components: Vec<f64> = direction[0].as_list().expect("components").iter().filter_map(Part21Value::as_real).collect();
    assert!((components[0] - 0.3f64.cos()).abs() < 1e-8 && (components[1] - 0.3f64.sin()).abs() < 1e-8);
}

#[test]
fn beams_extrude_along_their_axis_with_the_profile_top_on_the_storey_top() {
    let document = document(&house());
    assert_eq!(tags(&document, "IFCBEAM"), ["b-1", "b-2"]);
    let (_, steel) = rows(&document, "IFCBEAM").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("b-1")).expect("b-1");
    let solid = solid_of(&document, steel);
    assert!((real(solid, 3) - 2.0).abs() < 1e-9, "the beam length");
    assert_eq!(count(&document, "IFCISHAPEPROFILEDEF"), 1);
    let placement = target(&document, steel, 5).and_then(|instance| instance.entity("IFCLOCALPLACEMENT")).expect("the placement");
    let axes = document.resolve(&placement[1]).and_then(|instance| instance.entity("IFCAXIS2PLACEMENT3D")).expect("the axes");
    let location = document.resolve(&axes[0]).and_then(|instance| instance.entity("IFCCARTESIANPOINT")).expect("the location");
    let z = location[0].as_list().expect("coordinates")[2].as_real().expect("a z");
    assert!((z - (3.0 - 0.15)).abs() < 1e-9, "the 300 mm deep I-shape hangs from the ground storey top: {z}");
}

#[test]
fn frames_carry_area_and_volume_quantities() {
    let document = document(&house());
    let column_volume: Vec<f64> = rows(&document, "IFCELEMENTQUANTITY")
        .iter()
        .filter(|(_, args)| string(args, 2).as_deref() == Some("Qto_ColumnBaseQuantities"))
        .flat_map(|(_, args)| args[5].as_list().expect("quantities").iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCQUANTITYVOLUME")).map(|row| real(row, 3)).collect::<Vec<f64>>())
        .collect();
    assert!(column_volume.iter().any(|volume| (volume - 0.09 * 3.0).abs() < 1e-9));
    assert!(column_volume.iter().any(|volume| (volume - std::f64::consts::PI * 0.04 * 2.5).abs() < 1e-9));
    assert!(count(&document, "IFCRELASSOCIATESMATERIAL") > 0);
}
