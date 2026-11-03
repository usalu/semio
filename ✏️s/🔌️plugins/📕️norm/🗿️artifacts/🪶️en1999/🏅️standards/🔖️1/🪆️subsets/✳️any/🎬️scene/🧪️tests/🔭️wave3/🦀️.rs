use super::*;
use scene::Validate;

fn compliant() -> En1999Snapshot {
    En1999Snapshot::compliant_roof_purlin()
}

fn find(document: &En1999Snapshot, report: &CheckReport, focus: &str, id: &str) -> scene::Diagram2d {
    diagrams(document, report, Some(focus)).into_iter().find(|d| d.id == id).unwrap_or_else(|| panic!("missing diagram {id}"))
}

fn check<'a>(report: &'a CheckReport, id: &str) -> &'a norm::CheckResult {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id}"))
}

fn polyline_count(diagram: &scene::Diagram2d, prefix: &str) -> usize {
    diagram.series.iter().filter(|s| matches!(s, scene::Series::Polyline { id, .. } if id.starts_with(prefix))).count()
}

pub(super) fn oracle_extras(document: &En1999Snapshot, all: &[scene::Diagram2d]) -> String {
    let report = checks::evaluate_structure(document);
    let params = checks::na_de::AnnexParams::for_choice(document.annex);
    let connections: Vec<String> = document
        .connections
        .iter()
        .filter_map(|c| {
            let material = document.materials.iter().find(|m| m.id == c.material_id)?;
            let band = |a: f64| checks::part_1_1::resolve_alloy_for_thickness(&material.designation, c.bolts.plate_thickness.max(a / 0.7));
            let (plate, at) = (band(0.0)?, band(c.welds.throat)?);
            let b = &c.bolts;
            Some(format!(
                "{{\"id\":\"{}\",\"d\":{},\"e\":{},\"p1\":{},\"p2\":{},\"rows\":{},\"perRow\":{},\"ecc\":{},\"method\":\"{}\",\"mu\":{},\"exponent\":{},\"clear\":{},\"filler\":\"{}\",\"betaW\":{},\"gammaMw\":{},\"gammaM2\":{},\"plateT\":{},\"fu\":{},\"rhoHaz\":{},\"throat\":{},\"fuAt\":{},\"rhoAt\":{}}}",
                c.id, b.diameter, b.edge_distance, b.pitch, b.gauge, b.rows, b.bolts_per_row, c.load_eccentricity, c.force_method.code(), semio_s_plugin_norm_compliance::ndp_common(checks::joints::IC_CURVE_MU_ROW), semio_s_plugin_norm_compliance::ndp_common(checks::joints::IC_CURVE_EXPONENT_ROW), b.edge_distance >= checks::joints::hole_diameter(b.diameter) / 2.0, c.welds.filler_alloy, c.welds.beta_w, params.gamma_mw, params.gamma_m2, b.plate_thickness, plate.f_u_pa, plate.rho_u_haz, c.welds.throat, at.f_u_pa, at.rho_u_haz
            ))
        })
        .collect();
    let sheets: Vec<String> = document
        .cold_formed
        .iter()
        .filter_map(|s| {
            let material = document.materials.iter().find(|m| m.id == s.material_id)?;
            let alloy = checks::part_1_1::resolve_alloy_for_thickness(&material.designation, s.thickness)?;
            let demand = |suffix: &str| report.checks.iter().find(|c| c.id == format!("en1999.1-4.{suffix}.{}", s.id)).map_or(0.0, |c| c.computed.value.abs());
            let samples: Vec<String> = all
                .iter()
                .find(|d| d.id == format!("sheet-resistance:{}", s.id))
                .map(|d| d.table_twin.columns[0].values.iter().filter_map(|t| checks::part_1_1::resolve_alloy_for_thickness(&material.designation, *t)).map(|a| format!("{{\"fo\":{},\"classA\":{},\"rhoHaz\":{},\"eps\":{}}}", a.f_o_pa, a.buckling_class_a, a.rho_o_haz, checks::part_1_1::epsilon(a.f_o_pa))).collect())
                .unwrap_or_default();
            Some(format!(
                "{{\"id\":\"{}\",\"width\":{},\"thickness\":{},\"welded\":{},\"classA\":{},\"eps\":{},\"gammaM1\":{},\"nEd\":{},\"mEd\":{},\"vEd\":{},\"samples\":[{}]}}",
                s.id, s.width, s.thickness, s.welded, alloy.buckling_class_a, checks::part_1_1::epsilon(alloy.f_o_pa), params.gamma_m1, demand("axial"), demand("bend"), demand("shear"), samples.join(",")
            ))
        })
        .collect();
    let tf_of = |m: &model::AluminiumMember, section: &model::AluminiumSection, tag: &str| {
        let system = eigen::torsional_flexural_system(m, section)?;
        let (n, x) = system.lowest_twisting()?;
        let (a, iy, iz, _, _, it, iw) = checks::part_1_1::section_geometry(section);
        Some(format!(
            "{{\"member\":\"{}{tag}\",\"section\":\"{}\",\"sectionData\":{{\"kind\":\"{}\",\"h\":{},\"b\":{},\"tw\":{},\"tf\":{}}},\"length\":{},\"factor\":{n},\"mode\":[{},{},{}],\"nu\":{},\"nv\":{},\"nt\":{},\"i0Sq\":{},\"y0\":{},\"z0\":{},\"constants\":{{\"a\":{a},\"iy\":{iy},\"iz\":{iz},\"it\":{it},\"iw\":{iw}}}}}",
            m.id, section.id, section.kind, section.height, section.width, section.web_thickness, section.flange_thickness, m.length, x[0], x[1], x[2], system.nu, system.nv, system.nt, system.i0_sq, system.y0, system.z0
        ))
    };
    let mut tf: Vec<String> = document
        .members
        .iter()
        .filter_map(|m| tf_of(m, document.sections.iter().find(|s| s.id == m.section_id)?, ""))
        .collect();
    tf.extend(document.members.iter().filter_map(|m| {
        let section = document.sections.iter().find(|s| s.id == m.section_id)?;
        (section.kind == model::SectionKind::ExtrudedI).then(|| tf_of(&model::AluminiumMember { support: model::SupportCondition::SimplySupported, length: 6.0, ..m.clone() }, &model::AluminiumSection { kind: model::SectionKind::Channel, ..section.clone() }, ":channel")).flatten()
    }));
    let shell_modes: Vec<String> = document
        .shells
        .iter()
        .filter_map(|shell| {
            let mut cuts: Vec<f64> = document.ring_stiffeners.iter().filter(|r| r.shell_id == shell.id && r.x > 0.0 && r.x < shell.length).map(|r| r.x).collect();
            cuts.sort_by(f64::total_cmp);
            let edges: Vec<f64> = std::iter::once(0.0).chain(cuts).chain(std::iter::once(shell.length)).collect();
            let bay = edges.windows(2).map(|w| (w[0], w[1])).fold((0.0, 0.0), |best, b| if b.1 - b.0 > best.1 - best.0 + 1e-12 { b } else { best });
            let mode = eigen::shell_axial(shell.radius, shell.thickness, shell.length, bay)?;
            let samples: Vec<String> = mode.samples.iter().map(|s| format!("[{},{},{},{},{}]", s.at.y, (s.at.x / (2.0 * shell.radius * (std::f64::consts::PI / 48.0).sin())).round(), s.delta.x, s.delta.y, s.delta.z)).collect();
            Some(format!("{{\"id\":\"{}\",\"r\":{},\"t\":{},\"halfWave\":{},\"waves\":{},\"window\":[{},{}],\"samples\":[{}]}}", shell.id, shell.radius, shell.thickness, mode.half_wave, mode.waves, mode.window.0, mode.window.1, samples.join(",")))
        })
        .collect();
    let buckling: Vec<String> = document
        .members
        .iter()
        .filter_map(|m| {
            let (section, alloy) = facts::member_parts(document, m).filter(|(s, _)| !checks::is_free(s))?;
            if !all.iter().any(|d| d.id == format!("torsional:{}", m.id)) {
                return None;
            }
            let haz = checks::haz_overrides(&checks::runs_of(&document.weld_runs, &m.id));
            let base = checks::part_1_1::EffCtx { f_o_pa: alloy.f_o_pa, class_a: alloy.buckling_class_a, eta_web: facts::eta_of(&report, &m.id, section, &alloy), local: true, haz_rho: None, haz_extents: &haz };
            let a_loc = checks::part_1_1::props(section, &base).a;
            let k0 = if section.elements.iter().any(|e| e.welded) && a_loc > 0.0 { checks::part_1_1::props(section, &checks::part_1_1::EffCtx { haz_rho: Some(alloy.rho_o_haz), ..base }).a / a_loc } else { 1.0 };
            let (alpha, lam0) = checks::part_1_1::buckling_curve(alloy.buckling_class_a);
            let (alpha_t, lam0_t) = checks::part_1_1::torsional_curve(alloy.buckling_class_a);
            let (_, i_y, i_z, _, _, i_t, i_w) = checks::part_1_1::section_geometry(section);
            let gross = checks::part_1_1::props(section, &checks::part_1_1::EffCtx::gross(alloy.f_o_pa, alloy.buckling_class_a));
            let (y0, z0) = checks::part_1_1::shear_centre_offset(section);
            let i0_sq = (i_y + i_z) / gross.a + y0 * y0 + z0 * z0;
            let k_support = match m.support {
                model::SupportCondition::Continuous => 0.7,
                model::SupportCondition::Cantilever => 2.0,
                model::SupportCondition::SimplySupported => 1.0,
            };
            let l_cr = if m.buckling_length_t > 0.0 { m.buckling_length_t } else { k_support * m.length };
            let g1 = checks::na_de::AnnexParams::for_choice(document.annex).gamma_m1;
            Some(format!(
                "{{\"id\":\"{}\",\"alpha\":{alpha},\"lam0\":{lam0},\"alphaT\":{alpha_t},\"lam0T\":{lam0_t},\"k0\":{k0},\"aLoc\":{a_loc},\"fo\":{},\"g1\":{g1},\"it\":{i_t},\"iw\":{i_w},\"i0Sq\":{i0_sq},\"lcr\":{l_cr}}}",
                m.id, alloy.f_o_pa
            ))
        })
        .collect();
    format!(",\"connections\":[{}],\"sheets\":[{}],\"tf\":[{}],\"shellModes\":[{}],\"buckling\":[{}]", connections.join(","), sheets.join(","), tf.join(","), shell_modes.join(","), buckling.join(","))
}

