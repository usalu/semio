//! 💦 Humidity equipment: steam/water humidifiers, dehumidifiers, solid desiccant.

use crate::props::{latent_heat_vaporization, saturation_pressure_pa};
use crate::units::H_FG_0C;
use serde::{Deserialize, Serialize};

// #region 🔖Humidifier
/// 💦 Humidifier types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Humidifier {
    SteamElectric { capacity_kg_s: f64, efficiency: f64 },
    SteamGas { capacity_kg_s: f64, efficiency: f64 },
    Atomizing { capacity_kg_s: f64, water_temp_c: f64 },
    WettedMedia { effectiveness: f64, pad_area_m2: f64 },
}

/// 📥 Humidifier boundary conditions.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HumidifierInlet {
    pub dry_bulb_c: f64,
    pub humidity_ratio: f64,
    pub mass_flow_kg_s: f64,
    pub target_humidity_ratio: f64,
    pub pressure_pa: f64,
}

/// 📤 Humidifier output.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HumidifierOutput {
    pub humidity_ratio: f64,
    pub water_added_kg_s: f64,
    pub power_w: f64,
    pub gas_consumption_w: f64,
}
// #endregion 🔖Humidifier

// #region 🔖Dehumidifier
/// 🌬️ Dehumidifier types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Dehumidifier {
    Refrigerant { cop: f64, capacity_kg_s: f64 },
    Desiccant { regen_temp_c: f64, moisture_removal_kg_s: f64, regen_power_w: f64 },
    SolidDesiccant { effectiveness: f64, max_removal_kg_s: f64 },
}

/// 📥 Dehumidifier boundary conditions.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DehumidifierInlet {
    pub dry_bulb_c: f64,
    pub humidity_ratio: f64,
    pub mass_flow_kg_s: f64,
    pub target_humidity_ratio: f64,
    pub pressure_pa: f64,
}

/// 📤 Dehumidifier output.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DehumidifierOutput {
    pub humidity_ratio: f64,
    pub moisture_removed_kg_s: f64,
    pub latent_cooling_w: f64,
    pub power_w: f64,
}
// #endregion 🔖Dehumidifier

// #region 🔖HumidifierOutput
/// 💦 Humidifier moisture addition rate [kg/s].
pub fn humidifier_output_kg_s(humidifier: &Humidifier, inlet: &HumidifierInlet) -> HumidifierOutput {
    let m_dot = inlet.mass_flow_kg_s.max(0.0);
    if m_dot < 1e-9 || inlet.humidity_ratio >= inlet.target_humidity_ratio {
        return HumidifierOutput {
            humidity_ratio: inlet.humidity_ratio,
            water_added_kg_s: 0.0,
            power_w: 0.0,
            gas_consumption_w: 0.0,
        };
    }

    let w_needed = inlet.target_humidity_ratio - inlet.humidity_ratio;
    let m_w_demand = w_needed * m_dot;

    match humidifier {
        Humidifier::SteamElectric { capacity_kg_s, efficiency } => {
            let m_w = m_w_demand.min(*capacity_kg_s);
            let power = m_w * H_FG_0C / efficiency.max(0.01);
            HumidifierOutput {
                humidity_ratio: inlet.humidity_ratio + m_w / m_dot,
                water_added_kg_s: m_w,
                power_w: power,
                gas_consumption_w: 0.0,
            }
        }
        Humidifier::SteamGas { capacity_kg_s, efficiency } => {
            let m_w = m_w_demand.min(*capacity_kg_s);
            let gas = m_w * H_FG_0C / efficiency.max(0.01);
            HumidifierOutput {
                humidity_ratio: inlet.humidity_ratio + m_w / m_dot,
                water_added_kg_s: m_w,
                power_w: 0.0,
                gas_consumption_w: gas,
            }
        }
        Humidifier::Atomizing { capacity_kg_s, water_temp_c } => {
            let m_w = m_w_demand.min(*capacity_kg_s);
            let evap_energy = m_w * latent_heat_vaporization(*water_temp_c);
            HumidifierOutput {
                humidity_ratio: inlet.humidity_ratio + m_w / m_dot,
                water_added_kg_s: m_w,
                power_w: evap_energy * 0.1,
                gas_consumption_w: 0.0,
            }
        }
        Humidifier::WettedMedia { effectiveness, .. } => {
            let w_sat = saturation_humidity_ratio(inlet.dry_bulb_c, inlet.pressure_pa);
            let w_max = inlet.humidity_ratio + effectiveness * (w_sat - inlet.humidity_ratio);
            let m_w = ((w_max - inlet.humidity_ratio) * m_dot).min(m_w_demand);
            HumidifierOutput {
                humidity_ratio: inlet.humidity_ratio + m_w / m_dot,
                water_added_kg_s: m_w,
                power_w: 50.0 * m_w,
                gas_consumption_w: 0.0,
            }
        }
    }
}
// #endregion 🔖HumidifierOutput

