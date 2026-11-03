//! 🧪️ Beam fire: the slab field, the lumped steel temperature and the reduced plastic moment against the independent oracle (`🧰️scripts/en1994-beam-fire-oracle.py`: scipy BDF on a converged grid, numpy replica of the explicit scheme, strip integration of the stdlib oracle).

use super::*;

const MINUTES: [f64; 4] = [30.0, 60.0, 90.0, 120.0];

fn beam() -> Beam {
    Beam { sec: ISection { h: 0.3, b: 0.15, tf: 0.0107, tw: 0.0071, a: 5381e-6, i_y: 83.56e-6 }, fyd: 355e6, ys: 0.15, thermal_m: 0.15, beff: 2.4, hc: 0.10, fcc: 25.5e6, am_v: 150.0, board_m: 0.02, rebar: RebarProduct::HotRolled }
}

fn close(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() < tol, "{a} vs {b}");
}

#[test]
fn steel_temperature_follows_the_lumped_capacity_oracle() {
    let e = beam().evolve(&MINUTES);
    for (p, expected) in e.points.iter().zip([191.3182, 368.8639, 507.6087, 612.9785]) {
        close(p.steel_c, expected, 0.05);
    }
    close(e.points[1].steel_c, 369.2838, 1.0);
}

#[test]
fn slab_field_matches_the_numpy_replica_of_the_scheme() {
    let e = beam().evolve(&MINUTES);
    let f = &e.fields[1];
    assert_eq!((f.nx, f.ny), (48, 15));
    for (k, field) in e.fields.iter().enumerate() {
        close(field.at(47, 0), [604.9405, 775.9448, 865.2680, 925.3858][k], 0.05);
        close(field.at(47, 7), [51.0465, 125.9835, 193.5142, 251.8618][k], 0.05);
        close(field.row_mean(11), [25.9485, 60.2950, 105.4269, 150.4617][k], 0.05);
        close(field.row_mean(14), [20.6310, 33.6077, 61.6792, 94.5969][k], 0.05);
        close(field.at(0, 0), [159.2500, 321.7296, 452.8111, 555.1054][k], 0.05);
        close(e.points[k].rebar_c, field.row_mean(11), 1e-9);
    }
}

#[test]
fn slab_field_converges_to_the_fine_grid_solution_away_from_the_flange() {
    let f = &beam().evolve(&MINUTES).fields[1];
    close(f.at(47, 0), 781.0459, 15.0);
    assert!(f.at(47, 0) > f.at(47, 7) && f.at(47, 7) > f.at(47, 14));
    assert!(f.at(0, 0) < f.at(47, 0), "the steel flange shields the soffit above it");
}

#[test]
fn field_is_monotone_in_time_and_bounded_by_the_gas_temperature() {
    let e = beam().evolve(&MINUTES);
    for pair in e.fields.windows(2) {
        assert!(pair[0].max() < pair[1].max());
    }
    for (p, f) in e.points.iter().zip(&e.fields) {
        assert!(f.max() < p.gas_c && f.theta.iter().all(|t| *t >= 20.0 - 1e-9));
        assert!(p.slab_top_c <= p.rebar_c && p.rebar_c <= p.soffit_c);
    }
}

#[test]
fn reduced_moment_follows_the_stress_block_oracle() {
    let b = beam();
    let e = b.evolve(&MINUTES);
    let cold = plastic_full(&b.sec, b.fyd, b.ys, b.beff, b.hc, b.fcc).m_nm;
    close(cold, 543263.8, 543263.8 * 1e-4);
    for (p, (steel_only, with_kc)) in e.points.iter().zip([(543263.80, 543263.80), (543263.80, 543263.80), (416424.84, 416424.84), (245753.88, 245746.88)]) {
        close(p.reduced.steel_only_nm, steel_only, steel_only * 1e-4);
        close(p.reduced.m_nm, with_kc, with_kc * 1e-4);
        assert!(p.reduced.m_nm <= p.reduced.steel_only_nm + 1e-6 && p.reduced.kc <= 1.0);
    }
    close(e.points[3].reduced.ky, 0.43885, 1e-4);
}

#[test]
fn resistance_never_increases_with_the_fire_duration() {
    let m: Vec<f64> = beam().evolve(&[0.0, 10.0, 20.0, 30.0, 45.0, 60.0, 90.0, 120.0]).points.iter().map(|p| p.reduced.m_nm).collect();
    assert!(m.windows(2).all(|w| w[1] <= w[0] + 1e-6));
    assert!(m[0] > 5e5 && m[7] < m[0]);
}

#[test]
fn the_temperature_field_is_computed_for_the_effective_thickness_not_the_total_depth() {
    let solid = beam();
    let ribbed = Beam { thermal_m: 0.12, ..solid };
    assert_eq!((solid.slab().thickness_m, ribbed.slab().thickness_m), (0.15, 0.12));
    let (a, b) = (solid.evolve(&[60.0]).points[0], ribbed.evolve(&[60.0]).points[0]);
    assert!(b.slab_top_c > a.slab_top_c, "a thinner effective slab heats its top more: {} vs {}", b.slab_top_c, a.slab_top_c);
}

#[test]
fn registry_rows_carry_the_unexposed_coefficient_and_the_steel_reduction_nodes_of_the_inline_table() {
    use semio_s_plugin_norm_compliance::{ndp_common, ndp_is_verified_for, AnnexChoice};
    assert_eq!(ndp_common("ndp.en1994.1-2.alpha-unexposed"), 9.0);
    assert!(!ndp_is_verified_for("ndp.en1994.1-2.alpha-unexposed", AnnexChoice::En));
    let ids = steel_reduction_ndp_ids();
    assert_eq!(ids.len(), 13);
    for (id, theta) in ids.iter().zip([20.0, 100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0, 1100.0, 1200.0]) {
        assert!((ndp_common(id) - ky_theta(theta)).abs() < 1e-12, "{id} differs from ky_theta({theta})");
    }
}

#[test]
fn the_reinforcement_strength_reduction_follows_the_product_of_the_beam_at_the_layer_temperature() {
    let hot = beam().evolve(&[90.0]).points[0];
    let cold = Beam { rebar: RebarProduct::ColdWorked, ..beam() }.evolve(&[90.0]).points[0];
    assert_eq!(hot.rebar_c, cold.rebar_c);
    assert_eq!(hot.rebar_ks, RebarProduct::HotRolled.reduction().at(hot.rebar_c));
    assert_eq!(cold.rebar_ks, RebarProduct::ColdWorked.reduction().at(cold.rebar_c));
    assert!(hot.rebar_ks > 0.0 && hot.rebar_ks <= 1.0 && cold.rebar_ks <= 1.0);
}