#[test]
fn the_bolt_group_diagram_draws_one_hole_per_fastener_and_drags_the_edge_distance_and_the_pitch() {
    let document = compliant();
    let report = checks::evaluate_structure(&document);
    let conn = &document.connections[0];
    let diagram = find(&document, &report, &conn.id, &format!("bolt-group:{}", conn.id));
    assert_eq!(diagram.validate(), Ok(()));
    let count = (conn.bolts.rows * conn.bolts.bolts_per_row) as usize;
    assert_eq!((diagram.markers.len(), polyline_count(&diagram, "hole-"), diagram.table_twin.columns[0].values.len()), (count, count, count));
    let shear = check(&report, &format!("en1999.8.5.bolt.{}", conn.id));
    assert!(diagram.markers.iter().all(|m| (m.utilisation.expect("utilisation") - shear.utilization).abs() < 1e-12 && m.bound_check.as_deref() == Some(shear.id.as_str())));
    let bound: Vec<_> = diagram.markers.iter().filter_map(|m| m.binding.as_ref().map(|b| (m.at.x, b))).collect();
    let edge = bound.iter().find(|(_, b)| b.path.ends_with("bolts.edgeDistance")).expect("edge marker");
    assert!((edge.1.map.base + edge.1.map.k * edge.0 - conn.bolts.edge_distance).abs() < 1e-12);
    if conn.bolts.rows > 1 {
        let pitch = bound.iter().find(|(_, b)| b.path.ends_with("bolts.pitch")).expect("pitch marker");
        assert!((pitch.1.map.base + pitch.1.map.k * pitch.0 - conn.bolts.pitch).abs() < 1e-12);
    }
}

