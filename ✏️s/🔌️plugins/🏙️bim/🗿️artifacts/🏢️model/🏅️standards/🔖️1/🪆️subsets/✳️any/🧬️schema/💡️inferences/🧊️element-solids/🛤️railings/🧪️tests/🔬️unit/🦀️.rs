use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{plan::solid_steps, ModelNode};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidKey;
use crate::ModelSnapshot;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::testing::{assert_matches_oracle, case, close, group_extent, raised};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, parts, SolidFamily};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels;
use crate::ModelInference;
use protocol::Inference;

const CASE: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/🛤️railings-posts/🔣️.json");

fn geometry(snapshot: &ModelSnapshot, id: &str) -> RailingGeometry {
    let railing = &snapshot.railings[id];
    railing_geometry(railing, &compute_storey_levels(snapshot)[&railing.storey])
}

#[semio_framework_async_macros::async_test]
async fn railings_reproduce_the_third_party_oracle_table() {
    let case = case(CASE);
    assert_matches_oracle(&case, &compute_element_solids(&case.snapshot), SolidFamily::Railing);
}

#[semio_framework_async_macros::async_test]
async fn posts_divide_every_segment_into_equal_parts_no_longer_than_the_spacing() {
    let snapshot = case(CASE).snapshot;
    let counts = |id: &str| geometry(&snapshot, id).posts.len();
    assert_eq!(counts("r-straight"), 5, "4 m at 1 m: five posts");
    assert_eq!(counts("r-l"), 7, "two 2 m segments at 0.8 m: three parts each, one shared corner post");
    assert_eq!(counts("r-u"), 6, "3 m, 1 m and 3 m at 1.5 m: 2 + 1 + 2 parts, shared corners");
    assert_eq!(counts("r-no-spacing"), 2, "no spacing leaves the vertex posts");
    let posts = geometry(&snapshot, "r-straight").posts;
    for (index, post) in posts.iter().enumerate() {
        assert!(close(index as f64, post.x, 1e-12) && close(0.0, post.y, 1e-12));
    }
    assert!(geometry(&snapshot, "r-l").posts.iter().any(|post| close(2.0, post.x, 1e-12) && close(0.0, post.y, 1e-12)), "a post stands on the corner");
}

#[semio_framework_async_macros::async_test]
async fn a_railing_has_the_analytic_volume_of_its_posts_and_its_mitred_rail() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    let expected = |posts: f64, height: f64, length: f64| posts * POST_SIZE * POST_SIZE * (height - RAIL_DEPTH) + RAIL_WIDTH * RAIL_DEPTH * length;
    assert!(close(expected(5.0, 1.0, 4.0), solids["r-straight"].volume, 1e-12));
    assert!(close(expected(7.0, 0.9, 4.0), solids["r-l"].volume, 1e-12), "a mitred corner keeps the centre-line length");
    assert!(close(expected(6.0, 1.1, 7.0), solids["r-u"].volume, 1e-12));
    let low = &solids["r-low"];
    assert!(close(RAIL_WIDTH * RAIL_DEPTH * 1.0, low.volume, 1e-12), "a railing lower than its rail is a rail alone");
    assert!(low.groups.iter().all(|group| group.part == parts::RAIL));
}

#[semio_framework_async_macros::async_test]
async fn posts_stand_on_the_base_and_the_rail_tops_the_railing() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    let railing = &solids["r-l"];
    assert!(close(0.1, railing.bounds.min.z, 1e-12) && close(1.0, railing.bounds.max.z, 1e-12), "base offset 0.1, height 0.9");
    assert_eq!(railing.groups.iter().map(|g| (g.part.as_str(), g.material.as_str())).collect::<Vec<_>>(), vec![(parts::POST, "m-steel"), (parts::RAIL, "m-steel")]);
    let (_, post_top, _) = group_extent(railing, 0);
    let (rail_bottom, rail_top, _) = group_extent(railing, 1);
    assert!(close(1.0 - RAIL_DEPTH, post_top, 1e-12) && close(1.0 - RAIL_DEPTH, rail_bottom, 1e-12) && close(1.0, rail_top, 1e-12), "posts end where the rail begins");
}

#[semio_framework_async_macros::async_test]
async fn changing_a_storey_height_leaves_ground_railings_and_lifts_the_ones_above() {
    let snapshot = case(CASE).snapshot;
    let before = compute_element_solids(&snapshot);
    assert_eq!(compute_element_solids(&raised(&snapshot, "st-ground", 0.4)), before, "a railing on the ground storey stands on the unchanged ground elevation");
    let mut upstairs = snapshot.clone();
    for railing in upstairs.railings.values_mut() {
        railing.storey = "st-first".into();
    }
    let (low, high) = (compute_element_solids(&upstairs), compute_element_solids(&raised(&upstairs, "st-ground", 0.4)));
    assert!(close(high["r-straight"].bounds.min.z - low["r-straight"].bounds.min.z, 0.4, 1e-9) && close(high["r-straight"].volume, low["r-straight"].volume, 1e-12));
}

#[semio_framework_async_macros::async_test]
async fn degenerate_railings_are_absent_and_the_result_is_deterministic() {
    let snapshot = case(CASE).snapshot;
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred, ModelInference::infer(&snapshot).expect("infers"));
    assert!(!inferred.element_solids.contains_key("r-point"));
    let plan = solid_steps(&snapshot, SolidFamily::Railing);
    assert!(plan.iter().all(|step| step.parents == vec![ModelNode::Storey("st-ground".into())]));
    assert!(solid_steps(&ModelSnapshot::default(), SolidFamily::Railing).is_empty());
}
