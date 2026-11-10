//! 💧️ DIN 4108-3 of the compliance checks: the vapour pressure and the Glaser condensation checks. See [`super`].

//#region 🔖️ComplianceHelpers
// #region 🔖️Part3
use crate::*;

/// 🌫️ Saturation vapour pressure [Pa] at `t_c` °C: Magnus form over water at and above 0 °C and over ice below (EN ISO 13788 Eq. 4 and 5, used by DIN 4108-3:2018, see <https://www.iso.org/standard/51615.html>); constants `unverified` against the DIN text.
pub fn saturation_vapor_pressure_pa(t_c: f64) -> f64 {
    if t_c >= 0.0 {
        610.5 * (17.269 * t_c / (237.3 + t_c)).exp()
    } else {
        610.5 * (21.875 * t_c / (265.5 + t_c)).exp()
    }
}

/// 💨 Vapour diffusion coefficient of still air δ0 [kg/(m·s·Pa)] (EN ISO 13788; `unverified` against DIN 4108-3:2018).
pub const DELTA_0_KG_M_S_PA: f64 = 2.0e-10;

/// 🛡️ Smallest equivalent air layer thickness [m] used as a divisor when a condensation plane lies on a boundary.
pub const GLASER_SD_FLOOR_M: f64 = 1.0e-3;

/// 🌦️ One climate period of the Glaser method (DIN 4108-3:2018 Annex A): interior and exterior air state and the duration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlaserPeriod {
    pub t_int_c: f64,
    pub rh_int: f64,
    pub t_ext_c: f64,
    pub rh_ext: f64,
    pub hours: f64,
}

/// ❄️ DIN 4108-3:2018 Annex A condensation period: interior 20 °C / 50 %, exterior −5 °C / 80 %, 90 days (`unverified` duration).
pub const GLASER_WINTER: GlaserPeriod = GlaserPeriod { t_int_c: 20.0, rh_int: 0.50, t_ext_c: -5.0, rh_ext: 0.80, hours: 2160.0 };

/// ☀️ DIN 4108-3:2018 Annex A evaporation period: 12 °C / 70 % on both sides, 90 days (`unverified` duration).
pub const GLASER_SUMMER: GlaserPeriod = GlaserPeriod { t_int_c: 12.0, rh_int: 0.70, t_ext_c: 12.0, rh_ext: 0.70, hours: 2160.0 };

/// 💨 Vapour diffusion resistance Z = s_d/δ0 [m²·s·Pa/kg] of an equivalent air layer thickness `sd_m`.
pub fn vapour_diffusion_resistance_z(sd_m: f64) -> f64 {
    sd_m / DELTA_0_KG_M_S_PA
}

/// 💨 Vapour diffusion flux g = (p_a − p_b)/Z [kg/(m²·s)] across a resistance `z`.
pub fn vapour_flux_kg_m2_s(p_a: f64, p_b: f64, z: f64) -> f64 {
    (p_a - p_b) / z
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

/// 📈️ The Glaser profile at the layer interfaces (interior to exterior): equivalent air layer thickness `sd`, vapour pressure `p` with the condensation planes held at saturation, saturation pressure `p_sat`, temperature and the interface indices of the condensation planes.
#[derive(Clone, Debug, PartialEq)]
pub struct VapourProfile {
    pub sd_m: Vec<f64>,
    pub p_pa: Vec<f64>,
    pub p_sat_pa: Vec<f64>,
    pub t_c: Vec<f64>,
    pub planes: Vec<usize>,
}

fn piecewise_pressures(sd: &[f64], p_int: f64, p_ext: f64, p_sat: &[f64], planes: &[usize]) -> Vec<f64> {
    let last = sd.len() - 1;
    let mut nodes = vec![0];
    nodes.extend(planes.iter().copied());
    nodes.push(last);
    nodes.sort_unstable();
    let fixed = |k: usize| if k == 0 { p_int } else if k == last { p_ext } else { p_sat[k] };
    let mut p = vec![0.0; sd.len()];
    for pair in nodes.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let span = sd[b] - sd[a];
        for k in a..=b {
            let t = if span > 0.0 { (sd[k] - sd[a]) / span } else { 0.0 };
            p[k] = fixed(a) + (fixed(b) - fixed(a)) * t;
        }
    }
    p
}