#[test]
fn a_pure_weld_has_no_bolt_group_and_a_pure_bolting_no_weld_diagram() {
    let mut document = compliant();
    let id = document.connections[0].id.clone();
    document.connections[0].kind = model::ConnectionKind::Welded;
    let report = checks::evaluate_structure(&document);
    let ids: Vec<String> = diagrams(&document, &report, Some(&id)).into_iter().map(|d| d.id).collect();
    assert!(ids.contains(&format!("weld-resistance:{id}")) && !ids.contains(&format!("bolt-group:{id}")), "{ids:?}");
    document.connections[0].kind = model::ConnectionKind::Bolted;
    let report = checks::evaluate_structure(&document);
    let ids: Vec<String> = diagrams(&document, &report, Some(&id)).into_iter().map(|d| d.id).collect();
    assert!(!ids.contains(&format!("weld-resistance:{id}")) && ids.contains(&format!("bolt-group:{id}")), "{ids:?}");
}

#[test]
fn the_weld_resistance_diagram_pins_the_throat_marker_to_the_check_and_drags_the_throat() {
    let document = compliant();
    let report = checks::evaluate_structure(&document);
    let conn = &document.connections[0];
    let diagram = find(&document, &report, &conn.id, &format!("weld-resistance:{}", conn.id));
    assert_eq!(diagram.validate(), Ok(()));
    let weld = check(&report, &format!("en1999.8.6.weld.{}", conn.id));
    let marker = &diagram.markers[0];
    assert!((marker.utilisation.expect("utilisation") - weld.utilization).abs() < 1e-12);
    assert!((marker.at.y - weld.limit.value / 1e3).abs() < 1e-9 * marker.at.y.max(1.0));
    assert_eq!(marker.binding.as_ref().map(|b| b.path.as_str()), Some(format!("connections[id={}].welds.throat", conn.id).as_str()));
    let metal = &diagram.table_twin.columns[1].values;
    assert!(metal.windows(2).all(|w| w[1] >= w[0] - 1e-9), "a thicker weld metal throat never carries less");
    let limit = diagram.series.iter().find_map(|s| match s { scene::Series::Limit { id, value, .. } if id == "f-ed" => Some(*value), _ => None }).expect("design force limit");
    assert!((limit - weld.computed.value / 1e3).abs() < 1e-9 * limit.max(1.0));
}

