//! 🧮️ Pure DIN 4108 and EN ISO 6946 helpers of the compliance checks: the surface resistances, the layer and total resistances, the U-value and the insulation thickness. See [`super`].

use super::*;

//#region 🔖️ComplianceHelpers
/// 🧱 Interior surface resistance for walls (EN ISO 6946 / DIN 4108-2), horizontal heat flow [m²K/W].
pub const R_SI_WALL: f64 = 0.13;

/// 🧱 Exterior surface resistance (EN ISO 6946) [m²K/W].
pub const R_SE: f64 = 0.04;

/// 🧱 Interior surface resistance for roofs (upward heat flow) [m²K/W].
pub const R_SI_ROOF: f64 = 0.10;

/// 🧱 Interior surface resistance for floors (downward heat flow) [m²K/W].
pub const R_SI_FLOOR: f64 = 0.17;

/// 🦠 DIN 4108-2 mould criterion — temperature factor at interior surface.
pub const F_RSI_MINIMUM: f64 = 0.70;

/// ❄️ DIN 4108-3 winter condensed mass limit for most constructions [kg/m²].
pub const GLASER_MASS_LIMIT: f64 = 1.0;

/// ❄️ DIN 4108-3 condensed mass limit for wood / wood-based layers [kg/m²].
pub const GLASER_MASS_LIMIT_WOOD: f64 = 0.5;

pub(super) fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub(super) fn entity_path(collection: &str, id: &str) -> String {
    format!("{collection}[id={id}]")
}

pub(super) fn entity_field_path(collection: &str, id: &str, field: &str) -> String {
    format!("{collection}[id={id}].{field}")
}

pub(super) fn layer_field_path(element_id: &str, layer_id: &str, field: &str) -> String {
    format!("elements[id={element_id}].layers[id={layer_id}].{field}")
}

pub(super) fn window_field_path(zone_id: &str, window_id: &str, field: &str) -> String {
    format!("zones[id={zone_id}].windows[id={window_id}].{field}")
}

pub(super) fn bridge_field_path(bridge_id: &str, field: &str) -> String {
    format!("thermalBridges[id={bridge_id}].{field}")
}

/// ❄️ DIN 4108-3 Annex A — winter exterior climate for Glaser condensation assessment.
pub const GLASER_WINTER_T_EXT_C: f64 = -5.0;

/// ❄️ DIN 4108-3 Annex A — winter exterior relative humidity.
pub const GLASER_WINTER_RH_EXT: f64 = 0.80;

/// 🏠 DIN 4108-3 Annex A — interior design temperature for Glaser.
pub const GLASER_INTERIOR_T_C: f64 = 20.0;

/// 🏠 DIN 4108-3 Annex A — interior relative humidity for Glaser.
pub const GLASER_INTERIOR_RH: f64 = 0.50;

/// ☀️ DIN 4108-3 Annex A — summer exterior temperature (walls / floors).
pub const GLASER_SUMMER_T_EXT_C: f64 = 12.0;

/// ☀️ DIN 4108-3 Annex A — summer exterior relative humidity.
pub const GLASER_SUMMER_RH_EXT: f64 = 0.70;

/// ☀️ DIN 4108-3 Annex A — summer exterior temperature for roofs (elevated).
pub const GLASER_SUMMER_ROOF_T_EXT_C: f64 = 20.0;

/// 🧱 ISO 6946 surface resistances with heat-flow direction from inclination (0° horizontal … 90° vertical).
pub(super) fn surface_resistances_inclined(kind: &str, adjacent: &str, inclination_deg: f64) -> (f64, f64) {
    let vertical = (inclination_deg.abs() % 180.0).min(180.0 - (inclination_deg.abs() % 180.0));
    let t = (vertical / 90.0).clamp(0.0, 1.0);
    let r_si = match kind {
        "roof" => R_SI_ROOF + (R_SI_WALL - R_SI_ROOF) * t,
        "floor" => R_SI_FLOOR + (R_SI_WALL - R_SI_FLOOR) * t,
        _ => R_SI_ROOF + (R_SI_WALL - R_SI_ROOF) * t,
    };
    let r_se = match adjacent {
        "ground" | "unheated" | "otherHeated" => 0.0,
        _ => R_SE,
    };
    (r_si, r_se)
}

/// 📐️ Homogeneous layer thermal resistance R = d/λ [m²K/W] (EN ISO 6946 §6.1).
pub fn layer_resistance(layer: &LayerDocument) -> f64 {
    if layer.lambda <= 0.0 {
        return f64::INFINITY;
    }
    layer.thickness_m / layer.lambda
}

/// 📐️ ISO 6946 §6.7 lower bound — isothermal planes; inhomogeneous R_i = 1/Σ(f_j/R_ij).
pub fn layer_resistance_lower(layer: &LayerDocument) -> f64 {
    if layer.segments.is_empty() {
        return layer_resistance(layer);
    }
    let mut sum = 0.0;
    for seg in &layer.segments {
        if seg.fraction <= 0.0 || seg.lambda <= 0.0 {
            continue;
        }
        let r_ij = layer.thickness_m / seg.lambda;
        sum += seg.fraction / r_ij;
    }
    if sum <= 0.0 {
        f64::INFINITY
    } else {
        1.0 / sum
    }
}

