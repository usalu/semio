use super::compute::{take_computed, take_hashed};
use super::*;
use protocol::Inference;
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
    use protocol::Inference;
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
    use protocol::Inference;
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

mod wall_depth {
    use super::super::compute::take_hashed;
    use super::super::*;
    use protocol::Inference;
    use crate::{Assigned, Entry, ModelDiff, ProjectPatch, RoofPatch, RoofShape, SlabPatch, Slope, WallPatch};
    use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

    const ATTIC: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧗️wall-depth/🏠️attic/📸️snapshot/🔣️.json");

    fn attic() -> ModelSnapshot {
        from_json_str(ATTIC, JsonMemberPolicy::Reject).expect("the attic decodes")
    }

    fn settle(session: &mut ModelInferenceSession, snapshot: &ModelSnapshot) {
        session.update(snapshot, &ModelDiff::default());
    }

    #[test]
    fn attached_walls_are_cache_transparent_warm_equals_cold_equals_uncached() {
        let snapshot = attic();
        let uncached = ModelInference::infer(&snapshot).expect("infers");
        assert!(uncached.wall_layout["w-east"].top_profile.len() == 3 && !uncached.wall_layout["w-base"].base_profile.is_empty() && uncached.element_solids.contains_key("ws-door"));
        take_hashed();
        let mut session = ModelInferenceSession::new();
        let cold = session.refresh(&snapshot).clone();
        assert_eq!(session.report().computed_by_kind.get("surface"), Some(&3), "two roofs and a slab: {:?}", session.report());
        let warm = session.refresh(&snapshot).clone();
        assert_eq!(session.report().computed, 0, "a second refresh is all cache hits");
        assert_eq!((cold, warm), (uncached.clone(), uncached));
    }

    #[test]
    fn a_roof_edit_recomputes_its_surface_and_the_walls_attached_to_it_and_nothing_else() {
        let snapshot = attic();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let before = session.inference().wall_layout["w-hip"].top_z;
        let edit = ModelDiff::roofs("r-hip", Entry::Patched(RoofPatch { shape: Some(RoofShape::Hip { pitch: 0.5 }), ..Default::default() }));
        let after = protocol::apply_diff(&edit, &snapshot).expect("applies");
        let incremental = session.update(&after, &edit).clone();
        let report = session.report().clone();
        assert!(!report.gated, "{report:?}");
        assert_eq!((report.computed_by_kind.get("surface"), report.computed_by_kind.get("wall-layout")), (Some(&1), Some(&1)), "the surface of the hip roof and the layout of the wall under it: {report:?}");
        for untouched in ["storey", "band", "opening-frame"] {
            assert_eq!(report.computed_by_kind.get(untouched), None, "{untouched}: {report:?}");
        }
        assert!(incremental.wall_layout["w-hip"].top_z > before, "a steeper hip roof lifts the ridge plateau");
        assert!(close_eq(incremental.wall_layout["w-east"].top_z, session_free_top("w-east", &snapshot)), "the gable room is untouched");
        assert_eq!(incremental, ModelInference::infer(&after).expect("infers"), "the incremental result equals a fresh inference");
    }

    fn close_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1e-12
    }

    fn session_free_top(wall: &str, snapshot: &ModelSnapshot) -> f64 {
        ModelInference::infer(snapshot).expect("infers").wall_layout[wall].top_z
    }

    #[test]
    fn a_slab_edit_moves_the_base_of_the_wall_that_stands_on_it_and_its_sweep() {
        let snapshot = attic();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let base_before = session.inference().wall_layout["w-base"].base_z;
        let edit = ModelDiff::slabs("sl-slope", Entry::Patched(SlabPatch { slope: Some(Assigned::new(Some(Slope { direction: 0.0, angle: 0.2 }))), ..Default::default() }));
        let after = protocol::apply_diff(&edit, &snapshot).expect("applies");
        let incremental = session.update(&after, &edit).clone();
        let report = session.report().clone();
        assert_eq!((report.computed_by_kind.get("surface"), report.computed_by_kind.get("wall-layout")), (Some(&1), Some(&1)), "{report:?}");
        assert_eq!(report.computed_by_kind.get("solid"), Some(&3), "the slab, the wall on it and its baseboard: {report:?}");
        assert!(incremental.wall_layout["w-base"].base_z < base_before, "the base fell with the slab");
        assert_eq!(incremental, ModelInference::infer(&after).expect("infers"));
    }

    #[test]
    fn an_edit_of_a_wall_that_attaches_to_nothing_computes_no_surface_and_a_foreign_diff_is_gated() {
        let snapshot = attic();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let edit = ModelDiff::walls("w-rail", Entry::Patched(WallPatch { base_offset: Some(0.1), ..Default::default() }));
        let after = protocol::apply_diff(&edit, &snapshot).expect("applies");
        let incremental = session.update(&after, &edit).clone();
        assert_eq!(session.report().computed_by_kind.get("surface"), None, "{:?}", session.report());
        assert_eq!(incremental, ModelInference::infer(&after).expect("infers"));
        let renamed = ModelDiff { project: Some(ProjectPatch { name: Some("Renamed".into()), ..Default::default() }), ..Default::default() };
        let named = protocol::apply_diff(&renamed, &after).expect("applies");
        session.update(&named, &renamed);
        assert!(session.report().gated && session.report().computed == 0, "{:?}", session.report());
    }

    #[test]
    fn deleting_the_last_attached_wall_removes_the_surface_nodes() {
        let mut snapshot = attic();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        for id in ["w-east", "w-north", "w-west", "w-south", "w-hip", "w-base"] {
            snapshot.walls.remove(id);
        }
        snapshot.wall_sweeps.remove("ws-slope");
        let after = session.refresh(&snapshot).clone();
        assert_eq!(after, ModelInference::infer(&snapshot).expect("infers"));
        assert_eq!(session.report().computed_by_kind.get("surface"), None, "no wall attaches any more, so no surface node is planned");
    }
}