#[test]
fn the_sheet_diagrams_follow_the_checks_and_drag_the_width_and_the_thickness() {
    let document = compliant();
    let report = checks::evaluate_structure(&document);
    let sheet = &document.cold_formed[0];
    let local = find(&document, &report, &sheet.id, &format!("sheet-local:{}", sheet.id));
    assert_eq!(local.validate(), Ok(()));
    let marker = &local.markers[0];
    let binding = marker.binding.as_ref().expect("width binding");
    assert_eq!(binding.path, format!("coldFormed[id={}].width", sheet.id));
    let beta_eps = check(&report, &format!("en1999.1-4.local.{}", sheet.id)).computed.value;
    assert!((marker.at.x - beta_eps.min(60.0)).abs() < 1e-9 && (binding.map.k * marker.at.x - sheet.width).abs() < 1e-9 * sheet.width);
    let resistance = find(&document, &report, &sheet.id, &format!("sheet-resistance:{}", sheet.id));
    assert_eq!(resistance.validate(), Ok(()));
    let nm = check(&report, &format!("en1999.1-4.nm.{}", sheet.id));
    let thickness = &resistance.markers[0];
    assert!((thickness.utilisation.expect("utilisation") - nm.utilization).abs() < 1e-12 && thickness.bound_check.as_deref() == Some(nm.id.as_str()));
    assert_eq!(thickness.binding.as_ref().map(|b| b.path.as_str()), Some(format!("coldFormed[id={}].thickness", sheet.id).as_str()));
    assert_eq!(resistance.table_twin.columns.len(), 5);
}

fn parts_of(document: &En1999Snapshot) -> (model::AluminiumMember, model::AluminiumSection) {
    let member = model::AluminiumMember { support: model::SupportCondition::SimplySupported, length: 6.0, ..document.members[0].clone() };
    (member.clone(), document.sections.iter().find(|s| s.id == member.section_id).expect("section").clone())
}

fn near(actual: f64, expected: f64, tolerance: f64, what: &str) {
    assert!((actual - expected).abs() <= tolerance * expected.abs(), "{what}: {actual} vs {expected}");
}

#[test]
fn a_doubly_symmetric_section_buckles_in_pure_torsion_at_the_closed_form_load() {
    let (member, section) = parts_of(&compliant());
    let system = eigen::torsional_flexural_system(&member, &section).expect("pinned I section");
    assert!(system.y0.abs() < 1e-4 && system.z0.abs() < 1e-4, "doubly symmetric: shear centre at the centroid");
    let (n, x) = system.lowest_twisting().expect("a twisting mode");
    near(n, system.nt, 1e-6, "N_cr,T");
    assert!(x[2].abs() > 0.999, "{x:?}");
    let (a, iy, iz, _, _, it, iw) = checks::part_1_1::section_geometry(&section);
    let pi_l = std::f64::consts::PI / member.length;
    near(system.nt, (checks::na_de::G_PA * it + pi_l * pi_l * checks::na_de::E_PA * iw) / ((iy + iz) / a), 1e-3, "closed form");
}

