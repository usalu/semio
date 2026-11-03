//! 🕸️ Frame model check of EN 1993-1-1 5.2.1: the elastic critical load factor α_cr of a general planar frame from the mechanics-kernel eigenvalue problem and the analysis mode it requires. See [EN 1993-1-1](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::global;
use semio_s_artifact_norm_en1993_frame_eigen as frame_eigen;
use semio_s_artifact_norm_en1993_model::FrameBar;

fn alpha_cr_min() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.alpha-cr-min")
}

fn subject(bar: &FrameBar, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&bar.id, format!("frameBars[id={}].{field}", bar.id), loc(en, de))
}

fn finish(report: &mut CheckReport, annex: AnnexChoice, id: String, subject: SubjectRef, title: LocalizedCopy, explanation: LocalizedCopy, build: impl FnOnce(semio_s_plugin_norm_compliance::CheckBuilder) -> semio_s_plugin_norm_compliance::CheckBuilder) {
    let base = CheckResult::assess(id, "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "5", "5.2.1"), subject, title).annex(annex).explanation(explanation);
    report.push(build(base).build());
}

fn upsize(b: semio_s_plugin_norm_compliance::CheckBuilder, member: &semio_s_artifact_norm_en1993_model::SteelMember, section: &semio_s_artifact_norm_en1993_model::SteelSection, eta: f64) -> semio_s_plugin_norm_compliance::CheckBuilder {
    if eta <= 1.0 {
        return b;
    }
    let k = if eta.is_finite() { eta } else { 2.0 };
    let options = next_section_options(section, section.area * k, section.w_pl_y * k);
    if options.is_empty() {
        return b.remedy(Remedy::at_least(
            SubjectRef::new(&member.id, format!("sections[id={}].area", section.id), loc("Section area", "Querschnittsfläche")),
            area_m2(section.area),
            area_m2(section.area * k),
            loc(&format!("Increase the section area to at least {:.1} cm² or reduce the frame loads.", section.area * k * 1e4), &format!("Querschnittsfläche auf mindestens {:.1} cm² erhöhen oder die Rahmenlasten verringern.", section.area * k * 1e4)),
        ));
    }
    b.remedy(Remedy::one_of(
        SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), loc("Section", "Querschnitt")),
        options.clone(),
        loc(&format!("Upsize the section: {}.", options.join(", ")), &format!("Querschnitt vergrößern: {}.", options.join(", "))),
    ))
}

fn titles() -> (LocalizedCopy, LocalizedCopy) {
    (loc("Elastic critical load factor α_cr of the frame", "Elastischer Verzweigungslastfaktor α_cr des Rahmens"), loc("Frame model", "Rahmenmodell"))
}

fn solved(annex: AnnexChoice, bar: &FrameBar, eigen: &frame_eigen::FrameEigen, document: &En1993Snapshot, report: &mut CheckReport) {
    let (title, _) = titles();
    let sway = eigen.sway();
    let alpha = eigen.governing();
    let plastic = false;
    let limit = global::alpha_cr_limit(plastic);
    let mode = global::analysis_mode(alpha, plastic);
    let amplification = global::amplification(alpha).map_or("-".to_string(), |k| format!("{k:.3}"));
    let (mode_en, mode_de) = match mode {
        global::AnalysisMode::FirstOrder => ("first-order analysis", "Berechnung nach Theorie I. Ordnung"),
        global::AnalysisMode::AmplifiedFirstOrder => ("first-order analysis with amplified sway moments", "Theorie I. Ordnung mit vergrößerten Horizontalmomenten"),
        global::AnalysisMode::SecondOrder => ("second-order analysis", "Berechnung nach Theorie II. Ordnung"),
    };
    let (kind_en, kind_de) = if sway.is_some() { ("sway mode", "Verschiebungsform") } else { ("lowest mode (no sway mode among the first eight)", "niedrigste Form (keine Verschiebungsform unter den ersten acht)") };
    let explanation = loc(
        &format!("Kernel eigenvalue of {} nodes and {} bars: α_cr={alpha:.2} ({kind_en}), lowest of all modes {:.2}; elastic limit {limit:.0}; amplification 1/(1-1/α_cr)={amplification}; required: {mode_en}. Out-of-plane degrees of freedom are restrained.", document.frame_nodes.len(), document.frame_bars.len(), eigen.lowest()),
        &format!("Kern-Eigenwert mit {} Knoten und {} Stäben: α_cr={alpha:.2} ({kind_de}), niedrigster Wert aller Formen {:.2}; elastische Grenze {limit:.0}; Vergrößerung 1/(1-1/α_cr)={amplification}; erforderlich: {mode_de}. Die Freiheitsgrade aus der Ebene sind gehalten.", document.frame_nodes.len(), document.frame_bars.len(), eigen.lowest()),
    );
    let heaviest = document.frame_nodes.iter().filter(|n| n.load_z != 0.0).min_by(|a, b| a.load_z.total_cmp(&b.load_z));
    finish(report, annex, format!("en1993.1-1.5.2.frame.{}", bar.id), subject(bar, "memberId", "Steel member", "Stahlbauteil"), title, explanation, |b| {
        let b = b.minimum(dimensionless(alpha), dimensionless(alpha_cr_min()));
        if alpha >= alpha_cr_min() {
            return b;
        }
        let Some(node) = heaviest else { return b };
        let allowed = node.load_z * alpha / alpha_cr_min();
        b.remedy(Remedy::at_least(
            SubjectRef::new(&node.id, format!("frameNodes[id={}].loadZ", node.id), loc("Design load in z", "Bemessungslast in z")),
            force_n(node.load_z),
            force_n(allowed),
            loc(&format!("Reduce the vertical design load of node '{}' to {:.0} kN (all loads scale with α_cr/3) or stiffen the frame.", node.id, allowed / 1e3), &format!("Vertikale Bemessungslast des Knotens '{}' auf {:.0} kN verringern (alle Lasten skalieren mit α_cr/3) oder den Rahmen aussteifen.", node.id, allowed / 1e3)),
        ))
    });
}

