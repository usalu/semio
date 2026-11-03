//! 🧪️ Encased I-section polygon against hand integration and the closed-form plastic moments of EN 1994-1-1 6.7.3.2.

use super::*;

fn column() -> (EncasedI, Strengths) {
    (
        EncasedI { kind: Encasement::Full, b_c: 0.4, h_c: 0.4, h: 0.2, b: 0.2, tw: 0.0085, tf: 0.0130, as_m2: 4.0 * 314.16e-6, cover: 0.05 },
        Strengths { fyd: 355e6, fcd: 0.85 * 30e6 / 1.5, fsd: 500e6 / 1.15 },
    )
}

fn rel(a: f64, b: f64) -> f64 {
    (a - b).abs() / b.abs().max(1e-12)
}

#[test]
fn point_a_is_the_sum_of_the_three_plastic_resistances() {
    let (g, s) = column();
    let a_a = 2.0 * 0.2 * 0.013 + (0.2 - 0.026) * 0.0085;
    let a_c = 0.16 - a_a - 4.0 * 314.16e-6;
    let expected = a_a * s.fyd + a_c * s.fcd + 4.0 * 314.16e-6 * s.fsd;
    assert!(rel(g.n_pl(&s), expected) < 1e-9);
    assert!(rel(interaction_polygon(&g, &s)[0].0, expected) < 1e-9);
}

#[test]
fn points_b_and_d_match_the_exact_piecewise_integration_oracle() {
    let (g, s) = column();
    let poly = interaction_polygon(&g, &s);
    assert!(poly[3].0 == 0.0 && poly[1].1 == poly[3].1);
    assert!(rel(poly[3].1, 342_880.1) < 2e-3, "{}", poly[3].1);
    assert!(rel(poly[2].1, 407_639.4) < 2e-3, "{}", poly[2].1);
    assert!(rel(poly[2].0, 0.5 * g.n_pm(&s)) < 1e-12);
}

#[test]
fn polygon_is_ordered_a_c_d_b_with_d_carrying_the_largest_moment() {
    let (g, s) = column();
    let p = interaction_polygon(&g, &s);
    assert!(p[0].0 > p[1].0 && p[1].0 > p[2].0 && p[2].0 > p[3].0);
    assert!(p[2].1 >= p[1].1 && p[2].1 >= p[3].1);
}

#[test]
fn neutral_axis_far_above_the_section_gives_the_full_tension_and_far_below_the_full_compression() {
    let (g, s) = column();
    let compression = g.resultant(&s, -1.0);
    assert!(rel(compression.0, g.n_pl(&s)) < 1e-4 && compression.1.abs() < 1e-3 * g.n_pl(&s));
    let tension = g.resultant(&s, 1.0);
    assert!(tension.0 < 0.0);
}

#[test]
fn partially_encased_has_concrete_only_between_the_flanges() {
    let (mut g, s) = column();
    g.kind = Encasement::BetweenFlanges;
    let chamber = (0.2 - 0.0085) * (0.2 - 0.026) - 4.0 * 314.16e-6;
    assert!(rel(g.concrete_area(), chamber) < 1e-9);
    assert!(interaction_polygon(&g, &s)[3].1 > 0.0);
}
