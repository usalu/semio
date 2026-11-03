//! 🧵️ Weld runs and free section outlines of the EN 1999 compliance tests: worked closed forms (rectangle with a hole, b_haz table), the member the runs imply, the reference checks with their remedies and a member on a free outline. See [`super`].

use super::*;
use semio_s_artifact_norm_en1999_checks::{free_constants, haz_extent, haz_overrides, outline_props, polygon_props, resolved, rings, ring_moments};
use semio_s_artifact_norm_en1999_model::{AluminiumSection, En1999Snapshot, FatigueCategory, PatchLoad, RingStiffener, SectionVertex, TransverseStiffener, WeldRun};

fn close(a: f64, b: f64, rel: f64) -> bool { (a - b).abs() <= rel * b.abs().max(1e-12) }

fn run(id: &str, member: &str, element: &str, direction: &str) -> WeldRun {
    WeldRun { id: id.into(), member_id: member.into(), element_id: element.into(), start: 0.5, length: 1.0, offset: 0.01, throat: 0.004, process: semio_s_artifact_norm_en1999_model::WeldProcess::Mig, t_junction: false, direction: direction.into(), haz_extent: 0.0 }
}

fn vertex(id: &str, section: &str, ring: &str, element: &str, y: f64, z: f64) -> SectionVertex {
    SectionVertex { id: id.into(), section_id: section.into(), ring: if ring == "hole" { semio_s_artifact_norm_en1999_model::OutlineRing::Hole } else { semio_s_artifact_norm_en1999_model::OutlineRing::Outer }, element_id: element.into(), y, z }
}

fn rectangle(section: &str, ring: &str, prefix: &str, w: f64, h: f64, element: &str) -> Vec<SectionVertex> {
    [(-w / 2.0, -h / 2.0), (w / 2.0, -h / 2.0), (w / 2.0, h / 2.0), (-w / 2.0, h / 2.0)].iter().enumerate().map(|(i, p)| vertex(&format!("{prefix}{i}"), section, ring, element, p.0, p.1)).collect()
}

#[test]
fn polygon_with_a_hole_reproduces_the_closed_form_properties() {
    let outer = [(-0.05, -0.025), (0.05, -0.025), (0.05, 0.025), (-0.05, 0.025)];
    let hole = [(-0.03, -0.01), (-0.03, 0.01), (0.03, 0.01), (0.03, -0.01)];
    let p = polygon_props(&outer, &[&hole[..]]);
    assert!(close(p.a, 0.1 * 0.05 - 0.06 * 0.02, 1e-12));
    assert!(close(p.iy, 0.1 * 0.05f64.powi(3) / 12.0 - 0.06 * 0.02f64.powi(3) / 12.0, 1e-12));
    assert!(close(p.iz, 0.05 * 0.1f64.powi(3) / 12.0 - 0.02 * 0.06f64.powi(3) / 12.0, 1e-12));
    assert!(p.yg.abs() < 1e-15 && p.zg.abs() < 1e-15 && p.iyz.abs() < 1e-18);
    assert!(close(p.wy, p.iy / 0.025, 1e-12));
    assert!(ring_moments(&hole)[0] > 0.0, "a clockwise ring is oriented to a positive area");
}

#[test]
fn a_shifted_polygon_keeps_its_centroidal_inertia() {
    let a = [(0.0, 0.0), (0.2, 0.0), (0.2, 0.1), (0.0, 0.1)];
    let p = polygon_props(&a, &[]);
    assert!(close(p.yg, 0.1, 1e-12) && close(p.zg, 0.05, 1e-12));
    assert!(close(p.iy, 0.2 * 0.1f64.powi(3) / 12.0, 1e-12));
}

