use super::*;
use crate::{Axis, ModelInference, Point2, SpaceBoundary};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const CLEAN: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/⚠️diagnostics/🏡️clean/📸️snapshot/🔣️.json");
const DEFECTS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/⚠️diagnostics/💥️defects/📸️snapshot/🔣️.json");

fn clean() -> ModelSnapshot {
    from_json_str(CLEAN, JsonMemberPolicy::Reject).expect("clean house decodes")
}

fn defects() -> ModelSnapshot {
    from_json_str(DEFECTS, JsonMemberPolicy::Reject).expect("defects decode")
}

fn rows(found: &[Diagnostic]) -> Vec<(String, Vec<String>)> {
    found.iter().map(|row| (row.code.slug().to_string(), row.elements.clone())).collect()
}

fn has(found: &[Diagnostic], code: DiagnosticCode, elements: &[&str]) -> bool {
    found.iter().any(|row| row.code == code && row.elements == elements)
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

#[semio_framework_async_macros::async_test]
async fn a_clean_model_has_no_findings() {
    assert_eq!(rows(&compute_diagnostics(&clean())), Vec::<(String, Vec<String>)>::new());
}

#[semio_framework_async_macros::async_test]
async fn every_defect_of_the_defect_model_is_reported_once() {
    let found = compute_diagnostics(&defects());
    use DiagnosticCode::*;
    for (code, elements) in [
        (ClashWallColumn, vec!["c-1", "w-into-column"]),
        (ClashWallColumn, vec!["c-2", "w-into-column"]),
        (ClashColumnColumn, vec!["c-1", "c-2"]),
        (ClashStairWall, vec!["s-comfort", "w-into-column"]),
        (ClashStairWall, vec!["s-comfort", "w-part"]),
        (RefWallType, vec!["w-missing-type"]),
        (RefOpeningHost, vec!["o-orphan"]),
        (OpeningOutsideHost, vec!["o-outside"]),
        (DegenerateAxis, vec!["w-zero"]),
        (StoreyLevelDuplicate, vec!["st-dup", "st-first"]),
        (StoreyLevelGap, vec!["st-dup", "st-far"]),
        (StairComfort, vec!["s-comfort"]),
        (SpaceDuplicateNumber, vec!["sp-living", "sp-twin"]),
    ] {
        assert_eq!(found.iter().filter(|row| row.code == code && row.elements == elements).count(), 1, "{code:?} {elements:?} in {:?}", rows(&found));
    }
    assert_eq!(found.len(), 13, "nothing else is reported: {:?}", rows(&found));
}

#[semio_framework_async_macros::async_test]
async fn findings_carry_severity_key_storey_and_the_missing_id() {
    let found = compute_diagnostics(&defects());
    let wall_type = found.iter().find(|row| row.code == DiagnosticCode::RefWallType).expect("finding");
    assert_eq!((wall_type.severity, wall_type.message_key.as_str(), wall_type.storey.as_deref(), wall_type.missing.as_slice()), (Severity::Error, "bim.diagnostic.reference.wall-type", Some("st-ground"), &["wt-missing".to_string()][..]));
    let clash = found.iter().find(|row| row.code == DiagnosticCode::ClashColumnColumn).expect("finding");
    assert_eq!(clash.severity, Severity::Error);
    assert!(close(clash.values["overlap_area"], 0.2 * 0.3) && close(clash.values["overlap_height"], 3.0) && close(clash.values["overlap_volume"], 0.2 * 0.3 * 3.0));
    assert!(found.windows(2).all(|pair| pair[0].severity >= pair[1].severity), "most severe first");
}

#[semio_framework_async_macros::async_test]
async fn moving_a_wall_into_a_column_creates_a_clash_and_moving_it_back_removes_it() {
    let moved_to = |x: f64| {
        let mut snapshot = clean();
        snapshot.walls.get_mut("w-part").expect("wall").axis = Axis::Line { start: Point2 { x, y: 0.0 }, end: Point2 { x, y: 3.0 } };
        compute_diagnostics(&snapshot)
    };
    assert!(compute_diagnostics(&clean()).is_empty());
    let clashing = moved_to(6.0);
    assert_eq!(rows(&clashing), vec![("clash.wall-column".to_string(), vec!["c-1".to_string(), "w-part".to_string()])]);
    let row = &clashing[0];
    assert!(close(row.values["overlap_area"], 0.15 * 0.2) && close(row.values["overlap_volume"], 0.15 * 0.2 * 3.0), "the wall end 0.2 deep in the column");
    assert!(moved_to(4.0).is_empty(), "back where it was, the clash is gone");
}

#[semio_framework_async_macros::async_test]
async fn walls_that_cross_or_join_do_not_clash_but_overlapping_walls_do() {
    let mut snapshot = clean();
    let line = |x0: f64, y0: f64, x1: f64, y1: f64| Axis::Line { start: Point2 { x: x0, y: y0 }, end: Point2 { x: x1, y: y1 } };
    snapshot.walls.get_mut("w-part").expect("wall").axis = line(2.0, 1.0, 2.0, 5.0);
    snapshot.walls.get_mut("w-part").expect("wall").location = crate::LocationLine::Center;
    let mut crossing = snapshot.clone();
    crossing.walls.insert("w-cross".into(), crossing.walls["w-part"].clone());
    crossing.walls.get_mut("w-cross").expect("wall").axis = line(1.0, 3.0, 4.0, 3.0);
    assert!(compute_diagnostics(&crossing).iter().all(|row| !row.code.slug().starts_with("clash.wall-wall")), "an X join is a join");
    let mut overlapping = snapshot.clone();
    overlapping.walls.insert("w-twin".into(), overlapping.walls["w-part"].clone());
    overlapping.walls.get_mut("w-twin").expect("wall").axis = line(2.0, 2.0, 2.0, 5.5);
    let found = compute_diagnostics(&overlapping);
    assert!(has(&found, DiagnosticCode::ClashWallWall, &["w-part", "w-twin"]), "{:?}", rows(&found));
    assert!(close(found.iter().find(|row| row.code == DiagnosticCode::ClashWallWall).expect("clash").values["overlap_area"], 3.0 * 0.15));
}

#[semio_framework_async_macros::async_test]
async fn a_beam_that_ends_on_a_column_rests_on_it_and_one_that_passes_through_clashes() {
    let mut snapshot = clean();
    snapshot.beams.get_mut("b-1").expect("beam").axis = crate::Axis::Line { start: Point2 { x: 6.0, y: 1.0 }, end: Point2 { x: 6.0, y: 3.0 } };
    assert!(!compute_diagnostics(&snapshot).iter().any(|row| row.code == DiagnosticCode::ClashBeamColumn), "the beam ends in the column");
    snapshot.beams.get_mut("b-1").expect("beam").axis = crate::Axis::Line { start: Point2 { x: 6.0, y: 1.0 }, end: Point2 { x: 6.0, y: 5.0 } };
    let found = compute_diagnostics(&snapshot);
    assert!(has(&found, DiagnosticCode::ClashBeamColumn, &["b-1", "c-1"]), "{:?}", rows(&found));
}

#[semio_framework_async_macros::async_test]
async fn a_beam_raised_into_the_slab_above_clashes_with_it() {
    let mut snapshot = clean();
    snapshot.beams.get_mut("b-1").expect("beam").top_offset = 0.0;
    let found = compute_diagnostics(&snapshot);
    assert!(has(&found, DiagnosticCode::ClashBeamSlab, &["b-1", "sl-first"]), "{:?}", rows(&found));
    assert!(close(found.iter().find(|row| row.code == DiagnosticCode::ClashBeamSlab).expect("clash").values["overlap_height"], 0.22));
}

#[semio_framework_async_macros::async_test]
async fn the_severity_and_the_texts_come_from_one_table() {
    let mut slugs = BTreeSet::new();
    let names = |text: &str| {
        let mut found: Vec<String> = text.split('{').skip(1).filter_map(|rest| rest.split('}').next()).map(str::to_string).collect();
        found.sort();
        found.dedup();
        found
    };
    for code in DiagnosticCode::ALL {
        let row = row_of(*code);
        assert!(!row.en.is_empty() && !row.de.is_empty(), "{code:?} has both texts");
        assert_eq!(names(row.en), names(row.de), "{code:?} uses the same placeholders in both languages");
        assert!(slugs.insert(row.slug), "unique slug {}", row.slug);
        assert_eq!(code.message_key(), format!("bim.diagnostic.{}", row.slug));
    }
    assert!(DiagnosticCode::ALL.len() >= 60);
    assert_eq!(DiagnosticCode::ClashColumnColumn.severity(), Severity::Error);
    assert_eq!(DiagnosticCode::StoreyNoDatum.severity(), Severity::Info);
}

#[semio_framework_async_macros::async_test]
async fn findings_render_in_english_and_german_and_in_no_other_locale() {
    let finding = Diagnostic::new(DiagnosticCode::ClashColumnColumn, &["c-1", "c-2"]).with("overlap_volume", 0.18);
    assert_eq!(finding.text("en").as_deref(), Some("Columns c-1, c-2 overlap by 0.180 m3."));
    assert_eq!(finding.text("de").as_deref(), Some("Stützen c-1, c-2 überschneiden sich um 0.180 m3."));
    assert_eq!(finding.text("fr"), None);
    let missing = Diagnostic::new(DiagnosticCode::RefWallType, &["w-1"]).lacking("wt-x");
    assert_eq!(missing.text("en").as_deref(), Some("Wall w-1 uses the missing wall type wt-x."));
    assert_eq!(LOCALES, ["en", "de"]);
}

#[semio_framework_async_macros::async_test]
async fn storeys_without_a_datum_and_with_non_positive_heights_are_reported() {
    let mut snapshot = clean();
    snapshot.storeys.get_mut("st-ground").expect("storey").level = -1;
    snapshot.storeys.get_mut("st-first").expect("storey").height = 0.0;
    let found = compute_diagnostics(&snapshot);
    assert!(has(&found, DiagnosticCode::StoreyNoDatum, &["bldg-1"]));
    assert!(has(&found, DiagnosticCode::DegenerateStorey, &["st-first"]));
    assert!(has(&found, DiagnosticCode::StoreyLevelGap, &["st-ground", "st-first"]), "levels -1 and 1 skip 0: {:?}", rows(&found));
}

#[semio_framework_async_macros::async_test]
async fn degenerate_loops_profiles_and_paths_are_reported() {
    let mut snapshot = clean();
    snapshot.slabs.get_mut("sl-ground").expect("slab").boundary.truncate(2);
    snapshot.slabs.get_mut("sl-first").expect("slab").boundary = [(0.0, 0.0), (6.0, 4.0), (6.0, 0.0), (0.0, 3.0)].iter().map(|(x, y)| crate::Vertex { point: Point2 { x: *x, y: *y }, bulge: 0.0 }).collect();
    snapshot.railings.get_mut("r-1").expect("railing").path.truncate(1);
    snapshot.beam_types.get_mut("bt-30x50").expect("type").profile = crate::Profile::Rectangle { width: 0.0, depth: 0.5 };
    snapshot.columns.get_mut("c-1").expect("column").position = Point2 { x: f64::NAN, y: 0.0 };
    let found = compute_diagnostics(&snapshot);
    assert!(has(&found, DiagnosticCode::DegenerateLoop, &["sl-ground"]));
    assert!(has(&found, DiagnosticCode::SelfIntersectingLoop, &["sl-first"]), "{:?}", rows(&found));
    assert!(has(&found, DiagnosticCode::DegeneratePath, &["r-1"]));
    assert!(has(&found, DiagnosticCode::DegenerateProfile, &["b-1"]));
    assert!(has(&found, DiagnosticCode::NonFinite, &["c-1"]));
}

#[semio_framework_async_macros::async_test]
async fn a_space_whose_walls_do_not_close_is_not_enclosed_and_a_seed_in_a_wall_is_reported() {
    let mut open = clean();
    open.walls.remove("w-west");
    let found = compute_diagnostics(&open);
    assert!(has(&found, DiagnosticCode::SpaceNotEnclosed, &["sp-hall"]), "{:?}", rows(&found));
    assert!(!has(&found, DiagnosticCode::SpaceNotEnclosed, &["sp-living"]), "an explicit outline needs no walls");
    let mut buried = clean();
    buried.spaces.get_mut("sp-hall").expect("space").boundary = SpaceBoundary::Bounded { seed: Point2 { x: 4.0, y: 1.0 } };
    let found = compute_diagnostics(&buried);
    assert!(has(&found, DiagnosticCode::SpaceSeedInWall, &["sp-hall"]), "{:?}", rows(&found));
}

#[semio_framework_async_macros::async_test]
async fn dangling_references_orphans_and_duplicate_ids_are_reported() {
    let mut snapshot = clean();
    snapshot.buildings.get_mut("bldg-1").expect("building").site = "site-x".into();
    snapshot.columns.get_mut("c-1").expect("column").column_type = "ct-x".into();
    snapshot.walls.get_mut("w-east").expect("wall").top = crate::TopConstraint::Storey { storey: "st-x".into(), offset: 0.0 };
    snapshot.walls.get_mut("w-west").expect("wall").storey = "st-y".into();
    snapshot.wall_types.get_mut("wt-150").expect("type").layers[0].material = "m-x".into();
    snapshot.properties.insert("ghost".into(), Default::default());
    let beam = snapshot.beams["b-1"].clone();
    snapshot.columns.insert("b-1".into(), snapshot.columns["c-1"].clone());
    snapshot.beams.insert("b-2".into(), beam);
    let found = compute_diagnostics(&snapshot);
    use DiagnosticCode::*;
    for (code, elements) in [
        (RefBuildingSite, vec!["bldg-1"]),
        (RefColumnType, vec!["c-1"]),
        (RefTopStorey, vec!["w-east"]),
        (RefElementStorey, vec!["w-west"]),
        (RefLayerMaterial, vec!["wt-150"]),
        (RefPropertyElement, vec!["ghost"]),
        (DuplicateId, vec!["b-1"]),
    ] {
        assert!(has(&found, code, &elements), "{code:?} {elements:?} in {:?}", rows(&found));
    }
}


#[semio_framework_async_macros::async_test]
async fn diagnostics_determinism_default_and_gating_laws() {
    use crate::{Entry, ModelDiff, StoreyPatch};
    let snapshot = defects();
    assert_eq!(ModelInference::infer(&snapshot).expect("infers").diagnostics, ModelInference::infer(&snapshot).expect("infers").diagnostics);
    assert!(ModelInference::infer(&ModelSnapshot::default()).expect("infers").diagnostics.is_empty());
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).diagnostics.clone();
    let untouched = session.update(&snapshot, &ModelDiff::default()).diagnostics.clone();
    assert_eq!(first, untouched, "a diff outside the reads serves the stored result");
    assert!(session.report().gated);
    let height = ModelDiff::storeys("st-far", Entry::Patched(StoreyPatch { height: Some(0.0), ..Default::default() }));
    let edited = protocol::apply_diff(&height, &snapshot).expect("applies");
    let recomputed = session.update(&edited, &height).diagnostics.clone();
    let on = |found: &[Diagnostic], storey: &str| found.iter().filter(|row| row.storey.as_deref() == Some(storey)).cloned().collect::<Vec<_>>();
    assert_eq!(on(&first, "st-ground"), on(&recomputed, "st-ground"), "an edit of another storey leaves the findings of this storey");
    assert!(recomputed.iter().any(|row| row.code == DiagnosticCode::DegenerateStorey && row.storey.as_deref() == Some("st-far")));
}

