use super::*;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;
use scene::projection_kit::test_support::revision;

fn report_of(document: &En1992Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    for member in &document.members {
        report.extend(checks::evaluate_member(document, member));
    }
    report
}

#[test]
fn the_truss_resistances_follow_the_closed_forms_over_cot_theta() {
    let document = En1992Snapshot::compliant_office_frame();
    let report = report_of(&document);
    let input = SceneInput { document: &document, report: &report, placements: &[], revision: revision() };
    let member = document.members.iter().find(|m| basis(&input, m).is_some()).expect("member with links");
    let s = basis(&input, member).expect("basis");
    let (one, _) = truss_resistances(&input, &s, 1.0);
    let (two, _) = truss_resistances(&input, &s, 2.0);
    assert!((one - s.asw_s * s.z * s.f_yk / GAMMA_S / 1e3).abs() < 1e-9 * one.abs().max(1.0));
    assert!((two - 2.0 * one).abs() < 1e-9 * two.abs().max(1.0));
    let (_, max_low) = truss_resistances(&input, &s, 1.0);
    let (_, max_high) = truss_resistances(&input, &s, 2.5);
    assert!(max_low > max_high && max_high > 0.0);
}

#[test]
fn the_strut_and_tie_forces_close_the_node_equilibrium() {
    let s = ShearBasis { b: 0.3, d: 0.5, z: 0.45, asw_s: 1e-3, f_ck: 30.0e6, f_yk: 500.0e6, v_ed: 400.0e3, check: "en1992.6.2.shear.x".into(), utilisation: 0.7 };
    let model = strut_and_tie(&s, 2.0);
    assert!((model.theta - (0.45_f64 / 1.0).atan()).abs() < 1e-12);
    assert!((model.strut * model.theta.sin() - s.v_ed).abs() < 1e-6 * s.v_ed);
    assert!((model.strut * model.theta.cos() - model.tie).abs() < 1e-6 * model.tie);
}

#[test]
fn the_shear_planes_stand_at_d_from_both_supports() {
    let document = En1992Snapshot::compliant_office_frame();
    let report = report_of(&document);
    let scene = crate::project(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    let input = SceneInput { document: &document, report: &report, placements: &[], revision: revision() };
    for member in document.members.iter().filter(|m| basis(&input, m).is_some_and(|s| m.span > 2.0 * s.d) && crate::layout::class_of(m.kind) != Class::Upright) {
        for tag in ["start", "end"] {
            assert!(scene.entities.iter().any(|e| e.entity.id == format!("{}:{SHEAR_PLANE_TAG}-{tag}", member.id)), "{} {tag}", member.id);
        }
    }
}
