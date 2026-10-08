use crate::standards::v1::subsets::any::io::export::ifc::testkit::{document, house, real, rows, string, target};

#[test]
fn spaces_are_named_by_number_long_named_by_name_and_extruded_over_their_clear_height() {
    let document = document(&house());
    let (_, bedroom) = rows(&document, "IFCSPACE").into_iter().find(|(_, args)| string(args, 4).as_deref() == Some("sp-2")).expect("the bedroom");
    assert_eq!(string(bedroom, 2).as_deref(), Some("1.01"));
    assert_eq!(string(bedroom, 7).as_deref(), Some("Bedroom"));
    assert_eq!(string(bedroom, 3).as_deref(), Some("bedroom"));
    assert_eq!(bedroom[9].as_enum(), Some("INTERNAL"));
    let shape = target(&document, bedroom, 6).and_then(|instance| instance.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("the shape");
    let representation = document.resolve(&shape[2].as_list().expect("representations")[0]).and_then(|instance| instance.entity("IFCSHAPEREPRESENTATION")).expect("the representation");
    let solid = document.resolve(&representation[3].as_list().expect("items")[0]).and_then(|instance| instance.entity("IFCEXTRUDEDAREASOLID")).expect("the solid");
    assert!(real(solid, 3) > 2.0 && real(solid, 3) <= 2.8, "the clear height of the first storey");
}

#[test]
fn a_space_is_aggregated_under_its_storey_and_reports_its_floor_area() {
    let document = document(&house());
    let (_, aggregate) = rows(&document, "IFCRELAGGREGATES")
        .into_iter()
        .find(|(_, relation)| document.resolve(&relation[4]).is_some_and(|whole| whole.is_type("IFCBUILDINGSTOREY")) && relation[5].as_list().is_some_and(|parts| parts.iter().any(|part| document.resolve(part).is_some_and(|instance| instance.is_type("IFCSPACE")))))
        .expect("a storey aggregating spaces");
    assert!(!aggregate[5].as_list().expect("parts").is_empty());
    let areas: Vec<f64> = rows(&document, "IFCQUANTITYAREA").iter().filter(|(_, args)| string(args, 0).as_deref() == Some("GrossFloorArea")).map(|(_, args)| real(args, 3)).collect();
    assert!(areas.iter().any(|area| (area - 3.6 * 5.6).abs() < 1e-6), "the explicit 3.6 by 5.6 bedroom: {areas:?}");
}
