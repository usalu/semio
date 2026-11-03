//! 🎯️ References of the wave-5 rules, asserted against `🧰️scripts/en1999-wave5/wave5_ref.py` (sectionproperties and shapely for the ring section, scipy for the S-N damage). See [`super`].

use super::*;
use semio_s_artifact_norm_en1999_model::{FatigueCategory, FatigueDetail};

fn close(a: f64, b: f64, rel: f64) -> bool { (a - b).abs() <= rel * b.abs().max(1e-300) }

#[test]
fn the_ring_section_reproduces_the_sectionproperties_composite() {
    let (a, i, z) = part_1_4_1_5::ring_section(0.010, 0.78 * (0.5f64 * 0.010).sqrt(), 0.060, 0.006);
    assert!(close(a, 9.11543289325507e-4, 1e-9) && close(i, 3.794300279075092e-7, 1e-9) && close(z, 0.013822711600809787, 1e-9), "{a} {i} {z}");
}

#[test]
fn ring_bays_split_the_shell_at_the_rings_inside_its_length() {
    assert_eq!(part_1_4_1_5::ring_bays(2.0, &[]), vec![2.0]);
    assert_eq!(part_1_4_1_5::ring_bays(2.0, &[1.5, 0.5, 0.0, 2.0, 3.0, 0.5]), vec![0.5, 1.0, 0.5]);
}

#[test]
fn the_transverse_web_load_reproduces_the_scipy_reference() {
    let t = part_1_4_1_5::transverse_load_resistance(0.120, 0.006, 0.010, 0.080, 260.0e6, 0.05, 0.0, [6.0, 2.0, 0.9, 0.5, 0.02], 1.1);
    assert!(close(t.k_f, 6.0, 1e-12) && close(t.f_cr, 680_400.0, 1e-12));
    assert!(close(t.l_y, 0.15053156730955466, 1e-9) && close(t.lambda_f, 0.5874811528402776, 1e-9) && close(t.chi_f, 0.8510911330221658, 1e-9));
    assert!(close(t.l_eff, 0.12811608217709128, 1e-9) && close(t.f_rd, 181_691.89836023856, 1e-9), "{t:?}");
}

fn block(id: &str, ds: f64, n: f64) -> FatigueDetail {
    FatigueDetail { id: id.into(), member_id: "m".into(), detail_category: "71".into(), delta_sigma_c: 0.0, delta_sigma_ed: ds, n_cycles: n, m1: 4.3, m2: 6.3 }
}

fn site(m1: f64, m2: f64) -> FatigueCategory {
    FatigueCategory { id: "site".into(), member_id: "m".into(), element_id: String::new(), detail_category: "71".into(), detail_type: semio_s_artifact_norm_en1999_model::DetailType::ButtWeld, location_x: 1.0, m1, m2 }
}

#[test]
fn the_category_damage_reproduces_the_scipy_miner_sum() {
    let details = vec![block("a", 120.0e6, 1.0e6), block("b", 70.0e6, 3.0e6), block("c", 40.0e6, 5.0e6)];
    let d = part_1_3::category_damage(&site(0.0, 0.0), &details, 1.0, 1.0).expect("damage");
    assert!(close(d.total, 6.290016393079678, 1e-9) && close(d.m1, 4.3, 1e-12) && close(d.m2, 6.3, 1e-12), "{d:?}");
    assert_eq!(d.governing, 0);
    let steeper = part_1_3::category_damage(&site(5.0, 0.0), &details, 1.0, 1.0).expect("damage");
    assert!(close(steeper.m2, 7.0, 1e-12) && steeper.total != d.total, "the site slopes override the block slopes, m2 defaults to m1 + 2");
    assert_eq!(part_1_3::category_blocks(&site(0.0, 0.0), &details).len(), 3);
}

#[test]
fn a_group_with_a_category_row_is_recognised_by_member_and_detail_category() {
    let details = vec![block("a", 50.0e6, 1.0e6), FatigueDetail { member_id: "other".into(), ..block("b", 50.0e6, 1.0e6) }];
    assert!(part_1_3::group_has_category(&[0], &details, &[site(0.0, 0.0)]));
    assert!(!part_1_3::group_has_category(&[1], &details, &[site(0.0, 0.0)]));
}

#[test]
fn the_edge_stiffener_strut_reproduces_the_scipy_winkler_eigenvalue() {
    let k = part_1_4_1_5::edge_stiffener_spring(0.003, 0.05);
    let (a, i) = part_1_4_1_5::edge_stiffener_section(0.02, 0.004);
    assert!(close(k, 4_153_846.153846153, 1e-9) && close(a, 8.0e-5, 1e-12) && close(i, 2.6666666666666674e-9, 1e-12), "{k} {a} {i}");
    let sigma = part_1_4_1_5::edge_stiffener_sigma_cr(k, i, a);
    assert!(close(sigma, 696_143_221.3383856, 1e-9) && close(sigma, 696_370_537.1255653, 1e-3), "{sigma}");
    assert!(close(part_1_4_1_5::edge_stiffener_utilization(240.0e6, sigma, 1.0), 240.0e6 / sigma, 1e-12));
}