/// 📈️ The Glaser profile of a stack under `period`: interface temperatures, saturation pressures and the vapour pressure with condensation planes added one by one at the interface of largest excess (EN ISO 13788 Glaser iteration).
pub fn vapour_profile(layers: &[LayerDocument], r_si: f64, r_se: f64, period: GlaserPeriod) -> VapourProfile {
    let temps = interface_temperatures_c(layers, r_si, r_se, period.t_int_c, period.t_ext_c);
    let p_int = period.rh_int * saturation_vapor_pressure_pa(period.t_int_c);
    let p_ext = period.rh_ext * saturation_vapor_pressure_pa(period.t_ext_c);
    let mut sd = vec![0.0];
    for layer in layers {
        sd.push(sd.last().copied().unwrap_or(0.0) + sd_m(layer));
    }
    let t_c: Vec<f64> = (0..=layers.len()).map(|i| temps[(i + 1).min(temps.len() - 1)]).collect();
    let p_sat_pa: Vec<f64> = t_c.iter().map(|t| saturation_vapor_pressure_pa(*t)).collect();
    let mut planes: Vec<usize> = Vec::new();
    loop {
        let p = piecewise_pressures(&sd, p_int, p_ext, &p_sat_pa, &planes);
        let worst = (1..layers.len())
            .filter(|k| !planes.contains(k) && p[*k] > p_sat_pa[*k])
            .max_by(|a, b| (p[*a] - p_sat_pa[*a]).total_cmp(&(p[*b] - p_sat_pa[*b])));
        match worst {
            Some(k) => planes.push(k),
            None => {
                planes.sort_unstable();
                return VapourProfile { sd_m: sd, p_pa: p, p_sat_pa, t_c, planes };
            }
        }
    }
}

fn plane_neighbours(profile: &VapourProfile, plane: usize) -> (usize, usize) {
    let before = profile.planes.iter().copied().filter(|k| *k < plane).max().unwrap_or(0);
    let after = profile.planes.iter().copied().filter(|k| *k > plane).min().unwrap_or(profile.sd_m.len() - 1);
    (before, after)
}

/// 💧️ Glaser condensed mass of the condensation period [kg/m²]: at every plane the flux in minus the flux out, g = (p − p_c)/Z, over the period duration; planes are summed and the total is not negative.
pub fn glaser_condensed_mass_kg_m2(layers: &[LayerDocument], r_si: f64, r_se: f64, period: GlaserPeriod) -> f64 {
    if layers.is_empty() {
        return 0.0;
    }
    let profile = vapour_profile(layers, r_si, r_se, period);
    let seconds = period.hours * 3600.0;
    let mass: f64 = profile
        .planes
        .iter()
        .map(|plane| {
            let (a, b) = plane_neighbours(&profile, *plane);
            let z_in = vapour_diffusion_resistance_z((profile.sd_m[*plane] - profile.sd_m[a]).max(GLASER_SD_FLOOR_M));
            let z_out = vapour_diffusion_resistance_z((profile.sd_m[b] - profile.sd_m[*plane]).max(GLASER_SD_FLOOR_M));
            (vapour_flux_kg_m2_s(profile.p_pa[a], profile.p_sat_pa[*plane], z_in) + vapour_flux_kg_m2_s(profile.p_pa[b], profile.p_sat_pa[*plane], z_out)) * seconds
        })
        .sum();
    mass.max(0.0)
}

/// 💧️ Glaser mass that evaporates in the evaporation period [kg/m²]: the condensation planes of `winter` stay at the saturation pressure of `summer` and dry towards both neighbours, g = (p_sat − p)/Z, over the summer duration; negative fluxes count zero.
pub fn glaser_evaporated_mass_kg_m2(layers: &[LayerDocument], r_si: f64, r_se: f64, winter: GlaserPeriod, summer: GlaserPeriod) -> f64 {
    if layers.is_empty() {
        return 0.0;
    }
    let cold = vapour_profile(layers, r_si, r_se, winter);
    let warm = vapour_profile(layers, r_si, r_se, summer);
    let last = cold.sd_m.len() - 1;
    let p_boundary = |k: usize| if k == 0 { summer.rh_int * saturation_vapor_pressure_pa(summer.t_int_c) } else { summer.rh_ext * saturation_vapor_pressure_pa(summer.t_ext_c) };
    let fixed = |k: usize| if k == 0 || k == last { p_boundary(k) } else { warm.p_sat_pa[k] };
    let seconds = summer.hours * 3600.0;
    cold.planes
        .iter()
        .map(|plane| {
            let (a, b) = plane_neighbours(&cold, *plane);
            let z_in = vapour_diffusion_resistance_z((cold.sd_m[*plane] - cold.sd_m[a]).max(GLASER_SD_FLOOR_M));
            let z_out = vapour_diffusion_resistance_z((cold.sd_m[b] - cold.sd_m[*plane]).max(GLASER_SD_FLOOR_M));
            (vapour_flux_kg_m2_s(warm.p_sat_pa[*plane], fixed(a), z_in).max(0.0) + vapour_flux_kg_m2_s(warm.p_sat_pa[*plane], fixed(b), z_out).max(0.0)) * seconds
        })
        .sum()
}

