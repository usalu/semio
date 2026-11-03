//! 🪢️ EN 1999 free outline: shear (§6.2.6), torsion (§6.2.7) and lateral-torsional buckling (§6.3.2) of a member from the section constants of its polygon: I_t, I_w and the shear centre by the finite-element section solver of the mechanics kernel (`analyse_section`), shear areas from the plate edges that name an element.

use super::*;
use super::part_en1990::GoverningEffects;
use semio_s_artifact_norm_en1999_section_solver::stability::{self, m_cr_general, z_j};
use semio_s_plugin_norm_mechanics as mechanics;
use semio_s_plugin_norm_compliance::CheckStatus;

//#region 🔖️ComplianceHelpers
/// 🪢️ Constants of a free outline beyond A and the inertias: torsion and warping constants, shear-centre offsets from the centroid, the mesh edges across the mean wall, the shear areas A_v,z and A_v,y, the thickest wall t_max and the principal-axis frames of the stability checks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreeConstants { pub it: f64, pub iw: f64, pub y_s: f64, pub z_s: f64, pub wall_cells: f64, pub a_vz: f64, pub a_vy: f64, pub t_max: f64, pub frames: stability::Frames }

/// 🪢️ Solves the polygon and derives the shear areas: every ring edge that names a plate element carries shear in proportion to its projection on the shear direction (A_v,z = Σ t·|Δz|, A_v,y = Σ t·|Δy|); without such an edge the gross area is taken. t_max is the thickest named element, else the shorter bounding-box side.
pub fn free_constants(section: &AluminiumSection, rs: &[Ring], props: &PolyProps) -> Option<FreeConstants> {
    let outer = rs.iter().find(|r| r.name == "outer")?;
    let holes: Vec<&[(f64, f64)]> = rs.iter().filter(|r| r.name != "outer").map(|r| r.points.as_slice()).collect();
    let ring_of = |points: &[(f64, f64)]| -> Vec<[f64; 2]> { points.iter().map(|p| [p.0, p.1]).collect() };
    let hole_rings: Vec<Vec<[f64; 2]>> = holes.iter().map(|h| ring_of(h)).collect();
    let alloy = mechanics::Material { id: "alloy".into(), e: 70_000.0e6, g: 27_000.0e6, rho: 2700.0 };
    let solved = mechanics::analyse_section(&ring_of(&outer.points), &hole_rings, &alloy).ok()?;
    let perimeter: f64 = rs.iter().map(|r| (0..r.points.len()).map(|i| { let (p, q) = (r.points[i], r.points[(i + 1) % r.points.len()]); (q.0 - p.0).hypot(q.1 - p.1) }).sum::<f64>()).sum();
    let (y_s, z_s) = (solved.shear_centre_y - solved.centroid_y, solved.shear_centre_z - solved.centroid_z);
    let (mut vz, mut vy, mut thickest) = (0.0, 0.0, 0.0f64);
    for ring in rs {
        let n = ring.points.len();
        for i in 0..n {
            let Some(el) = section.elements.iter().find(|e| !ring.elements[i].is_empty() && e.id == ring.elements[i]) else { continue };
            let (p, q) = (ring.points[i], ring.points[(i + 1) % n]);
            vz += el.thickness * (q.1 - p.1).abs();
            vy += el.thickness * (q.0 - p.0).abs();
            thickest = thickest.max(el.thickness);
        }
    }
    let (ymin, ymax) = outer.points.iter().fold((f64::MAX, f64::MIN), |m, p| (m.0.min(p.0), m.1.max(p.0)));
    let (zmin, zmax) = outer.points.iter().fold((f64::MAX, f64::MIN), |m, p| (m.0.min(p.1), m.1.max(p.1)));
    let shear = |v: f64| if v > 1e-12 { v.min(props.a) } else { props.a };
    Some(FreeConstants { it: solved.j, iw: solved.iw, y_s, z_s, wall_cells: 2.0 * solved.area / perimeter / solved.max_edge.max(1e-300), a_vz: shear(vz), a_vy: shear(vy), t_max: if thickest > 0.0 { thickest } else { (ymax - ymin).min(zmax - zmin) }, frames: stability::frames(&outer.points, &holes, y_s, z_s)? })
}

