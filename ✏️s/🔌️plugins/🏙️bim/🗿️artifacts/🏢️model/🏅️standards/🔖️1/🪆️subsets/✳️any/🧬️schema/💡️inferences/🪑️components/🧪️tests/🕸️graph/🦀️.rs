use crate::standards::v1::subsets::any::schema::inferences::diagnostics::Severity;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, plan, ModelInferenceSession, ModelNode};
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanKind, PlanStyle};
use crate::standards::v1::subsets::any::schema::inferences::quantities::QuantityKind;
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use crate::{ComponentPatch, Entry, FamilyParameterPatch, MepElementPatch, ModelDiff, ModelSnapshot, Point3, ProjectPatch};
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use std::collections::BTreeSet;

const ROOM: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪑️components/🏠️room/📸️snapshot/🔣️.json");
const TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪑️components/🏠️room/💡️inference/🪑️components/🔣️.json");

fn room() -> ModelSnapshot {
    from_json_str(ROOM, JsonMemberPolicy::Reject).expect("the room decodes")
}

fn settle(session: &mut ModelInferenceSession, snapshot: &ModelSnapshot) {
    session.update(snapshot, &ModelDiff::default());
}

fn count(session: &ModelInferenceSession, kind: &str) -> usize {
    session.report().computed_by_kind.get(kind).copied().unwrap_or(0)
}

fn edited(session: &mut ModelInferenceSession, before: &ModelSnapshot, diff: ModelDiff) -> (ModelSnapshot, ModelInference) {
    let after = protocol::apply_diff(&diff, before).expect("applies");
    let incremental = session.update(&after, &diff).clone();
    assert_eq!(incremental, ModelInference::infer(&after).expect("infers"), "an incremental update equals a fresh inference");
    (after, incremental)
}

#[test]
fn components_and_runs_are_cache_transparent_warm_equals_cold_equals_uncached() {
    let snapshot = room();
    let uncached = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(uncached.components.len(), 16);
    assert_eq!(uncached.mep.len(), 6);
    let mut session = ModelInferenceSession::new();
    let cold = session.refresh(&snapshot).clone();
    assert_eq!(count(&session, "component"), 16, "{:?}", session.report());
    assert_eq!(count(&session, "mep"), 6);
    assert_eq!(count(&session, "mep-clash"), 1, "one scan per storey with runs");
    assert_eq!(cold, uncached);
    let warm = session.refresh(&snapshot).clone();
    assert_eq!(session.report().computed, 0, "a second refresh is all cache hits");
    assert_eq!(warm, uncached);
    assert!(!warm.components.is_empty() && !warm.mep.is_empty() && warm.element_solids.contains_key("c-table") && warm.element_solids.contains_key("m-supply") && !warm.plan_linework.is_empty());
    assert_eq!(warm.quantities, uncached.quantities);
    assert_eq!(warm.diagnostics, uncached.diagnostics);
    assert_eq!(warm.diagnostic_index, uncached.diagnostic_index);
}

#[test]
fn the_plan_is_topological_for_every_selection_that_involves_components() {
    let snapshot = room();
    for wanted in [kinds::COMPONENTS, kinds::MEPS, kinds::SOLIDS, kinds::PLANS, kinds::QUANTITIES, kinds::DIAGNOSTICS, kinds::ALL] {
        let steps = plan::build(&snapshot, kinds::closure(wanted));
        let position = |key: &ModelNode| steps.iter().position(|step| &step.key == key).unwrap_or_else(|| panic!("{key:?} is planned"));
        for step in &steps {
            for parent in &step.parents {
                assert!(position(parent) < position(&step.key), "{:?} before {:?}", parent, step.key);
            }
        }
    }
    let steps = plan::build(&snapshot, kinds::closure(kinds::COMPONENTS));
    let step = |id: &str| steps.iter().find(|step| step.key == ModelNode::Component(id.into())).expect("planned");
    assert_eq!(step("c-basin-south").parents, [ModelNode::Storey("st-ground".into()), ModelNode::Family("fam-basin".into()), ModelNode::WallLayout("w-south".into())]);
    assert_eq!(step("c-ghost").parents, [ModelNode::Storey("st-ground".into())], "a missing family and an unhosted component have no further parent");
    assert_eq!(step("c-nowhere").parents, [ModelNode::Storey("st-ground".into()), ModelNode::Family("fam-basin".into())], "a missing host is no parent");
}

