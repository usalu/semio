use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, rows, string, target};

#[test]
fn a_building_grid_lists_vertical_lines_as_u_axes_and_horizontal_lines_as_v_axes() {
    let document = document(&house());
    assert_eq!(count(&document, "IFCGRID"), 1);
    assert_eq!(count(&document, "IFCGRIDAXIS"), 3);
    let (_, grid) = rows(&document, "IFCGRID").into_iter().next().expect("a grid");
    assert_eq!(string(grid, 4).as_deref(), Some("g-a,g-b,g-1"), "the ids in axis order: U then V");
    let labels = |index: usize| -> Vec<String> { grid[index].as_list().expect("axes").iter().filter_map(|axis| document.resolve(axis)).filter_map(|axis| axis.entity("IFCGRIDAXIS")).filter_map(|args| string(args, 0)).collect() };
    assert_eq!(labels(7), ["A", "B"]);
    assert_eq!(labels(8), ["1"]);
    assert!(target(&document, grid, 5).is_some());
}

#[test]
fn the_grid_is_contained_in_its_building() {
    let document = document(&house());
    let relation = rows(&document, "IFCRELCONTAINEDINSPATIALSTRUCTURE").into_iter().find(|(_, args)| document.resolve(&args[5]).is_some_and(|whole| whole.is_type("IFCBUILDING"))).expect("the building containment");
    assert!(relation.1[4].as_list().is_some_and(|items| items.iter().any(|item| document.resolve(item).is_some_and(|instance| instance.is_type("IFCGRID")))));
}
