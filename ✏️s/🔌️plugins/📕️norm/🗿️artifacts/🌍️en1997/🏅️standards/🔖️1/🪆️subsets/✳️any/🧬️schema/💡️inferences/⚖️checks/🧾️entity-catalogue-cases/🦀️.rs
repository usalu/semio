//! 🧾️ Catalogue reference verifications of the snapshot entities: the retaining wall (RW-01, RW-02, RW-04, RW-05, RW-06, RW-10), the slope (SLOPE-01, SLOPE-02, SLOPE-03, SLOPE-04), the uplift case (UPL-01), the footing sliding (GEO-F04) and DIN 4017 bearing resistance (GEO-F03), the passive resistance of an embedded wall (RW-03), the resistance-settlement of a pile (PILE-09) and the groundwater drawdown at an embedded wall (DEW-03). Each is one catalogue case of the entity; limit comparisons are reported as information (the governing project check of the entity decides compliance), and every result is retargeted to the entity row.

use super::*;
use crate::catalogue::{earth_pressure as ep, evaluate_outcomes, outcomes_to_results, slopes::Circle, Case, Mode, Outcome, R};
use crate::geo_entities::{case, retarget};
use semio_s_artifact_norm_en1997_model::{EmbeddedWall, LabTest, LayerWater, Pile, RetainingWall, Slope, UpliftCase};

/// 📖️ Turns limit comparisons into their utilisation number so a reference method informs without judging.
fn informative(outcomes: R) -> R {
    outcomes.map(|list| list.into_iter().map(|o| if o.mode == Mode::Limit { Outcome::number(&o.key, &o.en, &o.de, o.utilization()) } else { o }).collect())
}

fn reference(report: &mut CheckReport, entry: &str, input: &Case, collection: &str, id: &str, label: &LocalizedCopy) {
    report.extend(retarget(outcomes_to_results(entry, input, informative(evaluate_outcomes(entry, input))), collection, id, &label.en, &label.de));
}

/// 🗄️ Earth-pressure, surcharge, traffic and gravity-wall reference verifications of one retaining wall from its backfill, geometry and loads.
pub fn check_wall_catalogue(doc: &En1997Snapshot, wall: &RetainingWall, report: &mut CheckReport) {
    let label = loc(&format!("Wall {}", wall.id), &format!("Wand {}", wall.id));
    let (phi, h, gamma) = (wall.backfill_phi_deg.clamp(0.0, 50.0), wall.height, wall.backfill_gamma);
    let delta = wall.wall_friction_deg.clamp(0.0, phi);
    let base = case(doc, &wall.id).with_n("gamma", gamma).with_n("h", h).with_n("phi_deg", phi);
    reference(report, "RW-01", &base.clone().with_n("delta_deg", delta).with_n("q", wall.surcharge), "retainingWalls", &wall.id, &label);
    reference(report, "RW-02", &base, "retainingWalls", &wall.id, &label);
    reference(report, "RW-04", &base.clone().with_n("ocr", wall.ocr.max(1.0)).with_n("q", wall.surcharge), "retainingWalls", &wall.id, &label);
    let ka = ep::ka_coulomb(phi, delta, 0.0, 0.0);
    let water = case(doc, &wall.id).with_n("h", h).with_n("ka", ka).with_n("gamma", gamma).with_n("z_w", doc.groundwater_level.clamp(0.0, h)).with_n("q", wall.surcharge);
    reference(report, "RW-05", &water, "retainingWalls", &wall.id, &label);
    let traffic = case(doc, &wall.id).with_n("q_total", wall.surcharge * wall.base_width).with_n("b", wall.base_width).with_n("z", h).with_n("ka", ka);
    reference(report, "RW-10", &traffic, "retainingWalls", &wall.id, &label);
    let (cos_d, sin_d) = (delta.to_radians().cos(), delta.to_radians().sin());
    let (e_soil, e_surcharge) = (0.5 * ka * gamma * h * h, ka * wall.surcharge * h);
    let eah_g = e_soil * cos_d + wall.horizontal_permanent;
    let arm_g = if eah_g > 1e-9 { (e_soil * cos_d * h / 3.0 + wall.horizontal_permanent * h / 2.0) / eah_g } else { h / 3.0 };
    let weight = crate::lateral_and_seismic::wall_weight(wall);
    let gravity = case(doc, &wall.id)
        .with_n("b", wall.base_width)
        .with_n("w", weight)
        .with_n("x_w", 0.5 * wall.base_width)
        .with_n("eah_g", eah_g)
        .with_n("arm_g", arm_g)
        .with_n("eah_q", e_surcharge * cos_d)
        .with_n("arm_q", 0.5 * h)
        .with_n("eav", e_soil * sin_d)
        .with_n("x_v", wall.base_width)
        .with_n("phi_base_deg", phi);
    reference(report, "RW-06", &gravity, "retainingWalls", &wall.id, &label);
    if wall.reinforcement_spacing > 0.0 && wall.reinforcement_friction > 0.0 {
        let reinforced = case(doc, &wall.id)
            .with_n("k", ka)
            .with_n("sigma_v", gamma * h + wall.surcharge)
            .with_n("s_v", wall.reinforcement_spacing)
            .with_n("f_star", wall.reinforcement_friction)
            .with_n("b", if wall.reinforcement_width > 0.0 { wall.reinforcement_width } else { 1.0 });
        reference(report, "RW-09", &reinforced, "retainingWalls", &wall.id, &label);
    }
}

