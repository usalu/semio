//! 🧮️ DIN V 18599 balance model: usage profiles, zone transmission and ventilation, solar gains, utilization and the zone balances.

use super::*;

const HOURS_PER_MONTH: f64 = DIN_V_18599_1_HOURS_PER_MONTH;
const RHO_CA: f64 = DIN_V_18599_1_RHO_CA;

/// ⚡️ GEG Annex 4 / DIN V 18599-10 primary energy factors f_p (non-renewable).
pub fn primary_energy_factor(carrier: &str) -> f64 {
    use geg_anlage4_primary_energy_factors as f;
    match carrier {
        "natural_gas" | "gas" => f::NATURAL_GAS,
        "heating_oil" | "oil" => f::HEATING_OIL,
        "electricity" | "heat_pump_electric" => f::ELECTRICITY_GRID,
        "district_heating" => f::DISTRICT_HEATING,
        "biomass" | "wood" => f::BIOMASS,
        _ => f::NATURAL_GAS,
    }
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
        Adjacency::Unheated => f::UNHEATED,
        Adjacency::Heated => f::HEATED,
    }
}

/// 🧱 Transmission heat transfer coefficient H_T [W/K] from envelope + ΔU_WB surcharge.
pub fn transmission_loss_coefficient(doc: &Din18599Snapshot) -> f64 {
    let sum_ua: f64 = doc.elements.iter().map(|e| adjacency_factor(e.adjacency) * e.u_value_w_m2k * e.area_m2).sum();
    let a_env: f64 = doc.elements.iter().map(|e| e.area_m2).sum();
    sum_ua + doc.delta_u_wb_w_m2k * a_env
}

/// 🌬️ Ventilation heat transfer coefficient H_V [W/K] (DIN V 18599-6).
pub fn ventilation_loss_coefficient(doc: &Din18599Snapshot) -> f64 {
    zone_balances(doc, &doc.elements, doc.delta_u_wb_w_m2k).iter().map(|z| z.h_v).sum()
}

/// 🏠️ Specific transmission heat loss H′T [W/(m²·K)] — H_T / A with A = heat-transferring envelope area (GEG).
pub fn h_t_prime(doc: &Din18599Snapshot) -> f64 {
    let a_env: f64 = doc.elements.iter().map(|e| e.area_m2).sum::<f64>().max(1e-9);
    transmission_loss_coefficient(doc) / a_env
}

/// 📏 GEG §16 / Anlage 2 H′T limit for residential by attachment and A_N.
pub fn geg_ht_prime_limit(attachment: Attachment, a_n: f64) -> f64 {
    use geg_anlage2_ht_prime_limits as lim;
    let small = a_n <= lim::AN_THRESHOLD_M2;
    match attachment {
        Attachment::Detached => if small { lim::DETACHED_AN_LE_350 } else { lim::DETACHED_AN_GT_350 },
        Attachment::SemiDetached | Attachment::EndTerrace => if small { lim::SEMI_OR_END_AN_LE_350 } else { lim::SEMI_OR_END_AN_GT_350 },
        Attachment::MidTerrace => lim::MID_TERRACE,
    }
}

/// 🧱 GEG Anlage 2 reference U-values by element kind.
pub fn reference_u(kind: ElementKind) -> f64 {
    use geg_anlage2_reference_u as u;
    match kind {
        ElementKind::Wall => u::WALL,
        ElementKind::Roof => u::ROOF,
        ElementKind::Floor => u::FLOOR,
        ElementKind::Window => u::WINDOW,
        ElementKind::Door => u::DOOR,
    }
}

