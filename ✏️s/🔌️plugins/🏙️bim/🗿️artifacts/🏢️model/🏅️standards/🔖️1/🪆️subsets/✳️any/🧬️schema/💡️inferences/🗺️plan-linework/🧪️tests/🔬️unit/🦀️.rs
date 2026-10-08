use super::*;
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, plan, ModelInferenceSession, ModelNode};
use crate::{Axis, ModelInference, OpeningKind, Phase, Point2, TopConstraint};
use protocol::Inference;
use std::collections::BTreeSet;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🗺️plan-linework/🏡️house/📸️snapshot/🔣️.json");
const CURVED: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🗺️plan-linework/🌀️curved/📸️snapshot/🔣️.json");

fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("house decodes")
}

fn curved() -> ModelSnapshot {
    from_json_str(CURVED, JsonMemberPolicy::Reject).expect("curved decodes")
}

fn plan(snapshot: &ModelSnapshot, storey: &str) -> PlanLinework {
    compute_plan_linework(snapshot).remove(storey).expect("the storey has a plan")
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

fn of<'a>(plan: &'a PlanLinework, element: &str, kind: PlanKind) -> Vec<&'a PlanPolyline> {
    plan.polylines.iter().filter(|line| line.element == element && line.kind == kind).collect()
}

fn regions<'a>(plan: &'a PlanLinework, element: &str, kind: PlanKind) -> Vec<&'a PlanRegion> {
    plan.regions.iter().filter(|region| region.element == element && region.kind == kind).collect()
}

#[semio_framework_async_macros::async_test]
async fn the_plan_is_cut_at_the_convention_height_above_each_storey() {
    let plans = compute_plan_linework(&house());
    assert_eq!(plans.keys().collect::<Vec<_>>(), vec!["st-first", "st-ground"]);
    assert!(close(plans["st-ground"].cut_height, 1.2) && close(plans["st-ground"].cut_elevation, 1.2));
    assert!(close(plans["st-first"].cut_height, 1.2) && close(plans["st-first"].cut_elevation, 3.0 + 1.2));
}

#[semio_framework_async_macros::async_test]
async fn cut_walls_are_poche_with_gaps_where_an_opening_cuts_the_plane() {
    let ground = plan(&house(), "st-ground");
    assert_eq!(regions(&ground, "w-south", PlanKind::WallCut).len(), 3, "the door and the south window split the wall into three pieces");
    assert_eq!(regions(&ground, "w-north", PlanKind::WallCut).len(), 1, "the high window does not reach the cut plane");
    let expected = (8.3 * 6.3 - 7.7 * 5.7) - (1.0 + 1.2) * 0.3 + 2.85 * 0.15;
    assert!(close(ground.area_of(PlanKind::WallCut), expected), "mitered ring minus the gaps plus the trimmed partition: {} vs {}", ground.area_of(PlanKind::WallCut), expected);
    assert!(ground.regions.iter().filter(|region| region.kind == PlanKind::WallCut).all(|region| region.style == PlanStyle::Cut));
}