#[test]
fn haz_extent_follows_the_thickness_step_the_process_and_the_junction() {
    let mut r = run("w", "m", "e", "longitudinal");
    assert!(close(haz_extent(&r, 0.004), 0.020, 1e-12));
    assert!(close(haz_extent(&r, 0.010), 0.030, 1e-12));
    r.process = semio_s_artifact_norm_en1999_model::WeldProcess::Tig;
    assert!(close(haz_extent(&r, 0.004), 0.030, 1e-12));
    r.t_junction = true;
    assert!(close(haz_extent(&r, 0.004), 0.020 * 1.5 * 1.33, 1e-12));
    r.haz_extent = 0.017;
    assert!(close(haz_extent(&r, 0.004), 0.017, 1e-12));
}

#[test]
fn runs_override_the_element_flags_and_a_transverse_run_sets_the_localized_weld() {
    let doc = En1999Snapshot::compliant_roof_purlin();
    let (member, section) = (&doc.members[0], &doc.sections[0]);
    let element = section.elements[0].id.clone();
    let mut longitudinal = run("a", &member.id, &element, "longitudinal");
    longitudinal.process = semio_s_artifact_norm_en1999_model::WeldProcess::Tig;
    let transverse = run("b", &member.id, &section.elements[1].id, "transverse");
    let (m, s) = resolved(member, section, &[&longitudinal, &transverse]);
    assert!(s.elements[0].welded && s.elements[0].weld_process == semio_s_artifact_norm_en1999_model::WeldProcess::Tig && close(s.elements[0].weld_position, 0.01, 1e-12));
    assert!(s.elements[1].welded && s.elements[1].weld_direction == semio_s_artifact_norm_en1999_model::WeldDirection::Transverse);
    assert!(m.transverse_weld && close(m.transverse_weld_offset, 0.5f64.min(member.length - 0.5), 1e-12));
    let (plain, _) = resolved(member, section, &[]);
    assert_eq!(&plain, member);
}

#[test]
fn a_transverse_run_never_lowers_the_lateral_torsional_utilisation() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    let id = doc.members[0].id.clone();
    let element = doc.sections[0].elements[0].id.clone();
    let before = evaluate_structure(&doc);
    doc.weld_runs.push(run("x", &id, &element, "transverse"));
    let after = evaluate_structure(&doc);
    let u = |r: &semio_s_plugin_norm_compliance::CheckReport| r.checks.iter().filter(|c| c.id.starts_with("en1999.6.3.") && c.subject.entity_id == id).map(|c| c.utilization).fold(0.0, f64::max);
    assert!(u(&after) >= u(&before) - 1e-12, "{} < {}", u(&after), u(&before));
    assert!(after.checks.iter().any(|c| c.id == "en1999.6.1.6.haz.x"), "the run reports its HAZ extent");
}

#[test]
fn a_haz_wider_than_its_element_fails_by_the_shared_status_rule_and_never_warns() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    let id = doc.members[0].id.clone();
    let element = doc.sections[0].elements[0].clone();
    let mut wide = run("wide", &id, &element.id, "longitudinal");
    wide.haz_extent = element.width * 0.97;
    doc.weld_runs.push(wide);
    let report = evaluate_structure(&doc);
    let c = report.checks.iter().find(|c| c.id == "en1999.6.1.6.haz.wide").expect("haz check");
    assert!(c.utilization > 1.0, "u={}", c.utilization);
    assert!(c.status == semio_s_plugin_norm_compliance::CheckStatus::Fail || c.status == semio_s_plugin_norm_compliance::CheckStatus::UnverifiedBasis, "status={:?} at u={}", c.status, c.utilization);
}

#[test]
fn a_run_with_unknown_member_element_or_geometry_fails_with_a_remedy() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    let member = doc.members[0].id.clone();
    let mut bad = run("bad", "nobody", "nothing", "diagonal");
    bad.throat = 0.0;
    doc.weld_runs.push(bad);
    let mut long = run("long", &member, &doc.sections[0].elements[0].id, "longitudinal");
    long.length = 1.0e3;
    doc.weld_runs.push(long);
    let report = evaluate_structure(&doc);
    for id in ["member", "throat"] {
        let c = report.checks.iter().find(|c| c.id == format!("en1999.ref.weldrun.{id}.bad")).unwrap_or_else(|| panic!("{id}"));
        assert_eq!(c.status, CheckStatus::Fail);
        assert!(!c.remedies.is_empty(), "{id} offers a remedy");
    }
    let geometry = report.checks.iter().find(|c| c.id == "en1999.ref.weldrun.geometry.long").expect("geometry");
    assert_eq!(geometry.status, CheckStatus::Fail);
}