#[test]
fn a_family_parameter_edit_recomputes_that_family_and_exactly_the_components_that_depend_on_it() {
    let snapshot = room();
    let mut session = ModelInferenceSession::new();
    settle(&mut session, &snapshot);
    let edit = ModelDiff::family_parameters("fam-chair.size", Entry::Patched(FamilyParameterPatch { value: Some("0.5 m".into()), ..Default::default() }));
    let (_, after) = edited(&mut session, &snapshot, edit);
    assert!(!session.report().gated);
    assert_eq!(count(&session, "family"), 1, "{:?}", session.report());
    assert_eq!(count(&session, "component"), 2, "the two chairs: {:?}", session.report());
    assert_eq!(count(&session, "solid"), 2);
    assert_eq!(count(&session, "mep"), 0);
    assert_eq!(count(&session, "mep-clash"), 0);
    assert!((after.components["c-chair"].footprint_area - 0.25).abs() < 1e-12);
}

#[test]
fn an_edit_of_a_family_without_overrides_recomputes_its_components_through_the_shared_family_value() {
    let snapshot = room();
    let mut session = ModelInferenceSession::new();
    settle(&mut session, &snapshot);
    let edit = ModelDiff::family_parameters("fam-basin.width", Entry::Patched(FamilyParameterPatch { value: Some("0.7 m".into()), ..Default::default() }));
    let (_, after) = edited(&mut session, &snapshot, edit);
    assert_eq!(count(&session, "family"), 1);
    assert_eq!(count(&session, "component"), 3, "the two basins and the nowhere basin: {:?}", session.report());
    assert!((after.components["c-basin-south"].footprint_area - 0.7 * 0.45).abs() < 1e-12);
}

#[test]
fn an_unrelated_edit_and_a_rename_compute_no_component_and_no_run() {
    let snapshot = room();
    let mut session = ModelInferenceSession::new();
    settle(&mut session, &snapshot);
    let before = session.inference().clone();
    let renamed = ModelDiff { project: Some(ProjectPatch { name: Some("Renamed".into()), ..Default::default() }), ..Default::default() };
    let after = protocol::apply_diff(&renamed, &snapshot).expect("applies");
    let served = session.update(&after, &renamed).clone();
    assert!(session.report().gated && session.report().computed == 0, "{:?}", session.report());
    assert_eq!(served, before);
    let rename = ModelDiff::components("c-table", Entry::Patched(ComponentPatch { name: Some("Dining table".into()), ..Default::default() }));
    let (snapshot, _) = edited(&mut session, &snapshot, rename);
    assert_eq!(session.report().computed, 0, "a name is read by no inference: {:?}", session.report());
    let rename = ModelDiff::mep_elements("m-supply", Entry::Patched(MepElementPatch { name: Some("Supply".into()), ..Default::default() }));
    edited(&mut session, &snapshot, rename);
    assert_eq!(session.report().computed, 0, "{:?}", session.report());
}

#[test]
fn moving_one_component_recomputes_its_node_solid_quantity_and_the_plan_and_diagnostics_of_its_storey_only() {
    let snapshot = room();
    let mut session = ModelInferenceSession::new();
    settle(&mut session, &snapshot);
    let edit = ModelDiff::components("c-table", Entry::Patched(ComponentPatch { elevation: Some(0.1), ..Default::default() }));
    let (_, after) = edited(&mut session, &snapshot, edit);
    assert_eq!(count(&session, "component"), 1, "{:?}", session.report());
    assert_eq!(count(&session, "solid"), 1);
    assert_eq!(count(&session, "quantity"), 1);
    assert_eq!(count(&session, "plan"), 1);
    assert_eq!(count(&session, "mep"), 0);
    assert_eq!(count(&session, "mep-clash"), 0);
    assert!((after.components["c-table"].placement.z - 0.1).abs() < 1e-12);
}