#[test]
fn the_ring_line_load_and_tributary_follow_the_closed_forms() {
    assert!(close(part_1_4_1_5::ring_critical_line_load(2.5e-8, 0.5), 42_000.0, 1e-12));
    assert!(close(part_1_4_1_5::ring_tributary(4.0, &[1.0, 3.0], 1.0), 1.5, 1e-12));
    assert!(close(part_1_4_1_5::ring_tributary(4.0, &[1.0, 3.0], 3.0), 1.5, 1e-12));
    assert!(close(part_1_4_1_5::ring_tributary(4.0, &[2.0], 2.0), 2.0, 1e-12));
}

#[test]
fn the_effective_web_stiffener_section_reproduces_the_sectionproperties_union() {
    let strip = 2.0 * 15.0 * (250.0f64 / 260.0).sqrt() * 0.008 + 0.006;
    let half = 15.0 * (250.0f64 / 260.0).sqrt() * 0.008;
    let two = web_stiffener::effective_section(0.05, 0.006, true, 0.008, half);
    assert!(close(two.strip, strip, 1e-12) && close(two.area, 0.00253071489732656, 1e-9) && close(two.inertia, 6.398971461190749e-07, 1e-9) && two.centroid == 0.0, "{two:?}");
    let one = web_stiffener::effective_section(0.05, 0.006, false, 0.008, half);
    assert!(close(one.area, 0.002230714897326559, 1e-9) && close(one.inertia, 2.911663196872829e-07, 1e-9) && close(one.centroid, 0.0039000949921599937, 1e-9), "{one:?}");
    let k = web_stiffener::Constants { strip: 15.0, short: 1.5, long: 0.75, aspect: 2.0f64.sqrt(), length: 0.75 };
    assert!(close(web_stiffener::required_inertia(0.12, 0.008, 2.0, &k), 4.608e-8, 1e-12), "a/h_w above the aspect limit: 0.75·h_w·t_w³");
    assert!(close(web_stiffener::required_inertia(0.12, 0.008, 0.12, &k), 9.216e-8, 1e-12), "a/h_w below the aspect limit: 1.5·h_w³·t_w³/a²");
    assert_eq!(web_stiffener::panels(6.0, &[2.0, 4.0], 2.0), (2.0, 2.0));
    assert_eq!(web_stiffener::panels(6.0, &[4.0], 4.0), (4.0, 2.0));
}

#[test]
fn transverse_web_stiffeners_reproduce_the_independent_oracle_and_offer_remedies() {
    use semio_s_artifact_norm_en1999_model::{En1999Snapshot, TransverseStiffener};
    use semio_s_plugin_norm_compliance::CheckStatus;
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    let girder = doc.members[0].id.clone();
    doc.members[0].actions[0].local_load_k = 40_000.0;
    let row = |id: &str, position: f64, width: f64, thickness: f64, sides: &str| TransverseStiffener { id: id.into(), girder_id: girder.clone(), position, width, thickness, sides: if sides == "one" { semio_s_artifact_norm_en1999_model::StiffenerSides::One } else { semio_s_artifact_norm_en1999_model::StiffenerSides::Two } };
    doc.transverse_stiffeners = vec![row("stiffener-1", 2.0, 0.05, 0.006, "two"), row("stiffener-thin", 4.0, 0.01, 0.001, "one")];
    let report = evaluate_structure(&doc);
    let by = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{id}"));
    for (id, expected) in [("en1999.6.7.6.web-stiffness.stiffener-1", 0.07201157292147886), ("en1999.6.7.6.web-strength.stiffener-1", 0.0904545636540384), ("en1999.6.7.6.web-stiffness.stiffener-thin", 4.19944270155647), ("en1999.6.7.6.web-strength.stiffener-thin", 0.14674344070647724)] {
        assert!(close(by(id).utilization, expected, 1e-6), "{id}: {} vs {expected}", by(id).utilization);
    }
    let thin = by("en1999.6.7.6.web-stiffness.stiffener-thin");
    assert_eq!(thin.status, CheckStatus::Fail);
    assert_eq!(thin.subject.entity_id, "stiffener-thin");
    assert!(thin.remedies.iter().any(|r| r.target.path == "transverseStiffeners[id=stiffener-thin].width") && thin.remedies.iter().any(|r| r.target.path == "transverseStiffeners[id=stiffener-thin].thickness"), "{:?}", thin.remedies);
    assert!(thin.explanation.en.contains("unverified registry rows") && thin.ndps.iter().any(|n| n == "ndp.en1999.web-stiffener.strip-factor"));
    assert_ne!(by("en1999.6.7.6.web-stiffness.stiffener-1").status, CheckStatus::Fail);
    doc.transverse_stiffeners[0].position = 9.0;
    doc.transverse_stiffeners[1].girder_id = "nobody".into();
    let broken = evaluate_structure(&doc);
    assert_eq!(broken.checks.iter().find(|c| c.id == "en1999.6.7.6.web-position.stiffener-1").map(|c| c.status), Some(CheckStatus::Fail));
    assert_eq!(broken.checks.iter().find(|c| c.id == "en1999.6.7.6.web-ref.stiffener-thin").map(|c| c.status), Some(CheckStatus::Fail));
}
