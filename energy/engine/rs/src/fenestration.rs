//! 🪟 Fenestration heat transfer: glazing layers, frames, shades, and condensation.

use crate::material::{R_FILM_EXTERIOR_M2K_W, R_FILM_INTERIOR_M2K_W};
use crate::props::{dew_point_c, saturation_pressure_pa};

// #region 🔖GlazingLayer
/// 🪟 Single glazing layer optical and thermal properties.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlazingLayer {
    pub thickness_m: f64,
    pub conductivity_w_m_k: f64,
    pub solar_transmittance: f64,
    pub solar_reflectance: f64,
    pub visible_transmittance: f64,
    pub ir_emissivity: f64,
}

impl GlazingLayer {
    /// 🧊 Layer thermal resistance [m²·K/W].
    pub fn resistance_m2k_w(&self) -> f64 {
        self.thickness_m / self.conductivity_w_m_k.max(1e-6)
    }
}
// #endregion 🔖GlazingLayer

// #region 🔖ShadeState
/// 🌤️ Interior or exterior shade position and properties.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShadeState {
    pub deployed: bool,
    pub solar_transmittance: f64,
    pub solar_reflectance: f64,
    pub visible_transmittance: f64,
    pub ir_transmittance: f64,
}

impl ShadeState {
    pub const OPEN: Self = Self {
        deployed: false,
        solar_transmittance: 1.0,
        solar_reflectance: 0.0,
        visible_transmittance: 1.0,
        ir_transmittance: 1.0,
    };

    /// 🌤️ Effective solar transmittance with shade deployed.
    pub fn effective_solar_transmittance(&self) -> f64 {
        if self.deployed {
            self.solar_transmittance
        } else {
            1.0
        }
    }
}
// #endregion 🔖ShadeState

// #region 🔖WindowModel
/// 🪟 Complete window thermal and optical model.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowModel {
    pub glazing_layers: Vec<GlazingLayer>,
    pub gap_resistance_m2k_w: Vec<f64>,
    pub frame_fraction: f64,
    pub frame_u_value_w_m2k: f64,
    pub divider_fraction: f64,
    pub divider_conductance_w_k: f64,
    pub interior_shade: ShadeState,
    pub exterior_shade: ShadeState,
}

impl WindowModel {
    /// 🔥 Center-of-glass U-value [W/(m²·K)] from layer stack.
    pub fn center_u_value_w_m2k(&self) -> f64 {
        let mut r = R_FILM_INTERIOR_M2K_W + R_FILM_EXTERIOR_M2K_W;
        for (i, layer) in self.glazing_layers.iter().enumerate() {
            r += layer.resistance_m2k_w();
            if let Some(&gap_r) = self.gap_resistance_m2k_w.get(i) {
                r += gap_r;
            }
        }
        1.0 / r.max(1e-6)
    }

    /// ☀️ Center-of-glass solar heat gain coefficient (normal incidence).
    pub fn center_shgc(&self) -> f64 {
        let mut tau = 1.0_f64;
        for layer in &self.glazing_layers {
            tau *= layer.solar_transmittance;
        }
        tau *= self.exterior_shade.effective_solar_transmittance();
        tau *= self.interior_shade.effective_solar_transmittance();
        tau
    }

    /// 🔥 Area-weighted overall U including frame and divider.
    pub fn overall_u_value_w_m2k(&self, area_m2: f64) -> f64 {
        let a_cog = area_m2 * (1.0 - self.frame_fraction - self.divider_fraction).max(0.0);
        let a_frame = area_m2 * self.frame_fraction;
        let u_cog = self.center_u_value_w_m2k();
        let u_frame = self.frame_u_value_w_m2k;
        let divider_loss = self.divider_conductance_w_k;
        let total_cond = a_cog * u_cog + a_frame * u_frame + divider_loss;
        total_cond / area_m2.max(1e-6)
    }

    /// 🌡️ Interior glazing surface temperature [°C] (simplified steady state).
    pub fn interior_glazing_temp_c(
        &self,
        outside_temp_c: f64,
        inside_temp_c: f64,
        h_interior_w_m2k: f64,
        h_exterior_w_m2k: f64,
    ) -> f64 {
        let r_int = 1.0 / h_interior_w_m2k.max(0.1);
        let r_ext = 1.0 / h_exterior_w_m2k.max(0.1);
        let mut r_glazing = 0.0_f64;
        for (i, layer) in self.glazing_layers.iter().enumerate() {
            r_glazing += layer.resistance_m2k_w();
            if let Some(&gap_r) = self.gap_resistance_m2k_w.get(i) {
                r_glazing += gap_r;
            }
        }
        let r_total = r_ext + r_glazing + r_int;
        inside_temp_c + (outside_temp_c - inside_temp_c) * r_int / r_total.max(1e-6)
    }
}
// #endregion 🔖WindowModel

