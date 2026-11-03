//! 🏛️ Annex block checks: ring-stiffened shells under external pressure (EN 1993-1-6), stiffened plate buckling (EN 1993-1-5 Annex A) with the finite element cross-check of Annex C, corrugated web shear (EN 1993-1-5 Annex D), orthotropic deck minima and wheel fatigue (EN 1993-2 9.4, 9.5) and purlins restrained by sheeting (EN 1993-1-3). See [EN 1993](https://eurocodes.jrc.ec.europa.eu/).

use super::special_parts_checks::{bilingual, cap, emit, grow, lift, shrink, solved, subj, Case};
use super::*;
use crate::clauses::{bridge_fatigue, cold_formed, shell_stability};
use semio_s_artifact_norm_en1993_clauses_annex::{corrugated_web, orthotropic_deck, purlin_free_flange, shell_pressure, stiffened_plate};
use semio_s_artifact_norm_en1993_model::{BridgeFatigue, ColdFormedMember, PlatedPanel, SiloShell};

fn e_mpa() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.e-modulus")
}
fn nu() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.poisson")
}
const RING_FACTOR: f64 = 3.0;

fn external_pressure(annex: AnnexChoice, params: AnnexParams, s: &SiloShell, report: &mut CheckReport) {
    let Some(class) = s.fabrication_class.as_str().chars().next() else { return };
    let l_ring = if s.ring_spacing > 0.0 { s.ring_spacing } else { s.length };
    if !(s.external_pressure > 0.0 && s.radius > 0.0 && s.thickness > 0.0 && s.fy > 0.0 && l_ring > 0.0) {
        return;
    }
    let (r, t, fy, gm1) = (s.radius * 1e3, s.thickness * 1e3, s.fy / 1e6, params.gamma_m1);
    let (p, l) = (s.external_pressure / 1e6, l_ring * 1e3);
    let c_theta = if s.boundary_factor_hoop > 0.0 { s.boundary_factor_hoop } else { 1.0 };
    let subject = |field: &str, en: &str, de: &str| subj(&s.id, "siloShells", field, en, de);
    let hoop = |th: f64, len: f64| shell_stability::hoop_design(len, r, th, fy, class, c_theta, gm1);
    let sigma_ed = shell_pressure::hoop_stress(p, r, t);
    let d = hoop(t, l);
    let p_vm = shell_pressure::von_mises_p_cr(e_mpa(), nu(), t, 2.0 * r, l);
    let case = Case {
        id: format!("en1993.1-6.D.ext.{}", s.id),
        part: "DIN EN 1993-1-6",
        family: "EN 1993-1-6",
        chapter: "8",
        section: "8.5.2",
        subject: subject("externalPressure", "External pressure", "Außendruck"),
        title: loc("Circumferential buckling under external pressure (LS3)", "Umfangsbeulen unter Außendruck (LS3)"),
        demand: stress_pa(sigma_ed * 1e6),
        capacity: stress_pa(d.sigma_rd * 1e6),
        explanation: bilingual(
            format!("σ_θ,Ed=p r/t={sigma_ed:.1} MPa against σ_θ,Rd={:.1} MPa over l={l:.0} mm (σ_θ,Rcr={:.1} MPa, λ̄={:.3}, χ={:.3}, C_θ={c_theta:.2}, class {class}); the classical von Mises collapse pressure is {p_vm:.3} MPa against p={p:.3} MPa.", d.sigma_rd, d.sigma_cr, d.lambda, d.chi),
            format!("σ_θ,Ed=p r/t={sigma_ed:.1} MPa gegen σ_θ,Rd={:.1} MPa über l={l:.0} mm (σ_θ,Rcr={:.1} MPa, λ̄={:.3}, χ={:.3}, C_θ={c_theta:.2}, Klasse {class}); der klassische von-Mises-Beuldruck beträgt {p_vm:.3} MPa gegenüber p={p:.3} MPa.", d.sigma_rd, d.sigma_cr, d.lambda, d.chi),
        ),
        ndps: &["ndp.en1993.v-801.2"],
        gammas: &[AnnexParams::GAMMA_M1],
    };
    emit(report, annex, case, || {
        let th = grow(t, None, |x| hoop(x, l).sigma_rd >= shell_pressure::hoop_stress(p, r, x));
        let spacing = shrink(l, None, |x| hoop(t, x).sigma_rd >= sigma_ed);
        solved([
            th.map(|th| lift(subject("thickness", "Wall thickness", "Wanddicke"), length_m(s.thickness), length_m(th / 1e3), &format!("Increase the wall thickness to at least {th:.1} mm."), &format!("Wanddicke auf mindestens {th:.1} mm erhöhen."))),
            spacing.map(|spacing| cap(subject("ringSpacing", "Ring stiffener spacing", "Ringsteifenabstand"), length_m(l_ring), length_m(spacing / 1e3), &format!("Or space the ring stiffeners at most {:.2} m apart.", spacing / 1e3), &format!("Oder Ringsteifen höchstens {:.2} m auseinander anordnen.", spacing / 1e3))),
        ])
    });
    if s.ring_spacing > 0.0 && s.ring_area > 0.0 && s.ring_inertia > 0.0 {
        let ring = shell_pressure::ring_section(r, t, s.ring_area * 1e6, s.ring_inertia * 1e12, s.ring_offset * 1e3);
        let p_cr = shell_pressure::ring_p_cr(e_mpa(), ring.inertia, ring.radius, l);
        let case = Case {
            id: format!("en1993.1-6.D.ring.{}", s.id),
            part: "DIN EN 1993-1-6",
            family: "EN 1993-1-6",
            chapter: "8",
            section: "8.5.2",
            subject: subject("ringInertia", "Ring stiffener inertia", "Ringsteifenträgheit"),
            title: loc("Ring stiffener general instability", "Allgemeines Ringsteifenbeulen"),
            demand: stress_pa(RING_FACTOR * p * 1e6),
            capacity: stress_pa(p_cr * 1e6),
            explanation: bilingual(
                format!("Ring with l_e={:.0} mm of shell: I={:.3e} mm⁴, centroid radius {:.0} mm, p_cr=3 E I/(R³ l_r)={p_cr:.3} MPa against {RING_FACTOR:.1} p={:.3} MPa (engineering factor, unverified).", shell_pressure::effective_shell_width(r, t), ring.inertia, ring.radius, RING_FACTOR * p),
                format!("Ring mit l_e={:.0} mm Schale: I={:.3e} mm⁴, Schwerpunktradius {:.0} mm, p_cr=3 E I/(R³ l_r)={p_cr:.3} MPa gegen {RING_FACTOR:.1} p={:.3} MPa (Ingenieurfaktor, ungeprüft).", shell_pressure::effective_shell_width(r, t), ring.inertia, ring.radius, RING_FACTOR * p),
            ),
            ndps: &["ndp.en1993.v-806.1"],
            gammas: &[],
        };
        emit(report, annex, case, || {
            let needed = shell_pressure::ring_inertia_required(e_mpa(), p, RING_FACTOR, ring.radius, l);
            let inertia = s.ring_inertia * 1e12 + (needed - ring.inertia);
            vec![lift(subject("ringInertia", "Ring stiffener inertia", "Ringsteifenträgheit"), Quantity::new(QuantityKind::Dimensionless, s.ring_inertia), Quantity::new(QuantityKind::Dimensionless, inertia / 1e12), &format!("Increase the ring second moment to at least {:.3e} m⁴.", inertia / 1e12), &format!("Ringträgheitsmoment auf mindestens {:.3e} m⁴ erhöhen.", inertia / 1e12))]
        });
    }
}

