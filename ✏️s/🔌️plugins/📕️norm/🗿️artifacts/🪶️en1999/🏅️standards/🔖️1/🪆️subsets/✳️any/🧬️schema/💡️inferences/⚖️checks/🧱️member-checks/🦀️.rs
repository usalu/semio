//! 🧱️ EN 1999 member checks: the digests and the utilization helper behind `check_member`, which verifies one aluminium member against every clause.

use super::*;

//#region 🔖️ComplianceHelpers
fn elements_digest(section: &AluminiumSection) -> String {
    if section.elements.is_empty() {
        return format!("kind={k};D={d:.4};t={t:.4}", k=section.kind, d=section.outer_diameter, t=section.web_thickness);
    }
    section.elements.iter().map(|e| format!(
        "{id}:out={out}:weld={w}:wp={wp:.5}:b={b:.5}:t={th:.5}",
        id=e.id, out=e.outstand, w=e.welded, wp=e.weld_position, b=e.width, th=e.thickness
    )).collect::<Vec<_>>().join("|")
}

pub(super) fn actions_digest(actions: &[semio_s_artifact_norm_en1999_model::MemberAction]) -> String {
    actions.iter().map(|a| format!(
        "{id}/{kind}/{cat}/{src}/g={g:.4}/q={q:.4}/N={n:.4}/Vy={vy:.4}/Vz={vz:.4}/My={my:.4}/Mz={mz:.4}",
        id=a.id, kind=a.kind, cat=a.category, src=a.source,
        g=a.g_k_line, q=a.q_k_line, n=a.n_k, vy=a.v_y_k, vz=a.v_z_k, my=a.m_y_k, mz=a.m_z_k
    )).collect::<Vec<_>>().join(" | ")
}

fn member_support_digest(member: &AluminiumMember) -> String {
    format!(
        "support={sup};c1={c1:.4};LcrY={ly:.4};LcrZ={lz:.4};LcrT={lt:.4};Llt={llt:.4};restrained={r}",
        sup=member.support.code(), c1=member.c1, ly=member.buckling_length_y, lz=member.buckling_length_z,
        lt=member.buckling_length_t, llt=member.ltb_length, r=member.restrained_ltb
    )
}

pub(super) fn member_label(id: &str) -> LocalizedCopy {
    loc(&format!("Member {id}"), &format!("Bauteil {id}"))
}

pub(super) fn conn_label(id: &str) -> LocalizedCopy {
    loc(&format!("Connection {id}"), &format!("Anschluss {id}"))
}

pub(super) fn utilization_check(
    id: String,
    part: &str,
    clause: ClauseId,
    subject: SubjectRef,
    title: LocalizedCopy,
    computed: Quantity,
    limit: Quantity,
    annex: AnnexChoice,
    explanation: LocalizedCopy,
    mut remedies: Vec<Remedy>,
) -> CheckResult {
    let u = if limit.value.abs() > 0.0 { computed.value.abs() / limit.value.abs() } else if computed.value.abs() > 0.0 { f64::INFINITY } else { 0.0 };
    if u > 1.0 && remedies.is_empty() && !subject.path.is_empty() {
        let required = if subject.path.contains("bucklingLength") || subject.path.contains("ltbLength") {
            // Zero would fall back to member.length in L_cr — use a positive fraction of computed demand.
            Quantity::new(computed.kind, (computed.value.abs() / u.max(1.0) * 0.99).max(1e-6))
        } else {
            Quantity::new(computed.kind, computed.value * (1.0 / u.max(1.0)) * 0.99)
        };
        remedies.push(Remedy::at_most(
            subject.clone(),
            computed,
            required,
            loc("Reduce the governing demand until utilization ≤ 1.", "Maßgebende Beanspruchung senken, bis Ausnutzung ≤ 1."),
        ));
    }
    let mut b = CheckResult::assess(id, part, clause, subject, title)
        .utilization(computed, limit)
        .annex(annex)
        .explanation(explanation);
    for r in remedies { b = b.remedy(r); }
    b.build()
}

