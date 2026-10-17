//! 🔥️ DIN V 18599 derived balance: domestic hot water, element paths, the whole-building balance and the required U-value for H_T'.

use super::*;

/// 🚿 DIN V 18599-10 Nutzenergie Trinkwarmwasser [kWh/(Person·a)] by use class.
pub fn dhw_profile_specific_kwh_person_a(use_class: UseClass) -> f64 {
    match use_class {
        UseClass::Residential => din_v_18599_10_dhw_specific::RESIDENTIAL,
        UseClass::Office => din_v_18599_10_dhw_specific::OFFICE,
        UseClass::School => din_v_18599_10_dhw_specific::SCHOOL,
    }
}

/// 🚿 DIN V 18599-8 Q_W limit: profile Nutzenergie + loss allowances scaled to that Nutzenergie.
pub fn dhw_limit_kwh_a(doc: &Din18599Snapshot) -> f64 {
    let n = occupants(doc) as f64;
    let q_nd = n * dhw_profile_specific_kwh_person_a(doc.use_class);
    let storage_cap = (din_v_18599_8_dhw_loss_fractions::STORAGE_OF_USEFUL * q_nd).max(din_v_18599_8_dhw_loss_fractions::STORAGE_FLOOR_KWH_A);
    let dist_cap = (din_v_18599_8_dhw_loss_fractions::DISTRIBUTION_OF_USEFUL * q_nd).max(din_v_18599_8_dhw_loss_fractions::DISTRIBUTION_FLOOR_KWH_A);
    q_nd + storage_cap + dist_cap
}

pub(super) fn element_u_path(e: &EnvelopeElement) -> String {
    format!("elements[id={}].uValueWM2k", e.id)
}

pub(super) fn element_g_path(e: &EnvelopeElement) -> String {
    format!("elements[id={}].gValue", e.id)
}

pub(super) fn element_fc_path(e: &EnvelopeElement) -> String {
    format!("elements[id={}].fc", e.id)
}

fn fan_operating_hours_weighted(doc: &Din18599Snapshot) -> f64 {
    let v: f64 = doc.zones.iter().map(|z| z.volume_m3).sum::<f64>().max(1e-9);
    doc.zones.iter().map(|z| usage_profile_row(z.usage_profile).fan_hours_a * z.volume_m3 / v).sum()
}

fn balance_for(doc: &Din18599Snapshot, elements: &[EnvelopeElement], delta_u: f64, heat_eff: f64, carrier: &str, pv_area: f64) -> BalanceDerived {
    let climate = &doc.climate;
    let zones = zone_balances(doc, elements, delta_u);
    let h_t: f64 = zones.iter().map(|z| z.h_t).sum();
    let h_v: f64 = zones.iter().map(|z| z.h_v).sum();
    let q_h: f64 = zones.iter().map(|z| z.q_h_nd_kwh).sum();
    let q_c: f64 = zones.iter().map(|z| z.q_c_nd_kwh).sum();
    let q_l: f64 = zones.iter().map(|z| z.q_l_kwh).sum();
    let a_env: f64 = elements.iter().map(|e| e.area_m2).sum::<f64>().max(1e-9);
    let v_e = doc.heated_volume_m3.max(1e-9);
    let q_w = occupants(doc) as f64 * doc.dhw.specific_demand_kwh_person_a + doc.dhw.storage_loss_kwh_a + doc.dhw.distribution_loss_kwh_a;
    let auto = automation_factor(doc.automation_class);
    let q_f_heat = q_h / heat_eff * auto;
    let q_f_dhw = q_w / heat_eff.max(0.5) * auto;
    let q_f_cool = if doc.cooling.is_installed() {
        q_c / doc.cooling.eer().max(0.5) * auto
    } else {
        0.0
    };
    let q_f_light = q_l * auto;
    let q_f_aux = doc.ventilation.fan_power_w * fan_operating_hours_weighted(doc) / 1000.0;
    let annual_g = climate.g_h_w_m2.iter().sum::<f64>() / 12.0 * 8760.0 / 1000.0;
    let q_pv = pv_area * doc.renewables.pv_efficiency * annual_g + doc.renewables.solar_thermal_kwh_a;
    let q_p = q_f_heat * primary_energy_factor(carrier)
        + q_f_dhw * primary_energy_factor(&doc.dhw.energy_carrier)
        + q_f_cool * primary_energy_factor(doc.cooling.energy_carrier())
        + (q_f_light + q_f_aux) * primary_energy_factor("electricity")
        - q_pv * primary_energy_factor("electricity");
    BalanceDerived {
        h_t,
        h_v,
        h_t_prime: h_t / a_env,
        a_over_v_e: a_env / v_e,
        q_h_nd_kwh: q_h,
        q_c_nd_kwh: q_c,
        q_w_kwh: q_w,
        q_l_kwh: q_l,
        q_f_heat_kwh: q_f_heat,
        q_f_dhw_kwh: q_f_dhw,
        q_f_cool_kwh: q_f_cool,
        q_f_light_kwh: q_f_light,
        q_f_aux_kwh: q_f_aux,
        q_pv_kwh: q_pv,
        q_p_kwh: q_p,
        q_p_ref_kwh: 0.0,
        q_p_limit_kwh: 0.0,
        q_h_ref_kwh: 0.0,
    }
}