#[test]
fn outline_problems_are_found_and_valid_rings_give_properties() {
    let mut v = rectangle("s", "outer", "o", 0.1, 0.05, "");
    v.extend(rectangle("s", "hole", "h", 0.06, 0.02, ""));
    let rs = rings(&v, "s");
    assert!(outline_props(&rs).is_some());
    let mut crossed = v.clone();
    crossed.swap(1, 2);
    assert!(outline_props(&rings(&crossed, "s")).is_none(), "a bow tie crosses itself");
    let mut outside = v.clone();
    outside[5].y = 0.5;
    assert!(outline_props(&rings(&outside, "s")).is_none(), "a hole may not leave the outer ring");
    assert!(outline_props(&rings(&v[4..], "s")).is_none(), "no ring is named outer");
}

#[test]
fn a_member_on_a_free_outline_gets_the_cross_section_and_buckling_checks() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    let member = doc.members[0].id.clone();
    let section = doc.sections[0].id.clone();
    doc.sections[0].kind = semio_s_artifact_norm_en1999_model::SectionKind::Free;
    let e0 = doc.sections[0].elements[0].id.clone();
    doc.outline_vertices = rectangle(&section, "outer", "v", 0.07, 0.14, &e0);
    let report = evaluate_structure(&doc);
    for prefix in ["en1999.6.2.3.n", "en1999.6.2.5.m", "en1999.6.3.1.fb"] {
        let c = report.checks.iter().find(|c| c.id == format!("{prefix}.{member}")).unwrap_or_else(|| panic!("{prefix}"));
        assert!(c.utilization.is_finite());
    }
    assert!(report.checks.iter().any(|c| c.id == format!("en1999.6.1.outline.props.{section}")));
    assert!(report.checks.iter().any(|c| c.id == format!("en1999.6.free.scope.{member}") && c.status == CheckStatus::NotApplicable));
    doc.outline_vertices.clear();
    let empty = evaluate_structure(&doc);
    assert_eq!(empty.checks.iter().find(|c| c.id == format!("en1999.ref.outline.{section}")).map(|c| c.status), Some(CheckStatus::Fail));
}

#[test]
fn an_outline_that_matches_the_parametric_section_reports_no_warning() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    let s = doc.sections[0].clone();
    let b = s.width;
    let h = s.height;
    let (tf, tw) = (s.flange_thickness, s.web_thickness);
    let pts = [(-b / 2.0, -h / 2.0), (b / 2.0, -h / 2.0), (b / 2.0, -h / 2.0 + tf), (tw / 2.0, -h / 2.0 + tf), (tw / 2.0, h / 2.0 - tf), (b / 2.0, h / 2.0 - tf), (b / 2.0, h / 2.0), (-b / 2.0, h / 2.0), (-b / 2.0, h / 2.0 - tf), (-tw / 2.0, h / 2.0 - tf), (-tw / 2.0, -h / 2.0 + tf), (-b / 2.0, -h / 2.0 + tf)];
    doc.outline_vertices = pts.iter().enumerate().map(|(i, p)| vertex(&format!("p{i}"), &s.id, "outer", "", p.0, p.1)).collect();
    let report = evaluate_structure(&doc);
    let c = report.checks.iter().find(|c| c.id == format!("en1999.6.1.outline.match.{}", s.id));
    if let Some(c) = c {
        assert_ne!(c.status, CheckStatus::Warning, "{}", c.utilization);
    }
}

