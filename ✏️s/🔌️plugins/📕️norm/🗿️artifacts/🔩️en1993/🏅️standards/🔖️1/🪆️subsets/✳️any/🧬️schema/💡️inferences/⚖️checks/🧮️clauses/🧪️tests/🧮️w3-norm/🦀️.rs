//! 🧮️ Clause tests of the wave 3 norm items: hand values and the numbers of the independent python oracles (`🔮️oracles/🔩️bolt-group-ic`, `🪜️effective-length`, `⏳️stress-range`, `🪛️fasteners`, `🥫️forming-gain`).

use crate::{bolt_group_ic as ic, cold_formed, effective_length as el, fasteners as fs, stress_range as sr};

fn close(a: f64, b: f64, tolerance: f64, what: &str) {
    assert!((a - b).abs() <= tolerance * b.abs().max(1e-12), "{what}: {a} vs {b}");
}

fn grid(rows: usize, per_row: usize, pitch: f64, gauge: f64) -> Vec<(f64, f64)> {
    (0..rows).flat_map(|r| (0..per_row).map(move |c| ((c as f64 - (per_row as f64 - 1.0) / 2.0) * gauge, (r as f64 - (rows as f64 - 1.0) / 2.0) * pitch))).collect()
}

#[test]
fn the_registry_carries_the_load_deformation_law_of_one_bolt() {
    let law = ic::law();
    assert_eq!((law.mu, law.lambda, law.delta_max), (10.0, 0.55, 8.64));
    close(ic::bolt_force(&law, 1.0e5, 1.0), 1.0e5, 1e-12, "the farthest bolt reaches R_ult");
    assert!(ic::bolt_force(&law, 1.0e5, 0.0) == 0.0);
}

#[test]
fn a_load_through_the_centroid_is_shared_equally() {
    let points = grid(3, 2, 75.0, 100.0);
    let s = ic::solve(&ic::law(), &points, 1.0e5, 0.0, 1000.0, 0.0).expect("concentric group");
    close(s.factor, 6.0 * 1.0e5 / 1000.0, 1e-12, "n R / F");
    assert!(s.centre.is_none());
    assert!(s.forces.iter().all(|g| g.0 == 0.0 && (g.1 - 1.0e5).abs() < 1e-9));
}

#[test]
fn a_pure_moment_turns_the_group_about_its_centroid() {
    let points = vec![(-50.0, 0.0), (50.0, 0.0)];
    let s = ic::solve(&ic::law(), &points, 1.0e5, 0.0, 0.0, 1.0e6).expect("pure moment");
    close(s.factor * 1.0e6, 1.0e7, 1e-9, "M_ult = 2 a R_ult");
    assert_eq!(s.centre, Some((0.0, 0.0)));
}

#[test]
fn the_elastic_vectors_follow_the_polar_moment() {
    let points = grid(3, 2, 75.0, 100.0);
    close(ic::elastic_max(&points, 0.0, 1000.0, 1000.0 * 150.0), 473.75568011839647, 1e-12, "oracle elastic");
    let v = ic::elastic_vectors(&points, 0.0, 1000.0, 0.0);
    assert!(v.iter().all(|q| q.0 == 0.0 && (q.1 - 1000.0 / 6.0).abs() < 1e-12));
    let sum: (f64, f64) = ic::elastic_vectors(&points, 300.0, 1000.0, 120000.0).iter().fold((0.0, 0.0), |a, q| (a.0 + q.0, a.1 + q.1));
    close(sum.0, 300.0, 1e-9, "sum x");
    close(sum.1, 1000.0, 1e-9, "sum y");
}

#[test]
fn the_instantaneous_centre_matches_the_scipy_oracle() {
    let law = ic::law();
    for (rows, pitch, gauge, fx, e, elastic, factor) in [(3, 75.0, 100.0, 0.0, 150.0, 473.75568011839647, 249.99999612305191), (4, 80.0, 90.0, 0.0, 100.0, 234.92265367799646, 536.1776768649606)] {
        let points = grid(rows, 2, pitch, gauge);
        let s = ic::solve(&law, &points, 1.0e5, fx, 1000.0, 1000.0 * e).expect("converges");
        close(s.factor, factor, 1e-6, "load factor");
        close(ic::elastic_max(&points, fx, 1000.0, 1000.0 * e), elastic, 1e-9, "elastic");
        assert!(s.factor > 1.0e5 / elastic, "the plastic distribution carries more than the elastic one");
    }
    let points = grid(3, 2, 75.0, 100.0);
    let s = ic::solve(&law, &points, 1.0e5, 300.0, 1000.0, 120000.0).expect("inclined");
    close(s.factor, 294.0790129777767, 1e-6, "inclined load factor");
    let sum = s.forces.iter().fold((0.0, 0.0), |a, g| (a.0 + g.0, a.1 + g.1));
    close(sum.0 / s.factor, 300.0, 1e-5, "equilibrium x");
    close(sum.1 / s.factor, 1000.0, 1e-5, "equilibrium y");
}

#[test]
fn the_showcase_fin_plate_group_gains_eighteen_percent_over_the_elastic_method() {
    let points = grid(3, 2, 70.0, 100.0);
    let s = ic::solve(&ic::law(), &points, 94080.0, 0.0, 30000.0, 30000.0 * 150.0).expect("converges");
    close(s.factor, 7.591173736544717, 1e-6, "oracle factor");
    close(s.factor * ic::elastic_max(&points, 0.0, 30000.0, 4.5e6) / 94080.0, 1.1836761510689595, 1e-6, "gain over the elastic method");
}

