//! 🥫️ EN 1999 thin-walled checks: `check_cold_formed` for cold-formed sheets and `check_shell` for shells.

use super::*;

//#region 🔖️ComplianceHelpers
pub(super) fn check_cold_formed(sheet: &ColdFormedSheet, material: &AluminiumMaterial, annex: AnnexChoice) -> Vec<CheckResult> {
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return vec![CheckResult::assess(
            format!("en1999.1-4.alloy.{}", sheet.id),
            "DIN EN 1999-1-4",
            ClauseId::new("EN 1999-1-4", "§5", "5.4"),
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
    let params = na_de::AnnexParams::for_choice(annex);
    let gov = part_en1990::governing_cold_formed_effects(sheet, annex);
    let (combo, action_id) = (gov.combination, gov.action_id.clone());
    let m_ed = gov.m_y_ed + gov.m_z_ed;
    let n_ed = gov.n_ed;
    let v_ed = gov.v_y_ed.hypot(gov.v_z_ed);
    let eps = part_1_1::epsilon(alloy.f_o_pa);
    let beta = if sheet.thickness > 0.0 { sheet.width / sheet.thickness } else { f64::INFINITY };
    let beta_limit = 22.0 * eps;
    let rho = if beta <= beta_limit { 1.0 } else { (beta_limit / beta).min(1.0) };
    let b_eff = rho * sheet.width;
    let wel = b_eff * sheet.thickness.powi(2) / 4.0;
    let m_rd = wel * alloy.f_o_pa / params.gamma_m1;
    let mut out = Vec::new();
    let mut remedies_b = Vec::new();
    if beta > beta_limit {
        let t_req = sheet.width / beta_limit;
        remedies_b.push(Remedy::at_least(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].thickness", sheet.id), loc("Sheet thickness", "Blechdicke")),
            q_length(sheet.thickness),
            q_length(t_req),
            loc(
                &format!("Increase thickness so β/ε ≤ {:.1} (t ≥ {:.2} mm).", beta_limit, t_req * 1e3),
                &format!("Dicke erhöhen, damit β/ε ≤ {:.1} (t ≥ {:.2} mm).", beta_limit, t_req * 1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-4.local.{}", sheet.id),
        "DIN EN 1999-1-4",
        ClauseId::new("EN 1999-1-4", "§5.4", "5.4"),
        SubjectRef::new(&sheet.id, format!("coldFormed[id={}].thickness", sheet.id), loc(&format!("Sheet {}", sheet.id), &format!("Blech {}", sheet.id))),
        loc("Local buckling of cold-formed sheeting", "Lokales Beulen kaltgeformter Bleche"),
        q_dim(beta / eps),
        q_dim(beta_limit),
        annex,
        loc(
            &format!("b={:.0} mm, t={:.2} mm, β/ε={:.1}, limit={:.1}, ρ={rho:.3}.", sheet.width*1e3, sheet.thickness*1e3, beta/eps, beta_limit),
            &format!("b={:.0} mm, t={:.2} mm, β/ε={:.1}, Grenze={:.1}, ρ={rho:.3}.", sheet.width*1e3, sheet.thickness*1e3, beta/eps, beta_limit),
        ),
        remedies_b,
    ));
    let mut remedies_m = Vec::new();
    if m_ed.abs() > m_rd && m_rd > 0.0 {
        remedies_m.extend(scale_action_leaf_remedies(
            "coldFormed", &sheet.id, &sheet.actions, "mYK", |a| a.m_y_k, m_ed, m_rd, q_moment,
            loc("Sheet moment", "Blechmoment"),
            &format!("Reduce M_y,k so M_Ed ≤ {:.2} kNm.", m_rd/1e3),
            &format!("M_y,k senken, sodass M_Ed ≤ {:.2} kNm.", m_rd/1e3),
        ));
        remedies_m.extend(scale_action_leaf_remedies(
            "coldFormed", &sheet.id, &sheet.actions, "gKLine", |a| a.g_k_line, m_ed, m_rd, q_force,
            loc("Sheet", "Blech"),
            "Reduce g_k so sheeting bending complies.",
            "g_k senken, damit Blechbiegung erfüllt ist.",
        ));
        remedies_m.extend(scale_action_leaf_remedies(
            "coldFormed", &sheet.id, &sheet.actions, "qKLine", |a| a.q_k_line, m_ed, m_rd, q_force,
            loc("Sheet", "Blech"),
            "Reduce q_k so sheeting bending complies.",
            "q_k senken, damit Blechbiegung erfüllt ist.",
        ));
        let t_req = if alloy.f_o_pa > 0.0 {
            // W_el ∝ t² → scale thickness by sqrt(u)
            sheet.thickness * (m_ed.abs() / m_rd).sqrt() * 1.05
        } else { sheet.thickness };
        remedies_m.push(Remedy::at_least(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].thickness", sheet.id), loc("Sheet thickness", "Blechdicke")),
            q_length(sheet.thickness),
            q_length(t_req),
            loc(
                &format!("Increase thickness to ≥ {:.2} mm for bending.", t_req * 1e3),
                &format!("Dicke auf ≥ {:.2} mm für Biegung erhöhen.", t_req * 1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-4.bend.{}", sheet.id),
        "DIN EN 1999-1-4",
        ClauseId::new("EN 1999-1-4", "§5.4", "5.4"),
        SubjectRef::new(&sheet.id, format!("coldFormed[id={}].actions[id={}].mYK", sheet.id, action_id), loc(&format!("Sheet {}", sheet.id), &format!("Blech {}", sheet.id))),
        loc("Sheeting bending resistance", "Biegewiderstand des Blechs"),
        q_moment(m_ed.abs()),
        q_moment(m_rd),
        annex,
        loc(
            &format!("M_Ed={:.2} kNm, M_c,Rd={:.2} kNm (W_eff={:.0} mm³, ULS {combo}, lead {action_id}).", m_ed.abs()/1e3, m_rd/1e3, wel*1e9),
            &format!("M_Ed={:.2} kNm, M_c,Rd={:.2} kNm (W_eff={:.0} mm³, GZT {combo}, führend {action_id}).", m_ed.abs()/1e3, m_rd/1e3, wel*1e9),
        ),
        remedies_m,
    ));

    // Axial (EN 1999-1-4 §6.1) — effective width + HAZ on welded sheets.
    let a_eff = b_eff * sheet.thickness * (if sheet.welded { alloy.rho_o_haz.min(alloy.rho_u_haz) } else { 1.0 });
    let n_rd = a_eff * alloy.f_o_pa / params.gamma_m1;
    let mut rem_n = Vec::new();
    if n_ed.abs() > n_rd && n_rd > 0.0 {
        rem_n.extend(scale_action_leaf_remedies(
            "coldFormed", &sheet.id, &sheet.actions, "nK", |a| a.n_k, n_ed, n_rd, q_force,
            loc("Sheet axial", "Blech-Normalkraft"),
            "Reduce N_k so cold-formed axial resistance governs.",
            "N_k senken, damit Normalkraftwiderstand kaltgeformt maßgebend ist.",
        ));
        let t_req = sheet.thickness * (n_ed.abs() / n_rd) * 1.05;
        rem_n.push(Remedy::at_least(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].thickness", sheet.id), loc("Sheet thickness", "Blechdicke")),
            q_length(sheet.thickness),
            q_length(t_req),
            loc(
                &format!("Increase thickness to ≥ {:.2} mm for axial resistance.", t_req * 1e3),
                &format!("Dicke auf ≥ {:.2} mm für Normalkraft erhöhen.", t_req * 1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-4.axial.{}", sheet.id),
        "DIN EN 1999-1-4",
        ClauseId::new("EN 1999-1-4", "§6.1", "6.1"),
        SubjectRef::new(&sheet.id, format!("coldFormed[id={}].actions[id={}].nK", sheet.id, action_id), loc(&format!("Sheet {}", sheet.id), &format!("Blech {}", sheet.id))),
        loc("Cold-formed axial resistance", "Normalkraftwiderstand kaltgeformt"),
        q_force(n_ed.abs()),
        q_force(n_rd.max(1e-9)),
        annex,
        loc(
            &format!("N_Ed={:.1} kN, welded={}, η/ρ applied, N_Rd={:.1} kN (ULS {combo}, lead {action_id}).", n_ed/1e3, sheet.welded, n_rd/1e3),
            &format!("N_Ed={:.1} kN, geschweißt={}, η/ρ angesetzt, N_Rd={:.1} kN (GZT {combo}, führend {action_id}).", n_ed/1e3, sheet.welded, n_rd/1e3),
        ),
        rem_n,
    ));

    // Web crippling / support reaction from span UDL proxy: R = 2 M_Ed / span
    let r_ed = if sheet.span > 0.0 { 2.0 * m_ed.abs() / sheet.span } else { 0.0 };
    let r_rd = 0.5 * sheet.thickness * sheet.thickness * alloy.f_o_pa / params.gamma_m1; // simplified
    let mut rem_r = Vec::new();
    if r_ed > r_rd && r_rd > 0.0 {
        rem_r.push(Remedy::at_least(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].span", sheet.id), loc("Sheet span", "Blechstützweite")),
            q_length(sheet.span),
            q_length((2.0 * m_ed.abs() / r_rd).max(sheet.span)),
            loc("Increase span support spacing reduction or thicken sheet.", "Stützweite/Auflager verbessern oder Blech dicken."),
        ));
        rem_r.push(Remedy::at_least(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].thickness", sheet.id), loc("Sheet thickness", "Blechdicke")),
            q_length(sheet.thickness),
            q_length((r_ed * params.gamma_m1 / (0.5 * alloy.f_o_pa)).sqrt()),
            loc("Increase thickness against web crippling.", "Dicke gegen Stegkrüppeln erhöhen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-4.support.{}", sheet.id),
        "DIN EN 1999-1-4",
        ClauseId::new("EN 1999-1-4", "§6.1.5", "6.1.5"),
        SubjectRef::new(&sheet.id, format!("coldFormed[id={}].span", sheet.id), loc(&format!("Sheet {}", sheet.id), &format!("Blech {}", sheet.id))),
        loc("Support reaction / web crippling", "Auflagerkraft / Stegkrüppeln"),
        q_force(r_ed),
        q_force(r_rd.max(1e-9)),
        annex,
        loc(
            &format!("Span L={:.2} m → R_Ed={:.2} kN, R_Rd={:.2} kN.", sheet.span, r_ed/1e3, r_rd/1e3),
            &format!("Stützweite L={:.2} m → R_Ed={:.2} kN, R_Rd={:.2} kN.", sheet.span, r_ed/1e3, r_rd/1e3),
        ),
        rem_r,
    ));

    // Interaction N–M
    let a_v = sheet.thickness * sheet.width;
    let v_rd = a_v * alloy.f_o_pa / (params.gamma_m1 * 3.0f64.sqrt());
    let mut remedies_v = Vec::new();
    if v_ed > v_rd && v_rd > 0.0 {
        remedies_v.push(Remedy::at_most(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].actions[id={}].vZK", sheet.id, action_id), loc("Sheet", "Blech")),
            q_force(v_ed),
            q_force(v_rd),
            loc("Reduce sheet shear from V_Ed.", "Blechquerkraft V_Ed reduzieren."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-4.shear.{}", sheet.id),
        "DIN EN 1999-1-4",
        ClauseId::new("EN 1999-1-4", "§6.1.7", "6.1.7"),
        SubjectRef::new(&sheet.id, format!("coldFormed[id={}].actions[id={}].vZK", sheet.id, action_id), loc("Sheet", "Blech")),
        loc("Cold-formed shear resistance", "Kaltprofil Querkraftwiderstand"),
        q_force(v_ed),
        q_force(v_rd),
        annex,
        loc(
            &format!("V_Ed={:.1} kN, V_Rd={:.1} kN (ULS {combo}, lead {action_id}).", v_ed/1e3, v_rd/1e3),
            &format!("V_Ed={:.1} kN, V_Rd={:.1} kN (GZT {combo}, führend {action_id}).", v_ed/1e3, v_rd/1e3),
        ),
        remedies_v,
    ));

    let u_i = if n_rd > 0.0 && m_rd > 0.0 { n_ed.abs()/n_rd + m_ed.abs()/m_rd } else { 0.0 };
    let mut rem_nm = Vec::new();
    if u_i > 1.0 {
        // Scale both N and M characteristic leaves by 1/u so sequential (or either path set) lands ≤ 1.
        let n_lim = n_ed.abs() / u_i * 0.99;
        let m_lim = m_ed.abs() / u_i * 0.99;
        rem_nm.extend(scale_action_leaf_remedies(
            "coldFormed", &sheet.id, &sheet.actions, "mYK", |a| a.m_y_k, m_ed, m_lim, q_moment,
            loc("Sheet moment", "Blechmoment"),
            "Reduce M_y,k so N–M interaction ≤ 1.",
            "M_y,k senken, sodass N–M-Interaktion ≤ 1.",
        ));
        rem_nm.extend(scale_action_leaf_remedies(
            "coldFormed", &sheet.id, &sheet.actions, "nK", |a| a.n_k, n_ed, n_lim, q_force,
            loc("Sheet axial", "Blech-Normalkraft"),
            "Reduce N_k so N–M interaction ≤ 1.",
            "N_k senken, sodass N–M-Interaktion ≤ 1.",
        ));
        rem_nm.extend(scale_action_leaf_remedies(
            "coldFormed", &sheet.id, &sheet.actions, "gKLine", |a| a.g_k_line, u_i, 1.0, q_force,
            loc("Sheet", "Blech"),
            "Reduce g_k so N–M interaction ≤ 1.",
            "g_k senken, sodass N–M-Interaktion ≤ 1.",
        ));
        rem_nm.extend(scale_action_leaf_remedies(
            "coldFormed", &sheet.id, &sheet.actions, "qKLine", |a| a.q_k_line, u_i, 1.0, q_force,
            loc("Sheet", "Blech"),
            "Reduce q_k so N–M interaction ≤ 1.",
            "q_k senken, sodass N–M-Interaktion ≤ 1.",
        ));
        let t_req = sheet.thickness * u_i.sqrt() * 1.05;
        rem_nm.push(Remedy::at_least(
            SubjectRef::new(&sheet.id, format!("coldFormed[id={}].thickness", sheet.id), loc("Sheet thickness", "Blechdicke")),
            q_length(sheet.thickness),
            q_length(t_req),
            loc(
                &format!("Increase thickness to ≥ {:.2} mm for N–M.", t_req * 1e3),
                &format!("Dicke auf ≥ {:.2} mm für N–M erhöhen.", t_req * 1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-4.nm.{}", sheet.id),
        "DIN EN 1999-1-4",
        ClauseId::new("EN 1999-1-4", "§6.1.4", "6.1.4"),
        SubjectRef::new(&sheet.id, format!("coldFormed[id={}].actions[id={}].mYK", sheet.id, action_id), loc(&format!("Sheet {}", sheet.id), &format!("Blech {}", sheet.id))),
        loc("Cold-formed N–M interaction", "N–M-Interaktion kaltgeformt"),
        q_dim(u_i),
        q_dim(1.0),
        annex,
        loc(
            &format!("|N|/N_Rd + |M|/M_Rd = {u_i:.3} (welded={}, ULS {combo}, lead {action_id}).", sheet.welded),
            &format!("|N|/N_Rd + |M|/M_Rd = {u_i:.3} (geschweißt={}, GZT {combo}, führend {action_id}).", sheet.welded),
        ),
        rem_nm,
    ));
    out
}

pub(super) fn check_shell(shell: &AluminiumShell, material: &AluminiumMaterial, annex: AnnexChoice) -> Vec<CheckResult> {
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return vec![CheckResult::assess(
            format!("en1999.1-5.alloy.{}", shell.id),
            "DIN EN 1999-1-5",
            ClauseId::new("EN 1999-1-5", "§5", "5.3"),
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
    let params = na_de::AnnexParams::for_choice(annex);
    let gov = part_en1990::governing_shell_effects(shell, annex);
    let (combo, action_id) = (gov.combination, gov.action_id.clone());
    let sigma_x_ed = gov.n_ed;
    let sigma_theta_ed = gov.m_y_ed + gov.m_z_ed;
    let tau_ed = gov.v_y_ed.hypot(gov.v_z_ed);
    let e = na_de::E_PA;
    let r = shell.radius.max(1e-9);
    let t = shell.thickness.max(1e-9);
    let l = shell.length.max(t);
    let alpha = 0.62;
    let beta_shell = 0.60;
    let eta = 1.0;
    let lambda_0 = 0.20;
    let sigma_x_rcr = 0.605 * e * (t / r);
    let sigma_theta_rcr = 0.92 * e * (t / r) * (r / l).sqrt();
    let tau_rcr = 0.75 * e * (t / r) * (r / l).powf(0.75);
    let lambda_x = if sigma_x_rcr > 0.0 { (alloy.f_o_pa / sigma_x_rcr).sqrt() } else { 0.0 };
    let phi_x = 0.5 * (1.0 + alpha * (lambda_x - lambda_0) + beta_shell * lambda_x.powi(2));
    let chi_x = (1.0 / (phi_x + (phi_x * phi_x - beta_shell * lambda_x.powi(2)).max(0.0).sqrt())).min(1.0) * eta;
    let lambda_th = if sigma_theta_rcr > 0.0 { (alloy.f_o_pa / sigma_theta_rcr).sqrt() } else { 0.0 };
    let phi_th = 0.5 * (1.0 + alpha * (lambda_th - lambda_0) + beta_shell * lambda_th.powi(2));
    let chi_th = (1.0 / (phi_th + (phi_th * phi_th - beta_shell * lambda_th.powi(2)).max(0.0).sqrt())).min(1.0);
    let sigma_x_rd = chi_x * sigma_x_rcr / params.gamma_m1;
    let sigma_theta_rd = chi_th * sigma_theta_rcr.min(alloy.f_o_pa) / params.gamma_m1;
    let tau_rd = tau_rcr / params.gamma_m1;
    let mut out = Vec::new();
    let mut remedies_x = Vec::new();
    if sigma_x_ed.abs() > sigma_x_rd && sigma_x_rd > 0.0 {
        remedies_x.push(Remedy::at_most(
            SubjectRef::new(&shell.id, format!("shells[id={}].actions[id={}].nK", shell.id, action_id), loc("Meridional stress", "Meridianspannung")),
            q_stress(sigma_x_ed.abs()),
            q_stress(sigma_x_rd),
            loc(
                &format!("Reduce σ_x,Ed to ≤ {:.0} MPa or increase t/r.", sigma_x_rd/1e6),
                &format!("σ_x,Ed auf ≤ {:.0} MPa reduzieren oder t/r erhöhen.", sigma_x_rd/1e6),
            ),
        ));
        let t_req = if e > 0.0 && shell.radius > 0.0 {
            sigma_x_ed.abs() * params.gamma_m1 / (chi_x * 0.605 * e) * shell.radius
        } else {
            shell.thickness
        };
        remedies_x.push(Remedy::at_least(
            SubjectRef::new(&shell.id, format!("shells[id={}].thickness", shell.id), loc("Shell thickness", "Schalendicke")),
            q_length(shell.thickness),
            q_length(t_req),
            loc(
                &format!("Increase shell thickness to ≥ {:.2} mm.", t_req*1e3),
                &format!("Schalendicke auf ≥ {:.2} mm erhöhen.", t_req*1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-5.buckle.{}", shell.id),
        "DIN EN 1999-1-5",
        ClauseId::new("EN 1999-1-5", "§5.3", "5.3"),
        SubjectRef::new(&shell.id, format!("shells[id={}].actions[id={}].nK", shell.id, action_id), loc(&format!("Shell {}", shell.id), &format!("Schale {}", shell.id))),
        loc("Shell meridional buckling", "Schalenbeulen in Meridianrichtung"),
        q_stress(sigma_x_ed.abs()),
        q_stress(sigma_x_rd),
        annex,
        loc(
            &format!("σ_x,Ed={:.0} MPa, σ_x,Rd={:.0} MPa (σ_x,Rcr={:.0} MPa, χ={chi_x:.2}, ULS {combo}, lead {action_id}).", sigma_x_ed.abs()/1e6, sigma_x_rd/1e6, sigma_x_rcr/1e6),
            &format!("σ_x,Ed={:.0} MPa, σ_x,Rd={:.0} MPa (σ_x,Rcr={:.0} MPa, χ={chi_x:.2}, GZT {combo}, führend {action_id}).", sigma_x_ed.abs()/1e6, sigma_x_rd/1e6, sigma_x_rcr/1e6),
        ),
        remedies_x,
    ));
    let mut remedies_t = Vec::new();
    if sigma_theta_ed.abs() > sigma_theta_rd && sigma_theta_rd > 0.0 {
        remedies_t.extend(scale_action_leaf_remedies(
            "shells", &shell.id, &shell.actions, "mYK", |a| a.m_y_k, sigma_theta_ed, sigma_theta_rd, q_stress,
            loc("Circumferential stress", "Umfangsspannung"),
            &format!("Reduce M_y,k (σ_θ) to ≤ {:.0} MPa design.", sigma_theta_rd/1e6),
            &format!("M_y,k (σ_θ) senken auf Bemessung ≤ {:.0} MPa.", sigma_theta_rd/1e6),
        ));
        remedies_t.extend(scale_action_leaf_remedies(
            "shells", &shell.id, &shell.actions, "mZK", |a| a.m_z_k, sigma_theta_ed, sigma_theta_rd, q_stress,
            loc("Circumferential stress", "Umfangsspannung"),
            "Reduce M_z,k so circumferential shell stress complies.",
            "M_z,k senken, damit Umfangsspannung der Schale erfüllt ist.",
        ));
        let t_req = shell.thickness * (sigma_theta_ed.abs() / sigma_theta_rd) * 1.05;
        remedies_t.push(Remedy::at_least(
            SubjectRef::new(&shell.id, format!("shells[id={}].thickness", shell.id), loc("Shell thickness", "Schalendicke")),
            q_length(shell.thickness),
            q_length(t_req),
            loc(
                &format!("Increase shell thickness to ≥ {:.2} mm.", t_req*1e3),
                &format!("Schalendicke auf ≥ {:.2} mm erhöhen.", t_req*1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-5.ring.{}", shell.id),
        "DIN EN 1999-1-5",
        ClauseId::new("EN 1999-1-5", "§5.3", "5.3"),
        SubjectRef::new(&shell.id, format!("shells[id={}].actions[id={}].mYK", shell.id, action_id), loc(&format!("Shell {}", shell.id), &format!("Schale {}", shell.id))),
        loc("Shell circumferential membrane stress", "Umfangsmembranspannung der Schale"),
        q_stress(sigma_theta_ed.abs()),
        q_stress(sigma_theta_rd),
        annex,
        loc(
            &format!("σ_θ,Ed={:.0} MPa, σ_θ,Rd={:.0} MPa, L={:.0} mm (ULS {combo}, lead {action_id}).", sigma_theta_ed.abs()/1e6, sigma_theta_rd/1e6, shell.length*1e3),
            &format!("σ_θ,Ed={:.0} MPa, σ_θ,Rd={:.0} MPa, L={:.0} mm (GZT {combo}, führend {action_id}).", sigma_theta_ed.abs()/1e6, sigma_theta_rd/1e6, shell.length*1e3),
        ),
        remedies_t,
    ));
    let mut remedies_tau = Vec::new();
    if tau_ed.abs() > tau_rd && tau_rd > 0.0 {
        remedies_tau.extend(scale_action_leaf_remedies(
            "shells", &shell.id, &shell.actions, "vZK", |a| a.v_z_k, tau_ed, tau_rd, q_stress,
            loc("Shear stress", "Schubspannung"),
            &format!("Reduce V_z,k (τ) to ≤ {:.0} MPa design.", tau_rd/1e6),
            &format!("V_z,k (τ) senken auf Bemessung ≤ {:.0} MPa.", tau_rd/1e6),
        ));
        remedies_tau.extend(scale_action_leaf_remedies(
            "shells", &shell.id, &shell.actions, "vYK", |a| a.v_y_k, tau_ed, tau_rd, q_stress,
            loc("Shear stress", "Schubspannung"),
            "Reduce V_y,k so shell shear complies.",
            "V_y,k senken, damit Schalenschub erfüllt ist.",
        ));
        let t_req = shell.thickness * (tau_ed.abs() / tau_rd) * 1.05;
        remedies_tau.push(Remedy::at_least(
            SubjectRef::new(&shell.id, format!("shells[id={}].thickness", shell.id), loc("Shell thickness", "Schalendicke")),
            q_length(shell.thickness),
            q_length(t_req),
            loc(
                &format!("Increase shell thickness to ≥ {:.2} mm for shear.", t_req*1e3),
                &format!("Schalendicke auf ≥ {:.2} mm für Schub erhöhen.", t_req*1e3),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.1-5.shear.{}", shell.id),
        "DIN EN 1999-1-5",
        ClauseId::new("EN 1999-1-5", "§5.3", "5.3"),
        SubjectRef::new(&shell.id, format!("shells[id={}].actions[id={}].vZK", shell.id, action_id), loc(&format!("Shell {}", shell.id), &format!("Schale {}", shell.id))),
        loc("Shell shear buckling τ ≤ τ_Rcr/γ_M1", "Schalenschubbeulen τ ≤ τ_Rcr/γ_M1"),
        q_stress(tau_ed.abs()),
        q_stress(tau_rd.max(1e-9)),
        annex,
        loc(
            &format!("τ_Ed={:.0} MPa, τ_Rcr={:.0} MPa, τ_Rd={:.0} MPa (ULS {combo}, lead {action_id}).", tau_ed.abs()/1e6, tau_rcr/1e6, tau_rd/1e6),
            &format!("τ_Ed={:.0} MPa, τ_Rcr={:.0} MPa, τ_Rd={:.0} MPa (GZT {combo}, führend {action_id}).", tau_ed.abs()/1e6, tau_rcr/1e6, tau_rd/1e6),
        ),
        remedies_tau,
    ));
    out
}
//#endregion 🔖️ComplianceHelpers