#[test]
fn a_run_haz_extent_override_scales_the_haz_strip_of_the_section() {
    let doc = En1999Snapshot::compliant_roof_purlin();
    let (member, section) = (&doc.members[0], &doc.sections[0]);
    let alloy = part_1_1::resolve_alloy_for_section(&doc.materials[0].designation, section).expect("alloy");
    assert!(alloy.rho_o_haz < 1.0);
    let mut weld = run("h", &member.id, &section.elements[0].id, "longitudinal");
    weld.offset = 0.0;
    let (_, s) = resolved(member, section, &[&weld]);
    let element = &s.elements[0];
    let table = part_1_1::haz_width_for(element);
    assert!(table < element.width, "the table strip lies inside the element");
    let gross = part_1_1::props(&s, &part_1_1::EffCtx::gross(alloy.f_o_pa, alloy.buckling_class_a)).a;
    let loss = |extents: &[(String, f64)]| gross - part_1_1::props(&s, &part_1_1::EffCtx { f_o_pa: alloy.f_o_pa, class_a: alloy.buckling_class_a, eta_web: 1.0, local: false, haz_rho: Some(alloy.rho_o_haz), haz_extents: extents }).a;
    let id = element.id.clone();
    let at_table = loss(&[]);
    assert!(at_table > 0.0);
    assert!(close(loss(&[(id.clone(), table)]), at_table, 1e-9), "an override equal to the table changes nothing");
    assert!(close(loss(&[(id.clone(), 0.5 * table)]), 0.5 * at_table, 1e-9), "the strip loss is linear in b_haz up to the element width");
    assert!(close(loss(&[(id.clone(), 10.0)]), at_table * element.width / table, 1e-9), "a wide HAZ stops at the element width");
    assert!(close(loss(&[("other".into(), 10.0)]), at_table, 1e-9), "an override of another element is ignored");
}

#[test]
fn haz_overrides_take_the_widest_positive_extent_per_element() {
    let mut a = run("a", "m", "web", "longitudinal");
    a.haz_extent = 0.02;
    let mut b = run("b", "m", "web", "longitudinal");
    b.haz_extent = 0.05;
    let c = run("c", "m", "flange-top", "longitudinal");
    let overrides = haz_overrides(&[&a, &b, &c]);
    assert_eq!(overrides, vec![("web".to_string(), 0.05)]);
    assert!(haz_overrides(&[]).is_empty());
}

#[test]
fn a_wider_run_haz_extent_never_lowers_the_member_utilisation() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    let id = doc.members[0].id.clone();
    let element = doc.sections[0].elements[0].id.clone();
    let mut weld = run("w", &id, &element, "longitudinal");
    weld.offset = 0.0;
    doc.weld_runs.push(weld);
    let narrow = evaluate_structure(&doc);
    doc.weld_runs[0].haz_extent = 10.0;
    let wide = evaluate_structure(&doc);
    let u = |r: &semio_s_plugin_norm_compliance::CheckReport, prefix: &str| r.checks.iter().filter(|c| c.id.starts_with(prefix) && c.subject.entity_id == id).map(|c| c.utilization).fold(0.0, f64::max);
    for prefix in ["en1999.6.2.", "en1999.6.3."] {
        assert!(u(&wide, prefix) >= u(&narrow, prefix) - 1e-12, "{prefix}: {} < {}", u(&wide, prefix), u(&narrow, prefix));
    }
    assert!(u(&wide, "en1999.6.2.") > u(&narrow, "en1999.6.2."), "the widened HAZ reaches the cross-section resistance");
}

fn i_polygon(section: &str, b: f64, h: f64, tf: f64, tw: f64) -> Vec<SectionVertex> {
    let points = [(-b / 2.0, -h / 2.0), (b / 2.0, -h / 2.0), (b / 2.0, -h / 2.0 + tf), (tw / 2.0, -h / 2.0 + tf), (tw / 2.0, h / 2.0 - tf), (b / 2.0, h / 2.0 - tf), (b / 2.0, h / 2.0), (-b / 2.0, h / 2.0), (-b / 2.0, h / 2.0 - tf), (-tw / 2.0, h / 2.0 - tf), (-tw / 2.0, -h / 2.0 + tf), (-b / 2.0, -h / 2.0 + tf)];
    let names = ["flange-bot", "", "", "web", "", "", "flange-top", "", "", "", "", ""];
    points.iter().zip(names).enumerate().map(|(i, (p, n))| vertex(&format!("i{i}"), section, "outer", n, p.0, p.1)).collect()
}

