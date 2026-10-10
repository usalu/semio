use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{plan::solid_steps, ModelNode};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidKey;
use crate::{ModelSnapshot, Profile};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::extents_of;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::testing::{assert_matches_oracle, case, close, group_extent, raised};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, parts, SolidFamily};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels;
use crate::ModelInference;
use protocol::Inference;

const CASE: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/🛤️railings-posts/🔣️.json");

fn sections(snapshot: &ModelSnapshot, id: &str) -> ((f64, f64), (f64, f64)) {
    let railing = &snapshot.railings[id];
    (extents_of(&railing.post_profile), extents_of(&railing.profile))
}

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
    let snapshot = case(CASE).snapshot;
    let solids = compute_element_solids(&snapshot);
    let expected = |id: &str, posts: f64, height: f64, length: f64| {
        let ((post_width, post_depth), (rail_width, rail_depth)) = sections(&snapshot, id);
        posts * post_width * post_depth * (height - rail_depth) + rail_width * rail_depth * length
    };
    assert!(close(expected("r-straight", 5.0, 1.0, 4.0), solids["r-straight"].volume, 1e-12));
    assert!(close(expected("r-l", 7.0, 0.9, 4.0), solids["r-l"].volume, 1e-12), "a mitred corner keeps the centre-line length");
    assert!(close(expected("r-u", 6.0, 1.1, 7.0), solids["r-u"].volume, 1e-12));
    let low = &solids["r-low"];
    let (_, (rail_width, rail_depth)) = sections(&snapshot, "r-low");
    assert!(close(rail_width * rail_depth * 1.0, low.volume, 1e-12), "a railing lower than its rail is a rail alone");
    assert!(low.groups.iter().all(|group| group.part == parts::RAIL));
}

