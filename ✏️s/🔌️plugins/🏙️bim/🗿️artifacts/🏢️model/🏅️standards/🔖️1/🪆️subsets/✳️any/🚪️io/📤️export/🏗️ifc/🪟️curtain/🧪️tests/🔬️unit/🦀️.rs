use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, real, rows, string};

#[test]
fn a_curtain_wall_aggregates_its_mullions_and_its_panels() {
    let document = document(&house());
    let (wall, args) = rows(&document, "IFCCURTAINWALL").into_iter().next().expect("a curtain wall");
    assert_eq!(string(args, 7).as_deref(), Some("cw-1"));
    let (_, aggregate) = rows(&document, "IFCRELAGGREGATES").into_iter().find(|(_, relation)| relation[4].as_ref_id() == Some(wall.id)).expect("its parts");
    let kinds: Vec<String> = aggregate[5].as_list().expect("parts").iter().filter_map(|part| document.resolve(part)).filter_map(|instance| instance.primary().map(|(name, _)| name.to_string())).collect();
    assert_eq!(kinds, ["IFCMEMBER", "IFCPLATE"]);
    assert_eq!(count(&document, "IFCMEMBER"), 1);
    assert_eq!(count(&document, "IFCPLATE"), 1);
}

#[test]
fn the_curtain_wall_reports_its_length_height_and_side_area() {
    let document = document(&house());
    let (_, quantity) = rows(&document, "IFCELEMENTQUANTITY").into_iter().find(|(_, args)| string(args, 2).as_deref() == Some("Qto_CurtainWallQuantities")).expect("the curtain wall quantities");
    let values: Vec<f64> = quantity[5].as_list().expect("quantities").iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entities.first().map(|(_, args)| real(args, 3))).collect();
    assert_eq!(values.len(), 3);
    assert!((values[0] - 6.0).abs() < 1e-9 && (values[1] - 2.8).abs() < 1e-9 && (values[2] - 6.0 * 2.8).abs() < 1e-9);
}
