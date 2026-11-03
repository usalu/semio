//! 🧪 Worked examples of the soil-structure kernel; the numbers come from `🧰️scripts/en1998-wave3d/1-hand-values.py` (Gazetas formulas, Table 4.1 interpolation at a_g S = 0.15 g and the period ratio from the flexibility sum).

use super::*;

fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= 1.0e-9 * b.abs().max(1.0), "{a} vs {b}");
}

#[test]
fn the_shear_modulus_degrades_with_the_square_of_the_velocity_ratio() {
    let g_max = shear_modulus_max_pa(19000.0, 200.0);
    near(g_max, 77471967.38022426);
    near(degraded_shear_modulus_pa(g_max, 0.8), 49582059.123343535);
}

#[test]
fn the_circular_footing_springs_follow_gazetas() {
    let springs = Springs::circular(49582059.123343535, 2.0, 0.35).expect("springs");
    near(springs.horizontal_n_per_m, 480795724.8324222);
    near(springs.vertical_n_per_m, 610240727.6719204);
    near(springs.rocking_nm_per_rad, 1627308607.1251209);
    near(springs.torsion_nm_per_rad, 2115501189.2626574);
}

#[test]
fn a_footing_without_modulus_radius_or_a_valid_poisson_ratio_has_no_springs() {
    assert!(Springs::circular(0.0, 2.0, 0.35).is_none());
    assert!(Springs::circular(5.0e7, 0.0, 0.35).is_none());
    assert!(Springs::circular(5.0e7, 2.0, 0.5).is_none());
}

#[test]
fn the_pier_period_lengthens_by_the_flexibility_of_the_springs() {
    let springs = Springs::circular(49582059.123343535, 2.0, 0.35).expect("springs");
    let k_pier = 3.0 * 3.0e9 / 8.0_f64.powi(3);
    near(k_pier, 17578125.0);
    near(period_lengthening(k_pier, 8.0, &springs), 1.3144907851614342);
    near(1.0 / springs.top_stiffness_n_per_m(8.0), 1.0 / springs.horizontal_n_per_m + 64.0 / springs.rocking_nm_per_rad);
}

#[test]
fn table_4_1_is_interpolated_between_its_rows_and_held_outside() {
    use semio_s_plugin_norm_compliance::AnnexChoice::{De, En};
    near(table_4_1_vs_ratio(0.15, En), 0.8);
    near(table_4_1_damping(0.15, De), 0.045);
    near(table_4_1_vs_ratio(0.10, En), 0.9);
    near(table_4_1_vs_ratio(0.30, En), 0.6);
    near(table_4_1_vs_ratio(0.02, En), 0.9);
    near(table_4_1_vs_ratio(0.6, En), 0.6);
    near(table_4_1_damping(0.25, En), 0.08);
}