fn failed(annex: AnnexChoice, bar: &FrameBar, fault: frame_eigen::FrameFault, report: &mut CheckReport) {
    let (title, _) = titles();
    let reason = loc(&fault.en, &fault.de);
    finish(report, annex, format!("en1993.1-1.5.2.frame.{}", bar.id), subject(bar, "memberId", "Steel member", "Stahlbauteil"), title, reason.clone(), |b| b.not_applicable(reason));
}

fn interaction(section: &semio_s_artifact_norm_en1993_model::SteelSection, fy: f64, params: AnnexParams, effects: frame_eigen::BarEffects) -> (f64, u8) {
    let class = part_1_1::section_class_rolled_i(section, fy);
    let area = if class == 4 { part_1_1::effective_area_class4(section, fy) } else { section.area };
    let n_pl = part_1_1::axial_resistance_n(area, fy, params);
    let m_y = part_1_1::bending_resistance_nm(part_1_1::bending_modulus_y(section, class, fy), fy, class, params);
    let m_z = part_1_1::bending_resistance_nm(part_1_1::bending_modulus_z(section, class, fy), fy, class, params);
    (part_1_1::section_mn_eta(section, effects.n, effects.m, 0.0, n_pl, m_y, m_z, class), class)
}

struct Bow {
    bar: String,
    length: f64,
    curve: &'static str,
    ratio: f64,
    n_ed: f64,
}

impl Bow {
    fn e0(&self) -> f64 {
        self.ratio * self.length
    }

    fn q(&self) -> f64 {
        global::bow_equivalent_load(self.n_ed, self.e0(), self.length)
    }
}

struct Derivation {
    height: f64,
    columns: usize,
    phi: f64,
    sway: Vec<(String, f64)>,
    exempt: usize,
    bows: Vec<Bow>,
}

fn bar_length(document: &En1993Snapshot, bar: &FrameBar) -> f64 {
    let at = |id: &str| document.frame_nodes.iter().find(|n| n.id == id).map_or((0.0, 0.0), |n| (n.x, n.z));
    let (a, b) = (at(&bar.start_node_id), at(&bar.end_node_id));
    (b.0 - a.0).hypot(b.1 - a.1)
}

fn curve_of(curve: part_1_1::BucklingCurve) -> (usize, &'static str) {
    match curve {
        part_1_1::BucklingCurve::A0 => (0, "a0"),
        part_1_1::BucklingCurve::A => (1, "a"),
        part_1_1::BucklingCurve::B => (2, "b"),
        part_1_1::BucklingCurve::C => (3, "c"),
        part_1_1::BucklingCurve::D => (4, "d"),
    }
}