const RAMPS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🛝️ramp-runs/🏞️ramps/📸️snapshot/🔣️.json");

fn ramps() -> ModelSnapshot {
    from_json_str(RAMPS, JsonMemberPolicy::Reject).expect("ramps decode")
}

fn ramp_rows(found: &[Diagnostic], code: DiagnosticCode) -> Vec<String> {
    let mut ids: Vec<String> = found.iter().filter(|row| row.code == code).flat_map(|row| row.elements.clone()).collect();
    ids.sort();
    ids
}

#[semio_framework_async_macros::async_test]
async fn a_ramp_steeper_than_its_limit_is_reported_with_its_slope_rise_and_run() {
    let found = compute_diagnostics(&ramps());
    assert_eq!(ramp_rows(&found, DiagnosticCode::RampSlope), vec!["r-bent", "r-steep", "r-to-first-short"], "only the ramps above their limit: {:?}", rows(&found));
    let steep = found.iter().find(|row| row.code == DiagnosticCode::RampSlope && row.elements == ["r-steep"]).expect("finding");
    assert_eq!((steep.severity, steep.storey.as_deref(), steep.message_key.as_str()), (Severity::Error, Some("st-ground"), "bim.diagnostic.ramp.slope"));
    assert!(close(steep.values["slope_percent"], 10.0) && close(steep.values["limit_percent"], 100.0 / 12.0) && close(steep.values["rise"], 0.7) && close(steep.values["run"], 7.0));
    assert!(steep.text("en").expect("en").starts_with("Ramp r-steep climbs at "));
    assert!(steep.text("de").expect("de").starts_with("Rampe r-steep steigt mit "));
    assert!(!ramp_rows(&found, DiagnosticCode::RampSlope).iter().any(|id| id == "r-at-limit"), "a ramp exactly at its limit is within it");
}

