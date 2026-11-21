//! 💧️ DIN 4108-3 of the compliance checks: the vapour pressure and the Glaser condensation checks. See [`super`].

//#region 🔖️ComplianceHelpers
// #region 🔖️Part3
use crate::*;

#[path = "📊️annex-a-period/🦀️.rs"]
mod annex_a_period;
pub use annex_a_period::*;

#[path = "📏️material-limits/🦀️.rs"]
pub mod material_limits;

#[path = "📇️catalogue/🦀️.rs"]
pub mod catalogue;

#[path = "📐️construction-cases/🦀️.rs"]
pub mod construction_cases;

/// 🌫️ Saturation vapour pressure [Pa] at `t_c` °C: Magnus form over water at and above 0 °C and over ice below (DIN 4108-3:2024-03 C.15 and C.16).
pub fn saturation_vapor_pressure_pa(t_c: f64) -> f64 {
    if t_c >= 0.0 {
        610.5 * (17.269 * t_c / (237.3 + t_c)).exp()
    } else {
        610.5 * (21.875 * t_c / (265.5 + t_c)).exp()
    }
}

/// 💨 Vapour diffusion coefficient of still air δ0 [kg/(m·s·Pa)] (DIN 4108-3:2024-03 C.8–C.11).
pub const DELTA_0_KG_M_S_PA: f64 = 2.0e-10;

/// 🛡️ Smallest equivalent air layer thickness [m] used as a divisor when a condensation plane lies on a boundary.
pub const GLASER_SD_FLOOR_M: f64 = 1.0e-3;

/// 🌦️ Temperature, relative humidity and duration for a specified vapour-pressure profile.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlaserPeriod {
    pub t_int_c: f64,
    pub rh_int: f64,
    pub t_ext_c: f64,
    pub rh_ext: f64,
    pub hours: f64,
}

/// ❄️ DIN 4108-3:2024-03 Annex A condensation-period air conditions over 90 days.
pub const GLASER_WINTER: GlaserPeriod = GlaserPeriod { t_int_c: 20.0, rh_int: 0.50, t_ext_c: -5.0, rh_ext: 0.80, hours: 2160.0 };

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
    let p_int = period.rh_int * saturation_vapor_pressure_pa(period.t_int_c);
    let p_ext = period.rh_ext * saturation_vapor_pressure_pa(period.t_ext_c);
    profile_with_boundary(layers, r_si, r_se, period, p_int, p_ext, layers.iter().map(sd_m))
}