#[test]
fn a_channel_couples_bending_and_torsion_and_the_mode_is_the_null_vector_of_the_system() {
    let document = compliant();
    let (member, mut section) = parts_of(&document);
    section.kind = model::SectionKind::Channel;
    let system = eigen::torsional_flexural_system(&member, &section).expect("pinned channel");
    assert!(system.y0.abs() + system.z0.abs() > 1e-3, "a channel has its shear centre outside the web");
    let (n, x) = system.lowest_twisting().expect("a twisting mode");
    assert!(n < system.nt, "coupling lowers the load below the torsional one: {n} vs {}", system.nt);
    let m = system.matrix(n);
    let scale = system.nu.max(system.nv).max(system.i0_sq * system.nt);
    for row in m {
        assert!((row[0] * x[0] + row[1] * x[1] + row[2] * x[2]).abs() < 1e-7 * scale, "{row:?} · {x:?}");
    }
    let frame = kit::member_frame(&scene::Vec3::ZERO, &kit::v3(1.0, 0.0, 0.0));
    let eigen = eigen::torsional_flexural(&member, &section, &frame, 0.14).expect("mode shape");
    near(eigen.factor, n, 1e-12, "factor");
    let peak = eigen.samples.iter().map(|s| s.delta.x.abs().max(s.delta.y.abs()).max(s.delta.z.abs())).fold(0.0, f64::max);
    near(peak, 1.0, 1e-9, "largest displacement");
    let first = &eigen.samples[0].delta;
    let last = &eigen.samples[eigen.samples.len() - 1].delta;
    assert!(first.y.abs() + first.z.abs() < 1e-9 && last.y.abs() + last.z.abs() < 1e-9, "a fork support holds the ends");
}

#[test]
fn only_fork_supported_members_with_principal_section_axes_get_a_torsional_flexural_mode() {
    let document = compliant();
    let (member, section) = parts_of(&document);
    let cantilever = model::AluminiumMember { support: model::SupportCondition::Cantilever, ..member.clone() };
    assert!(eigen::torsional_flexural_system(&cantilever, &section).is_none());
    let angle = model::AluminiumSection { kind: model::SectionKind::Angle, ..section };
    assert!(eigen::torsional_flexural_system(&member, &angle).is_none());
}

pub(super) fn eccentric() -> En1999Snapshot {
    let mut document = compliant();
    document.connections[0].load_eccentricity = 0.08;
    document
}

pub(super) fn instantaneous_centre() -> En1999Snapshot {
    let mut document = eccentric();
    document.connections[0].force_method = model::FastenerForceMethod::InstantaneousCentre;
    document
}

#[test]
fn the_instantaneous_centre_method_unloads_the_governing_bolt_in_plan_and_the_check_agrees() {
    let (elastic, ductile) = (eccentric(), instantaneous_centre());
    let (report_e, report_d) = (checks::evaluate_structure(&elastic), checks::evaluate_structure(&ductile));
    let conn = &ductile.connections[0];
    let diagram = find(&ductile, &report_d, &conn.id, &format!("bolt-group:{}", conn.id));
    assert_eq!(diagram.validate(), Ok(()));
    let shear = (check(&report_e, &format!("en1999.8.5.bolt.{}", conn.id)), check(&report_d, &format!("en1999.8.5.bolt.{}", conn.id)));
    assert!(shear.1.utilization < shear.0.utilization, "ductile redistribution unloads the governing fastener");
    let top = diagram.markers.iter().map(|m| m.utilisation.expect("utilisation")).fold(0.0, f64::max);
    assert!((top - shear.1.utilization).abs() < 1e-12, "the governing fastener carries the check utilisation");
    assert!(shear.1.explanation.en.contains("instantaneousCentre"));
}

