//! 🌋️ Lateral pile and seismic wall verifications of the snapshot entities: the transversely loaded pile on a Winkler foundation (PILE-04) with its buckling interface (PILE-07, EN 1997-1 §7.7, §7.8) and the pseudo-static Mononobe-Okabe thrust on a retaining wall (RW-08, EN 1998-5 §7.3) with the seismic sliding of the base. Each reference is one catalogue case of the entity; the lateral and seismic checks are the judges, with remedies on the snapshot fields.

use super::*;
use crate::catalogue::{earth_pressure as ep, evaluate_outcomes, outcomes_to_results, piles, Mode, Outcome, R};
use crate::geo_entities::{case, retarget};
use semio_s_artifact_norm_en1997_model::{Pile, RetainingWall};
use semio_s_plugin_norm_mechanics::geotechnics::earth_pressure::{critical_wedge, thrust, Acceleration, Interface, Side, Thrust, Water};

const LATERAL_LIMIT_ID: &str = "ndp.en1997.7.7.lateral-deflection-limit";
const SEISMIC_SLIDING_ID: &str = "ndp.en1997.9.seismic.gamma-sliding";

/// 📏️ Deflection limit of the pile head [m]: the registry entry for the annex (10 mm, unverified, no normative value is stated for it in EN 1997-1 §7.7); a missing value refuses the check (`NdpGap`), it never defaults.
pub fn lateral_deflection_limit(annex: AnnexChoice) -> f64 {
    semio_s_plugin_norm_compliance::ndp_number(LATERAL_LIMIT_ID, annex)
}

/// 🌋 Partial factor on the seismic base sliding resistance: the registry entry for the annex (1.0, unverified, EN 1998-5 §3.1 leaves the seismic γ_M and γ_R to the National Annex); a missing value refuses the check (`NdpGap`), it never defaults.
pub fn seismic_sliding_factor(annex: AnnexChoice) -> f64 {
    semio_s_plugin_norm_compliance::ndp_number(SEISMIC_SLIDING_ID, annex)
}

/// 🪵️ The Winkler state of a pile from its snapshot row: characteristic length parameter λ, head deflection y₀ and extreme moment; `None` unless EI, k_h, the diameter and the head load are given.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LateralPile {
    pub lambda: f64,
    pub head_deflection: f64,
    pub max_moment: f64,
    pub long_pile: bool,
}

pub fn lateral_pile(pile: &Pile) -> Option<LateralPile> {
    let (ei, k, d, h) = (pile.bending_stiffness_ei, pile.horizontal_subgrade_modulus, pile.diameter, pile.lateral_head_load);
    (ei > 0.0 && k > 0.0 && d > 0.0 && h > 0.0).then(|| {
        let lambda = piles::lateral_lambda(k, d, ei);
        LateralPile { lambda, head_deflection: piles::lateral_deflection(h, lambda, k, d, 0.0), max_moment: 0.322 * h / lambda, long_pile: lambda * pile.length >= std::f64::consts::PI }
    })
}

/// 🪵️ Design axial force of one pile of a group of `count` for the buckling interface: `γ_G·G + γ_Q·Q` of the design situation over the pile count.
pub fn buckling_force(pile: &Pile, approach: DesignApproach, annex: AnnexChoice, situation: &str) -> f64 {
    let p = resolve_params(approach, annex, situation);
    design_actions(&p, pile.compression_permanent, pile.compression_variable) / f64::from(pile.count.max(1))
}

/// 🪵️ Elastic critical load and effective length of a pile (pinned ends, Winkler support k_h·D): `l_eff = π·√(EI/N_cr)`.
pub fn buckling_state(pile: &Pile) -> Option<(f64, f64)> {
    (pile.bending_stiffness_ei > 0.0 && pile.length > 0.0).then(|| {
        let n_cr = piles::winkler_buckling_load(pile.bending_stiffness_ei, pile.length, pile.horizontal_subgrade_modulus, pile.diameter);
        (n_cr, std::f64::consts::PI * (pile.bending_stiffness_ei / n_cr).sqrt())
    })
}

fn informative(outcomes: R) -> R {
    outcomes.map(|list| list.into_iter().map(|o| if o.mode == Mode::Limit { Outcome::number(&o.key, &o.en, &o.de, o.utilization()) } else { o }).collect())
}

