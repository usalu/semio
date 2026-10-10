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

fn rise_of(geometry: &RoofGeometry, thickness: f64) -> f64 {
    geometry.ridge_z - geometry.eave_z - thickness
}

#[semio_framework_async_macros::async_test]
async fn roofs_reproduce_the_third_party_oracle_table() {
    let case = case(CASE);
    assert_matches_oracle(&case, &compute_element_solids(&case.snapshot), SolidFamily::Roof);
}

#[semio_framework_async_macros::async_test]
async fn fallbacks_rises_and_ridge_lengths_agree_with_the_oracle() {
    let case = case(CASE);
    for (id, row) in case.expected.as_object().expect("table").iter().filter(|(_, row)| !row.is_null()) {
        let roof = geometry(&case.snapshot, id);
        assert_eq!(roof.fallback.map(RoofFallback::code), row["fallback"].as_str(), "{id}.fallback");
        assert!(close(row["eave_z"].as_f64().expect("number"), roof.eave_z, 1e-12), "{id}.eave_z");
        assert!(close(row["rise"].as_f64().expect("number"), rise_of(&roof, row["thickness"].as_f64().expect("number")), 1e-9), "{id}.rise: oracle {}, subject {}", row["rise"], rise_of(&roof, row["thickness"].as_f64().expect("number")));
        if let Some(length) = row["ridge_length"].as_f64() {
            let ridges = lines_of_kind(&roof, RoofLineKind::Ridge);
            assert_eq!(ridges.len(), 1, "{id} has one ridge");
            assert!(close(length, ridges[0], row["ridge_length_tolerance"].as_f64().expect("number")), "{id}.ridge_length: oracle {length}, subject {}", ridges[0]);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn the_upward_surface_of_a_pitched_roof_is_its_plan_area_over_the_cosine_of_the_pitch() {
    let case = case(CASE);
    let quantities = ModelInference::infer(&case.snapshot).expect("infers").quantities;
    let mut measured = 0;
    for (id, row) in case.expected.as_object().expect("table").iter().filter(|(_, row)| row.get("slope_area").is_some()) {
        assert!(close(row["slope_area"].as_f64().expect("number"), quantities.elements[id].surface_area, 1e-6), "{id}.surface_area");
        measured += 1;
    }
    assert!(measured >= 9, "hips and gables of every footprint: {measured}");
}

#[semio_framework_async_macros::async_test]
async fn roofs_have_the_analytic_volume_of_the_eave_area_times_the_layers() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    assert!(close(9.0 * 7.0 * 0.2, solids["r-flat"].volume, 1e-7), "flat: 8 x 6 grown by 0.5 on every side");
    assert!(close(9.0 * 7.0 * 0.1, solids["r-gable"].volume, 1e-7), "the vertical build-up is independent of the pitch");
    assert!(close(8.0 * 6.0 * 0.1, solids["r-hip"].volume, 1e-7));
    assert!(close(8.8 * 6.8 * 0.1, solids["r-hip-overhang"].volume, 1e-7));
    assert!(close(8.0 * 6.0 * 0.1, solids["r-mansard"].volume, 1e-7));
    assert!(close((8.0 * 3.0 + 4.0 * 3.0) * 0.1, solids["r-l-hip"].volume, 1e-7), "an L of two arms: 24 + 12 square metres");
    assert!(close((9.0 * 3.0 + 3.0 * 4.0) * 0.1, solids["r-t-hip"].volume, 1e-7) && close((9.0 * 3.0 + 2.0 * 3.0 * 3.0) * 0.1, solids["r-u-hip"].volume, 1e-7));
    assert!(close((36.0 + 28.0 * 0.4 + 4.0 * 0.16) * 0.1, solids["r-l-hip-overhang"].volume, 1e-7), "the mitred overhang of an L: perimeter x 0.4 plus four net corner squares");
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
    assert_eq!((lines_of_kind(&gable, RoofLineKind::Ridge).len(), lines_of_kind(&gable, RoofLineKind::Hip).len(), lines_of_kind(&gable, RoofLineKind::Verge).len()), (1, 0, 4), "a gable has a ridge and a verge up each corner of the gable ends");
    let mansard = geometry(&snapshot, "r-mansard");
    assert_eq!((lines_of_kind(&mansard, RoofLineKind::Ridge).len(), lines_of_kind(&mansard, RoofLineKind::Break).len(), lines_of_kind(&mansard, RoofLineKind::Hip).len()), (1, 4, 8));
    assert!(geometry(&snapshot, "r-flat").lines.is_empty() && geometry(&snapshot, "r-shed").lines.is_empty(), "single plane roofs have no inner lines");
}

#[semio_framework_async_macros::async_test]
async fn concave_footprints_have_valleys_and_convex_ones_do_not() {
    let snapshot = case(CASE).snapshot;
    for id in ["r-l-hip", "r-t-hip", "r-u-hip", "r-l-hip-overhang", "r-l-mansard", "r-l-gable"] {
        let roof = geometry(&snapshot, id);
        assert_eq!(roof.fallback.map(RoofFallback::code).filter(|code| !code.contains("adjust")), None, "{id}: a concave footprint is no reason to fall back");
        assert!(!lines_of_kind(&roof, RoofLineKind::Valley).is_empty() && !lines_of_kind(&roof, RoofLineKind::Hip).is_empty(), "{id} has hips and valleys");
        assert!(roof.ridge_z > roof.eave_z + 0.1 + 1e-6, "{id} rises above the eaves");
    }
    for id in ["r-hip", "r-gable", "r-mansard", "r-hip-overhang"] {
        assert!(lines_of_kind(&geometry(&snapshot, id), RoofLineKind::Valley).is_empty(), "{id}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_rise_of_a_hip_roof_over_an_l_is_the_tangent_of_the_pitch_times_the_half_width_of_its_widest_arm() {
    let mut snapshot = case(CASE).snapshot;
    for pitch in [0.2, 0.5, 0.9, 1.2] {
        snapshot.roofs.get_mut("r-l-hip").expect("roof").shape = RoofShape::Hip { pitch };
        let found = geometry(&snapshot, "r-l-hip");
        assert!(close(2.0 * f64::tan(pitch), rise_of(&found, 0.1), 1e-9), "pitch {pitch}: the 4 m arm collapses on a ridge 2 m from its eaves");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_gable_next_to_a_reflex_corner_is_built_as_a_hip_and_says_so() {
    let snapshot = case(CASE).snapshot;
    let gable = geometry(&snapshot, "r-l-gable");
    assert_eq!(gable.fallback, Some(RoofFallback::GableEndsAdjustToHip));
    let mut hip = snapshot.clone();
    hip.roofs.get_mut("r-l-gable").expect("roof").shape = RoofShape::Hip { pitch: 0.35 };
    assert_eq!(geometry(&hip, "r-l-gable").fallback, None);
    assert_eq!(compute_element_solids(&hip)["r-l-gable"], compute_element_solids(&snapshot)["r-l-gable"], "the adjusted gable is exactly the hip of the same pitch");
    assert_eq!(geometry(&snapshot, "r-gable").fallback, None, "a gable on a rectangle keeps its gable ends");
    assert_eq!(geometry(&snapshot, "r-gable-rotated").fallback, None, "so does one on a rotated rectangle");
    assert_eq!(RoofFallback::GableEndsAdjustToHip.code(), "roof.gable-ends-adjust-to-hip");
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    let reported: Vec<&str> = inferred.diagnostics.iter().filter(|row| row.code.slug() == "roof.gable-ends-adjust-to-hip").flat_map(|row| row.elements.iter().map(String::as_str)).collect();
    assert_eq!(reported, vec!["r-l-gable"]);
}

#[semio_framework_async_macros::async_test]
async fn what_a_shape_cannot_do_falls_back_to_a_flat_roof_with_a_reason() {
    let snapshot = case(CASE).snapshot;
    assert_eq!(geometry(&snapshot, "r-curved-gable").fallback, Some(RoofFallback::CurvedFootprint));
    assert_eq!(geometry(&snapshot, "r-bad-pitch").fallback, Some(RoofFallback::InvalidPitch));
    assert_eq!(geometry(&snapshot, "r-mansard-no-break").fallback, Some(RoofFallback::InvalidPitch));
    assert_eq!(geometry(&snapshot, "r-bowtie").fallback, Some(RoofFallback::DegenerateFootprint));
    assert_eq!(geometry(&snapshot, "r-l-hip").fallback, None, "an L is a hip roof now");
    assert_eq!(geometry(&snapshot, "r-gable").fallback, None);
    assert_eq!(geometry(&snapshot, "r-hip-zero-pitch").fallback, None, "a pitch of zero is a flat roof, not an error");
    assert_eq!(geometry(&snapshot, "r-curved-shed").fallback, None, "a shed is one plane: any footprint works");
    for id in ["r-curved-gable", "r-bad-pitch", "r-mansard-no-break", "r-hip-zero-pitch"] {
        let roof = geometry(&snapshot, id);
        assert!(close(roof.ridge_z, roof.eave_z + 0.1, 1e-12) && roof.lines.is_empty(), "{id} is flat at the eave height");
    }
    assert!(!compute_element_solids(&snapshot).contains_key("r-bowtie"), "a footprint crossing itself has no roof");
    assert_eq!(RoofFallback::SkeletonFailed.code(), "roof.fallback-flat.skeleton");
    assert_eq!(RoofFallback::CurvedFootprint.code(), "roof.fallback-flat.curved-footprint");
}

#[semio_framework_async_macros::async_test]
async fn every_fallback_is_reported_as_a_diagnostic_of_its_roof() {
    let inferred = ModelInference::infer(&case(CASE).snapshot).expect("infers");
    let reported = |slug: &str| -> Vec<&str> { inferred.diagnostics.iter().filter(|row| row.code.slug() == slug).flat_map(|row| row.elements.iter().map(String::as_str)).collect() };
    assert_eq!(reported("roof.fallback-flat.curved-footprint"), vec!["r-curved-gable"]);
    assert_eq!(reported("roof.fallback-flat.invalid-pitch"), vec!["r-bad-pitch", "r-mansard-no-break"]);
    assert_eq!(reported("roof.fallback-flat.degenerate-footprint"), vec!["r-bowtie"]);
    assert!(reported("roof.fallback-flat.skeleton").is_empty());
    for row in inferred.diagnostics.iter().filter(|row| row.code.slug().starts_with("roof.")) {
        assert!(row.text("en").is_some() && row.text("de").is_some(), "{} has an English and a German text", row.code.slug());
    }
}

#[semio_framework_async_macros::async_test]
async fn a_cancelled_skeleton_leaves_a_flat_roof_that_says_why() {
    let snapshot = case(CASE).snapshot;
    let roof = &snapshot.roofs["r-l-hip"];
    let thicknesses = layer_thicknesses(&snapshot.roof_types[&roof.roof_type].layers);
    let level = compute_storey_levels(&snapshot)[&roof.storey];
    let mut polls = 0;
    let cancelled = roof_geometry_controlled(roof, &thicknesses, &level, &mut || {
        polls += 1;
        false
    });
    assert!(polls >= 1, "the skeleton polls the control");
    assert_eq!(cancelled.fallback, Some(RoofFallback::SkeletonFailed));
    assert!(cancelled.lines.is_empty() && close(cancelled.ridge_z, cancelled.eave_z + 0.1, 1e-12));
    assert_eq!(roof_geometry_controlled(roof, &thicknesses, &level, &mut || true), roof_geometry(roof, &thicknesses, &level), "an untouched control is the plain run");
}

#[semio_framework_async_macros::async_test]
async fn layers_stack_upward_from_the_underside_first_layer_outermost() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    let flat = &solids["r-gable"];
    assert_eq!(flat.groups.iter().map(|g| (g.part.as_str(), g.material.as_str(), g.layer)).collect::<Vec<_>>(), vec![(parts::LAYER, "m-clay", 0), (parts::LAYER, "m-wood", 1)]);
    let (_, _, tiles) = group_extent(flat, 0);
    let (_, _, battens) = group_extent(flat, 1);
    assert!(close(63.0 * 0.04, tiles, 1e-7) && close(63.0 * 0.06, battens, 1e-7), "each layer is the eave area times its own thickness");
    let (tile_low, _, tile_volume) = group_extent(&solids["r-t-hip"], 0);
    let (batten_low, _, batten_volume) = group_extent(&solids["r-t-hip"], 1);
    assert!(close(5.8, batten_low, 1e-9) && close(5.86, tile_low, 1e-9), "the battens are the lowest layer, down to the eave height; the tiles lie 0.06 above");
    assert!(close(39.0 * 0.04, tile_volume, 1e-9) && close(39.0 * 0.06, batten_volume, 1e-9), "the shell of a concave roof keeps the eave area times the thickness of each layer");
}

#[semio_framework_async_macros::async_test]
async fn changing_a_storey_height_lifts_every_roof_standing_on_its_top() {
    let snapshot = case(CASE).snapshot;
    let (before, after) = (compute_element_solids(&snapshot), compute_element_solids(&raised(&snapshot, "st-first", 0.4)));
    for id in ["r-flat", "r-shed", "r-gable", "r-hip", "r-mansard", "r-l-hip", "r-t-hip", "r-u-hip", "r-l-mansard"] {
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
    assert!(solid_steps(&snapshot, SolidFamily::Roof).iter().all(|step| matches!(&step.key, ModelNode::Solid(key) if *key == SolidKey::of(SolidFamily::Roof, &key.id)) && step.parents == vec![ModelNode::Storey("st-first".into())]), "a roof depends on its storey and on nothing else of the graph");
}