#[test]
fn the_non_sway_nomogram_matches_the_scipy_oracle() {
    for (ga, gb, k) in [(1.0, 1.0, 0.7742650686480754), (0.5, 2.0, 0.7646542638997111), (10.0, 10.0, 0.9625009769338223), (0.1, 0.1, 0.5486975576213342), (2.0, 5.0, 0.891362426734382)] {
        close(el::k_non_sway(ga, gb), k, 1e-9, &format!("non-sway {ga}:{gb}"));
    }
    close(el::k_non_sway(0.0, 0.0), 0.5, 1e-9, "fixed at both ends");
    close(el::k_non_sway(0.0, el::UNBOUNDED), 0.6991556596428412, 1e-6, "fixed and pinned, tan x = x");
}

#[test]
fn the_sway_nomogram_matches_the_scipy_oracle() {
    for (ga, gb, k) in [(1.0, 1.0, 1.3172751026289118), (0.5, 2.0, 1.3667664809123463), (10.0, 10.0, 3.010392659171542), (0.1, 0.1, 1.0333048474667663), (2.0, 5.0, 1.86959151611498)] {
        close(el::k_sway(ga, gb).expect("finite"), k, 1e-9, &format!("sway {ga}:{gb}"));
    }
    close(el::k_sway(0.0, 0.0).expect("fixed"), 1.0, 1e-9, "fixed at both ends");
    close(el::k_sway(0.0, el::UNBOUNDED).expect("cantilever"), 2.0, 1e-3, "cantilever");
    assert!(el::k_sway(el::UNBOUNDED, el::UNBOUNDED).is_none(), "both ends pinned is a mechanism");
}

#[test]
fn the_distribution_factor_is_the_stiffness_ratio_and_a_missing_beam_is_a_pinned_end() {
    close(el::distribution(2.0e-5, 4.0e-5), 0.5, 1e-12, "G");
    assert_eq!(el::distribution(2.0e-5, 0.0), el::UNBOUNDED);
}

#[test]
fn the_stress_range_counts_the_compressive_part_with_the_registry_factor() {
    close(sr::range(60e6, -20e6, true), 72e6, 1e-12, "stress relieved");
    close(sr::range(60e6, -20e6, false), 80e6, 1e-12, "as welded");
    close(sr::range(-10e6, -50e6, true), 24e6, 1e-12, "both compressive");
    close(sr::fibre_stress(100000.0, 40000.0, 0.0, 0.0106, 1.126e-4, 3.923e-5, 0.12, 0.0), 52062736.686886296, 1e-12, "oracle sigma g");
    close(sr::fibre_stress(43333.0, 17333.0, 0.0, 0.0106, 1.126e-4, 3.923e-5, 0.12, 0.0), 22560132.54465632, 1e-12, "oracle sigma q");
    close(sr::fibre_stress(0.0, 0.0, 1000.0, 0.01, 1.0e-4, 2.0e-5, 0.0, 0.1), -5.0e6, 1e-12, "minor axis sign");
}

#[test]
fn the_screw_and_rivet_resistances_match_the_python_oracle() {
    close(fs::bearing(fs::Kind::Screw, 0.00075, 0.0015, 0.0055, 360e6, 1.25), 3002.3446121070315, 1e-12, "screw ratio 2");
    close(fs::bearing(fs::Kind::Screw, 0.001, 0.0025, 0.0055, 360e6, 1.25), 5068.8, 1e-12, "thick support");
    close(fs::bearing(fs::Kind::Rivet, 0.001, 0.001, 0.0048, 400e6, 1.25), 2523.9055449838056, 1e-12, "rivet equal thickness");
    close(fs::pull_over(0.016, 0.00075, 360e6, 1.25), 5184.0, 1e-12, "pull-over");
    close(fs::pull_out(0.0055, 0.0015, 360e6, 1.25), 1069.2, 1e-12, "pull-out");
    close(fs::net_section(0.1, 2.0, 0.0055, 0.00075, 360e6, 1.25), 19224.0, 1e-12, "net section");
    assert_eq!(fs::spacing(fs::Kind::Screw), [3.0, 1.5, 3.0, 3.0]);
    assert_eq!(fs::spacing(fs::Kind::Rivet), [1.5, 1.5, 3.0, 3.0]);
}

#[test]
fn the_average_yield_strength_is_capped_by_the_mean_of_f_u_and_f_yb() {
    close(cold_formed::average_yield(350e6, 420e6, 7.0, 4.0, 0.003, 0.0006), 379.4e6, 1e-12, "roll forming");
    close(cold_formed::average_yield(350e6, 420e6, 5.0, 4.0, 0.003, 0.0006), 371.0e6, 1e-12, "other forming");
    close(cold_formed::average_yield(350e6, 420e6, 7.0, 40.0, 0.003, 0.0006), 385.0e6, 1e-12, "cap");
    close(cold_formed::average_yield(350e6, 420e6, 7.0, 0.0, 0.003, 0.0006), 350.0e6, 1e-12, "no bends");
}