#[test]
fn an_eccentric_shear_draws_unequal_force_arrows_and_the_governing_bolt_in_plan() {
    let document = eccentric();
    let report = checks::evaluate_structure(&document);
    let conn = &document.connections[0];
    let diagram = find(&document, &report, &conn.id, &format!("bolt-group:{}", conn.id));
    assert_eq!(diagram.validate(), Ok(()));
    let count = (conn.bolts.rows * conn.bolts.bolts_per_row) as usize;
    assert_eq!(polyline_count(&diagram, "force-"), count, "one force arrow per fastener");
    let shear = check(&report, &format!("en1999.8.5.bolt.{}", conn.id));
    let utilisations: Vec<f64> = diagram.markers.iter().map(|m| m.utilisation.expect("utilisation")).collect();
    let top = utilisations.iter().cloned().fold(0.0, f64::max);
    assert!((top - shear.utilization).abs() < 1e-12, "the governing fastener carries the check utilisation");
    assert!(utilisations.iter().any(|u| *u < top - 1e-9), "the torque unloads the inner fasteners");
    assert!(diagram.markers.iter().any(|m| m.label.en.contains("(governing)")));
    let forces = &diagram.table_twin.columns[2].values;
    assert!((forces.iter().cloned().fold(0.0, f64::max) - shear.computed.value / 1e3).abs() < 1e-9 * forces[0].max(1.0));
    assert_eq!(diagram.table_twin.columns.len(), 4);
}

#[test]
fn the_scene_draws_one_force_arrow_per_bolt_and_highlights_the_governing_bolt_only_when_the_forces_differ() {
    use scene::projection_kit::test_support::{entity, revision};
    let project_of = |document: &En1999Snapshot| {
        let report = checks::evaluate_structure(document);
        project(&SceneInput { document, report: &report, placements: &[], revision: revision() })
    };
    let base = compliant();
    let concentric = project_of(&base);
    let eccentric_scene = project_of(&eccentric());
    let conn = &base.connections[0];
    let count = (conn.bolts.rows * conn.bolts.bolts_per_row) as usize;
    let arrows = |scene: &StructuralScene| scene.entities.iter().filter(|e| e.entity.id.starts_with(&format!("{}:fastener-force:", conn.id))).count();
    assert_eq!((arrows(&concentric), arrows(&eccentric_scene)), (count, count));
    assert_eq!(concentric.validate(), Ok(()));
    assert_eq!(eccentric_scene.validate(), Ok(()));
    assert!(!concentric.entities.iter().any(|e| e.entity.id.ends_with(":fastener-governing")), "equal forces have no governing bolt");
    let governing = entity(&eccentric_scene, &format!("{}:fastener-governing", conn.id));
    let scene::Geometry::Fastener(highlight) = &governing.geometry else { panic!("fastener geometry") };
    assert!(highlight.diameter > conn.bolts.diameter && highlight.pattern.is_none());
    let load = |scene: &StructuralScene, k: usize| match &scene.entities.iter().find(|e| e.entity.id == format!("{}:fastener-force:{}-{}", conn.id, k / conn.bolts.bolts_per_row as usize, k % conn.bolts.bolts_per_row as usize)).expect("arrow").geometry {
        scene::Geometry::Load(scene::LoadGlyph::Point { magnitude, .. }) => magnitude.value,
        other => panic!("{other:?}"),
    };
    let magnitudes: Vec<f64> = (0..count).map(|k| load(&eccentric_scene, k)).collect();
    let top = magnitudes.iter().cloned().fold(0.0, f64::max);
    assert!(magnitudes.iter().any(|m| *m < top - 1e-6), "unequal arrows");
    let equal: Vec<f64> = (0..count).map(|k| load(&concentric, k)).collect();
    assert!(equal.iter().all(|m| (m - equal[0]).abs() < 1e-9 * equal[0]), "a concentric shear is shared equally");
}

