//! 🔥️ EN 1999 fire and fatigue checks: `check_fire` (EN 1999-1-2) and `check_fatigue` (EN 1999-1-3).

use super::*;

//#region 🔖️ComplianceHelpers
pub fn check_fire(
    fire: &FireScenario,
    member: &AluminiumMember,
    section: &AluminiumSection,
    material: &AluminiumMaterial,
    annex: AnnexChoice,
) -> CheckResult {
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return CheckResult::assess(
            format!("en1999.1-2.fire.{}", fire.id),
            "DIN EN 1999-1-2",
            ClauseId::new("EN 1999-1-2", "§4", "4.2"),
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            loc("Fire resistance", "Brandschutz"),
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
        .build();
    };
    let params = na_de::AnnexParams::for_choice(annex);
    let gov = part_en1990::fire_member_effects(member, annex);
    let n_ed = gov.n_ed.abs();
    let m_ed = gov.m_y_ed.abs();
    let theta_from_duration = 20.0 + 345.0 * ((1.0 + fire.duration_s.max(0.0) / 60.0).log10());
    let theta_eff = fire.theta_a + (theta_from_duration - 20.0).max(0.0) * 0.25;
    let k = part_1_2::k_theta(theta_eff);
    let theta_cr = part_1_2::critical_temperature_c(alloy.f_o_pa);
    let (a_gross, _iy, _iz, wel_y, _wel_z, _it, _iw) = part_1_1::section_geometry(section);
    let n_rd = a_gross * alloy.f_o_pa / params.gamma_m1;
    let m_rd = wel_y * alloy.f_o_pa / params.gamma_m1;
    let n_fi = k * n_rd;
    let m_fi = k * m_rd;
    let u_n = if n_fi > 0.0 { n_ed / n_fi } else if n_ed > 0.0 { f64::INFINITY } else { 0.0 };
    let u_m = if m_fi > 0.0 { m_ed / m_fi } else if m_ed > 0.0 { f64::INFINITY } else { 0.0 };
    let u = u_n.max(u_m);
    let mut remedies = Vec::new();
    if u > 1.0 {
        let demand_ratio = if n_rd > 0.0 || m_rd > 0.0 {
            (if n_rd > 0.0 { n_ed / n_rd } else { 0.0 }).max(if m_rd > 0.0 { m_ed / m_rd } else { 0.0 })
        } else {
            f64::INFINITY
        };
        remedies.push(Remedy::at_most(
            SubjectRef::new(&fire.id, format!("fireScenarios[id={}].durationS", fire.id), loc("Fire duration", "Branddauer")),
            Quantity::new(QuantityKind::Time, fire.duration_s),
            Quantity::new(QuantityKind::Time, 0.0),
            loc("Clear fire exposure duration so θ_eff = θ_a.", "Branddauer auf null setzen, damit θ_eff = θ_a."),
        ));
        let k_req = demand_ratio.min(1.0);
        let theta_req = if k_req >= 1.0 { 100.0 } else { (550.0 - 450.0 * k_req).clamp(20.0, 550.0) };
        remedies.push(Remedy::at_most(
            SubjectRef::new(&fire.id, format!("fireScenarios[id={}].thetaA", fire.id), loc(&format!("Fire {}", fire.id), &format!("Brand {}", fire.id))),
            q_temp(fire.theta_a),
            q_temp(theta_req),
            loc(
                &format!("Lower θ_a to ≤ {:.0} °C so k_θ≥{k_req:.3} restores fire resistance.", theta_req),
                &format!("θ_a auf ≤ {:.0} °C senken, damit k_θ≥{k_req:.3} den Brandwiderstand wiederherstellt.", theta_req),
            ),
        ));
        remedies.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "nK", |a| a.n_k, n_ed.max(1e-9), n_rd.max(1e-9), q_force,
            member_label(&member.id),
            "Reduce N_k so fire utilization ≤ 1 at ambient capacity.",
            "N_k senken, damit Brandausnutzung bei Umgebungstragfähigkeit ≤ 1.",
        ));
        remedies.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "mYK", |a| a.m_y_k, m_ed.max(1e-9), m_rd.max(1e-9), q_moment,
            member_label(&member.id),
            "Reduce M_y,k so fire utilization ≤ 1 at ambient capacity.",
            "M_y,k senken, damit Brandausnutzung bei Umgebungstragfähigkeit ≤ 1.",
        ));
        remedies.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "gKLine", |a| a.g_k_line, u, 1.0, q_force,
            member_label(&member.id),
            "Reduce g_k so fire utilization ≤ 1.",
            "g_k senken, damit Brandausnutzung ≤ 1.",
        ));
        remedies.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "qKLine", |a| a.q_k_line, u, 1.0, q_force,
            member_label(&member.id),
            "Reduce q_k so fire utilization ≤ 1.",
            "q_k senken, damit Brandausnutzung ≤ 1.",
        ));
    }
    utilization_check(
        format!("en1999.1-2.fire.{}", fire.id),
        "DIN EN 1999-1-2",
        ClauseId::new("EN 1999-1-2", "§4", "4.2"),
        SubjectRef::new(&fire.id, format!("fireScenarios[id={}].thetaA", fire.id), loc(&format!("Fire {}", fire.id), &format!("Brand {}", fire.id))),
        loc("Fire resistance with k_θ(θ_a)", "Brandschutz mit k_θ(θ_a)"),
        q_dim(u),
        q_dim(1.0),
        annex,
        loc(
            &format!(
                "θ_eff={:.0} °C (duration {:.0} s), θ_cr={:.0} °C, k_θ={k:.3}, N_Ed={:.1} kN / N_fi,Rd={:.1} kN, M_y,Ed={:.2} kNm / M_fi,Rd={:.2} kNm, u={u:.3}.",
                theta_eff, fire.duration_s, theta_cr, n_ed/1e3, n_fi/1e3, m_ed/1e3, m_fi/1e3
            ),
            &format!(
                "θ_eff={:.0} °C (Branddauer {:.0} s), θ_cr={:.0} °C, k_θ={k:.3}, N_Ed={:.1} kN / N_fi,Rd={:.1} kN, M_y,Ed={:.2} kNm / M_fi,Rd={:.2} kNm, u={u:.3}.",
                theta_eff, fire.duration_s, theta_cr, n_ed/1e3, n_fi/1e3, m_ed/1e3, m_fi/1e3
            ),
        ),
        remedies,
    )
}

