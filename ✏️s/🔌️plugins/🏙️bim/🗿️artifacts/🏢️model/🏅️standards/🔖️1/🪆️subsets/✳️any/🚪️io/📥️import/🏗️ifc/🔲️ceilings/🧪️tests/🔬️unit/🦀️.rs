use super::*;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::house;
use crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3;
use crate::standards::v1::subsets::any::io::import::ifc::import_ifc2x3;
use crate::{CeilingType, Layer, LayerFunction, ModelSnapshot, Slope};

fn vertex(x: f64, y: f64) -> Vertex {
    Vertex { point: Point2 { x, y }, bulge: 0.0 }
}

fn layer(material: &str, thickness: f64) -> Layer {
    Layer { material: material.into(), thickness, function: LayerFunction::Finish }
}

/// 🔲️ The house with a board ceiling (a 6 x 4 m outline with a 1 x 1 m hole, 0.3 m below the top of the ground storey), a tile ceiling and a sloped board ceiling on the first storey.
fn with_ceilings() -> ModelSnapshot {
    let mut model = house();
    let storey = model.storeys.iter().find(|(_, storey)| storey.level == 0).map(|(id, _)| id.clone()).expect("the ground storey");
    let upper = model.storeys.iter().find(|(_, storey)| storey.level == 1).map(|(id, _)| id.clone()).expect("the first storey");
    let material = model.materials.keys().next().cloned().expect("a material");
    model.ceiling_types.insert("ct-board".into(), CeilingType { name: "Board on wool".into(), layers: vec![layer(&material, 0.0125), layer(&material, 0.05)] });
    model.ceiling_types.insert("ct-tile".into(), CeilingType { name: "Tile".into(), layers: vec![layer(&material, 0.02)] });
    let outline = vec![vertex(0.0, 0.0), vertex(6.0, 0.0), vertex(6.0, 4.0), vertex(0.0, 4.0)];
    let hole = vec![vertex(2.0, 1.0), vertex(2.0, 2.0), vertex(3.0, 2.0), vertex(3.0, 1.0)];
    let ceiling = |storey: &str, kind: &str, boundary: Vec<Vertex>, holes: Vec<Vec<Vertex>>, offset: f64, slope: Option<Slope>, name: &str| Ceiling { storey: storey.into(), ceiling_type: kind.into(), boundary, holes, offset, slope, name: name.into() };
    model.ceilings.insert("ce-board".into(), ceiling(&storey, "ct-board", outline.clone(), vec![hole], 0.3, None, "Hall Ceiling"));
    model.ceilings.insert("ce-tile".into(), ceiling(&storey, "ct-tile", vec![vertex(7.0, 0.0), vertex(9.0, 0.0), vertex(9.0, 3.0), vertex(7.0, 3.0)], Vec::new(), 0.15, None, ""));
    model.ceilings.insert("ce-raked".into(), ceiling(&upper, "ct-board", outline, Vec::new(), 0.2, Some(Slope { direction: std::f64::consts::FRAC_PI_2, angle: 0.05 }), "Raked Ceiling"));
    model
}

fn canonical(ring: &[Vertex]) -> Vec<[i64; 2]> {
    let area: f64 = ring.iter().zip(ring.iter().cycle().skip(1)).map(|(a, b)| a.point.x * b.point.y - b.point.x * a.point.y).sum();
    let mut points: Vec<[i64; 2]> = ring.iter().map(|vertex| [(vertex.point.x * 1e6).round() as i64, (vertex.point.y * 1e6).round() as i64]).collect();
    if area < 0.0 {
        points.reverse();
    }
    let first = points.iter().enumerate().min_by_key(|(_, point)| **point).map_or(0, |(index, _)| index);
    points.rotate_left(first);
    points
}

fn same(a: &Ceiling, b: &Ceiling) -> bool {
    let shape = |ceiling: &Ceiling| (canonical(&ceiling.boundary), ceiling.holes.iter().map(|hole| canonical(hole)).collect::<Vec<_>>());
    (a.offset - b.offset).abs() < 1e-9 && shape(a) == shape(b) && (&a.storey, &a.ceiling_type, &a.name, a.slope) == (&b.storey, &b.ceiling_type, &b.name, b.slope)
}

