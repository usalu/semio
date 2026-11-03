//! 🧩️ Joint plate and foundation block tests: the worked examples of the showcase match the python oracle (`🔮️oracles/🧩️plate-and-foundation`, closed forms with the third-party blue-prints Table 3.4), the failing variants raise remedies and the unit group follows the rows.

use crate::{check_full_steel_structure, En1993Units};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{audit_reads, evaluate_units, UnitSource};
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus};

fn find<'a>(report: &'a CheckReport, id: &str) -> &'a CheckResult {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id}"))
}

fn close(a: f64, b: f64, what: &str) {
    assert!((a - b).abs() <= 1e-9 * b.abs().max(1e-12), "{what}: {a} vs {b}");
}

fn report(doc: &En1993Snapshot) -> CheckReport {
    check_full_steel_structure(doc)
}

#[test]
fn joint_plates_and_foundation_blocks_match_the_python_oracle() {
    let r = report(&En1993Snapshot::special_parts_showcase());
    close(find(&r, "en1993.1-8.3.6.plate.plate-fin").utilization, 0.17313159394726949, "fin plate bolt group, F_b,Ed 16.29 kN against F_v,Rd 94.08 kN");
    close(find(&r, "en1993.1-8.3.10.plateblock.plate-fin").utilization, 0.09655020269893688, "fin plate block tearing, V_eff,2,Rd 310.72 kN");
    close(find(&r, "en1993.1-8.4.5.plateweld.plate-fin").utilization, 0.08193987728634466, "fin plate weld, directional method");
    close(find(&r, "en1993.1-8.3.6.plate.plate-flange").utilization, 0.6355307444352929, "flange cover bolts under F = M/(h - t_f) = 358.74 kN");
    close(find(&r, "en1993.1-1.6.2.3.platenet.plate-flange").utilization, 0.4854026416854391, "flange cover net section");
    close(find(&r, "en1993.1-8.6.2.grout.block-bp").utilization, 0.5, "grout thickness 30 mm against 0.2 min(a, b) = 60 mm");
    close(find(&r, "en1993.1-8.6.2.bearing.block-bp").utilization, 0.24265348438827505, "base plate bearing, f_jd = 40 MPa with k_j = 3");
    close(find(&r, "en1993.1-8.6.2.anchort.block-bp").utilization, 0.12295404469625433, "anchor tension T = 50 kN on two M24 8.8");
    close(find(&r, "en1993.1-8.6.2.anchorv.block-bp").utilization, 0.26772251667732794, "anchor shear without friction, alpha_b = 0.248");
    close(find(&r, "en1993.1-8.6.2.cbend.block-bp").utilization, 0.16513007058341486, "c-strip, c = 51.6 mm, required c = 8.52 mm from (17 + 2c)(240 + 2c) = 8750 mm2, t_p,req 4.95 mm of 30 mm");
    close(find(&r, "en1993.1-8.6.2.tbend.block-bp").utilization, 0.12295404469625433, "T-stub without prying, mode 3 governs at F_T,3 = 406.7 kN");
    close(find(&r, "en1993.1-8.6.2.mjrd.block-bp").utilization, 0.3592883216923917, "M_j,Rd = 167.0 kNm on the block");
    close(find(&r, "en1993.1-8.6.2.cone.block-bp").utilization, 0.1802984191225749, "cone N_Rk,c = 208.0 kN, N_Rd,c = 138.7 kN against 25 kN per anchor");
}

#[test]
fn a_thin_base_plate_fails_in_t_stub_mode_1_with_prying_and_a_short_embedment_in_the_cone() {
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.joints.iter_mut().find(|j| j.id == "joint-bp").unwrap().plate_thickness = 0.012;
    let r = report(&doc);
    let stub = find(&r, "en1993.1-8.6.2.tbend.block-bp");
    close(stub.utilization, 0.18630300320441165, "mode 1 with prying, F_T,Rd = 4 M_pl,1/m = 268.4 kN");
    assert!(stub.explanation.en.contains("mode 1 with prying"));
    doc.foundation_blocks[0].anchor_embedment = 0.05;
    let r = report(&doc);
    let cone = find(&r, "en1993.1-8.6.2.cone.block-bp");
    assert_eq!(cone.status, CheckStatus::Fail);
    assert_eq!(cone.remedies.len(), 1);
}