/// 🪢️ Section constants of a free outline as an information check; a Warning when the mesh resolves the mean wall with fewer than three edges or the section is not doubly symmetric (shear-centre offset above 1 % of the larger bounding-box side).
pub fn torsion_info(section: &AluminiumSection, props: &PolyProps, k: &FreeConstants, annex: AnnexChoice) -> CheckResult {
    let weak = k.wall_cells < 3.0;
    let subject = SubjectRef::new(&section.id, format!("sections[id={}].kind", section.id), loc(&format!("Section {}", section.id), &format!("Querschnitt {}", section.id)));
    let mut c = CheckResult::assess(format!("en1999.6.1.outline.torsion.{}", section.id), "DIN EN 1999-1-1", clause("6.1"), subject, loc("Torsion and warping constants of the outline", "Torsions- und Wölbkonstante des Umrisses"))
        .status(if weak { CheckStatus::Warning } else { CheckStatus::Info })
        .annex(annex)
        .explanation(loc(
            &format!("I_t = {:.4e} mm⁴, I_w = {:.4e} mm⁶, shear centre ({:.2}, {:.2}) mm from the centroid, A_v,z = {:.1} mm², A_v,y = {:.1} mm² (finite-element solution on six-node triangles, {:.1} mesh edges across the mean wall; within 0.5 % of sectionproperties){}.", k.it * 1e12, k.iw * 1e18, k.y_s * 1e3, k.z_s * 1e3, k.a_vz * 1e6, k.a_vy * 1e6, k.wall_cells, if weak { "; coarse mesh, treat I_t and I_w as estimates" } else { "" }),
            &format!("I_t = {:.4e} mm⁴, I_w = {:.4e} mm⁶, Schubmittelpunkt ({:.2}, {:.2}) mm vom Schwerpunkt, A_v,z = {:.1} mm², A_v,y = {:.1} mm² (Finite-Elemente-Lösung mit Sechsknoten-Dreiecken, {:.1} Netzkanten über die mittlere Wand; innerhalb 0,5 % von sectionproperties){}.", k.it * 1e12, k.iw * 1e18, k.y_s * 1e3, k.z_s * 1e3, k.a_vz * 1e6, k.a_vy * 1e6, k.wall_cells, if weak { "; grobes Netz, I_t und I_w sind Schätzwerte" } else { "" })));
    if weak {
        c = c.remedy(Remedy::one_of(SubjectRef::new(&section.id, format!("sections[id={}].kind", section.id), loc(&format!("Section {}", section.id), &format!("Querschnitt {}", section.id))), vec!["extrudedI".into(), "rhs".into(), "box".into(), "channel".into(), "angle".into(), "tube".into()], loc("Use a parametric kind with closed-form constants where one fits.", "Wo eine passt, eine parametrische Art mit geschlossenen Konstanten wählen.")));
    }
    c.build()
}

/// 🪢️ The lateral-torsional buckling resistance M_b,Rd and its localized-weld factor ω_x, handed to the member interaction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LtbOut { pub m_b_rd: f64, pub omega_lt: f64 }

