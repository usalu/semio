//! 🔬️ Worked resistance examples of the EN 1999 compliance tests: the hand-derived buckling, alloy, moment, fatigue, fire and cold-formed values. See [`super`].

use super::*;

#[test]
fn chi_from_lambda_class_a_worked() {
    let chi = part_1_1::chi_from_lambda(1.0, 0.20, 0.10);
    assert!((chi - 0.656_f64).abs() < 0.01, "chi={chi}");
}

#[test]
fn alloy_table_3_2_6082_t6() {
    let a = part_1_1::resolve_alloy("aw6082-t6").expect("catalogue alloy");
    assert!((a.f_o_pa - 260.0e6).abs() < 1.0);
    assert!((a.rho_o_haz - 0.64).abs() < 1e-9);
    assert!(a.buckling_class_a);
    let (des, row) = part_1_1::CATALOGUE_ALLOY_ROWS.iter().find(|(d, _)| *d == "aw6082-t6").expect("catalogue row");
    assert_eq!(*des, "aw6082-t6");
    assert!((row.f_o_pa - a.f_o_pa).abs() < 1.0);
    let snap = En1999Snapshot::compliant_roof_purlin();
    assert_eq!(snap.materials[0].designation, "aw6082-t6");
    let report = evaluate_structure(&snap);
    let n = report.checks.iter().find(|c| c.id.contains("6.2.3.n.purlin-1")).expect("N_Rd check");
    let n_rd = n.limit.value;
    assert!(n_rd > 1.0, "N_Rd must be positive for aw6082-t6 member");
    let section = snap.sections.iter().find(|s| s.id == snap.members[0].section_id).unwrap();
    let a_eff = part_1_1::effective_area(section, row.f_o_pa, true, row.rho_o_haz, row.rho_u_haz);
    let expected = a_eff * row.f_o_pa / na_de::AnnexParams::de().gamma_m1;
    assert!((n_rd - expected).abs() / expected.max(1.0) < 1e-9, "N_Rd limit {n_rd} must equal catalogue-based {expected}");
}

#[test]
fn alloy_5083_class_b() {
    let a = part_1_1::resolve_alloy("aw5083-o").expect("catalogue alloy");
    assert!(!a.buckling_class_a);
    assert!((a.rho_o_haz - 1.0).abs() < 1e-12);
}

#[test]
fn unknown_alloy_emits_fail_with_oneof_catalogue() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    doc.materials[0].designation = "aw9999-t6".into();
    let report = evaluate_structure(&doc);
    let fail = report.failing().find(|c| c.id.contains("alloy")).expect("unknown alloy must Fail");
    assert!(
        fail.remedies.iter().any(|r| r.target.path.contains("designation") && r.options.iter().any(|o| o.starts_with("aw"))),
        "expected OneOf catalogue remedy, got {:?}",
        fail.remedies
    );
}

#[test]
fn m_c_rd_6082_hand() {
    let wel = 24_000.0e-9;
    let m = wel * 260.0e6 / 1.1;
    assert!(((m / 1e3) - 5.6727_f64).abs() < 0.01);
}

#[test]
fn haz_rho_reduces_effective_modulus() {
    let doc = En1999Snapshot::noncompliant_multi_fail();
    let sec = &doc.sections[0];
    let alloy = part_1_1::resolve_alloy("aw6060-t6").expect("alloy");
    let wel = part_1_1::effective_wel_y(sec, alloy.f_o_pa, true, alloy.rho_o_haz, alloy.rho_u_haz);
    let (_, _, _, wel_gross, _, _, _) = part_1_1::section_geometry(sec);
    assert!(wel < wel_gross);
}

#[test]
fn fatigue_strength_at_5e5() {
    // Δσ_R = Δσ_C · (N_C/N)^{1/m1} = 71 · (2e6/5e5)^{1/4.3} ≈ 98.01 MPa
    let rd = part_1_3::fatigue_strength_pa(71.0e6, 4.3, 6.3, 500_000.0);
    assert!((rd / 1e6 - 98.01).abs() < 0.5, "rd_mpa={}", rd / 1e6);
}

#[test]
fn fire_k_theta_curve() {
    assert!((part_1_2::k_theta(100.0) - 1.0).abs() < 1e-12);
    assert!(part_1_2::k_theta(550.0) <= 0.0);
    assert!((part_1_2::k_theta(325.0) - 0.5).abs() < 1e-9);
}