/// 🪵️ PILE-04 lateral deflection and PILE-07 buckling of one pile with its remedies on `bendingStiffnessEi`.
pub fn check_pile(doc: &En1997Snapshot, pile: &Pile, approach: DesignApproach, annex: AnnexChoice, report: &mut CheckReport) {
    let (en, de) = (format!("Pile {}", pile.id), format!("Pfahl {}", pile.id));
    let ei_subject = || subject_entity(&pile.id, &id_path("piles", &pile.id, "bendingStiffnessEi"), &en, &de);
    if let Some(state) = lateral_pile(pile) {
        let limit = lateral_deflection_limit(annex);
        let lateral = case(doc, &pile.id).with_n("d", pile.diameter).with_n("h_d", pile.lateral_head_load).with_n("k_s", pile.horizontal_subgrade_modulus).with_n("ei", pile.bending_stiffness_ei).with_n("y_lim", limit);
        let mut results = retarget(outcomes_to_results("PILE-04", &lateral, evaluate_outcomes("PILE-04", &lateral)), "piles", &pile.id, &en, &de);
        for result in results.iter_mut().filter(|r| r.id.starts_with("en1997.pile-04.deflection.") && r.utilization > 1.0) {
            let required = pile.bending_stiffness_ei * (state.head_deflection / limit).powi(4);
            result.remedies.push(Remedy::at_least(
                ei_subject(),
                Quantity::new(QuantityKind::Dimensionless, pile.bending_stiffness_ei),
                Quantity::new(QuantityKind::Dimensionless, required),
                loc("Raise the bending stiffness EI (larger diameter or reinforcement) to limit the head deflection.", "Biegesteifigkeit EI erhöhen (größerer Durchmesser oder Bewehrung), um die Kopfverschiebung zu begrenzen."),
            ));
        }
        report.extend(results);
    }
    if let Some((n_cr, l_eff)) = buckling_state(pile) {
        let n_ed = buckling_force(pile, approach, annex, doc.design_situation.as_str());
        if n_ed > 0.0 {
            let buckling = case(doc, &pile.id).with_n("ei", pile.bending_stiffness_ei).with_n("l_eff", l_eff).with_n("n_ed", n_ed);
            let mut results = retarget(outcomes_to_results("PILE-07", &buckling, evaluate_outcomes("PILE-07", &buckling)), "piles", &pile.id, &en, &de);
            for result in results.iter_mut().filter(|r| r.id.starts_with("en1997.pile-07.buckling.") && r.utilization > 1.0) {
                result.remedies.push(Remedy::at_least(
                    ei_subject(),
                    Quantity::new(QuantityKind::Dimensionless, pile.bending_stiffness_ei),
                    Quantity::new(QuantityKind::Dimensionless, pile.bending_stiffness_ei * n_ed / n_cr),
                    loc("Raise the bending stiffness EI above the critical-load requirement.", "Biegesteifigkeit EI über die Anforderung der kritischen Last erhöhen."),
                ));
            }
            report.extend(results);
        }
    }
}

/// 🌋 Weight of the wall assembly per metre [N/m]: the permanent vertical load plus the stem and base concrete.
pub fn wall_weight(wall: &RetainingWall) -> f64 {
    wall.vertical_permanent + wall.stem_thickness * (wall.height + wall.base_width) * wall.concrete_gamma.max(1_000.0)
}

/// 🌋 Pseudo-static state of a wall under (k_h, k_v): Mononobe-Okabe thrust, base sliding demand and resistance per metre and the critical wedge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SeismicWall {
    pub thrust: Thrust,
    pub static_thrust: f64,
    pub demand: f64,
    pub resistance: f64,
    pub wedge_deg: f64,
}

pub fn seismic_wall(wall: &RetainingWall, base_width: f64) -> Option<SeismicWall> {
    if wall.seismic_kh <= 0.0 || wall.height <= 0.0 {
        return None;
    }
    let (phi, h, gamma) = (wall.backfill_phi_deg.clamp(0.0, 50.0), wall.height, wall.backfill_gamma);
    let delta = wall.wall_friction_deg.clamp(0.0, phi);
    let load = thrust(Side::Active, &Interface::vertical(phi, delta), gamma, h, Water::NONE, Acceleration { kh: wall.seismic_kh, kv: wall.seismic_kv })?;
    let vertical = load.vertical;
    let weight = wall_weight(&RetainingWall { base_width, ..wall.clone() });
    let (cos_d, sin_d) = (delta.to_radians().cos(), delta.to_radians().sin());
    let demand = load.total() * cos_d + wall.seismic_kh * weight;
    let resistance = (weight * vertical + load.total() * sin_d) * phi.to_radians().tan();
    let (wedge_deg, _) = critical_wedge(gamma, h, phi, delta, wall.seismic_kh, vertical);
    Some(SeismicWall { thrust: load, static_thrust: 0.5 * ep::ka_coulomb(phi, delta, 0.0, 0.0) * gamma * h * h, demand, resistance, wedge_deg })
}

