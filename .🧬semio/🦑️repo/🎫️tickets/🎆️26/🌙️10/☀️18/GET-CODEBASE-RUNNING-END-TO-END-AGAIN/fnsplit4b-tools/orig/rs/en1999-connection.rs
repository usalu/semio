//! 🔗️ EN 1999 connection checks: `check_connection` verifies one aluminium connection against every clause.

use super::*;

//#region 🔖️ComplianceHelpers
pub fn check_connection(conn: &AluminiumConnection, material: &AluminiumMaterial, member: Option<&AluminiumMember>, member_ids: &[String], annex: AnnexChoice) -> Vec<CheckResult> {
    let params = na_de::AnnexParams::for_choice(annex);
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return vec![CheckResult::assess(
            format!("en1999.3.2.alloy.{}", conn.id),
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
    let part = "DIN EN 1999-1-1";
    let mut out = Vec::new();
    let kind = conn.kind.to_ascii_lowercase();

    let gov = if conn.actions.is_empty() {
        member.map(|m| part_en1990::governing_member_effects(m, annex)).unwrap_or(part_en1990::GoverningEffects {
            action_id: "G".into(), combination: "uls-g", n_ed: 0.0, v_y_ed: 0.0, v_z_ed: 0.0, m_y_ed: 0.0, m_z_ed: 0.0,
        })
    } else {
        part_en1990::governing_connection_effects(conn, annex)
    };
    let combo = gov.combination;
    let action_id = gov.action_id.as_str();
    let lever = 0.50;
    let n_ed = gov.n_ed.abs() + 0.25 * (gov.m_y_ed.abs() + gov.m_z_ed.abs()) / lever;
    let v_ed = gov.v_y_ed.abs() + gov.v_z_ed.abs() + (gov.m_y_ed.abs() + gov.m_z_ed.abs()) / lever;
    let m_ed = gov.m_y_ed.abs() + gov.m_z_ed.abs();
    let conn_dig = actions_digest(&conn.actions);
    if member.is_none() {
        return vec![CheckResult::assess(
            format!("en1999.8.ref.{}", conn.id),
            "DIN EN 1999-1-1",
            ClauseId::new("EN 1999-1-1", "§8", "8"),
            SubjectRef::new(&conn.id, format!("connections[id={}].memberId", conn.id), loc("Connection", "Anschluss")),
            loc("Connection member reference", "Anschluss-Bauteilverweis"),
        )
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .explanation(loc(
            &format!("Connection '{}' references missing member '{}'.", conn.id, conn.member_id),
            &format!("Anschluss '{}' verweist auf fehlendes Bauteil '{}'.", conn.id, conn.member_id),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&conn.id, format!("connections[id={}].memberId", conn.id), conn_label(&conn.id)),
            member_ids.to_vec(),
            loc("Point connection.memberId at an existing member.", "connections.memberId auf ein vorhandenes Bauteil setzen."),
        ))
        .build()];
    }

    if kind == "bolted" || kind == "combined" {
        if part_1_1::bolt_fu_pa(&conn.bolts.material) <= 0.0 {
            out.push(CheckResult::assess(
                format!("en1999.8.5.boltgrade.{}", conn.id),
                part,
                ClauseId::new("EN 1999-1-1", "§8.5", "8.5"),
                SubjectRef::new(&conn.id, format!("connections[id={}].bolts.material", conn.id), conn_label(&conn.id)),
                loc("Bolt grade", "Schraubenfestigkeit"),
            )
            .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
            .explanation(loc(
                &format!("Unknown bolt grade '{}'.", conn.bolts.material),
                &format!("Unbekannte Schraubenfestigkeit '{}'.", conn.bolts.material),
            ))
            .annex(annex)
            .remedy(Remedy::one_of(
                SubjectRef::new(&conn.id, format!("connections[id={}].bolts.material", conn.id), conn_label(&conn.id)),
                vec!["4.6".into(), "5.6".into(), "8.8".into(), "10.9".into()],
                loc("Select a bolt grade from EN 1999-1-1 Table 3.4.", "Schraubenfestigkeit nach EN 1999-1-1 Tabelle 3.4 wählen."),
            ))
            .build());
        }
        let f_v = part_1_1::bolt_shear_resistance(&conn.bolts, params.gamma_m2);
        let f_b = part_1_1::bolt_bearing_resistance(&conn.bolts, alloy.f_u_pa, params.gamma_m2);
        let pitch_ratio = conn.bolts.pitch / conn.bolts.diameter.max(1e-6);
        let pitch_factor = (0.85 + 0.05 * pitch_ratio).clamp(0.80, 1.25);
        let f_rd = (if f_v > 0.0 && f_b > 0.0 { (f_v * f_b) / (f_v + f_b) } else { f_v.min(f_b) }) * pitch_factor;
        let mut remedies = Vec::new();
        let force_bolt = v_ed + n_ed.abs();
        if force_bolt > f_rd && f_rd > 0.0 {
            let n_now = (conn.bolts.rows * conn.bolts.bolts_per_row) as f64;
            let n_req = (n_now * force_bolt / f_rd * 1.05).ceil().max(n_now + 1.0) as u32;
            let per_row = (n_req / conn.bolts.rows.max(1)).max(conn.bolts.bolts_per_row + 1);
            remedies.push(Remedy::at_least(
                SubjectRef::new(&conn.id, format!("connections[id={}].bolts.boltsPerRow", conn.id), conn_label(&conn.id)),
                q_dim(conn.bolts.bolts_per_row as f64),
                q_dim(per_row as f64),
                loc(
                    &format!("Increase bolt count (rows×bolts) so F_Rd ≥ {:.1} kN.", force_bolt/1e3),
                    &format!("Schraubenzahl erhöhen, sodass F_Rd ≥ {:.1} kN.", force_bolt/1e3),
                ),
            ));
            let rows_req = (n_req / conn.bolts.bolts_per_row.max(1)).max(conn.bolts.rows + 1);
            remedies.push(Remedy::at_least(
                SubjectRef::new(&conn.id, format!("connections[id={}].bolts.rows", conn.id), conn_label(&conn.id)),
                q_dim(conn.bolts.rows as f64),
                q_dim(rows_req as f64),
                loc(
                    &format!("Increase bolt rows so F_Rd ≥ {:.1} kN.", force_bolt/1e3),
                    &format!("Schraubenreihen erhöhen, sodass F_Rd ≥ {:.1} kN.", force_bolt/1e3),
                ),
            ));
            remedies.extend(scale_action_leaf_remedies(
                "connections", &conn.id, &conn.actions, "vZK", |a| a.v_z_k, force_bolt, f_rd, q_force,
                conn_label(&conn.id),
                "Reduce V_z,k so bolted resistance governs.",
                "V_z,k senken, damit Schraubentragfähigkeit maßgebend ist.",
            ));
            remedies.extend(scale_action_leaf_remedies(
                "connections", &conn.id, &conn.actions, "nK", |a| a.n_k, force_bolt, f_rd, q_force,
                conn_label(&conn.id),
                "Reduce N_k so bolted resistance governs.",
                "N_k senken, damit Schraubentragfähigkeit maßgebend ist.",
            ));
        }
        if conn.bolts.edge_distance < 1.2 * conn.bolts.diameter {
            remedies.push(Remedy::at_least(
                SubjectRef::new(&conn.id, format!("connections[id={}].bolts.edgeDistance", conn.id), conn_label(&conn.id)),
                q_length(conn.bolts.edge_distance),
                q_length(1.2 * conn.bolts.diameter),
                loc("Increase edge distance e1 to ≥ 1.2 d.", "Randabstand e1 auf ≥ 1.2 d erhöhen."),
            ));
        }
        let p_min = 2.2 * conn.bolts.diameter;
        out.push(utilization_check(
            format!("en1999.8.5.pitch.{}", conn.id),
            part,
            ClauseId::new("EN 1999-1-1", "§8.5", "8.5.1"),
            SubjectRef::new(&conn.id, format!("connections[id={}].bolts.pitch", conn.id), conn_label(&conn.id)),
            loc("Minimum bolt pitch p1 ≥ 2.2 d", "Mindestlochabstand p1 ≥ 2.2 d"),
            q_length(p_min),
            q_length(conn.bolts.pitch.max(1e-9)),
            annex,
            loc(
                &format!("p1={:.4} m, 2.2d={:.4} m (bolt spacing).", conn.bolts.pitch, p_min),
                &format!("p1={:.4} m, 2.2d={:.4} m (Schraubenabstand).", conn.bolts.pitch, p_min),
            ),
            if conn.bolts.pitch + 1e-12 < p_min {
                vec![Remedy::at_least(
                    SubjectRef::new(&conn.id, format!("connections[id={}].bolts.pitch", conn.id), conn_label(&conn.id)),
                    q_length(conn.bolts.pitch),
                    q_length(p_min),
                    loc("Increase pitch to ≥ 2.2 d.", "Lochabstand auf ≥ 2.2 d erhöhen."),
                )]
            } else { Vec::new() },
        ));
        out.push(utilization_check(
            format!("en1999.8.5.bolt.{}", conn.id),
            part,
            ClauseId::new("EN 1999-1-1", "§8.5", "8.5"),
            SubjectRef::new(&conn.id, format!("connections[id={}].bolts.diameter", conn.id), conn_label(&conn.id)),
            loc("Bolted connection resistance", "Schraubenanschluss Tragfähigkeit"),
            q_force(v_ed + n_ed.abs()),
            q_force(f_rd),
            annex,
            loc(
                &format!("V_Ed={:.1} kN, N_Ed={:.1} kN, M_Ed={:.2} kNm, F_v,Rd={:.1} kN, F_b,Rd={:.1} kN, d={d:.4} m, e1={e1:.4} m, p1={p1:.4} m, e2={e2:.4} m (ULS {combo}, lead {action_id}, bolt {mat}); {conn_dig}", v_ed/1e3, n_ed.abs()/1e3, m_ed/1e3, f_v/1e3, f_b/1e3, d=conn.bolts.diameter, e1=conn.bolts.edge_distance, p1=conn.bolts.pitch, e2=conn.bolts.gauge, mat=conn.bolts.material),
                &format!("V_Ed={:.1} kN, N_Ed={:.1} kN, M_Ed={:.2} kNm, F_v,Rd={:.1} kN, F_b,Rd={:.1} kN, d={d:.4} m, e1={e1:.4} m, p1={p1:.4} m, e2={e2:.4} m (GZT {combo}, führend {action_id}, Schraube {mat}); {conn_dig}", v_ed/1e3, n_ed.abs()/1e3, m_ed/1e3, f_v/1e3, f_b/1e3, d=conn.bolts.diameter, e1=conn.bolts.edge_distance, p1=conn.bolts.pitch, e2=conn.bolts.gauge, mat=conn.bolts.material),
            ),
            remedies,
        ));
    }

    if kind == "welded" || kind == "combined" {
        let f_w = part_1_1::weld_resistance(&conn.welds, alloy.f_u_pa, params.gamma_m2) * (alloy.rho_u_haz.min(alloy.rho_o_haz) / (1.0 + part_1_1::haz_extent_m(&conn.welds, conn.bolts.plate_thickness).max(conn.welds.throat) * 10.0)).clamp(0.2, 1.0);
        let mut remedies = Vec::new();
        let force_weld = v_ed + n_ed.abs();
        if force_weld > f_w && f_w > 0.0 {
            let rho_haz = alloy.rho_u_haz.min(alloy.rho_o_haz);
            let mut t_req = conn.welds.throat;
            for _ in 0..48 {
                let mut trial = conn.welds.clone();
                trial.throat = t_req;
                let haz = part_1_1::haz_extent_m(&trial, conn.bolts.plate_thickness).max(trial.throat);
                let fac = (rho_haz / (1.0 + haz * 10.0)).clamp(0.2, 1.0);
                let f_w_base = part_1_1::weld_resistance(&trial, alloy.f_u_pa, params.gamma_m2);
                let need = force_weld / fac.max(1e-9);
                let a_need = need * trial.beta_w.max(0.01) * params.gamma_m2 / part_1_1::filler_fu_pa(&trial.filler_alloy, alloy.f_u_pa).max(1e-9);
                t_req = (a_need / trial.length.max(1e-6)).max(t_req);
                if f_w_base * fac >= force_weld * 0.999 { break; }
            }
            remedies.push(Remedy::at_least(
                SubjectRef::new(&conn.id, format!("connections[id={}].welds.throat", conn.id), conn_label(&conn.id)),
                q_length(conn.welds.throat),
                q_length(t_req),
                loc(
                    &format!("Increase weld throat from {:.1} mm to at least {:.1} mm.", conn.welds.throat*1e3, t_req*1e3),
                    &format!("Kehlnahtdicke von {:.1} mm auf mindestens {:.1} mm erhöhen.", conn.welds.throat*1e3, t_req*1e3),
                ),
            ));
            let haz0 = part_1_1::haz_extent_m(&conn.welds, conn.bolts.plate_thickness).max(conn.welds.throat);
            let fac0 = (rho_haz / (1.0 + haz0 * 10.0)).clamp(0.2, 1.0);
            let a_need = force_weld / fac0.max(1e-9) * conn.welds.beta_w.max(0.01) * params.gamma_m2
                / part_1_1::filler_fu_pa(&conn.welds.filler_alloy, alloy.f_u_pa).max(1e-9);
            let l_req = a_need / conn.welds.throat.max(1e-6);
            remedies.push(Remedy::at_least(
                SubjectRef::new(&conn.id, format!("connections[id={}].welds.length", conn.id), conn_label(&conn.id)),
                q_length(conn.welds.length),
                q_length(l_req),
                loc(
                    &format!("Increase weld length from {:.0} mm to at least {:.0} mm (or relocate weld).", conn.welds.length*1e3, l_req*1e3),
                    &format!("Nahtlänge von {:.0} mm auf mindestens {:.0} mm erhöhen (oder Naht verlegen).", conn.welds.length*1e3, l_req*1e3),
                ),
            ));
            remedies.push(Remedy::at_most(
                SubjectRef::new(&conn.id, format!("connections[id={}].welds.hazExtent", conn.id), conn_label(&conn.id)),
                q_length(conn.welds.haz_extent),
                q_length(0.0),
                loc("Reduce stored HAZ extent so weld resistance recovers.", "Gespeicherte WEZ-Ausdehnung senken, damit Nahttragfähigkeit steigt."),
            ));
            remedies.extend(scale_action_leaf_remedies(
                "connections", &conn.id, &conn.actions, "vZK", |a| a.v_z_k, force_weld, f_w, q_force,
                conn_label(&conn.id),
                "Reduce V_z,k so weld resistance governs.",
                "V_z,k senken, damit Schweißtragfähigkeit maßgebend ist.",
            ));
        }
        out.push(utilization_check(
            format!("en1999.8.6.weld.{}", conn.id),
            part,
            ClauseId::new("EN 1999-1-1", "§8.6", "8.6"),
            SubjectRef::new(&conn.id, format!("connections[id={}].welds.throat", conn.id), conn_label(&conn.id)),
            loc("Welded connection resistance", "Schweißanschluss Tragfähigkeit"),
            q_force(v_ed + n_ed.abs()),
            q_force(f_w),
            annex,
            loc(
                &format!("V_Ed={:.1} kN, N_Ed={:.1} kN, M_Ed={:.2} kNm, F_w,Rd={:.1} kN, a={:.1} mm, ℓ={:.0} mm, HAZ={:.0} mm, βw={bw:.3}, filler={filler} (ULS {combo}, lead {action_id}); {conn_dig}", v_ed/1e3, n_ed.abs()/1e3, m_ed/1e3, f_w/1e3, conn.welds.throat*1e3, conn.welds.length*1e3, conn.welds.haz_extent*1e3, bw=conn.welds.beta_w, filler=conn.welds.filler_alloy),
                &format!("V_Ed={:.1} kN, N_Ed={:.1} kN, M_Ed={:.2} kNm, F_w,Rd={:.1} kN, a={:.1} mm, ℓ={:.0} mm, WEZ={:.0} mm, βw={bw:.3}, Schweißzusatz={filler} (GZT {combo}, führend {action_id}); {conn_dig}", v_ed/1e3, n_ed.abs()/1e3, m_ed/1e3, f_w/1e3, conn.welds.throat*1e3, conn.welds.length*1e3, conn.welds.haz_extent*1e3, bw=conn.welds.beta_w, filler=conn.welds.filler_alloy),
            ),
            remedies,
        ));
    }

    if !conn.actions.is_empty() && (kind == "bolted" || kind == "welded" || kind == "combined") {
        let sls = part_en1990::sls_connection_effects(conn, annex, "frequent");
        let n_ed_s = sls.n_ed.abs() + 0.25 * (sls.m_y_ed.abs() + sls.m_z_ed.abs()) / lever;
        let v_ed_s = sls.v_y_ed.abs() + sls.v_z_ed.abs() + (sls.m_y_ed.abs() + sls.m_z_ed.abs()) / lever;
        let force_s = v_ed_s + n_ed_s;
        let mut f_lim = 0.0;
        if kind == "bolted" || kind == "combined" {
            let f_v = part_1_1::bolt_shear_resistance(&conn.bolts, params.gamma_m2);
            let f_b = part_1_1::bolt_bearing_resistance(&conn.bolts, alloy.f_u_pa, params.gamma_m2);
            let pitch_ratio = conn.bolts.pitch / conn.bolts.diameter.max(1e-6);
            let pitch_factor = (0.85 + 0.05 * pitch_ratio).clamp(0.80, 1.25);
            f_lim = (if f_v > 0.0 && f_b > 0.0 { (f_v * f_b) / (f_v + f_b) } else { f_v.min(f_b) }) * pitch_factor;
        }
        if kind == "welded" || kind == "combined" {
            let f_w = part_1_1::weld_resistance(&conn.welds, alloy.f_u_pa, params.gamma_m2)
                * (alloy.rho_u_haz.min(alloy.rho_o_haz)
                    / (1.0 + part_1_1::haz_extent_m(&conn.welds, conn.bolts.plate_thickness).max(conn.welds.throat) * 10.0))
                    .clamp(0.2, 1.0);
            f_lim = if f_lim > 0.0 { f_lim.min(f_w) } else { f_w };
        }
        let mut rem_s = Vec::new();
        if force_s > f_lim && f_lim > 0.0 {
            rem_s.extend(scale_action_leaf_remedies(
                "connections", &conn.id, &conn.actions, "vZK", |a| a.v_z_k, force_s, f_lim, q_force,
                conn_label(&conn.id),
                "Reduce V_z,k so frequent SLS connection force ≤ F_Rd.",
                "V_z,k senken, damit häufige GZG-Anschlusskraft ≤ F_Rd.",
            ));
            rem_s.extend(scale_action_leaf_remedies(
                "connections", &conn.id, &conn.actions, "nK", |a| a.n_k, force_s, f_lim, q_force,
                conn_label(&conn.id),
                "Reduce N_k so frequent SLS connection force ≤ F_Rd.",
                "N_k senken, damit häufige GZG-Anschlusskraft ≤ F_Rd.",
            ));
            rem_s.extend(scale_action_leaf_remedies(
                "connections", &conn.id, &conn.actions, "qKLine", |a| a.q_k_line, force_s, f_lim, q_force,
                conn_label(&conn.id),
                "Reduce q_k so frequent SLS connection force ≤ F_Rd.",
                "q_k senken, damit häufige GZG-Anschlusskraft ≤ F_Rd.",
            ));
            rem_s.extend(scale_action_leaf_remedies(
                "connections", &conn.id, &conn.actions, "gKLine", |a| a.g_k_line, force_s, f_lim, q_force,
                conn_label(&conn.id),
                "Reduce g_k so frequent SLS connection force ≤ F_Rd.",
                "g_k senken, damit häufige GZG-Anschlusskraft ≤ F_Rd.",
            ));
            rem_s.extend(scale_action_leaf_remedies(
                "connections", &conn.id, &conn.actions, "mYK", |a| a.m_y_k, force_s, f_lim, q_moment,
                conn_label(&conn.id),
                "Reduce M_y,k so frequent SLS connection force ≤ F_Rd.",
                "M_y,k senken, damit häufige GZG-Anschlusskraft ≤ F_Rd.",
            ));
        }
        out.push(utilization_check(
            format!("en1999.8.sls-freq.{}", conn.id),
            part,
            ClauseId::new("EN 1990", "§6.5.3", "6.5.3"),
            SubjectRef::new(
                &conn.id,
                format!("connections[id={}].actions[id={}].category", conn.id, sls.action_id),
                conn_label(&conn.id),
            ),
            loc("Connection force (frequent SLS ψ₁)", "Anschlusskraft (häufiges GZG ψ₁)"),
            q_force(force_s),
            q_force(f_lim.max(1e-9)),
            annex,
            loc(
                &format!(
                    "F_Ed,freq={:.1} kN ≤ F_Rd={:.1} kN (SLS {comb}, lead {aid}, ψ₁ via category); {conn_dig}",
                    force_s / 1e3,
                    f_lim / 1e3,
                    comb = sls.combination,
                    aid = sls.action_id
                ),
                &format!(
                    "F_Ed,freq={:.1} kN ≤ F_Rd={:.1} kN (GZG {comb}, führend {aid}, ψ₁ über Kategorie); {conn_dig}",
                    force_s / 1e3,
                    f_lim / 1e3,
                    comb = sls.combination,
                    aid = sls.action_id
                ),
            ),
            rem_s,
        ));
    }

    if out.is_empty() {
        out.push(
            CheckResult::assess(
                format!("en1999.8.na.{}", conn.id),
                part,
                ClauseId::new("EN 1999-1-1", "§8", "8"),
                SubjectRef::new(&conn.id, format!("connections[id={}].kind", conn.id), conn_label(&conn.id)),
                loc("Connection check", "Anschlussnachweis"),
            )
            .not_applicable(loc("Connection kind not bolted/welded.", "Anschlussart weder geschraubt noch geschweißt."))
            .annex(annex)
            .build(),
        );
    }
    out
}
//#endregion 🔖️ComplianceHelpers
