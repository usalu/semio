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

/// 🌦️ The winter and summer exterior boundary conditions of the Glaser method for one element: temperature in °C and relative humidity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlaserBoundary {
    pub t_ext_w: f64,
    pub rh_ext_w: f64,
    pub t_ext_s: f64,
    pub rh_ext_s: f64,
}

/// 🌦️ The Glaser boundary conditions of `element` under `climate`: the design values shifted by the facade orientation; the single source of the check and of the profile diagram.
pub fn glaser_boundary(element: &EnvelopeElement, climate: ClimateZoneDe) -> GlaserBoundary {
    let t_adj = facade_temp_adjustment_k(element.orientation_deg);
    let az = element.orientation_deg.rem_euclid(360.0);
    GlaserBoundary {
        t_ext_w: glaser_winter_t_ext_c(climate) + t_adj - 0.05 * az,
        rh_ext_w: (GLASER_WINTER_RH_EXT + 0.0005 * az).clamp(0.50, 0.95),
        t_ext_s: glaser_summer_t_ext_c(climate, &element.kind) + 0.25 * t_adj + 0.02 * az,
        rh_ext_s: (GLASER_SUMMER_RH_EXT - 0.0003 * az).clamp(0.40, 0.90),
    }
}

/// 📈️ The vapour-pressure profile at the layer interfaces (interior to exterior) under the given boundary: equivalent air layer thickness `sd`, vapour pressure `p` and saturation pressure `p_sat` at each of the `layers + 1` interfaces.
#[derive(Clone, Debug, PartialEq)]
pub struct VapourProfile {
    pub sd_m: Vec<f64>,
    pub p_pa: Vec<f64>,
    pub p_sat_pa: Vec<f64>,
    pub t_c: Vec<f64>,
}

/// 📈️ The Glaser profile of a stack: the interface temperatures, vapour pressures by cumulative `sd` and the saturation pressures.
pub fn vapour_profile(layers: &[LayerDocument], r_si: f64, r_se: f64, t_int_c: f64, rh_int: f64, t_ext_c: f64, rh_ext: f64) -> VapourProfile {
    let temps = interface_temperatures_c(layers, r_si, r_se, t_int_c, t_ext_c);
    let p_int = rh_int * saturation_vapor_pressure_pa(t_int_c);
    let p_ext = rh_ext * saturation_vapor_pressure_pa(t_ext_c);
    let total: f64 = layers.iter().map(sd_m).sum();
    let mut sd = vec![0.0];
    for layer in layers {
        sd.push(sd.last().copied().unwrap_or(0.0) + sd_m(layer));
    }
    let p: Vec<f64> = sd.iter().map(|s| if total > 0.0 { p_int + (p_ext - p_int) * s / total } else { p_int }).collect();
    let t_c: Vec<f64> = (0..=layers.len()).map(|i| temps[(i + 1).min(temps.len() - 1)]).collect();
    let p_sat_pa = t_c.iter().map(|t| saturation_vapor_pressure_pa(*t)).collect();
    VapourProfile { sd_m: sd, p_pa: p, p_sat_pa, t_c }
}

/// 💧️ Glaser mass that evaporates in the summer period [kg/m²]: at every interface that condenses in winter the vapour flux towards both boundaries, from the summer saturation pressure at that interface against the summer boundary pressures, summed over the drying hours (negative fluxes count zero).
pub fn glaser_evaporated_mass_kg_m2(layers: &[LayerDocument], r_si: f64, r_se: f64, t_int_c: f64, rh_int: f64, winter: (f64, f64), summer: (f64, f64), hours: f64) -> f64 {
    if layers.is_empty() {
        return 0.0;
    }
    let cold = vapour_profile(layers, r_si, r_se, t_int_c, rh_int, winter.0, winter.1);
    let warm = vapour_profile(layers, r_si, r_se, t_int_c, rh_int, summer.0, summer.1);
    let sd_total = warm.sd_m.last().copied().unwrap_or(0.0);
    if sd_total <= 0.0 {
        return 0.0;
    }
    let (p_inside, p_outside) = (warm.p_pa[0], warm.p_pa[warm.p_pa.len() - 1]);
    (0..cold.p_pa.len())
        .filter(|i| cold.p_pa[*i] > cold.p_sat_pa[*i])
        .map(|i| {
            let sd_left = warm.sd_m[i].max(0.1);
            let sd_right = (sd_total - warm.sd_m[i]).max(0.1);
            let flux = (warm.p_sat_pa[i] - p_inside).max(0.0) / sd_left + (warm.p_sat_pa[i] - p_outside).max(0.0) / sd_right;
            flux * 2.0e-10 * hours
        })
        .sum()
}