fn constants_of(vertices: &[SectionVertex]) -> semio_s_artifact_norm_en1999_checks::FreeConstants {
    let rs = rings(vertices, "s");
    let props = outline_props(&rs).expect("valid outline");
    let section = AluminiumSection { id: "s".into(), kind: semio_s_artifact_norm_en1999_model::SectionKind::Free, height: 0.0, width: 0.0, flange_thickness: 0.0, web_thickness: 0.0, outer_diameter: 0.0, elements: En1999Snapshot::compliant_roof_purlin().sections[0].elements.clone() };
    free_constants(&section, &rs, &props).expect("constants")
}

#[test]
fn free_outline_constants_reproduce_the_sectionproperties_values() {
    let cases: [(&str, Vec<SectionVertex>, f64, f64); 3] = [
        ("solid 70x140", rectangle("s", "outer", "o", 0.07, 0.14, ""), 1.0981e-5, 2.3909e-9),
        ("box 100x60x4", { let mut v = rectangle("s", "outer", "o", 0.1, 0.06, ""); v.extend(rectangle("s", "hole", "h", 0.092, 0.052, "")); v }, 1.5619e-6, 5.7316e-11),
        ("I 100x120x8x6", i_polygon("s", 0.1, 0.12, 0.008, 0.006), 4.1243e-8, 4.1705e-9),
    ];
    for (name, vertices, it, iw) in cases {
        let k = constants_of(&vertices);
        assert!(close(k.it, it, 0.04), "{name}: I_t {} vs {it}", k.it);
        assert!(close(k.iw, iw, 0.08), "{name}: I_w {} vs {iw}", k.iw);
    }
}

#[test]
fn shear_areas_follow_the_named_plate_edges_and_default_to_the_gross_area() {
    let k = constants_of(&i_polygon("s", 0.1, 0.12, 0.008, 0.006));
    let hw = 0.12 - 2.0 * 0.008;
    assert!(close(k.a_vz, hw * 0.008, 1e-9) || close(k.a_vz, hw * doc_web_thickness(), 1e-9), "{}", k.a_vz);
    assert!(close(k.a_vy, 2.0 * 0.1 * doc_flange_thickness(), 1e-9), "{}", k.a_vy);
    let solid = constants_of(&rectangle("s", "outer", "o", 0.07, 0.14, ""));
    assert!(close(solid.a_vz, 0.07 * 0.14, 1e-9) && close(solid.a_vy, 0.07 * 0.14, 1e-9), "no named edge: gross area");
    assert!(close(solid.t_max, 0.07, 1e-9), "no named element: the shorter bounding-box side");
}

fn doc_web_thickness() -> f64 { En1999Snapshot::compliant_roof_purlin().sections[0].elements[2].thickness }

fn doc_flange_thickness() -> f64 { En1999Snapshot::compliant_roof_purlin().sections[0].elements[0].thickness }

#[test]
fn a_member_on_a_free_i_outline_gets_shear_torsion_and_ltb_like_the_parametric_section() {
    let mut parametric = En1999Snapshot::compliant_roof_purlin();
    parametric.members[0].actions[0].torsion_k = 800.0;
    let section = parametric.sections[0].clone();
    let member = parametric.members[0].id.clone();
    let mut free = parametric.clone();
    free.sections[0].kind = semio_s_artifact_norm_en1999_model::SectionKind::Free;
    free.outline_vertices = i_polygon(&section.id, section.width, section.height, section.flange_thickness, section.web_thickness);
    let (a, b) = (evaluate_structure(&parametric), evaluate_structure(&free));
    let u = |r: &semio_s_plugin_norm_compliance::CheckReport, id: &str| r.checks.iter().find(|c| c.id == format!("{id}.{member}")).unwrap_or_else(|| panic!("{id}")).utilization;
    for (id, rel) in [("en1999.6.2.6.v", 0.03), ("en1999.6.2.7.t", 0.12)] {
        assert!(close(u(&b, id), u(&a, id), rel), "{id}: free {} vs parametric {}", u(&b, id), u(&a, id));
    }
    let (free_ltb, parametric_ltb) = (u(&b, "en1999.6.3.2.ltb"), u(&a, "en1999.6.3.2.ltb"));
    assert!(free_ltb > 0.0 && free_ltb < 1.0e3 && free_ltb / parametric_ltb > 0.6 && free_ltb / parametric_ltb < 1.6, "{free_ltb} vs {parametric_ltb}");
    assert!(b.checks.iter().any(|c| c.id == format!("en1999.6.1.outline.torsion.{}", section.id)));
}

