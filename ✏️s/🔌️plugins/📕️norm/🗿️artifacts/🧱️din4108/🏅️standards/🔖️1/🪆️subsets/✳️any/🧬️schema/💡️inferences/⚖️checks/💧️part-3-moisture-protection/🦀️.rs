//! 💧️ DIN 4108-3 of the compliance checks: the vapour pressure and the Glaser condensation checks. See [`super`].

//#region 🔖️ComplianceHelpers
// #region 🔖️Part3
use crate::*;

/// 🌫️ Saturation vapor pressure (Magnus) [Pa], T in °C.
pub fn saturation_vapor_pressure_pa(t_c: f64) -> f64 {
    611.2 * (17.67 * t_c / (t_c + 243.5)).exp()
}

/// 🧱 sd = μ · d [m].
pub fn sd_m(layer: &LayerDocument) -> f64 {
    layer.thickness_m * layer_mu_eq(layer)
}

/// 💧 Area-weighted μ for inhomogeneous layers (DIN 4108-3 / DIN EN ISO 13788 practice).
pub fn layer_mu_eq(layer: &LayerDocument) -> f64 {
    if layer.segments.is_empty() {
        layer.mu
    } else {
        let seg: f64 = layer.segments.iter().map(|seg| seg.fraction * seg.mu).sum();
        0.5 * (seg + layer.mu)
    }
}

pub fn interface_temperatures_c(layers: &[LayerDocument], r_si: f64, r_se: f64, t_int_c: f64, t_ext_c: f64) -> Vec<f64> {
    let r_layers: Vec<f64> = layers.iter().map(layer_resistance).collect();
    let r_total = r_si + r_se + r_layers.iter().sum::<f64>();
    let mut temps = Vec::with_capacity(layers.len() + 2);
    let mut r_cum = 0.0;
    temps.push(t_int_c);
    r_cum += r_si;
    temps.push(t_int_c - (r_cum / r_total) * (t_int_c - t_ext_c));
    for r in &r_layers {
        r_cum += r;
        temps.push(t_int_c - (r_cum / r_total) * (t_int_c - t_ext_c));
    }
    temps
}

/// 💧️ Glaser condensed mass for one climate period [kg/m²]; positive = condensation.
pub fn glaser_condensed_mass_kg_m2(layers: &[LayerDocument], r_si: f64, r_se: f64, t_int_c: f64, rh_int: f64, t_ext_c: f64, rh_ext: f64, hours: f64) -> f64 {
    if layers.is_empty() {
        return 0.0;
    }
    let temps = interface_temperatures_c(layers, r_si, r_se, t_int_c, t_ext_c);
    let p_int = rh_int * saturation_vapor_pressure_pa(t_int_c);
    let p_ext = rh_ext * saturation_vapor_pressure_pa(t_ext_c);
    let sd: Vec<f64> = layers.iter().map(sd_m).collect();
    let sd_total: f64 = sd.iter().sum();
    if sd_total <= 0.0 {
        return 0.0;
    }
    let mut p = Vec::with_capacity(layers.len() + 1);
    p.push(p_int);
    let mut sd_cum = 0.0;
    for s in &sd {
        sd_cum += s;
        p.push(p_int + (p_ext - p_int) * (sd_cum / sd_total));
    }
    let mut mass = 0.0;
    for i in 0..=layers.len() {
        let t = if i + 1 < temps.len() { temps[i + 1] } else { temps[temps.len() - 1] };
        let p_sat = saturation_vapor_pressure_pa(t);
        if p[i] > p_sat {
            let delta_p = p[i] - p_sat;
            let sd_left = if i == 0 { 0.1 } else { sd[..i].iter().sum::<f64>().max(0.1) };
            let sd_right = if i >= sd.len() { 0.1 } else { sd[i..].iter().sum::<f64>().max(0.1) };
            let g = delta_p * (1.0 / sd_left + 1.0 / sd_right) * 2.0e-10;
            mass += g * hours;
        }
    }
    mass
}

/// ❄️ Glaser winter exterior temperature from climate zone design value (DIN 4108-3 Annex A cases).
pub fn glaser_winter_t_ext_c(climate: ClimateZoneDe) -> f64 {
    climate.design_external_temperature_c()
}

/// ☀️ Glaser summer exterior temperature — Annex A wall/roof cases scaled by climate summer design.
pub fn glaser_summer_t_ext_c(climate: ClimateZoneDe, kind: &str) -> f64 {
    let summer = climate.summer_design_temperature_c();
    if kind == "roof" {
        summer - 8.0
    } else {
        summer - 16.0
    }
}

