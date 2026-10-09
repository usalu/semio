use super::compute::{take_computed, take_hashed};
use super::*;
use crate::{Axis, Entry, ModelDiff, Opening, Point2, Wall, WallPatch};
use std::time::Instant;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const FULL: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");

fn line(from: (f64, f64), to: (f64, f64)) -> Axis {
    Axis::Line { start: Point2 { x: from.0, y: from.1 }, end: Point2 { x: to.0, y: to.1 } }
}

fn big_model(rows: usize, columns: usize, windows: usize) -> ModelSnapshot {
    let mut snapshot: ModelSnapshot = from_json_str(FULL, JsonMemberPolicy::Reject).expect("the snapshot decodes");
    let template: Wall = snapshot.walls["w-south"].clone();
    let window = snapshot.openings["o-win-1"].clone();
    snapshot.walls.clear();
    snapshot.openings.clear();
    snapshot.curtain_walls.clear();
    snapshot.spaces.clear();
    snapshot.columns.clear();
    snapshot.beams.clear();
    snapshot.slabs.clear();
    snapshot.stairs.clear();
    snapshot.railings.clear();
    snapshot.roofs.clear();
    let side = 4.0;
    for row in 0..=rows {
        for column in 0..columns {
            let (x, y) = (column as f64 * side, row as f64 * side);
            snapshot.walls.insert(format!("h-{row:03}-{column:03}"), Wall { axis: line((x, y), (x + side, y)), ..template.clone() });
        }
    }
    for column in 0..=columns {
        for row in 0..rows {
            let (x, y) = (column as f64 * side, row as f64 * side);
            snapshot.walls.insert(format!("v-{column:03}-{row:03}"), Wall { axis: line((x, y), (x, y + side)), ..template.clone() });
        }
    }
    let hosts: Vec<String> = snapshot.walls.keys().filter(|id| id.starts_with("h-")).step_by(3).take(windows).cloned().collect();
    for (at, host) in hosts.into_iter().enumerate() {
        snapshot.openings.insert(format!("o-{at:03}"), Opening { host, offset: 1.5, ..window.clone() });
    }
    snapshot
}

/// 🏁️ The release benchmark of the incremental update: one wall moved in a model of 511 walls and 64 windows. Run `cargo test --release -p semio-s-artifact-bim-model --lib -- --ignored --nocapture one_wall_move`.
#[test]
#[ignore = "release benchmark"]
fn one_wall_move_in_511_walls_and_64_windows_updates_in_under_15_ms() {
    let mut snapshot = big_model(15, 16, 64);
    assert_eq!((snapshot.walls.len(), snapshot.openings.len()), (511, 64));
    let mut session = ModelInferenceSession::new();
    let start = Instant::now();
    session.update(&snapshot, &ModelDiff::default());
    let first = start.elapsed();
    let nodes = session.report().nodes;
    let mut samples = Vec::new();
    let mut last = session.report().clone();
    for row in 1..=9usize {
        let y = row as f64 * 4.0 + 0.5;
        let moved = ModelDiff::walls(format!("h-{row:03}-007"), Entry::Patched(WallPatch { axis: Some(line((28.0, y), (32.0, y))), ..Default::default() }));
        snapshot = protocol::apply_diff(&moved, &snapshot).expect("applies");
        let start = Instant::now();
        session.update(&snapshot, &moved);
        samples.push(start.elapsed());
        last = session.report().clone();
    }
    samples.sort();
    let median = samples[samples.len() / 2];
    let start = Instant::now();
    let fresh = crate::ModelInference::infer(&snapshot).expect("infers");
    let fresh_time = start.elapsed();
    assert_eq!(session.inference(), &fresh, "the incremental result equals a fresh inference");
    println!("bench one wall move: nodes {nodes}, first update {first:?}, move min {:?} median {median:?} max {:?}, fresh infer {fresh_time:?}, last report {last:?}", samples[0], samples[samples.len() - 1]);
    take_computed();
    take_hashed();
    if !cfg!(debug_assertions) {
        assert!(median.as_millis() < 15, "one wall move took {median:?}");
    }
}