/// ⛰️ The slip circle of a slope for the catalogue methods: the user circle or the critical Bishop circle of the governing layer as homogeneous ground with the given design strength.
pub fn slope_circle(doc: &En1997Snapshot, slope: &Slope, gwl: f64, tan_phi: f64, c: f64) -> Option<Circle> {
    use crate::catalogue::slopes::critical_circle;
    let layer = doc.layers.iter().find(|l| l.id == slope.governing_layer_id)?;
    let proto = Circle { h: slope.height, beta_deg: slope.angle_deg, xc: 0.0, yc: 0.0, r: 1.0, gamma: layer.gamma, gamma_sat: layer.gamma_prime + 9_810.0, gwl_level: slope.height - gwl, ru: 0.0, q: 0.0 };
    let circle = if slope.circle_radius > 1e-6 { Circle { xc: slope.circle_centre_x, yc: slope.circle_centre_z, r: slope.circle_radius, ..proto } } else { critical_circle(&proto, tan_phi, c, 60)?.0 };
    Some(circle)
}

/// ⛰️ Bishop, Janbu, infinite-slope and plane-wedge reference verifications of one slope with the GEO-3 factors of the project check.
pub fn check_slope_catalogue(doc: &En1997Snapshot, slope: &Slope, annex: AnnexChoice, gwl: f64, report: &mut CheckReport) {
    let label = loc(&format!("Slope {}", slope.id), &format!("Böschung {}", slope.id));
    let geo3 = DesignApproach::Geo3.annex_params(annex);
    let Some(layer) = doc.layers.iter().find(|l| l.id == slope.governing_layer_id) else { return };
    let (tan_phi, c) = (layer.phi_prime_deg.to_radians().tan() / geo3.gamma_phi, layer.cohesion_effective / geo3.gamma_c);
    let strength = |c0: Case| c0.with_s("approach", "geo3").with_n("phi_k_deg", layer.phi_prime_deg).with_n("c_k", layer.cohesion_effective);
    if let Some(circle) = slope_circle(doc, slope, gwl, tan_phi, c) {
        let method = strength(case(doc, &slope.id))
            .with_n("h", circle.h)
            .with_n("beta_deg", circle.beta_deg)
            .with_n("xc", circle.xc)
            .with_n("yc", circle.yc)
            .with_n("r", circle.r)
            .with_n("gamma", circle.gamma)
            .with_n("gamma_sat", circle.gamma_sat)
            .with_n("gwl_level", circle.gwl_level);
        reference(report, "SLOPE-01", &method, "slopes", &slope.id, &label);
        reference(report, "SLOPE-02", &method, "slopes", &slope.id, &label);
    }
    let infinite = strength(case(doc, &slope.id)).with_n("beta_deg", slope.angle_deg).with_n("gamma", layer.gamma).with_n("z", slope.height);
    reference(report, "SLOPE-03", &infinite, "slopes", &slope.id, &label);
    let wedge = strength(case(doc, &slope.id)).with_n("h", slope.height).with_n("beta_deg", slope.angle_deg).with_n("gamma", layer.gamma);
    reference(report, "SLOPE-04", &wedge, "slopes", &slope.id, &label);
}

