use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use crate::{Assigned, AreaSchemePatch, Entry, ModelDiff, SpacePatch, ZonePatch};
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const ZONING: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🏘️zones/🏡️zoning/📸️snapshot/🔣️.json");
const ZONING_TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🏘️zones/🏡️zoning/💡️inference/🏘️zones/🔣️.json");

fn zoning() -> ModelSnapshot {
    from_json_str(ZONING, JsonMemberPolicy::Reject).expect("the fixture decodes")
}

fn infer(snapshot: &ModelSnapshot) -> ModelInference {
    ModelInference::infer(snapshot).expect("infers")
}

fn disagreements(expected: &serde_json::Value, actual: &serde_json::Value, path: &str) -> Vec<String> {
    use serde_json::Value;
    match (expected, actual) {
        (Value::Object(want), Value::Object(got)) => {
            let mut problems: Vec<String> = if want.len() == got.len() { Vec::new() } else { vec![format!("{path}: {} members expected, {} found", want.len(), got.len())] };
            problems.extend(want.iter().flat_map(|(key, value)| got.get(key).map_or_else(|| vec![format!("{path}.{key}: missing")], |found| disagreements(value, found, &format!("{path}.{key}")))));
            problems
        }
        (Value::Number(want), Value::Number(got)) => {
            let (want, got) = (want.as_f64().expect("number"), got.as_f64().expect("number"));
            if (want - got).abs() <= 1e-9 * want.abs().max(1.0) { Vec::new() } else { vec![format!("{path}: oracle {want}, subject {got}")] }
        }
        _ if expected == actual => Vec::new(),
        _ => vec![format!("{path}: oracle {expected}, subject {actual}")],
    }
}

#[semio_framework_async_macros::async_test]
async fn the_subject_reproduces_the_third_party_oracle_table() {
    let inferred = infer(&zoning());
    let produced: serde_json::Value = serde_json::from_str(&table_json(&inferred.zone_totals, &inferred.scheme_totals)).expect("JSON table");
    let expected: serde_json::Value = serde_json::from_str(ZONING_TABLE).expect("JSON table");
    let problems = disagreements(&expected, &produced, "");
    assert!(problems.is_empty(), "{problems:#?}");
}

#[semio_framework_async_macros::async_test]
async fn a_zone_counts_its_members_and_only_the_resolved_ones_add_up() {
    let inferred = infer(&zoning());
    let day = &inferred.zone_totals["z-day"];
    assert_eq!((day.spaces, day.resolved), (2, 1), "the open space belongs to the zone but has no room");
    assert!((day.net_area - 21.09).abs() < 1e-9);
    assert!((day.occupancy - 0.1 * 21.09).abs() < 1e-9, "occupancy is density times net floor area");
}

#[semio_framework_async_macros::async_test]
async fn a_scheme_counts_by_usage_and_zone_and_measures_gross_or_net() {
    let mut snapshot = zoning();
    let inferred = infer(&snapshot);
    assert_eq!(inferred.scheme_totals["as-gfa"].spaces, 4, "empty lists count every space");
    assert_eq!(inferred.scheme_totals["as-nsa"].spaces, 3, "usages Living and Kitchen");
    assert_eq!(inferred.scheme_totals["as-sleep"].spaces, 1, "usage and zone must both match");
    let gross = inferred.scheme_totals["as-gfa"].area;
    snapshot.area_schemes.get_mut("as-gfa").expect("the scheme").measure = AreaMeasure::Net;
    let net = infer(&snapshot).scheme_totals["as-gfa"].area;
    assert!((gross - net - 0.16).abs() < 1e-9, "the net area only cuts the column out of the room: {gross} {net}");
}

#[semio_framework_async_macros::async_test]
async fn a_space_in_no_zone_adds_no_occupancy() {
    let mut snapshot = zoning();
    snapshot.spaces.get_mut("sp-up").expect("the space").zone = None;
    let inferred = infer(&snapshot);
    assert_eq!(inferred.zone_totals["z-night"].spaces, 1);
    assert_eq!(inferred.scheme_totals["as-gfa"].spaces, 4, "an empty zone list counts spaces in no zone");
    assert_eq!(inferred.scheme_totals["as-day"].spaces, 2);
    assert!((inferred.scheme_totals["as-sleep"].occupancy).abs() < 1e-12, "no zone, no density");
}

#[semio_framework_async_macros::async_test]
async fn a_zone_membership_edit_re_infers_only_the_zones_and_schemes() {
    let snapshot = zoning();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let diff = ModelDiff::spaces("sp-west", Entry::Patched(SpacePatch { zone: Some(Assigned::new(Some("z-night".into()))), ..Default::default() }));
    let edited = protocol::apply_diff(&diff, &snapshot).expect("applies");
    session.update(&edited, &diff);
    let computed = &session.report().computed_by_kind;
    for kind in ["room", "wall-layout", "solid", "plan", "opening-frame", "quantity", "diagnostics"] {
        assert_eq!(computed.get(kind), None, "a membership edit recomputes no {kind}");
    }
    assert_eq!(computed.get("zone"), Some(&2), "both zones change members");
    assert!(computed.get("scheme").is_some_and(|count| *count >= 1));
    assert_eq!(session.inference(), &infer(&edited), "the session agrees with a fresh inference");
}

