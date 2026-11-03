use super::*;
use crate::model::{En1993Snapshot, PlatedPanel};

fn base() -> PlatedPanel {
    En1993Snapshot::compliant_heb240_frame().plated_panels[0].clone()
}

fn k_of(alpha: f64) -> f64 {
    shear_ritz(alpha, 1.0).expect("valid panel").0
}

#[test]
fn the_ritz_shear_coefficient_matches_the_scipy_oracle_and_the_classical_values() {
    for (alpha, scipy, classical) in [(1.0, 9.328349293898766, 9.34), (1.5, 7.0730549018388675, 7.1), (2.0, 6.548053338422909, 6.6), (3.0, 5.843302365885329, 5.9)] {
        assert!((k_of(alpha) - scipy).abs() <= 1e-6 * scipy, "{alpha}: {}", k_of(alpha));
        assert!((k_of(alpha) - classical).abs() <= 0.02 * classical);
    }
    assert!(shear_ritz(0.0, 1.0).is_none());
}

#[test]
fn the_shear_mode_ghost_is_a_normalised_harmonic_surface_with_the_critical_load_factor() {
    let panel = PlatedPanel { a: 2.0, b: 1.0, thickness: 0.008, shear_force: 4.0e5, ..base() };
    let mode = shear_mode(&panel).expect("shear mode");
    assert_eq!(mode.kind, DeformationKind::Mode);
    assert_eq!(mode.surface.len(), (GRID + 1) * (GRID + 1));
    let top = mode.surface.iter().fold(0.0_f64, |m, s| m.max(s.delta.z.abs()));
    assert!((top - 1.0).abs() < 1e-12);
    let d = YOUNG * panel.thickness.powi(3) / (12.0 * (1.0 - POISSON * POISSON));
    let tau_cr = k_of(2.0) * PI * PI * d / (panel.thickness * panel.b * panel.b);
    let alpha = mode.load_factor.expect("load factor");
    assert!((alpha - tau_cr / (4.0e5 / (panel.b * panel.thickness))).abs() <= 1e-9 * alpha);
    assert!(shear_mode(&PlatedPanel { shear_force: 0.0, ..panel }).is_some_and(|m| m.load_factor.is_none()));
}
