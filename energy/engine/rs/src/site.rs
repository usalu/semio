//! 🌤️ Site, weather, EPW ingest, design days, solar position, ground temperatures.

use crate::error::{Error, Severity};
use crate::props::{humidity_ratio_from_rh, moist_air_density};
use crate::units::{deg_to_rad, rad_to_deg};
use serde::{Deserialize, Serialize};

// #region 🔖WeatherRecord
/// 🌡️ One timestep of outdoor weather.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeatherRecord {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub dry_bulb_c: f64,
    pub dew_point_c: f64,
    pub relative_humidity: f64,
    pub atmospheric_pressure_pa: f64,
    pub wind_speed_m_s: f64,
    pub wind_direction_deg: f64,
    pub direct_normal_irradiance_w_m2: f64,
    pub diffuse_horizontal_irradiance_w_m2: f64,
    pub horizontal_infrared_w_m2: f64,
    pub precipitation_mm: f64,
    pub snow_depth_mm: f64,
}

impl WeatherRecord {
    pub fn humidity_ratio(&self) -> f64 {
        humidity_ratio_from_rh(self.dry_bulb_c, self.relative_humidity, self.atmospheric_pressure_pa)
    }

    pub fn air_density(&self) -> f64 {
        moist_air_density(self.dry_bulb_c, self.humidity_ratio(), self.atmospheric_pressure_pa)
    }
}
// #endregion 🔖WeatherRecord

// #region 🔖Epw
/// 📄 EPW weather file parsed into typed records.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpwWeather {
    pub location: String,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation_m: f64,
    pub time_zone_hours: f64,
    pub records: Vec<WeatherRecord>,
}

impl EpwWeather {
    /// 📥 Parse EPW text content (EnergyPlus Weather format).
    pub fn parse(content: &str) -> Result<Self, Error> {
        let mut lines = content.lines().filter(|l| !l.trim().is_empty());
        let header1 = lines.next().ok_or_else(|| Error::fatal("EPW: missing location line"))?;
        let parts: Vec<&str> = header1.split(',').collect();
        if parts.len() < 10 {
            return Err(Error::fatal("EPW: invalid location header"));
        }
        let latitude_deg: f64 = parts[6].parse().map_err(|_| Error::fatal("EPW: bad latitude"))?;
        let longitude_deg: f64 = parts[7].parse().map_err(|_| Error::fatal("EPW: bad longitude"))?;
        let time_zone_hours: f64 = parts[8].parse().map_err(|_| Error::fatal("EPW: bad timezone"))?;
        let elevation_m: f64 = parts[9].parse().map_err(|_| Error::fatal("EPW: bad elevation"))?;
        let location = parts[1].to_string();

        for _ in 0..7 {
            lines.next();
        }

        let mut records = Vec::new();
        for line in lines {
            let p: Vec<&str> = line.split(',').collect();
            if p.len() < 22 {
                continue;
            }
            let year: u16 = p[0].parse().unwrap_or(2026);
            let month: u8 = p[1].parse().unwrap_or(1);
            let day: u8 = p[2].parse().unwrap_or(1);
            let hour: u8 = p[3].parse::<u8>().unwrap_or(1).saturating_sub(1);
            let minute: u8 = p[4].parse().unwrap_or(0);
            let dry_bulb_c: f64 = p[6].parse().unwrap_or(20.0);
            let dew_point_c: f64 = p[7].parse().unwrap_or(10.0);
            let relative_humidity: f64 = p[8].parse::<f64>().unwrap_or(50.0) / 100.0;
            let atmospheric_pressure_pa: f64 = p[9].parse::<f64>().unwrap_or(101_325.0);
            let direct_normal_irradiance_w_m2: f64 = p[14].parse().unwrap_or(0.0);
            let diffuse_horizontal_irradiance_w_m2: f64 = p[15].parse().unwrap_or(0.0);
            let horizontal_infrared_w_m2: f64 = p[16].parse().unwrap_or(250.0);
            let wind_speed_m_s: f64 = p[20].parse().unwrap_or(0.0);
            let wind_direction_deg: f64 = p[21].parse().unwrap_or(0.0);
            let precipitation_mm: f64 = p[33].parse().unwrap_or(0.0);
            let snow_depth_mm: f64 = p[35].parse().unwrap_or(0.0);
            records.push(WeatherRecord {
                year,
                month,
                day,
                hour,
                minute,
                dry_bulb_c,
                dew_point_c,
                relative_humidity,
                atmospheric_pressure_pa,
                wind_speed_m_s,
                wind_direction_deg,
                direct_normal_irradiance_w_m2,
                diffuse_horizontal_irradiance_w_m2,
                horizontal_infrared_w_m2,
                precipitation_mm,
                snow_depth_mm,
            });
        }

        if records.is_empty() {
            return Err(Error::fatal("EPW: no data records"));
        }

        Ok(Self {
            location,
            latitude_deg,
            longitude_deg,
            elevation_m,
            time_zone_hours,
            records,
        })
    }

    pub fn record_at_index(&self, idx: usize) -> Option<&WeatherRecord> {
        self.records.get(idx)
    }

