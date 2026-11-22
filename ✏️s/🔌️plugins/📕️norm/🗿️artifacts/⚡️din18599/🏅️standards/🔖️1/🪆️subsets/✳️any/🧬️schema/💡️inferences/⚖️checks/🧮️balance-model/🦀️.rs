//! 🧮️ DIN V 18599 balance model: usage profiles, zone transmission and ventilation, solar gains, utilization and the zone balances.

use super::*;
use semio_s_artifact_norm_din18599_model::POTSDAM_HORIZONTAL_IRRADIANCE_W_M2;

const RHO_CA: f64 = DIN_V_18599_1_RHO_CA;

/// ⚡️ GEG Annex 4 / DIN V 18599-10 primary energy factors f_p (non-renewable).
pub fn primary_energy_factor(carrier: &str) -> f64 {
    use geg_anlage4_primary_energy_factors as f;
    let code = annex_energy_carrier(carrier);
    f::ROWS.iter()
        .find(|row| row.carrier == code)
        .and_then(|row| row.factor)
        .unwrap_or(if code == "district_heating" { 0.7 } else { f::NATURAL_GAS })
}

fn annex_energy_carrier(carrier: &str) -> &str {
    match carrier {
        "gas" => "natural_gas",
        "oil" => "heating_oil",
        "heat_pump_electric" => "electricity",
        "wood" => "biomass",
        _ => carrier,
    }
}

/// 🔎 The supplied carrier supports an Annex 4 screening value without plant or network-specific evidence.
pub fn has_direct_primary_energy_factor(carrier: &str) -> bool {
    matches!(annex_energy_carrier(carrier), "heating_oil" | "natural_gas" | "lpg" | "hard_coal" | "lignite" | "biogas" | "bio_oil" | "biomass" | "electricity")
}

pub(super) fn uses_grid_electricity(carrier: &str) -> bool {
    annex_energy_carrier(carrier) == "electricity"
}

/// 🎛️ DIN V 18599-11 automation / BACS factor on final energy.
pub fn automation_factor(class: AutomationClass) -> f64 {
    use din_v_18599_11_automation_factor as f;
    match class {
        AutomationClass::A => f::CLASS_A,
        AutomationClass::B => f::CLASS_B,
        AutomationClass::C => f::CLASS_C,
        AutomationClass::D => f::CLASS_D,
    }
}

pub(super) fn adjacency_factor(a: Adjacency) -> f64 {
    use din_v_18599_2_adjacency_fx as f;
    match a {
        Adjacency::Outdoor => f::OUTDOOR,
        Adjacency::Ground => f::GROUND,
        Adjacency::Unheated | Adjacency::UnheatedAttic => f::UNHEATED,
        Adjacency::Heated => f::HEATED,
    }
}

pub(super) fn heat_transferring_envelope_area(elements: &[EnvelopeElement]) -> f64 {
    elements.iter().filter(|e| e.adjacency != Adjacency::Heated).map(|e| e.area_m2).sum()
}

fn transmission_coefficient(elements: &[EnvelopeElement], area_m2: f64, delta_u: f64, u: impl Fn(&EnvelopeElement) -> f64) -> f64 {
    elements
        .iter()
        .filter(|e| e.adjacency != Adjacency::Heated)
        .map(|e| adjacency_factor(e.adjacency) * u(e) * e.area_m2)
        .sum::<f64>() + delta_u * area_m2
}

/// 🧱 Transmission heat transfer coefficient H_T [W/K] from the heat-transferring envelope and thermal bridges.
pub fn transmission_loss_coefficient(doc: &Din18599Snapshot) -> f64 {
    transmission_coefficient(&doc.elements, heat_transferring_envelope_area(&doc.elements), doc.delta_u_wb_w_m2k, |e| e.u_value_w_m2k)
}

/// 🌬️ Ventilation heat transfer coefficient H_V [W/K] (DIN V 18599-6).
pub fn ventilation_loss_coefficient(doc: &Din18599Snapshot) -> f64 {
    zone_balances(doc, &doc.elements, doc.delta_u_wb_w_m2k).iter().map(|z| z.h_v).sum()
}