#[test]
fn fire_utilization_applies_k_theta_to_member_resistance() {
    let doc = En1999Snapshot::compliant_roof_purlin();
    let k = part_1_2::k_theta(doc.fire_scenarios[0].theta_a);
    assert!(k < 1.0 && k > 0.0);
    let report = evaluate_structure(&doc);
    let check = report.checks.iter().find(|c| c.id.contains("1-2.fire")).expect("fire check");
    assert!(check.explanation.en.contains("N_fi,Rd") || check.explanation.en.contains("k_θ"));
}

#[test]
fn cold_formed_and_shells_na_when_absent() {
    let report = evaluate_structure(&En1999Snapshot::empty());
    assert!(report.checks.iter().any(|c| c.id == "en1999.1-4.empty" && c.status == CheckStatus::NotApplicable));
    assert!(report.checks.iter().any(|c| c.id == "en1999.1-5.empty" && c.status == CheckStatus::NotApplicable));
}

#[test]
fn cold_formed_and_shells_evaluate_when_present() {
    let pass = evaluate_structure(&En1999Snapshot::compliant_roof_purlin());
    assert!(pass.checks.iter().any(|c| c.id.contains("1-4.") && c.status != CheckStatus::NotApplicable));
    assert!(pass.checks.iter().any(|c| c.id.contains("1-5.") && c.status != CheckStatus::NotApplicable));
    assert!(!pass.checks.iter().any(|c| c.id == "en1999.1-4.empty"));
    assert!(!pass.checks.iter().any(|c| c.id == "en1999.1-5.empty"));
    let fail = evaluate_structure(&En1999Snapshot::noncompliant_multi_fail());
    let cold_fail = fail.checks.iter().filter(|c| c.id.contains("1-4.") && c.status == CheckStatus::Fail).collect::<Vec<_>>();
    let shell_fail = fail.checks.iter().filter(|c| c.id.contains("1-5.") && c.status == CheckStatus::Fail).collect::<Vec<_>>();
    assert!(!cold_fail.is_empty(), "expected EN 1999-1-4 Fail");
    assert!(!shell_fail.is_empty(), "expected EN 1999-1-5 Fail");
    assert!(cold_fail.iter().all(|c| c.remedies.iter().any(|r| r.applicable)), "1-4 remedies");
    assert!(shell_fail.iter().all(|c| c.remedies.iter().any(|r| r.applicable)), "1-5 remedies");
    let mut doc = En1999Snapshot::empty();
    doc.materials = En1999Snapshot::compliant_roof_purlin().materials.clone();
    let mat_id = doc.materials[0].id.clone();
    doc.cold_formed.push(semio_s_artifact_norm_en1999_model::ColdFormedSheet {
        id: "sheet-1".into(),
        material_id: mat_id.clone(),
        thickness: 0.0025,
        width: 0.20,
        span: 1.2,
        welded: false,
        actions: vec![semio_s_artifact_norm_en1999_model::MemberAction {
            id: "G".into(),
            kind: "permanent".into(),
            category: "self".into(),
            source: "external".into(),
            g_k_line: 0.0,
            q_k_line: 0.0,
            n_k: 0.0,
            v_y_k: 0.0,
            v_z_k: 0.0,
            m_y_k: 400.0 / 1.35,
            m_z_k: 0.0,
        }],
    });
    doc.shells.push(semio_s_artifact_norm_en1999_model::AluminiumShell {
        id: "shell-1".into(),
        material_id: mat_id,
        radius: 0.60,
        thickness: 0.008,
        length: 2.0,
        actions: vec![semio_s_artifact_norm_en1999_model::MemberAction {
            id: "G".into(),
            kind: "permanent".into(),
            category: "self".into(),
            source: "external".into(),
            g_k_line: 0.0,
            q_k_line: 0.0,
            n_k: 20.0e6 / 1.35,
            v_y_k: 0.0,
            v_z_k: 0.0,
            m_y_k: 40.0e6 / 1.35,
            m_z_k: 0.0,
        }],
    });
    let report = evaluate_structure(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("1-4.")));
    assert!(report.checks.iter().any(|c| c.id.contains("1-5.")));
    assert!(!report.checks.iter().any(|c| c.id == "en1999.1-4.empty"));
    assert!(!report.checks.iter().any(|c| c.id == "en1999.1-5.empty"));
}