/// 📐️ ISO 6946 §6.7 upper bound R_T,upper = 1/Σ(f_j/R_Tj) with sections from the first inhomogeneous layer.
pub fn total_resistance_upper(layers: &[LayerDocument], r_si: f64, r_se: f64) -> f64 {
    let Some(primary) = layers.iter().find(|layer| !layer.segments.is_empty()) else {
        return r_si + r_se + layers.iter().map(layer_resistance).sum::<f64>();
    };
    let mut sum_f_over_r = 0.0;
    for (j, seg) in primary.segments.iter().enumerate() {
        if seg.fraction <= 0.0 {
            continue;
        }
        let mut r_tj = r_si + r_se;
        for layer in layers {
            if layer.segments.is_empty() {
                r_tj += layer_resistance(layer);
            } else {
                let section = layer.segments.get(j).unwrap_or(seg);
                if section.lambda <= 0.0 {
                    r_tj = f64::INFINITY;
                    break;
                }
                r_tj += layer.thickness_m / section.lambda;
            }
        }
        if r_tj > 0.0 && r_tj.is_finite() {
            sum_f_over_r += seg.fraction / r_tj;
        }
    }
    if sum_f_over_r <= 0.0 {
        f64::INFINITY
    } else {
        1.0 / sum_f_over_r
    }
}

/// 📐️ ISO 6946 §6.7 lower bound R_T,lower = R_si + Σ R_i + R_se.
pub fn total_resistance_lower(layers: &[LayerDocument], r_si: f64, r_se: f64) -> f64 {
    r_si + r_se + layers.iter().map(layer_resistance_lower).sum::<f64>()
}

/// 📐️ Total resistance R_T [m²K/W] — series for homogeneous stacks; (R_upper+R_lower)/2 for ISO 6946 §6.7.
pub fn total_resistance(layers: &[LayerDocument], r_si: f64, r_se: f64) -> f64 {
    if layers.iter().any(|layer| !layer.segments.is_empty()) {
        0.5 * (total_resistance_upper(layers, r_si, r_se) + total_resistance_lower(layers, r_si, r_se))
    } else {
        r_si + r_se + layers.iter().map(layer_resistance).sum::<f64>()
    }
}

/// 🧭 Facade exterior temperature adjustment from azimuth (north colder) [K] — continuous cosine.
pub fn facade_temp_adjustment_k(orientation_deg: f64) -> f64 {
    -2.0 * orientation_deg.to_radians().cos()
}

/// 📉️ U = 1/R_T [W/(m²K)].
pub fn u_value(r_total: f64) -> f64 {
    if r_total <= 0.0 {
        f64::INFINITY
    } else {
        1.0 / r_total
    }
}

/// 🌉️ Σ(ψ·l) [W/K].
pub fn psi_l_sum(bridges: &[ThermalBridge]) -> f64 {
    bridges.iter().map(|b| b.psi * b.length_m).sum()
}

/// 📐️ Envelope area of opaque elements [m²].
pub fn opaque_envelope_area(elements: &[EnvelopeElement]) -> f64 {
    elements.iter().filter(|e| e.kind != "window" && e.kind != "door").map(|e| e.area_m2).sum()
}

pub(super) fn insulation_layer_index(layers: &[LayerDocument]) -> Option<usize> {
    layers
        .iter()
        .enumerate()
        .filter(|(_, l)| l.lambda > 0.0 && l.lambda < 0.1)
        .min_by(|a, b| a.1.lambda.partial_cmp(&b.1.lambda).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
}

pub(super) fn required_insulation_thickness(layers: &[LayerDocument], r_si: f64, r_se: f64, r_min: f64) -> Option<(usize, f64)> {
    let idx = insulation_layer_index(layers)?;
    let lambda = layers[idx].lambda;
    let r_other = r_si
        + r_se
        + layers
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != idx)
            .map(|(_, l)| layer_resistance(l))
            .sum::<f64>();
    let mut d_req = (r_min - r_other).max(0.0) * lambda;
    while r_other + d_req / lambda < r_min {
        d_req = f64::from_bits(d_req.to_bits().saturating_add(1));
    }
    Some((idx, d_req))
}

/// ✅️ Same Pass predicate as `CheckBuilder::utilization` (≤ 1), not a printed 4-decimal tie.
pub(super) fn u_prime_utilization_clears(u_prime: f64, u_max: f64) -> bool {
    u_max > 0.0 && u_prime.is_finite() && (u_prime / u_max) <= 1.0
}

/// 🧮 U′ with candidate insulation thickness using the same R_T sum as `total_resistance`.
pub(super) fn u_prime_at_insulation_thickness(element: &EnvelopeElement, idx: usize, thickness_m: f64, r_si: f64, r_se: f64, bridge_add: f64) -> f64 {
    let lambda = element.layers[idx].lambda;
    if lambda <= 0.0 {
        return f64::INFINITY;
    }
    let r_tot = r_si
        + r_se
        + element
            .layers
            .iter()
            .enumerate()
            .map(|(i, layer)| {
                if i == idx {
                    thickness_m / lambda
                } else {
                    layer_resistance(layer)
                }
            })
            .sum::<f64>();
    let u = u_value(r_tot) + element.delta_u_g + element.delta_u_f + element.delta_u_r;
    u + bridge_add
}
//#endregion 🔖️ComplianceHelpers