fn wave5_document() -> En1999Snapshot {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    let member = doc.members[0].id.clone();
    doc.members[0].actions[0].local_load_k = 5_000.0;
    doc.weld_runs.push(run("w1", &member, &doc.sections[0].elements[0].id, "longitudinal"));
    doc.fatigue_categories.push(FatigueCategory { id: "site-1".into(), member_id: member.clone(), element_id: String::new(), detail_category: "71".into(), detail_type: semio_s_artifact_norm_en1999_model::DetailType::ButtWeld, location_x: 3.0, m1: 0.0, m2: 0.0 });
    doc.patch_loads.push(PatchLoad { id: "patch-member".into(), subject_id: member, position: semio_s_artifact_norm_en1999_model::PatchLoadPosition::Interior, x: 3.0, bearing_length: 0.05, web_height: 0.0, web_angle: 0.0, corner_radius: 0.0, load: 10_000.0 });
    doc.patch_loads.push(PatchLoad { id: "patch-sheet".into(), subject_id: "sheet-pass".into(), position: semio_s_artifact_norm_en1999_model::PatchLoadPosition::End, x: 0.0, bearing_length: 0.05, web_height: 0.0, web_angle: 0.0, corner_radius: 0.004, load: 1_000.0 });
    doc.ring_stiffeners.push(RingStiffener { id: "ring-1".into(), shell_id: "shell-pass".into(), x: 1.0, depth: 0.05, thickness: 0.006, internal: false });
    doc.transverse_stiffeners.push(TransverseStiffener { id: "stiffener-1".into(), girder_id: doc.members[0].id.clone(), position: 3.0, width: 0.05, thickness: 0.006, sides: semio_s_artifact_norm_en1999_model::StiffenerSides::Two });
    let element = &mut doc.sections[0].elements[0];
    element.outstand = true;
    element.width = element.width.max(0.05);
    element.edge_stiffener_depth = 0.02;
    element.edge_stiffener_thickness = element.thickness.max(0.003);
    doc
}

#[test]
fn every_spec_id_is_traced_to_a_check_family_an_example_emits() {
    use semio_s_plugin_norm_compliance::spec_trace::{assert_traced, Emitted};
    let mut pinned = En1999Snapshot::compliant_roof_purlin();
    pinned.connections[0].kind = semio_s_artifact_norm_en1999_model::ConnectionKind::Pinned;
    let mut free = En1999Snapshot::compliant_roof_purlin();
    let section = free.sections[0].clone();
    free.sections[0].kind = semio_s_artifact_norm_en1999_model::SectionKind::Free;
    free.outline_vertices = i_polygon(&section.id, section.width, section.height, section.flange_thickness, section.web_thickness);
    let mut welded = En1999Snapshot::compliant_roof_purlin();
    welded.members[0].transverse_weld = true;
    welded.members[0].transverse_weld_offset = 0.75;
    let emitted = [En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::noncompliant_multi_fail(), pinned, free, welded, wave5_document(), En1999Snapshot::full_showcase()]
        .iter()
        .fold(Emitted::default(), |emitted, doc| emitted.with_checks(evaluate_structure(doc).checks.into_iter().map(|c| c.id)));
    assert_traced(&semio_s_artifact_norm_en1999_checks::spec_trace::TABLE, &emitted);
}

