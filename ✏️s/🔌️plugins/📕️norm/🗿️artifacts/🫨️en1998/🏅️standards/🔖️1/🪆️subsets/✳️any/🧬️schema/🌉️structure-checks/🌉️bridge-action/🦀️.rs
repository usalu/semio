//! 🌉️ Bridge seismic action checks of EN 1998-2: the importance class and the design ground acceleration (2.1), the cracked stiffness of the piers (2.3.6.1), the behaviour factor of the RC piers from the shear span (4.1.6) and the combination of the seismic components on the pier base moment (4.1.8). See [`super`].

use super::*;
use compliance::{ndp_number, CheckStatus};
use schema::bridge::{self, Direction};

use model::PierStiffnessBasis;

type Bridge = model::En1998Bridge;

const GAMMA_I: [&str; 3] = ["ndp.en1998.en2.gamma-i.i", "ndp.en1998.en2.gamma-i.ii", "ndp.en1998.en2.gamma-i.iii"];
const Q_RC: &str = "ndp.en1998.en2.q.rc-pier";
const SPAN_FULL: &str = "ndp.en1998.en2.q.shear-span-full";
const SPAN_MIN: &str = "ndp.en1998.en2.q.shear-span-min";
const MINOR: &str = "ndp.en1998.en2.combination.minor-factor";

/// 🌉️ Importance class check (class 1 to 3) with the design ground acceleration `a_g = γ_I a_gR`.
pub fn check_action(report: &mut CheckReport, annex: AnnexChoice, b: &Bridge, a_gr: f64) {
    let path = id_sel("bridges", &b.id);
    let class = b.importance_class.round().clamp(1.0, 3.0);
    let gamma_i = part_2::bridge_gamma_i(b.importance_class);
    let a_g = gamma_i * a_gr;
    let mut importance = CheckResult::assess(format!("en1998.2.{}.action.importance", b.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "2.1"), SubjectRef::new(&b.id, format!("{path}.importanceClass"), lc(&b.id, &b.id)), lc("Importance class and design ground acceleration a_g = γ_I a_gR", "Bedeutungsklasse und Bemessungs-Bodenbeschleunigung a_g = γ_I a_gR"))
        .minimum(q_dim(b.importance_class), q_dim(1.0))
        .annex(annex)
        .ndp(GAMMA_I[class as usize - 1])
        .explanation(lc(&format!("class {class:.0}: γ_I = {gamma_i}, a_g = {gamma_i}·{a_gr:.3} = {a_g:.3} m/s²"), &format!("Klasse {class:.0}: γ_I = {gamma_i}, a_g = {gamma_i}·{a_gr:.3} = {a_g:.3} m/s²")));
    if b.importance_class < 1.0 {
        importance = importance.remedy(Remedy::at_least(SubjectRef::new(&b.id, format!("{path}.importanceClass"), lc("Importance class", "Bedeutungsklasse")), q_dim(b.importance_class), q_dim(1.0), lc("Choose importance class 1, 2 or 3.", "Bedeutungsklasse 1, 2 oder 3 wählen.")));
    }
    report.push(importance.build());
}