/// 🏠️ Specific transmission heat loss H′T [W/(m²·K)] through the heat-transferring envelope.
pub fn h_t_prime(doc: &Din18599Snapshot) -> f64 {
    let area_m2 = heat_transferring_envelope_area(&doc.elements);
    transmission_coefficient(&doc.elements, area_m2, doc.delta_u_wb_w_m2k, |e| e.u_value_w_m2k) / area_m2.max(1e-9)
}

/// 📏 GEG § 16 limit H′T for the same-geometry residential reference envelope under Annex 1.
pub fn geg_ht_prime_limit(doc: &Din18599Snapshot) -> Option<f64> {
    let area_m2 = heat_transferring_envelope_area(&doc.elements);
    if !area_m2.is_finite() || area_m2 <= 0.0 {
        return None;
    }
    let reference_ht = transmission_coefficient(&doc.elements, area_m2, GEG_ANLAGE1_DELTA_U_WB_REF, reference_u);
    Some(GEG_SECTION_16_HT_PRIME_RATIO * reference_ht / area_m2)
}

/// 🪟️ GEG Annex 1 reference transmittance for a represented residential envelope component.
pub fn reference_u(element: &EnvelopeElement) -> f64 {
    use geg_anlage1_reference_u as u;
    match element.kind {
        ElementKind::Wall | ElementKind::Floor if element.adjacency == Adjacency::Outdoor => u::WALL_AND_FLOOR_TO_OUTDOORS,
        ElementKind::Wall | ElementKind::Floor | ElementKind::GroundSlabPerimeter | ElementKind::GroundSlabInterior => u::WALL_AND_FLOOR_TO_GROUND_OR_UNHEATED,
        ElementKind::Roof => u::ROOF,
        ElementKind::Window | ElementKind::GlazedDoor | ElementKind::CurtainWall => u::WINDOW,
        ElementKind::RoofGlazing | ElementKind::RoofWindow | ElementKind::RooflightStrip => u::ROOF_GLAZING,
        ElementKind::RoofDome => u::ROOF_DOME,
        ElementKind::Door => u::DOOR,
    }
}

/// 🏷️ DIN V 18599-10 profile row for a zone usage profile.
pub struct UsageProfileRow {
    pub fan_hours_a: f64,
    pub lighting_hours_a: f64,
    pub outdoor_air_change_1_h: f64,
    pub lighting_lpd_limit_w_m2: f64,
}

/// 📋 Resolve DIN V 18599-10 defaults for a typed zone usage profile.
pub fn usage_profile_row(profile: UsageProfile) -> UsageProfileRow {
    match profile {
        UsageProfile::WFH => UsageProfileRow {
            fan_hours_a: din_v_18599_10_fan_hours::RESIDENTIAL_WFH,
            lighting_hours_a: din_v_18599_10_lighting_hours::WFH,
            outdoor_air_change_1_h: din_v_18599_10_outdoor_air_change::WFH,
            lighting_lpd_limit_w_m2: din_v_18599_4_lighting_power_density::RESIDENTIAL,
        },
        UsageProfile::Office => UsageProfileRow {
            fan_hours_a: din_v_18599_10_fan_hours::OFFICE,
            lighting_hours_a: din_v_18599_10_lighting_hours::OFFICE,
            outdoor_air_change_1_h: din_v_18599_10_outdoor_air_change::OFFICE,
            lighting_lpd_limit_w_m2: din_v_18599_4_lighting_power_density::OFFICE,
        },
        UsageProfile::School => UsageProfileRow {
            fan_hours_a: din_v_18599_10_fan_hours::SCHOOL,
            lighting_hours_a: din_v_18599_10_lighting_hours::SCHOOL,
            outdoor_air_change_1_h: din_v_18599_10_outdoor_air_change::SCHOOL,
            lighting_lpd_limit_w_m2: din_v_18599_4_lighting_power_density::SCHOOL,
        },
    }
}

pub(super) fn occupants(doc: &Din18599Snapshot) -> u32 {
    doc.zones.iter().map(|z| z.occupants).sum()
}

