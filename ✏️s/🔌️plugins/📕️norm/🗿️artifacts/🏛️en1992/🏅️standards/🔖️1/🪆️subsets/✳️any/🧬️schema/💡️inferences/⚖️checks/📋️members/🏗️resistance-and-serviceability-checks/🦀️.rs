//! 🏗️ Resistance and serviceability checks: the ultimate resistances, the crack width, the reinforcement limits, the deflection and the serviceability stress limits of the member evaluation. See [`super`].

use super::*;

const STATIONS: usize = 10;

/// 🌀 Distance (mm) from the concrete edge to the surface of the outermost longitudinal bar for the wall limit of EN 1992-1-1 6.3.2(1): the nominal cover is measured to the links (4.4.1), so the closed link diameter is added to it.
pub fn torsion_cover_mm(member: &RcMember) -> f64 {
    (member.cover + member.stirrups.as_ref().map_or(0.0, |s| s.diameter)) * 1000.0
}

/// 🌀 Lower bound `2 (c + φ_l / 2)` (mm) of the effective wall thickness: twice the distance between the edge and the centre of the longitudinal bar, with the link diameter in the cover. See EN 1992-1-1 6.3.2(1).
pub fn torsion_t_min_mm(member: &RcMember) -> f64 {
    2.0 * torsion_cover_mm(member) + member.longitudinal.iter().map(|l| l.diameter * 1000.0).fold(0.0, f64::max)
}

/// 🌀 The catalogue case of T-02 and T-04 for one member: the polygon section (with its hollow), cover, bar diameter, web, effective depth, longitudinal area and the governing torsion and shear in N·mm and N with the truss inclination `cot`; the member checks and the torsion diagrams of the scene read the same case. See EN 1992-1-1 6.3.2 and [`Case`].
pub fn torsion_case(member: &RcMember, sec: &Sec, annex: AnnexChoice, f_ck: f64, f_yk: f64, b_w: f64, d: f64, a_s: f64, t_ed: f64, v_ed: f64, cot: f64) -> Case {
    let ring = |hole: bool| -> (Vec<f64>, Vec<f64>) { sec.loops.iter().find(|l| l.hole == hole).map_or((Vec::new(), Vec::new()), |l| l.pts.iter().copied().unzip()) };
    let (outer, hollow) = (ring(false), ring(true));
    let phi_l = member.longitudinal.iter().map(|l| l.diameter * 1000.0).fold(0.0, f64::max);
    let case = Case::new(&member.id, annex).with_n("fck", f_ck / 1e6).with_l("poly_y", &outer.0).with_l("poly_z", &outer.1).with_n("c", torsion_cover_mm(member)).with_n("phi_l", phi_l).with_n("bw", b_w * 1e3).with_n("d", d * 1e3).with_n("asl", a_s * 1e6).with_n("t_ed", t_ed.abs() * 1e3).with_n("v_ed", v_ed.abs()).with_n("cot_theta", cot).with_n("fyk", f_yk / 1e6);
    if hollow.0.is_empty() { case } else { case.with_l("hole_y", &hollow.0).with_l("hole_z", &hollow.1) }
}

/// 🧭 The chords of a section for the torsion longitudinal steel of EN 1992-1-1 6.3.2(3): the bar area (mm²) below and above the mid-height of the outer outline (tension chord below, as the bars of the flexure model) and the share of the outline perimeter below the mid-height, the part of ΣA_sl the tension chord carries in addition to its bending steel. Returns `(tension area, compression area, tension share)`.
pub fn torsion_chords(sec: &Sec) -> (f64, f64, f64) {
    let outer = sec.loops.iter().find(|l| !l.hole);
    let (lo, hi) = outer.map_or((0.0, 0.0), |l| l.pts.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |t, p| (t.0.min(p.1), t.1.max(p.1))));
    let mid = 0.5 * (lo + hi);
    let below = |accept: bool| -> f64 { sec.bars.iter().filter(|b| (b.z <= mid) == accept).map(|b| b.a).sum() };
    let share = outer.map_or(0.5, |l| {
        let n = l.pts.len();
        let (total, lower) = (0..n).fold((0.0, 0.0), |t, i| {
            let (p, q) = (l.pts[i], l.pts[(i + 1) % n]);
            let len = (q.0 - p.0).hypot(q.1 - p.1);
            let part = if (p.1 - mid).abs() < f64::EPSILON && (q.1 - mid).abs() < f64::EPSILON { 0.5 } else if (p.1 <= mid) == (q.1 <= mid) { if p.1 <= mid { 1.0 } else { 0.0 } } else { let f = (mid - p.1) / (q.1 - p.1); if p.1 <= mid { f } else { 1.0 - f } };
            (t.0 + len, t.1 + len * part)
        });
        if total > 0.0 { lower / total } else { 0.5 }
    });
    (below(true), below(false), share)
}

/// 📏 The catalogue case of the crack width (7.3.4) of one member under the quasi-permanent moment `m_qp` (N·m) and normal force `n_ed` (N, tension positive): the polygon section with hollow, the bars, cover, the diameter of the largest bar of the first non-top layer (0.016 m without bars), the steel modulus and the long-term action; the member check, its stations and the crack-width diagram of the scene read the same case. See EN 1992-1-1 7.3.4 and [`Case`].
pub fn crack_case(member: &RcMember, sec: &Sec, annex: AnnexChoice, f_ck: f64, e_s: f64, m_qp: f64, n_ed: f64) -> Case {
    let phi = member.longitudinal.iter().find(|l| l.position != semio_s_artifact_norm_en1992_model::BarPosition::Top).or(member.longitudinal.first()).map(|l| l.diameter).unwrap_or(0.016);
    let ring = |hole: bool| -> (Vec<f64>, Vec<f64>) { sec.loops.iter().find(|l| l.hole == hole).map_or((Vec::new(), Vec::new()), |l| l.pts.iter().copied().unzip()) };
    let (outer, hollow) = (ring(false), ring(true));
    let (bar_y, bar_z, bar_a): (Vec<f64>, Vec<f64>, Vec<f64>) = sec.bars.iter().fold((Vec::new(), Vec::new(), Vec::new()), |mut t, b| {
        t.0.push(b.y);
        t.1.push(b.z);
        t.2.push(b.a);
        t
    });
    let case = Case::new(&member.id, annex)
        .with_n("fck", f_ck / 1e6).with_l("poly_y", &outer.0).with_l("poly_z", &outer.1).with_l("bar_y", &bar_y).with_l("bar_z", &bar_z).with_l("bar_a", &bar_a)
        .with_n("c", member.cover * 1e3).with_n("phi", phi * 1e3).with_n("m_qp", m_qp.abs() * 1e3).with_n("n_ed", -n_ed).with_n("es", e_s / 1e6)
        .with_s("duration", "long");
    if hollow.0.is_empty() { case } else { case.with_l("hole_y", &hollow.0).with_l("hole_z", &hollow.1) }
}

