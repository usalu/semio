//! 🏛️ Annex block tests: ring-stiffened shell under external pressure, corrugated web shear, orthotropic deck and purlin on the sheeting spring, against the independent values of `🔮️oracles/🏛️annex-blocks/🐍️.py`.

use crate::check_full_steel_structure;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus};

fn find<'a>(report: &'a CheckReport, id: &str) -> &'a CheckResult {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id}"))
}

fn close(a: f64, b: f64, rel: f64, what: &str) {
    assert!((a - b).abs() <= rel * b.abs().max(1e-12), "{what}: {a} vs {b}");
}

fn showcase() -> CheckReport {
    check_full_steel_structure(&En1993Snapshot::annex_showcase())
}

#[test]
fn the_ring_stiffened_shell_matches_the_ls3_and_the_ring_oracle() {
    let report = showcase();
    let ls3 = find(&report, "en1993.1-6.D.ext.ring-1");
    close(ls3.utilization, 1.1476474666530592, 1e-9, "p r/t over the LS3 hoop stress between the rings");
    assert_eq!(ls3.status, CheckStatus::Fail);
    assert_eq!(ls3.remedies.len(), 2);
    assert!(ls3.explanation.en.contains("von Mises collapse pressure is 0.765"));
    let ring = find(&report, "en1993.1-6.D.ring.ring-1");
    close(ring.utilization, 0.66958832985029, 1e-9, "3 p over the Bresse pressure of the ring with its effective shell strip");
    assert!(!ring.ndps.is_empty());
}

#[test]
fn a_thicker_wall_or_denser_rings_clear_the_hoop_buckling_failure() {
    let mut doc = En1993Snapshot::annex_showcase();
    let row = doc.silo_shells.iter_mut().find(|s| s.id == "ring-1").expect("ring row");
    row.thickness = 0.012;
    assert_eq!(find(&check_full_steel_structure(&doc), "en1993.1-6.D.ext.ring-1").status, CheckStatus::Pass);
    let row = doc.silo_shells.iter_mut().find(|s| s.id == "ring-1").expect("ring row");
    row.thickness = 0.008;
    row.ring_inertia = 1.0e-7;
    row.ring_area = 5.0e-4;
    row.ring_offset = 0.0;
    let report = check_full_steel_structure(&doc);
    let ring = find(&report, "en1993.1-6.D.ring.ring-1");
    assert_eq!(ring.status, CheckStatus::Fail);
    assert_eq!(ring.remedies.len(), 1);
}

#[test]
fn the_corrugated_web_shear_matches_the_orthotropic_oracle() {
    let report = showcase();
    let check = find(&report, "en1993.1-5.D.shear.corr-1");
    close(check.utilization, 0.6692890164920153, 1e-9, "V_Ed over V_b,Rd of the 2 mm web with 140/40/50 mm folds");
    assert_eq!(check.status, CheckStatus::Pass);
    let mut doc = En1993Snapshot::annex_showcase();
    doc.plated_panels.iter_mut().find(|p| p.id == "corr-1").expect("web row").shear_force = 60_000.0;
    let report = check_full_steel_structure(&doc);
    let failing = find(&report, "en1993.1-5.D.shear.corr-1");
    assert_eq!(failing.status, CheckStatus::Fail);
    assert_eq!(failing.remedies.len(), 2);
}

#[test]
fn the_orthotropic_deck_checks_minima_and_the_wheel_strip_fatigue() {
    let report = showcase();
    close(find(&report, "en1993.2.9.4.deck.t.deck-1").utilization, 12.0 / 14.0, 1e-9, "12 mm minimum under a 70 mm surfacing");
    close(find(&report, "en1993.2.9.4.trough.t.deck-1").utilization, 6.0 / 8.0, 1e-9, "6 mm minimum trough wall");
    let fatigue = find(&report, "en1993.2.9.5.deck.deck-1");
    close(fatigue.utilization, 0.7269886908861918, 1e-9, "damage equivalent range of the clamped strip over the detail category");
    let mut doc = En1993Snapshot::annex_showcase();
    let row = doc.bridge_fatigue.iter_mut().find(|b| b.id == "deck-1").expect("deck row");
    row.deck_thickness = 0.010;
    row.wheel_load = 120_000.0;
    let report = check_full_steel_structure(&doc);
    assert_eq!(find(&report, "en1993.2.9.4.deck.t.deck-1").status, CheckStatus::Fail);
    let fatigue = find(&report, "en1993.2.9.5.deck.deck-1");
    assert_eq!(fatigue.status, CheckStatus::Fail);
    assert_eq!(fatigue.remedies.len(), 3);
}