mod sheets {
    use super::super::compute::take_hashed;
    use super::super::*;
    use protocol::Inference;
    use crate::{Entry, ModelDiff, Point2, ProjectPatch, Sheet, SheetPatch, StoreyPatch, Viewport, ViewportPatch};
    use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

    const FULL: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");

    fn placed() -> ModelSnapshot {
        let mut snapshot: ModelSnapshot = from_json_str(FULL, JsonMemberPolicy::Reject).expect("the house decodes");
        snapshot.sheets.insert("sh-plans".into(), Sheet::standard("A-101", "Plans"));
        snapshot.sheets.insert("sh-elevations".into(), Sheet::standard("A-301", "Elevations"));
        snapshot.viewports.insert("vp-ground".into(), Viewport::standard("sh-plans", "v-plan-st-ground", Point2 { x: 30.0, y: 30.0 }));
        snapshot.viewports.insert("vp-south".into(), Viewport::standard("sh-elevations", "v-elevation-south", Point2 { x: 30.0, y: 30.0 }));
        snapshot
    }

    fn settle(session: &mut ModelInferenceSession, snapshot: &ModelSnapshot) {
        session.update(snapshot, &ModelDiff::default());
    }

    fn apply(session: &mut ModelInferenceSession, snapshot: &mut ModelSnapshot, edit: &ModelDiff) -> ModelInference {
        *snapshot = protocol::apply_diff(edit, snapshot).expect("the edit applies");
        session.update(snapshot, edit).clone()
    }

    #[test]
    fn sheets_are_cache_transparent_warm_equals_cold_equals_uncached() {
        let snapshot = placed();
        let uncached = ModelInference::infer(&snapshot).expect("infers");
        assert_eq!(uncached.sheet_layouts.len(), 2);
        assert_eq!(uncached.sheet_layouts["sh-plans"].viewports.len(), 1);
        take_hashed();
        let mut session = ModelInferenceSession::new();
        let cold = session.refresh(&snapshot).clone();
        assert_eq!(session.report().computed_by_kind.get("sheet"), Some(&2), "one node per sheet: {:?}", session.report());
        assert_eq!(cold, uncached);
        let warm = session.refresh(&snapshot).clone();
        assert_eq!(session.report().computed, 0, "a second refresh is all cache hits");
        assert_eq!(warm, uncached);
    }

    #[test]
    fn a_diff_the_graph_does_not_read_is_gated_and_computes_no_sheet() {
        let mut snapshot = placed();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let before = session.inference().clone();
        let renamed = ModelDiff { project: Some(ProjectPatch { name: Some("Renamed".into()), ..Default::default() }), ..Default::default() };
        let served = apply(&mut session, &mut snapshot, &renamed);
        assert!(session.report().gated && session.report().computed == 0 && session.report().computed_by_kind.get("sheet").is_none(), "{:?}", session.report());
        assert_eq!(served, before);
        assert_eq!(served, ModelInference::infer(&snapshot).expect("infers"));
    }