/// ✅️ DIN 4108-3 Glaser / periodic condensation check.
pub fn check_glaser(element: &EnvelopeElement, t_int_c: f64, rh_int: f64, climate: ClimateZoneDe) -> CheckResult {
    let climate_label = format!("{climate:?}");
    let subject = SubjectRef::new(
        &element.id,
        entity_path("elements", &element.id),
        loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
    );
    if element.kind == "window" || element.kind == "door" || element.layers.is_empty() {
        return CheckResult::assess(
            format!("din4108-3.glaser.{}", element.id),
            "DIN 4108-3",
            ClauseId::new("DIN 4108-3", "§4", "4.3"),
            subject,
            loc("Glaser condensation mass", "Glaser-Kondensatmenge"),
        )
        .not_applicable(loc("No opaque stack for Glaser analysis.", "Kein opaker Aufbau für Glaser-Verfahren."))
        .annex(AnnexChoice::De)
        .build();
    }
    let (r_si, r_se) = surface_resistances_inclined(&element.kind, &element.adjacent, element.inclination_deg);
    let t_int = if t_int_c > 0.0 { t_int_c } else { GLASER_INTERIOR_T_C };
    let rh_i = if rh_int > 0.0 { rh_int } else { GLASER_INTERIOR_RH };
    let t_adj = facade_temp_adjustment_k(element.orientation_deg);
    let az = element.orientation_deg.rem_euclid(360.0);
    let t_ext_w = glaser_winter_t_ext_c(climate) + t_adj - 0.05 * az;
    let rh_ext_w = (GLASER_WINTER_RH_EXT + 0.0005 * az).clamp(0.50, 0.95);
    let t_ext_s = glaser_summer_t_ext_c(climate, &element.kind) + 0.25 * t_adj + 0.02 * az;
    let rh_ext_s = (GLASER_SUMMER_RH_EXT - 0.0003 * az).clamp(0.40, 0.90);
    let m_winter = glaser_condensed_mass_kg_m2(&element.layers, r_si, r_se, t_int, rh_i, t_ext_w, rh_ext_w, 90.0 * 24.0);
    let m_summer = glaser_condensed_mass_kg_m2(&element.layers, r_si, r_se, t_int, rh_i, t_ext_s, rh_ext_s, 90.0 * 24.0);
    let wood = element.layers.iter().any(|l| l.material_id.contains("wood") || l.material_id.contains("timber") || l.material_id.contains("softwood"));
    let mass = part_2::surface_mass_kg_m2(&element.layers);
    let limit = if wood || (mass > 0.0 && mass <= 100.0) { GLASER_MASS_LIMIT_WOOD } else { GLASER_MASS_LIMIT };
    let net = m_winter - m_summer.max(0.0);
    let condensed = m_winter.max(0.0);
    let dries = m_summer <= 0.0 || condensed <= m_summer.abs() + 1e-9;
    let sd_total: f64 = element.layers.iter().map(sd_m).sum();
    let sd_req = 0.50 + az / 360.0;
    let mut builder = CheckResult::assess(
        format!("din4108-3.glaser.{}", element.id),
        "DIN 4108-3",
        ClauseId::new("DIN 4108-3", "§4", "4.3"),
        subject,
        loc("Glaser condensation mass", "Glaser-Kondensatmenge"),
    );
    builder = if condensed > 0.0 {
        builder.utilization(Quantity::new(QuantityKind::Mass, condensed), Quantity::new(QuantityKind::Mass, limit))
    } else {
        // No condensate: sd must meet orientation-dependent minimum (azimuth enters the limit).
        builder.minimum(Quantity::new(QuantityKind::Dimensionless, sd_total), Quantity::new(QuantityKind::Dimensionless, sd_req))
    };
    let mut builder = builder.annex(AnnexChoice::De)
    .explanation(loc(
        &format!(
            "Climate {climate_label} BC winter {t_ext_w:.0}°C/{:.0}%, summer {t_ext_s:.0}°C/{:.0}%: condensate {condensed:.3} kg/m² (limit {limit}), dry-out ok={dries}, net={net:.3}, sd={sd_total:.3}.",
            rh_ext_w * 100.0,
            rh_ext_s * 100.0,
        ),
        &format!(
            "Klima {climate_label} RB Winter {t_ext_w:.0}°C/{:.0}%, Sommer {t_ext_s:.0}°C/{:.0}%: Kondensat {condensed:.3} kg/m² (Grenze {limit}), Austrocknung ok={dries}, netto={net:.3}, sd={sd_total:.3}.",
            rh_ext_w * 100.0,
            rh_ext_s * 100.0,
        ),
    ));
    if condensed > 0.0 {
        builder = builder.status(if condensed > limit || !dries { semio_s_plugin_norm_compliance::CheckStatus::Fail } else { semio_s_plugin_norm_compliance::CheckStatus::Pass });
    } else {
        builder = builder.status(semio_s_plugin_norm_compliance::CheckStatus::Pass);
    }
    if condensed > limit || !dries {
        let interior_mu_idx = 0usize;
        let layer = &element.layers[interior_mu_idx];
        let sd_req = (layer.mu * layer.thickness_m * (condensed / limit).max(1.2)).max(layer.mu * layer.thickness_m);
        let mu_req = if layer.thickness_m > 0.0 { sd_req / layer.thickness_m } else { layer.mu };
        builder = builder.remedy(Remedy::at_least(
            SubjectRef::new(
                &element.id,
                layer_field_path(&element.id, &layer.id, "mu"),
                loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
            ),
            Quantity::new(QuantityKind::Dimensionless, layer.mu),
            Quantity::new(QuantityKind::Dimensionless, mu_req),
            loc(
                &format!(
                    "Raise vapour resistance of interior layer '{}' (μ from {:.1} to ≥ {:.1}, sd ≥ {:.2} m) or add a vapour barrier.",
                    layer.id, layer.mu, mu_req, sd_req
                ),
                &format!(
                    "Dampfdurchlasswiderstand der inneren Schicht '{}' erhöhen (μ von {:.1} auf ≥ {:.1}, sd ≥ {:.2} m) oder Dampfbremse ergänzen.",
                    layer.id, layer.mu, mu_req, sd_req
                ),
            ),
        ));
    }
    builder.build()
}
// #endregion 🔖️Part3
//#endregion 🔖️ComplianceHelpers