struct Critical {
    sigma_cr: f64,
    k_sigma: f64,
    sigma_e: f64,
    kind: &'static str,
    detail_en: String,
    detail_de: String,
}

fn stiffened_critical(p: &PlatedPanel, count: usize, t: f64) -> Option<Critical> {
    let set = stiffened_plate::Stiffened { b: p.b * 1e3, t, count, area: p.stiffener_area * 1e6, inertia: p.stiffener_inertia * 1e12, offset: p.stiffener_eccentricity * 1e3 };
    let (a, e, v) = (p.a * 1e3, e_mpa(), nu());
    match count {
        1 => {
            let r = stiffened_plate::single(&set, a, 0.0, e, v);
            let (en, de) = (
                format!("one stiffener (A.2.2.1): a_c={:.0} mm, σ_cr,sl={:.1} MPa, sub-panel {:.1} MPa", r.a_c, r.sigma_cr_sl, r.sigma_cr_sub),
                format!("eine Steife (A.2.2.1): a_c={:.0} mm, σ_cr,sl={:.1} MPa, Teilfeld {:.1} MPa", r.a_c, r.sigma_cr_sl, r.sigma_cr_sub),
            );
            Some(Critical { sigma_cr: r.sigma_cr_p, k_sigma: r.k_sigma_p, sigma_e: r.sigma_e, kind: "ndp.en1993.v-376.a2", detail_en: en, detail_de: de })
        }
        c if c >= 3 => {
            let r = stiffened_plate::smeared(&set, a, 1.0, e, v);
            let (en, de) = (
                format!("{c} smeared stiffeners (A.1, ψ=1): α={:.2}, γ={:.2}, δ={:.3}", r.alpha, r.gamma, r.delta),
                format!("{c} verschmierte Steifen (A.1, ψ=1): α={:.2}, γ={:.2}, δ={:.3}", r.alpha, r.gamma, r.delta),
            );
            Some(Critical { sigma_cr: r.sigma_cr_p, k_sigma: r.k_sigma_p, sigma_e: r.sigma_e, kind: "ndp.en1993.v-376.a1", detail_en: en, detail_de: de })
        }
        _ => None,
    }
}