/// 🌋 RW-08 Mononobe-Okabe reference and the seismic base sliding of one wall; a wall without k_h has no seismic situation.
pub fn check_wall(doc: &En1997Snapshot, wall: &RetainingWall, annex: AnnexChoice, report: &mut CheckReport) {
    let (en, de) = (format!("Wall {}", wall.id), format!("Wand {}", wall.id));
    let Some(state) = seismic_wall(wall, wall.base_width) else {
        if wall.seismic_kh > 0.0 && wall.height > 0.0 {
            report.push(no_equilibrium(wall, annex, &en, &de));
        }
        return;
    };
    let (phi, delta) = (wall.backfill_phi_deg.clamp(0.0, 50.0), wall.wall_friction_deg.clamp(0.0, wall.backfill_phi_deg.clamp(0.0, 50.0)));
    let reference = case(doc, &wall.id).with_n("gamma", wall.backfill_gamma).with_n("h", wall.height).with_n("phi_deg", phi).with_n("delta_deg", delta).with_n("kh", wall.seismic_kh).with_n("kv", wall.seismic_kv);
    report.extend(retarget(outcomes_to_results("RW-08", &reference, informative(evaluate_outcomes("RW-08", &reference))), "retainingWalls", &wall.id, &en, &de));
    let factor = seismic_sliding_factor(annex);
    let resistance = state.resistance / factor;
    let path = id_path("retainingWalls", &wall.id, "baseWidth");
    let mut sliding = CheckResult::assess(
        format!("en1998.5.7.3.seismic-sliding.{}", wall.id),
        "EN 1998-5",
        ClauseId::new("EN 1998-5", "7", "7.3.2.2"),
        subject_entity(&wall.id, &path, &en, &de),
        loc("Seismic sliding of the wall base", "Gleiten der Wandsohle im Erdbebenfall"),
    )
    .utilization(Quantity::new(QuantityKind::Force, state.demand), Quantity::new(QuantityKind::Force, resistance))
    .annex(annex)
    .explanation(loc(
        &format!("k_h={:.3}, k_v={:.3}: E_ae={:.1} kN/m (static {:.1} kN/m), K_ae={:.3}, critical wedge {:.1} deg; H_Ed={:.1} kN/m vs R_d={:.1} kN/m (gamma {:.2} unverified).", wall.seismic_kh, wall.seismic_kv, state.thrust.total() / 1000.0, state.static_thrust / 1000.0, state.thrust.coefficient, state.wedge_deg, state.demand / 1000.0, resistance / 1000.0, factor),
        &format!("k_h={:.3}, k_v={:.3}: E_ae={:.1} kN/m (statisch {:.1} kN/m), K_ae={:.3}, kritischer Gleitkeil {:.1} Grad; H_Ed={:.1} kN/m gegen R_d={:.1} kN/m (gamma {:.2} ungeprüft).", wall.seismic_kh, wall.seismic_kv, state.thrust.total() / 1000.0, state.static_thrust / 1000.0, state.thrust.coefficient, state.wedge_deg, state.demand / 1000.0, resistance / 1000.0, factor),
    ));
    if state.demand > resistance {
        let required = (1..=400).map(|i| wall.base_width + 0.05 * f64::from(i)).find(|b| seismic_wall(wall, *b).is_some_and(|s| s.demand <= s.resistance / factor)).unwrap_or(wall.base_width * state.demand / resistance.max(1.0));
        sliding = sliding.remedy(Remedy::at_least(
            subject_entity(&wall.id, &path, &en, &de),
            Quantity::length_m(wall.base_width),
            Quantity::length_m(required),
            loc("Widen the base to raise the sliding resistance under the seismic thrust.", "Sohlbreite erhöhen, um den Gleitwiderstand unter dem Erdbebenerddruck zu steigern."),
        ));
    }
    report.push(sliding.build());
}

fn no_equilibrium(wall: &RetainingWall, annex: AnnexChoice, en: &str, de: &str) -> CheckResult {
    let limit = (wall.backfill_phi_deg.to_radians().tan() * (1.0 - wall.seismic_kv)).max(0.0);
    CheckResult::assess(
        format!("en1998.5.7.3.seismic-sliding.{}", wall.id),
        "EN 1998-5",
        ClauseId::new("EN 1998-5", "7", "7.3.2.2"),
        subject_entity(&wall.id, &id_path("retainingWalls", &wall.id, "seismicKh"), en, de),
        loc("Seismic sliding of the wall base", "Gleiten der Wandsohle im Erdbebenfall"),
    )
    .status(CheckStatus::Fail)
    .annex(annex)
    .explanation(loc(
        &format!("k_h={:.3} with k_v={:.3} leaves no limit equilibrium of the backfill (θ > φ′): the Mononobe-Okabe thrust does not exist.", wall.seismic_kh, wall.seismic_kv),
        &format!("k_h={:.3} mit k_v={:.3} lässt kein Grenzgleichgewicht der Hinterfüllung zu (θ > φ′): der Mononobe-Okabe-Erddruck existiert nicht.", wall.seismic_kh, wall.seismic_kv),
    ))
    .remedy(Remedy::at_most(
        subject_entity(&wall.id, &id_path("retainingWalls", &wall.id, "seismicKh"), en, de),
        Quantity::new(QuantityKind::Dimensionless, wall.seismic_kh),
        Quantity::new(QuantityKind::Dimensionless, limit),
        loc("Reduce k_h below (1 − k_v)·tan φ′ or improve the backfill friction angle.", "k_h unter (1 − k_v)·tan φ′ senken oder den Reibungswinkel der Hinterfüllung verbessern."),
    ))
    .build()
}