/// 📋️ Evaluate one member against EN 1999-1-1 cross-section and stability clauses.
pub fn check_member(
    member: &AluminiumMember,
    section: &AluminiumSection,
    material: &AluminiumMaterial,
    annex: AnnexChoice,
) -> Vec<CheckResult> {
    let params = na_de::AnnexParams::for_choice(annex);
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return vec![CheckResult::assess(
            format!("en1999.3.2.alloy.{}", member.id),
            "DIN EN 1999-1-1",
            ClauseId::new("EN 1999-1-1", "§3.2", "3.2"),
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            loc("Alloy designation", "Legierungsbezeichnung"),
        )
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .explanation(loc(
            &format!("Unknown alloy designation '{}'.", material.designation),
            &format!("Unbekannte Legierungsbezeichnung '{}'.", material.designation),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            part_1_1::CATALOGUE_ALLOYS.iter().map(|s| (*s).to_string()).collect(),
            loc("Pick a catalogue alloy from EN 1999-1-1 Table 3.2.", "Kataloglegierung nach EN 1999-1-1 Tabelle 3.2 wählen."),
        ))
        .build()];
    };
    let (a_gross, i_y, i_z, _wel_y, _wel_z, i_t, i_w) = part_1_1::section_geometry(section);
    let class = part_1_1::section_class(section, alloy.f_o_pa);
    let has_weld = section.elements.iter().any(|e| e.welded);
    let a_eff = part_1_1::effective_area(section, alloy.f_o_pa, has_weld, alloy.rho_o_haz, alloy.rho_u_haz);
    let wel_eff = part_1_1::effective_wel_y(section, alloy.f_o_pa, has_weld, alloy.rho_o_haz, alloy.rho_u_haz);
    let el_dig = elements_digest(section);
    let (alpha, lambda_0) = part_1_1::buckling_curve(alloy.buckling_class_a);
    let mut out = Vec::new();
    let part = "DIN EN 1999-1-1";

    let gov = part_en1990::governing_member_effects(member, annex);
    let action_id = gov.action_id.as_str();
    let (n_ed, v_y, v_z, m_y, m_z) = (gov.n_ed, gov.v_y_ed, gov.v_z_ed, gov.m_y_ed, gov.m_z_ed);
    let combo = gov.combination;
    let act_dig = actions_digest(&member.actions);
    let sup_dig = member_support_digest(member);

    // Classification: one check per plate element so outstand/weldPos leaves each govern.
    let eps = part_1_1::epsilon(alloy.f_o_pa);
    for el in &section.elements {
        let beta_eps = part_1_1::beta(el) / eps;
        let limit_beta = if el.outstand { 6.0 } else { 22.0 };
        let path = format!("sections[id={}].elements[id={}].thickness", section.id, el.id);
        let mut remedies = Vec::new();
        if beta_eps > limit_beta {
            let t_req = part_1_1::eta_factor(el.welded) * el.width / (limit_beta * eps);
            remedies.push(Remedy::at_least(
                SubjectRef::new(&section.id, path.clone(), loc(&format!("Plate {}", el.id), &format!("Blech {}", el.id))),
                q_length(el.thickness),
                q_length(t_req),
                loc(
                    &format!("Increase plate thickness of {} from {:.1} mm to at least {:.1} mm (class ≤ 3).", el.id, el.thickness * 1e3, t_req * 1e3),
                    &format!("Blechdicke von {} von {:.1} mm auf mindestens {:.1} mm erhöhen (Klasse ≤ 3).", el.id, el.thickness * 1e3, t_req * 1e3),
                ),
            ));
            remedies.push(Remedy::one_of(
                SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
                vec!["aw6082-t6".into(), "aw6061-t6".into(), "aw6063-t6".into()],
                loc("Select a higher-strength alloy/temper from Table 3.2.", "Höherfeste Legierung/Zustand nach Tabelle 3.2 wählen."),
            ));
        }
        out.push(utilization_check(
            format!("en1999.6.1.4.class.{}.{}", member.id, el.id),
            part,
            ClauseId::new("EN 1999-1-1", "§6.1.4", "6.1.4"),
            SubjectRef::new(&member.id, path, member_label(&member.id)),
            loc("Cross-section classification (Table 6.2)", "Querschnittsklasse (Tabelle 6.2)"),
            q_dim(beta_eps),
            q_dim(limit_beta),
            annex,
            loc(
                &format!("Plate {} β/ε = {beta_eps:.2} vs limit {limit_beta:.1} (class {:?}); outstand={}; weldPos={:.5}; {el_dig} (ULS {combo}, lead {action_id})", el.id, class, el.outstand, el.weld_position),
                &format!("Blech {} β/ε = {beta_eps:.2} gegenüber Grenzwert {limit_beta:.1} (Klasse {:?}); auskragend={}; Schweißlage={:.5}; {el_dig} (GZT {combo}, führend {action_id})", el.id, class, el.outstand, el.weld_position),
            ),
            remedies,
        ));
    }

    let path_n = format!("members[id={}].actions[id={}].nK", member.id, action_id);
    let path_m = format!("members[id={}].actions[id={}].mYK", member.id, action_id);
    let path_mz = format!("members[id={}].actions[id={}].mZK", member.id, action_id);
    let path_vy = format!("members[id={}].actions[id={}].vYK", member.id, action_id);
    let k_sup = match member.support {
        SupportCondition::Continuous => 0.7,
        SupportCondition::Cantilever => 2.0,
        SupportCondition::SimplySupported => 1.0,
    };
    let l_cr_y = k_sup * if member.buckling_length_y > 0.0 { member.buckling_length_y } else { member.length };
    let l_cr_z = k_sup * if member.buckling_length_z > 0.0 { member.buckling_length_z } else { member.length };
    let l_cr_t = k_sup * if member.buckling_length_t > 0.0 { member.buckling_length_t } else { member.length };
    let l_lt = if member.ltb_length > 0.0 { member.ltb_length } else { member.length };

    // Tension / compression §6.2.3 / 6.2.4
    let n_rd = a_eff * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_n = Vec::new();
    if n_ed.abs() > n_rd {
        let a_req = n_ed.abs() * params.gamma_m1 / alloy.f_o_pa;
        remedies_n.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i180".into(), "sec-box120".into()],
            loc(
                &format!("Select a larger section so A_eff ≥ {a_req:.2e} m² (N_Ed = {:.1} kN).", n_ed.abs() / 1e3),
                &format!("Größeren Querschnitt wählen, sodass A_eff ≥ {a_req:.2e} m² (N_Ed = {:.1} kN).", n_ed.abs() / 1e3),
            ),
        ));
        remedies_n.push(Remedy::one_of(
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            vec!["aw6082-t6".into(), "aw6061-t6".into()],
            loc("Upgrade alloy/temper to raise f_o.", "Legierung/Zustand erhöhen, um f_o zu steigern."),
        ));
        remedies_n.push(Remedy::at_most(
            SubjectRef::new(&member.id, path_n.clone(), member_label(&member.id)),
            q_force(n_ed),
            q_force(n_rd.copysign(n_ed)),
            loc(
                &format!("Reduce |N_Ed| from {:.1} kN to at most {:.1} kN.", n_ed.abs()/1e3, n_rd/1e3),
                &format!("|N_Ed| von {:.1} kN auf höchstens {:.1} kN reduzieren.", n_ed.abs()/1e3, n_rd/1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.2.3.n.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.3", "6.2.3"),
        SubjectRef::new(&member.id, path_n.clone(), member_label(&member.id)),
        loc("Axial resistance N_Rd (gross/net/HAZ)", "Normalkraftwiderstand N_Rd (brutto/netto/WEZ)"),
        q_force(n_ed.abs()),
        q_force(n_rd),
        annex,
        loc(
            &format!("N_Ed={:.1} kN, A_eff={:.1} mm², f_o={:.0} MPa, N_Rd={:.1} kN (ULS {combo}, lead {action_id}); {act_dig}; {sup_dig}; {el_dig}", n_ed/1e3, a_eff*1e6, alloy.f_o_pa/1e6, n_rd/1e3),
            &format!("N_Ed={:.1} kN, A_eff={:.1} mm², f_o={:.0} MPa, N_Rd={:.1} kN (GZT {combo}, führend {action_id}); {act_dig}; {sup_dig}; {el_dig}", n_ed/1e3, a_eff*1e6, alloy.f_o_pa/1e6, n_rd/1e3),
        ),
        remedies_n,
    ));

    // Bending §6.2.5
    let m_rd = wel_eff * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_m = Vec::new();
    if m_y.abs() > m_rd {
        remedies_m.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into(), "sec-chs168".into()],
            loc("Select a section with larger W_el,y.", "Querschnitt mit größerem W_el,y wählen."),
        ));
        if m_rd > 0.0 {
            remedies_m.push(Remedy::at_most(
                SubjectRef::new(&member.id, path_m.clone(), member_label(&member.id)),
                q_moment(m_y),
                q_moment(m_rd.copysign(m_y)),
                loc(
                    &format!("Reduce |M_y,Ed| from {:.2} kNm to at most {:.2} kNm.", m_y.abs()/1e3, m_rd/1e3),
                    &format!("|M_y,Ed| von {:.2} kNm auf höchstens {:.2} kNm reduzieren.", m_y.abs()/1e3, m_rd/1e3),
                ),
            ));
        } else {
            remedies_m.push(Remedy::at_most(
                SubjectRef::new(&member.id, path_m.clone(), member_label(&member.id)),
                q_moment(m_y),
                q_moment(0.0),
                loc("Reduce |M_y,Ed| to zero until section resistance is available.", "|M_y,Ed| auf null senken, bis Querschnittstragfähigkeit vorliegt."),
            ));
        }
    }
    out.push(utilization_check(
        format!("en1999.6.2.5.m.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.5", "6.2.5"),
        SubjectRef::new(&member.id, path_m.clone(), member_label(&member.id)),
        loc("Bending resistance M_c,Rd", "Biegewiderstand M_c,Rd"),
        q_moment(m_y.abs()),
        q_moment(m_rd),
        annex,
        loc(
            &format!("M_y,Ed={:.2} kNm, W_eff={:.0} mm³, M_c,Rd={:.2} kNm (ULS {combo}, lead {action_id}); {act_dig}; {sup_dig}", m_y/1e3, wel_eff*1e9, m_rd/1e3),
            &format!("M_y,Ed={:.2} kNm, W_eff={:.0} mm³, M_c,Rd={:.2} kNm (GZT {combo}, führend {action_id}); {act_dig}; {sup_dig}", m_y/1e3, wel_eff*1e9, m_rd/1e3),
        ),
        remedies_m,
    ));

    // Shear §6.2.6
    let a_v = part_1_1::shear_area(section);
    let v_rd = a_v * alloy.f_o_pa / (params.gamma_m1 * 3.0f64.sqrt());
    // web buckling reduction when slender
    let web = section.elements.iter().find(|e| !e.outstand);
    let mut v_rd_b = v_rd;
    if let Some(w) = web {
        let hw_t = if w.thickness > 0.0 { w.width / w.thickness } else { 0.0 };
        if hw_t > 39.0 * part_1_1::epsilon(alloy.f_o_pa) {
            v_rd_b *= (39.0 * part_1_1::epsilon(alloy.f_o_pa) / hw_t).min(1.0);
        }
    }
    let mut remedies_v = Vec::new();
    if v_z.abs() > v_rd_b && v_rd_b > 0.0 {
        remedies_v.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i180".into()],
            loc("Select a section with thicker web.", "Querschnitt mit dickeren Steg wählen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.2.6.v.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.6", "6.2.6"),
        SubjectRef::new(&member.id, format!("members[id={}].actions[id={}].vZK", member.id, action_id), member_label(&member.id)),
        loc("Shear resistance V_c,Rd (incl. web buckling)", "Querkraftwiderstand V_c,Rd (inkl. Stegbeulen)"),
        q_force(v_z.abs()),
        q_force(v_rd_b),
        annex,
        loc(
            &format!("V_z,Ed={:.1} kN, V_c,Rd={:.1} kN (ULS {combo}, lead {action_id}).", v_z/1e3, v_rd_b/1e3),
            &format!("V_z,Ed={:.1} kN, V_c,Rd={:.1} kN (GZT {combo}, führend {action_id}).", v_z/1e3, v_rd_b/1e3),
        ),
        remedies_v,
    ));

    // Bending about z §6.2.5 (weak axis)
    let (_, _, _, _, wel_z_geom, _, _) = part_1_1::section_geometry(section);
    let wel_z = if wel_z_geom > 0.0 {
        wel_z_geom
    } else if section.width > 0.0 {
        let (_, _, iz, _, _, _, _) = part_1_1::section_geometry(section);
        2.0 * iz / section.width
    } else {
        0.0
    };
    let m_rd_z = wel_z * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_mz = Vec::new();
    if m_z.abs() > m_rd_z && m_rd_z > 0.0 {
        remedies_mz.extend(scale_action_leaf_remedies(
            "members",
            &member.id,
            &member.actions,
            "mZK",
            |a| a.m_z_k,
            m_z,
            m_rd_z,
            q_moment,
            member_label(&member.id),
            &format!("Reduce action M_z,k so |M_z,Ed| ≤ {:.2} kNm.", m_rd_z/1e3),
            &format!("Einwirkung M_z,k senken, sodass |M_z,Ed| ≤ {:.2} kNm.", m_rd_z/1e3),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.2.5.mz.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.5", "6.2.5"),
        SubjectRef::new(&member.id, path_mz.clone(), member_label(&member.id)),
        loc("Bending resistance M_z,c,Rd", "Biegewiderstand M_z,c,Rd"),
        q_moment(m_z.abs()),
        q_moment(m_rd_z.max(1e-9)),
        annex,
        loc(
            &format!("M_z,Ed={:.2} kNm on L={:.2} m (ULS {combo}, lead {action_id}).", m_z/1e3, member.length),
            &format!("M_z,Ed={:.2} kNm bei L={:.2} m (GZT {combo}, führend {action_id}).", m_z/1e3, member.length),
        ),
        remedies_mz,
    ));

    // Shear V_y §6.2.6
    let mut remedies_vy = Vec::new();
    if v_y.abs() > v_rd_b && v_rd_b > 0.0 {
        remedies_vy.push(Remedy::at_most(
            SubjectRef::new(&member.id, path_vy.clone(), member_label(&member.id)),
            q_force(v_y),
            q_force(v_rd_b.copysign(v_y)),
            loc("Reduce |V_y,Ed|.", "|V_y,Ed| reduzieren."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.2.6.vy.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.6", "6.2.6"),
        SubjectRef::new(&member.id, path_vy.clone(), member_label(&member.id)),
        loc("Shear resistance V_y,c,Rd", "Querkraftwiderstand V_y,c,Rd"),
        q_force(v_y.abs()),
        q_force(v_rd_b.max(1e-9)),
        annex,
        loc(
            &format!("V_y,Ed={:.1} kN (ULS {combo}, lead {action_id}).", v_y/1e3),
            &format!("V_y,Ed={:.1} kN (GZT {combo}, führend {action_id}).", v_y/1e3),
        ),
        remedies_vy,
    ));

    // Combined §6.2.8 simplified
    let u_comb = if n_rd > 0.0 && m_rd > 0.0 { n_ed.abs()/n_rd + m_y.abs()/m_rd + (if m_rd_z > 0.0 { m_z.abs()/m_rd_z } else { 0.0 }) } else { 0.0 };
    let mut remedies_c = Vec::new();
    if u_comb > 1.0 {
        remedies_c.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into()],
            loc("Select a larger section to satisfy combined N–M.", "Größeren Querschnitt für N–M-Interaktion wählen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.2.8.nm.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.8", "6.2.8"),
        SubjectRef::new(&member.id, path_n.clone(), member_label(&member.id)),
        loc("Combined axial and bending", "Interaktion Normalkraft und Biegung"),
        q_dim(u_comb),
        q_dim(1.0),
        annex,
        loc(
            &format!("N_Ed/N_Rd + M_y,Ed/M_c,Rd = {u_comb:.3} (ULS {combo}, lead {action_id})."),
            &format!("N_Ed/N_Rd + M_y,Ed/M_c,Rd = {u_comb:.3} (GZT {combo}, führend {action_id})."),
        ),
        remedies_c,
    ));

    // Flexural buckling χ §6.3.1
    let lb_y = part_1_1::lambda_bar(l_cr_y, i_y, a_eff.max(a_gross), alloy.f_o_pa);
    let lb_z = part_1_1::lambda_bar(l_cr_z, i_z, a_eff.max(a_gross), alloy.f_o_pa);
    let chi_y = part_1_1::chi_from_lambda(lb_y, alpha, lambda_0);
    let chi_z = part_1_1::chi_from_lambda(lb_z, alpha, lambda_0);
    let chi = chi_y.min(chi_z);
    let n_b_rd_y = chi_y * a_eff * alloy.f_o_pa / params.gamma_m1;
    let n_b_rd_z = chi_z * a_eff * alloy.f_o_pa / params.gamma_m1;
    let n_b_rd = chi * a_eff * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_b = Vec::new();
    if n_ed.abs() > n_b_rd && n_b_rd > 0.0 {
        // invert for L_cr: need chi' such that chi'*A*f/γ = |N|
        let chi_req = (n_ed.abs() * params.gamma_m1 / (a_eff * alloy.f_o_pa)).min(1.0);
        // binary search L_cr
        let mut lo = 0.1; let mut hi = member.buckling_length_y.max(member.buckling_length_z).max(0.5);
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            let lb = part_1_1::lambda_bar(mid, i_y.min(i_z), a_eff.max(a_gross), alloy.f_o_pa);
            let ch = part_1_1::chi_from_lambda(lb, alpha, lambda_0);
            if ch >= chi_req { lo = mid; } else { hi = mid; }
        }
        remedies_b.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
            q_length(member.buckling_length_y),
            q_length(lo),
            loc(
                &format!("Reduce buckling length L_cr,y from {:.2} m to at most {:.2} m (χ computed = {chi:.3}).", member.buckling_length_y, lo),
                &format!("Biegedrillknicklänge L_cr,y von {:.2} m auf höchstens {:.2} m verkürzen (χ = {chi:.3}).", member.buckling_length_y, lo),
            ),
        ));
        remedies_b.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into()],
            loc("Select a stockier section to raise χ·A.", "Stockigeren Querschnitt wählen, um χ·A zu erhöhen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.fb.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
        loc("Flexural buckling N_b,Rd (χ computed)", "Biegeknicken N_b,Rd (χ berechnet)"),
        q_force(n_ed.abs()),
        q_force(n_b_rd_y),
        annex,
        loc(
            &format!("λ̄_y={lb_y:.6}, χ_y={chi_y:.6}, N_b,Rd,y={:.1} kN (ULS {combo}, lead {action_id}).", n_b_rd_y/1e3),
            &format!("λ̄_y={lb_y:.6}, χ_y={chi_y:.6}, N_b,Rd,y={:.1} kN (GZT {combo}, führend {action_id}).", n_b_rd_y/1e3),
        ),
        remedies_b,
    ));
    let mut remedies_fb_z = Vec::new();
    if n_ed.abs() > n_b_rd_z && n_b_rd_z > 0.0 {
        let chi_req_z = (n_ed.abs() * params.gamma_m1 / (a_eff * alloy.f_o_pa)).min(1.0);
        let mut lo_z = 0.1;
        let mut hi_z = member.buckling_length_z.max(0.5);
        for _ in 0..40 {
            let mid = 0.5 * (lo_z + hi_z);
            let lb = part_1_1::lambda_bar(mid, i_z, a_eff.max(a_gross), alloy.f_o_pa);
            let ch = part_1_1::chi_from_lambda(lb, alpha, lambda_0);
            if ch >= chi_req_z { lo_z = mid; } else { hi_z = mid; }
        }
        remedies_fb_z.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthZ", member.id), member_label(&member.id)),
            q_length(member.buckling_length_z),
            q_length(lo_z.max(0.05)),
            loc(
                &format!("Reduce buckling length L_cr,z to at most {:.2} m.", lo_z.max(0.05)),
                &format!("Knicklänge L_cr,z auf höchstens {:.2} m verkürzen.", lo_z.max(0.05)),
            ),
        ));
        remedies_fb_z.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "nK", |a| a.n_k, n_ed, n_b_rd_z, q_force,
            member_label(&member.id),
            "Reduce |N_k| so flexural buckling about z complies.",
            "|N_k| senken, damit Biegeknicken um z erfüllt ist.",
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.fb.z.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthZ", member.id), member_label(&member.id)),
        loc("Flexural buckling N_b,Rd,z", "Biegeknicken N_b,Rd,z"),
        q_force(n_ed.abs()),
        q_force(n_b_rd_z),
        annex,
        loc(
            &format!("λ̄_z={lb_z:.6}, χ_z={chi_z:.6}, N_b,Rd,z={:.1} kN (ULS {combo}, lead {action_id}).", n_b_rd_z/1e3),
            &format!("λ̄_z={lb_z:.6}, χ_z={chi_z:.6}, N_b,Rd,z={:.1} kN (GZT {combo}, führend {action_id}).", n_b_rd_z/1e3),
        ),
        remedies_fb_z,
    ));
    // Slenderness always tracks L_cr even when χ = 1 (plateau).
    let lb_t_slend = part_1_1::lambda_bar(l_cr_t, i_y.min(i_z), a_eff.max(a_gross), alloy.f_o_pa);
    let mut remedies_lam_y = Vec::new();
    if lb_y > 3.0 {
        let l_req = (member.buckling_length_y * 3.0 / lb_y * 0.99).max(0.05);
        remedies_lam_y.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
            q_length(member.buckling_length_y),
            q_length(l_req),
            loc(
                &format!("Reduce L_cr,y to at most {:.2} m so λ̄_y ≤ 3.", l_req),
                &format!("L_cr,y auf höchstens {:.2} m verkürzen, damit λ̄_y ≤ 3.", l_req),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.lambda.y.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
        loc("Non-dimensional slenderness λ̄_y", "Bezogene Schlankheit λ̄_y"),
        q_dim(lb_y),
        q_dim(3.0),
        annex,
        loc(
            &format!("λ̄_y={lb_y:.6} from L_cr,y (ULS {combo}, lead {action_id})."),
            &format!("λ̄_y={lb_y:.6} aus L_cr,y (GZT {combo}, führend {action_id})."),
        ),
        remedies_lam_y,
    ));
    let mut remedies_lam_t = Vec::new();
    if lb_t_slend > 3.0 {
        let l_req = (member.buckling_length_t * 3.0 / lb_t_slend * 0.99).max(0.05);
        remedies_lam_t.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthT", member.id), member_label(&member.id)),
            q_length(member.buckling_length_t),
            q_length(l_req),
            loc(
                &format!("Reduce L_cr,T to at most {:.2} m so λ̄_T ≤ 3.", l_req),
                &format!("L_cr,T auf höchstens {:.2} m verkürzen, damit λ̄_T ≤ 3.", l_req),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.lambda.t.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1.4"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthT", member.id), member_label(&member.id)),
        loc("Non-dimensional slenderness λ̄_T", "Bezogene Schlankheit λ̄_T"),
        q_dim(lb_t_slend),
        q_dim(3.0),
        annex,
        loc(
            &format!("λ̄_T from L_cr,T={l_cr_t:.4} m (ULS {combo}, lead {action_id})."),
            &format!("λ̄_T aus L_cr,T={l_cr_t:.4} m (GZT {combo}, führend {action_id})."),
        ),
        remedies_lam_t,
    ));
    out.push(utilization_check(
        format!("en1999.6.3.1.lcr.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
        loc("Buckling lengths L_cr,y/z/t", "Knicklängen L_cr,y/z/t"),
        q_length(l_cr_y + l_cr_z + l_cr_t),
        q_length(((l_cr_y + l_cr_z + l_cr_t) * 2.0).max(1e-3)),
        annex,
        loc(
            &format!("ΣL_cr={:.3} m (y={:.3}, z={:.3}, t={:.3}); L={:.3} m; support k={k_sup:.2} (ULS {combo}, lead {action_id}).", l_cr_y + l_cr_z + l_cr_t, l_cr_y, l_cr_z, l_cr_t, member.length),
            &format!("ΣL_cr={:.3} m (y={:.3}, z={:.3}, t={:.3}); L={:.3} m; Auflager k={k_sup:.2} (GZT {combo}, führend {action_id}).", l_cr_y + l_cr_z + l_cr_t, l_cr_y, l_cr_z, l_cr_t, member.length),
        ),
        Vec::new(),
    ));

    // Torsional / flexural-torsional
    let lb_t = part_1_1::torsional_lambda_bar(a_eff.max(a_gross), i_t, i_y, i_z, l_cr_t, alloy.f_o_pa);
    let chi_t = part_1_1::chi_from_lambda(lb_t, alpha, lambda_0);
    let n_t_rd = chi_t * a_eff * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_t = Vec::new();
    if n_ed.abs() > n_t_rd && n_t_rd > 0.0 {
        remedies_t.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-box120".into(), "sec-tube114".into()],
            loc("Select a closed section to raise I_t.", "Geschlossenen Querschnitt wählen, um I_t zu erhöhen."),
        ));
        remedies_t.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthT", member.id), member_label(&member.id)),
            q_length(l_cr_t),
            q_length(l_cr_t * 0.5),
            loc("Halve torsional buckling length L_cr,T.", "Torsionsknicklänge L_cr,T halbieren."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.tb.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1.4"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthT", member.id), member_label(&member.id)),
        loc("Torsional / flexural-torsional buckling", "Torsions- / Biegedrillknicken"),
        q_force(n_ed.abs()),
        q_force(n_t_rd),
        annex,
        loc(
            &format!("L_cr,T={l_cr_t:.4} m, λ̄_T={lb_t:.6}, χ_T={chi_t:.6}, N_t,Rd={:.1} kN (ULS {combo}, lead {action_id}); {sup_dig}", n_t_rd/1e3),
            &format!("L_cr,T={l_cr_t:.4} m, λ̄_T={lb_t:.6}, χ_T={chi_t:.6}, N_t,Rd={:.1} kN (GZT {combo}, führend {action_id}); {sup_dig}", n_t_rd/1e3),
        ),
        remedies_t,
    ));

    // LTB §6.3.2
    let m_cr = if member.restrained_ltb {
        f64::INFINITY
    } else {
        part_1_1::m_cr_ltb(member.c1, na_de::E_PA, i_z, i_w, i_t, l_lt)
    };
    let lambda_lt = if m_cr.is_finite() && m_cr > 0.0 { (wel_eff * alloy.f_o_pa / m_cr).sqrt() } else { 0.0 };
    let chi_lt = part_1_1::chi_from_lambda(lambda_lt, alpha, lambda_0);
    let m_b_rd = chi_lt * m_rd;
    let mut remedies_lt = Vec::new();
    if m_y.abs() > m_b_rd && m_b_rd > 0.0 {
        remedies_lt.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].ltbLength", member.id), member_label(&member.id)),
            q_length(l_lt),
            q_length(l_lt * 0.5),
            loc(
                &format!("Reduce LTB length from {:.2} m (M_cr={:.1} kNm, χ_LT={chi_lt:.3}).", l_lt, m_cr/1e3),
                &format!("Biegedrillknicklänge von {:.2} m verkürzen (M_cr={:.1} kNm, χ_LT={chi_lt:.3}).", l_lt, m_cr/1e3),
            ),
        ));
        remedies_lt.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into()],
            loc("Select a section with larger I_z / I_w.", "Querschnitt mit größerem I_z / I_w wählen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.2.ltb.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.2", "6.3.2"),
        SubjectRef::new(&member.id, format!("members[id={}].ltbLength", member.id), member_label(&member.id)),
        loc("Lateral-torsional buckling M_b,Rd", "Biegedrillknicken M_b,Rd"),
        q_moment(m_y.abs()),
        q_moment(m_b_rd),
        annex,
        loc(
            &format!("M_cr={:.1} kNm, λ̄_LT={lambda_lt:.6}, χ_LT={chi_lt:.6}, M_b,Rd={:.2} kNm (ULS {combo}, lead {action_id}); {sup_dig}; {act_dig}", m_cr/1e3, m_b_rd/1e3),
            &format!("M_cr={:.1} kNm, λ̄_LT={lambda_lt:.6}, χ_LT={chi_lt:.6}, M_b,Rd={:.2} kNm (GZT {combo}, führend {action_id}); {sup_dig}; {act_dig}", m_cr/1e3, m_b_rd/1e3),
        ),
        remedies_lt,
    ));

    // Interaction 6.3.3
    let u_i = if n_b_rd > 0.0 && m_b_rd > 0.0 {
        n_ed.abs() / n_b_rd + m_y.abs() / m_b_rd
    } else { 0.0 };
    let mut remedies_i = Vec::new();
    if u_i > 1.0 {
        remedies_i.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into()],
            loc("Select a larger section for member buckling interaction.", "Größeren Querschnitt für Knickinteraktion wählen."),
        ));
        remedies_i.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
            q_length(member.buckling_length_y),
            q_length(member.buckling_length_y * 0.6),
            loc("Shorten buckling / LTB lengths (add restraints).", "Knick-/Biegedrillknicklängen verkürzen (Halterungen)."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.3.int.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.3", "6.3.3"),
        SubjectRef::new(&member.id, path_n, member_label(&member.id)),
        loc("Member buckling interaction", "Interaktion Stabknicken"),
        q_dim(u_i),
        q_dim(1.0),
        annex,
        loc(
            &format!("N_Ed/N_b,Rd + M_Ed/M_b,Rd = {u_i:.3} (ULS {combo}, lead {action_id})."),
            &format!("N_Ed/N_b,Rd + M_Ed/M_b,Rd = {u_i:.3} (GZT {combo}, führend {action_id})."),
        ),
        remedies_i,
    ));

    // SLS deflection §7.2 (quasi-permanent) + elastic stress (characteristic)
    let sls_qp = part_en1990::sls_member_effects(member, annex, "quasiPermanent");
    let sls_char = part_en1990::sls_member_effects(member, annex, "characteristic");
    let (_, iy, _, _, _, _, _) = part_1_1::section_geometry(section);
    let e_mod = na_de::E_PA;
    let w_qp = if e_mod * iy > 0.0 && member.length > 0.0 {
        // simply-supported UDL proxy from |M| ≈ q L²/8 → q = 8M/L²; w = 5 q L⁴ / (384 EI)
        let m_qp = sls_qp.m_y_ed.abs();
        let q = 8.0 * m_qp / member.length.powi(2);
        5.0 * q * member.length.powi(4) / (384.0 * e_mod * iy)
    } else { 0.0 };
    let w_lim = member.length / 200.0; // EN 1999-1-1 §7.2 / DE NA vertical deflection
    let mut rem_w = Vec::new();
    if w_qp > w_lim {
        if member.length <= 0.0 {
            rem_w.push(Remedy::at_least(
                SubjectRef::new(&member.id, format!("members[id={}].length", member.id), member_label(&member.id)),
                q_length(member.length),
                q_length(1.0),
                loc("Set a positive member length for SLS deflection.", "Positive Bauteillänge für GZG-Durchbiegung setzen."),
            ));
        } else if w_lim > 0.0 {
            rem_w.extend(scale_action_leaf_remedies(
                "members", &member.id, &member.actions, "mYK", |a| a.m_y_k, sls_qp.m_y_ed, sls_qp.m_y_ed.abs() * w_lim / w_qp.max(1e-12), q_moment,
                member_label(&member.id),
                "Reduce M_y,k so quasi-permanent deflection ≤ L/200.",
                "M_y,k senken, damit quasi-ständige Durchbiegung ≤ L/200.",
            ));
            rem_w.extend(scale_action_leaf_remedies(
                "members", &member.id, &member.actions, "gKLine", |a| a.g_k_line, w_qp, w_lim, q_force,
                member_label(&member.id),
                "Reduce g_k so quasi-permanent deflection ≤ L/200.",
                "g_k senken, damit quasi-ständige Durchbiegung ≤ L/200.",
            ));
            rem_w.extend(scale_action_leaf_remedies(
                "members", &member.id, &member.actions, "qKLine", |a| a.q_k_line, w_qp, w_lim, q_force,
                member_label(&member.id),
                "Reduce q_k so quasi-permanent deflection ≤ L/200.",
                "q_k senken, damit quasi-ständige Durchbiegung ≤ L/200.",
            ));
        }
    }
    out.push(utilization_check(
        format!("en1999.7.2.defl.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§7.2", "7.2"),
        SubjectRef::new(&member.id, format!("members[id={}].length", member.id), member_label(&member.id)),
        loc("SLS deflection (quasi-permanent)", "GZG Durchbiegung (quasi-ständig)"),
        q_length(w_qp),
        q_length(w_lim.max(1e-9)),
        annex,
        loc(
            &format!("w={:.1} mm ≤ L/200={:.1} mm (SLS {comb}, lead {aid}); {act_dig}; {sup_dig}", w_qp*1e3, w_lim*1e3, comb=sls_qp.combination, aid=sls_qp.action_id),
            &format!("w={:.1} mm ≤ L/200={:.1} mm (GZG {comb}, führend {aid}); {act_dig}; {sup_dig}", w_qp*1e3, w_lim*1e3, comb=sls_qp.combination, aid=sls_qp.action_id),
        ),
        rem_w,
    ));
    let sigma_sls = if wel_eff > 0.0 { sls_char.m_y_ed.abs() / wel_eff } else { 0.0 };
    let sigma_lim = 0.8 * alloy.f_o_pa; // elastic SLS stress limit
    let mut rem_s = Vec::new();
    if sigma_sls > sigma_lim {
        let m_lim = if wel_eff > 0.0 { sigma_lim * wel_eff } else { 0.0 };
        rem_s.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "mYK", |a| a.m_y_k, sls_char.m_y_ed, m_lim, q_moment,
            member_label(&member.id),
            "Reduce M_y,k so characteristic SLS σ ≤ 0.8 f_o.",
            "M_y,k senken, damit charakteristische GZG-Spannung σ ≤ 0,8 f_o.",
        ));
        rem_s.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "gKLine", |a| a.g_k_line, sigma_sls, sigma_lim, q_force,
            member_label(&member.id),
            "Reduce g_k so characteristic SLS σ ≤ 0.8 f_o.",
            "g_k senken, damit charakteristische GZG-Spannung σ ≤ 0,8 f_o.",
        ));
        rem_s.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "qKLine", |a| a.q_k_line, sigma_sls, sigma_lim, q_force,
            member_label(&member.id),
            "Reduce q_k so characteristic SLS σ ≤ 0.8 f_o.",
            "q_k senken, damit charakteristische GZG-Spannung σ ≤ 0,8 f_o.",
        ));
    }
    out.push(utilization_check(
        format!("en1999.7.2.stress.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§7.2", "7.2"),
        SubjectRef::new(&member.id, format!("members[id={}].actions[id={}].mYK", member.id, sls_char.action_id), member_label(&member.id)),
        loc("SLS elastic stress (characteristic)", "GZG elastische Spannung (charakteristisch)"),
        q_stress(sigma_sls),
        q_stress(sigma_lim.max(1e-9)),
        annex,
        loc(
            &format!("σ={:.0} MPa ≤ 0.8 f_o={:.0} MPa (SLS {comb}, lead {aid}); {act_dig}; {sup_dig}; {el_dig}", sigma_sls/1e6, sigma_lim/1e6, comb=sls_char.combination, aid=sls_char.action_id),
            &format!("σ={:.0} MPa ≤ 0,8 f_o={:.0} MPa (GZG {comb}, führend {aid}); {act_dig}; {sup_dig}; {el_dig}", sigma_sls/1e6, sigma_lim/1e6, comb=sls_char.combination, aid=sls_char.action_id),
        ),
        rem_s,
    ));

    let sls_freq = part_en1990::sls_member_effects(member, annex, "frequent");
    let sigma_freq = if wel_eff > 0.0 { sls_freq.m_y_ed.abs() / wel_eff } else { 0.0 };
    let mut rem_f = Vec::new();
    if sigma_freq > sigma_lim {
        let m_lim = if wel_eff > 0.0 { sigma_lim * wel_eff } else { 0.0 };
        rem_f.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "mYK", |a| a.m_y_k, sls_freq.m_y_ed, m_lim, q_moment,
            member_label(&member.id),
            "Reduce M_y,k so frequent SLS σ ≤ 0.8 f_o.",
            "M_y,k senken, damit häufige GZG-Spannung σ ≤ 0,8 f_o.",
        ));
        rem_f.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "gKLine", |a| a.g_k_line, sigma_freq, sigma_lim, q_force,
            member_label(&member.id),
            "Reduce g_k so frequent SLS σ ≤ 0.8 f_o.",
            "g_k senken, damit häufige GZG-Spannung σ ≤ 0,8 f_o.",
        ));
        rem_f.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "qKLine", |a| a.q_k_line, sigma_freq, sigma_lim, q_force,
            member_label(&member.id),
            "Reduce q_k so frequent SLS σ ≤ 0.8 f_o.",
            "q_k senken, damit häufige GZG-Spannung σ ≤ 0,8 f_o.",
        ));
    }
    out.push(utilization_check(
        format!("en1999.7.2.sls-freq.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§7.2", "7.2"),
        SubjectRef::new(&member.id, format!("members[id={}].actions[id={}].mYK", member.id, sls_freq.action_id), member_label(&member.id)),
        loc("SLS elastic stress (frequent ψ₁)", "GZG elastische Spannung (häufig ψ₁)"),
        q_stress(sigma_freq),
        q_stress(sigma_lim.max(1e-9)),
        annex,
        loc(
            &format!("σ={:.0} MPa ≤ 0.8 f_o={:.0} MPa (SLS {comb}, lead {aid}); {act_dig}; {sup_dig}", sigma_freq/1e6, sigma_lim/1e6, comb=sls_freq.combination, aid=sls_freq.action_id),
            &format!("σ={:.0} MPa ≤ 0,8 f_o={:.0} MPa (GZG {comb}, führend {aid}); {act_dig}; {sup_dig}", sigma_freq/1e6, sigma_lim/1e6, comb=sls_freq.combination, aid=sls_freq.action_id),
        ),
        rem_f,
    ));

    out
}
//#endregion 🔖️ComplianceHelpers
