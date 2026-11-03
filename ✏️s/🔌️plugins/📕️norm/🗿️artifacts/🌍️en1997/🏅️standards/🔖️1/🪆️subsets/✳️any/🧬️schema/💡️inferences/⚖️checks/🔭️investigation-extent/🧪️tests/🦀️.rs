use super::*;
use semio_s_artifact_norm_en1997_model::{compliant_demo, InvestigationKind};
use semio_s_plugin_norm_compliance::RemedyBound;

fn point(id: &str, x: f64, y: f64, depth: f64) -> Investigation {
    Investigation { id: id.into(), position_x: x, position_y: y, ground_level: 0.0, depth, kind: InvestigationKind::Borehole, readings: Vec::new() }
}

fn extent(points: Vec<Investigation>, gk: u8) -> CheckReport {
    let mut doc = compliant_demo();
    doc.geotechnical_category = gk;
    doc.investigations = points;
    let mut report = CheckReport::default();
    push_investigation_extent(&mut report, &doc);
    report
}

fn find<'a>(report: &'a CheckReport, id: &str) -> &'a CheckResult {
    report.checks.iter().find(|check| check.id == id).unwrap_or_else(|| panic!("no check {id}"))
}

/// 📐️ Hand values of Annex B.3: embedment 1.5 m + max(6 m, 2·2.5 m) = 7.5 m; embedment 1 m + max(6 m, 2·4 m) = 9 m; pile 14 m + max(5 m, 3·0.6 m) = 19 m; pile 10 m + max(5 m, 3·2 m) = 16 m.
#[test]
fn the_required_depths_are_the_embedment_or_length_plus_the_annex_b3_depth_below_the_base() {
    let doc = compliant_demo();
    let footing = SpreadFoundation { width: 2.5, length: 2.5, embedment: 1.5, ..doc.footings[0].clone() };
    assert!((footing_required_depth_m(&footing, 2) - 7.5).abs() < 1e-12);
    let wide = SpreadFoundation { width: 4.0, length: 3.0, embedment: 1.0, ..footing };
    assert!((footing_required_depth_m(&wide, 2) - 9.0).abs() < 1e-12);
    let pile = Pile { diameter: 0.6, length: 14.0, ..doc.piles[0].clone() };
    assert!((pile_required_depth_m(&pile) - 19.0).abs() < 1e-12);
    let thick = Pile { diameter: 2.0, length: 10.0, ..pile };
    assert!((pile_required_depth_m(&thick) - 16.0).abs() < 1e-12);
}

/// 📏️ Points at (0, 0), (30, 0) and (30, 40): the nearest neighbours are 30, 30 and 40 m, so the widest gap is the third point to the second.
#[test]
fn the_widest_gap_is_the_largest_nearest_neighbour_distance() {
    let points = vec![point("a", 0.0, 0.0, 20.0), point("b", 30.0, 0.0, 20.0), point("c", 30.0, 40.0, 20.0)];
    let distances: Vec<f64> = nearest_spacings(&points).into_iter().flatten().map(|(_, d)| d).collect();
    assert_eq!(distances, vec![30.0, 30.0, 40.0]);
    assert_eq!(widest_gap(&points), Some((2, 1, 40.0)));
    assert_eq!(widest_gap(&points[..1]), None);
}

