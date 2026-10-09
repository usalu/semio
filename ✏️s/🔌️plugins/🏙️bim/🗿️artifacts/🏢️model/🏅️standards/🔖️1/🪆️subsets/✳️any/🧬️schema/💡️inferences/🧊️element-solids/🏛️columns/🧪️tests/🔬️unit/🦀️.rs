use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{plan::solid_steps, ModelNode};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidKey;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::testing::{assert_matches_oracle, case, close, raised};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, SolidFamily};
use crate::ModelInference;
use protocol::Inference;

const CASE: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/🏛️columns-profiles/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn columns_reproduce_the_third_party_oracle_table() {
    let case = case(CASE);
    assert_matches_oracle(&case, &compute_element_solids(&case.snapshot), SolidFamily::Column);
}

#[semio_framework_async_macros::async_test]
async fn columns_have_the_analytic_volumes_of_their_profiles() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    assert!(close(0.4 * 0.3 * 3.0, solids["c-rect"].volume, 1e-12), "rectangle: width x depth x storey height");
    assert!(close((2.0 * 0.2 * 0.015 + 0.01 * (0.3 - 0.03)) * 2.0, solids["c-i"].volume, 1e-12), "I shape: two flanges and a web over the free height");
    assert!(close(0.07 * 3.5, solids["c-custom"].volume, 1e-12), "custom L outline: 0.07 m2 over the storey top plus 0.5");
    let round = std::f64::consts::PI * 0.25f64.powi(2) * 2.7;
    assert!((solids["c-circle"].volume - round).abs() <= round * 1e-3 && solids["c-circle"].volume < round, "circle: inscribed within the sagitta bound");
}