/// 📊 Mean U for non-residential GEG Anlage 3 (by element kind groups).
pub fn mean_u_for_kind(doc: &Din18599Snapshot, kind: ElementKind) -> Option<(f64, f64)> {
    let (area, ua) = doc
        .elements
        .iter()
        .filter(|e| e.kind == kind)
        .fold((0.0, 0.0), |(a, ua), e| (a + e.area_m2, ua + e.u_value_w_m2k * e.area_m2));
    if area <= 0.0 {
        None
    } else {
        let limit = match kind {
            ElementKind::Wall => geg_anlage3_mean_u::WALL,
            ElementKind::Roof => geg_anlage3_mean_u::ROOF,
            ElementKind::Floor => geg_anlage3_mean_u::FLOOR,
            ElementKind::Window => geg_anlage3_mean_u::WINDOW,
            ElementKind::Door => geg_anlage3_mean_u::DOOR,
        };
        Some((ua / area, limit))
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
}

fn zone_transmission(elements: &[&EnvelopeElement], delta_u: f64, volume_share: f64, a_env_total: f64) -> f64 {
    let sum_ua: f64 = elements.iter().map(|e| adjacency_factor(e.adjacency) * e.u_value_w_m2k * e.area_m2).sum();
    sum_ua + delta_u * a_env_total * volume_share
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

/// 🧮 Zone-level balances; building totals aggregate these.
pub fn zone_balances(doc: &Din18599Snapshot, elements: &[EnvelopeElement], delta_u: f64) -> Vec<ZoneBalance> {
    let climate = &doc.climate;
    let a_env_total: f64 = elements.iter().map(|e| e.area_m2).sum::<f64>().max(1e-9);
    let v_net: f64 = doc.zones.iter().map(|z| z.volume_m3).sum::<f64>().max(1e-9);
    let mut out = Vec::with_capacity(doc.zones.len());
    for zone in &doc.zones {
        let z_elems = elements_for_zone(elements, &zone.id);
        let share = zone.volume_m3 / v_net;
        let h_t = zone_transmission(&z_elems, delta_u, share, a_env_total);
        let h_v = zone_ventilation_hv(zone, doc, share);
        let theta_i = zone.theta_i_heat_c;
        let theta_c = zone.theta_i_cool_c;
        let qi_w = zone.internal_gains_w_m2;
        let row = usage_profile_row(zone.usage_profile);
        let mut q_h = 0.0;
        let mut q_c = 0.0;
        for m in 0..12 {
            let te = climate.theta_e_c[m];
            let qt = h_t * (theta_i - te).max(0.0) * HOURS_PER_MONTH / 1000.0;
            let qv = h_v * (theta_i - te).max(0.0) * HOURS_PER_MONTH / 1000.0;
            let qi = qi_w * zone.area_m2 * HOURS_PER_MONTH / 1000.0;
            let mut qs = 0.0;
            let g_h = climate.g_h_w_m2[m];
            for e in z_elems.iter().filter(|e| e.g_value > 0.0) {
                qs += g_h * orientation_solar_factor(e.orientation_deg, e.tilt_deg) * e.area_m2 * e.g_value * e.fc.clamp(0.0, 1.0) * HOURS_PER_MONTH / 1000.0;
            }
            let loss = qt + qv;
            let gain = qi + qs;
            let gamma = if loss <= 1e-9 { 1e9 } else { gain / loss };
            let eta_u = utilization_factor(gamma);
            q_h += (loss - eta_u * gain).max(0.0);
            let loss_c = (h_t + h_v) * (theta_c - te).max(0.0) * HOURS_PER_MONTH / 1000.0;
            let qt_c = h_t * (te - theta_c).max(0.0) * HOURS_PER_MONTH / 1000.0;
            let qv_c = h_v * (te - theta_c).max(0.0) * HOURS_PER_MONTH / 1000.0;
            let gamma_c = if loss_c <= 1e-9 { 1e9 } else { (qi + qs) / loss_c };
            let eta_c = utilization_factor(gamma_c);
            q_c += ((qi + qs) - eta_c * loss_c).max(0.0) + qt_c + qv_c;
        }
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
        });
    }
    out
}

fn orientation_solar_factor(orientation_deg: f64, tilt_deg: f64) -> f64 {
    use din_v_18599_2_solar_geometry_factors as s;
    let az = orientation_deg.rem_euclid(360.0);
    let from_south = (az - 180.0).abs().min(360.0 - (az - 180.0).abs());
    let horiz = s::NORTH_HORIZ + (s::SOUTH_HORIZ - s::NORTH_HORIZ) * (1.0 - from_south / 180.0);
    let t = tilt_deg.clamp(0.0, 90.0) / 90.0;
    let tilt = s::LOW_TILT + (s::STEEP_TILT - s::LOW_TILT) * t;
    horiz * tilt
}

/// ☀️ Monthly solar gains Q_S,m [kWh] from fenestration (DIN V 18599-2).
pub fn solar_gains_month_kwh(doc: &Din18599Snapshot, climate: &MonthlyClimate, month: usize) -> f64 {
    let irr = climate.g_h_w_m2[month];
    doc.elements
        .iter()
        .filter(|e| e.g_value > 0.0)
        .map(|e| irr * orientation_solar_factor(e.orientation_deg, e.tilt_deg) * e.area_m2 * e.g_value * e.fc.clamp(0.0, 1.0) * HOURS_PER_MONTH / 1000.0)
        .sum()
}

fn utilization_factor(gamma: f64) -> f64 {
    let a = DIN_V_18599_2_UTILIZATION_A;
    if gamma.abs() < 1e-9 {
        1.0
    } else if (gamma - 1.0).abs() < 1e-9 {
        a / (a + 1.0)
    } else {
        (1.0 - gamma.powf(-a)) / (1.0 - gamma.powf(-(a + 1.0)))
    }
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
    pub q_w_kwh: f64,
    pub q_l_kwh: f64,
    pub q_f_heat_kwh: f64,
    pub q_f_dhw_kwh: f64,
    pub q_f_cool_kwh: f64,
    pub q_f_light_kwh: f64,
    pub q_f_aux_kwh: f64,
    pub q_pv_kwh: f64,
    pub q_p_kwh: f64,
    pub q_p_ref_kwh: f64,
    pub q_p_limit_kwh: f64,
    pub q_h_ref_kwh: f64,
}

pub(super) fn heating_system_efficiency(doc: &Din18599Snapshot) -> f64 {
    (doc.heating.generation_efficiency * doc.heating.distribution_efficiency * doc.heating.storage_efficiency * doc.heating.transfer_efficiency).max(DIN_V_18599_5_ETA_SYS_FLOOR)
}