#[test]
fn a_plate_wider_than_the_flange_t_stub_bears_on_b_fc_plus_2c_and_a_fully_compressed_base_splits_over_both_flanges() {
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.joints.iter_mut().find(|j| j.id == "joint-bp").unwrap().plate_width = 0.45;
    let r = report(&doc);
    let bearing = find(&r, "en1993.1-8.6.2.bearing.block-bp");
    close(bearing.utilization, 0.21865450222195487, "hand value: f_jd = 37.19 MPa, b_eff = 124.0 mm, l_eff = 240 + 2 x 53.5 = 347.0 mm, F_c,Rd = 1600.7 kN against C_Ed = 350 kN");
    assert!(bearing.explanation.en.contains("l_eff=b_fc+2c=347 mm"));
    close(find(&r, "en1993.1-8.6.2.cbend.block-bp").utilization, 0.1802761683667285, "c-strip of the wide plate");
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.joints.iter_mut().find(|j| j.id == "joint-bp").unwrap().base_moment = 20_000.0;
    let r = report(&doc);
    close(find(&r, "en1993.1-8.6.2.bearing.block-bp").utilization, 0.1502140617641703, "N = -300 kN, M = 20 kNm, levers 150 mm: C_l = 83.3 kN, C_r = 216.7 kN, the right flange governs");
}

#[test]
fn the_fastening_kernel_cone_follows_the_closed_form_and_is_unchanged_without_an_edge() {
    use semio_s_plugin_norm_mechanics::fastening::{cone_breakout, ConeInput};
    let c = cone_breakout(&ConeInput::single(30.0, 400.0, 8.9, 200.0));
    close(c.n_rk_c, 207988.51250329506, "N_Rk,c with a free edge at 200 mm");
    let free = cone_breakout(&ConeInput::single(30.0, 400.0, 8.9, 0.0));
    close(free.ratio, 1.0, "no edge: A_c,N = A0_c,N");
    close(free.n_rk_c, 8.9 * 30.0_f64.sqrt() * 400.0_f64.powf(1.5), "no edge: N0_Rk,c");
}

#[test]
fn every_plate_and_block_has_its_checks_and_the_web_cover_follows_the_eccentric_group() {
    let r = report(&En1993Snapshot::special_parts_showcase());
    for id in ["plate.plate-web", "plateblock.plate-web"] {
        assert!(r.checks.iter().any(|c| c.id.ends_with(id)), "missing {id}");
    }
    assert!(r.checks.iter().all(|c| c.id != "en1993.1-1.6.2.3.platenet.plate-web" && c.id != "en1993.1-8.4.5.plateweld.plate-web"));
}

#[test]
fn failing_plates_and_blocks_carry_remedies() {
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.joint_plates[0].eccentricity = 0.8;
    doc.foundation_blocks[0].anchor_stress_area = 0.00002;
    doc.foundation_blocks[0].grout_thickness = 0.1;
    let r = report(&doc);
    let bolts = find(&r, "en1993.1-8.3.6.plate.plate-fin");
    assert_eq!(bolts.status, CheckStatus::Fail);
    assert_eq!(bolts.remedies.len(), 1);
    let anchors = find(&r, "en1993.1-8.6.2.anchort.block-bp");
    assert_eq!(anchors.status, CheckStatus::Fail);
    assert_eq!(anchors.remedies.len(), 1);
    close(find(&r, "en1993.1-8.6.2.grout.block-bp").utilization, 0.1 / 0.06, "grout thickness above 0.2 min(a, b)");
    assert!(r.checks.iter().all(|c| c.id != "en1993.1-8.6.2.bearing.block-bp"), "no bearing verdict without the beta_j = 2/3 condition");
}

#[test]
fn the_unit_fold_equals_the_full_evaluation_and_the_plate_units_declare_their_reads() {
    let doc = En1993Snapshot::special_parts_showcase();
    assert_eq!(evaluate_units::<En1993Units>(&doc), check_full_steel_structure(&doc));
    let units = <En1993Units as UnitSource>::units(&doc);
    for unit in units.iter().filter(|u| format!("{:?}", u.id).contains("jointPlates") || format!("{:?}", u.id).contains("foundationBlocks")) {
        assert_eq!(audit_reads(unit, &doc), Vec::<String>::new(), "undeclared reads of {:?}", unit.id);
    }
    assert!(units.iter().any(|u| format!("{:?}", u.id).contains("jointPlates")) && units.iter().any(|u| format!("{:?}", u.id).contains("foundationBlocks")));
}
