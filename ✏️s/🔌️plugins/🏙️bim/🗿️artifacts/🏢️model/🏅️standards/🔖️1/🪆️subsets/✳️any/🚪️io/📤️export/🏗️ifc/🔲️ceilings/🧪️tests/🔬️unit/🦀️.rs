use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, quantity, real, rows, string, tags, target};
use crate::{Ceiling, CeilingType, Layer, LayerFunction, ModelSnapshot, Point2, Slope, Vertex};

fn vertex(x: f64, y: f64) -> Vertex {
    Vertex { point: Point2 { x, y }, bulge: 0.0 }
}

fn square(side: f64) -> Vec<Vertex> {
    vec![vertex(0.0, 0.0), vertex(side, 0.0), vertex(side, side), vertex(0.0, side)]
}

/// 🔲️ The house with a two-layer ceiling type (12.5 mm board on 37.5 mm battens) and one ceiling on its ground storey.
pub fn with_ceiling(boundary: Vec<Vertex>, slope: Option<Slope>) -> ModelSnapshot {
    let mut model = house();
    let storey = model.storeys.iter().find(|(_, storey)| storey.level == 0).map(|(id, _)| id.clone()).expect("the ground storey");
    let material = model.materials.keys().next().cloned().expect("a material");
    let layer = |thickness| Layer { material: material.clone(), thickness, function: LayerFunction::Finish };
    model.ceiling_types.insert("ct-board".into(), CeilingType { name: "Board".into(), layers: vec![layer(0.0125), layer(0.0375)] });
    model.ceilings.insert("c-1".into(), Ceiling { storey, ceiling_type: "ct-board".into(), boundary, holes: Vec::new(), offset: 0.2, slope, name: "Hall ceiling".into() });
    model
}

#[test]
fn a_level_ceiling_is_a_ceiling_covering_extruded_by_the_sum_of_its_layers() {
    let document = document(&with_ceiling(square(4.0), None));
    assert_eq!(tags(&document, "IFCCOVERING"), ["c-1"]);
    let (_, args) = rows(&document, "IFCCOVERING").into_iter().next().expect("a covering");
    assert_eq!(args[8].as_enum(), Some("CEILING"));
    assert!(target(&document, args, 6).is_some(), "the covering carries a body");
    assert!(rows(&document, "IFCEXTRUSIONAREASOLID").iter().any(|(_, solid)| (real(solid, 3) - 0.05).abs() < 1e-9), "the extrusion is as deep as the layers are thick");
}

#[test]
fn the_ceiling_type_is_a_covering_type_with_the_same_layer_set() {
    let document = document(&with_ceiling(square(4.0), None));
    assert!(count(&document, "IFCCOVERINGTYPE") >= 1);
    assert!(rows(&document, "IFCMATERIALLAYERSETUSAGE").iter().any(|(_, usage)| usage[3].as_real().is_some_and(|thickness| (thickness - 0.05).abs() < 1e-9)), "the usage carries the total thickness");
}

#[test]
fn the_ceiling_carries_its_covering_base_quantities() {
    let document = document(&with_ceiling(square(4.0), None));
    let gross = quantity(&document, "c-1", "Qto_CoveringBaseQuantities", "GrossArea").expect("a gross area");
    assert!((gross - 16.0).abs() < 1e-6, "{gross}");
    assert!(quantity(&document, "c-1", "Qto_CoveringBaseQuantities", "NetArea").is_some());
}

#[test]
fn a_sloped_ceiling_is_written_from_its_inferred_solid() {
    let document = document(&with_ceiling(square(4.0), Some(Slope { direction: 0.0, angle: 0.1 })));
    let (_, args) = rows(&document, "IFCCOVERING").into_iter().next().expect("a covering");
    assert!(target(&document, args, 6).is_some());
    assert_eq!(rows(&document, "IFCEXTRUSIONAREASOLID").iter().filter(|(_, solid)| (real(solid, 3) - 0.05).abs() < 1e-9).count(), 0, "a faceted brep, not an extrusion");
}

#[test]
fn a_ceiling_without_a_boundary_is_skipped_and_the_export_is_deterministic() {
    let degenerate = with_ceiling(vec![vertex(0.0, 0.0), vertex(1.0, 0.0)], None);
    assert!(tags(&document(&degenerate), "IFCCOVERING").is_empty());
    let model = with_ceiling(square(3.0), None);
    assert_eq!(document(&model), document(&model));
}