/// ⬆️ Uplift equilibrium of one uplift case as the catalogue entry UPL-01.
pub fn check_uplift_catalogue(doc: &En1997Snapshot, uplift: &UpliftCase, report: &mut CheckReport) {
    let label = loc(&format!("Uplift {}", uplift.id), &format!("Aufschwimmen {}", uplift.id));
    let c = case(doc, &uplift.id).with_n("g_dst", uplift.permanent_destabilizing).with_n("q_dst", uplift.variable_destabilizing).with_n("g_stb", uplift.permanent_stabilizing);
    reference(report, "UPL-01", &c, "upliftCases", &uplift.id, &label);
}

/// 🏛️ Base sliding of one footing load case as the catalogue entry GEO-F04 (base friction `delta` = φ′ or the undrained shear strength over the base area; the passive share is left out).
pub fn footing_sliding_catalogue(doc: &En1997Snapshot, footing: &SpreadFoundation, lc: &FoundationLoadCase, situation: &str, phi_deg: f64, undrained: Option<f64>, report: &mut CheckReport) {
    let label = loc(&format!("Footing {}", footing.id), &format!("Fundament {}", footing.id));
    let mut c = case(doc, &footing.id).with_s("situation", situation).with_n("h_g", lc.horizontal_permanent).with_n("h_q", lc.horizontal_variable).with_n("v_k", lc.vertical_permanent).with_n("delta_deg", phi_deg);
    if let Some(cu) = undrained {
        c = c.with_n("cu_k", cu).with_n("a_eff", footing.width * footing.length);
    }
    reference(report, "GEO-F04", &c, "footings", &footing.id, &label);
}

/// 💧 Consolidation settlement of the layer at the toe of an embedded wall from the groundwater drawdown of the excavation as the catalogue entry DEW-03.
pub fn drawdown_catalogue(doc: &En1997Snapshot, wall: &EmbeddedWall, head: f64, layer: &SoilLayer, report: &mut CheckReport) {
    let label = loc(&format!("Embedded wall {}", wall.id), &format!("Eingespannte Wand {}", wall.id));
    let c = case(doc, &wall.id).with_n("delta_h", head).with_n("h", layer.depth_bottom - layer.depth_top).with_n("e_oed", layer.oedometric_modulus);
    reference(report, "DEW-03", &c, "embeddedWalls", &wall.id, &label);
}

/// 🏛️ DIN 4017 bearing resistance of one footing load case as the catalogue entry GEO-F03 with the design actions of the project check (drained strength, horizontal load and moment along the width).
pub fn footing_din4017_catalogue(doc: &En1997Snapshot, footing: &SpreadFoundation, lc: &FoundationLoadCase, situation: &str, layer: &SoilLayer, gamma_below: f64, report: &mut CheckReport) {
    let label = loc(&format!("Footing {}", footing.id), &format!("Fundament {}", footing.id));
    let p = resolve_params(parse_design_approach(doc.design_approach.as_str()), doc.annex, situation);
    let c = case(doc, &format!("{}.{}", footing.id, lc.id))
        .with_s("situation", situation)
        .with_n("b", footing.width)
        .with_n("l", footing.length)
        .with_n("d", footing.embedment)
        .with_n("alpha_deg", footing.base_inclination_deg)
        .with_n("v_d", design_actions(&p, lc.vertical_permanent, lc.vertical_variable))
        .with_n("hb_d", design_actions(&p, lc.horizontal_permanent, lc.horizontal_variable))
        .with_n("mb_d", design_actions(&p, lc.moment_permanent, lc.moment_variable))
        .with_n("phi_k_deg", layer.phi_prime_deg)
        .with_n("c_k", layer.cohesion_effective)
        .with_n("gamma1", gamma_below)
        .with_n("gamma2", layer.gamma);
    reference(report, "GEO-F03", &c, "footings", &footing.id, &label);
}