/// 🏗 Checks the ultimate resistances of the governing ultimate situation.
pub(super) fn check_ultimate_resistances(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    f_ck: f64,
    f_yk: f64,
    e_s: f64,
    sec: &Sec,
    geo: &Geo,
    uls: Option<&DesignEffects>,
    acc: Option<&DesignEffects>,
    uls_v: Option<&DesignEffects>,
    uls_t: Option<&DesignEffects>,
    uls_p: Option<&DesignEffects>,
) {
    let (b, d, h, a_s, rho) = (geo.b_w, geo.d, geo.h, geo.a_s, geo.rho_l());
    if let Some(uls) = uls {
        let uv = uls_v.unwrap_or(uls);
        let ut = uls_t.unwrap_or(uls);
        let up = uls_p.unwrap_or(uls);
        let params = na_de::AnnexParams::for_situation(annex, &uls.situation);
        let section = sec;
        let m_rd = member_m_rd_nm(section, uls.n_ed);
        let mut flex = CheckResult::assess(format!("en1992.6.1.flexure.{}", member.id), part_11(), clause_11("§6.1"), member_ref(doc, member, &member_path(&member.id, "actions")), l("Bending with axial force", "Biegung mit Normalkraft")).remedies_attached_by("attach_minimal_remedies")
            .utilization(moment_nm(uls.m_ed.abs()), moment_nm(m_rd.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("|M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (α_cc={:.2}, governing {})", uls.m_ed.abs() / 1e3, m_rd / 1e3, params.alpha_cc, uls.combination_id),
                &format!("|M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (α_cc={:.2}, maßgebend {})", uls.m_ed.abs() / 1e3, m_rd / 1e3, params.alpha_cc, uls.combination_id),
            ));
        if let Some(profile) = Profile::of(member) {
            flex = flex.sampled(STATIONS, |s| {
                let m = uls.m_ed.abs() * profile.moment(s);
                let station = Station::ratio_of(&[s], moment_nm(m), moment_nm(m_rd.max(1.0))).with(force_n(uls.n_ed));
                (station.utilization, station.values)
            });
        } else if matches!(member.kind, MemberKind::Column | MemberKind::Wall | MemberKind::TensionMember) {
            flex = flex.stations([0.0, 1.0].map(|s| Station::ratio_of(&[s], moment_nm(uls.m_ed.abs()), moment_nm(m_rd.max(1.0))).with(force_n(uls.n_ed))).to_vec());
        }
        out.push(ndps(flex, &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent"]).build());
        if let Some(acc_e) = &acc {
            let params_acc = na_de::AnnexParams::for_situation(annex, "accidental");
            let m_rd_acc = member_m_rd_nm(&member_section_in(doc, member, f_ck, f_yk, e_s, annex, true), acc_e.n_ed);
            let flex_acc = CheckResult::assess(format!("en1992.6.1.flexure.acc.{}", member.id), part_11(), clause_11("§6.1"), member_ref(doc, member, &member_path(&member.id, "actions")), l("Accidental bending ACC-6.11", "Außergewöhnliche Biegung ACC-6.11")).remedies_attached_by("attach_minimal_remedies")
                .utilization(moment_nm(acc_e.m_ed.abs()), moment_nm(m_rd_acc.max(1.0)))
                .annex(annex)
                .explanation(l(
                    &format!("ACC-6.11: |M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (γ_c={:.2}, γ_s={:.2}, {})", acc_e.m_ed.abs() / 1e3, m_rd_acc / 1e3, params_acc.gamma_c, params_acc.gamma_s, acc_e.combination_id),
                    &format!("Außergewöhnlich ACC-6.11: |M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (γ_c={:.2}, γ_s={:.2}, {})", acc_e.m_ed.abs() / 1e3, m_rd_acc / 1e3, params_acc.gamma_c, params_acc.gamma_s, acc_e.combination_id),
                ));
            out.push(ndps(flex_acc, &["ndp.en1992.m-01.alpha-cc-accidental", "ndp.en1992.m-01.gamma-c-accidental"]).build());
        }

        let v_rd_c = part_1_1::shear_v_rd_c_n(b, d, geo.area, f_ck, rho, uv.n_ed, annex);
        let z = 0.9 * d;
        let sigma_cp = if geo.area > 0.0 { (-uv.n_ed).max(0.0) / geo.area } else { 0.0 };
        let asw_s = member.asw_per_s();
        let f_ywd = params.f_yd_pa(f_yk);
        let (v_rd_cs, cot) = part_1_1::shear_capacity_n(b, z, f_ck, f_yk, asw_s, sigma_cp, uv.v_ed.abs(), annex);
        let v_rd_s = if asw_s > 0.0 { part_1_1::shear_v_rd_s_n(asw_s, z, f_ywd, cot) } else { 0.0 };
        let v_rd_max = part_1_1::shear_v_rd_max_n(b, z, f_ck, cot, annex);
        let v_limit = if asw_s > 0.0 { if uv.v_ed.abs() <= v_rd_c { v_rd_c } else { v_rd_cs } } else { v_rd_c.min(0.5 * b * d * 0.6 * (1.0 - f_ck / 250.0e6) * params.f_cd_pa(f_ck)) };
        let mut shear = CheckResult::assess(format!("en1992.6.2.shear.{}", member.id), part_11(), clause_11("§6.2"), member_ref(doc, member, &member_path(&member.id, "actions")), l("Shear resistance", "Querkrafttragfähigkeit")).remedies_attached_by("attach_minimal_remedies")
            .utilization(force_n(uv.v_ed), force_n(v_limit.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("V_Ed={:.1} kN; V_Rd,c={:.1}, V_Rd,s={:.1}, V_Rd,max={:.1} kN; cotθ={:.2}; governing {}", uv.v_ed / 1e3, v_rd_c / 1e3, v_rd_s / 1e3, v_rd_max / 1e3, cot, uv.combination_id),
                &format!("V_Ed={:.1} kN; V_Rd,c={:.1}, V_Rd,s={:.1}, V_Rd,max={:.1} kN; cotθ={:.2}; maßgebend {}", uv.v_ed / 1e3, v_rd_c / 1e3, v_rd_s / 1e3, v_rd_max / 1e3, cot, uv.combination_id),
            ));
        if let Some(profile) = Profile::of(member) {
            shear = shear.sampled(STATIONS, |s| {
                let station = Station::ratio_of(&[s], force_n(uv.v_ed.abs() * profile.shear(s)), force_n(v_limit.max(1.0)));
                (station.utilization, station.values)
            });
        }
        out.push(ndps(shear, &["ndp.en1992.v-01.1", "ndp.en1992.v-02.1", "ndp.en1992.v-03.1", "ndp.en1992.v-03.2", "ndp.en1992.e-03.1"]).build());

        if ut.t_ed > 0.0 {
            let tors = sec.torsion_section(torsion_t_min_mm(member));
            let t_rd = tors.map_or(0.0, |t| part_1_1::torsion_t_rd_nm(f_ck, t.a_k * 1.0e-6, t.t_ef * 1.0e-3, annex));
            let tor = CheckResult::assess(format!("en1992.6.3.torsion.{}", member.id), part_11(), clause_11("§6.3"), member_ref(doc, member, &member_path(&member.id, "actions")), l("Torsion resistance", "Torsionstragfähigkeit")).remedies_attached_by("attach_minimal_remedies")
                .utilization(moment_nm(ut.t_ed), moment_nm(t_rd.max(1.0)))
                .annex(annex)
                .explanation(l(
                    &format!("Torsion check: T_Ed={:.2} kNm ≤ T_Rd={:.2} kNm (gov {})", ut.t_ed / 1e3, t_rd / 1e3, ut.combination_id),
                    &format!("Torsionsnachweis: T_Ed={:.2} kNm ≤ T_Rd={:.2} kNm (maßgebend {})", ut.t_ed / 1e3, t_rd / 1e3, ut.combination_id),
                ));
            out.push(ndps(tor, &["ndp.en1992.t-04.1"]).build());
            let case = torsion_case(member, sec, annex, f_ck, f_yk, b, d, a_s, ut.t_ed, uv.v_ed, cot);
            if let Some(stirrups) = member.stirrups.as_ref().filter(|s| s.spacing > 0.0 && s.diameter > 0.0) {
                let leg_mm2_per_mm = std::f64::consts::PI * (stirrups.diameter * 500.0).powi(2) / (stirrups.spacing * 1e3);
                let (tension_bars, compression_bars, tension_share) = torsion_chords(sec);
                let shear_share = uv.v_ed.abs() / (z * 1e3 * f_ywd * 1e-6 * cot) / f64::from(stirrups.legs.max(1));
                let reinforced = case.clone().with_n("asw_s_provided", (leg_mm2_per_mm - shear_share).max(1e-9)).with_n("asl_provided", tension_bars.max(1e-3)).with_n("asl_bending", tension_bars * uls.m_ed.abs() / m_rd.max(1.0)).with_n("tension_share", tension_share).with_n("asl_compression_provided", compression_bars.max(1e-3));
                if let Ok(outcomes) = evaluate_outcomes("T-02", &reinforced) {
                    for (key, id, en, de, clause) in [
                        ("util_links", "en1992.6.3.torsion-links", "Torsion links (one leg after the shear share)", "Torsionsbügel (ein Schenkel nach Abzug des Querkraftanteils)", "§6.3.2(3), Eq. 6.28"),
                        ("util_long", "en1992.6.3.torsion-longitudinal", "Torsion longitudinal steel in the tension chord (added to the bending steel)", "Torsionslängsbewehrung im Zuggurt (zusätzlich zur Biegebewehrung)", "§6.3.2(3), Eq. 6.28"),
                        ("util_long_comp", "en1992.6.3.torsion-longitudinal-compression", "Torsion longitudinal steel in the compression chord", "Torsionslängsbewehrung im Druckgurt", "§6.3.2(3), Eq. 6.28"),
                    ] {
                        let Some(o) = outcomes.iter().find(|o| o.key == key) else { continue };
                        let ratio = Quantity::new(QuantityKind::Dimensionless, o.utilization());
                        let result = CheckResult::assess(format!("{id}.{}", member.id), part_11(), clause_11(clause), member_ref(doc, member, &member_path(&member.id, "stirrups")), l(en, de)).remedies_attached_by("attach_minimal_remedies")
                            .utilization(ratio, Quantity::new(QuantityKind::Dimensionless, 1.0))
                            .annex(annex)
                            .explanation(l(
                                &format!("Required over provided = {:.3} (T_Ed={:.2} kNm, cot θ={:.2}, governing {})", o.utilization(), ut.t_ed / 1e3, cot, ut.combination_id),
                                &format!("Erforderlich zu vorhanden = {:.3} (T_Ed={:.2} kNm, cot θ={:.2}, maßgebend {})", o.utilization(), ut.t_ed / 1e3, cot, ut.combination_id),
                            ));
                        out.push(ndps(result, &["ndp.en1992.t-04.1"]).build());
                    }
                }
            }
            if let Ok(outcomes) = evaluate_outcomes("T-04", &case) {
                for o in outcomes.iter().filter(|o| o.key == "util_max") {
                    let u = o.utilization();
                    let ix = CheckResult::assess(format!("en1992.6.3.interaction.{}", member.id), part_11(), clause_11("§6.3.2"), member_ref(doc, member, &member_path(&member.id, "actions")), l("Combined torsion and shear", "Torsion und Querkraft kombiniert")).remedies_attached_by("attach_minimal_remedies")
                        .utilization(Quantity::new(QuantityKind::Dimensionless, u), Quantity::new(QuantityKind::Dimensionless, 1.0))
                        .annex(annex)
                        .explanation(l(&format!("T_Ed/T_Rd,max + V_Ed/V_Rd,max = {:.3} ≤ 1 (governing T: {}, governing V: {})", u, ut.combination_id, uv.combination_id), &format!("T_Ed/T_Rd,max + V_Ed/V_Rd,max = {:.3} ≤ 1 (maßgebend T: {}, maßgebend V: {})", u, ut.combination_id, uv.combination_id)));
                    out.push(ndps(ix, &["ndp.en1992.t-04.1"]).build());
                }
            }
        }

        if matches!(member.kind, MemberKind::FlatSlab) {
            if let Some(p) = &member.punching {
                let pos = match p.column_position { semio_s_artifact_norm_en1992_model::ColumnPosition::Edge => Position::Edge, semio_s_artifact_norm_en1992_model::ColumnPosition::Corner => Position::Corner, _ => Position::Interior };
                let outcome = match part_1_1::punching_beta_of(p, d, up.v_ed_punch, annex) {
                    Ok(outcome) => outcome,
                    Err(refusal) => {
                        let (reason_en, reason_de) = refusal.label();
                        out.push(CheckResult::assess(format!("en1992.6.4.punching.{}", member.id), part_11(), clause_11("§6.4"), member_ref(doc, member, &member_path(&member.id, "punching")), l("Punching shear", "Durchstanztragfähigkeit")).not_applicable(l(reason_en, reason_de)).annex(annex).build());
                        return;
                    }
                };
                let beta = outcome.beta;
                let (rule_en, rule_de) = outcome.rule.label();
                let u0 = u0_rect(pos, p.column_width * 1e3, p.column_depth * 1e3, d * 1e3) / 1e3;
                let rects: Vec<OpeningRect> = member.openings.iter().map(|o| OpeningRect { x: o.x, y: o.y, width: o.width, depth: o.depth }).collect();
                let wedges = match effective_wedges(p.column_width, p.column_depth, d, &rects) {
                    Ok(wedges) => wedges,
                    Err(refusal) => {
                        let (reason_en, reason_de) = refusal.label();
                        out.push(CheckResult::assess(format!("en1992.6.4.punching.{}", member.id), part_11(), clause_11("§6.4.2"), member_ref(doc, member, &member_path(&member.id, "openings")), l("Punching shear", "Durchstanztragfähigkeit")).not_applicable(l(reason_en, reason_de)).annex(annex).build());
                        return;
                    }
                };
                let u1_full = u1_rect(pos, p.column_width * 1e3, p.column_depth * 1e3, d * 1e3, 0.0, 0.0) / 1e3;
                let u1_lost = if wedges.is_empty() { 0.0 } else { perimeter_lengths(pos, p.column_width, p.column_depth, 2.0 * d, &wedges).1 };
                let u1 = u1_full - u1_lost;
                if u1 <= 1e-9 {
                    let (reason_en, reason_de) = OpeningRefusal::NoEffectivePerimeter.label();
                    out.push(CheckResult::assess(format!("en1992.6.4.punching.{}", member.id), part_11(), clause_11("§6.4.2"), member_ref(doc, member, &member_path(&member.id, "openings")), l("Punching shear", "Durchstanztragfähigkeit")).not_applicable(l(reason_en, reason_de)).annex(annex).build());
                    return;
                }
                let u0_over_d = u0 / d.max(1e-6);
                let v_rd_c_pa = part_1_1::punching_v_rd_c_pa(f_ck, d, rho, annex, pos, u0_over_d);
                let v_rd_max_pa = part_1_1::punching_v_rd_max_pa(v_rd_c_pa, f_ck, annex);
                let v_ed_pa = if u1 * d > 0.0 { beta * up.v_ed_punch / (u1 * d) } else { 0.0 };
                let v_ed_ref = if matches!(annex, AnnexChoice::De) || u0 * d <= 0.0 { v_ed_pa } else { beta * up.v_ed_punch / (u0 * d) };
                let fywd_ef = (ndp(annex, "ndp.en1992.punch-fywd-ef-base") * 1.0e6 + ndp(annex, "ndp.en1992.punch-fywd-ef-slope") * d * 1.0e3 * 1.0e6).min(params.f_yd_pa(f_yk));
                let asw = member.stud_asw().unwrap_or(p.asw);
                let s_r = member.stud_radial_spacing().unwrap_or(0.75 * d);
                let v_rd_cs_pa = if asw > 0.0 {
                    ndp(annex, "ndp.en1992.punch-cs-vrdc-factor") * v_rd_c_pa + ndp(annex, "ndp.en1992.punch-cs-asw-factor") * (d / s_r) * asw * fywd_ef / (u1 * d).max(1e-9)
                } else {
                    v_rd_c_pa
                };
                let mut punch = CheckResult::assess(format!("en1992.6.4.punching.{}", member.id), part_11(), clause_11("§6.4"), member_ref(doc, member, &member_path(&member.id, "punching")), l("Punching shear", "Durchstanztragfähigkeit")).remedies_attached_by("attach_minimal_remedies")
                    .utilization(stress_pa(v_ed_pa), stress_pa(v_rd_cs_pa.max(1.0)))
                    .annex(annex)
                    .explanation(l(
                        &format!("Punching: v_Ed={:.2} MPa (β={:.2} [{}], u₁={:.2} m less {:.2} m between the opening tangents, u₀/d={:.2}) ≤ v_Rd,c={:.2} MPa; A_sw per perimeter={:.0} mm², s_r={:.0} mm ({})", v_ed_pa / 1e6, beta, rule_en, u1, u1_lost, u0_over_d, v_rd_cs_pa / 1e6, asw * 1e6, s_r * 1e3, up.combination_id),
                        &format!("Durchstanzen: v_Ed={:.2} MPa (β={:.2} [{}], u₁={:.2} m abzüglich {:.2} m zwischen den Öffnungstangenten, u₀/d={:.2}) ≤ v_Rd,c={:.2} MPa; A_sw je Rundschnitt={:.0} mm², s_r={:.0} mm ({})", v_ed_pa / 1e6, beta, rule_de, u1, u1_lost, u0_over_d, v_rd_cs_pa / 1e6, asw * 1e6, s_r * 1e3, up.combination_id),
                    ));
                if u1 > 0.0 && d > 0.0 {
                    let base = up.v_ed_punch / (u1 * d);
                    let stations = control_perimeter(p.column_position, p.column_width, p.column_depth, d, member.span, &wedges)
                        .into_iter()
                        .map(|q| {
                            let v = base * local_factor(p.column_position, beta, q.lever);
                            Station::ratio_of(&[q.x, q.y], stress_pa(v), stress_pa(v_rd_cs_pa.max(1.0)))
                        })
                        .collect();
                    punch = punch.field("punching-perimeter", stations);
                }
                out.push(ndps(punch, &["ndp.en1992.p-03.1", "ndp.en1992.p-03.2", "ndp.en1992.p-03.3", "ndp.en1992.p-04.1", "ndp.en1992.punch-small-u0.limit", "ndp.en1992.punch-small-u0.slope", "ndp.en1992.punch-small-u0.offset", "ndp.en1992.punch-small-u0.floor"]).build());
                let pmax = CheckResult::assess(format!("en1992.6.4.punching-max.{}", member.id), part_11(), clause_11("§6.4.5"), member_ref(doc, member, &member_path(&member.id, "punching")), l("Punching crushing limit", "Durchstanzen Druckstrebe")).remedies_attached_by("attach_minimal_remedies")
                    .utilization(stress_pa(v_ed_ref), stress_pa(v_rd_max_pa.max(1.0)))
                    .annex(annex)
                    .explanation(l("Punching strut: v_Ed ≤ v_Rd,max (DE-NA 1.4·v_Rd,c)", "Durchstanz-Druckstrebe: v_Ed ≤ v_Rd,max (DE-NA 1,4·v_Rd,c)"));
                out.push(ndps(pmax, &["ndp.en1992.p-03.3", "ndp.en1992.p-04.1", "ndp.en1992.p-05.1"]).build());
                if let Some(outer) = member.stud_reach() {
                    let u_out = beta * up.v_ed_punch / (v_rd_c_pa.max(1.0) * d);
                    let a_out = if wedges.is_empty() { offset_of_perimeter(pos, p.column_width, p.column_depth, u_out) } else { offset_of_effective_perimeter(pos, p.column_width, p.column_depth, u_out, &wedges) };
                    let reach = outer + k_out(annex) * d;
                    let uout = CheckResult::assess(format!("en1992.6.4.uout.{}", member.id), part_11(), clause_11("§6.4.5"), member_ref(doc, member, &member_path(&member.id, "studRows")), l("Outer control perimeter beyond the studs", "Äußerer Rundschnitt außerhalb der Dübel")).remedies_attached_by("attach_minimal_remedies")
                        .utilization(Quantity::new(QuantityKind::Dimensionless, a_out.max(0.0) / reach.max(1e-6)), Quantity::new(QuantityKind::Dimensionless, 1.0))
                        .annex(annex)
                        .explanation(l(
                            &format!("u_out = β V_Ed/(v_Rd,c d) = {:.2} m reaches {:.0} mm from the column face; the outermost stud at {:.0} mm plus k d = {:.0} mm must cover it", u_out, a_out * 1e3, outer * 1e3, reach * 1e3),
                            &format!("u_out = β V_Ed/(v_Rd,c d) = {:.2} m reicht {:.0} mm von der Stützenkante; der äußerste Dübel bei {:.0} mm plus k d = {:.0} mm muss ihn abdecken", u_out, a_out * 1e3, outer * 1e3, reach * 1e3),
                        ));
                    out.push(ndps(uout, &["ndp.en1992.punch-k-out", "ndp.en1992.p-04.1"]).build());
                }
                if let Some(layout) = stud_layout(member, pos, p.column_width, p.column_depth) {
                    let (utilisation, worst) = (layout.utilisation(d), layout.governing(d));
                    let studs = CheckResult::assess(format!("en1992.9.4.3.studs.{}", member.id), part_11(), clause_11("§9.4.3"), member_ref(doc, member, &member_path(&member.id, "studRows")), l("Shear stud layout", "Dübelanordnung")).remedies_attached_by("attach_minimal_remedies")
                        .utilization(Quantity::new(QuantityKind::Dimensionless, utilisation), Quantity::new(QuantityKind::Dimensionless, 1.0))
                        .annex(annex)
                        .explanation(l(
                            &format!("Studs: {} rows x up to {} studs, s_0={:.0} mm (0.3 d … 0.5 d), s_r={:.0} mm (≤ 0.75 d), s_t={:.0} mm (≤ {:.1} d at r={:.0} mm); governing {}", layout.rows, layout.studs_per_row, layout.s0 * 1e3, layout.s_r * 1e3, layout.s_t * 1e3, layout.st_limit_factor, layout.s_t_radius * 1e3, worst),
                            &format!("Dübel: {} Reihen x bis zu {} Dübel, s_0={:.0} mm (0,3 d … 0,5 d), s_r={:.0} mm (≤ 0,75 d), s_t={:.0} mm (≤ {:.1} d bei r={:.0} mm); maßgebend {}", layout.rows, layout.studs_per_row, layout.s0 * 1e3, layout.s_r * 1e3, layout.s_t * 1e3, layout.st_limit_factor, layout.s_t_radius * 1e3, worst),
                        ));
                    out.push(ndps(studs, &["ndp.en1992.p-04.1"]).build());
                }
            }
        }
    } else {
        out.push(
            CheckResult::assess(format!("en1992.uls.na.{}", member.id), part_11(), clause_11("§6"), member_ref(doc, member, &member_path(&member.id, "actions")), l("ULS actions", "Einwirkungen GZT"))
                .not_applicable(l("No ULS load case on member", "Kein GZT-Lastfall am Bauteil"))
                .annex(annex)
                .build(),
        );
    }
}

/// 🪓 Checks the crack width of the quasi permanent situation.
pub(super) fn check_crack_width(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    f_ck: f64,
    e_s: f64,
    sec: &Sec,
    geo: &Geo,
    sls_qp: Option<&DesignEffects>,
) {
    let a_s = geo.a_s;
    if let Some(sls) = sls_qp {
        let w_max = part_1_1::w_max_m(annex, member.exposure);
        let case = crack_case(member, sec, annex, f_ck, e_s, sls.m_ed, sls.n_ed);
        let w_k = match crack_width(&case) { Ok(k) => k.wk / 1e3, Err(e) if e.starts_with("section is uncracked") => 0.0, Err(_) => f64::INFINITY };
        let mut crack = CheckResult::assess(format!("en1992.7.3.crack.{}", member.id), part_11(), clause_11("§7.3.4"), member_ref(doc, member, &member_path(&member.id, "actions")), l("Crack width", "Rissbreite")).remedies_attached_by("attach_minimal_remedies")
            .utilization(length_m(w_k), length_m(w_max))
            .annex(annex)
            .explanation(l(
                &format!("Crack width w_k={:.3} mm ≤ w_max={:.2} mm (SLS {})", w_k * 1000.0, w_max * 1000.0, sls.combination_id),
                &format!("Rissbreite w_k={:.3} mm ≤ w_max={:.2} mm (GZG {})", w_k * 1000.0, w_max * 1000.0, sls.combination_id),
            ));
        if let Some(profile) = Profile::of(member) {
            crack = crack.sampled(STATIONS, |s| {
                let w = crack_width(&case.clone().with_n("m_qp", sls.m_ed.abs() * profile.moment(s) * 1e3)).map(|k| k.wk / 1e3).unwrap_or(0.0);
                (w / w_max.max(1e-9), vec![length_m(w), length_m(w_max)])
            });
        }
        out.push(ndps(crack, &["ndp.en1992.sl-06.1"]).build());
    }
}

/// 🕸️ The statically determinate truss of a deep beam in the member plane: nodes `(x along the span, z up from the centroid)`, members `(from, to)` (tie first, then the two struts), their axial forces in N (tension positive), the point load in N and the required tie area in m². See EN 1992-1-1 6.5.
#[derive(Clone, Debug, PartialEq)]
pub struct DeepTruss {
    pub nodes: [(f64, f64); 3],
    pub members: [(usize, usize); 3],
    pub forces: Vec<f64>,
    pub load: f64,
    pub tie_area: f64,
}

/// 📐 Largest span-to-height ratio of a deep beam whose D-region is verified by the truss (6.5, 5.3.1(3)).
pub const DEEP_BEAM_RATIO: f64 = 3.0;

/// 🧮 The truss of a deep beam (span not above [`DEEP_BEAM_RATIO`] times the height): the support nodes 0,15 h from the ends, the tie at the effective depth (0,1 h above the bottom without one), the top node at mid-span 0,1 h below the top; the point load is twice the governing ultimate shear and the tie area is its force over f_yd. `None` for other members, without ultimate shear or reinforcement grade, or without a tie in tension.
pub fn deep_beam_truss(doc: &En1992Snapshot, m: &RcMember) -> Option<DeepTruss> {
    if m.kind != MemberKind::Beam || m.span <= 0.0 || m.height <= 0.0 || m.span / m.height > DEEP_BEAM_RATIO {
        return None;
    }
    let effects = combine_member_actions(m, &doc.action_factors());
    let load = 2.0 * governing(&effects, "uls")?.v_ed.abs();
    let steel = doc.reinforcement(&m.reinforcement_grade_id)?;
    if load <= 0.0 {
        return None;
    }
    let (l, h) = (m.span, m.height);
    let z_tie = if m.effective_depth > 0.0 && m.effective_depth < h { h / 2.0 - m.effective_depth } else { -h / 2.0 + 0.1 * h };
    let nodes = [(0.15 * h, z_tie), (l - 0.15 * h, z_tie), (l / 2.0, h / 2.0 - 0.1 * h)];
    let members = [(0usize, 1usize), (0, 2), (1, 2)];
    let (x, z): (Vec<f64>, Vec<f64>) = nodes.iter().copied().unzip();
    let forces = solve_truss(&x, &z, &members.map(|p| p.0), &members.map(|p| p.1), &[0.0, 0.0, 0.0], &[0.0, 0.0, -load], &[(0, 0), (0, 1), (1, 1)])?;
    let tie = forces.first().copied().filter(|f| *f > 0.0)?;
    Some(DeepTruss { nodes, members, forces: forces.clone(), load, tie_area: tie / (f_yd(doc.annex, steel.f_yk / 1.0e6, false) * 1.0e6) })
}

/// 🕸 Checks the tie of the strut-and-tie model of a deep beam: the required area `F_td/f_yd` of the truss against the provided tension reinforcement (6.5.4, ST-02).
pub(super) fn check_strut_and_tie(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, f_ck: f64, geo: &Geo) {
    let Some(truss) = deep_beam_truss(doc, member) else { return };
    let tie = CheckResult::assess(format!("en1992.6.5.tie.{}", member.id), part_11(), clause_11("§6.5.4"), member_ref(doc, member, &member_path(&member.id, "longitudinal")), l("Strut-and-tie: tie reinforcement", "Stabwerkmodell: Zugband")).remedies_attached_by("attach_minimal_remedies")
        .utilization(area_m2(truss.tie_area), area_m2(geo.a_s.max(1e-9)))
        .annex(annex)
        .explanation(l(
            &format!("Deep beam (L/h ≤ {:.0}): F = {:.1} kN, tie force {:.1} kN needs A_s,req = {:.0} mm² ≤ A_s,prov = {:.0} mm²", DEEP_BEAM_RATIO, truss.load / 1e3, truss.forces[0] / 1e3, truss.tie_area * 1e6, geo.a_s * 1e6),
            &format!("Wandartiger Träger (L/h ≤ {:.0}): F = {:.1} kN, Zugbandkraft {:.1} kN braucht A_s,erf = {:.0} mm² ≤ A_s,vorh = {:.0} mm²", DEEP_BEAM_RATIO, truss.load / 1e3, truss.forces[0] / 1e3, truss.tie_area * 1e6, geo.a_s * 1e6),
        ));
    out.push(tie.build());
    let subject = || member_ref(doc, member, &member_path(&member.id, "bearingWidth"));
    let a_mm = member.bearing_width * 1.0e3;
    if a_mm <= 0.0 {
        for (id, clause, en, de) in [("en1992.6.5.strut", "§6.5.2", "Strut-and-tie: concrete strut", "Stabwerkmodell: Betondruckstrebe"), ("en1992.6.5.node-cct", "§6.5.4", "Strut-and-tie: support node CCT", "Stabwerkmodell: Auflagerknoten CCT"), ("en1992.6.5.node-ccc", "§6.5.4", "Strut-and-tie: load node CCC", "Stabwerkmodell: Lastknoten CCC")] {
            out.push(CheckResult::assess(format!("{id}.{}", member.id), part_11(), clause_11(clause), subject(), l(en, de)).not_applicable(l("The bearing plate width a is not given; no strut width is assumed.", "Die Auflagerplattenbreite a ist nicht angegeben; es wird keine Strebenbreite angenommen.")).annex(annex).build());
        }
        return;
    }
    let ((x0, z0), (xa, za)) = (truss.nodes[0], truss.nodes[2]);
    let theta = (za - z0).atan2(xa - x0);
    let width_mm = a_mm * theta.sin();
    let b_mm = member.width * 1.0e3;
    let f_c = truss.forces[1].abs().max(truss.forces[2].abs());
    let base = Case::new(&member.id, annex).with_n("fck", f_ck / 1.0e6);
    let strut_case = base.clone().with_s("kind", "cracked").with_n("strut_force", f_c).with_n("b", b_mm).with_n("w", width_mm);
    let reaction = truss.load / 2.0;
    let cct_case = base.clone().with_s("node", "CCT").with_n("sigma_c", reaction / (b_mm * a_mm));
    let ccc_case = base.with_s("node", "CCC").with_n("sigma_c", truss.load / (b_mm * a_mm));
    let ndp_strut = ["ndp.en1992.strut-cracked", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent"];
    let ndp_node = ["ndp.en1992.node-k1", "ndp.en1992.node-k2", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent"];
    for (id, clause, en, de, entry, case, ndp_ids, what_en, what_de) in [
        ("en1992.6.5.strut", "§6.5.2", "Strut-and-tie: concrete strut", "Stabwerkmodell: Betondruckstrebe", "ST-01", strut_case, &ndp_strut[..], format!("σ_c = F_c/(b w) with F_c = {:.1} kN, w = a sin θ = {:.0} mm, transverse tension assumed (0.6 ν' f_cd)", f_c / 1.0e3, width_mm), format!("σ_c = F_c/(b w) mit F_c = {:.1} kN, w = a sin θ = {:.0} mm, Querzug angenommen (0,6 ν' f_cd)", f_c / 1.0e3, width_mm)),
        ("en1992.6.5.node-cct", "§6.5.4", "Strut-and-tie: support node CCT", "Stabwerkmodell: Auflagerknoten CCT", "ST-03", cct_case, &ndp_node[..], format!("σ_c = R/(b a) with R = {:.1} kN under the support plate a = {:.0} mm (strut, tie and reaction meet)", reaction / 1.0e3, a_mm), format!("σ_c = R/(b a) mit R = {:.1} kN unter der Auflagerplatte a = {:.0} mm (Strebe, Zugband und Auflagerkraft treffen sich)", reaction / 1.0e3, a_mm)),
        ("en1992.6.5.node-ccc", "§6.5.4", "Strut-and-tie: load node CCC", "Stabwerkmodell: Lastknoten CCC", "ST-03", ccc_case, &ndp_node[..], format!("σ_c = F/(b a) with F = {:.1} kN under the load plate a = {:.0} mm (two struts and the load meet)", truss.load / 1.0e3, a_mm), format!("σ_c = F/(b a) mit F = {:.1} kN unter der Lastplatte a = {:.0} mm (zwei Streben und die Last treffen sich)", truss.load / 1.0e3, a_mm)),
    ] {
        let Ok(outcomes) = evaluate_outcomes(entry, &case) else { continue };
        let (Some(u), Some(lim)) = (outcomes.iter().find(|o| o.key == "util"), outcomes.iter().find(|o| o.key == "sigma_rd_max")) else { continue };
        let result = CheckResult::assess(format!("{id}.{}", member.id), part_11(), clause_11(clause), subject(), l(en, de)).remedies_attached_by("attach_minimal_remedies")
            .utilization(Quantity::new(QuantityKind::Dimensionless, u.utilization()), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .annex(annex)
            .explanation(l(&format!("{what_en}; σ_Rd,max = {:.2} N/mm²", lim.capacity / 1.0e6), &format!("{what_de}; σ_Rd,max = {:.2} N/mm²", lim.capacity / 1.0e6)));
        out.push(ndps(result, ndp_ids).build());
    }
}

/// 🔩 Checks the minimum and maximum tension reinforcement.
pub(super) fn check_reinforcement_limits(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    f_ck: f64,
    f_yk: f64,
    geo: &Geo,
) {
    let a_s = geo.a_s;
    let a_min = part_1_1::a_s_min_m2(f_ck, f_yk, geo.b_t, geo.d);
    let a_max = part_1_1::a_s_max_m2(geo.area);
    let amin = CheckResult::assess(format!("en1992.9.minas.{}", member.id), part_11(), clause_11("§9.2.1"), member_ref(doc, member, &member_path(&member.id, "longitudinal")), l("Minimum reinforcement", "Mindestbewehrung")).remedies_attached_by("attach_minimal_remedies")
        .minimum(area_m2(a_s), area_m2(a_min))
        .annex(annex)
        .explanation(l(
            &format!("Minimum steel: A_s={:.0} mm² ≥ A_s,min={:.0} mm²", a_s * 1e6, a_min * 1e6),
            &format!("Mindestbewehrung: A_s={:.0} mm² ≥ A_s,min={:.0} mm²", a_s * 1e6, a_min * 1e6),
        ));
    out.push(ndps(amin, &[]).build());
    let amax = CheckResult::assess(format!("en1992.9.maxas.{}", member.id), part_11(), clause_11("§9.2.1"), member_ref(doc, member, &member_path(&member.id, "longitudinal")), l("Maximum reinforcement", "Höchstbewehrung")).remedies_attached_by("attach_minimal_remedies")
        .utilization(area_m2(a_s), area_m2(a_max.max(1e-9)))
        .annex(annex)
        .explanation(l(
            &format!("Maximum steel: A_s={:.0} mm² ≤ A_s,max={:.0} mm²", a_s * 1e6, a_max * 1e6),
            &format!("Höchstbewehrung: A_s={:.0} mm² ≤ A_s,max={:.0} mm²", a_s * 1e6, a_max * 1e6),
        ));
    out.push(ndps(amax, &[]).build());
}

/// 📉 Checks the span to depth deflection limit.
pub(super) fn check_deflection(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, f_ck: f64, geo: &Geo) {
    let (d, b_ref) = (geo.d, geo.b_c.max(geo.b_w));
    let rho = if b_ref * d > 0.0 { geo.a_s / (b_ref * d) } else { 0.0 };
    if member.span > 0.0 && matches!(member.kind, MemberKind::Beam | MemberKind::Slab | MemberKind::FlatSlab) {
        let a_top: f64 = member.longitudinal.iter().filter(|l| l.position == semio_s_artifact_norm_en1992_model::BarPosition::Top).map(|l| l.count as f64 * std::f64::consts::PI * l.diameter * l.diameter / 4.0).sum();
        let rho_prime = if b_ref * d > 0.0 { a_top / (b_ref * d) } else { 0.0 };
        let flange = if geo.b_w > 0.0 && geo.b_c / geo.b_w > ndp(annex, "ndp.en1992.ld-flange-ratio") { ndp(annex, "ndp.en1992.ld-flange-factor") } else { 1.0 };
        let lim_table = flange * part_1_1::basic_ld_limit(member.support, f_ck, rho.max(1e-4), rho_prime, annex);
        let k = part_1_1::support_k(member.support);
        let lim_de = part_1_1::de_ld_caps(k, member.span, member.deflection_sensitive);
        let lim = lim_table.min(lim_de);
        let ld = member.span / d.max(1e-6);
        let defl = CheckResult::assess(format!("en1992.7.4.ld.{}", member.id), part_11(), clause_11("§7.4.2"), member_ref(doc, member, &member_path(&member.id, "effectiveDepth")), l("Span-to-depth limit", "Biegeschlankheit l/d")).remedies_attached_by("attach_minimal_remedies")
            .utilization(Quantity::new(QuantityKind::Dimensionless, ld), Quantity::new(QuantityKind::Dimensionless, lim.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("l/d={:.1} ≤ {:.1} (Table 7.4N={:.1}; DE K·35 / K²·150/l={:.1}; K={:.2}; sensitive={}; flange factor {:.1})", ld, lim, lim_table, lim_de, k, member.deflection_sensitive, flange),
                &format!("l/d={:.1} ≤ {:.1} (Tab. 7.4N={:.1}; DE K·35 / K²·150/l={:.1}; K={:.2}; empfindlich={}; Gurtfaktor {:.1})", ld, lim, lim_table, lim_de, k, member.deflection_sensitive, flange),
            ));
        out.push(ndps(defl, &["ndp.en1992.sl-07.1"]).build());
    }
}

/// 🌊️ Checks the quasi-permanent deflection along the span against `L/250` (7.4.1 (4), registry row `ndp.en1992.defl-limit-qp`): the elastic line of the actual loads and support with creep (Annex B.1, RH 50 %, 28 d, 50 years) and the cracking interpolation of 7.4.3 (Eq. 7.18, `β = 0.5`).
pub(super) fn check_deflection_profile(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    concrete: &semio_s_artifact_norm_en1992_model::ConcreteGrade,
    e_s: f64,
    sec: &Sec,
    geo: &Geo,
    sls_qp: Option<&DesignEffects>,
) {
    let (Some(sls), Some(profile)) = (sls_qp, Profile::quasi_permanent(member, &doc.action_factors())) else { return };
    let h = geo.h;
    let h0 = 2.0e3 * geo.area / geo.u.max(1e-9);
    let phi = creep_phi(concrete.f_ck / 1.0e6 + 8.0, 100.0 * member.relative_humidity.clamp(0.2, 1.0), h0, 28.0, 18_250.0, "N");
    let e_eff = concrete.e_cm() / (1.0 + phi);
    let i_1 = geo.i_y;
    let mut cracked = sec.clone();
    cracked.c.ecm = e_eff / 1.0e6;
    cracked.es = e_s / 1.0e6;
    let i_2 = cracked.elastic(sls.m_ed.abs() * 1.0e3, -sls.n_ed, false).filter(|e| e.x < h * 1.0e3 - 1e-6).map_or(i_1, |e| e.i_ii * 1.0e-12);
    let m_cr = sec.cracking(-sls.n_ed).0 / 1.0e3;
    let zeta = if sls.m_ed.abs() > m_cr { 1.0 - 0.5 * (m_cr / sls.m_ed.abs()).powi(2) } else { 0.0 };
    let ei = e_eff / ((1.0 - zeta) / i_1 + zeta / i_2.max(1e-18));
    let divisor = ndp(annex, "ndp.en1992.defl-limit-qp");
    let limit = (if matches!(member.support, SupportCondition::Cantilever) { 2.0 * member.span } else { member.span }) / divisor;
    let w_at = |s: f64| profile.deflection_ei(s) / ei;
    let w_max = (0..=STATIONS).map(|i| w_at(i as f64 / STATIONS as f64)).fold(0.0, f64::max);
    let check = CheckResult::assess(format!("en1992.7.4.deflection.{}", member.id), part_11(), clause_11("§7.4.3"), member_ref(doc, member, &member_path(&member.id, "height")), l("Deflection along the span", "Durchbiegung entlang der Spannweite")).remedies_attached_by("attach_minimal_remedies")
        .utilization(length_m(w_max), length_m(limit.max(1e-9)))
        .annex(annex)
        .explanation(l(
            &format!("w_max={:.1} mm ≤ L/{divisor:.0}={:.1} mm (φ={:.2}, ζ={:.2}, EI={:.1} MNm², {})", w_max * 1e3, limit * 1e3, phi, zeta, ei / 1e6, sls.combination_id),
            &format!("w_max={:.1} mm ≤ L/{divisor:.0}={:.1} mm (φ={:.2}, ζ={:.2}, EI={:.1} MNm², {})", w_max * 1e3, limit * 1e3, phi, zeta, ei / 1e6, sls.combination_id),
        ))
        .sampled(STATIONS, |s| {
            let w = w_at(s);
            (w / limit.max(1e-9), vec![length_m(w), length_m(limit)])
        });
    out.push(ndps(check, &["ndp.en1992.defl-limit-qp"]).build());
}

/// ⚖ Checks the serviceability stress limits of the characteristic and quasi permanent situations.
pub(super) fn check_sls_stress_limits(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    f_ck: f64,
    f_yk: f64,
    sec: &Sec,
    geo: &Geo,
    sls_qp: Option<&DesignEffects>,
    sls_char: Option<&DesignEffects>,
) {
    let (d, a_s) = (geo.d, geo.a_s);
    let stresses = |e: &DesignEffects| -> (f64, f64) {
        let (m, n) = (e.m_ed.abs() * 1.0e3, -e.n_ed);
        sec.elastic(m, n, false).filter(|s| s.x < geo.h * 1.0e3 - 1e-6).or_else(|| sec.elastic(m, n, true)).map_or((0.0, 0.0), |s| (s.sigma_s * 1.0e6, s.sigma_c * 1.0e6))
    };
    if let Some(sc) = sls_char {
        let (sig_s, sig_c) = stresses(sc);
        let lim_s = ndp(annex, "ndp.en1992.sigma-s-char") * f_yk;
        let lim_c = c_char_factor(annex) * f_ck;
        let ss = CheckResult::assess(format!("en1992.7.2.sigma-s.{}", member.id), part_11(), clause_11("§7.2"), member_ref(doc, member, &member_path(&member.id, "longitudinal")), l("SLS steel stress", "GZG Stahlspannung")).remedies_attached_by("attach_minimal_remedies")
            .utilization(stress_pa(sig_s), stress_pa(lim_s.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("σ_s={:.0} MPa ≤ 0.8 f_yk={:.0} MPa (characteristic {})", sig_s / 1e6, lim_s / 1e6, sc.combination_id),
                &format!("σ_s={:.0} MPa ≤ 0,8 f_yk={:.0} MPa (charakteristisch {})", sig_s / 1e6, lim_s / 1e6, sc.combination_id),
            ));
        out.push(ndps(ss, &["ndp.en1992.sl-02.1"]).build());
        let mut scc = CheckResult::assess(format!("en1992.7.2.sigma-c.{}", member.id), part_11(), clause_11("§7.2"), member_ref(doc, member, &member_path(&member.id, "concreteGradeId")), l("SLS concrete stress", "GZG Betondruckspannung")).remedies_attached_by("attach_minimal_remedies")
            .utilization(stress_pa(sig_c), stress_pa(lim_c.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("σ_c={:.1} MPa ≤ {:.0}% f_ck={:.1} MPa (char {})", sig_c / 1e6, 60, lim_c / 1e6, sc.combination_id),
                &format!("σ_c={:.1} MPa ≤ {:.0}% f_ck={:.1} MPa (char {})", sig_c / 1e6, 60, lim_c / 1e6, sc.combination_id),
            ));
        scc = scc.explanation(l(
            &format!("Concrete stress σ_c={:.1} MPa ≤ 0.60 f_ck under characteristic SLS ({})", sig_c / 1e6, sc.combination_id),
            &format!("Betondruckspannung σ_c={:.1} MPa ≤ 0,60 f_ck unter charakteristischer GZG ({})", sig_c / 1e6, sc.combination_id),
        ));
        out.push(ndps(scc, &["ndp.en1992.sl-02.1"]).build());
    }
    if let Some(qp) = sls_qp {
        let sig_c = stresses(qp).1;
        let lim = ndp(annex, "ndp.en1992.sigma-c-qp") * f_ck;
        let qp_c = CheckResult::assess(format!("en1992.7.2.creep.{}", member.id), part_11(), clause_11("§7.2"), member_ref(doc, member, &member_path(&member.id, "concreteGradeId")), l("Quasi-permanent concrete stress", "Quasi-ständige Betondruckspannung")).remedies_attached_by("attach_minimal_remedies")
            .utilization(stress_pa(sig_c), stress_pa(lim.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("σ_c={:.1} MPa ≤ 0.45 f_ck={:.1} MPa for linear creep ({})", sig_c / 1e6, lim / 1e6, qp.combination_id),
                &format!("σ_c={:.1} MPa ≤ 0,45 f_ck={:.1} MPa für lineares Kriechen ({})", sig_c / 1e6, lim / 1e6, qp.combination_id),
            ));
        out.push(ndps(qp_c, &["ndp.en1992.sl-02.1"]).build());
    }
}

fn c_char_factor(annex: AnnexChoice) -> f64 {
    ndp(annex, "ndp.en1992.sigma-c-char")
}