/// 🧮 Full derived balance for the actual building including GEG reference run.
pub fn derive_balance(doc: &Din18599Snapshot) -> BalanceDerived {
    let mut actual = balance_for(doc, &doc.elements, doc.delta_u_wb_w_m2k, heating_system_efficiency(doc), &doc.heating.energy_carrier, doc.renewables.pv_area_m2);
    let ref_elements: Vec<EnvelopeElement> = doc
        .elements
        .iter()
        .map(|e| {
            let mut r = e.clone();
            r.u_value_w_m2k = reference_u(e.kind);
            if e.kind == ElementKind::Window {
                r.g_value = GEG_ANLAGE1_WINDOW_G_REF;
                r.fc = 1.0;
            }
            r
        })
        .collect();
    let reference = balance_for(doc, &ref_elements, GEG_ANLAGE1_DELTA_U_WB_REF, geg_anlage1_tabelle1_heating_eta::SYSTEM_PRODUCT, "natural_gas", 0.0);
    actual.q_p_ref_kwh = reference.q_p_kwh;
    actual.q_p_limit_kwh = doc.geg_qp_factor * reference.q_p_kwh;
    actual.q_h_ref_kwh = reference.q_h_nd_kwh;
    actual
}

/// 📊 DIN V 18599-12:2018 Tabelle 5 — specific primary energy [kWh/(m²·a)] before A_N and f_BAC.
pub fn tabular_specific_primary_energy_kwh_m2(use_class: UseClass) -> f64 {
    match use_class {
        UseClass::Residential => din_v_18599_12_tabelle5_qp_specific::RESIDENTIAL,
        UseClass::Office => din_v_18599_12_tabelle5_qp_specific::OFFICE,
        UseClass::School => din_v_18599_12_tabelle5_qp_specific::SCHOOL,
    }
}

pub(super) fn energy(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Energy, v)
}
pub(super) fn heat_transfer(v: f64) -> Quantity {
    Quantity::new(QuantityKind::HeatTransferCoefficient, v)
}
pub(super) fn dimensionless(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

pub(super) fn whole_building() -> SubjectRef {
    SubjectRef::whole(LocalizedCopy::new("Building", "Gebäude"))
}

pub(super) fn element_ref(e: &EnvelopeElement, path: &str) -> SubjectRef {
    SubjectRef::new(&e.id, path, LocalizedCopy::new(e.label_en.clone(), e.label_de.clone()))
}

fn worst_element<'a>(doc: &'a Din18599Snapshot) -> Option<(usize, &'a EnvelopeElement)> {
    doc.elements
        .iter()
        .enumerate()
        .filter(|(_, e)| matches!(e.kind, ElementKind::Wall | ElementKind::Roof | ElementKind::Floor) && e.adjacency != Adjacency::Heated)
        .max_by(|(_, a), (_, b)| a.u_value_w_m2k.partial_cmp(&b.u_value_w_m2k).unwrap_or(std::cmp::Ordering::Equal))
}

/// 🔎 Required U on worst element so H′T ≤ limit (analytic on that element's contribution).
pub fn required_u_for_ht_prime(doc: &Din18599Snapshot, limit: f64) -> Option<(usize, f64)> {
    let (idx, el) = worst_element(doc)?;
    let a_env: f64 = doc.elements.iter().map(|e| e.area_m2).sum::<f64>().max(1e-9);
    let h_t_max = limit * a_env;
    let f = adjacency_factor(el.adjacency);
    let others: f64 = doc
        .elements
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != idx)
        .map(|(_, e)| adjacency_factor(e.adjacency) * e.u_value_w_m2k * e.area_m2)
        .sum();
    let a_env: f64 = doc.elements.iter().map(|e| e.area_m2).sum();
    let bridges = doc.delta_u_wb_w_m2k * a_env;
    let need_ua = (h_t_max - others - bridges).max(0.0);
    let u_req = if f * el.area_m2 <= 1e-9 { 0.0 } else { need_ua / (f * el.area_m2) };
    Some((idx, u_req))
}

pub(super) fn copy(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}