/// 🧱️ Passive earth pressure of the embedment of one embedded wall on the curved failure surface as the catalogue entry RW-03.
pub fn embedded_passive_catalogue(doc: &En1997Snapshot, wall: &EmbeddedWall, report: &mut CheckReport) {
    let label = loc(&format!("Embedded wall {}", wall.id), &format!("Eingespannte Wand {}", wall.id));
    let c = case(doc, &wall.id).with_n("gamma", wall.backfill_gamma).with_n("t", wall.embedment).with_n("phi_deg", wall.phi_deg).with_n("delta_deg", wall.delta_passive_deg);
    reference(report, "RW-03", &c, "embeddedWalls", &wall.id, &label);
}

/// 🪵️ Resistance-settlement of one pile at its serviceability compression per pile as the catalogue entry PILE-09: the model shaft and base resistance, mobilised on the bilinear curve, plus the elastic shortening with E_c of the pile.
pub fn pile_settlement_catalogue(doc: &En1997Snapshot, pile: &Pile, report: &mut CheckReport) {
    let modulus = crate::catalogue::piles::pile_elastic_modulus_pa(pile);
    if modulus <= 0.0 {
        return;
    }
    let label = loc(&format!("Pile {}", pile.id), &format!("Pfahl {}", pile.id));
    let area = std::f64::consts::PI * pile.diameter * pile.diameter / 4.0;
    let c = case(doc, &pile.id)
        .with_n("n", (pile.compression_permanent + pile.compression_variable) / f64::from(pile.count.max(1)))
        .with_n("rs_max", crate::part_1::shaft_resistance_n(pile.alpha_s, pile.diameter, pile.unit_shaft_resistance, pile.length))
        .with_n("rb_max", crate::part_1::base_resistance_n(pile.unit_base_resistance, area))
        .with_n("d", pile.diameter)
        .with_n("ea", modulus * area)
        .with_n("l", pile.length);
    reference(report, "PILE-09", &c, "piles", &pile.id, &label);
}

/// 🧪️ Primary consolidation with time (SLS-02) and secondary compression (SLS-03) of the layer of one oedometer series: thickness and mid-depth effective stress from the layer, the indices and the stress increment from the series; every missing input skips its entry.
pub fn lab_settlement_catalogue(doc: &En1997Snapshot, test: &LabTest, layer: &SoilLayer, report: &mut CheckReport) {
    let label = loc(&format!("Laboratory test {}", test.id), &format!("Laborversuch {}", test.id));
    let thickness = layer.depth_bottom - layer.depth_top;
    if thickness <= 0.0 || test.void_ratio <= 0.0 {
        return;
    }
    let base = case(doc, &test.id).with_n("h", thickness).with_n("e0", test.void_ratio);
    if test.compression_index > 0.0 && test.stress_increment > 0.0 {
        let mut c = base
            .clone()
            .with_n("cc", test.compression_index)
            .with_n("sigma0", crate::station_fields::effective_stress(&doc.layers, doc.groundwater_level, 0.5 * (layer.depth_top + layer.depth_bottom)).max(1.0))
            .with_n("dsigma", test.stress_increment);
        if test.swelling_index > 0.0 {
            c = c.with_n("cs", test.swelling_index);
        }
        if test.preload_stress > 0.0 {
            c = c.with_n("sigma_p", test.preload_stress);
        }
        if test.drainage_height > 0.0 && test.consolidation_time > 0.0 {
            c = c.with_n("cv", crate::catalogue::settlement::t90_factor() * test.drainage_height * test.drainage_height / test.consolidation_time).with_n("h_dr", test.drainage_height);
            if test.design_life > 0.0 {
                c = c.with_n("t", test.design_life);
            }
        }
        reference(report, "SLS-02", &c, "labTests", &test.id, &label);
    }
    if test.secondary_compression_index > 0.0 && test.consolidation_time > 0.0 && test.design_life > test.consolidation_time {
        let c = base.with_n("c_alpha", test.secondary_compression_index).with_n("t1", test.consolidation_time).with_n("t2", test.design_life);
        reference(report, "SLS-03", &c, "labTests", &test.id, &label);
    }
}