#[test]
fn a_hole_makes_the_profile_one_with_voids_and_the_net_area_drops_it() {
    let mut model = with_ceiling(square(4.0), None);
    model.ceilings.get_mut("c-1").expect("the ceiling").holes = vec![vec![vertex(1.0, 1.0), vertex(1.0, 2.0), vertex(2.0, 2.0), vertex(2.0, 1.0)]];
    let document = document(&model);
    assert_eq!(count(&document, "IFCARBITRARYPROFILEDEFWITHVOIDS"), 1);
    let net = quantity(&document, "c-1", "Qto_CoveringBaseQuantities", "NetArea").expect("a net area");
    let gross = quantity(&document, "c-1", "Qto_CoveringBaseQuantities", "GrossArea").expect("a gross area");
    assert!((gross - 16.0).abs() < 1e-6 && (net - 15.0).abs() < 1e-6, "gross {gross}, net {net}");
    assert!((quantity(&document, "c-1", "Qto_CoveringBaseQuantities", "Width").expect("a width") - 0.05).abs() < 1e-9);
}

#[test]
fn the_ceiling_hangs_its_layers_below_the_storey_top_by_the_offset() {
    let model = with_ceiling(square(4.0), None);
    let document = document(&model);
    let (_, args) = rows(&document, "IFCCOVERING").into_iter().next().expect("a covering");
    let placement = target(&document, args, 5).expect("a placement");
    let local = placement.entity("IFCLOCALPLACEMENT").expect("a local placement");
    let axis = target(&document, local, 1).and_then(|instance| instance.entity("IFCAXIS2PLACEMENT3D")).expect("an axis placement");
    let origin = target(&document, axis, 0).and_then(|instance| instance.entity("IFCCARTESIANPOINT")).expect("an origin");
    let coordinates = origin[0].as_list().expect("coordinates");
    let storey = model.storeys.iter().find(|(_, storey)| storey.level == 0).map(|(_, storey)| storey.height).expect("the ground storey");
    let z = coordinates[2].as_real().expect("a height");
    assert!((z - (storey - 0.2 - 0.05)).abs() < 1e-9, "the underside lies the offset plus the thickness below the storey top: {z}");
}

#[test]
fn only_a_sloped_ceiling_carries_its_authored_record() {
    let rows_named = |document: &semio_s_artifact_stdio_ifc::part21::Part21Document| rows(document, "IFCPROPERTYSINGLEVALUE").into_iter().filter(|(_, args)| string(args, 0).as_deref() == Some(super::RECORD_ROW)).count();
    assert_eq!(rows_named(&document(&with_ceiling(square(4.0), None))), 0);
    let sloped = document(&with_ceiling(square(4.0), Some(Slope { direction: 0.0, angle: 0.1 })));
    let record = rows(&sloped, "IFCPROPERTYSINGLEVALUE").into_iter().find(|(_, args)| string(args, 0).as_deref() == Some(super::RECORD_ROW)).expect("the authored record row");
    let text = record.1[2].as_typed().and_then(|(_, items)| items.first()).and_then(|item| item.as_str()).expect("a string");
    assert!(text.contains("\"slope\"") && text.contains("Hall ceiling"), "{text}");
}

#[test]
fn a_ceiling_is_contained_in_its_storey_and_typed_by_its_covering_type() {
    let document = document(&with_ceiling(square(4.0), None));
    let (covering, _) = rows(&document, "IFCCOVERING").into_iter().next().expect("a covering");
    let contained = rows(&document, "IFCRELCONTAINEDINSPATIALSTRUCTURE").into_iter().any(|(_, relation)| relation[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id() == Some(covering.id))));
    assert!(contained, "the covering sits in a storey");
    let typed = rows(&document, "IFCRELDEFINESBYTYPE").into_iter().any(|(_, relation)| relation[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id() == Some(covering.id))) && target(&document, relation, 5).is_some_and(|kind| kind.entity("IFCCOVERINGTYPE").is_some()));
    assert!(typed, "the covering is typed by its IfcCoveringType");
}
