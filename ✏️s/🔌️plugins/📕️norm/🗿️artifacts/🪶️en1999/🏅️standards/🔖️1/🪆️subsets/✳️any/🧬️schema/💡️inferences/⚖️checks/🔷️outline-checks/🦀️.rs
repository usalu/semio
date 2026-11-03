//! 🔷️ EN 1999 free section outline: polygon with holes, its section properties, the strip-model class and effective section, and the cross-section and buckling checks of members on a `free` section.

use super::*;
use semio_s_artifact_norm_en1999_model::{PlateElement, SectionVertex};
use semio_s_plugin_norm_compliance::CheckStatus;

//#region 🔖️ComplianceHelpers
type Pt = (f64, f64);

/// 🔷️ One closed ring of the outline in vertex order; `elements[i]` names the plate element of the edge from `points[i]` to the next point.
#[derive(Clone, Debug, PartialEq)]
pub struct Ring { pub name: String, pub points: Vec<Pt>, pub elements: Vec<String> }

/// 🔷️ Section properties of the polygon about its centroid (y horizontal, z vertical; I_y = ∫z² dA).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PolyProps { pub a: f64, pub yg: f64, pub zg: f64, pub iy: f64, pub iz: f64, pub iyz: f64, pub wy: f64, pub wz: f64 }

/// 🔷️ Class and effective section of the outline under the strip model of local buckling.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Effective { pub class: part_1_1::SectionClass, pub a: f64, pub iy: f64, pub iz: f64, pub wy: f64, pub wz: f64, pub governing: f64, pub limit: f64 }

pub fn is_free(section: &AluminiumSection) -> bool { section.kind == semio_s_artifact_norm_en1999_model::SectionKind::Free }

/// 🏷️ The ring name the outline checks and the scene use.
fn ring_name(ring: semio_s_artifact_norm_en1999_model::OutlineRing) -> &'static str {
    match ring {
        semio_s_artifact_norm_en1999_model::OutlineRing::Outer => "outer",
        semio_s_artifact_norm_en1999_model::OutlineRing::Hole => "hole",
    }
}

/// 🔷️ The rings of one section in order of first appearance.
pub fn rings(vertices: &[SectionVertex], section_id: &str) -> Vec<Ring> {
    let mut out: Vec<Ring> = Vec::new();
    for v in vertices.iter().filter(|v| v.section_id == section_id) {
        match out.iter_mut().find(|r| r.name == ring_name(v.ring)) {
            Some(r) => { r.points.push((v.y, v.z)); r.elements.push(v.element_id.clone()); }
            None => out.push(Ring { name: ring_name(v.ring).to_owned(), points: vec![(v.y, v.z)], elements: vec![v.element_id.clone()] }),
        }
    }
    out
}

fn cross(p: Pt, q: Pt) -> f64 { p.0 * q.1 - q.0 * p.1 }

/// 🔷️ `[A, ∫y dA, ∫z dA, ∫z² dA, ∫y² dA, ∫yz dA]` of one ring by the shoelace sums, oriented to a positive area.
pub fn ring_moments(points: &[Pt]) -> [f64; 6] {
    let n = points.len();
    let mut m = [0.0; 6];
    for i in 0..n {
        let (p, q) = (points[i], points[(i + 1) % n]);
        let c = cross(p, q);
        m[0] += c / 2.0;
        m[1] += (p.0 + q.0) * c / 6.0;
        m[2] += (p.1 + q.1) * c / 6.0;
        m[3] += (p.1 * p.1 + p.1 * q.1 + q.1 * q.1) * c / 12.0;
        m[4] += (p.0 * p.0 + p.0 * q.0 + q.0 * q.0) * c / 12.0;
        m[5] += (p.0 * q.1 + 2.0 * p.0 * p.1 + 2.0 * q.0 * q.1 + q.0 * p.1) * c / 24.0;
    }
    if m[0] < 0.0 { for v in &mut m { *v = -*v; } }
    m
}

/// 🔷️ Properties of the outer ring minus the holes.
pub fn polygon_props(outer: &[Pt], holes: &[&[Pt]]) -> PolyProps {
    let mut m = ring_moments(outer);
    for h in holes {
        let hm = ring_moments(h);
        for i in 0..6 { m[i] -= hm[i]; }
    }
    let a = m[0].max(1e-300);
    let (yg, zg) = (m[1] / a, m[2] / a);
    let (iy, iz, iyz) = (m[3] - a * zg * zg, m[4] - a * yg * yg, m[5] - a * yg * zg);
    let zmax = outer.iter().map(|p| (p.1 - zg).abs()).fold(1e-300, f64::max);
    let ymax = outer.iter().map(|p| (p.0 - yg).abs()).fold(1e-300, f64::max);
    PolyProps { a: m[0], yg, zg, iy, iz, iyz, wy: iy / zmax, wz: iz / ymax }
}