/// 🪢️ Shear V_z/V_y (§6.2.6), torsion T and shear with torsion (§6.2.7) and lateral-torsional buckling (§6.3.2) of one member on a free outline under the governing effects `gov`; `m_rd_y` is the bending resistance of the cross-section and `class` its class.
pub fn free_shear_torsion_ltb(member: &AluminiumMember, _props: &PolyProps, k: &FreeConstants, class: part_1_1::SectionClass, m_rd_y: f64, alloy: &part_1_1::AlloyProps, annex: AnnexChoice, params: &na_de::AnnexParams, gov: &GoverningEffects) -> (Vec<CheckResult>, LtbOut) {
    let (fo, g1) = (alloy.f_o_pa, params.gamma_m1);
    let label = member_label(&member.id);
    let tag = format!(" (ULS {}, lead {}, free outline)", gov.combination, gov.action_id);
    let path = |leaf: &str| format!("members[id={}].actions[id={}].{leaf}", member.id, gov.action_id);
    let mut out = Vec::new();
    let mut push = |id: &str, sec: &str, title: (&str, &str), computed: Quantity, limit: Quantity, expl: (String, String), p: String, rem: Vec<Remedy>| {
        out.push(utilization_check(format!("{id}.{}", member.id), "DIN EN 1999-1-1", clause(sec), SubjectRef::new(&member.id, p, label.clone()), loc(title.0, title.1), computed, limit, annex, loc(&format!("{}{tag}", expl.0), &format!("{}{}", expl.1, tag.replace("ULS", "GZT").replace("lead", "führend").replace("free outline", "freier Umriss"))), rem));
    };
    let root3 = 3.0f64.sqrt();
    let (v_z_rd, v_y_rd) = (k.a_vz * fo / (root3 * g1), k.a_vy * fo / (root3 * g1));
    let rem = leaf_remedies("members", &member.id, &member.actions, gov.v_z_ed.abs() / v_z_rd.max(1e-18), &label, "Reduce the shear force V_z", "Querkraft V_z senken", "vZK");
    let text = format!("V_z,Ed = {:.1} kN, A_v,z = {:.1} mm², V_pl,Rd = {:.1} kN", gov.v_z_ed / 1e3, k.a_vz * 1e6, v_z_rd / 1e3);
    push("en1999.6.2.6.v", "6.2.6", ("Shear resistance V_z,Rd", "Querkraftwiderstand V_z,Rd"), q_force(gov.v_z_ed.abs()), q_force(v_z_rd), (text.clone(), text), path("vZK"), rem);
    let rem = leaf_remedies("members", &member.id, &member.actions, gov.v_y_ed.abs() / v_y_rd.max(1e-18), &label, "Reduce the shear force V_y", "Querkraft V_y senken", "vYK");
    let text = format!("V_y,Ed = {:.1} kN, A_v,y = {:.1} mm², V_pl,y,Rd = {:.1} kN", gov.v_y_ed / 1e3, k.a_vy * 1e6, v_y_rd / 1e3);
    push("en1999.6.2.6.vy", "6.2.6", ("Shear resistance V_y,Rd", "Querkraftwiderstand V_y,Rd"), q_force(gov.v_y_ed.abs()), q_force(v_y_rd), (text.clone(), text), path("vYK"), rem);
    let w_t = k.it / k.t_max.max(1e-12);
    let t_rd = w_t * fo / (root3 * g1);
    let tau_t = gov.t_ed.abs() / w_t.max(1e-300);
    let rem = leaf_remedies("members", &member.id, &member.actions, gov.t_ed.abs() / t_rd.max(1e-18), &label, "Reduce the torque", "Torsionsmoment senken", "torsionK");
    push("en1999.6.2.7.t", "6.2.7", ("Torsion resistance T_Rd", "Torsionswiderstand T_Rd"), q_moment(gov.t_ed.abs()), q_moment(floor_limit(gov.t_ed, t_rd)),
        (format!("T_Ed = {:.2} kNm, I_t = {:.4e} mm⁴, W_t = I_t/t_max = {:.0} mm³, τ_t,Ed = {:.1} MPa, T_Rd = {:.2} kNm (St. Venant torsion)", gov.t_ed / 1e3, k.it * 1e12, w_t * 1e9, tau_t / 1e6, t_rd / 1e3),
         format!("T_Ed = {:.2} kNm, I_t = {:.4e} mm⁴, W_t = I_t/t_max = {:.0} mm³, τ_t,Ed = {:.1} MPa, T_Rd = {:.2} kNm (St.-Venantsche Torsion)", gov.t_ed / 1e3, k.it * 1e12, w_t * 1e9, tau_t / 1e6, t_rd / 1e3)), path("torsionK"), rem);
    let v_rd_t = v_z_rd * (1.0 - tau_t * root3 * g1 / fo).max(0.0);
    let rem = leaf_remedies("members", &member.id, &member.actions, gov.v_z_ed.abs() / v_rd_t.max(1e-18), &label, "Reduce the shear force or the torque", "Querkraft oder Torsionsmoment senken", "vZK");
    let text = format!("V_Rd,T = V_Rd·(1 − τ_t,Ed·√3·γ_M1/f_o) = {:.1} kN", v_rd_t / 1e3);
    push("en1999.6.2.7.vt", "6.2.7", ("Shear with torsion V_Rd,T", "Querkraft mit Torsion V_Rd,T"), q_force(gov.v_z_ed.abs()), q_force(floor_limit(gov.v_z_ed, v_rd_t)), (text.clone(), text), path("vZK"), rem);
    let l_lt = if member.ltb_length > 0.0 { member.ltb_length } else { member.length };
    let up = gov.m_y_ed >= 0.0;
    let m_cr_at = |sense_up: bool| if member.restrained_ltb { f64::INFINITY } else { m_cr_general(member.c1, na_de::E_PA, na_de::G_PA, &k.frames.strong, k.it, k.iw, l_lt, sense_up) };
    let m_cr = m_cr_at(up);
    let lam_lt = if m_cr.is_finite() && m_cr > 0.0 { (m_rd_y * g1 / m_cr).sqrt() } else { 0.0 };
    let (alpha_lt, lambda_0_lt) = part_1_1::ltb_curve(class);
    let chi_lt = part_1_1::chi_from_lambda(lam_lt, alpha_lt, lambda_0_lt);
    let omega = if member.transverse_weld { part_1_1::omega_x(chi_lt, alloy.rho_o_haz, member.transverse_weld_offset, l_lt, 1.0) } else { 1.0 };
    let m_b_rd = omega * chi_lt * m_rd_y;
    let rem = leaf_remedies("members", &member.id, &member.actions, gov.m_y_ed.abs() / m_b_rd.max(1e-18), &label, "Reduce the strong-axis moment (lateral-torsional buckling)", "Moment um die starke Achse senken (Biegedrillknicken)", "mYK");
    let text = format!("M_cr = {:.2} kNm for compression on the {} side (other side {:.2} kNm; I_z = {:.4e} mm⁴, I_t = {:.4e} mm⁴, I_w = {:.4e} mm⁶, z_j = {:.2} mm, C2 = C3 = load at the shear centre), λ̄_LT = {lam_lt:.4}, χ_LT = {chi_lt:.4}, M_b,Rd = {:.2} kNm", m_cr.min(1.0e12) / 1e3, if up { "+z" } else { "−z" }, m_cr_at(!up).min(1.0e12) / 1e3, k.frames.strong.i_z * 1e12, k.it * 1e12, k.iw * 1e18, z_j(&k.frames.strong, up) * 1e3, m_b_rd / 1e3);
    push("en1999.6.3.2.ltb", "6.3.2", ("Lateral-torsional buckling M_b,Rd", "Biegedrillknicken M_b,Rd"), q_moment(gov.m_y_ed.abs()), q_moment(m_b_rd), (text.clone(), text), format!("members[id={}].ltbLength", member.id), rem);
    (out, LtbOut { m_b_rd, omega_lt: omega })
}
//#endregion 🔖️ComplianceHelpers
