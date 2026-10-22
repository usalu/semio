use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use std::collections::BTreeMap;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout::StoreyAnnotations;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout::CurtainLayout;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::{Diagnostic, DiagnosticIndex};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::{SolidEntry, SolidKey};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components::ComponentEntry;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::FamilyValue;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep::clash::MepClashes;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep::MepValue;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets::{ClashSetResult, SolidProbe};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results::RuleResult;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames::{CutRect, HostExtent, OpeningFrame};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility::PhaseVisibility;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework::PlanLinework;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::{ElementQuantity, QuantityTotals};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules::ScheduleTable;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::{SchemeTotals, ZoneTotals};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces::StoreyRooms;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs::StairRun;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs::RampRun;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::joins::Band;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout::SheetLayout;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::attach::AttachSurface;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties::EffectiveProperties;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::{EnergyScope, EnergyTotals, EnvelopeSpace};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::WallLayout;
#[allow(unused_imports)]
use semio_framework_value::DslValue;
#[allow(unused_imports)]
use std::sync::Arc;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::compute::{take_computed, take_hashed};
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, plan, ModelNode, NodeKind, ModelInferenceSession, ModelValue, ModelGraph};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Axis, Point2};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Wall;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::MaterialPatch;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::site_and_spaces::StoreyPatch;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::walls_and_curtain_walls::WallPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{Inference, InferredField};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_framework_test_fixtures::fixture_text;

static HOUSE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/💡️inferences/🏠️house/📸️snapshot/🔣️.json"));
static FULL: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json"));
static JOINS: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/💡️inferences/🔗️wall-joins/📸️snapshot/🔣️.json"));

fn decode(text: &str) -> ModelSnapshot {
    from_json_str(text, JsonMemberPolicy::Reject).expect("the snapshot decodes")
}

fn full() -> ModelSnapshot {
    decode((*FULL))
}

fn house() -> ModelSnapshot {
    decode((*HOUSE))
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
    for snapshot in [full(), decode((*JOINS))] {
        for wanted in [semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::ALL, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::LEVELS, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::LAYOUTS, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::FRAMES, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::SOLIDS, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::ROOMS, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::PLANS, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::QUANTITIES, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::DIAGNOSTICS, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::RUNS, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::CURTAINS] {
            let steps = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::plan::build(&snapshot, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::closure(wanted));
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
    let kinds_of = |wanted: u64| semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::plan::build(&snapshot, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::closure(wanted)).iter().map(|step| step.key.kind()).collect::<std::collections::BTreeSet<_>>();
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
    for snapshot in [full(), decode((*JOINS))] {
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
    use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::{curtain_layout, stair_runs, storey_levels};
    use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
    use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::{opening_frames, wall_layout};
    use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
    use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities;
    use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces;
    for snapshot in [full(), decode((*JOINS))] {
        let whole = ModelInference::infer(&snapshot).expect("infers");
        assert_eq!(semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone::compute_storey_levels(&snapshot), whole.storey_levels);
        assert_eq!(semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone::compute_wall_layout(&snapshot), whole.wall_layout);
        assert_eq!(semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone::compute_curtain_layout(&snapshot), whole.curtain_layout);
        assert_eq!(semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone::compute_stair_runs(&snapshot), whole.stair_runs);
        assert_eq!(semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone::compute_spaces(&snapshot), whole.spaces);
        assert_eq!(semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone::compute_opening_frames(&snapshot), whole.opening_frames);
        assert_eq!(semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone::compute_element_solids(&snapshot), whole.element_solids);
        assert_eq!(semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone::compute_plan_linework(&snapshot), whole.plan_linework);
        assert_eq!(semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone::compute_diagnostics(&snapshot), whole.diagnostics);
        assert_eq!(semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone::compute_quantities(&snapshot, &whole), whole.quantities);
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
        current = protocol::apply_diff_cold(&edit, &current).expect("the edit applies");
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
    let edited = protocol::apply_diff_cold(&moved, &snapshot).expect("applies");
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
    use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::ceilings_roofs_openings::OpeningPatch;
    let snapshot = full();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let moved = ModelDiff::openings("o-win-1", Entry::Patched(OpeningPatch { offset: Some(2.5), ..Default::default() }));
    let edited = protocol::apply_diff_cold(&moved, &snapshot).expect("applies");
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
    let steps = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::plan::build(&snapshot, semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::closure(semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::ALL));
    let mut values: BTreeMap<ModelNode, ModelValue> = BTreeMap::new();
    for step in &steps {
        let parents: Vec<ModelValue> = step.parents.iter().map(|parent| values[parent].clone()).collect();
        let value = <ModelGraph<{ semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::kinds::ALL }> as InferredField<ModelSnapshot>>::compute(&snapshot, &step.key, &parents);
        assert_eq!(value.node, step.key);
        values.insert(step.key.clone(), value);
    }
    take_computed();
}