fn stiffened_resistance(sigma_cr: f64, fy: f64, gm1: f64) -> f64 {
    let lambda = (fy / sigma_cr.max(1e-9)).sqrt();
    part_1_5::plate_reduction_factor(lambda) * fy / gm1
}

fn stiffened_plate_check(annex: AnnexChoice, params: AnnexParams, p: &PlatedPanel, report: &mut CheckReport) {
    let count = stiffened_plate::count(p.b * 1e3, p.stiffener_spacing * 1e3);
    if count == 0 || !(p.a > 0.0 && p.b > 0.0 && p.thickness > 0.0 && p.fy > 0.0 && p.compression_stress > 0.0 && p.stiffener_area > 0.0 && p.stiffener_inertia > 0.0) {
        return;
    }
    let subject = |field: &str, en: &str, de: &str| subj(&p.id, "platedPanels", field, en, de);
    let (t, fy, sigma_ed, gm1) = (p.thickness * 1e3, p.fy / 1e6, p.compression_stress / 1e6, params.gamma_m1);
    let Some(c) = stiffened_critical(p, count, t) else {
        let reason = loc("Two longitudinal stiffeners (A.2.2.2) are not encoded: use one stiffener, three or more, or the finite element α_cr of Annex C.", "Zwei Längssteifen (A.2.2.2) sind nicht hinterlegt: eine Steife, drei oder mehr oder den FE-α_cr nach Anhang C verwenden.");
        report.push(CheckResult::assess(format!("en1993.1-5.A.{}", p.id), "DIN EN 1993-1-5", ClauseId::new("EN 1993-1-5", "Annex A", "A.2.2.2"), subject("stiffenerSpacing", "Stiffener spacing", "Steifenabstand"), loc("Critical stress of the stiffened plate", "Kritische Spannung der ausgesteiften Platte")).annex(annex).not_applicable(reason).build());
        return;
    };
    let sigma_rd = stiffened_resistance(c.sigma_cr, fy, gm1);
    let lambda = (fy / c.sigma_cr).sqrt();
    let case = Case {
        id: format!("en1993.1-5.A.{}", p.id),
        part: "DIN EN 1993-1-5",
        family: "EN 1993-1-5",
        chapter: "Annex A",
        section: if count == 1 { "A.2.2.1" } else { "A.1" },
        subject: subject("stiffenerSpacing", "Stiffener spacing", "Steifenabstand"),
        title: loc("Buckling of the longitudinally stiffened plate", "Beulen der längsausgesteiften Platte"),
        demand: stress_pa(sigma_ed * 1e6),
        capacity: stress_pa(sigma_rd * 1e6),
        explanation: bilingual(
            format!("{}; σ_E={:.2} MPa, k_σ,p={:.2}, σ_cr,p={:.1} MPa, λ̄_p={lambda:.3}, ρ={:.3}; σ_Ed={sigma_ed:.1} MPa against ρ f_y/γ_M1={sigma_rd:.1} MPa (uniform compression ψ=1; formula unverified: the spec V-376 (en1993-target.md line 277) names Annex A but gives no formulas, so k_σ,p and σ_cr,sl are from memory; the stiffener properties are used as entered: gross, with the adjacent plate parts as the owner defines them).", c.detail_en, c.sigma_e, c.k_sigma, c.sigma_cr, sigma_rd * gm1 / fy),
            format!("{}; σ_E={:.2} MPa, k_σ,p={:.2}, σ_cr,p={:.1} MPa, λ̄_p={lambda:.3}, ρ={:.3}; σ_Ed={sigma_ed:.1} MPa gegen ρ f_y/γ_M1={sigma_rd:.1} MPa (gleichmäßiger Druck ψ=1; Formel ungeprüft: die Spezifikation V-376 (en1993-target.md Zeile 277) nennt Anhang A ohne Formeln, k_σ,p und σ_cr,sl stammen aus dem Gedächtnis; die Steifeneigenschaften gelten wie eingegeben: brutto, mit den angrenzenden Blechteilen wie vom Anwender festgelegt).", c.detail_de, c.sigma_e, c.k_sigma, c.sigma_cr, sigma_rd * gm1 / fy),
        ),
        ndps: &[c.kind],
        gammas: &[AnnexParams::GAMMA_M1],
    };
    emit(report, annex, case, || {
        let more = (count + 1..=count + 8).find(|&n| stiffened_critical(p, n, t).is_some_and(|k| stiffened_resistance(k.sigma_cr, fy, gm1) >= sigma_ed));
        let th = grow(t, None, |x| stiffened_critical(p, count, x).is_some_and(|k| stiffened_resistance(k.sigma_cr, fy, gm1) >= sigma_ed));
        let mut list = Vec::new();
        if let Some(n) = more {
            let pitch = stiffened_plate::pitch(p.b * 1e3, n) / 1e3;
            list.push(cap(subject("stiffenerSpacing", "Stiffener spacing", "Steifenabstand"), length_m(p.stiffener_spacing), length_m(pitch), &format!("Space the stiffeners at most {pitch:.3} m apart ({n} stiffeners)."), &format!("Steifenabstand höchstens {pitch:.3} m wählen ({n} Steifen).")));
        }
        if let Some(th) = th {
            list.push(lift(subject("thickness", "Plate thickness", "Blechdicke"), length_m(p.thickness), length_m(th / 1e3), &format!("Or increase the plate thickness to at least {th:.1} mm."), &format!("Oder die Blechdicke auf mindestens {th:.1} mm erhöhen.")));
        }
        list
    });
    if p.alpha_cr > 0.0 {
        let analytic = c.sigma_cr / sigma_ed;
        let ratio = p.alpha_cr / analytic;
        report.push(
            CheckResult::assess(format!("en1993.1-5.C.{}", p.id), "DIN EN 1993-1-5", ClauseId::new("EN 1993-1-5", "Annex C", "C.1"), subject("alphaCr", "Finite element load factor α_cr", "FE-Laststeigerungsfaktor α_cr"), loc("Finite element α_cr against the Annex A critical stress", "FE-α_cr gegenüber der kritischen Spannung nach Anhang A"))
                .annex(annex)
                .ndp("ndp.en1993.v-376.c1")
                .informative(dimensionless(ratio))
                .explanation(bilingual(
                    format!("α_cr from the finite element model {:.2} against σ_cr,p/σ_Ed={analytic:.2} of Annex A: ratio {ratio:.3}. Formula unverified (spec V-376 gives no Annex A or C formulas). A ratio far from 1 means the model or the Annex A idealisation (uniform compression, stiffener properties as entered) differs; Annex C limits (mesh, imperfections, material) are not checked.", p.alpha_cr),
                    format!("α_cr aus dem FE-Modell {:.2} gegen σ_cr,p/σ_Ed={analytic:.2} nach Anhang A: Verhältnis {ratio:.3}. Formel ungeprüft (Spezifikation V-376 nennt keine Formeln für Anhang A oder C). Ein Verhältnis weit von 1 bedeutet, dass Modell oder Idealisierung nach Anhang A (gleichmäßiger Druck, Steifeneigenschaften wie eingegeben) abweichen; die Grenzen aus Anhang C (Netz, Imperfektionen, Werkstoff) werden nicht geprüft.", p.alpha_cr),
                ))
                .build(),
        );
    }
}

