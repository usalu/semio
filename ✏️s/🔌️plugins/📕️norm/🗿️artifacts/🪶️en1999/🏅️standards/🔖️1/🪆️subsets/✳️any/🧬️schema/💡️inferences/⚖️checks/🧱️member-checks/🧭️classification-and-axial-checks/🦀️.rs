//! 🧭️ Classification and axial checks: the section classification and the tension, compression and combined axial checks of the member checks. See [`super`].

use super::*;

/// 🧭 Classifies every plate element of the section, one check per element.
pub(super) fn check_classification(
    member: &AluminiumMember,
    section: &AluminiumSection,
    material: &AluminiumMaterial,
    annex: AnnexChoice,
    alloy: &part_1_1::AlloyProps,
    class: part_1_1::SectionClass,
    el_dig: &str,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    combo: &'static str,
) {
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
}

/// ⬍ Checks the tension and compression resistance of the cross section.
pub(super) fn check_tension_and_compression(
    member: &AluminiumMember,
    material: &AluminiumMaterial,
    annex: AnnexChoice,
    params: &na_de::AnnexParams,
    alloy: &part_1_1::AlloyProps,
    a_eff: f64,
    el_dig: &str,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    n_ed: f64,
    combo: &'static str,
    act_dig: &str,
    sup_dig: &str,
    path_n: &str,
) -> f64 {
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
            SubjectRef::new(&member.id, path_n.to_owned(), member_label(&member.id)),
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
        SubjectRef::new(&member.id, path_n.to_owned(), member_label(&member.id)),
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
    n_rd
}

/// 🔀 Checks the simplified combined axial and bending utilization.
pub(super) fn check_combined_axial_bending(
    member: &AluminiumMember,
    annex: AnnexChoice,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    m_y: f64,
    m_z: f64,
    n_ed: f64,
    combo: &'static str,
    path_n: &str,
    n_rd: f64,
    m_rd: f64,
    m_rd_z: f64,
) {
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
        SubjectRef::new(&member.id, path_n.to_owned(), member_label(&member.id)),
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
}
