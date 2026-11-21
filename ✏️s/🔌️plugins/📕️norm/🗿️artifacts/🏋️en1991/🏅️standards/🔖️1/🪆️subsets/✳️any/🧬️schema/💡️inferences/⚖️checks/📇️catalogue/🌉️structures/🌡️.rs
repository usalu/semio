//! 🌡️ EN 1991-4:2006 thermal pressure on circular silo walls under §5.6.2.

use super::*;

pub(super) fn wall_cooling(c: &Case) -> R {
    let radius = c.pos("dc")? / 2.0;
    let thickness = c.pos("silo_wall_thickness")?;
    let expansion = c.pos("silo_wall_expansion")? * 1e-6;
    let wall_modulus = c.pos("silo_wall_modulus")?;
    let delta = c.n("silo_delta_cooling")?;
    if !delta.is_finite() || delta < 0.0 {
        return Err("input silo_delta_cooling must not be negative".into());
    }
    let poisson = c.o("silo_solid_poisson", 0.3);
    if !(0.0..=0.5).contains(&poisson) {
        return Err("input silo_solid_poisson must lie within 0 to 0.5".into());
    }

    let (temperature_multiplier, unloading_modulus, basis_en, basis_de) = match c.s("silo_modulus_source")? {
        "tested" => (1.2, c.pos("silo_unloading_modulus")?, "Unloading modulus from the material test at the assessed depth.", "Entlastungsmodul aus dem Materialversuch für die Nachweistiefe."),
        "density" => {
            let unit_weight_kn_m3 = c.pos("silo_solid_weight")? / 1000.0;
            let base_stress = c.pos("silo_base_filling_stress")?;
            let estimated = 7.0 * unit_weight_kn_m3.powf(1.5) * base_stress;
            (3.0, estimated, "C.15–C.16 estimates the modulus from unit weight and base filling stress to assess the need for testing.", "C.15–C.16 schätzt den Modul aus Wichte und Füllspannung am Wandfuß zur Beurteilung des Prüfbedarfs.")
        }
        source => return Err(format!("unknown model {source}")),
    };
    if !unloading_modulus.is_finite() || unloading_modulus <= 0.0 {
        return Err("E_sU must be a positive finite number".into());
    }

    let radial_compliance = radius / thickness / wall_modulus + (1.0 - poisson) / unloading_modulus;
    let pressure = temperature_multiplier * expansion * delta / radial_compliance;
    if !pressure.is_finite() {
        return Err("p_hT must be finite".into());
    }
    Ok(vec![
        Outcome::info("unloading_modulus", "Unloading modulus E_sU", "Entlastungsmodul E_sU", QuantityKind::Stress, unloading_modulus)
            .steps(basis_en).steps_de(basis_de),
        Outcome::info("temperature_multiplier", "Temperature load multiplier C_T", "Temperaturlastfaktor C_T", QuantityKind::Dimensionless, temperature_multiplier),
        Outcome::info("wall_pressure", "Additional normal pressure p_hT", "Zusätzlicher Normaldruck p_hT", QuantityKind::Pressure, pressure)
            .steps(format!("Equation 5.101: C_T·α_w·ΔT / (r/(t·E_w) + (1−ν)/E_sU) = {:.3} kPa.", pressure / 1000.0))
            .steps_de(format!("Gleichung 5.101: C_T·α_w·ΔT / (r/(t·E_w) + (1−ν)/E_sU) = {:.3} kPa.", pressure / 1000.0)),
    ])
}
