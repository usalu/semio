//! 💧 Evaporative cooling: direct and indirect with effectiveness and water use.

use crate::props::{humidity_ratio_from_rh, latent_heat_vaporization, saturation_pressure_pa, wet_bulb_c};
use crate::units::{CP_DRY_AIR, H_FG_0C};
use serde::{Deserialize, Serialize};

// #region 🔖EvaporativeCooler
/// 💧 Evaporative cooler configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EvaporativeCooler {
    Direct { effectiveness: f64, pad_area_m2: f64 },
    Indirect { sensible_effectiveness: f64, primary_flow_m3_s: f64, secondary_flow_m3_s: f64 },
}

/// 📥 Evaporative cooler inlet state.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvaporativeInlet {
    pub dry_bulb_c: f64,
    pub humidity_ratio: f64,
    pub mass_flow_kg_s: f64,
    pub pressure_pa: f64,
}

/// 📤 Evaporative cooler outlet state.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvaporativeOutput {
    pub dry_bulb_c: f64,
    pub humidity_ratio: f64,
    pub sensible_cooling_w: f64,
    pub latent_heat_w: f64,
    pub water_consumption_kg_s: f64,
    pub effectiveness_achieved: f64,
}
// #endregion 🔖EvaporativeCooler

// #region 🔖WetBulb
/// 🌡️ Simplified wet-bulb estimate from dry-bulb and humidity ratio.
pub fn wet_bulb_approx_c(t_db_c: f64, w: f64, p_atm: f64) -> f64 {
    let rh = relative_humidity_from_w(t_db_c, w, p_atm);
    t_db_c * rh.atan() / (std::f64::consts::FRAC_PI_2 + 0.15 * rh.atan()) + 0.5
}

fn relative_humidity_from_w(t_c: f64, w: f64, p_atm: f64) -> f64 {
    let p_ws = saturation_pressure_pa(t_c);
    let p_w = w * p_atm / (0.621_945 + w);
    (p_w / p_ws).clamp(0.0, 1.0)
}
// #endregion 🔖WetBulb

// #region 🔖Simulate
/// 💧 Simulate direct or indirect evaporative cooling.
pub fn evaporative_cool(
    cooler: &EvaporativeCooler,
    inlet: &EvaporativeInlet,
    enabled: bool,
) -> EvaporativeOutput {
    if !enabled || inlet.mass_flow_kg_s < 1e-9 {
        return EvaporativeOutput {
            dry_bulb_c: inlet.dry_bulb_c,
            humidity_ratio: inlet.humidity_ratio,
            sensible_cooling_w: 0.0,
            latent_heat_w: 0.0,
            water_consumption_kg_s: 0.0,
            effectiveness_achieved: 0.0,
        };
    }

    match cooler {
        EvaporativeCooler::Direct { effectiveness, .. } => {
            let eps = effectiveness.clamp(0.0, 1.0);
            let t_wb = wet_bulb_c(inlet.dry_bulb_c, inlet.humidity_ratio, inlet.pressure_pa);
            let t_out = inlet.dry_bulb_c - eps * (inlet.dry_bulb_c - t_wb);
            let w_sat_out = humidity_ratio_from_rh(t_out, 0.95, inlet.pressure_pa);
            let w_out = inlet.humidity_ratio + eps * (w_sat_out - inlet.humidity_ratio);
            let sensible = inlet.mass_flow_kg_s * CP_DRY_AIR * (inlet.dry_bulb_c - t_out);
            let water_evap = (w_out - inlet.humidity_ratio).max(0.0) * inlet.mass_flow_kg_s;
            let latent = water_evap * latent_heat_vaporization(t_out);
            EvaporativeOutput {
                dry_bulb_c: t_out,
                humidity_ratio: w_out,
                sensible_cooling_w: sensible.max(0.0),
                latent_heat_w: latent,
                water_consumption_kg_s: water_evap,
                effectiveness_achieved: eps,
            }
        }
        EvaporativeCooler::Indirect { sensible_effectiveness, .. } => {
            let eps = sensible_effectiveness.clamp(0.0, 1.0);
            let t_wb = wet_bulb_c(inlet.dry_bulb_c, inlet.humidity_ratio, inlet.pressure_pa);
            let t_out = inlet.dry_bulb_c - eps * (inlet.dry_bulb_c - t_wb);
            let sensible = inlet.mass_flow_kg_s * CP_DRY_AIR * (inlet.dry_bulb_c - t_out);
            let water_evap = sensible / H_FG_0C;
            EvaporativeOutput {
                dry_bulb_c: t_out,
                humidity_ratio: inlet.humidity_ratio,
                sensible_cooling_w: sensible.max(0.0),
                latent_heat_w: 0.0,
                water_consumption_kg_s: water_evap * 0.5,
                effectiveness_achieved: eps,
            }
        }
    }
}
// #endregion 🔖Simulate

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_cooling_lowers_dry_bulb() {
        let cooler = EvaporativeCooler::Direct { effectiveness: 0.8, pad_area_m2: 10.0 };
        let inlet = EvaporativeInlet {
            dry_bulb_c: 35.0,
            humidity_ratio: humidity_ratio_from_rh(35.0, 0.3, P_STD),
            mass_flow_kg_s: 1.0,
            pressure_pa: P_STD,
        };
        let out = evaporative_cool(&cooler, &inlet, true);
        assert!(out.dry_bulb_c < inlet.dry_bulb_c);
        assert!(out.humidity_ratio > inlet.humidity_ratio);
        assert!(out.water_consumption_kg_s > 0.0);
    }

    #[test]
    fn indirect_preserves_humidity_ratio() {
        let cooler = EvaporativeCooler::Indirect {
            sensible_effectiveness: 0.65,
            primary_flow_m3_s: 1.0,
            secondary_flow_m3_s: 1.0,
        };
        let inlet = EvaporativeInlet {
            dry_bulb_c: 32.0,
            humidity_ratio: 0.01,
            mass_flow_kg_s: 1.2,
            pressure_pa: P_STD,
        };
        let out = evaporative_cool(&cooler, &inlet, true);
        assert!((out.humidity_ratio - inlet.humidity_ratio).abs() < 1e-9);
        assert!(out.sensible_cooling_w > 0.0);
    }

    #[test]
    fn disabled_no_effect() {
        let cooler = EvaporativeCooler::Direct { effectiveness: 0.9, pad_area_m2: 5.0 };
        let inlet = EvaporativeInlet {
            dry_bulb_c: 30.0,
            humidity_ratio: 0.012,
            mass_flow_kg_s: 0.8,
            pressure_pa: P_STD,
        };
        let out = evaporative_cool(&cooler, &inlet, false);
        assert!((out.dry_bulb_c - inlet.dry_bulb_c).abs() < 1e-9);
    }
}
