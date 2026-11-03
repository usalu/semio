//! 🌉️ Bridge checks of EN 1998-2 on the pier and bearing rows: the spectrum analysis of the deck on the piers and bearings by the mechanics kernel, the modal mass participation, the flexure and the capacity-design shear of every pier, the displacement and the seat length of every bearing. See [`super`].

use super::*;
use semio_s_artifact_norm_en1998_schema::bridge::{self, Direction};
use semio_s_plugin_norm_compliance::ndp_number;

type Bridge = semio_s_artifact_norm_en1998_model::En1998Bridge;

const PARTICIPATION: &str = "ndp.en1998.en1.modal.min-mass-participation";
const GAMMA_0: &str = "ndp.en1998.en2.pier.gamma-0";
const GROUND_COEFFICIENT: &str = "ndp.en1998.en2.seat.ground-displacement-coefficient";
const GROUND_FACTOR: &str = "ndp.en1998.en2.seat.ground-displacement-factor";

/// 🌍️ The spectrum constants of the site: the elastic spectrum with the importance factor and the design spectrum for a behaviour factor.
pub struct Ground {
    pub a_g: f64,
    pub s: f64,
    pub tb: f64,
    pub tc: f64,
    pub td: f64,
    pub gamma_i: f64,
}

impl Ground {
    pub fn elastic(&self, t: f64) -> f64 {
        self.gamma_i * part_1::elastic_response_spectrum(self.a_g, self.s, self.tb, self.tc, self.td, t)
    }

    pub fn design(&self, q: f64, t: f64) -> f64 {
        part_1::design_spectrum_at(self.a_g, self.gamma_i, self.s, self.tb, self.tc, self.td, q, t)
    }
}

/// 🌊️ The two spectrum analyses of one bridge with pier or bearing rows.
pub struct Analysis {
    pub supports: Vec<bridge::Support>,
    pub longitudinal: Option<bridge::Response>,
    pub transverse: Option<bridge::Response>,
}

/// 🌊️ The longitudinal analysis (rigid deck) and the transverse analysis (deck stiffness `deckEiNm2`, rigid when 0) of the deck mass on the support lines of the rows; `None` without any support line.
pub fn analyse(document: &En1998Snapshot, b: &Bridge, ground: &Ground, q: f64, mass_kg: f64, annex: AnnexChoice) -> Option<Analysis> {
    let supports = bridge::supports(&b.id, &document.piers, &document.bearings, b.pier_cracked_inertia_ratio, &part_3::AnnexA::from_registry(annex));
    if supports.is_empty() {
        return None;
    }
    let xs: Vec<f64> = supports.iter().map(|s| s.x_m).collect();
    let run = |direction: Direction, ei: f64| {
        let stiffness: Vec<f64> = supports.iter().map(|s| s.stiffness_n_per_m(direction)).collect();
        bridge::analyse(direction, mass_kg, b.deck_length_m, ei, &xs, &stiffness, &|t| ground.design(q, t), &|t| ground.elastic(t))
    };
    Some(Analysis { longitudinal: run(Direction::Longitudinal, 0.0), transverse: run(Direction::Transverse, b.deck_ei_nm2), supports })
}

fn worst(analysis: &Analysis, pick: impl Fn(&bridge::Response, usize) -> f64, index: usize) -> (f64, Direction) {
    let l = analysis.longitudinal.as_ref().map_or(0.0, |r| pick(r, index));
    let t = analysis.transverse.as_ref().map_or(0.0, |r| pick(r, index));
    if t > l { (t, Direction::Transverse) } else { (l, Direction::Longitudinal) }
}

fn direction_name(direction: Direction) -> (&'static str, &'static str) {
    if direction == Direction::Longitudinal { ("longitudinal", "längs") } else { ("transverse", "quer") }
}