fn orient(a: Pt, b: Pt, c: Pt) -> f64 { (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0) }

fn self_intersects(p: &[Pt]) -> bool {
    let n = p.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if j == i + 1 || (i == 0 && j == n - 1) { continue; }
            let (a, b, c, d) = (p[i], p[(i + 1) % n], p[j], p[(j + 1) % n]);
            if orient(a, b, c) * orient(a, b, d) < 0.0 && orient(c, d, a) * orient(c, d, b) < 0.0 { return true; }
        }
    }
    false
}

fn contains(poly: &[Pt], pt: Pt) -> bool {
    let n = poly.len();
    let mut inside = false;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if (a.1 > pt.1) != (b.1 > pt.1) && pt.0 < (b.0 - a.0) * (pt.1 - a.1) / (b.1 - a.1) + a.0 { inside = !inside; }
    }
    inside
}

/// 🔷️ Why the outline is not a valid polygon with holes: no outer ring, a ring with fewer than three or non-finite vertices, zero area, a self-intersection or a hole vertex outside the outer ring.
pub fn problem(rings: &[Ring]) -> Option<(String, String)> {
    let Some(outer) = rings.iter().find(|r| r.name == "outer") else {
        return Some(("no ring is named `outer`".into(), "kein Ring heißt `outer`".into()));
    };
    for r in rings {
        if r.points.len() < 3 { return Some((format!("ring `{}` has fewer than three vertices", r.name), format!("Ring `{}` hat weniger als drei Knoten", r.name))); }
        if r.points.iter().any(|p| !p.0.is_finite() || !p.1.is_finite()) { return Some((format!("ring `{}` has a non-finite vertex", r.name), format!("Ring `{}` hat einen nicht endlichen Knoten", r.name))); }
        if ring_moments(&r.points)[0] <= 1e-18 { return Some((format!("ring `{}` encloses no area", r.name), format!("Ring `{}` schließt keine Fläche ein", r.name))); }
        if self_intersects(&r.points) { return Some((format!("ring `{}` crosses itself", r.name), format!("Ring `{}` schneidet sich selbst", r.name))); }
    }
    for h in rings.iter().filter(|r| r.name != "outer") {
        if h.points.iter().any(|p| !contains(&outer.points, *p)) { return Some((format!("hole `{}` leaves the outer ring", h.name), format!("Loch `{}` liegt nicht im Außenring", h.name))); }
    }
    None
}

/// 🔷️ Properties of the whole outline; `None` when it is not a valid polygon.
pub fn outline_props(rings: &[Ring]) -> Option<PolyProps> {
    if problem(rings).is_some() { return None; }
    let outer = rings.iter().find(|r| r.name == "outer")?;
    let holes: Vec<&[Pt]> = rings.iter().filter(|r| r.name != "outer").map(|r| r.points.as_slice()).collect();
    Some(polygon_props(&outer.points, &holes))
}