// #region 🔖HeatTransfer
/// 🔥 Window conductive heat loss [W] (positive = heat into zone from outside).
pub fn window_conduction_w(outside_temp_c: f64, inside_temp_c: f64, u_value_w_m2k: f64, area_m2: f64) -> f64 {
    u_value_w_m2k * (outside_temp_c - inside_temp_c) * area_m2
}

/// ☀️ Solar gain through window [W].
pub fn window_solar_gain_w(
    beam_normal_irradiance_w_m2: f64,
    incidence_cosine: f64,
    shgc: f64,
    area_m2: f64,
) -> f64 {
    beam_normal_irradiance_w_m2 * incidence_cosine.max(0.0) * shgc * area_m2
}
// #endregion 🔖HeatTransfer

// #region 🔖Condensation
/// 💧 Condensation risk on interior glazing surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CondensationRisk {
    None,
    Risk { margin_k: f64 },
    Condensing,
}

/// 💧 Assess interior surface condensation vs zone dew point.
pub fn condensation_risk(
    interior_surface_temp_c: f64,
    _zone_air_temp_c: f64,
    humidity_ratio: f64,
    atmospheric_pressure_pa: f64,
) -> CondensationRisk {
    let dew = dew_point_c(humidity_ratio, atmospheric_pressure_pa);
    let margin = interior_surface_temp_c - dew;
    if margin <= 0.0 {
        CondensationRisk::Condensing
    } else if margin < 2.0 {
        CondensationRisk::Risk { margin_k: margin }
    } else {
        CondensationRisk::None
    }
}

/// 💧 Interior surface RH given surface temperature.
pub fn interior_surface_rh(surface_temp_c: f64, zone_air_temp_c: f64, zone_rh: f64) -> f64 {
    let p_ws_air = saturation_pressure_pa(zone_air_temp_c);
    let p_w = zone_rh * p_ws_air;
    let p_ws_surf = saturation_pressure_pa(surface_temp_c);
    (p_w / p_ws_surf.max(1.0)).clamp(0.0, 1.5)
}
// #endregion 🔖Condensation

#[cfg(test)]
mod tests {
    use super::*;

    fn double_glazing() -> WindowModel {
        WindowModel {
            glazing_layers: vec![
                GlazingLayer {
                    thickness_m: 0.004,
                    conductivity_w_m_k: 0.9,
                    solar_transmittance: 0.82,
                    solar_reflectance: 0.08,
                    visible_transmittance: 0.88,
                    ir_emissivity: 0.84,
                },
                GlazingLayer {
                    thickness_m: 0.004,
                    conductivity_w_m_k: 0.9,
                    solar_transmittance: 0.74,
                    solar_reflectance: 0.12,
                    visible_transmittance: 0.80,
                    ir_emissivity: 0.84,
                },
            ],
            gap_resistance_m2k_w: vec![0.15],
            frame_fraction: 0.15,
            frame_u_value_w_m2k: 2.5,
            divider_fraction: 0.05,
            divider_conductance_w_k: 0.5,
            interior_shade: ShadeState::OPEN,
            exterior_shade: ShadeState::OPEN,
        }
    }

    #[test]
    fn double_glazing_u_below_single() {
        let win = double_glazing();
        let u_double = win.center_u_value_w_m2k();
        let single = WindowModel {
            glazing_layers: vec![win.glazing_layers[0]],
            gap_resistance_m2k_w: vec![],
            ..win.clone()
        };
        assert!(u_double < single.center_u_value_w_m2k());
    }

    #[test]
    fn shade_reduces_shgc() {
        let mut win = double_glazing();
        win.interior_shade = ShadeState {
            deployed: true,
            solar_transmittance: 0.1,
            solar_reflectance: 0.5,
            visible_transmittance: 0.1,
            ir_transmittance: 0.2,
        };
        assert!(win.center_shgc() < 0.2);
    }

    #[test]
    fn conduction_cold_outside_negative_into_zone() {
        let q = window_conduction_w(-10.0, 20.0, 1.2, 2.0);
        assert!(q < 0.0);
    }

    #[test]
    fn condensation_when_surface_cold() {
        let risk = condensation_risk(5.0, 22.0, 0.012, 101_325.0);
        assert!(matches!(risk, CondensationRisk::Condensing | CondensationRisk::Risk { .. }));
    }
}