    #[test]
    fn a_sheet_diff_recomputes_only_its_sheet_and_no_view() {
        let mut snapshot = placed();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let edit = ModelDiff::sheets("sh-plans", Entry::Patched(SheetPatch { drawn_by: Some("UG".into()), ..Default::default() }));
        let incremental = apply(&mut session, &mut snapshot, &edit);
        let report = session.report().clone();
        assert!(!report.gated && report.computed_by_kind.get("sheet") == Some(&1), "{report:?}");
        assert_eq!(report.computed_by_kind.get("view"), None, "a title block edit redraws no view");
        assert_eq!(report.computed_by_kind.get("storey"), None);
        assert_eq!(incremental, ModelInference::infer(&snapshot).expect("infers"));
    }

    #[test]
    fn a_viewport_diff_recomputes_the_sheet_that_holds_it_and_equals_a_fresh_inference() {
        let mut snapshot = placed();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let edits = [
            ModelDiff::viewports("vp-south", Entry::Patched(ViewportPatch { scale: Some(50), ..Default::default() })),
            ModelDiff::viewports("vp-south", Entry::Patched(ViewportPatch { position: Some(Point2 { x: 60.0, y: 40.0 }), ..Default::default() })),
            ModelDiff::viewports("vp-south", Entry::Patched(ViewportPatch { view: Some("v-elevation-east".into()), ..Default::default() })),
        ];
        for edit in edits {
            let incremental = apply(&mut session, &mut snapshot, &edit);
            let report = session.report().clone();
            assert_eq!(report.computed_by_kind.get("sheet"), Some(&1), "{report:?}");
            assert_eq!(incremental, ModelInference::infer(&snapshot).expect("infers"));
        }
        assert_eq!(session.inference().sheet_layouts["sh-elevations"].viewports[0].scale, 50);
    }

    #[test]
    fn a_model_edit_that_changes_a_drawn_view_recomputes_the_sheet_that_places_it() {
        let mut snapshot = placed();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let edit = ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(3.3), ..Default::default() }));
        let incremental = apply(&mut session, &mut snapshot, &edit);
        assert!(session.report().computed_by_kind.get("sheet").copied().unwrap_or(0) >= 1, "{:?}", session.report());
        assert_eq!(incremental, ModelInference::infer(&snapshot).expect("infers"));
    }

    #[test]
    fn deleting_the_last_sheet_removes_its_node_and_its_entry() {
        let mut snapshot = placed();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        snapshot.viewports.clear();
        snapshot.sheets.clear();
        let after = session.refresh(&snapshot).clone();
        assert!(after.sheet_layouts.is_empty(), "no sheet, no node, no entry");
        assert_eq!(after, ModelInference::infer(&snapshot).expect("infers"));
    }
}

mod coordination {
    use super::super::*;
    use crate::{Axis, BeamPatch, ClashSetPatch, Entry, Issue, IssuePriority, IssueStatus, ModelDiff, Point2, ProjectPatch, Rule, RuleKind, RuleScope, RuleSeverity};
    use protocol::Inference;
    use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

    const FRAME: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧨️clash-sets/🏢️frame/📸️snapshot/🔣️.json");

    fn frame() -> ModelSnapshot {
        let mut snapshot: ModelSnapshot = from_json_str(FRAME, JsonMemberPolicy::Reject).expect("the frame decodes");
        snapshot.rules.insert("rl-door".into(), Rule { name: "Door".into(), kind: RuleKind::MinDoorWidth, limit: 0.9, severity: RuleSeverity::Warning, scope: RuleScope::all() });
        snapshot
    }

    fn settle(session: &mut ModelInferenceSession, snapshot: &ModelSnapshot) {
        session.update(snapshot, &ModelDiff::default());
    }

    fn apply(session: &mut ModelInferenceSession, snapshot: &mut ModelSnapshot, edit: &ModelDiff) -> ModelInference {
        *snapshot = protocol::apply_diff(edit, snapshot).expect("the edit applies");
        session.update(snapshot, edit).clone()
    }

    fn clashes(inference: &ModelInference, set: &str) -> Vec<(String, String)> {
        inference.clash_sets[set].clashes.iter().map(|clash| (clash.first.clone(), clash.second.clone())).collect()
    }

    #[test]
    fn clash_sets_and_rules_are_cache_transparent_warm_equals_cold_equals_uncached() {
        let snapshot = frame();
        let uncached = ModelInference::infer(&snapshot).expect("infers");
        assert_eq!(uncached.clash_sets.len(), 3);
        assert_eq!(uncached.rule_results.len(), 1);
        assert!(uncached.clash_sets.values().any(|result| result.hard() > 0), "the frame has hard clashes");
        let mut session = ModelInferenceSession::new();
        let cold = session.refresh(&snapshot).clone();
        let report = session.report().clone();
        assert_eq!(report.computed_by_kind.get("clash-set"), Some(&3), "{report:?}");
        assert_eq!(report.computed_by_kind.get("rule"), Some(&1), "{report:?}");
        assert!(report.computed_by_kind.get("probe").copied().unwrap_or(0) >= 5, "{report:?}");
        assert_eq!(cold, uncached);
        let warm = session.refresh(&snapshot).clone();
        assert_eq!(session.report().computed, 0, "a second refresh is all cache hits");
        assert_eq!(warm, uncached);
    }

