//! 🧪️ Crack width against structuralcodes (`🧰️scripts/en1994-crack-width-oracle.py`: ε_sm − ε_cm = 6.95481e-4, s_r,max = 0.311785 m, w_k = 0.216841 mm).

use super::*;

#[test]
fn crack_width_matches_structuralcodes() {
    let phi = bar_diameter_m(0.001340, 0.15);
    assert!((phi - 0.016).abs() < 1e-4, "{phi}");
    let w = crack_width_m(240e6, 0.001340, 0.15, 0.2, 30e6, 33e9);
    assert!((w - 2.16841e-4).abs() / 2.16841e-4 < 0.02, "{w}");
}

#[test]
fn the_crack_width_grows_with_the_bar_stress_and_vanishes_without_it() {
    let at = |s: f64| crack_width_m(s, 0.001340, 0.15, 0.2, 30e6, 33e9);
    assert_eq!(at(0.0), 0.0);
    assert!(at(300e6) > at(200e6));
}
