//! 🪨 Soil-structure interaction of a bridge pier on a circular footing (EN 1998-5 Annex and EN 1998-2): the small-strain modulus G_max = ρ v_s², its degradation G = G_max (v_s/v_s,max)² from EN 1998-5 Table 4.1 (the ratios are unverified registry rows), the Gazetas springs of a circular footing on the half space and the lengthening of the period of a cantilever pier on these springs. See [`super`].

/// 🌍️ Gravitational acceleration used to turn a unit weight into a density [m/s²].
const GRAVITY: f64 = 9.81;

use semio_s_plugin_norm_compliance::{ndp_interpolate, AnnexChoice};

/// 📉 Rows of EN 1998-5 Table 4.1 along a_g S/g (the labels of the registry cells and the acceleration ratios).
const TABLE_4_1_AXIS: [(&str, f64); 3] = [("a010", 0.10), ("a020", 0.20), ("a030", 0.30)];

/// 📉 Velocity ratio v_s/v_s,max of EN 1998-5 Table 4.1 at a_g S/g, interpolated linearly between 0.10, 0.20 and 0.30 and held outside (unverified registry cells `ndp.en1998.en5.site-response.vs-ratio.*`).
pub fn table_4_1_vs_ratio(a_g_s_over_g: f64, annex: AnnexChoice) -> f64 {
    ndp_interpolate("ndp.en1998.en5.site-response.vs-ratio", &[], &TABLE_4_1_AXIS, a_g_s_over_g, annex)
}

/// 📉 Soil damping ratio of EN 1998-5 Table 4.1 at a_g S/g, interpolated like [`table_4_1_vs_ratio`] (unverified registry cells `ndp.en1998.en5.site-response.damping.*`).
pub fn table_4_1_damping(a_g_s_over_g: f64, annex: AnnexChoice) -> f64 {
    ndp_interpolate("ndp.en1998.en5.site-response.damping", &[], &TABLE_4_1_AXIS, a_g_s_over_g, annex)
}

/// 🪨 Small-strain shear modulus G_max = (γ/g) v_s² [Pa] from the unit weight γ [N/m³] and the shear wave velocity v_s [m/s].
pub fn shear_modulus_max_pa(unit_weight_n_m3: f64, vs_m_s: f64) -> f64 {
    unit_weight_n_m3 / GRAVITY * vs_m_s * vs_m_s
}

/// 📉 Degraded shear modulus G = G_max (v_s/v_s,max)² [Pa] for the velocity ratio of EN 1998-5 Table 4.1.
pub fn degraded_shear_modulus_pa(g_max_pa: f64, vs_ratio: f64) -> f64 {
    g_max_pa * vs_ratio * vs_ratio
}

/// 🦶 Springs of a rigid circular footing on the elastic half space (Gazetas): horizontal K_x, vertical K_z, rocking K_rx and torsion K_t.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Springs {
    pub horizontal_n_per_m: f64,
    pub vertical_n_per_m: f64,
    pub rocking_nm_per_rad: f64,
    pub torsion_nm_per_rad: f64,
}

impl Springs {
    /// 🦶 K_x = 8GR/(2−ν), K_z = 4GR/(1−ν), K_rx = 8GR³/(3(1−ν)), K_t = 16GR³/3 for the shear modulus G [Pa], the radius R [m] and ν < 0.5; `None` for a non-positive modulus or radius or a Poisson ratio outside [0, 0.5).
    pub fn circular(g_pa: f64, radius_m: f64, poisson: f64) -> Option<Self> {
        if !(g_pa > 0.0 && radius_m > 0.0 && (0.0..0.5).contains(&poisson)) {
            return None;
        }
        Some(Self {
            horizontal_n_per_m: 8.0 * g_pa * radius_m / (2.0 - poisson),
            vertical_n_per_m: 4.0 * g_pa * radius_m / (1.0 - poisson),
            rocking_nm_per_rad: 8.0 * g_pa * radius_m.powi(3) / (3.0 * (1.0 - poisson)),
            torsion_nm_per_rad: 16.0 * g_pa * radius_m.powi(3) / 3.0,
        })
    }

    /// 🦶 Stiffness of the foundation at the pier top, 1/K_f = 1/K_x + h²/K_rx [N/m].
    pub fn top_stiffness_n_per_m(&self, height_m: f64) -> f64 {
        1.0 / (1.0 / self.horizontal_n_per_m + height_m * height_m / self.rocking_nm_per_rad)
    }
}

/// ⏱ Ratio T_ssi/T_fixed = √(1 + k_p/K_x + k_p h²/K_rx) of the period of a cantilever pier with the stiffness k_p = 3EI/h³ and a mass at the top on the foundation springs against the period on a fixed base.
pub fn period_lengthening(pier_stiffness_n_per_m: f64, height_m: f64, springs: &Springs) -> f64 {
    (1.0 + pier_stiffness_n_per_m / springs.horizontal_n_per_m + pier_stiffness_n_per_m * height_m * height_m / springs.rocking_nm_per_rad).sqrt()
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