fn derive(document: &En1993Snapshot, node_loads: &frame_eigen::FrameLoads, base: &frame_eigen::SecondOrder) -> Derivation {
    let frame_envelope::Sway { height, columns, phi, forces: sway, storeys } = frame_envelope::sway_imperfection(document, node_loads, base);
    let exempt = storeys.iter().filter(|s| s.exempt).count();
    let bows = document
        .frame_bars
        .iter()
        .filter(|b| !(b.release_start == semio_s_artifact_norm_en1993_model::FrameBarRelease::Pinned && b.release_end == semio_s_artifact_norm_en1993_model::FrameBarRelease::Pinned))
        .filter_map(|bar| {
            let member = document.members.iter().find(|m| m.id == bar.member_id)?;
            let (section, material) = (find_section(document, &member.section_id)?, find_material(document, &member.material_id)?);
            let n_ed = -base.bar(&bar.id)?.first.n;
            let length = bar_length(document, bar);
            let l_cr = if member.buckling_length_y > 0.0 { member.buckling_length_y } else { length };
            let lambda = part_1_1::lambda_bar(l_cr, part_1_1::radius_of_gyration(section.iy, section.area), material.e_modulus, material.fy);
            let (index, curve) = curve_of(part_1_1::buckling_curve_rolled_i(section, 'y', material.fy));
            (n_ed > 0.0 && length > 0.0 && global::bow_must_be_included(lambda, section.area, material.fy, n_ed)).then(|| Bow { bar: bar.id.clone(), length, curve, ratio: global::bow_ratio(index, false), n_ed })
        })
        .collect();
    Derivation { height, columns, phi, sway, exempt, bows }
}

fn imperfections(d: &Derivation, sway_sign: f64, bow_sign: f64, with_bow: bool) -> frame_eigen::Imperfections {
    frame_eigen::Imperfections {
        sway: d.sway.iter().map(|(n, f)| (n.clone(), sway_sign * f)).collect(),
        bow: if with_bow { d.bows.iter().map(|b| (b.bar.clone(), bow_sign * b.q())).collect() } else { Vec::new() },
    }
}

fn unfavourable(document: &En1993Snapshot, loads: &frame_eigen::FrameLoads, d: &Derivation, with_bow: bool) -> Result<frame_eigen::SecondOrder, frame_eigen::FrameFault> {
    let sways: Vec<f64> = if d.sway.is_empty() { vec![1.0] } else { vec![1.0, -1.0] };
    let bows: Vec<f64> = if with_bow && !d.bows.is_empty() { vec![1.0, -1.0] } else { vec![1.0] };
    let mut best: Option<frame_eigen::SecondOrder> = None;
    for sway in &sways {
        for bow in &bows {
            let result = joint_frame::solve_with_imperfections(document, loads, &imperfections(d, *sway, *bow, with_bow))?;
            if best.as_ref().is_none_or(|b| frame_envelope::peak_moment(&result) > frame_envelope::peak_moment(b)) {
                best = Some(result);
            }
        }
    }
    best.ok_or_else(|| frame_eigen::FrameFault { en: "No imperfection variant could be evaluated.".into(), de: "Keine Imperfektionsvariante konnte ausgewertet werden.".into() })
}

fn imperfection_text(d: &Derivation) -> (String, String) {
    let bows = d.bows.iter().map(|b| format!("{}: e0=L/{:.0}={:.1} mm (curve {})", b.bar, 1.0 / b.ratio, b.e0() * 1e3, b.curve)).collect::<Vec<_>>().join("; ");
    let (en_bows, de_bows) = if d.bows.is_empty() { ("no bow imperfection required (5.3.2(6))".to_string(), "keine Vorkrümmung erforderlich (5.3.2(6))".to_string()) } else { (format!("bow imperfections {bows}, equivalent load 8·N·e0/L² (formula unverified)"), format!("Vorkrümmungen {bows}, Ersatzlast 8·N·e0/L² (Formel ungeprüft)")) };
    if d.phi > 0.0 {
        (
            format!("sway imperfection φ={:.3e}=1/{:.0} (h={:.2} m, m={}, α_h and α_m of 5.3.2(3)) as horizontal forces φ·V at {} nodes in the unfavourable direction (ndp.en1993.1-1.phi0; {} storeys exempt by H_Ed ≥ 0.15 V_Ed, 5.3.2(4)), {en_bows}", d.phi, 1.0 / d.phi, d.height, d.columns, d.sway.len(), d.exempt),
            format!("Schiefstellung φ={:.3e}=1/{:.0} (h={:.2} m, m={}, α_h und α_m nach 5.3.2(3)) als Horizontalkräfte φ·V an {} Knoten in ungünstiger Richtung (ndp.en1993.1-1.phi0; {} Geschosse durch H_Ed ≥ 0,15 V_Ed ausgenommen, 5.3.2(4)), {de_bows}", d.phi, 1.0 / d.phi, d.height, d.columns, d.sway.len(), d.exempt),
        )
    } else {
        (format!("no compressed column: no sway imperfection, {en_bows}"), format!("keine gedrückte Stütze: keine Schiefstellung, {de_bows}"))
    }
}