/// 🔷️ Class and effective section: every edge that names a plate element is classified with its edge length as b and the element thickness as t; a class-4 edge loses (1 − ρ_c)·b·t at its midpoint (own inertia of the lost strip included).
pub fn effective(section: &AluminiumSection, props: &PolyProps, rings: &[Ring], f_o_pa: f64, class_a: bool, eta: f64) -> Effective {
    let (mut class, mut governing, mut limit) = (part_1_1::SectionClass::Class1, 0.0, 22.0);
    let (mut lost, mut sy, mut sz, mut izz, mut iyy) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for ring in rings {
        let n = ring.points.len();
        for i in 0..n {
            let Some(el) = section.elements.iter().find(|e| !ring.elements[i].is_empty() && e.id == ring.elements[i]) else { continue };
            let (p, q) = (ring.points[i], ring.points[(i + 1) % n]);
            let len = ((q.0 - p.0).powi(2) + (q.1 - p.1).powi(2)).sqrt();
            let edge = PlateElement { width: len, ..el.clone() };
            let b_eps = part_1_1::element_beta_over_eps(&edge, f_o_pa, eta);
            let outstand = part_1_1::behaves_outstand(&edge);
            let lim = part_1_1::class_limits(class_a, outstand, edge.welded);
            let c = part_1_1::class_of(b_eps, lim);
            if c > class || (c == class && b_eps / lim[2] > governing / limit) { governing = b_eps; limit = lim[2]; }
            class = class.max(c);
            let rho = part_1_1::rho_c(b_eps, class_a, outstand, edge.welded);
            let da = (1.0 - rho) * len * el.thickness;
            if da <= 0.0 || len <= 0.0 { continue; }
            let (ym, zm) = (0.5 * (p.0 + q.0), 0.5 * (p.1 + q.1));
            let (cs, sn) = ((q.0 - p.0) / len, (q.1 - p.1) / len);
            lost += da;
            sy += da * ym;
            sz += da * zm;
            iyy += da * (zm * zm + (len * len * sn * sn + el.thickness * el.thickness * cs * cs) / 12.0);
            izz += da * (ym * ym + (len * len * cs * cs + el.thickness * el.thickness * sn * sn) / 12.0);
        }
    }
    let a = (props.a - lost).max(1e-300);
    let (yg, zg) = ((props.a * props.yg - sy) / a, (props.a * props.zg - sz) / a);
    let iy = (props.iy + props.a * props.zg * props.zg - iyy) - a * zg * zg;
    let iz = (props.iz + props.a * props.yg * props.yg - izz) - a * yg * yg;
    let outer: Vec<Pt> = rings.iter().find(|r| r.name == "outer").map(|r| r.points.clone()).unwrap_or_default();
    let zmax = outer.iter().map(|p| (p.1 - zg).abs()).fold(1e-300, f64::max);
    let ymax = outer.iter().map(|p| (p.0 - yg).abs()).fold(1e-300, f64::max);
    Effective { class, a, iy, iz, wy: iy / zmax, wz: iz / ymax, governing, limit }
}

fn section_subject(section: &AluminiumSection, field: &str) -> SubjectRef {
    SubjectRef::new(&section.id, format!("sections[id={}].{field}", section.id), loc(&format!("Section {}", section.id), &format!("Querschnitt {}", section.id)))
}