#[semio_framework_async_macros::async_test]
async fn openings_draw_their_symbols_in_the_style_of_their_cut() {
    let ground = plan(&house(), "st-ground");
    let leaf = of(&ground, "o-door", PlanKind::DoorLeaf);
    let swing = of(&ground, "o-door", PlanKind::DoorSwing);
    assert!(leaf.len() == 1 && swing.len() == 1 && leaf[0].style == PlanStyle::Cut && swing[0].style == PlanStyle::Projection);
    assert!(close(swing[0].vertices[0].bulge, (std::f64::consts::FRAC_PI_2 / 4.0).tan().copysign(swing[0].vertices[0].bulge)), "a quarter circle has bulge tan(pi / 8)");
    assert_eq!(of(&ground, "o-win-south", PlanKind::WindowFrame)[0].style, PlanStyle::Cut);
    assert_eq!(of(&ground, "o-win-south", PlanKind::WindowSill).len(), 2);
    assert_eq!(of(&ground, "o-win-south", PlanKind::WindowGlazing).len(), 1);
    let high = of(&ground, "o-win-north", PlanKind::WindowFrame);
    assert_eq!(high.len(), 1);
    assert_eq!(high[0].style, PlanStyle::Hidden, "a window above the cut plane is dashed");
    assert!(of(&ground, "o-win-north", PlanKind::WindowSill).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn layer_lines_follow_the_interfaces_and_stop_at_the_gaps() {
    let ground = plan(&house(), "st-ground");
    assert_eq!(of(&ground, "w-south", PlanKind::WallLayer).len(), 3, "one interface (brick / insulation) in three runs");
    assert_eq!(of(&ground, "w-east", PlanKind::WallLayer).len(), 1);
    assert!(of(&ground, "w-part", PlanKind::WallLayer).is_empty(), "a single layer has no interface");
    let line = &of(&ground, "w-east", PlanKind::WallLayer)[0];
    assert!(line.vertices.iter().all(|v| close(v.x, 8.05)), "the interface sits 0.2 from the left face, i.e. 0.05 to the right of the axis: {:?}", line.vertices);
}

#[semio_framework_async_macros::async_test]
async fn members_columns_beams_slabs_roofs_and_railings_follow_the_cut() {
    let (ground, first) = (plan(&house(), "st-ground"), plan(&house(), "st-first"));
    let column = regions(&ground, "c-1", PlanKind::ColumnCut);
    assert!(column.len() == 1 && close(ground.area_of(PlanKind::ColumnCut), 0.16));
    let beam = of(&ground, "b-1", PlanKind::BeamOutline);
    assert!(beam.len() == 1 && beam[0].closed && beam[0].style == PlanStyle::Hidden, "a beam below the storey top hangs above the cut");
    assert!(close(ground.length_of(PlanKind::BeamOutline), 2.0 * 8.0 + 2.0 * 0.3));
    let slab = of(&ground, "sl-ground", PlanKind::SlabEdge);
    assert!(slab.len() == 1 && slab[0].style == PlanStyle::Projection && close(ground.length_of(PlanKind::SlabEdge), 28.0));
    let hole = of(&first, "sl-first", PlanKind::SlabHole);
    assert!(hole.len() == 1 && hole[0].style == PlanStyle::Projection);
    let rail = of(&ground, "r-1", PlanKind::RailingPath);
    assert!(rail.len() == 1 && !rail[0].closed && rail[0].style == PlanStyle::Projection, "a 1.0 m railing is below the 1.2 m cut");
    let roof = of(&first, "rf-1", PlanKind::RoofOutline);
    assert!(roof.len() == 1 && roof[0].style == PlanStyle::Hidden && close(first.length_of(PlanKind::RoofOutline), 2.0 * (8.6 + 6.6)), "the overhang grows the outline by 0.3 on every side");
}

#[semio_framework_async_macros::async_test]
async fn curtain_walls_draw_their_axis_and_one_mullion_per_grid_line() {
    let first = plan(&house(), "st-first");
    assert_eq!(of(&first, "cw-first-north", PlanKind::CurtainAxis).len(), 1);
    assert_eq!(regions(&first, "cw-first-north", PlanKind::CurtainMullion).len(), 5, "four panels of 2 m need five mullions");
    assert!(close(first.area_of(PlanKind::CurtainMullion), 5.0 * 0.06 * 0.1));
}

#[semio_framework_async_macros::async_test]
async fn stairs_show_risers_the_cut_line_and_the_up_arrow() {
    let ground = plan(&house(), "st-ground");
    let styles = |style: PlanStyle| of(&ground, "s-1", PlanKind::StairRiser).into_iter().filter(|line| line.style == style).count();
    assert_eq!((styles(PlanStyle::Projection), styles(PlanStyle::Hidden)), (6, 10), "risers below the cut are solid, above it dashed");
    assert_eq!(of(&ground, "s-1", PlanKind::StairCutLine).len(), 1);
    assert_eq!(of(&ground, "s-1", PlanKind::StairCutLine)[0].style, PlanStyle::Cut);
    assert_eq!(of(&ground, "s-1", PlanKind::StairArrow).len(), 2, "the shaft and the head");
    assert_eq!(of(&ground, "s-1", PlanKind::StairOutline).len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn spaces_have_an_outline_and_a_tag_with_number_name_and_area() {
    let ground = plan(&house(), "st-ground");
    let tag = ground.texts.iter().find(|text| text.element == "sp-living" && text.kind == PlanKind::SpaceTag).expect("tag");
    assert_eq!((tag.label.as_str(), tag.detail.as_str()), ("G.01", "Living"));
    assert!(close(tag.measure.expect("area"), 7.7 * 5.7) && close(tag.x, 4.0) && close(tag.y, 3.0));
    assert_eq!(of(&ground, "sp-living", PlanKind::SpaceOutline).len(), 1);
    let hall = ground.texts.iter().find(|text| text.element == "sp-hall" && text.kind == PlanKind::SpaceTag).expect("tag");
    assert!(close(hall.measure.expect("area"), 7.7 * 5.7 - 2.85 * 0.15), "a bounded space is the room around its seed: the interior minus the notch of the partition");
    assert!(close(ground.length_of(PlanKind::SpaceOutline), 26.8 + 32.5), "the explicit rectangle and the bounded room with the partition notch");
}

#[semio_framework_async_macros::async_test]
async fn an_open_room_is_tagged_at_its_seed_without_an_outline() {
    let mut open = house();
    open.walls.remove("w-west");
    let ground = plan(&open, "st-ground");
    let tag = ground.texts.iter().find(|text| text.element == "sp-hall" && text.kind == PlanKind::SpaceTag).expect("tag");
    assert!(tag.measure.is_none() && close(tag.x, 2.0) && close(tag.y, 2.0));
    assert!(of(&ground, "sp-hall", PlanKind::SpaceOutline).is_empty());
    assert_eq!(of(&ground, "sp-living", PlanKind::SpaceOutline).len(), 1, "an explicit outline is drawn as authored");
}

#[semio_framework_async_macros::async_test]
async fn grid_lines_have_bubbles_and_labels_on_every_storey_of_the_building() {
    for storey in ["st-ground", "st-first"] {
        let sheet = plan(&house(), storey);
        assert_eq!((sheet.count_of(PlanKind::GridLine), sheet.count_of(PlanKind::GridBubble), sheet.count_of(PlanKind::GridLabel)), (2, 4, 4));
        assert!(close(sheet.length_of(PlanKind::GridLine), 8.0 + 10.0));
    }
    let ground = plan(&house(), "st-ground");
    let b = ground.bounds;
    assert!(close(b.min_x, -1.6) && close(b.min_y, -1.6) && close(b.max_x, 9.6) && close(b.max_y, 7.6), "grid bubbles bound the plan: {b:?}");
}

#[semio_framework_async_macros::async_test]
async fn every_primitive_has_a_unique_id_and_the_id_of_the_element_it_depicts() {
    let sheet = plan(&house(), "st-ground");
    let ids: Vec<&String> = sheet.regions.iter().map(|r| &r.id).chain(sheet.polylines.iter().map(|l| &l.id)).chain(sheet.texts.iter().map(|t| &t.id)).collect();
    let unique: BTreeSet<&&String> = ids.iter().collect();
    assert_eq!(unique.len(), ids.len());
    let house = house();
    let known = |element: &str| house.walls.contains_key(element) || house.openings.contains_key(element) || house.columns.contains_key(element) || house.beams.contains_key(element) || house.slabs.contains_key(element) || house.stairs.contains_key(element) || house.railings.contains_key(element) || house.spaces.contains_key(element) || house.grids.contains_key(element);
    assert!(sheet.regions.iter().map(|r| &r.element).chain(sheet.polylines.iter().map(|l| &l.element)).chain(sheet.texts.iter().map(|t| &t.element)).all(|element| known(element)));
}

#[semio_framework_async_macros::async_test]
async fn curved_walls_keep_their_arcs_and_the_exact_area() {
    let snapshot = curved();
    let ground = plan(&snapshot, "st-ground");
    let (radius, sweep) = (5.0, 4.0 * 0.5_f64.atan());
    let expected = 0.3 * radius * sweep - 0.3 * radius * (1.2 / radius);
    assert!(close(ground.area_of(PlanKind::WallCut), expected), "annular sector minus the window gap: {} vs {}", ground.area_of(PlanKind::WallCut), expected);
    assert!(regions(&ground, "w-arc", PlanKind::WallCut).iter().all(|region| region.outer.iter().any(|v| v.bulge != 0.0)), "the faces stay arcs");
    assert!(close(ground.area_of(PlanKind::ColumnCut), std::f64::consts::PI * 0.25 * 0.25), "a round column is two exact half circles");
}

#[semio_framework_async_macros::async_test]
async fn a_wall_that_ends_below_the_cut_is_drawn_in_projection_and_a_demolished_wall_dashed() {
    let mut snapshot = house();
    snapshot.walls.get_mut("w-east").expect("wall").top = TopConstraint::Unconnected { height: 1.0 };
    snapshot.walls.get_mut("w-west").expect("wall").phase = Phase::Demolished;
    let ground = plan(&snapshot, "st-ground");
    assert!(regions(&ground, "w-east", PlanKind::WallCut).is_empty());
    assert_eq!(of(&ground, "w-east", PlanKind::WallOutline)[0].style, PlanStyle::Projection);
    assert!(regions(&ground, "w-west", PlanKind::WallCut).is_empty());
    assert_eq!(of(&ground, "w-west", PlanKind::WallOutline)[0].style, PlanStyle::Hidden);
}

#[semio_framework_async_macros::async_test]
async fn raising_a_sill_above_the_cut_closes_the_gap_in_the_poche() {
    let mut snapshot = house();
    snapshot.openings.get_mut("o-win-south").expect("opening").sill_override = Some(1.5);
    let ground = plan(&snapshot, "st-ground");
    assert_eq!(regions(&ground, "w-south", PlanKind::WallCut).len(), 2, "only the door still cuts the plane");
    assert_eq!(of(&ground, "o-win-south", PlanKind::WindowFrame)[0].style, PlanStyle::Hidden);
    assert!(matches!(snapshot.openings["o-win-south"].kind, OpeningKind::Window { .. }));
}

#[semio_framework_async_macros::async_test]
async fn moving_a_wall_moves_its_poche_and_nothing_else() {
    let before = plan(&house(), "st-ground");
    let mut snapshot = house();
    snapshot.walls.get_mut("w-part").expect("wall").axis = Axis::Line { start: Point2 { x: 5.0, y: 0.0 }, end: Point2 { x: 5.0, y: 3.0 } };
    let after = plan(&snapshot, "st-ground");
    let x_of = |sheet: &PlanLinework| sheet.regions.iter().filter(|region| region.element == "w-part").flat_map(|region| region.outer.iter().map(|v| v.x)).fold(f64::NEG_INFINITY, f64::max);
    assert!(close(x_of(&after) - x_of(&before), 1.0));
    let others = |sheet: &PlanLinework| sheet.regions.iter().filter(|region| region.element != "w-part" && region.element != "w-south").cloned().collect::<Vec<_>>();
    assert_eq!(others(&before), others(&after));
    assert_eq!(plan(&house(), "st-first"), plan(&snapshot, "st-first"), "the other storey is untouched");
}

#[semio_framework_async_macros::async_test]
async fn changing_a_storey_height_moves_the_cut_of_the_storeys_above_only() {
    let mut taller = house();
    taller.storeys.get_mut("st-ground").expect("storey").height = 3.4;
    let (before, after) = (compute_plan_linework(&house()), compute_plan_linework(&taller));
    assert!(close(after["st-first"].cut_elevation - before["st-first"].cut_elevation, 0.4));
    let apart_from_stairs = |plan: &PlanLinework| plan.polylines.iter().filter(|line| !matches!(line.kind, PlanKind::StairOutline | PlanKind::StairRiser | PlanKind::StairCutLine | PlanKind::StairArrow | PlanKind::StairLanding)).cloned().collect::<Vec<_>>();
    assert_eq!(apart_from_stairs(&after["st-ground"]), apart_from_stairs(&before["st-ground"]), "only the stair, which has to climb more, changes");
    assert_eq!(after["st-ground"].regions, before["st-ground"].regions);
    assert!(of(&after["st-ground"], "s-1", PlanKind::StairRiser).len() > of(&before["st-ground"], "s-1", PlanKind::StairRiser).len());
    assert!(close(after["st-ground"].cut_elevation, before["st-ground"].cut_elevation));
}

#[semio_framework_async_macros::async_test]
async fn plan_determinism_and_default_laws() {
    let snapshot = house();
    assert_eq!(ModelInference::infer(&snapshot).expect("infers").plan_linework, ModelInference::infer(&snapshot).expect("infers").plan_linework);
    assert!(ModelInference::infer(&ModelSnapshot::default()).expect("infers").plan_linework.is_empty());
}


#[semio_framework_async_macros::async_test]
async fn a_plan_depends_on_its_own_storey_the_storeys_its_tops_target_and_the_values_it_draws() {
    let mut snapshot = house();
    snapshot.walls.get_mut("w-first-west").expect("wall").top = TopConstraint::Storey { storey: "st-ground".into(), offset: 0.5 };
    let steps = plan::build(&snapshot, kinds::closure(kinds::PLANS));
    let parents = |storey: &str| steps.iter().find(|step| step.key == ModelNode::Plan(storey.into())).map(|step| step.parents.clone()).expect("planned");
    assert_eq!(parents("st-ground")[0], ModelNode::Storey("st-ground".into()));
    assert_eq!(parents("st-first")[..2], [ModelNode::Storey("st-first".into()), ModelNode::Storey("st-ground".into())]);
    assert!(parents("st-first").contains(&ModelNode::WallLayout("w-first-west".into())), "the plan draws the layout, it does not derive it");
    assert!(parents("st-first").iter().any(|parent| matches!(parent, ModelNode::Room(_))) || snapshot.spaces.values().all(|space| space.storey != "st-first"));
    let position = |key: &ModelNode| steps.iter().position(|step| &step.key == key).expect("planned");
    steps.iter().for_each(|step| step.parents.iter().for_each(|parent| assert!(position(parent) < position(&step.key), "parents come first")));
    assert_eq!(compute_storey_levels(&snapshot).len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn a_height_edit_recomputes_the_plans_of_the_storeys_it_reaches_and_a_density_edit_none() {
    use crate::{Entry, MaterialPatch, ModelDiff, StoreyPatch};
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).plan_linework.clone();
    let paint = ModelDiff::materials("m-brick", Entry::Patched(MaterialPatch { density: Some(1900.0), ..Default::default() }));
    let untouched = session.update(&snapshot, &paint).plan_linework.clone();
    assert_eq!(first, untouched);
    assert_eq!(session.report().computed_by_kind.get("plan"), None, "a density edit recomputes no plan");
    let height = ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(3.4), ..Default::default() }));
    let edited = protocol::apply_diff(&height, &snapshot).expect("applies");
    let recomputed = session.update(&edited, &height).plan_linework.clone();
    assert!(close(recomputed["st-first"].cut_elevation - first["st-first"].cut_elevation, 0.4));
}
