use super::*;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, real, rows, string, target};

#[test]
fn the_compound_angle_splits_degrees_minutes_seconds_and_millionths_with_the_sign_on_every_part() {
    let parts = |value: f64| match compound_angle(value) {
        V::List(items) => items.iter().filter_map(|item| if let V::Int(number) = item { Some(*number) } else { None }).collect::<Vec<i64>>(),
        other => panic!("not a list: {other:?}"),
    };
    assert_eq!(parts(47.0), [47, 0, 0, 0]);
    assert_eq!(parts(47.5), [47, 30, 0, 0]);
    assert_eq!(parts(-0.5), [0, -30, 0, 0]);
    assert_eq!(parts(8.5417), [8, 32, 30, 120_000]);
}

#[test]
fn the_project_carries_the_units_the_context_and_the_phases() {
    let model = house();
    let document = document(&model);
    let (_, project) = rows(&document, "IFCPROJECT").into_iter().next().expect("a project");
    assert_eq!(string(project, 2).as_deref(), Some("IFC House"));
    assert_eq!(string(project, 6).as_deref(), Some("Existing;New"));
    assert_eq!(project[7].as_list().map(<[V]>::len), Some(1));
    let units = target(&document, project, 8).and_then(|instance| instance.entity("IFCUNITASSIGNMENT")).expect("the unit assignment");
    let names: Vec<&str> = units[0].as_list().expect("units").iter().filter_map(|unit| document.resolve(unit)).filter_map(|unit| unit.entity("IFCSIUNIT")).filter_map(|args| args[3].as_enum()).collect();
    assert_eq!(names, ["METRE", "SQUARE_METRE", "CUBIC_METRE", "RADIAN"]);
}

#[test]
fn the_site_has_a_compound_position_an_elevation_and_a_boundary_footprint() {
    let document = document(&house());
    let (_, site) = rows(&document, "IFCSITE").into_iter().next().expect("a site");
    assert_eq!(string(site, 2).as_deref(), Some("Plot"));
    assert_eq!(real(site, 11), 408.0);
    let latitude: Vec<i64> = site[9].as_list().expect("latitude").iter().filter_map(|item| if let V::Int(number) = item { Some(*number) } else { None }).collect();
    assert_eq!(&latitude[..2], [47, 22]);
    let footprint = target(&document, site, 6).and_then(|instance| instance.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("the footprint shape");
    assert_eq!(footprint[2].as_list().map(<[V]>::len), Some(1));
}

#[test]
fn a_building_sits_on_its_site_with_its_rotation_and_a_storey_per_level_at_its_elevation() {
    let model = house();
    let document = document(&model);
    assert_eq!(count(&document, "IFCBUILDING"), 1);
    assert_eq!(count(&document, "IFCBUILDINGSTOREY"), model.storeys.len());
    let (_, building) = rows(&document, "IFCBUILDING").into_iter().next().expect("a building");
    assert_eq!(real(building, 9), 408.5, "the reference height is absolute: site plus building elevation");
    let mut elevations: Vec<(String, f64)> = rows(&document, "IFCBUILDINGSTOREY").iter().map(|(_, args)| (string(args, 4).expect("the id"), real(args, 9))).collect();
    elevations.sort_by(|a, b| a.1.total_cmp(&b.1));
    assert_eq!(elevations, [("st-base".into(), -2.6), ("st-ground".into(), 0.0), ("st-first".into(), 3.0), ("st-roof".into(), 5.8)]);
}

#[test]
fn the_aggregation_chain_runs_project_site_building_storey() {
    let document = document(&house());
    let wholes: Vec<String> = rows(&document, "IFCRELAGGREGATES")
        .iter()
        .filter_map(|(_, args)| document.resolve(&args[4]).and_then(|whole| whole.primary()).map(|(name, _)| name.to_string()))
        .collect();
    for whole in ["IFCPROJECT", "IFCSITE", "IFCBUILDING", "IFCBUILDINGSTOREY"] {
        assert!(wholes.iter().any(|name| name == whole), "{whole} aggregates its parts: {wholes:?}");
    }
}

#[test]
fn each_storey_records_its_level_and_height_for_the_import() {
    let document = document(&house());
    let levels: Vec<i64> = rows(&document, "IFCPROPERTYSINGLEVALUE")
        .iter()
        .filter(|(_, args)| string(args, 0).as_deref() == Some("Level"))
        .filter_map(|(_, args)| args[2].as_typed().and_then(|(_, items)| if let V::Int(number) = &items[0] { Some(*number) } else { None }))
        .collect();
    assert_eq!(levels.len(), 4);
    let heights: Vec<f64> = rows(&document, "IFCQUANTITYLENGTH").iter().filter(|(_, args)| string(args, 0).as_deref() == Some("GrossHeight")).map(|(_, args)| real(args, 3)).collect();
    assert_eq!(heights.len(), 4);
    assert!(heights.contains(&2.6) && heights.contains(&3.0) && heights.contains(&2.8) && heights.contains(&0.5));
}
