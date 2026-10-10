//! 🧱️ The thermal transmittance of a layered construction after ISO 6946: `R_T = R_si + sum(d / lambda) + R_se` and `U = 1 / R_T`, with the surface resistances of the direction of the heat flow (ISO 6946, table 7):
//! upward 0.10 and 0.04, horizontal 0.13 and 0.04, downward 0.17 and 0.04 square metre kelvin per watt. The far side of a construction is the outdoor air (`R_se`), the ground (no surface resistance, the ground
//! is accounted for by the ground factor of the aggregates) or another room (`R_si` of the same direction). The simplified method: homogeneous layers only, no correction for fasteners, air gaps or inverted roofs.
//! 📎 https://www.iso.org/standard/65708.html

/// ♨️ The direction of the heat flow through a construction, which fixes the surface resistances.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum HeatFlow {
    Up,
    Horizontal,
    Down,
}

impl HeatFlow {
    /// 🌡️ The internal surface resistance `R_si` in square metre kelvin per watt.
    pub fn rsi(self) -> f64 {
        match self {
            Self::Up => 0.10,
            Self::Horizontal => 0.13,
            Self::Down => 0.17,
        }
    }

    /// 🌬️ The external surface resistance `R_se` in square metre kelvin per watt.
    pub fn rse(self) -> f64 {
        0.04
    }
}

/// 🔭️ What lies on the far side of a construction.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum FarSide {
    Outdoor,
    Ground,
    Room,
}

impl FarSide {
    /// 🌡️ The surface resistance of the far side for `flow`.
    pub fn resistance(self, flow: HeatFlow) -> f64 {
        match self {
            Self::Outdoor => flow.rse(),
            Self::Ground => 0.0,
            Self::Room => flow.rsi(),
        }
    }
}

/// 🍰️ The thermal resistance `sum(d / lambda)` of the layers `(thickness in metres, conductivity in watts per metre and kelvin)`, `None` when there is no layer or a layer lacks a positive thickness or conductivity.
pub fn resistance(layers: &[(f64, f64)]) -> Option<f64> {
    if layers.is_empty() || layers.iter().any(|(thickness, conductivity)| !(thickness.is_finite() && *thickness > 0.0 && conductivity.is_finite() && *conductivity > 0.0)) {
        return None;
    }
    Some(layers.iter().map(|(thickness, conductivity)| thickness / conductivity).sum())
}

/// 🧱️ The total resistance `R_T` of the layers between the room (`R_si`) and the far side.
pub fn total_resistance(layers: &[(f64, f64)], flow: HeatFlow, far: FarSide) -> Option<f64> {
    resistance(layers).map(|layers| flow.rsi() + layers + far.resistance(flow))
}

/// 🧱️ The thermal transmittance `U` in watts per square metre and kelvin, `None` when a layer has no thermal data.
pub fn transmittance(layers: &[(f64, f64)], flow: HeatFlow, far: FarSide) -> Option<f64> {
    total_resistance(layers, flow, far).map(|total| 1.0 / total)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
