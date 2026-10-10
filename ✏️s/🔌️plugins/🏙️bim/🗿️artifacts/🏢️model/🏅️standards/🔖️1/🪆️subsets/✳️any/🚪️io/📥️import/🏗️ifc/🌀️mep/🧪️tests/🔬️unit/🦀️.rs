use crate::standards::v1::subsets::any::io::export::ifc::testkit::components;
use crate::standards::v1::subsets::any::io::export::ifc::{export_ifc2x3, export_ifc4};
use crate::standards::v1::subsets::any::io::import::ifc::{import_ifc2x3, import_ifc4};
use crate::{MepShape, MepSystem};

#[test]
fn the_routed_elements_survive_the_round_trip_exactly_in_both_schemas() {
    let model = components();
    let (back, notes) = import_ifc2x3(&export_ifc2x3(&model).expect("2x3 export").0).expect("the file imports");
    assert!(notes.iter().all(|note| !note.contains("mep-")), "{notes:?}");
    assert_eq!(back.mep_elements, model.mep_elements);
    let (back, notes) = import_ifc4(&export_ifc4(&model).expect("4 export").0).expect("the file imports");
    assert!(notes.iter().all(|note| !note.contains("mep-")), "{notes:?}");
    assert_eq!(back.mep_elements, model.mep_elements);
}

#[test]
fn a_foreign_segment_is_read_along_its_extrusion_with_the_system_of_its_group() {
    use crate::standards::v1::subsets::any::io::export::ifc::codec::encode_document;
    use crate::standards::v1::subsets::any::io::export::ifc::writer::{en, real, reals, refs, rf, unset};
    use crate::standards::v1::subsets::any::io::export::ifc::{model_to_part21, Schema};
    use semio_s_artifact_stdio_ifc::part21::Part21Instance;
    let model = components();
    let (mut document, _) = model_to_part21(Schema::Ifc2x3, &model).expect("the export");
    let context = document.by_type("IFCGEOMETRICREPRESENTATIONSUBCONTEXT").next().expect("a body context").id;
    let mut next = document.next_id();
    let mut add = |name: &str, args: Vec<semio_s_artifact_stdio_ifc::part21::Part21Value>| {
        let id = next;
        next += 1;
        (id, Part21Instance { id, entities: vec![(name.to_string(), args)] })
    };
    let (point2, a) = add("IFCCARTESIANPOINT", vec![reals(&[0.0, 0.0])]);
    let (placement2, b) = add("IFCAXIS2PLACEMENT2D", vec![rf(point2), unset()]);
    let (profile, c) = add("IFCRECTANGLEPROFILEDEF", vec![en("AREA"), unset(), rf(placement2), real(0.4), real(0.25)]);
    let (point3, d) = add("IFCCARTESIANPOINT", vec![reals(&[0.0, 0.0, 0.0])]);
    let (placement3, e) = add("IFCAXIS2PLACEMENT3D", vec![rf(point3), unset(), unset()]);
    let (direction, f) = add("IFCDIRECTION", vec![reals(&[1.0, 0.0, 0.0])]);
    let (solid, g) = add("IFCEXTRUDEDAREASOLID", vec![rf(profile), rf(placement3), rf(direction), real(5.0)]);
    let (representation, h) = add("IFCSHAPEREPRESENTATION", vec![rf(context), crate::standards::v1::subsets::any::io::export::ifc::writer::text("Body"), crate::standards::v1::subsets::any::io::export::ifc::writer::text("SweptSolid"), refs(&[solid])]);
    let (shape, k) = add("IFCPRODUCTDEFINITIONSHAPE", vec![unset(), unset(), refs(&[representation])]);
    let supply = document.instances.iter_mut().find(|instance| instance.entity("IFCFLOWSEGMENT").is_some_and(|args| args[7].as_str() == Some("mep-supply"))).expect("the duct");
    supply.entities[0].1[6] = rf(shape);
    document.instances.extend([a, b, c, d, e, f, g, h, k]);
    let text = String::from_utf8(encode_document(document).expect("encodes")).expect("text").replace("'MepElement'", "'Other'");
    let (back, _) = import_ifc2x3(text.as_bytes()).expect("the file imports");
    let duct = &back.mep_elements["mep-supply"];
    assert_eq!(duct.system, MepSystem::Supply);
    assert_eq!(duct.shape, MepShape::Duct { width: 0.4, height: 0.25 });
    assert_eq!(duct.storey, "st-ground");
    assert_eq!(duct.path.len(), 2);
    assert!((duct.path[1].x - duct.path[0].x - 5.0).abs() < 1e-9 && duct.path[1].y == duct.path[0].y && duct.path[1].z == duct.path[0].z);
}
