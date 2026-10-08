use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{plan::solid_steps, ModelNode};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidKey;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::testing::{assert_matches_oracle, case, close, group_extent, raised};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, SolidFamily};
use crate::ModelInference;
use protocol::Inference;

const CASE: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/⬜️slabs-holes-slope/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn slabs_reproduce_the_third_party_oracle_table() {
    let case = case(CASE);
    assert_matches_oracle(&case, &compute_element_solids(&case.snapshot), SolidFamily::Slab);
}

#[semio_framework_async_macros::async_test]
async fn a_slab_with_a_hole_has_the_net_area_times_the_layer_thickness() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    assert!(close((24.0 - 1.0) * 0.25, solids["s-holed"].volume, 1e-12), "6 x 4 less a 1 x 1 hole, wood 0.05 + concrete 0.2");
    assert!(close(5.0 * 4.0 * 0.15, solids["s-sloped"].volume, 1e-12), "a slope does not change the volume: the thickness stays vertical");
    let net = 16.0 + 2.0 * std::f64::consts::PI - std::f64::consts::PI * 0.25;
    assert!(close(net * 0.25, solids["s-curved"].volume, 1e-3), "a half-round end and a round hole within the sagitta bound");
}

#[semio_framework_async_macros::async_test]
async fn layers_stack_downward_from_the_top_first_layer_topmost() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    let holed = &solids["s-holed"];
    assert_eq!(holed.groups.iter().map(|g| (g.part.as_str(), g.material.as_str(), g.layer)).collect::<Vec<_>>(), vec![(parts::LAYER, "m-wood", 0), (parts::LAYER, "m-concrete", 1)]);
    let (low0, high0, volume0) = group_extent(holed, 0);
    let (low1, high1, volume1) = group_extent(holed, 1);
    assert!(close(0.0, high0, 1e-12) && close(-0.05, low0, 1e-12), "finish layer: the top 0.05");
    assert!(close(-0.05, high1, 1e-12) && close(-0.25, low1, 1e-12), "structure layer below it");
    assert!(close(23.0 * 0.05, volume0, 1e-12) && close(23.0 * 0.2, volume1, 1e-12));
}

#[semio_framework_async_macros::async_test]
async fn a_slope_tilts_the_plane_about_the_uphill_edge() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    let sloped = &solids["s-sloped"];
    let fall = 5.0 * 0.05f64.tan();
    assert!(close(-0.05, sloped.bounds.max.z, 1e-12), "the uphill edge keeps the reference height: storey + offset");
    assert!(close(-0.05 - fall - 0.15, sloped.bounds.min.z, 1e-12), "the downhill edge is lower by length x tan(angle), plus the thickness");
    let diagonal = &solids["s-diagonal-slope"];
    let (cos, sin) = ((std::f64::consts::PI / 3.0).cos(), (std::f64::consts::PI / 3.0).sin());
    let extent = 4.0 * cos + 3.0 * sin;
    assert!(close(3.0 - 0.1, diagonal.bounds.max.z, 1e-12) && close(3.0 - 0.1 - extent * 0.1f64.tan() - 0.15, diagonal.bounds.min.z, 1e-12));
}

#[semio_framework_async_macros::async_test]
async fn changing_a_storey_height_lifts_the_slabs_of_the_storeys_above_only() {
    let snapshot = case(CASE).snapshot;
    let (before, after) = (compute_element_solids(&snapshot), compute_element_solids(&raised(&snapshot, "st-ground", 0.4)));
    for id in ["s-diagonal-slope", "s-curved"] {
        assert!(close(after[id].bounds.max.z - before[id].bounds.max.z, 0.4, 1e-9), "{id} stands on the storey above");
        assert!(close(after[id].volume, before[id].volume, 1e-12));
    }
    for id in ["s-holed", "s-sloped"] {
        assert_eq!(after[id], before[id], "{id} stands on the storey that grew: its top is the storey elevation");
    }
}

#[semio_framework_async_macros::async_test]
async fn slabs_without_layers_or_type_are_absent_and_the_result_is_deterministic() {
    let snapshot = case(CASE).snapshot;
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred, ModelInference::infer(&snapshot).expect("infers"));
    assert!(["s-bare", "s-unknown-type"].iter().all(|id| !inferred.element_solids.contains_key(*id)));
    assert_eq!(solid_steps(&snapshot, SolidFamily::Slab).len(), snapshot.slabs.len());
    assert!(solid_steps(&ModelSnapshot::default(), SolidFamily::Slab).is_empty());
    assert!(slab_layers(&snapshot.slabs["s-bare"], &snapshot.slab_types["slt-bare"], &StoreyLevel::default()).is_empty());
}
