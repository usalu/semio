use super::*;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::testing::{assert_matches_oracle, case, close, group_extent, raised};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, SolidFamily};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::plan::solid_steps;
use crate::ModelInference;
use protocol::Inference;

const CASE: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/🔲️ceilings-holes-slope/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn ceilings_reproduce_the_third_party_oracle_table() {
    let case = case(CASE);
    assert_matches_oracle(&case, &compute_element_solids(&case.snapshot), SolidFamily::Ceiling);
}

#[semio_framework_async_macros::async_test]
async fn a_ceiling_with_a_hole_has_the_net_area_times_the_layer_thickness() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    assert!(close((24.0 - 1.0) * 0.0625, solids["c-holed"].volume, 1e-12), "6 x 4 less a 1 x 1 hole, board 0.0125 + wool 0.05");
    assert!(close(5.0 * 4.0 * 0.02, solids["c-sloped"].volume, 1e-12), "a slope does not change the volume: the thickness stays vertical");
    let net = 16.0 + 2.0 * std::f64::consts::PI - std::f64::consts::PI * 0.25;
    assert!(close(net * 0.0625, solids["c-curved"].volume, 1e-3), "a half-round end and a round hole within the sagitta bound");
}

#[semio_framework_async_macros::async_test]
async fn layers_hang_downward_from_the_drop_below_the_storey_top_first_layer_topmost() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    let holed = &solids["c-holed"];
    assert_eq!(holed.groups.iter().map(|g| (g.part.as_str(), g.material.as_str(), g.layer)).collect::<Vec<_>>(), vec![(parts::LAYER, "m-board", 0), (parts::LAYER, "m-wool", 1)]);
    let (low0, high0, volume0) = group_extent(holed, 0);
    let (low1, high1, volume1) = group_extent(holed, 1);
    assert!(close(2.7, high0, 1e-12) && close(2.7 - 0.0125, low0, 1e-12), "visible board: the top layer, 0.3 m below the storey top at 3.0 m");
    assert!(close(2.7 - 0.0125, high1, 1e-12) && close(2.7 - 0.0625, low1, 1e-12), "wool layer below the board");
    assert!(close(23.0 * 0.0125, volume0, 1e-12) && close(23.0 * 0.05, volume1, 1e-12));
}

#[semio_framework_async_macros::async_test]
async fn a_slope_tilts_the_plane_about_the_uphill_edge() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    let sloped = &solids["c-sloped"];
    let fall = 5.0 * 0.05f64.tan();
    assert!(close(2.9, sloped.bounds.max.z, 1e-12), "the uphill edge keeps its drop: storey top - offset");
    assert!(close(2.9 - fall - 0.02, sloped.bounds.min.z, 1e-12), "the downhill edge is lower by length x tan(angle), plus the thickness");
    let diagonal = &solids["c-diagonal-slope"];
    let (cos, sin) = ((std::f64::consts::PI / 3.0).cos(), (std::f64::consts::PI / 3.0).sin());
    let extent = 4.0 * cos + 3.0 * sin;
    assert!(close(5.6, diagonal.bounds.max.z, 1e-12) && close(5.6 - extent * 0.1f64.tan() - 0.02, diagonal.bounds.min.z, 1e-12));
}

#[semio_framework_async_macros::async_test]
async fn the_span_and_the_underside_follow_the_same_plane() {
    let snapshot = case(CASE).snapshot;
    let levels = crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels(&snapshot);
    let (holed, sloped) = (&snapshot.ceilings["c-holed"], &snapshot.ceilings["c-sloped"]);
    assert_eq!(span(&snapshot, holed, &levels["st-ground"]), (2.7 - 0.0625, 2.7));
    assert!(close(2.7 - 0.0625, underside_at(&snapshot, holed, &levels["st-ground"], Point2 { x: 1.0, y: 1.0 }).expect("under the board"), 1e-12));
    assert_eq!(underside_at(&snapshot, holed, &levels["st-ground"], Point2 { x: 2.5, y: 1.5 }), None, "no underside under the hole");
    assert_eq!(underside_at(&snapshot, holed, &levels["st-ground"], Point2 { x: 7.0, y: 1.5 }), None, "no underside beyond the boundary");
    let at = |x: f64| underside_at(&snapshot, sloped, &levels["st-ground"], Point2 { x, y: 2.0 }).expect("under the tile");
    assert!(close(2.9 - 0.02 - 0.5 * 0.05f64.tan(), at(0.5), 1e-12) && close(at(1.0) - at(3.0), 2.0 * 0.05f64.tan(), 1e-12), "the underside falls by tan(angle) per metre along the fall direction");
    assert!(close(fall_of(sloped), 5.0 * 0.05f64.tan(), 1e-12) && fall_of(holed) == 0.0);
}