/// ✅️ DIN 4108-3 drying reserve: the evaporable mass of the evaporation period must cover the condensed mass of the condensation period, both under the Annex A climate.
pub fn check_drying_reserve(element: &EnvelopeElement) -> CheckResult {
    let subject = SubjectRef::new(&element.id, entity_path("elements", &element.id), loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)));
    let title = loc("Glaser drying reserve", "Glaser-Trocknungsreserve");
    let id = format!("din4108-3.drying.{}", element.id);
    let clause = ClauseId::new("DIN 4108-3", "§4", "4.3");
    if element.kind == "window" || element.kind == "door" || element.layers.is_empty() {
        return CheckResult::assess(id, "DIN 4108-3", clause, subject, title).not_applicable(loc("No opaque stack for Glaser analysis.", "Kein opaker Aufbau für Glaser-Verfahren.")).annex(AnnexChoice::De).build();
    }
    let (r_si, r_se) = surface_resistances_inclined(&element.kind, &element.adjacent, element.inclination_deg);
    let condensed = glaser_condensed_mass_kg_m2(&element.layers, r_si, r_se, GLASER_WINTER);
    if condensed <= 0.0 {
        return CheckResult::assess(id, "DIN 4108-3", clause, subject, title).not_applicable(loc("No condensation in the condensation period, no drying required.", "Kein Tauwasserausfall in der Tauperiode, keine Trocknung erforderlich.")).annex(AnnexChoice::De).build();
    }
    let evaporated = glaser_evaporated_mass_kg_m2(&element.layers, r_si, r_se, GLASER_WINTER, GLASER_SUMMER);
    let mut builder = CheckResult::assess(id, "DIN 4108-3", clause, subject.clone(), title)
        .utilization(Quantity::new(QuantityKind::Mass, condensed), Quantity::new(QuantityKind::Mass, evaporated.max(1e-9)))
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!("Condensed {condensed:.3} kg/m² in the condensation period, evaporable {evaporated:.3} kg/m² in the evaporation period."),
            &format!("Tauwasser {condensed:.3} kg/m² in der Tauperiode, verdunstbar {evaporated:.3} kg/m² in der Verdunstungsperiode."),
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

/// ✅️ DIN 4108-3 Glaser / periodic condensation check under the Annex A climate (condensation period 20 °C/50 % to −5 °C/80 %, evaporation period 12 °C/70 % on both sides).
pub fn check_glaser(element: &EnvelopeElement) -> CheckResult {
    let subject = SubjectRef::new(
        &element.id,
        entity_path("elements", &element.id),
        loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
    );
    let title = loc("Glaser condensation mass", "Glaser-Kondensatmenge");
    let id = format!("din4108-3.glaser.{}", element.id);
    let clause = ClauseId::new("DIN 4108-3", "§4", "4.3");
    if element.kind == "window" || element.kind == "door" || element.layers.is_empty() {
        return CheckResult::assess(id, "DIN 4108-3", clause, subject, title)
            .not_applicable(loc("No opaque stack for Glaser analysis.", "Kein opaker Aufbau für Glaser-Verfahren."))
            .annex(AnnexChoice::De)
            .build();
    }
    let (r_si, r_se) = surface_resistances_inclined(&element.kind, &element.adjacent, element.inclination_deg);
    let (winter, summer) = (GLASER_WINTER, GLASER_SUMMER);
    let condensed = glaser_condensed_mass_kg_m2(&element.layers, r_si, r_se, winter);
    let evaporated = glaser_evaporated_mass_kg_m2(&element.layers, r_si, r_se, winter, summer);
    let wood = element.layers.iter().any(|l| l.material_id.contains("wood") || l.material_id.contains("timber") || l.material_id.contains("softwood"));
    let mass = part_2::surface_mass_kg_m2(&element.layers);
    let limit = if wood || (mass > 0.0 && mass <= 100.0) { GLASER_MASS_LIMIT_WOOD } else { GLASER_MASS_LIMIT };
    let dries = condensed <= evaporated;
    let sd_total: f64 = element.layers.iter().map(sd_m).sum();
    let mut builder = CheckResult::assess(id, "DIN 4108-3", clause, subject, title)
        .utilization(Quantity::new(QuantityKind::Mass, condensed), Quantity::new(QuantityKind::Mass, limit))
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!(
                "Annex A climate, condensation period {:.0}°C/{:.0}% to {:.0}°C/{:.0}% for {:.0} h, evaporation period {:.0}°C/{:.0}% for {:.0} h: condensate {condensed:.3} kg/m² (limit {limit}), evaporable {evaporated:.3} kg/m², dries={dries}, sd={sd_total:.3} m.",
                winter.t_int_c, winter.rh_int * 100.0, winter.t_ext_c, winter.rh_ext * 100.0, winter.hours, summer.t_ext_c, summer.rh_ext * 100.0, summer.hours
            ),
            &format!(
                "Klima nach Anhang A, Tauperiode {:.0}°C/{:.0}% bis {:.0}°C/{:.0}% für {:.0} h, Verdunstungsperiode {:.0}°C/{:.0}% für {:.0} h: Tauwasser {condensed:.3} kg/m² (Grenze {limit}), verdunstbar {evaporated:.3} kg/m², trocknet={dries}, sd={sd_total:.3} m.",
                winter.t_int_c, winter.rh_int * 100.0, winter.t_ext_c, winter.rh_ext * 100.0, winter.hours, summer.t_ext_c, summer.rh_ext * 100.0, summer.hours
            ),
        ));
    builder = builder.status(if condensed > limit || !dries { semio_s_plugin_norm_compliance::CheckStatus::Fail } else { semio_s_plugin_norm_compliance::CheckStatus::Pass });
    if condensed > limit || !dries {
        let layer = &element.layers[0];
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