/// 💧 Single-well discharge (DEW-01) of the pumped well in one layer water: undisturbed head and water level of the well above the layer base, confined when the piezometric level stands above the layer top.
pub fn well_catalogue(doc: &En1997Snapshot, water: &LayerWater, layer: &SoilLayer, report: &mut CheckReport) {
    let label = loc(&format!("Layer water {}", water.id), &format!("Schichtwasser {}", water.id));
    if water.well_radius <= 0.0 || water.well_water_depth <= 0.0 || water.permeability <= 0.0 {
        return;
    }
    let mut c = case(doc, &water.id)
        .with_n("k", water.permeability)
        .with_n("h_aquifer", layer.depth_bottom - water.piezometric_depth)
        .with_n("h_well", layer.depth_bottom - water.well_water_depth)
        .with_n("r_well", water.well_radius)
        .with_n("thickness", layer.depth_bottom - layer.depth_top)
        .with_s("aquifer", if water.piezometric_depth < layer.depth_top { "confined" } else { "unconfined" });
    if water.influence_radius > 0.0 {
        c = c.with_n("r_influence", water.influence_radius);
    }
    if water.well_capacity > 0.0 {
        c = c.with_n("q_well", water.well_capacity);
    }
    reference(report, "DEW-01", &c, "layerWaters", &water.id, &label);
}

/// 🪵️ Negative skin friction (PILE-05) of the settling ground along the shaft with the downdrag verdict F_c,d + F_neg,d against the design compressive resistance R_c,d of the pile and the dynamic-test evaluation with the driving formula (PILE-08) of one pile; the dynamic series is rebuilt from its count, mean and lowest value, which fix the characteristic value min{mean/ξ5; min/ξ6}.
pub fn pile_ground_catalogue(doc: &En1997Snapshot, pile: &Pile, report: &mut CheckReport) {
    let label = loc(&format!("Pile {}", pile.id), &format!("Pfahl {}", pile.id));
    if pile.settling_thickness > 0.0 && pile.diameter > 0.0 {
        let h1 = pile.settling_thickness.min(pile.length);
        let mut c = case(doc, &pile.id)
            .with_n("d", pile.diameter)
            .with_n("h1", h1)
            .with_n("sv1", crate::station_fields::effective_stress(&doc.layers, doc.groundwater_level, 0.5 * h1));
        if pile.negative_friction_beta > 0.0 {
            c = c.with_n("beta", pile.negative_friction_beta);
        }
        let factors = c.factors();
        let per_pile = f64::from(pile.count.max(1));
        let resistance = pile_design_resistance(pile, parse_design_approach(doc.design_approach.as_str()), &factors);
        c = c.with_n("f_cd", (factors.gamma_g * pile.compression_permanent + factors.gamma_q * pile.compression_variable) / per_pile).with_n("r_c_d", resistance.r_c_d / per_pile);
        reference(report, "PILE-05", &c, "piles", &pile.id, &label);
    }
    let count = pile.dynamic_test_count.round().min(64.0);
    if count >= 1.0 && pile.dynamic_resistance_mean > 0.0 && pile.dynamic_resistance_min > 0.0 && pile.dynamic_resistance_min <= pile.dynamic_resistance_mean {
        let mut c = case(doc, &pile.id).with_n("rm1", pile.dynamic_resistance_min);
        for k in 2..=(count as usize) {
            c = c.with_n(&format!("rm{k}"), (count * pile.dynamic_resistance_mean - pile.dynamic_resistance_min) / (count - 1.0));
        }
        if pile.hammer_efficiency > 0.0 && pile.hammer_weight > 0.0 && pile.drop_height > 0.0 && pile.set_per_blow > 0.0 {
            c = c.with_n("eta_h", pile.hammer_efficiency).with_n("w", pile.hammer_weight).with_n("h_drop", pile.drop_height).with_n("s_set", pile.set_per_blow).with_n("s_e", pile.elastic_compression);
        }
        let factors = c.factors();
        c = c.with_n("f_cd", (factors.gamma_g * pile.compression_permanent + factors.gamma_q * pile.compression_variable) / f64::from(pile.count.max(1)));
        reference(report, "PILE-08", &c, "piles", &pile.id, &label);
    }
}
