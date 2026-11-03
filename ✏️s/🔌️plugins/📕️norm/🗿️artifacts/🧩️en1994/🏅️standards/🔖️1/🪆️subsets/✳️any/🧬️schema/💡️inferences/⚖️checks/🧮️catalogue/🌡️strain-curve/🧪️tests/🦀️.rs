//! 🧪️ Strain-compatibility curve against the independent numpy strip integration of the oracle and the polygon of the same section.

use super::super::encased::{EncasedI, Encasement, Strengths};
use super::super::interaction_polygon;
use super::*;

fn tube() -> ColSection {
    ColSection { b: 0.3, t: 0.01, as_m2: 4.0 * 314.16e-6, yr: 0.1, fyd: 355e6, fcd: 30e6 / 1.5, fsd: 500e6 / 1.15 }
}

#[test]
fn the_curve_ends_at_squash_load_with_no_moment_and_stays_inside_the_polygon_hull() {
    let curve = Strips::tube(&tube()).curve(400);
    let last = curve.last().unwrap();
    assert!(last.1.abs() < 1e-3 * curve.iter().map(|q| q.1.abs()).fold(0.0, f64::max), "{last:?}");
    let poly = interaction_polygon(&tube());
    assert!(last.0 <= poly[0].0 * 1.001, "{} {}", last.0, poly[0].0);
}

#[test]
fn moment_is_below_the_exact_plastic_moment_at_every_axial_force() {
    let s = tube();
    let curve = Strips::tube(&s).curve(400);
    for n in [0.5e6, 1.5e6, 3.0e6] {
        let plastic = (0..2000).map(|k| s.nm_at(-0.15 + 0.3 * f64::from(k) / 1999.0)).collect::<Vec<_>>();
        let mut plastic = plastic;
        plastic.sort_by(|a, b| a.0.total_cmp(&b.0));
        assert!(moment_at(&curve, n) <= 1.01 * moment_at(&plastic, n), "{n}");
    }
}

#[test]
fn encased_strips_give_a_curve_whose_peak_moment_is_close_to_the_polygon_point_d() {
    let g = EncasedI { kind: Encasement::Full, b_c: 0.4, h_c: 0.4, h: 0.2, b: 0.2, tw: 0.0085, tf: 0.0130, as_m2: 4.0 * 314.16e-6, cover: 0.05 };
    let s = Strengths { fyd: 355e6, fcd: 0.85 * 30e6 / 1.5, fsd: 500e6 / 1.15 };
    let peak = g.strips(&s).curve(400).iter().map(|q| q.1).fold(0.0, f64::max);
    let d = super::super::encased::interaction_polygon(&g, &s)[2].1;
    assert!(peak <= 1.01 * d && peak > 0.75 * d, "{peak} {d}");
}