#[semio_framework_async_macros::async_test]
async fn raising_a_storey_lifts_its_ceilings_and_those_of_the_storeys_above() {
    let snapshot = case(CASE).snapshot;
    let (before, after) = (compute_element_solids(&snapshot), compute_element_solids(&raised(&snapshot, "st-ground", 0.4)));
    for id in ["c-holed", "c-sloped"] {
        assert!(close(after[id].bounds.max.z - before[id].bounds.max.z, 0.4, 1e-9), "{id} hangs from the top of the storey that grew");
        assert!(close(after[id].volume, before[id].volume, 1e-12));
    }
    for id in ["c-diagonal-slope", "c-curved"] {
        assert!(close(after[id].bounds.max.z - before[id].bounds.max.z, 0.4, 1e-9), "{id} stands on the storey above");
    }
    let lowered = compute_element_solids(&raised(&snapshot, "st-first", 0.4));
    for id in ["c-holed", "c-sloped"] {
        assert_eq!(lowered[id], before[id], "{id} does not depend on a storey above it");
    }
    for id in ["c-diagonal-slope", "c-curved"] {
        assert!(close(lowered[id].bounds.max.z - before[id].bounds.max.z, 0.4, 1e-9), "{id} hangs from the top of the storey that grew");
    }
}

#[semio_framework_async_macros::async_test]
async fn ceilings_without_layers_or_type_are_absent_and_the_result_is_deterministic() {
    let snapshot = case(CASE).snapshot;
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred, ModelInference::infer(&snapshot).expect("infers"));
    assert!(["c-bare", "c-unknown-type"].iter().all(|id| !inferred.element_solids.contains_key(*id)));
    assert_eq!(solid_steps(&snapshot, SolidFamily::Ceiling).len(), snapshot.ceilings.len());
    assert!(solid_steps(&ModelSnapshot::default(), SolidFamily::Ceiling).is_empty());
    assert!(ceiling_layers(&snapshot.ceilings["c-bare"], &snapshot.ceiling_types["ct-bare"], &StoreyLevel::default()).is_empty());
    assert_eq!(thickness(&snapshot, &snapshot.ceilings["c-unknown-type"]), 0.0);
}

const TAKEOFF: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🔲️ceilings/🔲️takeoff/📸️snapshot/🔣️.json");
const TAKEOFF_TABLE: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🔲️ceilings/🔲️takeoff/💡️inference/🔲️ceilings/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_take_off_and_the_span_reproduce_the_shapely_oracle_table() {
    let snapshot: ModelSnapshot = semio_framework_pack_json::from_json_str(TAKEOFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the take-off model decodes");
    let table: serde_json::Value = serde_json::from_str(TAKEOFF_TABLE).expect("the oracle table");
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(table.as_object().expect("rows").len(), snapshot.ceilings.len());
    for (id, ceiling) in &snapshot.ceilings {
        let row = &table[id];
        let (Some(quantity), Some(level)) = (inferred.quantities.elements.get(id).filter(|_| inferred.element_solids.contains_key(id)), inferred.storey_levels.get(&ceiling.storey)) else {
            assert!(row.is_null(), "{id}: the subject emits no solid, so the oracle has no row");
            continue;
        };
        let (bottom, top) = span(&snapshot, ceiling, level);
        let count = ceiling.boundary.len() as f64;
        let first = ceiling.boundary[0].point;
        let mean = (ceiling.boundary.iter().map(|vertex| vertex.point.x).sum::<f64>() / count, ceiling.boundary.iter().map(|vertex| vertex.point.y).sum::<f64>() / count);
        let probe = Point2 { x: first.x + 0.05 * (mean.0 - first.x), y: first.y + 0.05 * (mean.1 - first.y) };
        let under = underside_at(&snapshot, ceiling, level, probe);
        let found = [("gross_area", quantity.gross_area), ("net_area", quantity.net_area), ("surface_area", quantity.surface_area), ("perimeter", quantity.perimeter), ("width", quantity.width), ("gross_volume", quantity.gross_volume), ("net_volume", quantity.net_volume), ("mass", quantity.mass), ("top_z", top), ("bottom_z", bottom)];
        for (name, value) in found {
            assert!(close(row[name].as_f64().expect("a number"), value, 1e-9), "{id}.{name}: oracle {} against {value}", row[name]);
        }
        match (row["underside_z"].as_f64(), under) {
            (Some(want), Some(got)) => assert!(close(want, got, 1e-9), "{id}.underside_z: oracle {want} against {got}"),
            (want, got) => assert_eq!(want, got, "{id}.underside_z"),
        }
    }
}