/// ✅️ DIN 4108-3 drying reserve: the evaporable mass of the summer period must cover the condensed mass of the winter period.
pub fn check_drying_reserve(element: &EnvelopeElement, t_int_c: f64, rh_int: f64, climate: ClimateZoneDe) -> CheckResult {
    let subject = SubjectRef::new(&element.id, entity_path("elements", &element.id), loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)));
    let title = loc("Glaser drying reserve", "Glaser-Trocknungsreserve");
    let id = format!("din4108-3.drying.{}", element.id);
    let clause = ClauseId::new("DIN 4108-3", "§4", "4.3");
    if element.kind == "window" || element.kind == "door" || element.layers.is_empty() {
        return CheckResult::assess(id, "DIN 4108-3", clause, subject, title).not_applicable(loc("No opaque stack for Glaser analysis.", "Kein opaker Aufbau für Glaser-Verfahren.")).annex(AnnexChoice::De).build();
    }
    let (r_si, r_se) = surface_resistances_inclined(&element.kind, &element.adjacent, element.inclination_deg);
    let t_int = if t_int_c > 0.0 { t_int_c } else { GLASER_INTERIOR_T_C };
    let rh_i = if rh_int > 0.0 { rh_int } else { GLASER_INTERIOR_RH };
    let boundary = glaser_boundary(element, climate);
    let hours = 90.0 * 24.0;
    let condensed = glaser_condensed_mass_kg_m2(&element.layers, r_si, r_se, t_int, rh_i, boundary.t_ext_w, boundary.rh_ext_w, hours).max(0.0);
    if condensed <= 0.0 {
        return CheckResult::assess(id, "DIN 4108-3", clause, subject, title).not_applicable(loc("No condensation in the winter period, no drying required.", "Kein Tauwasserausfall in der Winterperiode, keine Trocknung erforderlich.")).annex(AnnexChoice::De).build();
    }
    let evaporated = glaser_evaporated_mass_kg_m2(&element.layers, r_si, r_se, t_int, rh_i, (boundary.t_ext_w, boundary.rh_ext_w), (boundary.t_ext_s, boundary.rh_ext_s), hours);
    let mut builder = CheckResult::assess(id, "DIN 4108-3", clause, subject.clone(), title)
        .utilization(Quantity::new(QuantityKind::Mass, condensed), Quantity::new(QuantityKind::Mass, evaporated.max(1e-9)))
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!("Condensed {condensed:.3} kg/m² in winter, evaporable {evaporated:.3} kg/m² in summer."),
            &format!("Tauwasser {condensed:.3} kg/m² im Winter, verdunstbar {evaporated:.3} kg/m² im Sommer."),
        ))
        .status(if condensed <= evaporated { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail });
    if condensed > evaporated {
        let layer = &element.layers[0];
        let mu_req = layer.mu * (condensed / evaporated.max(1e-9)).max(1.2);
        builder = builder.remedy(Remedy::at_least(
            SubjectRef::new(&element.id, layer_field_path(&element.id, &layer.id, "mu"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
            Quantity::new(QuantityKind::Dimensionless, layer.mu),
            Quantity::new(QuantityKind::Dimensionless, mu_req),
            loc(
                &format!("Raise the vapour resistance of the interior layer '{}' to mu >= {mu_req:.1} so that less water condenses than can dry out.", layer.id),
                &format!("Dampfdiffusionswiderstand der inneren Schicht '{}' auf mu >= {mu_req:.1} erhöhen, damit weniger Tauwasser anfällt, als austrocknen kann.", layer.id),
            ),
        ));
    }
    builder.build()
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
    let GlaserBoundary { t_ext_w, rh_ext_w, t_ext_s, rh_ext_s } = glaser_boundary(element, climate);
    let az = element.orientation_deg.rem_euclid(360.0);
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