mod annotations {
    use super::super::compute::take_hashed;
    use super::super::*;
    use crate::{AnnotationStylePatch, ColumnPatch, DimensionPatch, Entry, ModelDiff, OpeningPatch, Point2, ProjectPatch, StoreyPatch, TextNotePatch, WallTypePatch};
    use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

    const ROOM: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot/🔣️.json");

    fn room() -> ModelSnapshot {
        from_json_str(ROOM, JsonMemberPolicy::Reject).expect("the room decodes")
    }

    fn settle(session: &mut ModelInferenceSession, snapshot: &ModelSnapshot) {
        session.update(snapshot, &ModelDiff::default());
    }

    #[test]
    fn annotations_are_cache_transparent_warm_equals_cold_equals_uncached() {
        let snapshot = room();
        let uncached = ModelInference::infer(&snapshot).expect("infers");
        assert!(!uncached.annotations["st-ground"].dimensions.is_empty() && !uncached.annotations["st-ground"].tags.is_empty());
        take_hashed();
        let mut session = ModelInferenceSession::new();
        let cold = session.refresh(&snapshot).clone();
        assert_eq!(session.report().computed_by_kind.get("annotation"), Some(&1), "one node per annotated storey: {:?}", session.report());
        assert_eq!(cold.annotations, uncached.annotations);
        let warm = session.refresh(&snapshot).clone();
        assert_eq!(session.report().computed, 0, "a second refresh is all cache hits");
        assert_eq!(warm.annotations, uncached.annotations);
        assert_eq!(warm, uncached);
        assert_eq!(cold, uncached);
        assert!(!uncached.annotations.contains_key("st-first"), "a storey without annotations has no node");
    }

    #[test]
    fn a_diff_the_graph_does_not_read_is_gated_and_computes_no_annotation() {
        let snapshot = room();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let before = session.inference().clone();
        let renamed = ModelDiff { project: Some(ProjectPatch { name: Some("Renamed".into()), ..Default::default() }), ..Default::default() };
        let after = protocol::apply_diff(&renamed, &snapshot).expect("applies");
        let served = session.update(&after, &renamed).clone();
        assert!(session.report().gated && session.report().computed == 0 && session.report().computed_by_kind.get("annotation").is_none(), "{:?}", session.report());
        assert_eq!(served, before);
        assert_eq!(served, ModelInference::infer(&after).expect("infers"));
    }

    #[test]
    fn an_annotation_diff_is_not_gated_and_recomputes_only_the_annotation_nodes_that_read_it() {
        let snapshot = room();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let edit = ModelDiff::text_notes("note-1", Entry::Patched(TextNotePatch { text: Some("Verified".into()), ..Default::default() }));
        let after = protocol::apply_diff(&edit, &snapshot).expect("applies");
        let incremental = session.update(&after, &edit).clone();
        let report = session.report().clone();
        assert!(!report.gated && report.computed_by_kind.get("annotation") == Some(&1), "{report:?}");
        assert_eq!(report.computed_by_kind.get("wall-layout"), None, "a note edit touches no layout");
        assert_eq!(report.computed_by_kind.get("storey"), None);
        assert_eq!(incremental.annotations["st-ground"].notes["note-1"].text, "Verified");
        assert_eq!(incremental, ModelInference::infer(&after).expect("infers"));
    }