/// 🔷️ Validity of the outline and its section properties against the parametric section (or reported for a `free` section).
pub fn check_outline(section: &AluminiumSection, vertices: &[SectionVertex], annex: AnnexChoice) -> Vec<CheckResult> {
    let rs = rings(vertices, &section.id);
    if rs.is_empty() {
        if !is_free(section) { return Vec::new(); }
        return vec![CheckResult::assess(format!("en1999.ref.outline.{}", section.id), "DIN EN 1999-1-1", clause("6.1"), section_subject(section, "kind"), loc("Free section outline", "Freier Querschnittsumriss"))
            .status(CheckStatus::Fail)
            .explanation(loc(&format!("Section {} has kind `free` but no outline vertices.", section.id), &format!("Querschnitt {} hat die Art `free`, aber keine Umrissknoten.", section.id)))
            .annex(annex)
            .remedy(Remedy::one_of(section_subject(section, "kind"), vec!["extrudedI".into(), "rhs".into(), "box".into(), "channel".into(), "angle".into(), "tube".into()], loc("Use a parametric kind or add outline vertices.", "Parametrische Art wählen oder Umrissknoten ergänzen.")))
            .build()];
    }
    if let Some((en, de)) = problem(&rs) {
        return vec![CheckResult::assess(format!("en1999.ref.outline.{}", section.id), "DIN EN 1999-1-1", clause("6.1"), section_subject(section, "kind"), loc("Free section outline", "Freier Querschnittsumriss"))
            .status(CheckStatus::Fail)
            .explanation(loc(&format!("Outline of section {}: {en}.", section.id), &format!("Umriss des Querschnitts {}: {de}.", section.id)))
            .annex(annex)
            .remedy(Remedy::one_of(section_subject(section, "kind"), vec!["extrudedI".into(), "rhs".into(), "box".into(), "channel".into(), "angle".into(), "tube".into()], loc("Use a parametric kind until the outline is a valid polygon with holes.", "Parametrische Art wählen, bis der Umriss ein gültiges Polygon mit Löchern ist.")))
            .build()];
    }
    let Some(p) = outline_props(&rs) else { return Vec::new() };
    let mut out = Vec::new();
    let skew = p.iyz.abs() / (p.iy * p.iz).sqrt().max(1e-300);
    let mut info = CheckResult::assess(format!("en1999.6.1.outline.props.{}", section.id), "DIN EN 1999-1-1", clause("6.1"), section_subject(section, "kind"), loc("Section properties of the outline", "Querschnittswerte des Umrisses"))
        .status(if skew > 0.05 { CheckStatus::Warning } else { CheckStatus::Info })
        .annex(annex)
        .explanation(loc(
            &format!("A = {:.1} mm², centroid ({:.1}, {:.1}) mm, I_y = {:.4e} mm⁴, I_z = {:.4e} mm⁴, I_yz = {:.3e} mm⁴{}.", p.a * 1e6, p.yg * 1e3, p.zg * 1e3, p.iy * 1e12, p.iz * 1e12, p.iyz * 1e12, if skew > 0.05 { "; the section is not symmetric, the principal axes are not rotated here" } else { "" }),
            &format!("A = {:.1} mm², Schwerpunkt ({:.1}, {:.1}) mm, I_y = {:.4e} mm⁴, I_z = {:.4e} mm⁴, I_yz = {:.3e} mm⁴{}.", p.a * 1e6, p.yg * 1e3, p.zg * 1e3, p.iy * 1e12, p.iz * 1e12, p.iyz * 1e12, if skew > 0.05 { "; der Querschnitt ist unsymmetrisch, die Hauptachsen werden hier nicht gedreht" } else { "" })));
    if skew > 0.05 { info = info.remedy(Remedy::advisory(section_subject(section, "kind"), loc("Make the outline symmetric about a centroidal axis (edit the outline points).", "Umriss symmetrisch zu einer Schwerachse gestalten (Umrisspunkte ändern)."))); }
    out.push(info.build());
    if is_free(section) {
        if let Some(k) = free_constants(section, &rs, &p) {
            out.push(torsion_info(section, &p, &k, annex));
        }
    }
    if !is_free(section) {
        let parametric = part_1_1::section_geometry(section).0;
        if parametric > 0.0 {
            let rel = (p.a - parametric).abs() / parametric;
            let mut c = CheckResult::assess(format!("en1999.6.1.outline.match.{}", section.id), "DIN EN 1999-1-1", clause("6.1"), section_subject(section, "kind"), loc("Outline against the parametric section", "Umriss gegen den parametrischen Querschnitt"))
                .utilization(q_dim(rel), q_dim(0.05))
                .annex(annex)
                .explanation(loc(&format!("Polygon area {:.1} mm² against {:.1} mm² of the parametric {:?} section ({:.1} %).", p.a * 1e6, parametric * 1e6, section.kind, rel * 100.0), &format!("Polygonfläche {:.1} mm² gegen {:.1} mm² des parametrischen Querschnitts {} ({:.1} %).", p.a * 1e6, parametric * 1e6, section.kind, rel * 100.0)));
            if rel > 0.05 {
                c = c.status(CheckStatus::Warning).remedy(Remedy::one_of(section_subject(section, "kind"), vec!["free".into()], loc("Let the polygon define the section (kind free).", "Das Polygon den Querschnitt bestimmen lassen (Art free).")));
            }
            out.push(c.build());
        }
    }
    out
}

/// 🔷️ Cross-section, flexural-buckling, shear, torsion and lateral-torsional buckling checks of one member on a `free` section (I_t, I_w and the shear areas from [`free_constants`]); patch loading is not evaluated (reported as not applicable); torsional-flexural buckling and the member interaction come from [`free_stability`].
pub fn check_free_member(member: &AluminiumMember, section: &AluminiumSection, rs: &[Ring], material: &semio_s_artifact_norm_en1999_model::AluminiumMaterial, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Vec<CheckResult> {
    let params = na_de::AnnexParams::for_choice(annex);
    let Some(alloy) = part_1_1::resolve_alloy_for_section(&material.designation, section) else {
        return vec![alloy_unknown(format!("en1999.3.2.alloy.{}", member.id), &member.id, material, annex)];
    };
    let Some(props) = outline_props(rs) else { return Vec::new() };
    let constants = free_constants(section, rs, &props);
    let mut sets: Vec<Vec<CheckResult>> = part_en1990::uls_combinations(member, &params, reliability_class).iter().map(|g| free_uls(member, section, rs, &props, constants.as_ref(), &alloy, annex, &params, g)).collect();
    sets.push(vec![CheckResult::assess(format!("en1999.6.free.scope.{}", member.id), "DIN EN 1999-1-1", clause("6.3"), SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)), loc("Scope of the free-outline member check", "Umfang des Nachweises für freie Umrisse"))
        .not_applicable(loc("Patch loading is not evaluated for a free outline; the cross-section resistance, flexural buckling, torsional-flexural buckling, shear, torsion, lateral-torsional buckling and the member interaction are.", "Einzellasten werden für einen freien Umriss nicht ausgewertet; Querschnittswiderstand, Biegeknicken, Biegedrillknicken, Querkraft, Torsion, Biegedrillknicken und die Bauteilinteraktion werden es."))
        .annex(annex)
        .build()]);
    worst_by_id(sets)
}