#[semio_framework_async_macros::async_test]
async fn profiles_become_centred_loops_and_invalid_ones_become_empty() {
    assert_eq!(profile_loop(&Profile::Rectangle { width: 2.0, depth: 1.0 }).len(), 4);
    assert_eq!(profile_loop(&Profile::IShape { width: 0.2, depth: 0.3, web: 0.01, flange: 0.015 }).len(), 12);
    let circle = profile_loop(&Profile::Circle { diameter: 1.0 });
    assert!(circle.len() == 2 && circle.iter().all(|v| v.bulge == 1.0), "two semicircles");
    for invalid in [Profile::Rectangle { width: 0.0, depth: 1.0 }, Profile::Circle { diameter: -1.0 }, Profile::IShape { width: 0.2, depth: 0.3, web: 0.25, flange: 0.015 }, Profile::IShape { width: 0.2, depth: 0.03, web: 0.01, flange: 0.015 }] {
        assert!(profile_loop(&invalid).is_empty(), "{invalid:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn changing_a_storey_height_re_infers_exactly_the_columns_that_follow_it() {
    let snapshot = case(CASE).snapshot;
    let (before, after) = (compute_element_solids(&snapshot), compute_element_solids(&raised(&snapshot, "st-ground", 0.4)));
    let height = |solids: &std::collections::BTreeMap<String, ElementSolid>, id: &str| solids[id].bounds.max.z - solids[id].bounds.min.z;
    assert!(close(height(&after, "c-rect") - height(&before, "c-rect"), 0.4, 1e-9), "a StoreyTop column grows with its storey");
    assert!(close(height(&after, "c-custom") - height(&before, "c-custom"), 0.4, 1e-9), "an offset from the storey top grows with the storey too");
    assert!(close(height(&after, "c-circle") - height(&before, "c-circle"), 0.4, 1e-9), "a column constrained to the storey above follows that storey's elevation");
    assert!(close(height(&after, "c-i"), height(&before, "c-i"), 1e-9), "an Unconnected column keeps its height");
    assert!(close(after["c-i"].bounds.min.z - before["c-i"].bounds.min.z, 0.4, 1e-9), "a column on the storey above is lifted, not stretched");
    assert!(close(after["c-i"].volume, before["c-i"].volume, 1e-12));
}

#[semio_framework_async_macros::async_test]
async fn a_column_depends_on_its_storey_and_the_storey_its_top_targets() {
    let snapshot = case(CASE).snapshot;
    let plan = solid_steps(&snapshot, SolidFamily::Column);
    let parents = |id: &str| plan.iter().find(|step| step.key == ModelNode::Solid(SolidKey::of(SolidFamily::Column, id))).map(|step| step.parents.clone()).expect("planned");
    assert_eq!(parents("c-rect"), vec![ModelNode::Storey("st-ground".into())]);
    assert_eq!(parents("c-circle"), vec![ModelNode::Storey("st-ground".into()), ModelNode::Storey("st-first".into())]);
    assert_eq!(parents("c-i"), vec![ModelNode::Storey("st-first".into())]);
}

#[semio_framework_async_macros::async_test]
async fn column_solids_carry_the_type_material_in_one_body_group() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    for (id, material) in [("c-rect", "m-concrete"), ("c-i", "m-steel"), ("c-custom", "m-steel")] {
        let groups = &solids[id].groups;
        assert_eq!(groups.len(), 1, "{id}");
        assert_eq!((groups[0].part.as_str(), groups[0].material.as_str(), groups[0].layer), (parts::BODY, material, 0), "{id}");
    }
}

#[semio_framework_async_macros::async_test]
async fn columns_are_deterministic_and_the_empty_model_has_none() {
    let snapshot = case(CASE).snapshot;
    let (first, second) = (ModelInference::infer(&snapshot).expect("infers"), ModelInference::infer(&snapshot).expect("infers"));
    assert_eq!(first, second);
    assert!(["c-rect", "c-circle", "c-i", "c-custom"].iter().all(|id| first.element_solids.contains_key(*id)));
    assert!(["c-bad-type", "c-flat", "c-unknown-type"].iter().all(|id| !first.element_solids.contains_key(*id)), "columns without geometry are absent");
    assert!(solid_steps(&ModelSnapshot::default(), SolidFamily::Column).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_leaning_column_is_the_sheared_prism_of_its_stretched_section() {
    let mut snapshot = case(CASE).snapshot;
    snapshot.columns.get_mut("c-rect").expect("column").rotation = 0.0;
    let plumb = compute_element_solids(&snapshot)["c-rect"].clone();
    let angle = 0.3_f64;
    snapshot.columns.get_mut("c-rect").expect("column").tilt = Some(crate::Slope { direction: 0.0, angle });
    let leaning = compute_element_solids(&snapshot)["c-rect"].clone();
    let rise = plumb.bounds.max.z - plumb.bounds.min.z;
    assert!(close(leaning.volume, plumb.volume / angle.cos(), 1e-9), "area times rise over cos(angle)");
    assert!(close(leaning.bounds.max.z - leaning.bounds.min.z, rise, 1e-9), "both ends are cut by horizontal planes");
    let width = plumb.bounds.max.x - plumb.bounds.min.x;
    assert!(close(leaning.bounds.max.x - leaning.bounds.min.x, width / angle.cos() + rise * angle.tan(), 1e-9), "the base section is stretched and the top is moved along the lean");
    assert!(close(leaning.bounds.max.y - leaning.bounds.min.y, plumb.bounds.max.y - plumb.bounds.min.y, 1e-9), "nothing changes across the lean");
    assert!(close(leaning.bounds.min.x, plumb.bounds.min.x - (width / angle.cos() - width) / 2.0, 1e-9), "the base section stays centred on the position");
}

#[semio_framework_async_macros::async_test]
async fn the_lean_direction_turns_the_shift_in_plan() {
    let mut snapshot = case(CASE).snapshot;
    snapshot.columns.get_mut("c-rect").expect("column").rotation = 0.0;
    let plumb = compute_element_solids(&snapshot)["c-rect"].clone();
    snapshot.columns.get_mut("c-rect").expect("column").tilt = Some(crate::Slope { direction: std::f64::consts::FRAC_PI_2, angle: 0.2 });
    let leaning = compute_element_solids(&snapshot)["c-rect"].clone();
    let rise = plumb.bounds.max.z - plumb.bounds.min.z;
    assert!(close(leaning.bounds.max.x - leaning.bounds.min.x, plumb.bounds.max.x - plumb.bounds.min.x, 1e-9), "no lean across x");
    assert!(close(leaning.bounds.max.y - plumb.bounds.max.y, (0.3 / 0.2_f64.cos() - 0.3) / 2.0 + rise * 0.2_f64.tan(), 1e-9), "the top moves towards +y");
}

#[semio_framework_async_macros::async_test]
async fn the_footprint_at_a_height_follows_the_lean() {
    let mut snapshot = case(CASE).snapshot;
    let column = snapshot.columns.get_mut("c-rect").expect("column");
    column.rotation = 0.0;
    column.tilt = Some(crate::Slope { direction: 0.0, angle: 0.25 });
    let (column, profile) = (snapshot.columns["c-rect"].clone(), snapshot.column_types["ct-rect"].profile.clone());
    let centre = |ring: &[Point]| (ring.iter().map(|p| p.x).sum::<f64>() / ring.len() as f64, ring.iter().map(|p| p.y).sum::<f64>() / ring.len() as f64);
    let (base, high) = (centre(&footprint(&column, &profile, 0.0, 0.0)), centre(&footprint(&column, &profile, 0.0, 2.0)));
    assert!(close(base.0, column.position.x, 1e-12) && close(base.1, column.position.y, 1e-12));
    assert!(close(high.0 - base.0, 2.0 * 0.25_f64.tan(), 1e-12) && close(high.1, base.1, 1e-12));
}

#[test]
fn the_reach_of_a_column_grows_with_its_lean() {
    let snapshot = case(CASE).snapshot;
    let mut column = snapshot.columns["c-rect"].clone();
    let plumb = reach(&snapshot, &column).expect("a type");
    column.tilt = Some(crate::Slope { direction: 0.0, angle: 0.3 });
    assert!(reach(&snapshot, &column).expect("a type") > plumb + 1.0, "a lean may carry the section away from the base point");
    column.column_type = "ct-none".into();
    assert_eq!(reach(&snapshot, &column), None);
}