#[semio_framework_async_macros::async_test]
async fn posts_stand_on_the_base_and_the_rail_tops_the_railing() {
    let snapshot = case(CASE).snapshot;
    let solids = compute_element_solids(&snapshot);
    let (_, (_, rail_depth)) = sections(&snapshot, "r-l");
    let railing = &solids["r-l"];
    assert!(close(0.1, railing.bounds.min.z, 1e-12) && close(1.0, railing.bounds.max.z, 1e-12), "base offset 0.1, height 0.9");
    assert_eq!(railing.groups.iter().map(|g| (g.part.as_str(), g.material.as_str())).collect::<Vec<_>>(), vec![(parts::POST, "m-steel"), (parts::RAIL, "m-steel")]);
    let (_, post_top, _) = group_extent(railing, 0);
    let (rail_bottom, rail_top, _) = group_extent(railing, 1);
    assert!(close(1.0 - rail_depth, post_top, 1e-12) && close(1.0 - rail_depth, rail_bottom, 1e-12) && close(1.0, rail_top, 1e-12), "posts end where the rail begins");
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

fn part_volumes(solid: &ElementSolid) -> std::collections::BTreeMap<String, f64> {
    let mut found = std::collections::BTreeMap::new();
    for (index, group) in solid.groups.iter().enumerate() {
        *found.entry(group.part.clone()).or_insert(0.0) += group_extent(solid, index as u32).2;
    }
    found
}

#[semio_framework_async_macros::async_test]
async fn every_part_of_every_railing_has_the_volume_the_oracle_derives() {
    let case = case(CASE);
    let solids = compute_element_solids(&case.snapshot);
    for (id, row) in case.expected.as_object().expect("table").iter().filter(|(_, row)| !row.is_null()) {
        let found = part_volumes(&solids[id]);
        for (part, volume) in row["parts"].as_object().expect("parts") {
            let wanted = volume.as_f64().expect("number");
            assert!(close(wanted, found.get(part).copied().unwrap_or(0.0), row["volume_tolerance"].as_f64().expect("number")), "{id}.{part}: oracle {wanted}, subject {:?}", found.get(part));
        }
        assert_eq!(found.keys().filter(|part| found[*part].abs() > 1e-12).count(), row["parts"].as_object().expect("parts").values().filter(|volume| volume.as_f64().is_some_and(|v| v > 1e-12)).count(), "{id}: the same parts exist");
    }
}

#[semio_framework_async_macros::async_test]
async fn balusters_stand_strictly_between_the_posts_in_every_bay() {
    let snapshot = case(CASE).snapshot;
    let table = case(CASE).expected;
    for id in ["r-balusters", "r-balusters-glass", "r-balusters-short", "r-straight"] {
        let railing = &snapshot.railings[id];
        let found = geometry(&snapshot, id);
        assert_eq!(found.balusters.len() as u64, table[id]["balusters"].as_u64().expect("number"), "{id}: the oracle counts the same balusters");
        assert_eq!(baluster_count(railing) as usize, found.balusters.len(), "{id}: the count is the stations of the bays");
        assert!(found.balusters.iter().all(|station| found.posts.iter().all(|post| post.distance(*station) > 1e-6)), "{id}: a baluster never stands on a post");
    }
    let spacing = 0.13;
    let found = geometry(&snapshot, "r-balusters");
    let mut along: Vec<f64> = found.balusters.iter().map(|station| station.x - 10.0).filter(|x| *x < 1.5).collect();
    along.sort_by(f64::total_cmp);
    assert_eq!(along.len(), 11, "ceil(1.5 / 0.13) = 12 parts leave eleven stations");
    assert!(along.windows(2).all(|pair| close(1.5 / 12.0, pair[1] - pair[0], 1e-12)) && 1.5 / 12.0 <= spacing, "equal intervals no longer than the spacing");
    assert!(geometry(&snapshot, "r-balusters-short").balusters.is_empty(), "a spacing wider than the bay leaves it empty");
}

#[semio_framework_async_macros::async_test]
async fn the_infill_is_a_slab_per_bay_inset_by_half_a_post_in_glass_or_in_the_railing_material() {
    let snapshot = case(CASE).snapshot;
    let table = case(CASE).expected;
    let solids = compute_element_solids(&snapshot);
    let infill_material = |id: &str| solids[id].groups.iter().find(|group| group.part == parts::INFILL).map(|group| group.material.clone());
    assert_eq!(infill_material("r-glass").as_deref(), Some("m-glass"), "glass takes the glass material of the library");
    assert_eq!(infill_material("r-panel").as_deref(), Some("m-steel"), "a panel takes the material of the railing");
    assert_eq!(infill_material("r-straight"), None);
    let quantities = ModelInference::infer(&snapshot).expect("infers").quantities;
    for id in ["r-glass", "r-panel", "r-balusters-glass", "r-balusters-short"] {
        assert!(close(table[id]["infill_area"].as_f64().expect("number"), quantities.elements[id].surface_area, 1e-4), "{id}: the infill area of the quantity is the oracle's");
    }
    assert_eq!(quantities.elements["r-balusters"].balusters, 22);
    assert_eq!(quantities.elements["r-balusters-glass"].balusters, 30);
    assert_eq!((quantities.elements["r-straight"].balusters, quantities.elements["r-straight"].surface_area), (0, 0.0));
    let mut without_glass = snapshot.clone();
    without_glass.materials.remove("m-glass");
    assert_eq!(compute_element_solids(&without_glass)["r-glass"].groups.iter().find(|group| group.part == parts::INFILL).map(|group| group.material.as_str()), Some("m-steel"), "no glass in the library: the railing material");
}

#[semio_framework_async_macros::async_test]
async fn the_sections_and_the_infill_are_part_of_the_dependency_of_the_solid() {
    let snapshot = case(CASE).snapshot;
    let railing = snapshot.railings["r-glass"].clone();
    let base = dependency(&snapshot, &railing);
    let mut without_glass = snapshot.clone();
    without_glass.materials.remove("m-glass");
    assert_ne!(base, dependency(&without_glass, &railing), "the glass of the library is read");
    let mut other = railing.clone();
    other.infill = Infill::Glass { thickness: 0.02 };
    assert_ne!(base, dependency(&snapshot, &other));
    other = railing.clone();
    other.post_profile = Profile::Circle { diameter: 0.05 };
    assert_ne!(base, dependency(&snapshot, &other));
    let plain = snapshot.railings["r-straight"].clone();
    assert_eq!(dependency(&snapshot, &plain), dependency(&without_glass, &plain), "a railing without glass does not read the library");
}
