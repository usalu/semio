//! ⏱️ Latency gates of the heavy shared kernels (architecture §5.7): the slope grid search and the Mononobe–Okabe sweep must fit the 12 ms drag preview, the fibre moment-curvature the 400 ms commit-to-checks budget. Every figure is measured on what the user does: `cold` (the first evaluation of a fresh session, one sample, its own budget), `drag` (a different input every step) and `commit` (a different real edit of a fresh section every round, so the section memo never answers). Release budgets; debug builds get 20 times.

use crate::geotechnics::earth_pressure::{critical_wedge, thrust, Acceleration, Interface, Side, Water};
use crate::geotechnics::slope::{bishop_with, Circle, Pseudostatic};
use crate::section::fibre::{elastic_plastic, parabola_rectangle, rectangle_fibres, Fibre, FibreSection, Limits, Reinforcement};
use std::time::Instant;

const DRAG_MS: f64 = 12.0;
const COMMIT_MS: f64 = 400.0;
const COLD_MS: f64 = 1000.0;
const WARM_UP: usize = 3;

/// ⏱️ The cold sample, then the p50 and p95 in milliseconds of `rounds` runs after [`WARM_UP`] discarded ones; `work` receives a different round number on every call, so every call sees a different input.
fn scenarios(mut work: impl FnMut(usize), rounds: usize) -> (f64, (f64, f64)) {
    let mut time = |round: usize| {
        let started = Instant::now();
        work(round);
        started.elapsed().as_secs_f64() * 1000.0
    };
    let cold = time(0);
    (1..=WARM_UP).for_each(|round| drop(time(round)));
    let mut ms: Vec<f64> = (0..rounds).map(|round| time(WARM_UP + 1 + round)).collect();
    ms.sort_by(f64::total_cmp);
    let at = |percent: usize| ms[(ms.len() * percent / 100).min(ms.len() - 1)];
    (cold, (at(50), at(95)))
}

fn gate(name: &str, scenario: &str, (p50, measured): (f64, f64), release_ms: f64) {
    let budget = if cfg!(debug_assertions) { release_ms * 20.0 } else { release_ms };
    eprintln!("[latency] {name}/{scenario} p50_ms={p50:.3} p95_ms={measured:.3} budget_ms={budget:.1}");
    assert!(measured < budget, "{name}/{scenario}: p95 {measured:.2} ms exceeds the {budget} ms budget");
}

fn gate_cold(name: &str, cold_ms: f64) {
    gate(name, "cold", (cold_ms, cold_ms), COLD_MS);
}

fn rc_section(round: usize) -> FibreSection<'static> {
    let (b, h, d, strength, f_y) = (300.0, 500.0, 450.0, 30.0 + 0.01 * round as f64, 500.0);
    let bar = move |area: f64, z: f64| Reinforcement { fibre: Fibre::lumped(z, area), law: Box::new(move |e: f64| elastic_plastic(f_y, 200_000.0, e)), displaces: true };
    FibreSection { concrete: rectangle_fibres(b, h, 120), concrete_law: Box::new(move |e: f64| parabola_rectangle(strength, 2.0, 2.0, e)), reinforcement: vec![bar(1800.0 + 7.0 * round as f64, 0.5 * h - d), bar(900.0, d - 0.5 * h)], top: 0.5 * h, cache: Default::default() }
}

#[test]
fn the_slope_grid_search_over_168_circles_fits_the_drag_preview() {
    let proto = Circle { h: 10.0, beta_deg: 30.0, xc: 0.0, yc: 0.0, r: 1.0, gamma: 18_000.0, gamma_sat: 20_000.0, gwl_level: 5.0, ru: 0.0, q: 0.0 };
    let (tan_phi, c) = (30.0_f64.to_radians().tan(), 5_000.0);
    let search = |round: usize| {
        let proto = Circle { h: proto.h + 0.01 * round as f64, ..proto };
        let run = proto.h / proto.beta_deg.to_radians().tan();
        let mut best = f64::INFINITY;
        for ix in 0..7 {
            for iy in 0..6 {
                let (xc, yc) = (-0.5 * proto.h + (run + proto.h) * f64::from(ix) / 6.0, 0.6 * proto.h + 1.8 * proto.h * f64::from(iy) / 5.0);
                let to_toe = (run - xc).hypot(yc);
                for k in [1.0, 1.08, 1.18, 1.3] {
                    if let Some(f) = bishop_with(&Circle { xc, yc, r: to_toe * k, ..proto }, tan_phi, c, 30, &Pseudostatic::NONE) {
                        best = best.min(f);
                    }
                }
            }
        }
        std::hint::black_box(best);
    };
    let (cold, drag) = scenarios(search, 30);
    gate_cold("slope-grid-search", cold);
    gate("slope-grid-search", "drag", drag, DRAG_MS);
}

#[test]
fn the_mononobe_okabe_sweep_fits_the_drag_preview() {
    let interface = Interface::vertical(32.0, 16.0);
    let water = Water { height_m: 1.5, saturated_unit_weight: 20_000.0, permeable: true };
    let sweep = |round: usize| {
        let mut total = 0.0;
        for i in 0..100 {
            let kh = 0.02 + 0.003 * f64::from(i) + 1.0e-5 * round as f64;
            let acceleration = Acceleration { kh, kv: 0.5 * kh };
            total += thrust(Side::Active, &interface, 18_000.0, 5.0, water, acceleration).map_or(0.0, |t| t.total());
            total += critical_wedge(18_000.0, 5.0, 32.0, 16.0, kh, 1.0 + 0.5 * kh).1;
        }
        std::hint::black_box(total);
    };
    let (cold, drag) = scenarios(sweep, 30);
    gate_cold("mononobe-okabe-kernel", cold);
    gate("mononobe-okabe-kernel", "drag", drag, DRAG_MS);
}

#[test]
fn the_moment_curvature_curve_and_its_yield_and_ultimate_points_fit_the_commit_budget() {
    let limits = Limits { concrete_yield: 2.0, concrete_ultimate: 3.5, steel_yield: 2.5, steel_ultimate: 45.0 };
    let (cold, commit) = scenarios(|round| drop(std::hint::black_box(rc_section(round).characteristics(0.0, &limits))), 10);
    gate_cold("fibre-characteristics", cold);
    gate("fibre-characteristics", "commit", commit, COMMIT_MS);
    let (cold, commit) = scenarios(|round| drop(std::hint::black_box(rc_section(round).curve(0.0, 0.03, 40, 35.0))), 10);
    gate_cold("fibre-moment-curvature-curve", cold);
    gate("fibre-moment-curvature-curve", "commit", commit, COMMIT_MS);
}