#[semio_framework_async_macros::async_test]
async fn a_rise_without_a_sloped_run_and_a_ramp_without_a_path_are_reported_apart() {
    let found = compute_diagnostics(&ramps());
    assert_eq!(ramp_rows(&found, DiagnosticCode::RampNoRun), vec!["r-no-run"]);
    let row = found.iter().find(|row| row.code == DiagnosticCode::RampNoRun).expect("finding");
    assert_eq!((row.severity, row.message_key.as_str()), (Severity::Error, "bim.diagnostic.ramp.no-run"));
    assert!(close(row.values["rise"], 0.3));
    assert!(row.text("de").expect("de").starts_with("Rampe r-no-run hat einen Höhenunterschied"));
    assert!(has(&found, DiagnosticCode::DegenerateAxis, &["r-degenerate"]), "a path of no length is degenerate, not a missing run");
    assert!(!has(&found, DiagnosticCode::RampNoRun, &["r-degenerate"]) && !has(&found, DiagnosticCode::RampSlope, &["r-degenerate"]));
    assert!(!has(&found, DiagnosticCode::RampNoRun, &["r-merged"]), "a flat ramp that is all landing climbs nowhere and needs no run");
}

#[semio_framework_async_macros::async_test]
async fn lengthening_a_ramp_or_raising_its_limit_clears_the_slope_finding() {
    let mut longer = ramps();
    longer.ramps.get_mut("r-steep").expect("ramp").path[1].point.x = 20.0;
    assert!(!has(&compute_diagnostics(&longer), DiagnosticCode::RampSlope, &["r-steep"]), "a longer path flattens the slope");
    let mut relaxed = ramps();
    relaxed.ramps.get_mut("r-steep").expect("ramp").max_slope = 0.1;
    assert!(!has(&compute_diagnostics(&relaxed), DiagnosticCode::RampSlope, &["r-steep"]), "the slope limit is authored");
    let mut shorter = ramps();
    shorter.ramps.get_mut("r-straight").expect("ramp").path[1].point.x = 4.0;
    let found = compute_diagnostics(&shorter);
    assert!(has(&found, DiagnosticCode::RampSlope, &["r-straight"]), "a shorter path steepens it: {:?}", rows(&found));
}

#[semio_framework_async_macros::async_test]
async fn a_ramp_with_a_non_finite_dimension_is_reported_once_as_non_finite() {
    let mut snapshot = ramps();
    snapshot.ramps.get_mut("r-steep").expect("ramp").width = f64::NAN;
    let found = compute_diagnostics(&snapshot);
    assert!(has(&found, DiagnosticCode::NonFinite, &["r-steep"]));
    assert!(!has(&found, DiagnosticCode::RampSlope, &["r-steep"]), "the finite checks stop at the non-finite one");
}