fn member_buckling(document: &En1993Snapshot, bar: &FrameBar, params: AnnexParams, effects: frame_eigen::BarEffects) -> Option<(f64, String, String)> {
    use crate::clauses::stability as st;
    let member = document.members.iter().find(|m| m.id == bar.member_id)?;
    let kind = member_kind(member.member_type.as_str());
    let n_ed = -effects.n;
    if !kind_allows_member_interaction(&kind) || n_ed <= 0.0 {
        return None;
    }
    let (section, material) = (find_section(document, &member.section_id)?, find_material(document, &member.material_id)?);
    if part_1_1::hollow_route_open(section) {
        return None;
    }
    let class = part_1_1::section_class_rolled_i(section, material.fy);
    let a_res = if class == 4 { part_1_1::effective_area_class4(section, material.fy) } else { section.area };
    let w_y = part_1_1::bending_modulus_y(section, class, material.fy);
    let w_z = if class == 4 { section.w_el_z * (a_res / section.area.max(1e-12)).min(1.0) } else if class <= 2 { section.w_pl_z } else { section.w_el_z };
    let length = bar_length(document, bar);
    let (l_y, l_z) = (if member.buckling_length_y > 0.0 { member.buckling_length_y } else { length }, if member.buckling_length_z > 0.0 { member.buckling_length_z } else { length });
    let reduction = (a_res / section.area.max(1e-12)).min(1.0).sqrt();
    let lambda_y = part_1_1::lambda_bar(l_y, part_1_1::radius_of_gyration(section.iy, section.area), material.e_modulus, material.fy) * reduction;
    let lambda_z = part_1_1::lambda_bar(l_z, part_1_1::radius_of_gyration(section.iz, section.area), material.e_modulus, material.fy) * reduction;
    let chi_y = part_1_1::chi(lambda_y, part_1_1::buckling_curve_rolled_i(section, 'y', material.fy));
    let chi_z = part_1_1::chi(lambda_z, part_1_1::buckling_curve_rolled_i(section, 'z', material.fy));
    let m_cr = part_1_1::m_cr_rolled_nm(section, material, member.ltb_length.max(member.ltb_restraint_spacing), member.end_moment_ratio_psi, member.load_application.as_str(), member.moment_diagram.as_str());
    let chi_lt = if kind_allows_ltb(&kind) { part_1_1::chi_lt(part_1_1::lambda_lt(w_y, material.fy, m_cr), part_1_1::ltb_curve_table_6_4(section)) } else { 1.0 };
    let cm = match member.moment_diagram {
        semio_s_artifact_norm_en1993_model::MomentDiagram::Uniform => 1.0,
        semio_s_artifact_norm_en1993_model::MomentDiagram::Parabolic => st::cm_alpha_h(0.0, member.end_moment_ratio_psi, true),
        _ => st::cm_linear(member.end_moment_ratio_psi),
    };
    let r = st::interaction_annex_b(&st::InteractionInput {
        n_ed,
        my_ed: effects.m,
        mz_ed: 0.0,
        n_rk: a_res * material.fy,
        my_rk: w_y * material.fy,
        mz_rk: w_z * material.fy,
        chi_y,
        chi_z,
        chi_lt,
        lambda_y,
        lambda_z,
        cmy: cm,
        cmz: cm,
        cm_lt: cm,
        class,
        susceptible_torsion: kind_allows_ltb(&kind),
        rhs: false,
        gm1: params.gamma_m1,
        delta_my: if class == 4 { n_ed * part_1_1::e_n_y_class4(section, material.fy).abs() } else { 0.0 },
        delta_mz: 0.0,
    });
    let text = format!("N_Ed={:.1} kN, M_y,Ed={:.2} kNm, χ_y={chi_y:.3}, χ_z={chi_z:.3}, χ_LT={chi_lt:.3}, k_yy={:.3}, k_zy={:.3}, η(6.61)={:.3}, η(6.62)={:.3}, L_cr,y={l_y:.2} m, L_cr,z={l_z:.2} m", n_ed / 1e3, effects.m / 1e3, r.kyy, r.kzy, r.eta_61, r.eta_62);
    Some((r.eta_61.max(r.eta_62), text.clone(), text))
}