fn corrugated_shear(annex: AnnexChoice, params: AnnexParams, panel: &PlatedPanel, report: &mut CheckReport) {
    if !(panel.corrugation_flat > 0.0 && panel.corrugation_depth > 0.0 && panel.web_height > 0.0 && panel.thickness > 0.0 && panel.fy > 0.0 && panel.shear_force != 0.0) {
        return;
    }
    let g = corrugated_web::Corrugation { a0: panel.corrugation_flat * 1e3, a1: panel.corrugation_projection * 1e3, d: panel.corrugation_depth * 1e3 };
    let (hw, t, fy, v_ed) = (panel.web_height * 1e3, panel.thickness * 1e3, panel.fy / 1e6, panel.shear_force.abs());
    let at = |th: f64| corrugated_web::shear_resistance(&g, hw, th, fy, e_mpa(), nu(), params.gamma_m1);
    let r = at(t);
    let subject = |field: &str, en: &str, de: &str| subj(&panel.id, "platedPanels", field, en, de);
    let case = Case {
        id: format!("en1993.1-5.D.shear.{}", panel.id),
        part: "DIN EN 1993-1-5",
        family: "EN 1993-1-5",
        chapter: "Annex D",
        section: "D.2",
        subject: subject("shearForce", "Shear force", "Querkraft"),
        title: loc("Corrugated web shear resistance", "Schubtragfähigkeit des Trapezstegs"),
        demand: force_n(v_ed),
        capacity: force_n(r.v_b_rd),
        explanation: bilingual(
            format!("a_2={:.1} mm, pitch {:.0} mm, developed {:.0} mm; τ_cr,l={:.1} MPa (λ̄={:.3}), τ_cr,g={:.1} MPa (λ̄={:.3}); χ_c={:.3}; V_b,Rd={:.1} kN.", g.inclined(), g.pitch(), g.developed(), r.tau_cr_local, r.lambda_local, r.tau_cr_global, r.lambda_global, r.chi, r.v_b_rd / 1e3),
            format!("a_2={:.1} mm, Wellenlänge {:.0} mm, abgewickelt {:.0} mm; τ_cr,l={:.1} MPa (λ̄={:.3}), τ_cr,g={:.1} MPa (λ̄={:.3}); χ_c={:.3}; V_b,Rd={:.1} kN.", g.inclined(), g.pitch(), g.developed(), r.tau_cr_local, r.lambda_local, r.tau_cr_global, r.lambda_global, r.chi, r.v_b_rd / 1e3),
        ),
        ndps: &["ndp.en1993.v-376.1"],
        gammas: &[AnnexParams::GAMMA_M1],
    };
    emit(report, annex, case, || {
        let th = grow(t, None, |x| at(x).v_b_rd >= v_ed);
        let force = r.v_b_rd;
        solved([
            th.map(|th| lift(subject("thickness", "Web thickness", "Stegdicke"), length_m(panel.thickness), length_m(th / 1e3), &format!("Increase the web thickness to at least {th:.2} mm."), &format!("Stegdicke auf mindestens {th:.2} mm erhöhen."))),
            Some(cap(subject("shearForce", "Shear force", "Querkraft"), force_n(v_ed), force_n(force), &format!("Or limit V_Ed to {:.1} kN.", force / 1e3), &format!("Oder V_Ed auf {:.1} kN begrenzen.", force / 1e3))),
        ])
    });
}

