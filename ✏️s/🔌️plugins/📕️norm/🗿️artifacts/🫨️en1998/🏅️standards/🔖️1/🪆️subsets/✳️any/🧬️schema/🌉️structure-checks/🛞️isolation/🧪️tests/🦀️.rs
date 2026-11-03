//! 🧪 Isolation checks on the worked example of the kernel (Q_d 200 kN, K_d 6 MN/m, d_y 10 mm, M 300 t, site a_g S 3 m/s², T_C 0.5 s); the numbers come from `🧰️scripts/en1998-wave3d/1-hand-values.py`.

use super::*;
use compliance::CheckStatus;

fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= 1.0e-7 * b.abs().max(1.0e-12), "{a} vs {b}");
}

fn system() -> System {
    System { id: "iso-1".into(), bridge_id: "br-1".into(), building_id: String::new(), strength_n: 2.0e5, stiffness_n_per_m: 6.0e6, yield_displacement_m: 0.01, capacity_m: 0.35 }
}

fn supported(fixed_period_s: f64) -> Supported {
    Supported { is_bridge: true, id: "br-1".into(), mass_kg: 3.0e5, fixed_period_s, shear_rd_n: 1.0e6 }
}

fn spectrum() -> Spectrum {
    Spectrum { a_g: 2.5, s: 1.2, tb: 0.15, tc: 0.5, td: 2.0 }
}

fn run(system: &System, supported: &Supported) -> CheckReport {
    let mut report = CheckReport::default();
    check_system(&mut report, AnnexChoice::En, system, supported, &spectrum());
    report
}

fn check<'a>(report: &'a CheckReport, tail: &str) -> &'a CheckResult {
    report.checks.iter().find(|c| c.id == format!("en1998.isolation.iso-1.{tail}")).unwrap_or_else(|| panic!("{tail} missing"))
}

#[test]
fn the_design_displacement_period_damping_and_shear_follow_the_equivalent_linear_hand_values() {
    let report = run(&system(), &supported(0.35));
    let displacement = check(&report, "displacement");
    near(displacement.computed.value, 0.07485001860065095);
    near(displacement.utilization, 0.07485001860065095 / 0.35);
    assert_eq!(displacement.status, CheckStatus::Pass);
    let period = check(&report, "period");
    near(period.computed.value, 1.1686396311770098 / 0.35);
    near(period.utilization, 3.0 / (1.1686396311770098 / 0.35));
    assert_eq!(period.status, CheckStatus::UnverifiedBasis);
    near(check(&report, "maxPeriod").computed.value, 1.1686396311770098);
    near(check(&report, "maxPeriod").utilization, 1.1686396311770098 / 3.0);
    near(check(&report, "damping").computed.value, 0.1699482227299225);
    near(check(&report, "damping").utilization, 0.1699482227299225 / 0.30);
    let shear = check(&report, "shear");
    near(shear.computed.value, 649100.1116039057);
    near(shear.utilization, 0.6491001116039057);
}

#[test]
fn a_capacity_below_the_design_displacement_fails_with_the_displacement_as_remedy() {
    let mut weak = system();
    weak.capacity_m = 0.05;
    let displacement = check(&run(&weak, &supported(0.35)), "displacement").clone();
    assert_eq!(displacement.status, CheckStatus::Fail);
    let remedy = displacement.remedies.first().expect("remedy");
    near(remedy.required.value, 0.07485001860065095);
}

#[test]
fn a_short_ratio_to_the_fixed_base_period_fails_and_the_softer_stiffness_of_the_remedy_restores_it() {
    let report = run(&system(), &supported(0.5));
    let period = check(&report, "period");
    assert_eq!(period.status, CheckStatus::Fail);
    let allowed = period.remedies.first().expect("remedy").required.value;
    assert!(allowed < 6.0e6);
    let law = Bilinear { strength_n: 2.0e5, post_yield_n_per_m: allowed, yield_displacement_m: 0.01 };
    let state = law.design(3.0e5, &spectrum()).expect("equilibrium");
    assert!((state.period_s / 0.5 - 3.0).abs() < 1.0e-6, "{}", state.period_s / 0.5);
}

#[test]
fn parameters_that_are_no_hysteresis_loop_fail_with_a_remedy_per_parameter() {
    let mut broken = system();
    broken.stiffness_n_per_m = 0.0;
    broken.yield_displacement_m = 0.0;
    let report = run(&broken, &supported(0.35));
    let law = check(&report, "law");
    assert_eq!(law.status, CheckStatus::Fail);
    assert_eq!(law.remedies.len(), 2);
}

#[test]
fn a_building_reports_the_isolation_shear_as_information() {
    let building = Supported { is_bridge: false, id: "bldg-1".into(), mass_kg: 3.0e5, fixed_period_s: 0.35, shear_rd_n: 0.0 };
    let report = run(&system(), &building);
    assert_eq!(check(&report, "shear").status, CheckStatus::Info);
    near(check(&report, "period").computed.value, 1.1686396311770098 / 0.35);
}

#[test]
fn a_structure_without_mass_has_no_isolation_verdict() {
    let massless = Supported { mass_kg: 0.0, ..supported(0.35) };
    assert_eq!(check(&run(&system(), &massless), "design").status, CheckStatus::NotApplicable);
}
