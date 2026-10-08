use super::*;
use semio_framework_geometry::mesh::extrude;
use semio_framework_geometry::placement::ZPlane;
use semio_framework_geometry::Point;
use semio_s_artifact_stdio_ifc::part21::Part21Header;

fn cube() -> TriMesh {
    extrude(&[Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 1.0)], &[], ZPlane::flat(0.0), ZPlane::flat(1.0))
}

#[test]
fn a_cube_mesh_becomes_a_closed_shell_of_twelve_faces_over_eight_shared_points() {
    let mut ifc = Ifc::new("a", "o", 0.0);
    let brep = faceted_brep(&mut ifc, &cube()).expect("a brep");
    let document = ifc.finish(Part21Header::iso_10303_21_minimum());
    assert!(document.instance(brep).is_some_and(|instance| instance.is_type("IFCFACETEDBREP")));
    assert_eq!(document.by_type("IFCFACE").count(), 12);
    assert_eq!(document.by_type("IFCCLOSEDSHELL").count(), 1);
    assert_eq!(document.by_type("IFCCARTESIANPOINT").count(), 8, "the eight corners are shared, the world origin among them");
}

#[test]
fn an_empty_mesh_has_no_brep() {
    let mut ifc = Ifc::new("a", "o", 0.0);
    assert!(faceted_brep(&mut ifc, &TriMesh::new()).is_none());
    assert!(brep_definition(&mut ifc, &TriMesh::new()).is_none());
}

#[test]
fn a_definition_shape_names_the_body_brep_representation() {
    let mut ifc = Ifc::new("a", "o", 0.0);
    let definition = brep_definition(&mut ifc, &cube()).expect("a definition");
    let document = ifc.finish(Part21Header::iso_10303_21_minimum());
    let args = document.instance(definition).and_then(|instance| instance.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("the definition");
    let shape = document.resolve(&args[2].as_list().expect("representations")[0]).and_then(|instance| instance.entity("IFCSHAPEREPRESENTATION")).expect("the shape");
    assert_eq!((shape[1].as_str(), shape[2].as_str()), (Some("Body"), Some("Brep")));
}