fn check_deck(annex: AnnexChoice, params: AnnexParams, bf: &BridgeFatigue, report: &mut CheckReport) {
    if bf.deck_thickness <= 0.0 {
        return;
    }
    let (t_d, surfacing) = (bf.deck_thickness * 1e3, bf.surfacing_thickness * 1e3);
    let subject = |field: &str, en: &str, de: &str| subj(&bf.id, "bridgeFatigue", field, en, de);
    let t_min = orthotropic_deck::min_deck_thickness(surfacing);
    let case = Case {
        id: format!("en1993.2.9.4.deck.t.{}", bf.id),
        part: "DIN EN 1993-2",
        family: "EN 1993-2",
        chapter: "9",
        section: "9.4.2",
        subject: subject("deckThickness", "Deck plate thickness", "Fahrbahnblechdicke"),
        title: loc("Minimum deck plate thickness", "Mindestdicke des Fahrbahnblechs"),
        demand: length_m(t_min / 1e3),
        capacity: length_m(bf.deck_thickness),
        explanation: bilingual(format!("t_d={t_d:.1} mm against the minimum {t_min:.0} mm for a surfacing of {surfacing:.0} mm (unverified minima)."), format!("t_d={t_d:.1} mm gegen die Mindestdicke {t_min:.0} mm bei einem Belag von {surfacing:.0} mm (ungeprüfte Mindestwerte).")),
        ndps: &["ndp.en1993.v-881.1"],
        gammas: &[],
    };
    emit(report, annex, case, || vec![lift(subject("deckThickness", "Deck plate thickness", "Fahrbahnblechdicke"), length_m(bf.deck_thickness), length_m(t_min / 1e3), &format!("Increase the deck plate to at least {t_min:.0} mm."), &format!("Fahrbahnblech auf mindestens {t_min:.0} mm erhöhen."))]);
    if bf.trough_thickness > 0.0 {
        let min = orthotropic_deck::MIN_TROUGH_THICKNESS;
        let case = Case {
            id: format!("en1993.2.9.4.trough.t.{}", bf.id),
            part: "DIN EN 1993-2",
            family: "EN 1993-2",
            chapter: "9",
            section: "9.4.2",
            subject: subject("troughThickness", "Trough wall thickness", "Trogwanddicke"),
            title: loc("Minimum trough wall thickness", "Mindestdicke der Trogwand"),
            demand: length_m(min / 1e3),
            capacity: length_m(bf.trough_thickness),
            explanation: bilingual(format!("t_t={:.1} mm against the minimum {min:.0} mm (unverified minimum).", bf.trough_thickness * 1e3), format!("t_t={:.1} mm gegen die Mindestdicke {min:.0} mm (ungeprüfter Mindestwert).", bf.trough_thickness * 1e3)),
            ndps: &["ndp.en1993.v-881.2"],
            gammas: &[],
        };
        emit(report, annex, case, || vec![lift(subject("troughThickness", "Trough wall thickness", "Trogwanddicke"), length_m(bf.trough_thickness), length_m(min / 1e3), &format!("Increase the trough wall to at least {min:.0} mm."), &format!("Trogwand auf mindestens {min:.0} mm erhöhen."))]);
    }
    if !(bf.wheel_load > 0.0 && bf.wheel_patch > 0.0 && bf.trough_spacing > bf.trough_width && bf.trough_width > 0.0 && bf.heavy_vehicle_weight > 0.0 && bf.reference_weight > 0.0) {
        return;
    }
    let span = (bf.trough_spacing - bf.trough_width) * 1e3;
    let strip_at = |th: f64| orthotropic_deck::deck_strip(bf.wheel_load, orthotropic_deck::patch_side(bf.wheel_patch * 1e3, surfacing, th), span, th);
    let delta_at = |th: f64| {
        let l2 = bridge_fatigue::lambda_2(bf.heavy_vehicle_weight, bf.reference_weight, bf.observed_traffic, 0.5e6);
        let l3 = bridge_fatigue::lambda_3(bf.design_life);
        let l4 = bridge_fatigue::lambda_4(&[(bf.second_lane_traffic, bf.second_lane_weight)], bf.observed_traffic, bf.heavy_vehicle_weight);
        let delta_p = strip_at(th).sigma_hogging;
        (bridge_fatigue::delta_sigma_e2_bridge(bf.lambda_1, l2, l3, l4, if bf.lambda_max > 0.0 { bf.lambda_max } else { f64::INFINITY }, bf.phi2, delta_p), delta_p, l2, l3, l4)
    };
    let ks = bridge_fatigue::size_effect(t_d);
    let delta_c = bridge_fatigue::orthotropic_detail(&bf.detail_id).map_or(f64::from(bf.category), |d| d.min(f64::from(bf.category))) * ks;
    let (delta_e2, delta_p, l2, l3, l4) = delta_at(t_d);
    let strip = strip_at(t_d);
    let u = bridge_fatigue::fatigue_utilisation(delta_e2, delta_c, 1.0, params.gamma_mf);
    let case = Case {
        id: format!("en1993.2.9.5.deck.{}", bf.id),
        part: "DIN EN 1993-2",
        family: "EN 1993-2",
        chapter: "9",
        section: "9.5.2",
        subject: subject("wheelLoad", "Wheel load", "Radlast"),
        title: loc("Deck plate fatigue under the wheel load", "Ermüdung des Fahrbahnblechs unter der Radlast"),
        demand: dimensionless(u),
        capacity: dimensionless(1.0),
        explanation: bilingual(
            format!("Strip clamped over the clear span {span:.0} mm under Q_w={:.1} kN on a {:.0} mm patch: M_hog={:.0} N·mm/mm, M_sag={:.0} N·mm/mm, σ_hog={:.1} MPa=Δσ_p, σ_sag={:.1} MPa; λ2={l2:.3}, λ3={l3:.3}, λ4={l4:.3}: Δσ_E,2={delta_e2:.1} MPa against Δσ_C/γMf={:.1} MPa (screening strip model, no influence surface).", bf.wheel_load / 1e3, orthotropic_deck::patch_side(bf.wheel_patch * 1e3, surfacing, t_d), strip.m_hogging, strip.m_sagging, delta_p, strip.sigma_sagging, delta_c / params.gamma_mf),
            format!("Streifen eingespannt über die lichte Weite {span:.0} mm unter Q_w={:.1} kN auf {:.0} mm Aufstandsfläche: M_hog={:.0} N·mm/mm, M_sag={:.0} N·mm/mm, σ_hog={:.1} MPa=Δσ_p, σ_sag={:.1} MPa; λ2={l2:.3}, λ3={l3:.3}, λ4={l4:.3}: Δσ_E,2={delta_e2:.1} MPa gegen Δσ_C/γMf={:.1} MPa (Streifenmodell zur Abschätzung, keine Einflussfläche).", bf.wheel_load / 1e3, orthotropic_deck::patch_side(bf.wheel_patch * 1e3, surfacing, t_d), strip.m_hogging, strip.m_sagging, delta_p, strip.sigma_sagging, delta_c / params.gamma_mf),
        ),
        ndps: &["ndp.en1993.v-880.1", "ndp.en1993.v-881.3"],
        gammas: &[AnnexParams::GAMMA_MF],
    };
    emit(report, annex, case, || {
        let th = grow(t_d, None, |x| bridge_fatigue::fatigue_utilisation(delta_at(x).0, delta_c, 1.0, params.gamma_mf) <= 1.0);
        let wheel = bf.wheel_load / u.max(1.0);
        solved([
            th.map(|th| lift(subject("deckThickness", "Deck plate thickness", "Fahrbahnblechdicke"), length_m(bf.deck_thickness), length_m(th / 1e3), &format!("Increase the deck plate to at least {th:.1} mm."), &format!("Fahrbahnblech auf mindestens {th:.1} mm erhöhen."))),
            Some(cap(subject("troughSpacing", "Trough spacing", "Trogabstand"), length_m(bf.trough_spacing), length_m(bf.trough_width + span * 0.8 / 1e3), "Or reduce the clear span between the trough walls by 20 %.", "Oder die lichte Weite zwischen den Trogwänden um 20 % verringern.")),
            Some(cap(subject("wheelLoad", "Wheel load", "Radlast"), force_n(bf.wheel_load), force_n(wheel), &format!("Or limit the wheel load to {:.1} kN.", wheel / 1e3), &format!("Oder die Radlast auf {:.1} kN begrenzen.", wheel / 1e3))),
        ])
    });
}