fn profile_with_boundary(
    layers: &[LayerDocument],
    r_si: f64,
    r_se: f64,
    period: GlaserPeriod,
    p_int: f64,
    p_ext: f64,
    layer_sd: impl IntoIterator<Item = f64>,
) -> VapourProfile {
    let temps = interface_temperatures_c(layers, r_si, r_se, period.t_int_c, period.t_ext_c);
    let mut sd = vec![0.0];
    for thickness in layer_sd {
        let cumulative = sd.last().copied().unwrap_or(0.0);
        sd.push(cumulative + thickness);
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

/// ✅️ DIN 4108-3 period balance screening for the recorded envelope element.
pub fn check_period_balance(element: &EnvelopeElement, usage: &str, basis: catalogue::PeriodBasis) -> [CheckResult; 2] {
    use semio_s_plugin_norm_compliance::CheckStatus;
    let subject = SubjectRef::new(&element.id, entity_path("elements", &element.id), loc("Envelope element", "Hüllbauteil"));
    let condensed = CheckResult::assess(
        format!("din4108-3.glaser.{}", element.id), "DIN 4108-3:2024-03",
        ClauseId::new("DIN 4108-3", "5.2.2", "A.2.5"),
        subject.clone(), loc("Annex A condensation screening", "Tauwasser-Vorprüfung nach Anhang A"),
    ).annex(AnnexChoice::De);
    let drying = CheckResult::assess(
        format!("din4108-3.drying.{}", element.id), "DIN 4108-3:2024-03",
        ClauseId::new("DIN 4108-3", "5.2.2", "A.2.6"),
        subject, loc("Annex A drying capacity", "Verdunstungspotenzial nach Anhang A"),
    ).annex(AnnexChoice::De);
    if element.layers.is_empty() || matches!(element.kind.as_str(), "window" | "door") {
        let reason = loc("The element has no opaque layer stack.", "Für das Bauteil liegt kein opaker Schichtenaufbau vor.");
        return [condensed.not_applicable(reason.clone()).build(), drying.not_applicable(reason).build()];
    }
    if (usage != "residential" && basis.site_scope != Some(true))
        || element.adjacent != "exterior"
        || !matches!(element.kind.as_str(), "wall" | "roof")
    {
        let reason = loc(
            "Verify the room use and exposure for the period method under §5.2.1. Ground-contact elements require Annex D.",
            "Raumnutzung und Bauteilrandbedingungen für das Periodenverfahren nach 5.2.1 prüfen. Erdberührte Bauteile erfordern Anhang D.",
        );
        return [condensed.not_applicable(reason.clone()).build(), drying.not_applicable(reason).build()];
    }
    if basis.site_scope == Some(false) {
        let reason = loc(
            "The recorded site elevation or room conditions exclude the Annex A period method. Use Annex D.",
            "Erfasste Standorthöhe oder Raumzustand schließen das Periodenverfahren aus. Anhang D anwenden.",
        );
        return [condensed.not_applicable(reason.clone()).build(), drying.not_applicable(reason).build()];
    }
    let roof = element.kind == "roof";
    let exposure = if roof { basis.roof_exposure.unwrap_or(AnnexADryingExposure::ShadedOrLightRoof) } else { AnnexADryingExposure::WallOrAtticCeiling };
    let basis_confirmed = basis.site_scope == Some(true) && (!roof || basis.roof_exposure.is_some());
    let Some(balance) = annex_a_period_balance(&element.layers, exposure) else {
        let reason = loc(
            "Enter finite positive thicknesses, thermal conductivities and vapour resistance values for the layers.",
            "Endliche positive Schichtdicken, Wärmeleitfähigkeiten und Dampfdiffusionswiderstände eingeben.",
        );
        return [
            condensed.status(CheckStatus::UnverifiedBasis).explanation(reason.clone()).build(),
            drying.status(CheckStatus::UnverifiedBasis).explanation(reason).build(),
        ];
    };
    let mass = balance.condensed_kg_m2;
    let capacity = balance.evaporation_capacity_kg_m2;
    let wet_interfaces = balance.planes.iter().filter(|plane| plane.condensed_kg_m2 > 0.0).count();
    let mass_limits = material_limits::condensate_limits(mass, basis.materials);
    let condensation_status = match (mass_limits.complies, basis_confirmed) {
        (Some(true), true) => CheckStatus::Pass,
        (Some(false), true) => CheckStatus::Fail,
        (Some(_), false) if mass == 0.0 => CheckStatus::Info,
        _ => CheckStatus::UnverifiedBasis,
    };
    let limit = mass_limits.mass_limit_kg_m2;
    let mut explanation = format!("Calculated winter condensate {mass:.3} kg/m² at {wet_interfaces} interfaces. The mass limit is {limit:.1} kg/m².");
    let mut explanation_de = format!("Berechnetes Wintertauwasser {mass:.3} kg/m² an {wet_interfaces} Schichtgrenzen. Der Grenzwert beträgt {limit:.1} kg/m².");
    if mass > 0.0 {
        match basis.materials {
            Some(materials) => {
                let damage = if materials.materials_resist_damage { "verified" } else { "damage or loss of function identified" };
                let damage_de = if materials.materials_resist_damage { "nachgewiesen" } else { "Schaden oder Funktionsverlust festgestellt" };
                explanation.push_str(&format!(" Material durability: {damage}. Timber moisture increase {:.2}% (limit 5%), non-exempt wood products {:.2}% (limit 3%).", materials.timber_increase_pp, materials.wood_product_increase_pp));
                explanation_de.push_str(&format!(" Baustoffbeständigkeit: {damage_de}. Feuchtezunahme Holz {:.2} % (Grenze 5 %), nicht ausgenommene Holzwerkstoffe {:.2} % (Grenze 3 %).", materials.timber_increase_pp, materials.wood_product_increase_pp));
            }
            None => {
                explanation.push_str(" Record capillary absorption, material durability and moisture increases under §5.2.2.");
                explanation_de.push_str(" Kapillare Wasseraufnahme, Baustoffbeständigkeit und Feuchtezunahmen nach 5.2.2 erfassen.");
            }
        }
    }
    if !basis_confirmed {
        explanation.push_str(" Verify the site and room conditions and record roof exposure when applicable.");
        explanation_de.push_str(" Standort und Raumzustand prüfen und beim Dach die Randbedingung erfassen.");
    }
    let condensed = condensed.utilization(Quantity::new(QuantityKind::Mass, mass), Quantity::new(QuantityKind::Mass, limit))
        .status(condensation_status).explanation(loc(&explanation, &explanation_de));
    let drying = if mass <= 0.0 {
        drying.not_applicable(loc("The modelled winter period produces no condensate.", "Die modellierte Tauperiode ergibt keinen Tauwasserausfall."))
    } else {
        let status = match (capacity >= mass, basis_confirmed) {
            (true, true) => CheckStatus::Pass,
            (true, false) => CheckStatus::Info,
            (false, true) => CheckStatus::Fail,
            (false, false) => CheckStatus::UnverifiedBasis,
        };
        drying.utilization(Quantity::new(QuantityKind::Mass, mass), Quantity::new(QuantityKind::Mass, capacity.max(1e-9)))
            .status(status).explanation(loc(
                &format!("Potential evaporation {capacity:.3} kg/m² at {} Pa interface saturation, compared with {mass:.3} kg/m² condensed. The standard gives dark unshaded roofs a separate summer pressure.", exposure.pressure_pa()),
                &format!("Verdunstungspotenzial {capacity:.3} kg/m² bei {} Pa Sättigungsdampfdruck gegenüber {mass:.3} kg/m² Tauwasser. Für unverschattete dunkle Dächer gilt ein eigener Sommerwert.", exposure.pressure_pa()),
            ))
    };
    [condensed.build(), drying.build()]
}
// #endregion 🔖️Part3
//#endregion 🔖️ComplianceHelpers
