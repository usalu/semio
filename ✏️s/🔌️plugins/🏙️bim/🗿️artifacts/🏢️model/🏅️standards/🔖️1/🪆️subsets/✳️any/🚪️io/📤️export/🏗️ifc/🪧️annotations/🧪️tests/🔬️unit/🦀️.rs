use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, notated, real, rows, string, target};
use crate::standards::v1::subsets::any::io::export::ifc::model_to_part21;
use crate::standards::v1::subsets::any::io::export::ifc::projection::dimension_total;

#[test]
fn every_annotation_is_one_ifc_annotation_of_its_kind_contained_in_its_storey() {
    let model = notated();
    let document = document(&model);
    assert_eq!(count(&document, "IFCANNOTATION"), model.dimensions.len() + model.tags.len() + model.text_notes.len() + model.leaders.len());
    let kinds = |kind: &str| rows(&document, "IFCANNOTATION").iter().filter(|(_, args)| string(args, 4).as_deref() == Some(kind)).count();
    assert_eq!((kinds("Dimension"), kinds("Tag"), kinds("TextNote"), kinds("Leader")), (8, 5, 1, 1));
    let storey = rows(&document, "IFCBUILDINGSTOREY").into_iter().find(|(_, args)| string(args, 2).as_deref() == Some("Ground")).expect("the ground storey").0.id;
    let contained: usize = rows(&document, "IFCRELCONTAINEDINSPATIALSTRUCTURE")
        .iter()
        .filter(|(_, args)| args[5].as_ref_id() == Some(storey))
        .map(|(_, args)| args[4].as_list().map_or(0, |items| items.iter().filter(|item| document.resolve(item).is_some_and(|instance| instance.is_type("IFCANNOTATION"))).count()))
        .sum();
    assert_eq!(contained, 15, "all annotations of the storey are contained in it");
}

#[test]
fn the_representation_is_the_plan_drawing_of_the_annotation() {
    let document = document(&notated());
    let south = rows(&document, "IFCANNOTATION").into_iter().find(|(_, args)| string(args, 2).as_deref() == Some("South length")).expect("the south dimension").1;
    assert_eq!(string(south, 3).as_deref(), Some("8.00"), "the description is the printed text");
    let shape = target(&document, south, 6).and_then(|definition| definition.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("a product definition shape");
    let representation = document.resolve(&shape[2].as_list().expect("representations")[0]).and_then(|instance| instance.entity("IFCSHAPEREPRESENTATION")).expect("a representation");
    assert_eq!((string(representation, 1).as_deref(), string(representation, 2).as_deref()), (Some("Annotation"), Some("Annotation2D")));
    let kinds: Vec<&str> = representation[3].as_list().expect("items").iter().filter_map(|item| document.resolve(item)).map(|item| item.primary().map_or("", |(entity, _)| entity)).collect();
    assert!(kinds.contains(&"IFCPOLYLINE") && kinds.contains(&"IFCTEXTLITERALWITHEXTENT"), "{kinds:?}");
    assert_eq!(count(&document, "IFCTEXTLITERALWITHEXTENT"), 14, "8 dimension texts, 4 tags, a note and a leader text");
    let literal = rows(&document, "IFCTEXTLITERALWITHEXTENT").into_iter().find(|(_, args)| string(args, 0).as_deref() == Some("8.00")).expect("the printed 8.00").1;
    let extent = target(&document, literal, 3).and_then(|extent| extent.entity("IFCPLANAREXTENT")).expect("an extent");
    assert!((real(extent, 1) - 0.25).abs() < 1e-12 && (real(extent, 0) - 4.0 * 0.25 * 0.55).abs() < 1e-12, "the extent is the text height and its estimated advance");
    assert_eq!(string(literal, 4).as_deref(), Some("bottom-middle"));
}

#[test]
fn a_dimension_carries_its_measured_total_and_an_unresolved_one_draws_nothing_and_is_reported() {
    let model = notated();
    let (document, notes) = model_to_part21(crate::standards::v1::subsets::any::io::export::ifc::Schema::Ifc2x3, &model).expect("exports");
    let value = |name: &str| rows(&document, "IFCANNOTATION").into_iter().find(|(_, args)| string(args, 2).as_deref() == Some(name)).map(|(instance, _)| instance.id).expect("the dimension");
    let totals: Vec<f64> = dimension_total(&document, value("South length")).into_iter().collect();
    assert_eq!(totals.len(), 1);
    assert!((totals[0] - 8.0).abs() < 1e-9);
    let parallel = rows(&document, "IFCANNOTATION").into_iter().find(|(_, args)| string(args, 2).as_deref() == Some("Parallel faces")).expect("written").1;
    assert!(parallel[6].as_ref_id().is_none(), "no anchor geometry, no representation");
    assert!(notes.iter().any(|note| note.contains("dim-parallel")), "{notes:?}");
}

#[test]
fn a_model_without_annotations_writes_no_ifc_annotation() {
    assert_eq!(count(&document(&house()), "IFCANNOTATION"), 0);
}
