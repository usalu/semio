//! 🌊️ Deterministic wave passage in a welded steel pipe on stable ground, EN 1998-4:2006 §6.5.2 and Annex B.

use super::pipe_strain::welded_pipe_strain_limits;

/// 📋️ Pipe geometry, material and documented uniform-ground wave amplitudes in SI units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PipeWaveInput {
    pub peak_longitudinal_velocity_m_s: f64,
    pub peak_longitudinal_acceleration_m_s2: f64,
    pub longitudinal_wave_speed_m_s: f64,
    pub peak_transverse_acceleration_m_s2: f64,
    pub transverse_wave_speed_m_s: f64,
    pub pipe_radius_m: f64,
    pub wall_thickness_m: f64,
    pub elastic_modulus_pa: f64,
    pub available_bond_shear_pa: f64,
}

/// 📏️ Annex B demands and §6.5.2 steel strain limits, expressed as fractions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PipeWaveResponse {
    pub axial_strain: f64,
    pub curvature_per_m: f64,
    pub required_bond_shear_pa: f64,
    pub bond_sufficient: bool,
    pub envelope_strain: f64,
    pub tensile_strain_limit: f64,
    pub compressive_strain_limit: f64,
}

/// 🧮 A worst-phase axial-plus-bending envelope for the Annex B perfect-bond approximation.
pub fn bonded_pipe_wave_response(inputs: PipeWaveInput) -> Option<PipeWaveResponse> {
    let PipeWaveInput {
        peak_longitudinal_velocity_m_s: velocity, peak_longitudinal_acceleration_m_s2: a_long,
        longitudinal_wave_speed_m_s: c_long, peak_transverse_acceleration_m_s2: a_trans,
        transverse_wave_speed_m_s: c_trans, pipe_radius_m: radius, wall_thickness_m: thickness,
        elastic_modulus_pa: modulus, available_bond_shear_pa: available,
    } = inputs;
    let limits = welded_pipe_strain_limits(radius, thickness)?;
    if ![velocity, a_long, a_trans, available].into_iter().all(|x| x.is_finite() && x >= 0.0)
        || ![c_long, c_trans, modulus].into_iter().all(|x| x.is_finite() && x > 0.0)
    {
        return None;
    }
    let axial_strain = velocity / c_long;
    let curvature_per_m = a_trans / c_trans / c_trans;
    let required_bond_shear_pa = thickness * (a_long / c_long / c_long * modulus);
    let envelope_strain = axial_strain + curvature_per_m * radius;
    if ![axial_strain, curvature_per_m, required_bond_shear_pa, envelope_strain].into_iter().all(f64::is_finite) {
        return None;
    }
    Some(PipeWaveResponse {
        axial_strain,
        curvature_per_m,
        required_bond_shear_pa,
        bond_sufficient: available >= required_bond_shear_pa,
        envelope_strain,
        tensile_strain_limit: limits.tensile,
        compressive_strain_limit: limits.compressive,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buried_pipe_wave_response_matches_common_cases() {
        for line in include_str!("../../../🧪️tests/🧫️fixtures/🌊️buried-pipe-wave/🔣️.tsv").lines().skip(1) {
            let mut fields = line.split('\t');
            let name = fields.next().unwrap();
            let mut numbers = fields.map(|value| value.parse::<f64>().unwrap());
            let [velocity, a_long, c_long, a_trans, c_trans, radius, thickness, modulus, available] =
                numbers.by_ref().take(9).collect::<Vec<_>>().try_into().unwrap();
            let actual = bonded_pipe_wave_response(PipeWaveInput {
                peak_longitudinal_velocity_m_s: velocity,
                peak_longitudinal_acceleration_m_s2: a_long,
                longitudinal_wave_speed_m_s: c_long,
                peak_transverse_acceleration_m_s2: a_trans,
                transverse_wave_speed_m_s: c_trans,
                pipe_radius_m: radius,
                wall_thickness_m: thickness,
                elastic_modulus_pa: modulus,
                available_bond_shear_pa: available,
            });
            let expected: Vec<_> = numbers.collect();
            if expected.is_empty() {
                assert!(actual.is_none(), "{name}");
                continue;
            }
            let actual = actual.unwrap();
            let values = [
                actual.axial_strain, actual.curvature_per_m, actual.required_bond_shear_pa,
                if actual.bond_sufficient { 1.0 } else { 0.0 },
                actual.envelope_strain, actual.tensile_strain_limit, actual.compressive_strain_limit,
            ];
            assert_eq!(values.len(), expected.len(), "{name}");
            assert!(values.into_iter().zip(expected).all(|(a,b)| (a-b).abs() < 1e-10), "{name}");
        }
    }
}
