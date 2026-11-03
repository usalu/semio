//! 🧪️ Fatigue curves against the independent oracle (`🧰️scripts/en1994-fatigue-oracle.py`: endurance by scipy root finding on the resistance curve).

use super::*;

const GAMMA: f64 = 1.15;

fn stud() -> Curve {
    Curve { delta_c_pa: 90e6, n_ref: 2e6, m: 8.0, gamma: GAMMA }
}

fn steel() -> Curve {
    Curve { delta_c_pa: 80e6, n_ref: 2e6, m: 3.0, gamma: GAMMA }
}

fn rel(a: f64, b: f64) -> f64 {
    (a - b).abs() / b.abs().max(1e-30)
}

#[test]
fn resistance_endurance_and_damage_follow_the_root_finding_oracle() {
    for (curve, delta, n, resistance, endurance, damage, utilisation) in [
        (stud(), 40e6, 5e6, 69791441.41858667, 429444868.56570697, 0.011642938048600709, 0.5731361781180708),
        (steel(), 50e6, 1e7, 40681985.92296162, 5386372.976082852, 1.8565368652343732, 1.2290452116738757),
        (rebar_curve(), 100e6, 2e6, 130830122.38842149, 22459481.43487569, 0.08904925101673745, 0.7643499690622474),
    ] {
        assert!(rel(curve.resistance_pa(n), resistance) < 1e-12);
        assert!(rel(curve.endurance(delta), endurance) < 1e-9);
        assert!(rel(curve.damage(delta, n), damage) < 1e-9);
        assert!(rel(curve.utilisation(delta, n), utilisation) < 1e-12);
    }
}

#[test]
fn damage_is_the_utilisation_to_the_power_of_the_slope() {
    for (curve, delta, n) in [(stud(), 40e6, 5e6), (steel(), 50e6, 1e7), (rebar_curve(), 100e6, 2e6)] {
        assert!(rel(curve.damage(delta, n), curve.utilisation(delta, n).powf(curve.m)) < 1e-9);
    }
}

#[test]
fn the_resistance_at_the_reference_count_is_the_category_over_the_partial_factor() {
    assert!(rel(stud().resistance_pa(2e6), 90e6 / GAMMA) < 1e-12);
    assert!(rel(rebar_curve().resistance_pa(1e6), 162.5e6 / 1.15) < 1e-12);
    assert!(stud().resistance_pa(1e7) < stud().resistance_pa(1e6));
}

#[test]
fn stud_rows_scale_the_range_and_the_governing_row_is_the_support_row() {
    let rows = stud_rows(&stud(), 40e6, 5e6, &[(0.5, 1.0), (1.5, 0.8), (2.5, 0.5), (3.5, 0.2)]);
    for (row, damage) in rows.iter().zip([0.011642938048600709, 0.001953360865159924, 4.5480226752346654e-05, 2.9805921404417846e-08]) {
        assert!(rel(row.damage, damage) < 1e-9);
        assert!(rel(row.delta_pa, 40e6 * row.share) < 1e-12);
    }
    assert_eq!(governing_row(&rows), Some(0));
    assert_eq!(governing_row(&[]), None);
}
