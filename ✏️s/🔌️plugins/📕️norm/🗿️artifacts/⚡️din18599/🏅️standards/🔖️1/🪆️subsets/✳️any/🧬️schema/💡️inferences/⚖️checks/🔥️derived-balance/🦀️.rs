//! 🔥️ DIN V 18599 derived balance: domestic hot water, element paths, the whole-building balance and the required U-value for H_T'.

use super::*;

/// 🚿 DIN V 18599-10 Nutzenergie Trinkwarmwasser [kWh/(Person·a)] by use class.
pub fn dhw_profile_specific_kwh_person_a(use_class: UseClass) -> f64 {
    match use_class {
        UseClass::Residential | UseClass::ResidentialUpToTwoDwellings | UseClass::ResidentialMultifamily => din_v_18599_10_dhw_specific::RESIDENTIAL,
        UseClass::Office => din_v_18599_10_dhw_specific::OFFICE,
        UseClass::School => din_v_18599_10_dhw_specific::SCHOOL,
    }
}

/// 🏘️ Annual useful domestic hot-water energy for residential buildings per DIN V 18599-10 Table 4.
pub fn residential_dhw_useful_kwh_a(net_floor_area_m2: f64, dwelling_count: u32) -> Option<f64> {
    if !net_floor_area_m2.is_finite() || net_floor_area_m2 <= 0.0 || dwelling_count == 0 {
        return None;
    }
    let mean_dwelling_area = net_floor_area_m2 / f64::from(dwelling_count);
    let useful = net_floor_area_m2 * (16.5 - 0.05 * mean_dwelling_area).max(8.5);
    useful.is_finite().then_some(useful)
}

/// 🏠 Residential hot-water energy where the entered building geometry supports the standard profile.
pub fn residential_dhw_profile_kwh_a(doc: &Din18599Snapshot) -> Option<f64> {
    match doc.building_category {
        BuildingCategory::Residential => residential_dhw_useful_kwh_a(doc.net_floor_area_m2, doc.dhw.dwelling_count),
        BuildingCategory::NonResidential => None,
    }
}

/// 🚿 Engineering screening threshold: useful energy with modelled storage and distribution allowances.
pub fn dhw_limit_kwh_a(doc: &Din18599Snapshot) -> f64 {
    let q_nd = residential_dhw_profile_kwh_a(doc)
        .unwrap_or_else(|| occupants(doc) as f64 * dhw_profile_specific_kwh_person_a(doc.use_class));
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

/// ⚡ GModG §23 monthly limit for qualified generation and eligible electricity demand in kWh.
pub fn monthly_renewable_electricity_credit_kwh(generation_kwh: f64, eligible_electricity_kwh: f64) -> Option<f64> {
    (generation_kwh.is_finite() && generation_kwh >= 0.0
        && eligible_electricity_kwh.is_finite() && eligible_electricity_kwh >= 0.0)
        .then(|| generation_kwh.min(eligible_electricity_kwh))
}

fn balance_for(doc: &Din18599Snapshot, elements: &[EnvelopeElement], zones: Vec<ZoneBalance>, heat_eff: f64, carrier: &str, pv_area: f64) -> BalanceDerived {
    let climate = &doc.climate;
    let h_t: f64 = zones.iter().map(|z| z.h_t).sum();
    let h_v: f64 = zones.iter().map(|z| z.h_v).sum();
    let q_h: f64 = zones.iter().map(|z| z.q_h_nd_kwh).sum();
    let q_c: f64 = zones.iter().map(|z| z.q_c_nd_kwh).sum();
    let q_l: f64 = if doc.building_category == BuildingCategory::NonResidential {
        zones.iter().map(|z| z.q_l_kwh).sum()
    } else {
        0.0
    };
    let a_env = heat_transferring_envelope_area(elements).max(1e-9);
    let v_e = doc.heated_volume_m3.max(1e-9);
    let q_w_useful = residential_dhw_profile_kwh_a(doc).unwrap_or_else(|| occupants(doc) as f64 * doc.dhw.specific_demand_kwh_person_a);
    let q_w = q_w_useful + doc.dhw.storage_loss_kwh_a + doc.dhw.distribution_loss_kwh_a;
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
    let fixed_electricity = q_f_light + q_f_aux
        + if uses_grid_electricity(&doc.dhw.energy_carrier) { q_f_dhw } else { 0.0 };
    let annual_days: f64 = DIN_V_18599_1_DAYS_PER_MONTH.iter().sum();
    let renewable_electricity_months = std::array::from_fn(|month| {
        let days = DIN_V_18599_1_DAYS_PER_MONTH[month];
        let heating = if uses_grid_electricity(carrier) {
            zones.iter().map(|z| z.months[month].q_h_kwh).sum::<f64>() / heat_eff * auto
        } else { 0.0 };
        let cooling = if doc.cooling.is_installed() && uses_grid_electricity(doc.cooling.energy_carrier()) {
            zones.iter().map(|z| z.months[month].q_c_kwh).sum::<f64>() / doc.cooling.eer().max(0.5) * auto
        } else { 0.0 };
        let generation_kwh = pv_area.max(0.0) * doc.renewables.pv_efficiency.max(0.0)
            * climate.g_h_w_m2[month].max(0.0) * 24.0 * days / 1000.0;
        let eligible_electricity_kwh = heating + cooling + fixed_electricity * days / annual_days;
        let credit_kwh = monthly_renewable_electricity_credit_kwh(generation_kwh, eligible_electricity_kwh).unwrap_or(0.0);
        RenewableElectricityMonth { generation_kwh, eligible_electricity_kwh, credit_kwh }
    });
    let q_pv = renewable_electricity_months.iter().map(|month| month.credit_kwh).sum::<f64>();
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
        q_w_useful_kwh: q_w_useful,
        q_w_kwh: q_w,
        q_l_kwh: q_l,
        q_f_heat_kwh: q_f_heat,
        q_f_dhw_kwh: q_f_dhw,
        q_f_cool_kwh: q_f_cool,
        q_f_light_kwh: q_f_light,
        q_f_aux_kwh: q_f_aux,
        q_pv_kwh: q_pv,
        renewable_electricity_months,
        q_p_kwh: q_p,
        q_p_ref_kwh: 0.0,
        q_p_limit_kwh: 0.0,
        q_h_ref_kwh: 0.0,
    }
}