fn german(s: &str) -> String {
    s.replace("ULS", "GZT").replace("lead", "führend").replace("free outline", "freier Umriss").replace("class ", "Klasse ").replace("against", "gegen").replace("elastic W", "elastisches W")
}

fn free_uls(member: &AluminiumMember, section: &AluminiumSection, rs: &[Ring], props: &PolyProps, constants: Option<&FreeConstants>, alloy: &part_1_1::AlloyProps, annex: AnnexChoice, params: &na_de::AnnexParams, gov: &part_en1990::GoverningEffects) -> Vec<CheckResult> {
    let (fo, fu, g1, g2) = (alloy.f_o_pa, alloy.f_u_pa, params.gamma_m1, params.gamma_m2);
    let class_a = alloy.buckling_class_a;
    let (n, my, mz) = (gov.n_ed, gov.m_y_ed, gov.m_z_ed);
    let compression = -n / props.a.max(1e-18);
    let (s1, s2) = (compression + my.abs() / props.wy.max(1e-18), compression - my.abs() / props.wy.max(1e-18));
    let (s_max, s_min) = (s1.max(s2), s1.min(s2));
    let eta = if s_max > 0.0 { part_1_1::stress_gradient_eta(s_min / s_max) } else { 1.0 };
    let eff = effective(section, props, rs, fo, class_a, eta);
    let tag = |lead: &str| format!(" (ULS {}, lead {}, {lead})", gov.combination, gov.action_id);
    let label = member_label(&member.id);
    let nc = (-n).max(0.0);
    let path = |leaf: &str| format!("members[id={}].actions[id={}].{leaf}", member.id, gov.action_id);
    let mut out = Vec::new();
    let mut push = |id: &str, sec: &str, title: (&str, &str), computed: Quantity, limit: Quantity, expl: String, p: String, rem: Vec<Remedy>| {
        out.push(utilization_check(format!("{id}.{}", member.id), "DIN EN 1999-1-1", clause(sec), SubjectRef::new(&member.id, p, label.clone()), loc(title.0, title.1), computed, limit, annex, loc(&expl, &german(&expl)), rem));
    };
    let el_class = format!("class {:?}, β/ε = {:.2} against {:.1}", eff.class, eff.governing, eff.limit);
    let n_t_rd = (props.a * fo / g1).min(0.9 * props.a * fu / g2);
    let n_c_rd = eff.a * fo / g1;
    let (n_abs, n_rd, title) = if n >= 0.0 { (n, n_t_rd, ("Tension resistance N_t,Rd (yield, net/HAZ ultimate)", "Zugwiderstand N_t,Rd (Fließen, Netto/WEZ-Bruch)")) } else { (-n, n_c_rd, ("Compression resistance N_c,Rd (A_eff, HAZ)", "Druckwiderstand N_c,Rd (A_eff, WEZ)")) };
    let rem = leaf_remedies("members", &member.id, &member.actions, n_abs / n_rd.max(1e-18), &label, "Reduce the axial force", "Normalkraft senken", "nK");
    push("en1999.6.2.3.n", "6.2.3", title, q_force(n_abs), q_force(n_rd), format!("N_Ed = {:.1} kN, N_t,Rd = {:.1} kN, N_c,Rd = {:.1} kN (A_eff = {:.1} mm², {el_class}){}", n / 1e3, n_t_rd / 1e3, n_c_rd / 1e3, eff.a * 1e6, tag("free outline")), path("nK"), rem);
    let w_y = if eff.class == part_1_1::SectionClass::Class4 { eff.wy } else { props.wy };
    let w_z = if eff.class == part_1_1::SectionClass::Class4 { eff.wz } else { props.wz };
    let (m_rd_y, m_rd_z) = (w_y * fo / g1, w_z * fo / g1);
    let rem = leaf_remedies("members", &member.id, &member.actions, my.abs() / m_rd_y.max(1e-18), &label, "Reduce the strong-axis moment", "Moment um die starke Achse senken", "mYK");
    push("en1999.6.2.5.m", "6.2.5", ("Bending resistance M_y,Rd", "Biegewiderstand M_y,Rd"), q_moment(my.abs()), q_moment(m_rd_y), format!("M_y,Ed = {:.2} kNm, elastic W = {:.0} mm³, M_y,Rd = {:.2} kNm{}", my / 1e3, w_y * 1e9, m_rd_y / 1e3, tag("free outline")), path("mYK"), rem);
    let rem = leaf_remedies("members", &member.id, &member.actions, mz.abs() / m_rd_z.max(1e-18), &label, "Reduce the weak-axis moment", "Moment um die schwache Achse senken", "mZK");
    push("en1999.6.2.5.mz", "6.2.5", ("Bending resistance M_z,Rd", "Biegewiderstand M_z,Rd"), q_moment(mz.abs()), q_moment(m_rd_z), format!("M_z,Ed = {:.2} kNm, elastic W = {:.0} mm³, M_z,Rd = {:.2} kNm{}", mz / 1e3, w_z * 1e9, m_rd_z / 1e3, tag("free outline")), path("mZK"), Vec::new());
    let k_sup = match member.support { SupportCondition::Continuous => 0.7, SupportCondition::Cantilever => 2.0, SupportCondition::SimplySupported => 1.0 };
    let pick = |l: f64| if l > 0.0 { l } else { k_sup * member.length };
    let (alpha, lambda_0) = part_1_1::buckling_curve(class_a);
    let omega = |kc: f64, l: f64| if member.transverse_weld { part_1_1::omega_x(kc, alloy.rho_o_haz, member.transverse_weld_offset, l, 1.0) } else { 1.0 };
    for (id, axis, i, len_field, l) in [("en1999.6.3.1.fb", "y", eff.iy, "bucklingLengthY", pick(member.buckling_length_y)), ("en1999.6.3.1.fb.z", "z", eff.iz, "bucklingLengthZ", pick(member.buckling_length_z))] {
        let lam = part_1_1::lambda_bar(l, i, eff.a, fo);
        let kc = part_1_1::kappa(1.0, lam) * part_1_1::chi_from_lambda(lam, alpha, lambda_0);
        let n_b = omega(kc, l) * kc * eff.a * fo / g1;
        let rem = leaf_remedies("members", &member.id, &member.actions, nc / n_b.max(1e-18), &label, &format!("Reduce the compression force (flexural buckling about {axis})"), &format!("Druckkraft senken (Biegeknicken um {axis})"), "nK");
        push(id, "6.3.1.1", if axis == "y" { ("Flexural buckling N_b,Rd,y", "Biegeknicken N_b,Rd,y") } else { ("Flexural buckling N_b,Rd,z", "Biegeknicken N_b,Rd,z") }, q_force(nc), q_force(n_b), format!("λ̄_{axis} = {lam:.4}, N_b,Rd,{axis} = {:.1} kN (A_eff = {:.1} mm², L_cr = {l:.2} m){}", n_b / 1e3, eff.a * 1e6, tag("free outline")), format!("members[id={}].{len_field}", member.id), rem);
    }
    if let Some(k) = constants {
        let (checks, ltb) = free_shear_torsion_ltb(member, props, k, eff.class, m_rd_y, alloy, annex, params, gov);
        out.extend(checks);
        let chi_z = part_1_1::chi_from_lambda(part_1_1::lambda_bar(pick(member.buckling_length_z), eff.iz, eff.a, fo), alpha, lambda_0);
        let n_b = |i: f64, l: f64| { let kc = part_1_1::chi_from_lambda(part_1_1::lambda_bar(l, i, eff.a, fo), alpha, lambda_0); omega(kc, l) * kc * eff.a * fo / g1 };
        let r = FreeResistances { class: eff.class, a_eff: eff.a, n_b_y: n_b(eff.iy, pick(member.buckling_length_y)), n_b_z: n_b(eff.iz, pick(member.buckling_length_z)), chi_z, m_rd_y, m_rd_z, m_b_rd: ltb.m_b_rd, omega_lt: ltb.omega_lt };
        out.extend(free_stability(member, k, &r, alloy, annex, params, gov));
    }
    out
}
//#endregion 🔖️ComplianceHelpers