#[test]
fn editing_one_run_recomputes_that_run_its_solid_the_clash_scan_and_no_component() {
    let snapshot = room();
    let mut session = ModelInferenceSession::new();
    settle(&mut session, &snapshot);
    let path = vec![Point3 { x: 0.5, y: 2.0, z: 2.6 }, Point3 { x: 5.5, y: 2.0, z: 3.6 }];
    let edit = ModelDiff::mep_elements("m-supply", Entry::Patched(MepElementPatch { path: Some(path), ..Default::default() }));
    let (_, after) = edited(&mut session, &snapshot, edit);
    assert_eq!(count(&session, "mep"), 1, "{:?}", session.report());
    assert_eq!(count(&session, "mep-clash"), 1);
    assert_eq!(count(&session, "component"), 0);
    assert!(!after.diagnostics.iter().any(|found| found.code.slug() == "mep.clash" && found.elements == ["m-supply", "m-water"]), "the raised duct no longer meets the water pipe");
    assert!(after.diagnostics.iter().any(|found| found.code.slug() == "mep.terminal-unconnected" && found.elements == ["c-diffuser-ok"]), "the diffuser no longer sits at the end of the duct");
}

#[test]
fn a_moved_host_wall_recomputes_only_the_components_mounted_on_it_and_its_neighbours() {
    let snapshot = room();
    let mut session = ModelInferenceSession::new();
    settle(&mut session, &snapshot);
    let mut moved = snapshot.clone();
    let wall = moved.walls.get_mut("w-north").expect("wall");
    wall.axis = crate::Axis::Line { start: crate::Point2 { x: 6.0, y: 4.5 }, end: crate::Point2 { x: 0.0, y: 4.5 } };
    let after = session.refresh(&moved).clone();
    assert_eq!(after, ModelInference::infer(&moved).expect("infers"));
    let recomputed = count(&session, "component");
    assert!((1..=3).contains(&recomputed), "the basin on the north wall (and at most its neighbours): {:?}", session.report());
    let fit = after.components["c-basin-north"].placement.host.as_ref().expect("hosted");
    assert!((fit.face.y - 4.4).abs() < 1e-9, "the face moved with the wall");
}

#[test]
fn every_issue_code_of_the_package_is_raised_once_with_texts_in_both_languages() {
    let inference = ModelInference::infer(&room()).expect("infers");
    let slugs: BTreeSet<&str> = inference.diagnostics.iter().map(|found| found.code.slug()).collect();
    for want in ["component.outside-storey", "component.in-wall", "reference.component-family", "reference.component-host", "component.override", "mep.degenerate", "mep.clash", "mep.terminal-unconnected"] {
        assert!(slugs.contains(want), "{want} in {slugs:?}");
    }
    for found in inference.diagnostics.iter().filter(|found| found.code.category() == "component" || found.code.category() == "mep" || found.code.slug().starts_with("reference.component")) {
        let (en, de) = (found.text("en").expect("en"), found.text("de").expect("de"));
        assert!(!en.is_empty() && !de.is_empty() && en != de && !en.contains('{') && !de.contains('{'), "{en} | {de}");
    }
    let unconnected: Vec<&str> = inference.diagnostics.iter().filter(|found| found.code.slug() == "mep.terminal-unconnected").flat_map(|found| found.elements.iter().map(String::as_str)).collect();
    assert_eq!(unconnected, ["c-diffuser-lost"], "the lamp ends a tray and the second diffuser sits on a duct end");
    let clashes: Vec<Vec<&str>> = inference.diagnostics.iter().filter(|found| found.code.slug() == "mep.clash").map(|found| found.elements.iter().map(String::as_str).collect()).collect();
    assert_eq!(clashes.len(), 2);
    assert!(clashes.contains(&vec!["m-supply", "m-water"]) && clashes.contains(&vec!["m-power", "m-water"]));
    assert_eq!(inference.diagnostic_index.severity_of("c-high"), Some(Severity::Warning));
    assert_eq!(inference.diagnostic_index.severity_of("c-ghost"), Some(Severity::Error));
}