#[test]
fn wave5_rows_are_checked_with_their_own_subjects_and_unverified_coefficients_stay_unevaluated() {
    let report = evaluate_structure(&wave5_document());
    let by = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{id}"));
    let category = by("en1999.1-3.fat.category.site-1");
    assert_eq!(category.subject.entity_id, "site-1");
    assert!(category.utilization > 0.0);
    assert_eq!(by("en1999.1-4.web-crippling.patch-sheet").status, CheckStatus::UnverifiedBasis);
    assert_eq!(by("en1999.6.7.5.web.patch-member").subject.entity_id, "patch-member");
    let stiffener = by("en1999.6.7.6.web-stiffness.stiffener-1");
    assert_eq!(stiffener.subject.entity_id, "stiffener-1");
    assert!(matches!(stiffener.status, CheckStatus::UnverifiedBasis | CheckStatus::Fail), "{:?}", stiffener.status);
    let strength = by("en1999.6.7.6.web-strength.stiffener-1");
    assert!(matches!(strength.status, CheckStatus::UnverifiedBasis | CheckStatus::Fail), "{:?}", strength.status);
    assert!(strength.explanation.en.contains("ndp.en1999.web-stiffener") || strength.status == CheckStatus::Fail, "{}", strength.explanation.en);
    let ring = by("en1999.1-5.stiffener.ring-1");
    assert!(matches!(ring.status, CheckStatus::UnverifiedBasis | CheckStatus::Fail), "{:?}", ring.status);
    assert!(ring.explanation.en.contains("awaiting verified margin"), "{}", ring.explanation.en);
    let member = report.checks.iter().find(|c| c.id.starts_with("en1999.6.7.7.edge.")).expect("the edge stiffener flexural check is emitted");
    assert_eq!(member.subject.entity_id, wave5_document().members[0].id);
    assert!(member.utilization > 0.0 && member.explanation.en.contains("σ_cr,s"), "{}", member.explanation.en);
    assert!(by("en1999.1-4.web-crippling.patch-sheet").explanation.en.contains("Awaiting verified coefficients"));
    assert!(by("en1999.1-4.web-crippling.patch-sheet").explanation.de.contains("Wartet auf geprüfte Beiwerte") && by("en1999.1-4.web-crippling.patch-sheet").explanation.en.contains("6.1.7") && by("en1999.1-4.web-crippling.patch-sheet").explanation.de.contains("6.1.7"));
    assert!(report.checks.iter().all(|c| c.id != "en1999.1-3.fat.miner.fat-1"), "the category check replaces the plain spectrum check of its group");
}

const SHOWCASE_UNREACHED: &[(&str, &str)] = &[];

#[test]
fn the_full_showcase_alone_reaches_every_check_fragment_of_the_trace() {
    use semio_s_plugin_norm_compliance::spec_trace::{assert_reaches_all_but, Emitted};
    let showcase = En1999Snapshot::full_showcase();
    for (collection, rows) in [("materials", showcase.materials.len()), ("sections", showcase.sections.len()), ("members", showcase.members.len()), ("connections", showcase.connections.len()), ("fireScenarios", showcase.fire_scenarios.len()), ("fatigueDetails", showcase.fatigue_details.len()), ("coldFormed", showcase.cold_formed.len()), ("shells", showcase.shells.len()), ("placements", showcase.placements.len()), ("weldRuns", showcase.weld_runs.len()), ("outlineVertices", showcase.outline_vertices.len()), ("fatigueCategories", showcase.fatigue_categories.len()), ("patchLoads", showcase.patch_loads.len()), ("ringStiffeners", showcase.ring_stiffeners.len()), ("transverseStiffeners", showcase.transverse_stiffeners.len())] {
        assert!(rows > 0, "the showcase leaves {collection} empty");
    }
    let emitted = Emitted::default().with_checks(evaluate_structure(&showcase).checks.into_iter().map(|c| c.id));
    assert_reaches_all_but(&semio_s_artifact_norm_en1999_checks::spec_trace::TABLE, &emitted, SHOWCASE_UNREACHED);
}
