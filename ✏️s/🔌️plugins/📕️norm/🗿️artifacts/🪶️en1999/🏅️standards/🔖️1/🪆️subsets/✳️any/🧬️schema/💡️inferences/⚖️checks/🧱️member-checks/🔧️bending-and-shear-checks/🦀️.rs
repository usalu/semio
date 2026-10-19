//! 🔧️ Bending and shear checks: the bending about both axes and the shear about both axes of the member checks. See [`super`].

use super::*;

/// 🔧 Checks the bending resistance about the strong axis.
pub(super) fn check_bending_y(
    member: &AluminiumMember,
    annex: AnnexChoice,
    params: &na_de::AnnexParams,
    alloy: &part_1_1::AlloyProps,
    wel_eff: f64,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    m_y: f64,
    combo: &'static str,
    act_dig: &str,
    sup_dig: &str,
    path_m: &str,
) -> f64 {
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
                SubjectRef::new(&member.id, path_m.to_owned(), member_label(&member.id)),
                q_moment(m_y),
                q_moment(m_rd.copysign(m_y)),
                loc(
                    &format!("Reduce |M_y,Ed| from {:.2} kNm to at most {:.2} kNm.", m_y.abs()/1e3, m_rd/1e3),
                    &format!("|M_y,Ed| von {:.2} kNm auf höchstens {:.2} kNm reduzieren.", m_y.abs()/1e3, m_rd/1e3),
                ),
            ));
        } else {
            remedies_m.push(Remedy::at_most(
                SubjectRef::new(&member.id, path_m.to_owned(), member_label(&member.id)),
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
        SubjectRef::new(&member.id, path_m.to_owned(), member_label(&member.id)),
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
    m_rd
}

/// ✂ Checks the shear resistance with the web buckling reduction.
pub(super) fn check_shear_z(
    member: &AluminiumMember,
    section: &AluminiumSection,
    annex: AnnexChoice,
    params: &na_de::AnnexParams,
    alloy: &part_1_1::AlloyProps,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    v_z: f64,
    combo: &'static str,
) -> f64 {
    let a_v = part_1_1::shear_area(section);
    let v_rd = a_v * alloy.f_o_pa / (params.gamma_m1 * 3.0f64.sqrt());
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
    v_rd_b
}

/// 🪛 Checks the bending resistance about the weak axis.
pub(super) fn check_bending_z(
    member: &AluminiumMember,
    section: &AluminiumSection,
    annex: AnnexChoice,
    params: &na_de::AnnexParams,
    alloy: &part_1_1::AlloyProps,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    m_z: f64,
    combo: &'static str,
    path_mz: &str,
) -> f64 {
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
        SubjectRef::new(&member.id, path_mz.to_owned(), member_label(&member.id)),
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
    m_rd_z
}

/// 🪚 Checks the shear resistance of the weak axis.
pub(super) fn check_shear_y(
    member: &AluminiumMember,
    annex: AnnexChoice,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    v_y: f64,
    combo: &'static str,
    path_vy: &str,
    v_rd_b: f64,
) {
    let mut remedies_vy = Vec::new();
    if v_y.abs() > v_rd_b && v_rd_b > 0.0 {
        remedies_vy.push(Remedy::at_most(
            SubjectRef::new(&member.id, path_vy.to_owned(), member_label(&member.id)),
            q_force(v_y),
            q_force(v_rd_b.copysign(v_y)),
            loc("Reduce |V_y,Ed|.", "|V_y,Ed| reduzieren."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.2.6.vy.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.2.6", "6.2.6"),
        SubjectRef::new(&member.id, path_vy.to_owned(), member_label(&member.id)),
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
}