// #region 🔖DehumidifierOutput
/// 🌬️ Dehumidifier moisture removal rate [kg/s].
pub fn dehumidifier_output_kg_s(dehumidifier: &Dehumidifier, inlet: &DehumidifierInlet) -> DehumidifierOutput {
    let m_dot = inlet.mass_flow_kg_s.max(0.0);
    if m_dot < 1e-9 || inlet.humidity_ratio <= inlet.target_humidity_ratio {
        return DehumidifierOutput {
            humidity_ratio: inlet.humidity_ratio,
            moisture_removed_kg_s: 0.0,
            latent_cooling_w: 0.0,
            power_w: 0.0,
        };
    }

    let w_remove = inlet.humidity_ratio - inlet.target_humidity_ratio;
    let m_w_demand = w_remove * m_dot;

    match dehumidifier {
        Dehumidifier::Refrigerant { cop, capacity_kg_s } => {
            let m_w = m_w_demand.min(*capacity_kg_s);
            let latent = m_w * H_FG_0C;
            DehumidifierOutput {
                humidity_ratio: inlet.humidity_ratio - m_w / m_dot,
                moisture_removed_kg_s: m_w,
                latent_cooling_w: latent,
                power_w: latent / cop.max(0.5),
            }
        }
        Dehumidifier::Desiccant { moisture_removal_kg_s, regen_power_w, .. } => {
            let m_w = m_w_demand.min(*moisture_removal_kg_s);
            let latent = m_w * H_FG_0C;
            let plr = m_w / moisture_removal_kg_s.max(1e-9);
            DehumidifierOutput {
                humidity_ratio: inlet.humidity_ratio - m_w / m_dot,
                moisture_removed_kg_s: m_w,
                latent_cooling_w: latent * 0.8,
                power_w: regen_power_w * plr,
            }
        }
        Dehumidifier::SolidDesiccant { effectiveness, max_removal_kg_s } => {
            let m_w = m_w_demand.min(*max_removal_kg_s) * effectiveness;
            let latent = m_w * H_FG_0C;
            DehumidifierOutput {
                humidity_ratio: inlet.humidity_ratio - m_w / m_dot,
                moisture_removed_kg_s: m_w,
                latent_cooling_w: latent,
                power_w: latent * 0.3,
            }
        }
    }
}

fn saturation_humidity_ratio(t_c: f64, p_atm: f64) -> f64 {
    let p_ws = saturation_pressure_pa(t_c);
    0.621_945 * p_ws / (p_atm - p_ws).max(1.0)
}
// #endregion 🔖DehumidifierOutput

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steam_humidifier_adds_moisture() {
        let hum = Humidifier::SteamElectric { capacity_kg_s: 0.01, efficiency: 0.95 };
        let inlet = HumidifierInlet {
            dry_bulb_c: 20.0,
            humidity_ratio: 0.005,
            mass_flow_kg_s: 0.5,
            target_humidity_ratio: 0.009,
            pressure_pa: P_STD,
        };
        let out = humidifier_output_kg_s(&hum, &inlet);
        assert!(out.water_added_kg_s > 0.0);
        assert!(out.humidity_ratio > inlet.humidity_ratio);
        assert!(out.power_w > 0.0);
    }

    #[test]
    fn refrigerant_dehumidifier_removes_moisture() {
        let dehum = Dehumidifier::Refrigerant { cop: 2.5, capacity_kg_s: 0.005 };
        let inlet = DehumidifierInlet {
            dry_bulb_c: 26.0,
            humidity_ratio: 0.014,
            mass_flow_kg_s: 0.6,
            target_humidity_ratio: 0.009,
            pressure_pa: P_STD,
        };
        let out = dehumidifier_output_kg_s(&dehum, &inlet);
        assert!(out.moisture_removed_kg_s > 0.0);
        assert!(out.humidity_ratio < inlet.humidity_ratio);
    }

    #[test]
    fn at_target_no_humidification() {
        let hum = Humidifier::SteamElectric { capacity_kg_s: 0.01, efficiency: 1.0 };
        let inlet = HumidifierInlet {
            dry_bulb_c: 22.0,
            humidity_ratio: 0.01,
            mass_flow_kg_s: 0.5,
            target_humidity_ratio: 0.009,
            pressure_pa: P_STD,
        };
        let out = humidifier_output_kg_s(&hum, &inlet);
        assert_eq!(out.water_added_kg_s, 0.0);
    }
}
