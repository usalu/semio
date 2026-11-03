//! ⏳️ Stress range of a fatigue detail from the stresses of the fatigue load model (EN 1993-1-9 5.1 to 5.4, MPa). See [EN 1993-1-9](https://eurocodes.jrc.ec.europa.eu/).

use semio_s_plugin_norm_compliance::ndp_common;

/// 📐️ Normal stress at a fibre (tension positive) from the axial force and the bending moments: `N/A + M_y z/I_y - M_z y/I_z`.
pub fn fibre_stress(n: f64, my: f64, mz: f64, area: f64, iy: f64, iz: f64, z: f64, y: f64) -> f64 {
    let axial = if area > 0.0 { n / area } else { 0.0 };
    let major = if iy > 0.0 { my * z / iy } else { 0.0 };
    let minor = if iz > 0.0 { mz * y / iz } else { 0.0 };
    axial + major - minor
}

/// 🧮️ Compressive part of a stress cycle counted with the registry factor `ndp.en1993.1-9.stress-range.compression-factor` (0.6) for non-welded and stress-relieved details.
pub fn reduced(sigma: f64) -> f64 {
    if sigma < 0.0 { ndp_common("ndp.en1993.1-9.stress-range.compression-factor") * sigma } else { sigma }
}

/// 🧮️ Stress range of the cycle between the stresses of the two load cases; with `relieved` the compressive part is reduced.
pub fn range(sigma_a: f64, sigma_b: f64, relieved: bool) -> f64 {
    let (hi, lo) = (sigma_a.max(sigma_b), sigma_a.min(sigma_b));
    if relieved { reduced(hi) - reduced(lo) } else { hi - lo }
}