fn second_order_rows(document: &En1993Snapshot, annex: AnnexChoice, alpha: f64, scope: Scope<'_>, report: &mut CheckReport) {
    let params = AnnexParams::for_choice(annex);
    let fault_row = |fault: frame_eigen::FrameFault, report: &mut CheckReport| {
        let Some(bar) = document.frame_bars.first().filter(|b| scope.admits("frameBars", &b.id)) else { return };
        let reason = loc(&fault.en, &fault.de);
        finish(report, annex, format!("en1993.1-1.5.2.second-order.{}", bar.id), subject(bar, "memberId", "Steel member", "Stahlbauteil"), loc("Second-order internal forces of the frame", "Schnittgrößen des Rahmens nach Theorie II. Ordnung"), reason.clone(), |b| b.not_applicable(reason));
    };
    let sets = match frame_envelope::load_sets(document) {
        Ok(sets) => sets,
        Err(fault) => return fault_row(fault, report),
    };
    let mut governing: Option<(frame_envelope::LoadSet, Derivation, frame_eigen::SecondOrder)> = None;
    for set in sets {
        let base = match joint_frame::solve_with_imperfections(document, &set.loads, &frame_eigen::Imperfections::default()) {
            Ok(result) => result,
            Err(fault) => return fault_row(fault, report),
        };
        let derived = derive(document, &set.loads, &base);
        let full = match unfavourable(document, &set.loads, &derived, true) {
            Ok(result) => result,
            Err(fault) => return fault_row(fault, report),
        };
        if governing.as_ref().is_none_or(|(_, _, best)| frame_envelope::peak_moment(&full) > frame_envelope::peak_moment(best)) {
            governing = Some((set, derived, full));
        }
    }
    let Some((set, derived, full)) = governing else { return };
    let sway_only = if derived.bows.is_empty() { Ok(full.clone()) } else { unfavourable(document, &set.loads, &derived, false) };
    let (combination_en, combination_de) = if document.frame_actions.is_empty() { (String::new(), String::new()) } else { (format!(" under the governing EN 1990 combination '{}'", set.id), format!(" unter der maßgebenden Kombination '{}' nach EN 1990", set.id)) };
    let (imperfection_en, imperfection_de) = imperfection_text(&derived);
    for bar in document.frame_bars.iter().filter(|b| scope.admits("frameBars", &b.id)) {
        let Some(forces) = full.bar(&bar.id) else { continue };
        let Some(member) = document.members.iter().find(|m| m.id == bar.member_id) else { continue };
        let (Some(section), Some(material)) = (find_section(document, &member.section_id), find_material(document, &member.material_id)) else { continue };
        let ((eta_first, _), (eta, class)) = (interaction(section, material.fy, params, forces.first), interaction(section, material.fy, params, forces.second));
        let amplification = forces.moment_amplification();
        let estimate = global::amplification(alpha).map_or("-".to_string(), |k| format!("{k:.3}"));
        let state = if full.converged { format!("{} iterations", full.iterations) } else { format!("not converged after {} iterations", full.iterations) };
        let state_de = if full.converged { format!("{} Iterationen", full.iterations) } else { format!("nach {} Iterationen nicht konvergiert", full.iterations) };
        let explanation = loc(
            &format!("Method 5.2.2(3)c: second-order frame analysis{combination_en} of the kernel (P-Δ and P-δ, four elements per bar, {state}) with global and local imperfections ({imperfection_en}; unfavourable direction chosen by the sum of peak moments): N_Ed={:.1} kN, M_y,Ed={:.2} kNm (first order {:.2} kNm, amplification {amplification:.3}; 1/(1-1/α_cr)={estimate}), class {class}; section interaction η={eta:.3} (first order without imperfections {eta_first:.3}). Applies because α_cr={alpha:.2} is below the elastic limit {:.0}; the member buckling check of 6.3 on the same forces is the row 'member' of this bar.", forces.second.n / 1e3, forces.second.m / 1e3, forces.first.m / 1e3, global::alpha_cr_limit(false)),
            &format!("Verfahren 5.2.2(3)c: Rahmenberechnung nach Theorie II. Ordnung{combination_de} mit dem Kern (P-Δ und P-δ, vier Elemente je Stab, {state_de}) mit globalen und lokalen Imperfektionen ({imperfection_de}; ungünstige Richtung nach der Summe der Maximalmomente): N_Ed={:.1} kN, M_y,Ed={:.2} kNm (Theorie I. Ordnung {:.2} kNm, Vergrößerung {amplification:.3}; 1/(1-1/α_cr)={estimate}), Klasse {class}; Querschnittsinteraktion η={eta:.3} (Theorie I. Ordnung ohne Imperfektionen {eta_first:.3}). Gilt, weil α_cr={alpha:.2} unter der elastischen Grenze {:.0} liegt; der Stabilitätsnachweis nach 6.3 mit denselben Schnittgrößen ist die Zeile 'member' dieses Stabs.", forces.second.n / 1e3, forces.second.m / 1e3, forces.first.m / 1e3, global::alpha_cr_limit(false)),
        );
        finish(report, annex, format!("en1993.1-1.5.2.second-order.{}", bar.id), subject(bar, "memberId", "Steel member", "Stahlbauteil"), loc("Cross-section resistance under the second-order internal forces of the frame with imperfections", "Querschnittstragfähigkeit unter den Schnittgrößen des Rahmens nach Theorie II. Ordnung mit Imperfektionen"), explanation, |b| upsize(b.ndp(AnnexParams::GAMMA_M0).ndp("ndp.en1993.1-1.phi0").utilization(dimensionless(eta), dimensionless(1.0)), member, section, eta));
        let Ok(sway) = sway_only.as_ref() else { continue };
        let Some((eta_63, text, text_de)) = sway.bar(&bar.id).and_then(|f| member_buckling(document, bar, params, f.second)) else { continue };
        let explanation = loc(
            &format!("Method 5.2.2(3)b: second-order forces with the global imperfection only ({imperfection_en}); the member stability check 6.3.3 (Annex B interaction, equations 6.61 and 6.62) on the amplified forces with the member buckling lengths (non-sway): {text}."),
            &format!("Verfahren 5.2.2(3)b: Schnittgrößen nach Theorie II. Ordnung nur mit der globalen Imperfektion ({imperfection_de}); Stabilitätsnachweis 6.3.3 (Interaktion nach Anhang B, Gleichungen 6.61 und 6.62) mit den vergrößerten Schnittgrößen und den Knicklängen des Bauteils (unverschieblich): {text_de}."),
        );
        let subject = subject(bar, "memberId", "Steel member", "Stahlbauteil");
        let title = loc("Member buckling resistance under the second-order internal forces of the frame", "Bauteilstabilität unter den Schnittgrößen des Rahmens nach Theorie II. Ordnung");
        let base = CheckResult::assess(format!("en1993.1-1.5.2.member.{}", bar.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "6", "6.3.3"), subject, title).annex(annex).explanation(explanation);
        let (member, section) = (document.members.iter().find(|m| m.id == bar.member_id), document.members.iter().find(|m| m.id == bar.member_id).and_then(|m| find_section(document, &m.section_id)));
        let checked = base.ndp(AnnexParams::GAMMA_M1).ndp("ndp.en1993.1-1.phi0").utilization(dimensionless(eta_63), dimensionless(1.0));
        report.push(match (member, section) {
            (Some(member), Some(section)) => upsize(checked, member, section, eta_63),
            _ => checked,
        }.build());
    }
}

/// 🕸️ Appends one α_cr check of the frame model, anchored on the first bar of the document, and, when α_cr is below the elastic limit, the second-order checks of every bar with the imperfections of EN 1993-1-1 5.3.2: the cross-section rows (5.2.2(3)c) and the member stability rows of 6.3.3 (5.2.2(3)b).
pub fn check_frames(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    let Some(bar) = document.frame_bars.first().filter(|b| scope.admits("frameBars", &b.id)) else { return };
    match joint_frame::solve(document) {
        Ok(eigen) => {
            solved(annex, bar, &eigen, document, report);
            let alpha = eigen.governing();
            if alpha < global::alpha_cr_limit(false) {
                second_order_rows(document, annex, alpha, scope, report);
            }
        }
        Err(fault) => failed(annex, bar, fault, report),
    }
}
