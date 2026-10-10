use super::compute::{take_computed, take_hashed};
use super::*;
use crate::{Axis, Entry, MaterialPatch, ModelDiff, Point2, StoreyPatch, Wall, WallPatch};
use protocol::{Inference, InferredField};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🏠️house/📸️snapshot/🔣️.json");
const FULL: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");
const JOINS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🔗️wall-joins/📸️snapshot/🔣️.json");

fn decode(text: &str) -> ModelSnapshot {
    from_json_str(text, JsonMemberPolicy::Reject).expect("the snapshot decodes")
}

fn full() -> ModelSnapshot {
    decode(FULL)
}

fn house() -> ModelSnapshot {
    decode(HOUSE)
}

fn line(from: (f64, f64), to: (f64, f64)) -> Axis {
    Axis::Line { start: Point2 { x: from.0, y: from.1 }, end: Point2 { x: to.0, y: to.1 } }
}

fn rooms_grid(rows: usize, columns: usize) -> ModelSnapshot {
    let mut snapshot = house();
    let template: Wall = snapshot.walls["w-ground-south"].clone();
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
            snapshot.walls.insert(format!("h-{row:03}-{column:03}"), Wall { axis: line((x, y), (x + side, y)), storey: "st-ground".into(), ..template.clone() });
        }
    }
    for column in 0..=columns {
        for row in 0..rows {
            let (x, y) = (column as f64 * side, row as f64 * side);
            snapshot.walls.insert(format!("v-{column:03}-{row:03}"), Wall { axis: line((x, y), (x, y + side)), storey: "st-ground".into(), ..template.clone() });
        }
    }
    snapshot
}

#[test]
fn the_plan_is_topological_for_every_selection_and_names_only_planned_parents() {
    for snapshot in [full(), decode(JOINS)] {
        for wanted in [kinds::ALL, kinds::LEVELS, kinds::LAYOUTS, kinds::FRAMES, kinds::SOLIDS, kinds::ROOMS, kinds::PLANS, kinds::QUANTITIES, kinds::DIAGNOSTICS, kinds::RUNS, kinds::CURTAINS] {
            let steps = plan::build(&snapshot, kinds::closure(wanted));
            let position: BTreeMap<&ModelNode, usize> = steps.iter().enumerate().map(|(index, step)| (&step.key, index)).collect();
            assert_eq!(position.len(), steps.len(), "keys are unique");
            for step in &steps {
                for parent in &step.parents {
                    assert!(position.get(parent).is_some_and(|at| *at < position[&step.key]), "{:?} names {parent:?} before it is planned", step.key);
                }
            }
        }
    }
}

#[test]
fn a_selection_plans_only_its_kinds_and_their_ancestors() {
    let snapshot = full();
    let kinds_of = |wanted: u64| plan::build(&snapshot, kinds::closure(wanted)).iter().map(|step| step.key.kind()).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(kinds_of(kinds::LEVELS), [NodeKind::Storey].into());
    assert_eq!(kinds_of(kinds::LAYOUTS), [NodeKind::Storey, NodeKind::Band, NodeKind::WallLayout].into());
    assert!(!kinds_of(kinds::SOLIDS).contains(&NodeKind::Plan) && kinds_of(kinds::SOLIDS).contains(&NodeKind::OpeningFrame));
    assert_eq!(kinds_of(kinds::ALL).len(), NodeKind::ALL.len(), "the house exercises every kind");
}

#[test]
fn the_graph_is_deterministic_and_the_empty_model_infers_the_default() {
    let snapshot = house();
    assert_eq!(ModelInference::infer(&snapshot).expect("infers"), ModelInference::infer(&snapshot).expect("infers"));
    assert_eq!(ModelInference::infer(&ModelSnapshot::default()).expect("infers"), ModelInference::default());
}