/// 📏️ GK2 allows 40 m: a gap of 40 m is utilisation 1 and passes, 50 m fails with the advice to add a point at the midpoint; GK3 allows 25 m, so 40 m fails there (limits are the unverified registry rows).
#[test]
fn the_spacing_check_follows_the_category_limit_and_advises_a_point_at_the_midpoint() {
    let ok = extent(vec![point("a", 0.0, 0.0, 20.0), point("b", 30.0, 0.0, 20.0), point("c", 30.0, 40.0, 20.0)], 2);
    let spacing = find(&ok, "en1997.gi-01.spacing");
    assert_eq!(spacing.status, CheckStatus::Pass);
    assert!((spacing.utilization - 1.0).abs() < 1e-12 && spacing.subject.entity_id == "c");
    assert!(spacing.ndps.iter().any(|id| id == "ndp.en1997.2.4.2.spacing.gc12.max"));
    let wide = extent(vec![point("a", 0.0, 0.0, 20.0), point("b", 30.0, 0.0, 20.0), point("c", 30.0, 50.0, 20.0)], 2);
    let spacing = find(&wide, "en1997.gi-01.spacing");
    assert_eq!(spacing.status, CheckStatus::Fail);
    assert!((spacing.utilization - 1.25).abs() < 1e-12);
    assert!(!spacing.remedies.is_empty() && spacing.remedies.iter().all(|remedy| !remedy.applicable && remedy.action.en.contains("x = 30.0 m, y = 25.0 m") && !remedy.action.de.is_empty()));
    let category_three = extent(vec![point("a", 0.0, 0.0, 20.0), point("b", 30.0, 0.0, 20.0), point("c", 30.0, 40.0, 20.0)], 3);
    assert!((find(&category_three, "en1997.gi-01.spacing").utilization - 1.6).abs() < 1e-12);
    let single = extent(vec![point("a", 0.0, 0.0, 20.0)], 2);
    assert_eq!(find(&single, "en1997.gi-01.spacing").status, CheckStatus::NotApplicable);
}

#[test]
fn a_site_without_investigation_points_is_a_warning_with_a_notice_and_a_site_with_points_passes() {
    let none = extent(Vec::new(), 2);
    let points = find(&none, "en1997.gi-01.points");
    assert_eq!(points.status, CheckStatus::Warning);
    assert!(!points.remedies.is_empty() && points.remedies.iter().all(|remedy| !remedy.applicable));
    let some = extent(vec![point("a", 0.0, 0.0, 20.0)], 2);
    assert_eq!(find(&some, "en1997.gi-01.points").status, CheckStatus::Pass);
}

/// 🏗️ The demo pile P1 (D = 0.6 m, L = 14 m) needs 19 m: an investigation of 18 m fails with the remedy to deepen the deepest point to 19 m, 20 m passes.
#[test]
fn the_pile_depth_check_asks_for_the_depth_below_the_pile_base_and_deepens_the_deepest_point() {
    let short = extent(vec![point("bh", 0.0, 0.0, 18.0)], 2);
    let check = find(&short, "en1997.gi-01.pile.pile-P1");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!((check.utilization - 19.0 / 18.0).abs() < 1e-12);
    let remedy = &check.remedies[0];
    assert_eq!((remedy.bound, remedy.required.value, remedy.target.path.as_str()), (RemedyBound::AtLeast, 19.0, "investigations[id=bh].depth"));
    let deep = extent(vec![point("bh", 0.0, 0.0, 20.0)], 2);
    assert_eq!(find(&deep, "en1997.gi-01.pile.pile-P1").status, CheckStatus::Pass);
    assert_eq!(find(&deep, "en1997.gi-01.pile.pile-P1").subject.entity_id, "pile-P1");
}

/// 🔩️ An entered N_kt of 17 lies in the range 15…20; 25 and 12 are warnings that set the nearest end; no entered value gives no check.
#[test]
fn an_entered_cone_factor_outside_the_plausible_range_is_a_warning_with_the_nearest_end() {
    let layer = |nkt: f64| SoilLayer { cpt_nkt: nkt, ..compliant_demo().layers[0].clone() };
    assert!(cone_factor_check(&layer(0.0), AnnexChoice::De).is_none());
    let inside = cone_factor_check(&layer(17.0), AnnexChoice::De).unwrap();
    assert_eq!((inside.status, inside.remedies.len()), (CheckStatus::Info, 0));
    for (value, nearest) in [(25.0, 20.0), (12.0, 15.0)] {
        let check = cone_factor_check(&layer(value), AnnexChoice::De).unwrap();
        assert_eq!(check.status, CheckStatus::Warning);
        assert_eq!((check.remedies[0].bound, check.remedies[0].required.value), (RemedyBound::Exactly, nearest));
        assert!(check.remedies[0].target.path.ends_with(".cptNkt") && check.ndps.iter().any(|id| id == "ndp.en1997.2.cpt-nkt-range.max"));
    }
}