/// 🌉️ Mass participation, pier flexure and shear, bearing displacement and seat length of one bridge.
pub fn check_rows(report: &mut CheckReport, annex: AnnexChoice, document: &En1998Snapshot, b: &Bridge, analysis: &Analysis, ground: &Ground) {
    let bpath = id_sel("bridges", &b.id);
    let (min_participation, gamma_0, coefficient, factor) = (ndp_number(PARTICIPATION, annex), ndp_number(GAMMA_0, annex), ndp_number(GROUND_COEFFICIENT, annex), ndp_number(GROUND_FACTOR, annex));
    if let Some(t) = &analysis.transverse {
        let share = t.participation().min(1.0);
        let periods = t.periods_s.iter().map(|p| format!("{p:.3}")).collect::<Vec<_>>().join(", ");
        let c = CheckResult::assess(format!("en1998.2.{}.analysis.massParticipation", b.id), "DIN EN 1998-2", ClauseId::new("EN 1998-1", "1", "4.3.3.1"), SubjectRef::new(&b.id, format!("{bpath}.deckEiNm2"), lc(&b.id, &b.id)), lc("Effective modal mass of the transverse analysis", "Effektive Modalmasse der Querrichtungsanalyse"))
            .minimum(q_dim(share), q_dim(min_participation))
            .annex(annex)
            .ndp(PARTICIPATION)
            .explanation(lc(&format!("{} modes with T = {periods} s carry {:.1} % of the deck mass, at least {:.0} % are required", t.periods_s.len(), share * 100.0, min_participation * 100.0), &format!("{} Eigenformen mit T = {periods} s tragen {:.1} % der Überbaumasse, mindestens {:.0} % sind erforderlich", t.periods_s.len(), share * 100.0, min_participation * 100.0)));
        report.push(c.remedy(Remedy { applicable: false, ..Remedy::one_of(SubjectRef::new(&b.id, format!("{bpath}.deckEiNm2"), lc(&b.id, &b.id)), Vec::new(), lc("Change the deck flexural stiffness or the support stiffnesses so that the included modes carry the required share of the deck mass, or refine the model with more modes.", "Biegesteifigkeit des Überbaus oder Lagersteifigkeiten so ändern, dass die berücksichtigten Eigenformen den geforderten Anteil der Überbaumasse tragen, oder das Modell mit mehr Eigenformen verfeinern.")) }).build());
    }
    let d_g = bridge::ground_displacement_m(coefficient, ground.a_g * ground.gamma_i, ground.s, ground.tc, ground.td);
    for (index, support) in analysis.supports.iter().enumerate() {
        let (displacement, direction) = worst(analysis, |r, i| r.support_displacement_m[i].abs(), index);
        let (name_en, name_de) = direction_name(direction);
        if let Some(pier) = document.piers.iter().find(|p| p.id == support.pier_id).filter(|p| p.height_m > 0.0) {
            let ppath = id_sel("piers", &pier.id);
            let (force, force_direction) = worst(analysis, |r, i| r.support_force_n[i].abs(), index);
            let (force_en, force_de) = direction_name(force_direction);
            let m_ed = bridge::pier_base_moment_nm(force, pier.height_m);
            let mut flexure = CheckResult::assess(format!("en1998.2.{}.pier.{}.flexure", b.id, pier.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "5.5"), SubjectRef::new(&pier.id, format!("{ppath}.momentRdNm"), lc(&pier.id, &pier.id)), lc("Pier base moment M_Ed = F·H of the spectrum analysis", "Pfeilerfußmoment M_Ed = F·H der Antwortspektrenanalyse"))
                .utilization(q_mom(m_ed), floor_limit(q_mom(pier.moment_rd_nm)))
                .annex(annex)
                .explanation(lc(&format!("F = {force:.0} N ({force_en}) at H = {:.2} m gives M_Ed = {m_ed:.0} N·m against M_Rd = {:.0} N·m", pier.height_m, pier.moment_rd_nm), &format!("F = {force:.0} N ({force_de}) bei H = {:.2} m ergibt M_Ed = {m_ed:.0} N·m gegen M_Rd = {:.0} N·m", pier.height_m, pier.moment_rd_nm)));
            if m_ed > pier.moment_rd_nm {
                flexure = flexure.remedy(Remedy::at_least(SubjectRef::new(&pier.id, format!("{ppath}.momentRdNm"), lc("Plastic hinge moment M_Rd", "Plastisches Gelenkmoment M_Rd")), q_mom(pier.moment_rd_nm), q_mom(m_ed), lc(&format!("Increase the flexural reinforcement of the pier to M_Rd of at least {m_ed:.0} N·m."), &format!("Biegebewehrung des Pfeilers auf M_Rd von mindestens {m_ed:.0} N·m erhöhen."))));
            }
            report.push(flexure.build());
            let v_cd = part_2::pier_capacity_design_shear_n(gamma_0, pier.moment_rd_nm, pier.height_m);
            let mut shear = CheckResult::assess(format!("en1998.2.{}.pier.{}.shear", b.id, pier.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "5.6.3.3"), SubjectRef::new(&pier.id, format!("{ppath}.shearRdN"), lc(&pier.id, &pier.id)), lc("Pier capacity-design shear V_CD = γ₀·M_Rd/L_s", "Pfeiler-Kapazitätsquerkraft V_CD = γ₀·M_Rd/L_s"))
                .utilization(q_force(v_cd), floor_limit(q_force(pier.shear_rd_n)))
                .annex(annex)
                .ndp(GAMMA_0)
                .explanation(lc(&format!("V_CD = {gamma_0}·{:.0} N·m / {:.2} m = {v_cd:.0} N against V_Rd = {:.0} N", pier.moment_rd_nm, pier.height_m, pier.shear_rd_n), &format!("V_CD = {gamma_0}·{:.0} N·m / {:.2} m = {v_cd:.0} N gegen V_Rd = {:.0} N", pier.moment_rd_nm, pier.height_m, pier.shear_rd_n)));
            if v_cd > pier.shear_rd_n {
                shear = shear.remedy(Remedy::at_least(SubjectRef::new(&pier.id, format!("{ppath}.shearRdN"), lc("Pier shear resistance V_Rd", "Pfeilerquerkrafttragfähigkeit V_Rd")), q_force(pier.shear_rd_n), q_force(v_cd), lc("Increase the shear reinforcement of the pier.", "Querkraftbewehrung des Pfeilers erhöhen.")));
            }
            report.push(shear.build());
        }
        for bearing in support.bearing_ids.iter().filter_map(|id| document.bearings.iter().find(|k| &k.id == id)) {
            let kpath = id_sel("bearings", &bearing.id);
            let d_b = bridge::bearing_displacement_m(support, direction, displacement);
            let mut travel = CheckResult::assess(format!("en1998.2.{}.bearing.{}.displacement", b.id, bearing.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "7.5"), SubjectRef::new(&bearing.id, format!("{kpath}.dRdM"), lc(&bearing.id, &bearing.id)), lc("Bearing displacement against its capacity", "Lagerverschiebung gegen die Kapazität"))
                .utilization(q_len(d_b), floor_limit(q_len(bearing.d_rd_m)))
                .annex(annex)
                .explanation(lc(&format!("deck displacement {displacement:.4} m ({name_en}), bearing share {:.2}, d_Ed = {d_b:.4} m against d_Rd = {:.4} m (elastic spectrum with γ_I)", support.bearing_share(direction), bearing.d_rd_m), &format!("Überbauverschiebung {displacement:.4} m ({name_de}), Lageranteil {:.2}, d_Ed = {d_b:.4} m gegen d_Rd = {:.4} m (elastisches Spektrum mit γ_I)", support.bearing_share(direction), bearing.d_rd_m)));
            if d_b > bearing.d_rd_m {
                travel = travel.remedy(Remedy::at_least(SubjectRef::new(&bearing.id, format!("{kpath}.dRdM"), lc("Displacement capacity d_Rd", "Verschiebungskapazität d_Rd")), q_len(bearing.d_rd_m), q_len(d_b), lc(&format!("Choose a bearing with a displacement capacity of at least {d_b:.4} m."), &format!("Ein Lager mit einer Verschiebungskapazität von mindestens {d_b:.4} m wählen."))));
            }
            report.push(travel.build());
            let d_es = if support.is_pier() { d_b } else { displacement };
            let l_ov = bridge::seat_length_m(bearing.min_support_length_m, factor, d_g, d_es);
            let mut seat = CheckResult::assess(format!("en1998.2.{}.bearing.{}.seat", b.id, bearing.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "6.6.2"), SubjectRef::new(&bearing.id, format!("{kpath}.seatLengthM"), lc(&bearing.id, &bearing.id)), lc("Seat length l_ov = l_m + d_eg + d_Es", "Auflagerlänge l_ov = l_m + d_eg + d_Es"))
                .utilization(q_len(l_ov), floor_limit(q_len(bearing.seat_length_m)))
                .annex(annex)
                .ndps(&[GROUND_COEFFICIENT, GROUND_FACTOR])
                .explanation(lc(&format!("l_m = {:.3} m, d_g = {d_g:.4} m, d_eg = {factor}·d_g = {:.4} m, d_Es = {d_es:.4} m give l_ov = {l_ov:.4} m against {:.3} m", bearing.min_support_length_m, factor * d_g, bearing.seat_length_m), &format!("l_m = {:.3} m, d_g = {d_g:.4} m, d_eg = {factor}·d_g = {:.4} m, d_Es = {d_es:.4} m ergeben l_ov = {l_ov:.4} m gegen {:.3} m", bearing.min_support_length_m, factor * d_g, bearing.seat_length_m)));
            if l_ov > bearing.seat_length_m {
                seat = seat.remedy(Remedy::at_least(SubjectRef::new(&bearing.id, format!("{kpath}.seatLengthM"), lc("Seat length", "Auflagerlänge")), q_len(bearing.seat_length_m), q_len(l_ov), lc(&format!("Widen the seat to at least {l_ov:.3} m."), &format!("Das Auflager auf mindestens {l_ov:.3} m verbreitern."))));
            }
            report.push(seat.build());
        }
    }
}