fn elements_for_zone<'a>(elements: &'a [EnvelopeElement], zone_id: &str) -> Vec<&'a EnvelopeElement> {
    elements.iter().filter(|e| e.zone_id == zone_id).collect()
}

/// 🧮 Per-zone monthly balance (DIN V 18599-1/-2).
#[derive(Clone, Debug)]
pub struct ZoneBalance {
    pub zone_id: String,
    pub h_t: f64,
    pub h_v: f64,
    pub q_h_nd_kwh: f64,
    pub q_c_nd_kwh: f64,
    pub q_l_kwh: f64,
    pub area_m2: f64,
    pub volume_m3: f64,
    pub months: Vec<HeatingMonth>,
}

fn zone_transmission(elements: &[&EnvelopeElement], thermal_bridge_w_k: f64) -> f64 {
    let sum_ua: f64 = elements.iter().filter(|e| e.adjacency != Adjacency::Heated).map(|e| adjacency_factor(e.adjacency) * e.u_value_w_m2k * e.area_m2).sum();
    sum_ua + thermal_bridge_w_k
}

fn zone_ventilation_hv(zone: &ThermalZone, doc: &Din18599Snapshot, volume_share: f64) -> f64 {
    let row = usage_profile_row(zone.usage_profile);
    let v_mech_zone = doc.ventilation.airflow_m3_h * volume_share;
    let v_profile = row.outdoor_air_change_1_h * zone.volume_m3;
    let v_mech = v_mech_zone + v_profile;
    let v_inf = DIN_V_18599_2_INFILTRATION_N_INF * doc.heated_volume_m3.max(1e-9) * volume_share;
    let eta = doc.ventilation.heat_recovery_eta.clamp(0.0, 1.0);
    RHO_CA * (v_mech * (1.0 - eta) + v_inf)
}

/// 📅 One month of the DIN V 18599-2 balance of a zone: transmission and ventilation losses, internal and solar gains, utilisation factor and the resulting net heating and cooling demand, all in kWh.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatingMonth {
    pub q_t_kwh: f64,
    pub q_v_kwh: f64,
    pub q_i_kwh: f64,
    pub q_s_kwh: f64,
    pub eta_utilisation: f64,
    pub q_h_kwh: f64,
    pub q_c_kwh: f64,
}

/// 📅 The twelve monthly balances of `zone` against `climate`.
fn heating_months(doc: &Din18599Snapshot, zone: &ThermalZone, z_elems: &[&EnvelopeElement], h_t: f64, h_v: f64) -> Vec<HeatingMonth> {
    let climate = &doc.climate;
    let a = utilization_parameter(zone, h_t + h_v);
    let internal_gains = internal_gains_w_m2(doc, zone);
    (0..12)
        .map(|m| {
            let hours = 24.0 * DIN_V_18599_1_DAYS_PER_MONTH[m];
            let te = climate.theta_e_c[m];
            let dt = (zone.theta_i_heat_c - te).max(0.0);
            let q_t = h_t * dt * hours / 1000.0;
            let q_v = h_v * dt * hours / 1000.0;
            let q_i = internal_gains * zone.area_m2 * hours / 1000.0;
            let q_s: f64 = z_elems.iter().filter(|e| e.kind.transmits_solar_radiation() && e.g_value > 0.0).map(|e| solar_irradiance_for_element(climate, m, e) * e.area_m2 * e.g_value * e.fc.clamp(0.0, 1.0) * hours / 1000.0).sum();
            let loss = q_t + q_v;
            let gain = q_i + q_s;
            let gamma = if loss <= 1e-9 { 1e9 } else { gain / loss };
            let eta = heating_utilization(gamma, a);
            let loss_c = (h_t + h_v) * (zone.theta_i_cool_c - te).max(0.0) * hours / 1000.0;
            let over_c = (h_t + h_v) * (te - zone.theta_i_cool_c).max(0.0) * hours / 1000.0;
            let gamma_c = if loss_c <= 1e-9 { 1e9 } else { gain / loss_c };
            let q_c = (gain - cooling_utilization(gamma_c, a) * loss_c).max(0.0) + over_c;
            HeatingMonth { q_t_kwh: q_t, q_v_kwh: q_v, q_i_kwh: q_i, q_s_kwh: q_s, eta_utilisation: eta, q_h_kwh: (loss - eta * gain).max(0.0), q_c_kwh: q_c }
        })
        .collect()
}