#[test]
fn a_flat_ceiling_with_a_hole_survives_the_round_trip() {
    let model = with_ceilings();
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let (back, notes) = import_ifc2x3(&bytes).expect("the file imports");
    assert!(notes.iter().all(|note| !note.contains("IFCCOVERING")), "{notes:?}");
    assert_eq!(back.ceilings.keys().collect::<Vec<_>>(), ["ce-board", "ce-raked", "ce-tile"]);
    for id in ["ce-board", "ce-tile"] {
        assert!(same(&back.ceilings[id], &model.ceilings[id]), "{id}: {:?} against {:?}", back.ceilings[id], model.ceilings[id]);
    }
}

#[test]
fn a_sloped_ceiling_is_restored_from_its_authored_record() {
    let model = with_ceilings();
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let (back, _) = import_ifc2x3(&bytes).expect("the file imports");
    assert_eq!(back.ceilings["ce-raked"], model.ceilings["ce-raked"]);
}

#[test]
fn the_ceiling_types_keep_their_layers_and_the_ceilings_keep_their_type() {
    let model = with_ceilings();
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let (back, _) = import_ifc2x3(&bytes).expect("the file imports");
    assert_eq!(back.ceiling_types, model.ceiling_types);
    assert_eq!(back.ceilings["ce-board"].ceiling_type, "ct-board");
    assert_eq!(back.ceilings["ce-tile"].ceiling_type, "ct-tile");
}

#[test]
fn a_second_round_trip_changes_nothing() {
    let (first, _) = export_ifc2x3(&with_ceilings()).expect("the model exports");
    let (once, _) = import_ifc2x3(&first).expect("the file imports");
    let (second, _) = export_ifc2x3(&once).expect("the imported model exports");
    let (twice, _) = import_ifc2x3(&second).expect("the file imports again");
    assert_eq!(once.ceilings, twice.ceilings);
    assert_eq!(once.ceiling_types, twice.ceiling_types);
}

#[test]
fn a_covering_that_is_not_a_ceiling_is_reported_not_imported() {
    let (bytes, _) = export_ifc2x3(&with_ceilings()).expect("the model exports");
    let text = String::from_utf8(bytes).expect("a text file");
    let edited: Vec<String> = text.lines().map(|line| if line.contains("=IFCCOVERING(") { line.replace(".CEILING.", ".FLOORING.") } else { line.to_string() }).collect();
    let (back, notes) = import_ifc2x3(edited.join("\n").as_bytes()).expect("the file imports");
    assert!(back.ceilings.is_empty());
    assert!(notes.iter().any(|note| note.contains("IFCCOVERING: 3 not imported")), "{notes:?}");
}

#[test]
fn an_untyped_ceiling_gets_a_single_layer_type_as_thick_as_its_sweep() {
    let (bytes, _) = export_ifc2x3(&with_ceilings()).expect("the model exports");
    let text = String::from_utf8(bytes).expect("a text file");
    let edited: Vec<&str> = text.lines().filter(|line| !line.contains("=IFCRELDEFINESBYTYPE(")).collect();
    let (back, _) = import_ifc2x3(edited.join("\n").as_bytes()).expect("the file imports");
    for (id, thickness) in [("ce-tile", 0.02), ("ce-board", 0.0625)] {
        let kind = &back.ceiling_types[&back.ceilings[id].ceiling_type];
        assert_eq!(kind.layers.len(), 1, "{id}: an untyped ceiling gets one layer: {kind:?}");
        assert!((kind.layers[0].thickness - thickness).abs() < 1e-9, "{id}: the sweep gives the thickness: {kind:?}");
    }
}

#[test]
fn a_ceiling_with_a_bad_authored_record_is_skipped() {
    let (bytes, _) = export_ifc2x3(&with_ceilings()).expect("the model exports");
    let text = String::from_utf8(bytes).expect("a text file").replace("\"offset\"", "\"offsets\"");
    let (back, notes) = import_ifc2x3(text.as_bytes()).expect("the file imports");
    assert!(!back.ceilings.contains_key("ce-raked"));
    assert!(notes.iter().any(|note| note.contains("IFCCOVERING") && note.contains("not valid")), "{notes:?}");
}