#[test]
fn the_plan_draws_outlines_ticks_connectors_bands_and_drops_cut_projected_or_hidden_by_height() {
    let inference = ModelInference::infer(&room()).expect("infers");
    let plan = &inference.plan_linework["st-ground"];
    assert_eq!(plan.count_of(PlanKind::ComponentOutline), 13, "every component with geometry");
    assert_eq!(plan.count_of(PlanKind::ComponentFront), 13);
    assert_eq!(plan.count_of(PlanKind::ComponentConnector), 6, "two strokes at each of the three terminals");
    assert_eq!(plan.count_of(PlanKind::MepAxis), 5, "the degenerate run has no symbol");
    assert_eq!(plan.count_of(PlanKind::MepBand), 5);
    assert_eq!(plan.count_of(PlanKind::MepDrop), 1, "the waste riser");
    let style = |element: &str, kind: PlanKind| plan.polylines.iter().find(|line| line.element == element && line.kind == kind).map(|line| line.style).expect("drawn");
    assert_eq!(style("c-table", PlanKind::ComponentOutline), PlanStyle::Projection, "below the cut at 1.2 m");
    assert_eq!(style("c-niche", PlanKind::ComponentOutline), PlanStyle::Cut);
    assert_eq!(style("c-lamp", PlanKind::ComponentOutline), PlanStyle::Hidden, "above the cut");
    assert!(plan.regions.iter().any(|region| region.element == "m-supply" && region.kind == PlanKind::MepBand && region.style == PlanStyle::Hidden));
    let band = plan.regions.iter().find(|region| region.element == "m-supply").expect("band");
    let area = {
        let n = band.outer.len();
        (0..n).map(|i| band.outer[i].x * band.outer[(i + 1) % n].y - band.outer[(i + 1) % n].x * band.outer[i].y).sum::<f64>() / 2.0
    };
    assert!((area - 0.3 * 5.0).abs() < 1e-9, "a counter-clockwise band of width x length: {area}");
    let tick = plan.polylines.iter().find(|line| line.element == "c-basin-south" && line.kind == PlanKind::ComponentFront).expect("tick");
    assert!((tick.vertices[0].y - 0.55).abs() < 1e-9 && (tick.vertices[1].y - 0.65).abs() < 1e-9, "the tick leaves the front edge towards the room: {:?}", tick.vertices);
}

#[test]
fn the_quantities_sum_per_family_category_system_and_size() {
    let inference = ModelInference::infer(&room()).expect("infers");
    let table = &inference.quantities.elements["c-table"];
    assert_eq!(table.kind, QuantityKind::Component);
    assert_eq!(table.type_id, "fam-table");
    assert!((table.net_volume - 1.6 * 0.8 * 0.74).abs() < 1e-9 && (table.gross_area - 1.28).abs() < 1e-9);
    assert!(table.layers.iter().any(|row| row.material == "m-oak" && row.mass > 0.0), "{:?}", table.layers);
    let duct = &inference.quantities.elements["m-supply"];
    assert_eq!((duct.kind, duct.type_id.as_str()), (QuantityKind::Mep, "supply"));
    assert!((duct.length - 5.0).abs() < 1e-12 && (duct.gross_area - 0.06).abs() < 1e-12 && (duct.net_volume - 0.3).abs() < 1e-9 && (duct.surface_area - 5.0).abs() < 1e-12);
    let project = &inference.quantities.project;
    assert_eq!(project.kinds["component"].count, 16);
    assert_eq!(project.kinds["mep"].count, 6);
    assert_eq!(project.types["component:fam-table"].count, 5, "five table instances");
    assert_eq!(project.groups["component-category:furniture"].count, 7);
    assert_eq!(project.groups["component-system:supply"].count, 2);
    assert_eq!(project.groups["mep-system:domestic-water"].count, 1);
    assert_eq!(project.groups["mep-size:300x200"].count, 1);
    assert!((project.groups["mep-system:power"].length - 5.0).abs() < 1e-12);
    assert_eq!(inference.quantities.storeys["st-ground"].groups, project.groups, "all elements stand on the ground storey");
}

#[test]
fn the_text_table_is_the_table_the_oracle_wrote() {
    let inference = ModelInference::infer(&room()).expect("infers");
    let actual = crate::standards::v1::subsets::any::io::text::inferences::components::table_json(&inference);
    let problems = crate::standards::v1::subsets::any::schema::inferences::storey_levels::table_problems(TABLE, &actual);
    assert!(problems.is_empty(), "{problems:#?}");
    let projected = crate::standards::v1::subsets::any::io::text::snapshot::encode_inference_projection_json(&room(), "components").expect("the components projection exists");
    assert_eq!(projected, actual);
}