/// 🧮 Zone-level balances; building totals aggregate these.
pub fn zone_balances(doc: &Din18599Snapshot, elements: &[EnvelopeElement], delta_u: f64) -> Vec<ZoneBalance> {
    let a_env_total = heat_transferring_envelope_area(elements);
    zone_balances_with_bridges(doc, elements, |_, _, share| delta_u * a_env_total * share)
}

pub(super) fn zone_balances_with_bridges(
    doc: &Din18599Snapshot,
    elements: &[EnvelopeElement],
    thermal_bridge_w_k: impl Fn(&ThermalZone, &[&EnvelopeElement], f64) -> f64,
) -> Vec<ZoneBalance> {
    let v_net: f64 = doc.zones.iter().map(|z| z.volume_m3).sum::<f64>().max(1e-9);
    let mut out = Vec::with_capacity(doc.zones.len());
    for zone in &doc.zones {
        let z_elems = elements_for_zone(elements, &zone.id);
        let share = zone.volume_m3 / v_net;
        let h_t = zone_transmission(&z_elems, thermal_bridge_w_k(zone, &z_elems, share));
        let h_v = zone_ventilation_hv(zone, doc, share);
        let row = usage_profile_row(zone.usage_profile);
        let months = heating_months(doc, zone, &z_elems, h_t, h_v);
        let q_h: f64 = months.iter().map(|m| m.q_h_kwh).sum();
        let q_c: f64 = months.iter().map(|m| m.q_c_kwh).sum();
        let q_l = zone.lighting_power_w_m2 * zone.area_m2 * row.lighting_hours_a * doc.lighting.control_factor / 1000.0;
        out.push(ZoneBalance {
            zone_id: zone.id.clone(),
            h_t,
            h_v,
            q_h_nd_kwh: q_h,
            q_c_nd_kwh: q_c,
            q_l_kwh: q_l,
            area_m2: zone.area_m2,
            volume_m3: zone.volume_m3,
            months,
        });
    }
    out
}

/// 📅 The monthly heating balance of every zone of the actual building, by zone id.
pub fn zone_heating_months(doc: &Din18599Snapshot) -> Vec<(String, Vec<HeatingMonth>)> {
    zone_balances(doc, &doc.elements, doc.delta_u_wb_w_m2k)
        .into_iter()
        .map(|balance| (balance.zone_id, balance.months))
        .collect()
}

fn solar_irradiance_for_element(climate: &MonthlyClimate, month: usize, element: &EnvelopeElement) -> f64 {
    let horizontal_reference = POTSDAM_HORIZONTAL_IRRADIANCE_W_M2[month];
    potsdam_irradiance_w_m2(month, element.orientation_deg, element.tilt_deg)
        .map_or(0.0, |plane| climate.g_h_w_m2[month] * (plane / horizontal_reference))
}

/// ☀️ Monthly solar gains Q_S,m [kWh] from fenestration (DIN V 18599-2).
pub fn solar_gains_month_kwh(doc: &Din18599Snapshot, climate: &MonthlyClimate, month: usize) -> f64 {
    doc.elements
        .iter()
        .filter(|e| e.kind.transmits_solar_radiation() && e.g_value > 0.0)
        .map(|e| solar_irradiance_for_element(climate, month, e) * e.area_m2 * e.g_value * e.fc.clamp(0.0, 1.0) * 24.0 * DIN_V_18599_1_DAYS_PER_MONTH[month] / 1000.0)
        .sum()
}