/// 🌉️ Behaviour factor and component combination of every pier row: `q = 3.5 λ(α_s)` with `α_s = L_s/h` against the applied `q`, and `M_Ed = max(M_x + f M_y, f M_x + M_y)` of the longitudinal and transverse base moments against `M_Rd`.
pub fn check_piers(report: &mut CheckReport, annex: AnnexChoice, document: &En1998Snapshot, b: &Bridge, analysis: &super::Analysis) {
    let (q0, full, min, minor) = (ndp_number(Q_RC, annex), ndp_number(SPAN_FULL, annex), ndp_number(SPAN_MIN, annex), ndp_number(MINOR, annex));
    let q_applied = part_2::design_behaviour_factor(b.period_ratio);
    if document.piers.iter().any(|p| p.bridge_id == b.id && p.stiffness_basis == PierStiffnessBasis::Declared) {
        let r = b.pier_cracked_inertia_ratio;
        let bridge_subject = SubjectRef::new(&b.id, format!("{}.pierCrackedInertiaRatio", id_sel("bridges", &b.id)), lc("Cracked share I_eff / I_g of the piers", "Gerissener Anteil I_eff / I_g der Pfeiler"));
        let gross = if r == 1.0 { (" The gross section is declared, which overestimates the stiffness and underestimates the displacements unless justified.", " Es ist der Bruttoquerschnitt angegeben, der die Steifigkeit überschätzt und die Verschiebungen unterschätzt, sofern nicht begründet.") } else { ("", "") };
        let mut stiffness = CheckResult::assess(format!("en1998.2.{}.action.stiffness", b.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "2.3.6.1"), bridge_subject.clone(), lc("Declared cracked stiffness share I_eff / I_g of the piers", "Angegebener Anteil I_eff / I_g der gerissenen Pfeilersteifigkeit"))
            .minimum(q_dim(r), q_dim(f64::EPSILON))
            .annex(annex)
            .explanation(lc(&format!("I_eff / I_g = {r} is entered by the designer and scales the pier stiffness 3 E I_eff / H³ of the analysis (0 < share <= 1).{}", gross.0), &format!("I_eff / I_g = {r} wird vom Entwerfer angegeben und skaliert die Pfeilersteifigkeit 3 E I_eff / H³ der Berechnung (0 < Anteil <= 1).{}", gross.1)));
        if r > 1.0 {
            stiffness = stiffness.status(CheckStatus::Fail).remedy(Remedy::at_most(bridge_subject, q_dim(r), q_dim(1.0), lc("Declare a cracked share of at most 1.", "Gerissenen Anteil von höchstens 1 angeben.")));
        } else if r <= 0.0 {
            stiffness = stiffness.remedy(Remedy::at_least(bridge_subject, q_dim(r), q_dim(f64::EPSILON), lc("Declare the cracked share I_eff / I_g from the cracked section analysis (0 < share <= 1).", "Gerissenen Anteil I_eff / I_g aus der Berechnung des gerissenen Querschnitts angeben (0 < Anteil <= 1).")));
        }
        report.push(stiffness.build());
    }
    let section = part_3::AnnexA::from_registry(annex);
    for pier in document.piers.iter().filter(|p| p.bridge_id == b.id && p.stiffness_basis == PierStiffnessBasis::Computed) {
        let ppath = id_sel("piers", &pier.id);
        let subject = SubjectRef::new(&pier.id, format!("{ppath}.reinforcementRatio"), lc("Reinforcement of the pier section", "Bewehrung des Pfeilerquerschnitts"));
        let (longitudinal, transverse) = (bridge::pier_ei_ratio(pier, Direction::Longitudinal, 0.0, &section), bridge::pier_ei_ratio(pier, Direction::Transverse, 0.0, &section));
        let ratio = longitudinal.min(transverse);
        let ei = bridge::pier_section(pier, Direction::Longitudinal).effective_stiffness_nm2(&section).unwrap_or(0.0);
        let mut computed = CheckResult::assess(format!("en1998.2.{}.pier.{}.stiffness", b.id, pier.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "2.3.6.1"), subject.clone(), lc("Computed effective stiffness EI_eff = M_y/φ_y of the pier section", "Berechnete effektive Steifigkeit EI_eff = M_y/φ_y des Pfeilerquerschnitts"))
            .minimum(q_dim(ratio), q_dim(f64::EPSILON))
            .annex(annex)
            .explanation(lc(&format!("EI_eff = M_y/φ_y of the fibre section (Annex C) at N = {:.0} N: longitudinal EI_eff = {ei:.3e} N·m², EI_eff/EI_g = {longitudinal:.3} longitudinal, {transverse:.3} transverse.", pier.axial_load_n), &format!("EI_eff = M_y/φ_y des Fasermodells (Anhang C) bei N = {:.0} N: längs EI_eff = {ei:.3e} N·m², EI_eff/EI_g = {longitudinal:.3} längs, {transverse:.3} quer.", pier.axial_load_n)));
        if ratio <= 0.0 {
            computed = computed.remedy(Remedy::at_least(subject, q_dim(ratio), q_dim(f64::EPSILON), lc("Enter reinforcement ratio, cover, f_ck and f_yk of the pier (concrete up to the registry strength range) and an axial force the section can carry.", "Bewehrungsgrad, Betondeckung, f_ck und f_yk des Pfeilers (Beton bis zum Festigkeitsbereich der Registry) und eine vom Querschnitt aufnehmbare Normalkraft angeben.")));
        }
        report.push(computed.build());
    }
    for (index, support) in analysis.supports.iter().enumerate() {
        let Some(pier) = document.piers.iter().find(|p| p.id == support.pier_id).filter(|p| p.height_m > 0.0) else { continue };
        let ppath = id_sel("piers", &pier.id);
        let depth = pier.section_b_m.max(pier.section_h_m);
        let alpha_s = part_2::shear_span_ratio(pier.height_m, depth);
        let q_max = part_2::rc_pier_behaviour_factor(q0, alpha_s, full, min);
        let subject = SubjectRef::new(&pier.id, format!("{ppath}.heightM"), lc(&pier.id, &pier.id));
        let title = lc("Pier behaviour factor q = 3.5·λ(α_s) from the shear span", "Pfeiler-Verhaltensbeiwert q = 3,5·λ(α_s) aus der Schubschlankheit");
        let id = format!("en1998.2.{}.pier.{}.behaviour", b.id, pier.id);
        let clause = ClauseId::new("EN 1998-2", "2", "4.1.6");
        let behaviour = match q_max {
            Some(q_max) => CheckResult::assess(id, "DIN EN 1998-2", clause, subject, title)
                .utilization(q_dim(q_applied), q_dim(q_max))
                .annex(annex)
                .ndps(&[Q_RC, SPAN_FULL, SPAN_MIN])
                .explanation(lc(&format!("α_s = L_s/h = {:.2}/{depth:.2} = {alpha_s:.2} gives q_max = {q_max:.3}; the analysis uses q = {q_applied}", pier.height_m), &format!("α_s = L_s/h = {:.2}/{depth:.2} = {alpha_s:.2} ergibt q_max = {q_max:.3}; die Berechnung verwendet q = {q_applied}", pier.height_m))),
            None => CheckResult::assess(id, "DIN EN 1998-2", clause, subject.clone(), title)
                .minimum(q_dim(alpha_s), q_dim(min))
                .status(CheckStatus::Fail)
                .annex(annex)
                .ndps(&[SPAN_MIN])
                .explanation(lc(&format!("α_s = {alpha_s:.2} is below {min}: the pier is squat, outside the rule for ductile RC piers"), &format!("α_s = {alpha_s:.2} liegt unter {min}: der Pfeiler ist gedrungen, außerhalb der Regel für duktile Stahlbetonpfeiler")))
                .remedy(Remedy::at_least(subject, q_len(pier.height_m), q_len(min * depth), lc(&format!("Increase the pier height to at least {:.3} m or reduce its section.", min * depth), &format!("Pfeilerhöhe auf mindestens {:.3} m vergrößern oder den Querschnitt verkleinern.", min * depth)))),
        };
        report.push(behaviour.build());
        let moment = |direction: Direction| {
            let r = if direction == Direction::Longitudinal { &analysis.longitudinal } else { &analysis.transverse };
            r.as_ref().map_or(0.0, |r| bridge::pier_base_moment_nm(r.support_force_n[index], pier.height_m))
        };
        let (m_x, m_y) = (moment(Direction::Longitudinal), moment(Direction::Transverse));
        let m_ed = part_2::component_combination(m_x, m_y, 0.0, minor);
        let mut combined = CheckResult::assess(format!("en1998.2.{}.pier.{}.combination", b.id, pier.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "4.1.8"), SubjectRef::new(&pier.id, format!("{ppath}.momentRdNm"), lc(&pier.id, &pier.id)), lc("Pier base moment of the combined components E_x + 0.3 E_y", "Pfeilerfußmoment der kombinierten Komponenten E_x + 0,3 E_y"))
            .utilization(q_mom(m_ed), floor_limit(q_mom(pier.moment_rd_nm)))
            .annex(annex)
            .ndp(MINOR)
            .explanation(lc(&format!("M_x = {m_x:.0} N·m, M_y = {m_y:.0} N·m give M_Ed = max(M_x + {minor} M_y, {minor} M_x + M_y) = {m_ed:.0} N·m against M_Rd = {:.0} N·m", pier.moment_rd_nm), &format!("M_x = {m_x:.0} N·m, M_y = {m_y:.0} N·m ergeben M_Ed = max(M_x + {minor} M_y, {minor} M_x + M_y) = {m_ed:.0} N·m gegen M_Rd = {:.0} N·m", pier.moment_rd_nm)));
        if m_ed > pier.moment_rd_nm {
            combined = combined.remedy(Remedy::at_least(SubjectRef::new(&pier.id, format!("{ppath}.momentRdNm"), lc("Plastic hinge moment M_Rd", "Plastisches Gelenkmoment M_Rd")), q_mom(pier.moment_rd_nm), q_mom(m_ed), lc(&format!("Increase M_Rd to at least {m_ed:.0} N·m."), &format!("M_Rd auf mindestens {m_ed:.0} N·m erhöhen."))));
        }
        report.push(combined.build());
    }
}