    #[test]
    fn every_authored_input_of_an_annotation_recomputes_it_and_equals_a_fresh_inference() {
        let mut current = room();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &current);
        let edits: Vec<(&str, ModelDiff, bool)> = vec![
            ("dimension offset", ModelDiff::dimensions("dim-south", Entry::Patched(DimensionPatch { offset: Some(-1.5), ..Default::default() })), true),
            ("style precision", ModelDiff::annotation_styles("as-plan", Entry::Patched(AnnotationStylePatch { precision: Some(1), ..Default::default() })), true),
            ("anchored column", ModelDiff::columns("col-1", Entry::Patched(ColumnPatch { position: Some(Point2 { x: 5.0, y: 2.0 }), ..Default::default() })), true),
            ("anchored opening", ModelDiff::openings("o-win", Entry::Patched(OpeningPatch { offset: Some(3.0), ..Default::default() })), true),
            ("type name read by a tag", ModelDiff::wall_types("wt-300", Entry::Patched(WallTypePatch { name: Some("Brick 300 renamed".into()), ..Default::default() })), true),
            ("storey above", ModelDiff::storeys("st-first", Entry::Patched(StoreyPatch { height: Some(3.3), ..Default::default() })), false),
        ];
        for (what, edit, reads) in edits {
            current = protocol::apply_diff(&edit, &current).expect("the edit applies");
            let incremental = session.update(&current, &edit).clone();
            let report = session.report().clone();
            assert_eq!(report.computed_by_kind.get("annotation").copied().unwrap_or(0), usize::from(reads), "{what}: {report:?}");
            assert_eq!(incremental, ModelInference::infer(&current).expect("infers"), "{what}: the incremental result equals a fresh inference");
        }
        let set = &session.inference().annotations["st-ground"];
        assert_eq!(set.tags["tag-type"].text, "Brick 300 renamed");
        assert_eq!(set.dimensions["dim-south"].segments[0].text, "8.0");
    }

    #[test]
    fn deleting_the_annotations_of_a_storey_removes_its_node_and_its_entry() {
        let mut snapshot = room();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        snapshot.dimensions.clear();
        snapshot.tags.clear();
        snapshot.text_notes.clear();
        snapshot.leaders.clear();
        let after = session.refresh(&snapshot).clone();
        assert!(after.annotations.is_empty(), "no annotation, no node, no entry");
        assert_eq!(after, ModelInference::infer(&snapshot).expect("infers"));
    }
}

mod ceilings {
    use super::super::compute::take_hashed;
    use super::super::*;
    use crate::{Ceiling, CeilingPatch, CeilingType, CeilingTypePatch, Entry, Layer, LayerFunction, ModelDiff, Point2, ProjectPatch, Vertex};
    use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

    const HOUSE: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");

    fn corner(x: f64, y: f64) -> Vertex {
        Vertex { point: Point2 { x, y }, bulge: 0.0 }
    }

    fn layer(material: &str, thickness: f64) -> Layer {
        Layer { material: material.into(), thickness, function: LayerFunction::Finish }
    }

    fn hung_house() -> ModelSnapshot {
        let mut snapshot: ModelSnapshot = from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("the house decodes");
        let material = snapshot.materials.keys().next().cloned().expect("a material");
        snapshot.ceiling_types.insert("ct-board".into(), CeilingType { name: "Board".into(), layers: vec![layer(&material, 0.0625)] });
        snapshot.ceilings.insert("ce-hall".into(), Ceiling { storey: "st-ground".into(), ceiling_type: "ct-board".into(), boundary: vec![corner(-50.0, -50.0), corner(50.0, -50.0), corner(50.0, 50.0), corner(-50.0, 50.0)], holes: Vec::new(), offset: 0.6, slope: None, name: "Hall".into() });
        snapshot
    }

    fn settle(session: &mut ModelInferenceSession, snapshot: &ModelSnapshot) {
        session.update(snapshot, &ModelDiff::default());
    }

    #[test]
    fn ceilings_are_cache_transparent_warm_equals_cold_equals_uncached() {
        let snapshot = hung_house();
        let uncached = ModelInference::infer(&snapshot).expect("infers");
        assert!(uncached.element_solids.contains_key("ce-hall") && uncached.quantities.elements.contains_key("ce-hall") && !uncached.spaces["s-1"].ceiling.is_empty());
        take_hashed();
        let mut session = ModelInferenceSession::new();
        let cold = session.refresh(&snapshot).clone();
        assert!(session.report().computed_by_kind.get("solid").copied().unwrap_or(0) > 0 && !session.report().gated, "{:?}", session.report());
        let warm = session.refresh(&snapshot).clone();
        assert_eq!(session.report().computed, 0, "a second refresh is all cache hits");
        assert_eq!((cold, warm), (uncached.clone(), uncached));
    }

