//! 🌬️ DIN 4108-3:2024-03 ventilated roof clearances and Figures 15 and 16.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Outlet {
    OppositeEdge,
    EaveOrMonoPitch,
    RidgeOrHip,
}

pub struct Opening {
    pub roof_area_m2: f64,
    pub length_m: f64,
    pub free_m2: f64,
}

impl Opening {
    fn minimum(&self, ratio: f64, area_per_m: f64) -> Option<f64> {
        if !positive(self.roof_area_m2) || !positive(self.length_m) || !nonnegative(self.free_m2) {
            return None;
        }
        Some((ratio * self.roof_area_m2).max(area_per_m * self.length_m))
    }
}

pub struct VentilatedRoof {
    pub slope_deg: f64,
    pub channel_roof_area_m2: f64,
    pub channel_width_m: f64,
    pub path_length_m: f64,
    pub free_height_m: f64,
    pub inlet: Opening,
    pub outlet: Opening,
    pub outlet_type: Outlet,
    pub inboard_sd_m: f64,
    pub inboard_r_fraction: f64,
}

pub struct Limits {
    pub low_slope: bool,
    pub height_m: f64,
    pub inlet_m2: f64,
    pub outlet_m2: f64,
    pub inboard_sd_m: f64,
    pub complies: bool,
}

impl VentilatedRoof {
    /// 📐️ Checks one continuous airflow path in the selected figure and its two free openings.
    pub fn evaluate(&self) -> Option<Limits> {
        if !self.slope_deg.is_finite() || !(0.0..90.0).contains(&self.slope_deg)
            || !nonnegative(self.free_height_m) || !nonnegative(self.inboard_sd_m)
        {
            return None;
        }
        let low_slope = self.slope_deg < 5.0;
        let inlet = self.inlet.minimum(0.002, 0.020)?;
        let outlet = match self.outlet_type {
            Outlet::RidgeOrHip if !low_slope => self.outlet.minimum(0.0005, 0.005)?,
            _ => self.outlet.minimum(0.002, 0.020)?,
        };
        let mut height: f64 = 0.020;
        let mut sd = 2.0;
        let mut permitted = self.outlet_type != Outlet::OppositeEdge;
        if low_slope {
            if !positive(self.channel_roof_area_m2) || !positive(self.channel_width_m)
                || !positive(self.path_length_m) || !(0.0..=1.0).contains(&self.inboard_r_fraction)
            {
                return None;
            }
            height = (0.002 * self.channel_roof_area_m2 / self.channel_width_m).max(0.050);
            sd = 100.0;
            permitted = self.outlet_type == Outlet::OppositeEdge && self.path_length_m <= 10.0 && self.inboard_r_fraction <= 0.20;
        }
        let complies = permitted && self.free_height_m >= height && self.inlet.free_m2 >= inlet
            && self.outlet.free_m2 >= outlet && self.inboard_sd_m >= sd;
        Some(Limits { low_slope, height_m: height, inlet_m2: inlet, outlet_m2: outlet, inboard_sd_m: sd, complies })
    }
}

fn nonnegative(value: f64) -> bool { value.is_finite() && value >= 0.0 }
fn positive(value: f64) -> bool { value.is_finite() && value > 0.0 }
