//! 🧪️ Miner spectra against the independent numpy implementation of the bilinear S–N curve (`🧰️scripts/en1999-wave4/miner_ref.py`; Δσ_C = 71 MPa, m1 = 3.4, m2 = 5.4, γ_Ff = γ_Mf = 1).

use super::*;

const BLOCKS: [(f64, f64); 4] = [(60.0e6, 2.0e5), (40.0e6, 1.0e6), (25.0e6, 5.0e6), (12.0e6, 2.0e7)];

fn close(a: f64, b: f64) { assert!((a / b - 1.0).abs() < 1e-9, "{a} against {b}"); }

#[test]
fn the_miner_sum_reproduces_the_numpy_blocks_and_ignores_ranges_below_the_cut_off() {
    close(spectrum_damage(&BLOCKS, 71.0e6, 3.4, 5.4, 1.0, 1.0), 0.09509099032433865);
    close(damage(60.0e6, 2.0e5, 71.0e6, 3.4, 5.4, 1.0, 1.0), 0.05642036165499026);
    assert_eq!(damage(25.0e6, 5.0e6, 71.0e6, 3.4, 5.4, 1.0, 1.0), 0.0);
}

#[test]
fn the_knee_and_the_cut_off_of_the_s_n_curve_match_the_reference() {
    close(delta_sigma_d(71.0e6, 3.4), 54.2271986648574e6);
    close(delta_sigma_l(71.0e6, 3.4, 5.4), 31.13763969961171e6);
}

#[test]
fn the_equivalent_range_is_the_cycle_weighted_power_mean() {
    close(equivalent_range(&BLOCKS, 3.4), 21.430183581707396e6);
}

#[test]
fn the_category_entities_cover_annex_j() {
    assert_eq!(category_ids().len(), 20);
    close(detail_category_delta_sigma_c("71").unwrap(), 71.0e6);
    assert!(detail_category_delta_sigma_c("72").is_none());
}

#[test]
fn the_s_n_curve_is_continuous_at_the_knee() {
    let at_knee = fatigue_strength_pa(71.0e6, 3.4, 5.4, N_D);
    close(at_knee, delta_sigma_d(71.0e6, 3.4));
    close(cycles_to_failure(at_knee, 71.0e6, 3.4, 5.4), N_D);
}