#[test]
fn the_purlin_free_flange_matches_the_finite_difference_oracle() {
    let report = showcase();
    let check = find(&report, "en1993.1-3.E.purlin.purlin-1");
    close(check.utilization, 0.8257493447749388, 1e-5, "M_y over M_c,Rd plus the free flange moment on the sheeting spring");
    let mut doc = En1993Snapshot::annex_showcase();
    doc.cold_formed_members.iter_mut().find(|c| c.id == "purlin-1").expect("purlin row").purlin_load = 4000.0;
    let report = check_full_steel_structure(&doc);
    let failing = find(&report, "en1993.1-3.E.purlin.purlin-1");
    assert_eq!(failing.status, CheckStatus::Fail);
    assert_eq!(failing.remedies.len(), 2);
}

#[test]
fn rows_without_annex_data_emit_no_annex_check() {
    for doc in [En1993Snapshot::compliant_heb240_frame(), En1993Snapshot::special_parts_showcase()] {
        for check in check_full_steel_structure(&doc).checks {
            assert!(!["en1993.1-6.D.", "en1993.1-5.D.", "en1993.1-5.A.", "en1993.1-5.C.", "en1993.2.9.4.", "en1993.2.9.5.deck.", "en1993.1-3.E."].iter().any(|p| check.id.starts_with(p)), "unexpected {}", check.id);
        }
    }
}

#[test]
fn every_annex_failure_carries_a_remedy_and_unverified_values_block_compliance() {
    let report = showcase();
    for check in report.checks.iter().filter(|c| ["en1993.1-6.D.", "en1993.1-5.D.", "en1993.1-5.A.", "en1993.1-5.C.", "en1993.2.9.4.", "en1993.2.9.5.deck.", "en1993.1-3.E."].iter().any(|p| c.id.starts_with(p))) {
        assert!(check.utilization.is_finite() && !check.explanation.en.is_empty() && !check.explanation.de.is_empty(), "{}", check.id);
        if check.status == CheckStatus::Fail {
            assert!(!check.remedies.is_empty(), "{}", check.id);
        }
    }
    assert!(!report.complies());
}

#[test]
fn the_smeared_stiffened_plate_matches_the_orthotropic_navier_oracle() {
    let report = showcase();
    let check = find(&report, "en1993.1-5.A.stiff-1");
    close(check.utilization, 0.5250093898739615, 1e-9, "sigma_Ed over rho f_y/gamma_M1 of the plate with three smeared stiffeners (Navier k = 20.451)");
    assert_eq!(check.status, CheckStatus::Pass);
    assert!(check.explanation.en.contains("3 smeared stiffeners") && check.explanation.en.contains("k_σ,p=20.45"));
    assert!(check.ndps.iter().any(|n| n.ends_with("v-376.a1")));
}

#[test]
fn a_wider_stiffener_spacing_fails_with_the_spacing_remedy_and_one_stiffener_uses_the_single_formula() {
    let mut doc = En1993Snapshot::annex_showcase();
    let row = doc.plated_panels.iter_mut().find(|p| p.id == "stiff-1").expect("stiffened row");
    row.compression_stress = 260.0e6;
    let report = check_full_steel_structure(&doc);
    let failing = find(&report, "en1993.1-5.A.stiff-1");
    assert_eq!(failing.status, CheckStatus::Fail);
    assert!(failing.remedies.len() >= 1);
    let row = doc.plated_panels.iter_mut().find(|p| p.id == "stiff-1").expect("stiffened row");
    row.stiffener_spacing = 0.75;
    row.compression_stress = 100.0e6;
    let single = find(&check_full_steel_structure(&doc), "en1993.1-5.A.stiff-1").clone();
    assert!(single.explanation.en.contains("one stiffener (A.2.2.1)"));
    row_spacing(&mut doc, 0.5);
    let two = find(&check_full_steel_structure(&doc), "en1993.1-5.A.stiff-1").clone();
    assert_eq!(two.status, CheckStatus::NotApplicable);
}

fn row_spacing(doc: &mut En1993Snapshot, spacing: f64) {
    doc.plated_panels.iter_mut().find(|p| p.id == "stiff-1").expect("stiffened row").stiffener_spacing = spacing;
}

#[test]
fn the_finite_element_load_factor_is_compared_with_the_annex_a_critical_stress() {
    let mut doc = En1993Snapshot::annex_showcase();
    doc.plated_panels.iter_mut().find(|p| p.id == "stiff-1").expect("stiffened row").alpha_cr = 1.8;
    let report = check_full_steel_structure(&doc);
    let cross = find(&report, "en1993.1-5.C.stiff-1");
    assert_eq!(cross.status, CheckStatus::Info);
    close(cross.computed.value, 1.8 / 1.7251597507708993, 1e-9, "alpha_cr of the finite element model over sigma_cr,p / sigma_Ed");
    assert!(report.checks.iter().all(|c| !c.id.starts_with("en1993.1-5.C.") || c.id.ends_with("stiff-1")));
}
