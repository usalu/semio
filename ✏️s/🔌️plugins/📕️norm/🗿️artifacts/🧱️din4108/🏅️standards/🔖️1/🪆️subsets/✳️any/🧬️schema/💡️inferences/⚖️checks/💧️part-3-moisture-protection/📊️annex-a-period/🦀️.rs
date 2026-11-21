//! 📊 DIN 4108-3:2024-03 Annex A period balance with fixed standard boundary pressures.

use super::{LayerDocument, VapourProfile, DELTA_0_KG_M_S_PA, GLASER_WINTER, profile_with_boundary, sd_m};
use std::collections::VecDeque;

pub const ANNEX_A_WINTER_INTERIOR_PA: f64 = 1168.0;
pub const ANNEX_A_WINTER_EXTERIOR_PA: f64 = 321.0;
pub const ANNEX_A_SUMMER_BOUNDARY_PA: f64 = 1200.0;
pub const ANNEX_A_PERIOD_SECONDS: f64 = 7_776_000.0;
pub const ANNEX_A_R_SI: f64 = 0.25;
pub const ANNEX_A_R_SE: f64 = 0.04;

/// 🌤️ Exterior component exposure for the Annex A evaporation-period saturation pressure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnnexADryingExposure {
    WallOrAtticCeiling,
    ShadedOrLightRoof,
    UnshadedDarkRoof,
}

impl AnnexADryingExposure {
    pub const fn pressure_pa(self) -> f64 {
        match self {
            Self::WallOrAtticCeiling | Self::ShadedOrLightRoof => 1700.0,
            Self::UnshadedDarkRoof => 2000.0,
        }
    }
}

/// 💧 Condensed mass at a single layer interface during the normative winter period.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexACondensationPlane {
    pub interface_index: usize,
    pub sd_m: f64,
    pub condensed_kg_m2: f64,
}

/// 🌦️ Mass accumulating in winter and potential evaporation from those same layer locations.
#[derive(Clone, Debug, PartialEq)]
pub struct AnnexAPeriodBalance {
    pub planes: Vec<AnnexACondensationPlane>,
    pub condensed_kg_m2: f64,
    pub evaporation_capacity_kg_m2: f64,
}

/// 📈 Annex A.2.2 winter profile. The measured or declared equivalent air layer thicknesses are positive.
pub fn annex_a_winter_profile(layers: &[LayerDocument]) -> Option<VapourProfile> {
    if layers.is_empty() || layers.iter().any(|layer| {
        !layer.thickness_m.is_finite() || layer.thickness_m <= 0.0
            || !layer.lambda.is_finite() || layer.lambda <= 0.0
            || !sd_m(layer).is_finite() || sd_m(layer) <= 0.0
    }) {
        return None;
    }
    let thicknesses = layers.iter().enumerate().map(|(index, layer)| {
        let sd = sd_m(layer);
        if index + 1 == layers.len() { sd.max(0.1) } else { sd }
    });
    Some(profile_with_boundary(
        layers, ANNEX_A_R_SI, ANNEX_A_R_SE, GLASER_WINTER,
        ANNEX_A_WINTER_INTERIOR_PA, ANNEX_A_WINTER_EXTERIOR_PA, thicknesses,
    ))
}

/// ☀️ Annex A.2.6 evaporation capacity with the winter condensate as the initial water inventory.
pub fn annex_a_evaporation_capacity_kg_m2(
    total_sd_m: f64,
    planes: &[AnnexACondensationPlane],
    exposure: AnnexADryingExposure,
) -> Option<f64> {
    if !total_sd_m.is_finite() || total_sd_m <= 0.0 || planes.iter().any(|plane| {
        !plane.sd_m.is_finite() || plane.sd_m <= 0.0 || plane.sd_m >= total_sd_m
            || !plane.condensed_kg_m2.is_finite() || plane.condensed_kg_m2 < 0.0
    }) || planes.windows(2).any(|pair| pair[0].sd_m >= pair[1].sd_m) {
        return None;
    }
    let factor = DELTA_0_KG_M_S_PA * (exposure.pressure_pa() - ANNEX_A_SUMMER_BOUNDARY_PA);
    let mut wet: VecDeque<_> = planes.iter().copied().filter(|plane| plane.condensed_kg_m2 > 0.0).collect();
    let mut time = ANNEX_A_PERIOD_SECONDS;
    let mut capacity = 0.0;
    while time > 0.0 && !wet.is_empty() {
        let (first, last) = (*wet.front()?, *wet.back()?);
        let inward = factor / first.sd_m;
        let outward = factor / (total_sd_m - last.sd_m);
        if wet.len() == 1 {
            capacity += (inward + outward) * time;
            break;
        }
        let first_dry = first.condensed_kg_m2 / inward;
        let last_dry = last.condensed_kg_m2 / outward;
        let step = time.min(first_dry).min(last_dry);
        capacity += (inward + outward) * step;
        if step >= time {
            break;
        }
        time -= step;
        if step >= first_dry && step >= last_dry && wet.len() == 2 {
            break;
        }
        if step >= first_dry {
            wet.pop_front();
        } else {
            wet.front_mut()?.condensed_kg_m2 -= inward * step;
        }
        if step >= last_dry {
            wet.pop_back();
        } else {
            wet.back_mut()?.condensed_kg_m2 -= outward * step;
        }
    }
    capacity.is_finite().then_some(capacity)
}

/// 🧮 Normative period balance, including both condensation interfaces of Annex B.
pub fn annex_a_period_balance(layers: &[LayerDocument], exposure: AnnexADryingExposure) -> Option<AnnexAPeriodBalance> {
    let profile = annex_a_winter_profile(layers)?;
    let total = *profile.sd_m.last()?;
    let mut nodes = vec![0];
    nodes.extend(profile.planes.iter().copied());
    nodes.push(profile.sd_m.len() - 1);
    let planes = nodes.windows(3).map(|window| {
        let [inside, interface, outside] = [window[0], window[1], window[2]];
        let sd = profile.sd_m[interface];
        let p = profile.p_pa[interface];
        let inward = (profile.p_pa[inside] - p) / (sd - profile.sd_m[inside]);
        let outward = (p - profile.p_pa[outside]) / (profile.sd_m[outside] - sd);
        AnnexACondensationPlane {
            interface_index: interface,
            sd_m: sd,
            condensed_kg_m2: (DELTA_0_KG_M_S_PA * (inward - outward) * ANNEX_A_PERIOD_SECONDS).max(0.0),
        }
    }).collect::<Vec<_>>();
    let condensed_kg_m2 = planes.iter().map(|plane| plane.condensed_kg_m2).sum();
    let evaporation_capacity_kg_m2 = annex_a_evaporation_capacity_kg_m2(total, &planes, exposure)?;
    Some(AnnexAPeriodBalance { planes, condensed_kg_m2, evaporation_capacity_kg_m2 })
}

/// 🗺️ The elevation exclusion in DIN 4108-3:2024-03 §5.2.1.
pub fn annex_a_elevation_in_scope(elevation_m: f64) -> Option<bool> {
    elevation_m.is_finite().then_some(elevation_m <= 700.0)
}