    /// 📈 Interpolate weather to sub-hourly timestep.
    pub fn interpolate(&self, hour_index: f64) -> WeatherRecord {
        let idx = hour_index.floor() as usize;
        let frac = hour_index - idx as f64;
        let a = self.records.get(idx).copied().unwrap_or_else(|| self.records[0]);
        let b = self.records.get(idx + 1).copied().unwrap_or(a);
        WeatherRecord {
            dry_bulb_c: a.dry_bulb_c + frac * (b.dry_bulb_c - a.dry_bulb_c),
            dew_point_c: a.dew_point_c + frac * (b.dew_point_c - a.dew_point_c),
            relative_humidity: a.relative_humidity + frac * (b.relative_humidity - a.relative_humidity),
            wind_speed_m_s: a.wind_speed_m_s + frac * (b.wind_speed_m_s - a.wind_speed_m_s),
            direct_normal_irradiance_w_m2: a.direct_normal_irradiance_w_m2 + frac * (b.direct_normal_irradiance_w_m2 - a.direct_normal_irradiance_w_m2),
            diffuse_horizontal_irradiance_w_m2: a.diffuse_horizontal_irradiance_w_m2 + frac * (b.diffuse_horizontal_irradiance_w_m2 - a.diffuse_horizontal_irradiance_w_m2),
            ..a
        }
    }
}
// #endregion 🔖Epw

// #region 🔖DesignDay
/// 🌡️ Sizing design day specification.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DesignDayKind {
    Heating,
    Cooling,
    Custom,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DesignDay {
    pub name: String,
    pub kind: DesignDayKind,
    pub month: u8,
    pub day: u8,
    pub dry_bulb_max_c: f64,
    pub daily_range_k: f64,
    pub humidity_condition: DesignDayHumidity,
    pub wind_speed_m_s: f64,
    pub solar_model: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DesignDayHumidity {
    Wetbulb { wetbulb_at_max_c: f64 },
    Dewpoint { dewpoint_c: f64 },
    RelativeHumidity { rh: f64 },
}

impl DesignDay {
    pub fn hourly_dry_bulb(&self, hour: u8) -> f64 {
        let h = hour as f64;
        let min_t = self.dry_bulb_max_c - self.daily_range_k;
        if h < 6.0 || h > 18.0 {
            min_t
        } else {
            let phase = (h - 6.0) / 12.0 * std::f64::consts::PI;
            min_t + self.daily_range_k * phase.sin()
        }
    }
}
// #endregion 🔖DesignDay

// #region 🔖Solar
/// ☀️ Solar position for a site and datetime.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolarPosition {
    pub altitude_deg: f64,
    pub azimuth_deg: f64,
    pub equation_of_time_min: f64,
}

/// ☀️ Compute solar altitude and azimuth (simplified SPA).
pub fn solar_position(latitude_deg: f64, longitude_deg: f64, day_of_year: u16, hour_solar: f64) -> SolarPosition {
    let lat = deg_to_rad(latitude_deg);
    let decl = deg_to_rad(23.45 * (360.0 * (day_of_year as f64 - 81.0) / 365.0).to_radians().sin());
    let ha = deg_to_rad(15.0 * (hour_solar - 12.0));
    let sin_alt = lat.sin() * decl.sin() + lat.cos() * decl.cos() * ha.cos();
    let altitude_deg = rad_to_deg(sin_alt.clamp(-1.0, 1.0).asin());
    let cos_az = (decl.sin() - lat.sin() * sin_alt) / (lat.cos() * sin_alt.clamp(0.001, 1.0).acos().cos().max(1e-6));
    let azimuth_deg = rad_to_deg(cos_az.clamp(-1.0, 1.0).acos());
    let equation_of_time_min = 4.0 * (longitude_deg - 15.0 * (hour_solar / 24.0 * 24.0).round());
    SolarPosition {
        altitude_deg,
        azimuth_deg,
        equation_of_time_min,
    }
}

/// 🌡️ Sky temperature [K] from dry-bulb and dew-point (Brunt-type).
pub fn sky_temperature_k(t_dry_c: f64, t_dew_c: f64) -> f64 {
    let t_dry_k = t_dry_c + 273.15;
    let emissivity = 0.711 + 0.0056 * t_dew_c + 0.000_073 * t_dew_c * t_dew_c + 0.013 * (t_dew_c * 0.1).cos();
    t_dry_k * emissivity.powf(0.25)
}
// #endregion 🔖Solar

// #region 🔖Ground
/// 🌍 Ground temperature model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GroundTemperatureModel {
    Monthly { temperatures_c: [f64; 12] },
    Shallow { annual_amplitude_k: f64, phase_shift_days: f64, mean_c: f64 },
    Deep { temperature_c: f64 },
}

impl GroundTemperatureModel {
    pub fn temperature_c(&self, day_of_year: u16) -> f64 {
        match self {
            Self::Monthly { temperatures_c } => {
                let month = ((day_of_year as f64 - 1.0) / 30.44) as usize % 12;
                temperatures_c[month]
            }
            Self::Shallow { annual_amplitude_k, phase_shift_days, mean_c } => {
                let phase = 2.0 * std::f64::consts::PI * (day_of_year as f64 - phase_shift_days) / 365.0;
                mean_c + annual_amplitude_k * phase.cos()
            }
            Self::Deep { temperature_c } => *temperature_c,
        }
    }
}

/// 🚰 Water mains temperature model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WaterMainsModel {
    Constant { temperature_c: f64 },
    Monthly { temperatures_c: [f64; 12] },
}
// #endregion 🔖Ground

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epw_parses_minimal() {
        let epw = "LOCATION,Test,USA,TMY3,123,45.0,-75.0,-5.0,100.0\n\
DATA PERIODS,1,1,Data,Sunday,1/1,12/31\n\
2026,1,1,1,0,?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9*9*9?9*9,-5.0,-10.0,50,101325,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3.0,180,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0\n";
        let w = EpwWeather::parse(epw).unwrap();
        assert!((w.latitude_deg - 45.0).abs() < 1e-6);
        assert_eq!(w.records.len(), 1);
    }

    #[test]
    fn solar_noon_altitude_positive() {
        let pos = solar_position(45.0, 0.0, 172, 12.0);
        assert!(pos.altitude_deg > 0.0);
    }
}