pub fn check_fatigue(fat: &FatigueDetail, member: Option<&AluminiumMember>, annex: AnnexChoice) -> CheckResult {
    let gmf = part_1_3::gamma_mf(annex);
    let dc = part_1_3::detail_category_delta_sigma_c(&fat.detail_category).unwrap_or(fat.delta_sigma_c);
    let member_scale = match member {
        None if !fat.member_id.is_empty() => 0.0,
        Some(m) => {
            let gov = part_en1990::governing_member_effects(m, annex);
            1.0 + gov.m_y_ed.abs() / 1.0e6
        }
        _ => 1.0,
    };
    let delta_ed = if member_scale == 0.0 { fat.delta_sigma_ed } else { fat.delta_sigma_ed * member_scale.max(1e-6) };
    let rd_n = part_1_3::fatigue_strength_pa(dc, fat.m1, fat.m2, fat.n_cycles);
    let rd_m2 = part_1_3::fatigue_strength_pa(dc, fat.m1, fat.m2, fat.n_cycles.max(part_1_3::N_D * 1.01));
    let rd = (0.9 * rd_n + 0.1 * rd_m2) / gmf;
    let damage = part_1_3::damage_ratio(delta_ed, dc, fat.m1, fat.m2, fat.n_cycles, gmf);
    let mut remedies = Vec::new();
    if member.is_none() && !fat.member_id.is_empty() {
        return CheckResult::assess(
            format!("en1999.1-3.fat.{}", fat.id),
            "DIN EN 1999-1-3",
            ClauseId::new("EN 1999-1-3", "§7.1", "7.1"),
            SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].memberId", fat.id), loc("Fatigue", "Ermüdung")),
            loc("Fatigue member reference", "Ermüdungs-Bauteilverweis"),
        )
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .explanation(loc(
            &format!("Fatigue detail '{}' references missing member '{}'.", fat.id, fat.member_id),
            &format!("Ermüdungsdetail '{}' verweist auf fehlendes Bauteil '{}'.", fat.id, fat.member_id),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].memberId", fat.id), loc("Fatigue", "Ermüdung")),
            vec![fat.member_id.clone()],
            loc("Point fatigue.memberId at an existing member.", "fatigue.memberId auf vorhandenes Bauteil setzen."),
        ))
        .build();
    }
    if delta_ed > rd && rd > 0.0 {
        remedies.push(Remedy::at_most(
            SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].deltaSigmaEd", fat.id), loc(&format!("Fatigue {}", fat.id), &format!("Ermüdung {}", fat.id))),
            q_stress(delta_ed),
            q_stress(rd),
            loc(
                &format!("Reduce Δσ_Ed from {:.0} MPa to ≤ {:.0} MPa (detail {}).", delta_ed/1e6, rd/1e6, fat.detail_category),
                &format!("Δσ_Ed von {:.0} MPa auf ≤ {:.0} MPa reduzieren (Detail {}).", delta_ed/1e6, rd/1e6, fat.detail_category),
            ),
        ));
        remedies.push(Remedy::one_of(
            SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].detailCategory", fat.id), loc("Detail category", "Kerbfall")),
            vec!["71".into(), "80".into(), "90".into()],
            loc("Select a higher Annex J detail category.", "Höheren Annex-J-Kerbfall wählen."),
        ));
    }
    utilization_check(
        format!("en1999.1-3.fat.{}", fat.id),
        "DIN EN 1999-1-3",
        ClauseId::new("EN 1999-1-3", "§7.1", "7.1"),
        SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].deltaSigmaEd", fat.id), loc(&format!("Fatigue {}", fat.id), &format!("Ermüdung {}", fat.id))),
        loc("Fatigue damage (bi-linear S–N)", "Ermüdungsschädigung (bi-lineare S–N)"),
        q_stress(delta_ed),
        q_stress(rd.max(1e-9)),
        annex,
        loc(
            &format!(
                "Detail {}, Δσ_C={:.0} MPa, m1={:.1}/m2={:.1}, n={:.3e}, γ_Mf={gmf:.2}, D={damage:.3}, member {}.",
                fat.detail_category, dc/1e6, fat.m1, fat.m2, fat.n_cycles, fat.member_id
            ),
            &format!(
                "Detail {}, Δσ_C={:.0} MPa, m1={:.1}/m2={:.1}, n={:.3e}, γ_Mf={gmf:.2}, D={damage:.3}, Bauteil {}.",
                fat.detail_category, dc/1e6, fat.m1, fat.m2, fat.n_cycles, fat.member_id
            ),
        ),
        remedies,
    )
}
//#endregion 🔖️ComplianceHelpers