#[test]
fn the_axial_shell_pattern_has_the_lorenz_half_wave_and_the_bay_window() {
    let (r, t) = (0.5, 0.01);
    let half = eigen::shell_half_wave(r, t);
    near(half, 1.728 * (r * t).sqrt(), 1e-3, "half-wave 1.728·√(r·t)");
    let short = eigen::shell_axial(r, t, 0.3, (0.0, 0.3)).expect("a pattern");
    assert_eq!((short.waves, short.window), (2, (0.0, 0.3)), "a bay of 2.5 half-waves holds two whole ones");
    let long = eigen::shell_axial(r, t, 2.0, (0.0, 2.0)).expect("a pattern");
    assert_eq!(long.waves, 4);
    near(long.window.1 - long.window.0, 4.0 * half, 1e-12, "four half-waves from the bay start");
    assert!(long.generatrix.first() == Some(&0.0) && long.generatrix.last() == Some(&2.0), "the mesh spans the shell");
    assert!(long.samples.iter().all(|s| s.delta.z == 0.0), "radial displacement only");
    assert!(long.samples.iter().filter(|s| s.at.y > long.window.1 + 1e-9).all(|s| s.delta.x == 0.0 && s.delta.y == 0.0), "no displacement outside the window");
    let peak = long.samples.iter().map(|s| s.delta.x.hypot(s.delta.y)).fold(0.0, f64::max);
    near(peak, 1.0, 1e-9, "unit amplitude");
    assert!(eigen::shell_axial(0.0, t, 1.0, (0.0, 1.0)).is_none() && eigen::shell_axial(r, t, 1.0, (0.5, 0.5)).is_none());
}