#[semio_framework_async_macros::async_test]
async fn a_density_edit_re_infers_the_zone_and_the_schemes_that_read_it() {
    let snapshot = zoning();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let diff = ModelDiff::zones("z-day", Entry::Patched(ZonePatch { occupancy_density: Some(0.2), ..Default::default() }));
    let edited = protocol::apply_diff(&diff, &snapshot).expect("applies");
    let occupancy = session.update(&edited, &diff).zone_totals["z-day"].occupancy;
    assert!((occupancy - 0.2 * 21.09).abs() < 1e-9);
    assert_eq!(session.report().computed_by_kind.get("zone"), Some(&1));
    assert_eq!(session.report().computed_by_kind.get("quantity"), None);
    assert_eq!(session.inference(), &infer(&edited));
}

fn computed(session: &ModelInferenceSession) -> Vec<(String, usize)> {
    session.report().computed_by_kind.iter().filter(|(kind, _)| ["room", "wall-layout", "solid", "plan", "opening-frame", "quantity", "diagnostics", "zone", "scheme"].contains(kind)).map(|(kind, count)| (kind.to_string(), *count)).collect()
}

#[semio_framework_async_macros::async_test]
async fn a_scheme_usage_edit_re_infers_only_that_scheme() {
    let snapshot = zoning();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let diff = ModelDiff::area_schemes("as-nsa", Entry::Patched(AreaSchemePatch { usages: Some(vec!["Living".into()]), ..Default::default() }));
    let edited = protocol::apply_diff(&diff, &snapshot).expect("applies");
    assert_eq!(session.update(&edited, &diff).scheme_totals["as-nsa"].spaces, 2, "the living room and the open space");
    assert_eq!(computed(&session), [("scheme".to_string(), 1)], "no room, take-off or zone is recomputed");
    assert_eq!(session.inference(), &infer(&edited), "the session agrees with a fresh inference");
}

#[semio_framework_async_macros::async_test]
async fn a_scheme_zone_list_edit_moves_the_membership_of_the_scheme_only() {
    let snapshot = zoning();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let diff = ModelDiff::area_schemes("as-day", Entry::Patched(AreaSchemePatch { zones: Some(vec!["z-night".into()]), ..Default::default() }));
    let edited = protocol::apply_diff(&diff, &snapshot).expect("applies");
    let totals = session.update(&edited, &diff).scheme_totals.clone();
    assert_eq!(totals["as-day"].spaces, 2, "the kitchen and the bedroom live in the night zone");
    assert_eq!(totals["as-nsa"], infer(&snapshot).scheme_totals["as-nsa"], "another scheme keeps its totals");
    assert_eq!(computed(&session), [("scheme".to_string(), 1)]);
    assert_eq!(session.inference(), &infer(&edited));
}

#[semio_framework_async_macros::async_test]
async fn creating_and_deleting_a_scheme_adds_and_removes_only_its_own_totals() {
    let snapshot = zoning();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let created = ModelDiff::area_schemes("as-extra", Entry::Created(AreaScheme { name: "Extra".into(), measure: AreaMeasure::Gross, usages: vec!["Kitchen".into()], zones: Vec::new() }));
    let with = protocol::apply_diff(&created, &snapshot).expect("applies");
    assert_eq!(session.update(&with, &created).scheme_totals["as-extra"].spaces, 1);
    assert_eq!(computed(&session), [("scheme".to_string(), 1)]);
    let deleted = ModelDiff::area_schemes("as-extra", Entry::Deleted);
    let without = protocol::apply_diff(&deleted, &with).expect("applies");
    assert!(!session.update(&without, &deleted).scheme_totals.contains_key("as-extra"));
    assert_eq!(session.inference(), &infer(&snapshot), "the model is back where it started");
}

#[semio_framework_async_macros::async_test]
async fn clearing_the_last_member_of_a_zone_and_deleting_it_leaves_the_schemes_that_named_it_empty() {
    let snapshot = zoning();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let leave = ModelDiff::spaces("sp-up", Entry::Patched(SpacePatch { zone: Some(Assigned::new(None)), ..Default::default() }));
    let left = protocol::apply_diff(&leave, &snapshot).expect("applies");
    session.update(&left, &leave);
    let leave_kitchen = ModelDiff::spaces("sp-east", Entry::Patched(SpacePatch { zone: Some(Assigned::new(None)), ..Default::default() }));
    let emptied = protocol::apply_diff(&leave_kitchen, &left).expect("applies");
    assert_eq!(session.update(&emptied, &leave_kitchen).zone_totals["z-night"].spaces, 0);
    let delete = ModelDiff::zones("z-night", Entry::Deleted);
    let gone = protocol::apply_diff(&delete, &emptied).expect("applies");
    let inferred = session.update(&gone, &delete);
    assert!(!inferred.zone_totals.contains_key("z-night"));
    assert_eq!(inferred.scheme_totals["as-sleep"].spaces, 0, "a scheme that counts a deleted zone counts nothing");
    assert_eq!(session.inference(), &infer(&gone));
}

#[semio_framework_async_macros::async_test]
async fn the_totals_are_deterministic_and_empty_by_default() {
    let snapshot = zoning();
    let (first, second) = (infer(&snapshot), infer(&snapshot));
    assert_eq!((&first.zone_totals, &first.scheme_totals), (&second.zone_totals, &second.scheme_totals));
    let empty = infer(&ModelSnapshot::default());
    assert!(empty.zone_totals.is_empty() && empty.scheme_totals.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_scheme_over_an_empty_model_part_is_zero() {
    let mut snapshot = zoning();
    snapshot.area_schemes.insert("as-none".into(), AreaScheme { name: "None".into(), measure: AreaMeasure::Gross, usages: vec!["Workshop".into()], zones: Vec::new() });
    let totals = &infer(&snapshot).scheme_totals["as-none"];
    assert_eq!(totals, &SchemeTotals::default());
}