#[test]
fn every_projection_is_cache_transparent_warm_equals_cold_equals_uncached() {
    for snapshot in [full(), decode(JOINS)] {
        let uncached = ModelInference::infer(&snapshot).expect("infers");
        let mut session = ModelInferenceSession::new();
        let cold = session.refresh(&snapshot).clone();
        assert!(session.report().computed > 0 && !session.report().gated);
        let warm = session.refresh(&snapshot).clone();
        assert_eq!(session.report().computed, 0, "a second refresh is all cache hits");
        for (name, equal) in [
            ("storey_levels", cold.storey_levels == uncached.storey_levels && warm.storey_levels == uncached.storey_levels),
            ("wall_layout", cold.wall_layout == uncached.wall_layout && warm.wall_layout == uncached.wall_layout),
            ("curtain_layout", cold.curtain_layout == uncached.curtain_layout && warm.curtain_layout == uncached.curtain_layout),
            ("stair_runs", cold.stair_runs == uncached.stair_runs && warm.stair_runs == uncached.stair_runs),
            ("spaces", cold.spaces == uncached.spaces && warm.spaces == uncached.spaces),
            ("opening_frames", cold.opening_frames == uncached.opening_frames && warm.opening_frames == uncached.opening_frames),
            ("element_solids", cold.element_solids == uncached.element_solids && warm.element_solids == uncached.element_solids),
            ("plan_linework", cold.plan_linework == uncached.plan_linework && warm.plan_linework == uncached.plan_linework),
            ("diagnostics", cold.diagnostics == uncached.diagnostics && warm.diagnostics == uncached.diagnostics),
            ("quantities", cold.quantities == uncached.quantities && warm.quantities == uncached.quantities),
        ] {
            assert!(equal, "{name}: cold, warm and uncached agree");
        }
        assert_eq!(cold, uncached);
        assert_eq!(warm, uncached);
    }
}

#[test]
fn the_thin_projections_equal_the_whole_inference() {
    use super::super::super::{curtain_layout, diagnostics, element_solids, opening_frames, plan_linework, quantities, spaces, stair_runs, storey_levels, wall_layout};
    for snapshot in [full(), decode(JOINS)] {
        let whole = ModelInference::infer(&snapshot).expect("infers");
        assert_eq!(storey_levels::compute_storey_levels(&snapshot), whole.storey_levels);
        assert_eq!(wall_layout::compute_wall_layout(&snapshot), whole.wall_layout);
        assert_eq!(curtain_layout::compute_curtain_layout(&snapshot), whole.curtain_layout);
        assert_eq!(stair_runs::compute_stair_runs(&snapshot), whole.stair_runs);
        assert_eq!(spaces::compute_spaces(&snapshot), whole.spaces);
        assert_eq!(opening_frames::compute_opening_frames(&snapshot), whole.opening_frames);
        assert_eq!(element_solids::compute_element_solids(&snapshot), whole.element_solids);
        assert_eq!(plan_linework::compute_plan_linework(&snapshot), whole.plan_linework);
        assert_eq!(diagnostics::compute_diagnostics(&snapshot), whole.diagnostics);
        assert_eq!(quantities::compute_quantities(&snapshot, &whole), whole.quantities);
    }
}

#[test]
fn without_a_cache_the_engine_asks_for_no_dependency() {
    let snapshot = house();
    take_hashed();
    let _ = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(take_hashed(), 0, "the uncached run never hashes");
    let mut session = ModelInferenceSession::new();
    session.refresh(&snapshot);
    assert_eq!(take_hashed(), session.report().nodes, "a cached run hashes every node once");
}

#[test]
fn an_update_returns_the_stored_result_when_the_diff_touches_nothing_the_graph_reads() {
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).clone();
    let again = session.update(&snapshot, &ModelDiff::default()).clone();
    assert!(session.report().gated && session.report().computed == 0 && first == again);
}

#[test]
fn a_session_update_equals_a_fresh_inference_after_every_kind_of_edit() {
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let edits = [
        ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(3.4), ..Default::default() })),
        ModelDiff::walls("w-ground-south", Entry::Patched(WallPatch { axis: Some(line((0.0, 0.5), (8.0, 0.5))), ..Default::default() })),
        ModelDiff::materials("m-brick", Entry::Patched(MaterialPatch { density: Some(1900.0), ..Default::default() })),
    ];
    let mut current = snapshot;
    for edit in edits {
        current = protocol::apply_diff(&edit, &current).expect("the edit applies");
        let incremental = session.update(&current, &edit).clone();
        assert_eq!(incremental, ModelInference::infer(&current).expect("infers"), "the incremental inference equals a fresh one");
    }
}