#[test]
fn every_shell_gets_a_buckling_mode_sibling_with_a_surface_overlay_and_the_critical_load_factor() {
    use scene::projection_kit::test_support::{entity, revision};
    let document = compliant();
    let report = checks::evaluate_structure(&document);
    let scene = project(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    assert_eq!(scene.validate(), Ok(()));
    assert!(!document.shells.is_empty());
    for shell in &document.shells {
        let mode = entity(&scene, &format!("{}:{}", shell.id, tracks::LAYER_MODE_SHELL));
        assert_eq!(mode.layer, tracks::LAYER_MODE_SHELL);
        let overlay = mode.appearance.deformation.as_ref().expect("overlay");
        assert!(!overlay.surface.is_empty() && overlay.samples.is_empty() && overlay.kind == scene::DeformationKind::Mode);
        let scene::Geometry::Shell(geometry) = &mode.geometry else { panic!("shell geometry") };
        let scene::ShellShape::Revolved { generatrix, .. } = &geometry.shape else { panic!("revolved shell") };
        assert_eq!(overlay.surface.len(), generatrix.points.len() * 49, "one displacement per mesh node of the 48-segment sweep");
        let sx = check(&report, &format!("en1999.1-5.buckle.{}", shell.id)).computed.value.abs();
        if sx > 0.0 {
            let bays: Vec<f64> = {
                let mut cuts: Vec<f64> = document.ring_stiffeners.iter().filter(|r| r.shell_id == shell.id && r.x > 0.0 && r.x < shell.length).map(|r| r.x).collect();
                cuts.sort_by(f64::total_cmp);
                std::iter::once(0.0).chain(cuts).chain(std::iter::once(shell.length)).collect::<Vec<_>>().windows(2).map(|w| w[1] - w[0]).collect()
            };
            let longest = bays.iter().cloned().fold(0.0, f64::max);
            near(overlay.load_factor.expect("load factor"), checks::part_1_4_1_5::sigma_x_rcr(shell.radius, shell.thickness, longest) / sx, 1e-12, "α_cr = σ_x,Rcr/σ_x,Ed");
        }
    }
    assert!(scene.tracks.iter().any(|t| matches!(t.kind, scene::TrackKind::Mode)), "the mode track offers the shell mode");
}

fn profiled() -> En1999Snapshot {
    let mut document = compliant();
    document.cold_formed[0].thickness = 0.001;
    let id = document.cold_formed[0].id.clone();
    document.sheet_profiles.push(model::SheetProfile { id: "prof-1".into(), sheet_id: id, pitch: 0.200, top_flange: 0.060, bottom_flange: 0.060, web_height: 0.040 });
    document
}

#[test]
fn a_profiled_sheet_is_drawn_as_its_ribs_and_its_diagram_drags_the_profile() {
    use scene::projection_kit::test_support::{entity, revision};
    let document = profiled();
    let report = checks::evaluate_structure(&document);
    let sheet = &document.cold_formed[0];
    let scene = project(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    let drawn = entity(&scene, &sheet.id);
    let scene::Geometry::Member(member) = &drawn.geometry else { panic!("sheet member") };
    let scene::SectionShape::ThinWalled { centreline, .. } = &member.section else { panic!("thin-walled rib") };
    let ribs = (sheet.width / 0.200).round().max(1.0) as usize;
    assert_eq!(centreline.points.len(), 1 + 4 * ribs);
    let (lowest, highest) = centreline.points.iter().fold((f64::MAX, f64::MIN), |(lo, hi), p| (lo.min(p.y), hi.max(p.y)));
    near(highest - lowest, 0.040, 1e-9, "profile height");
    let diagram = find(&document, &report, &sheet.id, &format!("sheet-profile:{}", sheet.id));
    assert_eq!(diagram.validate(), Ok(()));
    let paths: Vec<&str> = diagram.markers.iter().filter_map(|m| m.binding.as_ref().map(|b| b.path.as_str())).collect();
    assert_eq!(paths, ["sheetProfiles[id=prof-1].pitch", "sheetProfiles[id=prof-1].topFlange", "sheetProfiles[id=prof-1].bottomFlange"]);
    let level = |id: &str| diagram.series.iter().find_map(|s| match s { scene::Series::Limit { id: found, value, .. } if found == id => Some(*value), _ => None }).expect("neutral axis");
    assert!(level("axis-effective") < level("axis-gross"), "the thinner compressed top flange moves the neutral axis down");
    near(level("axis-gross"), 0.020, 1e-9, "gross neutral axis of the symmetric rib");
    let bend = check(&report, &format!("en1999.1-4.bend.{}", sheet.id));
    assert!(diagram.markers.iter().all(|m| m.bound_check.as_deref() == Some(bend.id.as_str())));
    let resistance = find(&document, &report, &sheet.id, &format!("sheet-resistance:{}", sheet.id));
    assert_eq!(resistance.validate(), Ok(()));
    let grips = handles::ids(&document, &layout::Layout::of(&document, &[]), &sheet.id);
    for field in ["pitch", "top-flange", "bottom-flange", "web-height"] {
        assert!(grips.iter().any(|h| h.ends_with(&format!("gauge-prof-1-{field}"))), "{field}: {grips:?}");
    }
}

#[test]
fn a_hole_stagger_offsets_every_second_bolt_line_and_the_plan_draws_the_critical_net_path() {
    let mut document = compliant();
    document.connections[0].bolts.bolts_per_row = 3;
    document.connections[0].bolts.gauge = 0.040;
    document.connections[0].hole_stagger = 0.030;
    let report = checks::evaluate_structure(&document);
    let conn = &document.connections[0];
    let diagram = find(&document, &report, &conn.id, &format!("bolt-group:{}", conn.id));
    assert_eq!(diagram.validate(), Ok(()));
    let (e, per) = (conn.bolts.edge_distance, conn.bolts.bolts_per_row as usize);
    for (k, x) in diagram.table_twin.columns[0].values.iter().enumerate() {
        let row = (k / per) as f64;
        near(*x, e + row * conn.bolts.pitch + if k % per % 2 == 1 { 0.030 } else { 0.0 }, 1e-12, "x of the bolt");
    }
    let path = diagram.series.iter().find_map(|s| match s { scene::Series::Polyline { id, points, .. } if id == "net-path" => Some(points), _ => None }).expect("critical net path");
    assert_eq!(path.len(), 5, "edge, three holes, edge");
    let stagger = diagram.markers.iter().find_map(|m| m.binding.as_ref().filter(|b| b.path.ends_with("holeStagger")).map(|b| (m.at.x, b.map))).expect("stagger marker");
    near(stagger.1.base + stagger.1.k * stagger.0, 0.030, 1e-12, "stagger from the marker");
}

#[test]
fn a_fire_insulation_lowers_the_member_temperature_of_the_fire_entity_and_has_gauges() {
    let mut document = compliant();
    let fire = document.fire_scenarios[0].clone();
    let section = document.sections.iter().find(|s| s.id == document.members.iter().find(|m| m.id == fire.member_id).expect("member").section_id).expect("section").clone();
    let bare = fire_and_fatigue::member_temperature(&document, &fire, &section);
    document.fire_insulations.push(model::FireInsulation { id: "ins-1".into(), fire_id: fire.id.clone(), thickness: 0.020, conductivity: 0.20, specific_heat: 1200.0, density: 800.0 });
    let covered = fire_and_fatigue::member_temperature(&document, &fire, &section);
    assert!(covered < bare, "{covered} against {bare}");
    let grips = handles::ids(&document, &layout::Layout::of(&document, &[]), &fire.id);
    for field in ["thickness", "conductivity", "specific-heat", "density"] {
        assert!(grips.iter().any(|h| h.ends_with(&format!("gauge-ins-1-{field}"))), "{field}: {grips:?}");
    }
}
