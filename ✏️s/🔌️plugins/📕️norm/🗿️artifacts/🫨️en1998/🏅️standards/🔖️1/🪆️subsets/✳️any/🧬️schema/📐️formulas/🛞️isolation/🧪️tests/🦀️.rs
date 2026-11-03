//! 🧪 Worked examples of the isolation kernel; the numbers come from `🧰️scripts/en1998-wave3d/1-hand-values.py` (scipy brentq on the force balance, shoelace area of the loop) and the hand calculation of K_1, F and E_D.

use super::*;

fn near(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() <= tolerance * b.abs().max(1.0e-12), "{a} vs {b}");
}

fn bilinear() -> Bilinear {
    Bilinear { strength_n: 2.0e5, post_yield_n_per_m: 6.0e6, yield_displacement_m: 0.01 }
}

fn site() -> Spectrum {
    Spectrum { a_g: 2.5, s: 1.2, tb: 0.15, tc: 0.5, td: 2.0 }
}

#[test]
fn the_backbone_is_elastic_to_the_yield_displacement_and_bilinear_beyond() {
    let b = bilinear();
    near(b.elastic_stiffness(), 2.6e7, 1e-12);
    near(b.force_n(0.01), 2.6e5, 1e-12);
    near(b.force_n(0.075), 2.0e5 + 6.0e6 * 0.075, 1e-12);
    near(b.force_n(-0.075), b.force_n(0.075), 1e-12);
    near(b.dissipated_energy_nm(0.075), 4.0 * 2.0e5 * 0.065, 1e-12);
    assert_eq!(b.dissipated_energy_nm(0.004), 0.0);
}

#[test]
fn the_design_displacement_balances_the_spectral_force_with_the_damping_of_the_loop() {
    let state = bilinear().design(3.0e5, &site()).expect("equilibrium");
    near(state.displacement_m, 0.07485001860065095, 1e-8);
    near(state.effective_stiffness_n_per_m, 8672010.024033055, 1e-8);
    near(state.period_s, 1.1686396311770098, 1e-8);
    near(state.damping, 0.1699482227299225, 1e-8);
    near(state.eta, 0.6742792133526614, 1e-8);
    near(state.force_n, 649100.1116039057, 1e-8);
    near(state.force_n, 3.0e5 * site().elastic(state.period_s, state.eta), 1e-8);
}

#[test]
fn a_stiff_low_damping_system_has_a_short_period_and_eta_above_one() {
    let stiff = Bilinear { strength_n: 1.0e5, post_yield_n_per_m: 1.5e7, yield_displacement_m: 0.005 };
    let state = stiff.design(3.0e5, &site()).expect("equilibrium");
    near(state.displacement_m, 0.08371675112081757, 1e-8);
    near(state.period_s, 0.8551782941243092, 1e-8);
    near(state.damping, 0.044152459344244784, 1e-8);
    near(state.eta, 1.0305858272083401, 1e-8);
}

#[test]
fn the_hysteresis_loop_encloses_the_dissipated_energy() {
    let b = bilinear();
    let points = b.loop_points(0.075);
    let area = 0.5 * points.windows(2).map(|w| w[0].0 * w[1].1 - w[1].0 * w[0].1).sum::<f64>().abs();
    near(area, b.dissipated_energy_nm(0.075), 1e-12);
    assert_eq!(points.first(), points.last());
}

#[test]
fn parameters_that_are_no_loop_or_a_missing_mass_give_no_design_state() {
    assert!(Bilinear { strength_n: 2.0e5, post_yield_n_per_m: 0.0, yield_displacement_m: 0.01 }.design(3.0e5, &site()).is_none());
    assert!(Bilinear { strength_n: -1.0, post_yield_n_per_m: 6.0e6, yield_displacement_m: 0.01 }.design(3.0e5, &site()).is_none());
    assert!(bilinear().design(0.0, &site()).is_none());
}

#[test]
fn the_effective_period_follows_the_mass_and_the_secant_stiffness() {
    near(effective_period_s(3.0e5, 8672010.024033055), 1.1686396311770098, 1e-12);
}