#[test]
fn a_session_removes_the_entries_of_nodes_that_leave_the_graph() {
    let mut snapshot = full();
    let mut session = ModelInferenceSession::new();
    session.refresh(&snapshot);
    let doomed: Vec<String> = snapshot.openings.iter().filter(|(_, opening)| opening.host == "w-south").map(|(id, _)| id.clone()).collect();
    assert!(!doomed.is_empty());
    for id in &doomed {
        snapshot.openings.remove(id);
    }
    snapshot.walls.remove("w-south");
    let after = session.refresh(&snapshot).clone();
    assert!(!after.wall_layout.contains_key("w-south") && doomed.iter().all(|id| !after.opening_frames.contains_key(id) && !after.element_solids.contains_key(id) && !after.quantities.elements.contains_key(id)));
    assert_eq!(after, ModelInference::infer(&snapshot).expect("infers"));
}

#[test]
fn moving_one_wall_recomputes_only_its_neighbourhood() {
    let snapshot = rooms_grid(10, 10);
    let walls = snapshot.walls.len();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let first = session.report().clone();
    assert!(first.computed_by_kind["wall-layout"] == walls && first.computed_by_kind["band"] == walls, "the first run computes every wall once");
    let moved = ModelDiff::walls("h-005-005", Entry::Patched(WallPatch { axis: Some(line((20.0, 20.4), (24.0, 20.4))), ..Default::default() }));
    let edited = protocol::apply_diff(&moved, &snapshot).expect("applies");
    let incremental = session.update(&edited, &moved).clone();
    let report = session.report().clone();
    assert_eq!(report.computed_by_kind.get("band"), Some(&1), "only the moved wall's band is recomputed: {report:?}");
    let layouts = report.computed_by_kind["wall-layout"];
    assert!((1..=40).contains(&layouts) && layouts * 5 < walls, "{layouts} of {walls} layouts: the wall and the walls within two touches of it");
    assert_eq!(report.computed_by_kind.get("storey"), None, "no storey is recomputed");
    assert!(report.computed < report.nodes / 4, "{} of {} nodes", report.computed, report.nodes);
    assert_eq!(report.computed_by_kind.get("plan"), Some(&1), "the plan of the storey is redrawn once");
    assert_eq!(report.computed_by_kind.get("totals"), Some(&3), "the storey, the building and the project aggregates");
    assert_eq!(incremental, ModelInference::infer(&edited).expect("infers"), "and the result equals a fresh inference");
}

#[test]
fn moving_a_window_recomputes_its_frame_its_host_wall_and_filler_only() {
    use crate::OpeningPatch;
    let snapshot = full();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let moved = ModelDiff::openings("o-win-1", Entry::Patched(OpeningPatch { offset: Some(2.5), ..Default::default() }));
    let edited = protocol::apply_diff(&moved, &snapshot).expect("applies");
    let incremental = session.update(&edited, &moved).clone();
    let report = session.report().clone();
    assert_eq!(report.computed_by_kind.get("wall-layout"), None, "a moved opening does not touch any layout");
    assert_eq!(report.computed_by_kind.get("storey"), None);
    assert!(report.computed_by_kind.get("opening-frame").copied().unwrap_or(0) >= 1);
    assert_eq!(incremental, ModelInference::infer(&edited).expect("infers"));
}

#[test]
fn a_node_keeps_the_key_it_was_computed_for() {
    let snapshot = house();
    let steps = plan::build(&snapshot, kinds::closure(kinds::ALL));
    let mut values: BTreeMap<ModelNode, ModelValue> = BTreeMap::new();
    for step in &steps {
        let parents: Vec<ModelValue> = step.parents.iter().map(|parent| values[parent].clone()).collect();
        let value = <ModelGraph<{ kinds::ALL }> as InferredField<ModelSnapshot>>::compute(&snapshot, &step.key, &parents);
        assert_eq!(value.node, step.key);
        values.insert(step.key.clone(), value);
    }
    take_computed();
}