fn purlin(annex: AnnexChoice, params: AnnexParams, c: &ColdFormedMember, report: &mut CheckReport) {
    if !(c.purlin_span > 0.0 && c.purlin_load > 0.0 && c.web_height > 0.0 && c.flange_width > 0.0 && c.thickness > 0.0 && c.fy > 0.0 && c.moment_resistance > 0.0) {
        return;
    }
    let (t, b, lip, hw, fy) = (c.thickness * 1e3, c.flange_width * 1e3, c.lip_length * 1e3, c.web_height * 1e3, c.fy / 1e6);
    let (l, q) = (c.purlin_span * 1e3, c.purlin_load / 1e3);
    let subject = |field: &str, en: &str, de: &str| subj(&c.id, "coldFormedMembers", field, en, de);
    let at = |th: f64| {
        let flange = purlin_free_flange::free_flange(th, b, lip, hw);
        let c_d = if c.sheeting_stiffness > 0.0 { cold_formed::c_d_series(c.sheeting_stiffness, cold_formed::c_d_web_bending(e_mpa(), th, hw)) } else { 0.0 };
        let k = purlin_free_flange::lateral_spring(c_d, hw);
        let m_fz = purlin_free_flange::peak_moment_on_spring(c.purlin_lateral_ratio * q, l, e_mpa() * flange.inertia, k);
        let m_y = q * l * l / 8.0;
        let m_fz_rd = flange.inertia / flange.y_max * fy / params.gamma_m0;
        (m_y / (c.moment_resistance * 1e3) + m_fz / m_fz_rd, m_y, m_fz, m_fz_rd, k)
    };
    let (u, m_y, m_fz, m_fz_rd, k) = at(t);
    let case = Case {
        id: format!("en1993.1-3.E.purlin.{}", c.id),
        part: "DIN EN 1993-1-3",
        family: "EN 1993-1-3",
        chapter: "Annex E",
        section: "E.1",
        subject: subject("purlinLoad", "Purlin load", "Pfettenlast"),
        title: loc("Purlin with free flange on the sheeting spring", "Pfette mit freiem Gurt auf der Blechfeder"),
        demand: dimensionless(u),
        capacity: dimensionless(1.0),
        explanation: bilingual(
            format!("M_y,Ed=q L²/8={:.2} kNm against M_c,Rd={:.2} kNm; free flange on K=C_D/h²={k:.4} N/mm²: M_fz,Ed={:.1} N·m against M_fz,Rd={:.1} N·m; sum {u:.3} (simply supported span, no continuity factors).", m_y / 1e6, c.moment_resistance / 1e3, m_fz / 1e3, m_fz_rd / 1e3),
            format!("M_y,Ed=q L²/8={:.2} kNm gegen M_c,Rd={:.2} kNm; freier Gurt auf K=C_D/h²={k:.4} N/mm²: M_fz,Ed={:.1} N·m gegen M_fz,Rd={:.1} N·m; Summe {u:.3} (Einfeldträger, keine Durchlaufbeiwerte).", m_y / 1e6, c.moment_resistance / 1e3, m_fz / 1e3, m_fz_rd / 1e3),
        ),
        ndps: &["ndp.en1993.v-258.1"],
        gammas: &[AnnexParams::GAMMA_M0],
    };
    emit(report, annex, case, || {
        let th = grow(t, None, |x| at(x).0 <= 1.0);
        let load = c.purlin_load / u;
        let mut remedies = vec![cap(subject("purlinLoad", "Purlin load", "Pfettenlast"), Quantity::new(QuantityKind::Force, c.purlin_load), Quantity::new(QuantityKind::Force, load), &format!("Limit the purlin load to {load:.0} N/m."), &format!("Pfettenlast auf {load:.0} N/m begrenzen."))];
        if let Some(th) = th {
            remedies.push(lift(subject("thickness", "Thickness", "Dicke"), length_m(c.thickness), length_m(th / 1e3), &format!("Increase the thickness to at least {th:.2} mm."), &format!("Dicke auf mindestens {th:.2} mm erhöhen.")));
        }
        remedies.push(lift(subject("sheetingStiffness", "Sheeting restraint", "Drehbettung"), force_n(c.sheeting_stiffness), force_n(c.sheeting_stiffness.max(1.0) * 2.0), "Or use sheeting with a larger rotational stiffness C_D,C.", "Oder Trapezblech mit größerer Drehbettung C_D,C verwenden."));
        remedies
    });
}

/// 🏛️ Appends the annex block checks of every admitted row.
pub(super) fn check_annex_blocks(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    for s in document.silo_shells.iter().filter(|e| scope.admits("siloShells", &e.id)) {
        external_pressure(annex, params, s, report);
    }
    for p in document.plated_panels.iter().filter(|e| scope.admits("platedPanels", &e.id)) {
        corrugated_shear(annex, params, p, report);
        stiffened_plate_check(annex, params, p, report);
    }
    for b in document.bridge_fatigue.iter().filter(|e| scope.admits("bridgeFatigue", &e.id)) {
        check_deck(annex, params, b, report);
    }
    for c in document.cold_formed_members.iter().filter(|e| scope.admits("coldFormedMembers", &e.id)) {
        purlin(annex, params, c, report);
    }
}