/// 🧱 Effective heat storage capacity C_wirk [Wh/K] of a zone: the class value of DIN V 18599-2 per net floor area, or the explicit value for the custom class.
pub fn zone_heat_capacity_wh_k(zone: &ThermalZone) -> f64 {
    use din_v_18599_2_c_wirk_wh_m2k as c;
    zone.area_m2 * match zone.thermal_mass_class {
        ThermalMassClass::Light => c::LIGHT,
        ThermalMassClass::Medium => c::MEDIUM,
        ThermalMassClass::Heavy => c::HEAVY,
        ThermalMassClass::Custom if zone.c_wirk_wh_m2k > 0.0 => zone.c_wirk_wh_m2k,
        ThermalMassClass::Custom => c::MEDIUM,
    }
}

/// ⏲️ Numerical parameter a = a0 + τ/τ0 of the utilisation factor, with the time constant τ = C_wirk / H [h] (DIN V 18599-2, EN ISO 13790 12.2.1.1).
pub fn utilization_parameter(zone: &ThermalZone, h_w_k: f64) -> f64 {
    let tau_h = zone_heat_capacity_wh_k(zone) / h_w_k.max(1e-9);
    DIN_V_18599_2_UTILIZATION_A0 + tau_h / DIN_V_18599_2_UTILIZATION_TAU0_H
}

/// 🔥 Gain utilisation factor of the heating balance, η = (1 − γ^a) / (1 − γ^(a+1)), a/(a+1) at γ = 1 (EN ISO 13790 12.2.1.1).
pub fn heating_utilization(gamma: f64, a: f64) -> f64 {
    if gamma.abs() < 1e-9 {
        1.0
    } else if (gamma - 1.0).abs() < 1e-9 {
        a / (a + 1.0)
    } else if gamma < 1.0 {
        (1.0 - gamma.powf(a)) / (1.0 - gamma.powf(a + 1.0))
    } else {
        (1.0 / gamma - gamma.powf(-(a + 1.0))) / (1.0 - gamma.powf(-(a + 1.0)))
    }
}

/// ❄️ Loss utilisation factor of the cooling balance, η = (1 − γ^−a) / (1 − γ^−(a+1)), a/(a+1) at γ = 1 (EN ISO 13790 12.2.2).
pub fn cooling_utilization(gamma: f64, a: f64) -> f64 {
    if gamma.abs() < 1e-9 {
        1.0
    } else if (gamma - 1.0).abs() < 1e-9 {
        a / (a + 1.0)
    } else if gamma > 1.0 {
        (1.0 - gamma.powf(-a)) / (1.0 - gamma.powf(-(a + 1.0)))
    } else {
        (gamma - gamma.powf(a + 1.0)) / (1.0 - gamma.powf(a + 1.0))
    }
}

/// ☀️ Modelled renewable electricity generation, eligible electricity demand and bounded credit for a month.
#[derive(Clone, Copy, Debug)]
pub struct RenewableElectricityMonth {
    pub generation_kwh: f64,
    pub eligible_electricity_kwh: f64,
    pub credit_kwh: f64,
}

/// 🧮 Derived monthly / annual balance quantities.
#[derive(Clone, Debug)]
pub struct BalanceDerived {
    pub h_t: f64,
    pub h_v: f64,
    pub h_t_prime: f64,
    pub a_over_v_e: f64,
    pub q_h_nd_kwh: f64,
    pub q_c_nd_kwh: f64,
    pub q_w_useful_kwh: f64,
    pub q_w_kwh: f64,
    pub q_l_kwh: f64,
    pub q_f_heat_kwh: f64,
    pub q_f_dhw_kwh: f64,
    pub q_f_cool_kwh: f64,
    pub q_f_light_kwh: f64,
    pub q_f_aux_kwh: f64,
    pub q_pv_kwh: f64,
    pub renewable_electricity_months: [RenewableElectricityMonth; 12],
    pub q_p_kwh: f64,
    pub q_p_ref_kwh: f64,
    pub q_p_limit_kwh: f64,
    pub q_h_ref_kwh: f64,
}

pub(super) fn heating_system_efficiency(doc: &Din18599Snapshot) -> f64 {
    (doc.heating.generation_efficiency * doc.heating.distribution_efficiency * doc.heating.storage_efficiency * doc.heating.transfer_efficiency).max(DIN_V_18599_5_ETA_SYS_FLOOR)
}