    #[test]
    fn a_ceiling_drop_recomputes_its_solid_and_the_rooms_of_its_storey_and_nothing_else() {
        let snapshot = hung_house();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let before = session.inference().spaces["s-1"].clear_height;
        let edit = ModelDiff::ceilings("ce-hall", Entry::Patched(CeilingPatch { offset: Some(0.9), ..Default::default() }));
        let after = protocol::apply_diff(&edit, &snapshot).expect("applies");
        let incremental = session.update(&after, &edit).clone();
        let report = session.report().clone();
        assert!(!report.gated, "{report:?}");
        assert_eq!((report.computed_by_kind.get("solid"), report.computed_by_kind.get("room")), (Some(&1), Some(&1)), "the solid of the ceiling and the rooms of the ground storey: {report:?}");
        for untouched in ["storey", "wall-layout", "band", "opening-frame", "stair-run"] {
            assert_eq!(report.computed_by_kind.get(untouched), None, "{untouched}: {report:?}");
        }
        assert!((before - incremental.spaces["s-1"].clear_height - 0.3).abs() < 1e-9, "the clear height drops by the 0.3 m the ceiling hangs lower");
        assert_eq!(incremental, ModelInference::infer(&after).expect("infers"), "the incremental result equals a fresh inference");
    }

    #[test]
    fn a_ceiling_type_edit_recomputes_the_solid_and_the_rooms_and_a_name_edit_neither() {
        let snapshot = hung_house();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let renamed = ModelDiff::ceilings("ce-hall", Entry::Patched(CeilingPatch { name: Some("Entrance Hall".into()), ..Default::default() }));
        let named = protocol::apply_diff(&renamed, &snapshot).expect("applies");
        let served = session.update(&named, &renamed).clone();
        let report = session.report().clone();
        assert_eq!((report.computed_by_kind.get("solid"), report.computed_by_kind.get("room")), (None, None), "{report:?}");
        assert_eq!(served, ModelInference::infer(&named).expect("infers"));
        let material = named.materials.keys().next().cloned().expect("a material");
        let thicker = ModelDiff::ceiling_types("ct-board", Entry::Patched(CeilingTypePatch { layers: Some(vec![layer(&material, 0.0625), layer(&material, 0.05)]), ..Default::default() }));
        let deeper = protocol::apply_diff(&thicker, &named).expect("applies");
        let incremental = session.update(&deeper, &thicker).clone();
        let report = session.report().clone();
        assert_eq!((report.computed_by_kind.get("solid"), report.computed_by_kind.get("room")), (Some(&1), Some(&1)), "{report:?}");
        assert_eq!(incremental, ModelInference::infer(&deeper).expect("infers"));
    }

    #[test]
    fn a_diff_the_graph_does_not_read_is_gated_and_deleting_the_last_ceiling_removes_its_nodes() {
        let mut snapshot = hung_house();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let renamed = ModelDiff { project: Some(ProjectPatch { name: Some("Renamed".into()), ..Default::default() }), ..Default::default() };
        let after = protocol::apply_diff(&renamed, &snapshot).expect("applies");
        session.update(&after, &renamed);
        assert!(session.report().gated && session.report().computed == 0, "{:?}", session.report());
        snapshot.ceilings.clear();
        let removed = session.refresh(&snapshot).clone();
        assert!(!removed.element_solids.contains_key("ce-hall") && !removed.quantities.elements.contains_key("ce-hall") && removed.spaces["s-1"].ceiling.is_empty());
        assert_eq!(removed, ModelInference::infer(&snapshot).expect("infers"));
    }
}