    #[test]
    fn a_diff_the_graph_does_not_read_computes_nothing_of_the_coordination() {
        let mut snapshot = frame();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let renamed = ModelDiff { project: Some(ProjectPatch { name: Some("Renamed".into()), ..Default::default() }), ..Default::default() };
        apply(&mut session, &mut snapshot, &renamed);
        assert!(session.report().gated && session.report().computed == 0, "{:?}", session.report());
        let issue = Issue { title: "Check".into(), description: String::new(), status: IssueStatus::Open, priority: IssuePriority::Normal, assignee: String::new(), author: "UG".into(), created: "2026-10-09".into(), labels: Vec::new(), elements: vec!["w-1".into()], clash: None, viewpoint: None };
        let served = apply(&mut session, &mut snapshot, &ModelDiff::issues("i-1", Entry::Created(issue)));
        assert!(session.report().gated && session.report().computed == 0, "an issue is authored data no inference reads: {:?}", session.report());
        assert_eq!(served, ModelInference::infer(&snapshot).expect("infers"));
    }

    #[test]
    fn a_clash_set_diff_recomputes_only_its_set_and_no_solid_or_probe() {
        let mut snapshot = frame();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let edit = ModelDiff::clash_sets("cs-frame", Entry::Patched(ClashSetPatch { tolerance: Some(0.01), ..Default::default() }));
        let incremental = apply(&mut session, &mut snapshot, &edit);
        let report = session.report().clone();
        assert_eq!(report.computed_by_kind.get("clash-set"), Some(&1), "{report:?}");
        assert_eq!(report.computed_by_kind.get("probe"), None, "{report:?}");
        assert_eq!(report.computed_by_kind.get("solid"), None, "{report:?}");
        assert_eq!(incremental, ModelInference::infer(&snapshot).expect("infers"));
    }

    #[test]
    fn moving_a_beam_rebuilds_its_probe_and_the_sets_that_pick_it_and_moves_the_clash() {
        let mut snapshot = frame();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        let before = session.inference().clone();
        assert!(clashes(&before, "cs-structure").contains(&("b-2".to_string(), "w-1".to_string())), "{:?}", clashes(&before, "cs-structure"));
        let away = Axis::Line { start: Point2 { x: 1.0, y: 9.0 }, end: Point2 { x: 5.0, y: 9.0 } };
        let edit = ModelDiff::beams("b-2", Entry::Patched(BeamPatch { axis: Some(away), ..Default::default() }));
        let incremental = apply(&mut session, &mut snapshot, &edit);
        let report = session.report().clone();
        assert_eq!(report.computed_by_kind.get("probe"), Some(&1), "one probe is rebuilt: {report:?}");
        assert!(report.computed_by_kind.get("clash-set").copied().unwrap_or(0) >= 2, "{report:?}");
        assert!(!clashes(&incremental, "cs-structure").contains(&("b-2".to_string(), "w-1".to_string())));
        assert_eq!(incremental, ModelInference::infer(&snapshot).expect("infers"));
    }

    #[test]
    fn deleting_the_last_clash_set_and_rule_removes_their_nodes_and_entries() {
        let mut snapshot = frame();
        let mut session = ModelInferenceSession::new();
        settle(&mut session, &snapshot);
        snapshot.clash_sets.clear();
        snapshot.rules.clear();
        let after = session.refresh(&snapshot).clone();
        assert!(after.clash_sets.is_empty() && after.rule_results.is_empty());
        assert_eq!(after, ModelInference::infer(&snapshot).expect("infers"));
    }

    #[test]
    fn the_plan_is_topological_and_names_only_planned_parents_for_the_coordination_selections() {
        let snapshot = frame();
        for wanted in [kinds::CLASHES, kinds::RULES, kinds::CLASHES | kinds::RULES] {
            let steps = plan::build(&snapshot, kinds::closure(wanted));
            let mut seen = std::collections::BTreeSet::new();
            for step in &steps {
                assert!(step.parents.iter().all(|parent| seen.contains(parent)), "{:?} names a parent that is not planned before it", step.key);
                seen.insert(step.key.clone());
            }
            assert!(steps.iter().any(|step| matches!(step.key, ModelNode::ClashSet(_))) == (wanted & kinds::CLASHES != 0));
        }
    }
}