/// 🏠 Reference envelope element in the assigned building category. Non-residential solar shading uses the recorded factor.
pub fn energy_reference_element(doc: &Din18599Snapshot, element: &EnvelopeElement) -> EnvelopeElement {
    let mut reference = element.clone();
    match doc.building_category {
        BuildingCategory::Residential => {
            reference.u_value_w_m2k = reference_u(element);
            if element.kind.transmits_solar_radiation() {
                reference.g_value = if element.kind == ElementKind::RoofDome { 0.64 } else { GEG_ANLAGE1_WINDOW_G_REF };
                reference.fc = 1.0;
            }
        }
        BuildingCategory::NonResidential => {
            let values = doc.zones.iter().find(|zone| zone.id == element.zone_id)
                .and_then(|zone| annex2_reference_values(element.kind, element.adjacency, zone.theta_i_heat_c));
            if let Some(values) = values {
                reference.u_value_w_m2k = values.u_value_w_m2k;
                if let Some(g) = values.solar_transmittance {
                    reference.g_value = g;
                }
            }
        }
    }
    reference
}

/// 🧮 Full derived balance for the actual building including the modelled statutory reference envelope.
pub fn derive_balance(doc: &Din18599Snapshot) -> BalanceDerived {
    let actual_zones = zone_balances(doc, &doc.elements, doc.delta_u_wb_w_m2k);
    let mut actual = balance_for(doc, &doc.elements, actual_zones, heating_system_efficiency(doc), &doc.heating.energy_carrier, doc.renewables.pv_area_m2);
    let ref_elements: Vec<EnvelopeElement> = doc.elements.iter().map(|element| energy_reference_element(doc, element)).collect();
    let reference_zones = match doc.building_category {
        BuildingCategory::Residential => zone_balances(doc, &ref_elements, GEG_ANLAGE1_DELTA_U_WB_REF),
        BuildingCategory::NonResidential => zone_balances_with_bridges(doc, &ref_elements, |zone, elements, _| {
            annex2_reference_bridge_w_k(elements, zone.theta_i_heat_c)
        }),
    };
    let reference = balance_for(doc, &ref_elements, reference_zones, geg_anlage1_tabelle1_heating_eta::SYSTEM_PRODUCT, "natural_gas", 0.0);
    actual.q_p_ref_kwh = reference.q_p_kwh;
    actual.q_p_limit_kwh = doc.geg_qp_factor * reference.q_p_kwh;
    actual.q_h_ref_kwh = reference.q_h_nd_kwh;
    actual
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
        .filter(|(_, e)| matches!(e.kind, ElementKind::Wall | ElementKind::Roof | ElementKind::Floor | ElementKind::GroundSlabPerimeter | ElementKind::GroundSlabInterior) && e.adjacency != Adjacency::Heated)
        .max_by(|(_, a), (_, b)| a.u_value_w_m2k.partial_cmp(&b.u_value_w_m2k).unwrap_or(std::cmp::Ordering::Equal))
}

/// 🔎 Required U on worst element so H′T ≤ limit (analytic on that element's contribution).
pub fn required_u_for_ht_prime(doc: &Din18599Snapshot, limit: f64) -> Option<(usize, f64)> {
    let (idx, el) = worst_element(doc)?;
    let a_env = heat_transferring_envelope_area(&doc.elements);
    let h_t_max = limit * a_env;
    let f = adjacency_factor(el.adjacency);
    let others: f64 = doc
        .elements
        .iter()
        .enumerate()
        .filter(|(i, e)| *i != idx && e.adjacency != Adjacency::Heated)
        .map(|(_, e)| adjacency_factor(e.adjacency) * e.u_value_w_m2k * e.area_m2)
        .sum();
    let bridges = doc.delta_u_wb_w_m2k * a_env;
    let need_ua = (h_t_max - others - bridges).max(0.0);
    let u_req = if f * el.area_m2 <= 1e-9 { 0.0 } else { need_ua / (f * el.area_m2) };
    Some((idx, u_req))
}

pub(super) fn copy(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}
