use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{plan::solid_steps, ModelNode};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidKey;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{layer_thicknesses, testing::{assert_matches_oracle, case, close, group_extent, raised}};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, SolidFamily};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels;
use crate::ModelInference;
use protocol::Inference;

const CASE: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/🏔️roofs-shapes/🔣️.json");

fn geometry(snapshot: &ModelSnapshot, id: &str) -> RoofGeometry {
    let roof = &snapshot.roofs[id];
    roof_geometry(roof, &layer_thicknesses(&snapshot.roof_types[&roof.roof_type].layers), &compute_storey_levels(snapshot)[&roof.storey])
}

fn lines_of_kind(geometry: &RoofGeometry, kind: RoofLineKind) -> Vec<f64> {
    geometry.lines.iter().filter(|line| line.kind == kind).map(|line| ((line.end[0] - line.start[0]).powi(2) + (line.end[1] - line.start[1]).powi(2)).sqrt()).collect()
}

#[semio_framework_async_macros::async_test]
async fn roofs_reproduce_the_third_party_oracle_table() {
    let case = case(CASE);
    assert_matches_oracle(&case, &compute_element_solids(&case.snapshot), SolidFamily::Roof);
}

#[semio_framework_async_macros::async_test]
async fn fallbacks_and_ridge_lengths_agree_with_the_oracle() {
    let case = case(CASE);
    for (id, row) in case.expected.as_object().expect("table").iter().filter(|(_, row)| !row.is_null()) {
        let roof = geometry(&case.snapshot, id);
        assert_eq!(roof.fallback.map(RoofFallback::code), row["fallback"].as_str(), "{id}.fallback");
        assert!(close(row["eave_z"].as_f64().expect("number"), roof.eave_z, 1e-12), "{id}.eave_z");
        if let Some(length) = row["ridge_length"].as_f64() {
            let ridges = lines_of_kind(&roof, RoofLineKind::Ridge);
            assert_eq!(ridges.len(), 1, "{id} has one ridge");
            assert!(close(length, ridges[0], row["ridge_length_tolerance"].as_f64().expect("number")), "{id}.ridge_length: oracle {length}, subject {}", ridges[0]);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn roofs_have_the_analytic_volume_of_the_eave_area_times_the_layers() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    assert!(close(9.0 * 7.0 * 0.2, solids["r-flat"].volume, 1e-12), "flat: 8 x 6 grown by 0.5 on every side");
    assert!(close(9.0 * 7.0 * 0.1, solids["r-gable"].volume, 1e-12), "the vertical build-up is independent of the pitch");
    assert!(close(8.0 * 6.0 * 0.1, solids["r-hip"].volume, 1e-12));
    assert!(close(8.8 * 6.8 * 0.1, solids["r-hip-overhang"].volume, 1e-12));
    assert!(close(8.0 * 6.0 * 0.1, solids["r-mansard"].volume, 1e-12));
}

#[semio_framework_async_macros::async_test]
async fn the_ridge_and_the_hips_of_each_shape_are_where_the_planes_meet() {
    let snapshot = case(CASE).snapshot;
    let hip = geometry(&snapshot, "r-hip");
    assert_eq!(lines_of_kind(&hip, RoofLineKind::Hip).len(), 4);
    assert!(close(2.0, lines_of_kind(&hip, RoofLineKind::Ridge)[0], 1e-9), "an 8 x 6 hip roof has a ridge of 8 - 6");
    let top = hip.lines.iter().find(|line| line.kind == RoofLineKind::Ridge).expect("a ridge");
    assert!(close(top.start[2], hip.ridge_z, 1e-9) && close(top.end[2], hip.ridge_z, 1e-9), "the ridge is level at the top of the roof");
    let gable = geometry(&snapshot, "r-gable");
    assert_eq!((lines_of_kind(&gable, RoofLineKind::Ridge).len(), lines_of_kind(&gable, RoofLineKind::Hip).len()), (1, 0));
    let mansard = geometry(&snapshot, "r-mansard");
    assert_eq!((lines_of_kind(&mansard, RoofLineKind::Ridge).len(), lines_of_kind(&mansard, RoofLineKind::Break).len(), lines_of_kind(&mansard, RoofLineKind::Hip).len()), (1, 4, 8));
    assert!(geometry(&snapshot, "r-flat").lines.is_empty() && geometry(&snapshot, "r-shed").lines.is_empty(), "single plane roofs have no inner lines");
}

#[semio_framework_async_macros::async_test]
async fn what_a_shape_cannot_do_falls_back_to_a_flat_roof_with_a_reason() {
    let snapshot = case(CASE).snapshot;
    assert_eq!(geometry(&snapshot, "r-l-hip").fallback, Some(RoofFallback::NonConvexFootprint));
    assert_eq!(geometry(&snapshot, "r-curved-gable").fallback, Some(RoofFallback::CurvedFootprint));
    assert_eq!(geometry(&snapshot, "r-bad-pitch").fallback, Some(RoofFallback::InvalidPitch));
    assert_eq!(geometry(&snapshot, "r-gable").fallback, None);
    assert_eq!(geometry(&snapshot, "r-curved-shed").fallback, None, "a shed is one plane: any footprint works");
    for id in ["r-l-hip", "r-curved-gable", "r-bad-pitch"] {
        let roof = geometry(&snapshot, id);
        assert!(close(roof.ridge_z, roof.eave_z + 0.1, 1e-12) && roof.lines.is_empty(), "{id} is flat at the eave height");
    }
    assert_eq!(RoofFallback::NonConvexFootprint.code(), "roof.fallback-flat.non-convex-footprint");
}

#[semio_framework_async_macros::async_test]
async fn layers_stack_upward_from_the_underside_first_layer_outermost() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    let flat = &solids["r-gable"];
    assert_eq!(flat.groups.iter().map(|g| (g.part.as_str(), g.material.as_str(), g.layer)).collect::<Vec<_>>(), vec![(parts::LAYER, "m-clay", 0), (parts::LAYER, "m-wood", 1)]);
    let (_, _, tiles) = group_extent(flat, 0);
    let (_, _, battens) = group_extent(flat, 1);
    assert!(close(63.0 * 0.04, tiles, 1e-12) && close(63.0 * 0.06, battens, 1e-12), "each layer is the eave area times its own thickness");
}

#[semio_framework_async_macros::async_test]
async fn changing_a_storey_height_lifts_every_roof_standing_on_its_top() {
    let snapshot = case(CASE).snapshot;
    let (before, after) = (compute_element_solids(&snapshot), compute_element_solids(&raised(&snapshot, "st-first", 0.4)));
    for id in ["r-flat", "r-shed", "r-gable", "r-hip", "r-mansard", "r-l-hip"] {
        assert!(close(after[id].bounds.min.z - before[id].bounds.min.z, 0.4, 1e-9) && close(after[id].bounds.max.z - before[id].bounds.max.z, 0.4, 1e-9), "{id}");
        assert!(close(after[id].volume, before[id].volume, 1e-9), "{id} keeps its shape");
    }
    let unchanged = compute_element_solids(&raised(&snapshot, "st-ground", 0.0));
    assert_eq!(unchanged, before, "a no-op height edit changes nothing");
}

#[semio_framework_async_macros::async_test]
async fn roofs_are_deterministic_and_unknown_types_are_absent() {
    let snapshot = case(CASE).snapshot;
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred, ModelInference::infer(&snapshot).expect("infers"));
    assert!(!inferred.element_solids.contains_key("r-unknown-type"));
    assert_eq!(solid_steps(&snapshot, SolidFamily::Roof).len(), snapshot.roofs.len());
    assert!(solid_steps(&ModelSnapshot::default(), SolidFamily::Roof).is_empty());
}
