//! 🌡️ DIN 4108 thermal protection: minimum insulation, moisture, design values, U-value proof, airtightness.

use norm_core::{
    AnnexChoice, CheckReport, CheckResult, ClauseId, ClimateZoneDe, NormError, Quantity,
};

// #region 🔖Part2
pub mod part_2 {
    use super::*;

    /// 🏠 Building category for minimum thermal protection (DIN 4108-2).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BuildingCategory {
        Residential,
        Office,
        School,
        Industrial,
    }

    /// 📋 Layer in a building component for R-value accumulation.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Layer {
        pub thickness_m: f64,
        pub lambda_w_mk: f64,
    }

    /// 📐 Total thermal resistance R_T = R_si + Σ(d/λ) + R_se [m²K/W].
    pub fn total_resistance(layers: &[Layer], r_si: f64, r_se: f64) -> f64 {
        let mut r = r_si + r_se;
        for layer in layers {
            if layer.lambda_w_mk > 0.0 {
                r += layer.thickness_m / layer.lambda_w_mk;
            }
        }
        r
    }

    /// 📉 U-value from resistance [W/(m²K)].
    pub fn u_value_from_resistance(r_total: f64) -> f64 {
        if r_total <= 0.0 {
            return f64::INFINITY;
        }
        1.0 / r_total
    }

    fn limit_u_w_m2k(category: BuildingCategory) -> f64 {
        match category {
            BuildingCategory::Residential => 0.28,
            BuildingCategory::Office => 0.28,
            BuildingCategory::School => 0.28,
            BuildingCategory::Industrial => 0.50,
        }
    }

    /// ✅ Check minimum thermal protection per DIN 4108-2 §4.
    pub fn check_minimum_thermal_protection(
        category: BuildingCategory,
        layers: &[Layer],
        climate: ClimateZoneDe,
    ) -> Result<CheckResult, NormError> {
        if layers.is_empty() {
            return Err(NormError::IncompleteInput {
                field: "layers".into(),
            });
        }
        let _ = climate;
        let r_si = 0.13;
        let r_se = 0.04;
        let r = total_resistance(layers, r_si, r_se);
        let u = u_value_from_resistance(r);
        let limit = limit_u_w_m2k(category);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN 4108-2", "§4", "4.1"),
            Quantity::u_value_w_m2k(u),
            Quantity::u_value_w_m2k(limit),
            "minimum thermal protection U-value",
            AnnexChoice::En,
        ))
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    /// 💧 Layer for Glaser-style surface temperature check (DIN 4108-3 simplified).
    #[derive(Clone, Debug, PartialEq)]
    pub struct MoistureLayer {
        pub thickness_m: f64,
        pub lambda_w_mk: f64,
        pub mu: f64,
    }

    /// 🌡️ Interior surface temperature factor f_Rsi per layer stack.
    pub fn interior_surface_temperature_factor(
        layers: &[MoistureLayer],
        r_si: f64,
        r_se: f64,
        t_int_c: f64,
        t_ext_c: f64,
        rh_int: f64,
    ) -> f64 {
        let mut r_total = r_si + r_se;
        for layer in layers {
            if layer.lambda_w_mk > 0.0 {
                r_total += layer.thickness_m / layer.lambda_w_mk;
            }
        }
        let delta_t = t_int_c - t_ext_c;
        if delta_t.abs() < f64::EPSILON {
            return 1.0;
        }
        let t_si = t_int_c - (r_si / r_total) * delta_t;
        let _ = rh_int;
        (t_si - t_ext_c) / delta_t
    }

    /// ✅ Check interior surface temperature factor against limit 0.25 (DIN 4108-3).
    pub fn check_surface_temperature(
        layers: &[MoistureLayer],
        t_int_c: f64,
        t_ext_c: f64,
        rh_int: f64,
    ) -> Result<CheckResult, NormError> {
        if layers.is_empty() {
            return Err(NormError::IncompleteInput {
                field: "layers".into(),
            });
        }
        let f = interior_surface_temperature_factor(layers, 0.13, 0.04, t_int_c, t_ext_c, rh_int);
        let limit = 0.25;
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN 4108-3", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, f),
            Quantity::new(norm_core::QuantityKind::Dimensionless, limit),
            "interior surface temperature factor f_Rsi",
            AnnexChoice::En,
        ))
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::*;

    /// 📊 Design thermal conductivity λ with moisture conversion (DIN 4108-4 Table 1 excerpt).
    pub fn design_lambda(lambda_dry: f64, moisture_factor: f64) -> f64 {
        lambda_dry * moisture_factor
    }

    /// ✅ Verify design value within tabulated bounds.
    pub fn check_design_lambda(material: &str, lambda_design: f64) -> Result<CheckResult, NormError> {
        let limit = match material {
            "mineral_wool" => 0.040,
            "eps" => 0.038,
            "wood_fibre" => 0.045,
            "concrete" => 2.10,
            _ => {
                return Err(NormError::InvalidValue {
                    field: "material".into(),
                    reason: format!("unknown material: {material}"),
                });
            }
        };
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN 4108-4", "Table 1", "λ"),
            Quantity::new(norm_core::QuantityKind::ThermalConductivity, lambda_design),
            Quantity::new(norm_core::QuantityKind::ThermalConductivity, limit),
            "design thermal conductivity",
            AnnexChoice::En,
        ))
    }
}
// #endregion 🔖Part4

// #region 🔖Part6
pub mod part_6 {
    use super::*;
    use crate::part_2::{Layer, total_resistance, u_value_from_resistance};

    /// ✅ U-value proof per DIN 4108-6.
    pub fn check_u_value(layers: &[Layer], limit_u: f64) -> Result<CheckResult, NormError> {
        if layers.is_empty() {
            return Err(NormError::IncompleteInput {
                field: "layers".into(),
            });
        }
        let u = u_value_from_resistance(total_resistance(layers, 0.13, 0.04));
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN 4108-6", "§5", "5.1"),
            Quantity::u_value_w_m2k(u),
            Quantity::u_value_w_m2k(limit_u),
            "component U-value",
            AnnexChoice::En,
        ))
    }
}
// #endregion 🔖Part6

// #region 🔖Part7
pub mod part_7 {
    use super::*;

    /// 🌬️ Airtightness class per DIN 4108-7.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AirtightnessClass {
        Class1,
        Class2,
        Class3,
    }

    impl AirtightnessClass {
        pub fn n50_limit_h(self) -> f64 {
            match self {
                Self::Class1 => 1.0,
                Self::Class2 => 3.0,
                Self::Class3 => 6.0,
            }
        }
    }

    /// ✅ Check blower-door n50 against class limit.
    pub fn check_airtightness(n50_measured: f64, class: AirtightnessClass) -> CheckResult {
        let limit = class.n50_limit_h();
        CheckResult::from_utilization(
            ClauseId::new("DIN 4108-7", "§4", "4.2"),
            Quantity::new(norm_core::QuantityKind::AirPermeability, n50_measured),
            Quantity::new(norm_core::QuantityKind::AirPermeability, limit),
            "n50 airtightness",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part7

/// 📋 Run all applicable DIN 4108 checks for a typical opaque wall.
pub fn check_opaque_wall(
    category: part_2::BuildingCategory,
    layers: &[part_2::Layer],
    climate: ClimateZoneDe,
    airtightness_n50: f64,
) -> Result<CheckReport, NormError> {
    let mut report = CheckReport::default();
    report.push(part_2::check_minimum_thermal_protection(category, layers, climate)?);
    let moisture_layers: Vec<part_3::MoistureLayer> = layers
        .iter()
        .map(|l| part_3::MoistureLayer {
            thickness_m: l.thickness_m,
            lambda_w_mk: l.lambda_w_mk,
            mu: 10.0,
        })
        .collect();
    report.push(part_3::check_surface_temperature(
        &moisture_layers,
        20.0,
        climate.design_external_temperature_c(),
        0.5,
    )?);
    report.push(part_6::check_u_value(layers, 0.28)?);
    report.push(part_7::check_airtightness(
        airtightness_n50,
        part_7::AirtightnessClass::Class2,
    ));
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_wall() -> Vec<part_2::Layer> {
        vec![
            part_2::Layer {
                thickness_m: 0.24,
                lambda_w_mk: 0.81,
            },
            part_2::Layer {
                thickness_m: 0.14,
                lambda_w_mk: 0.035,
            },
        ]
    }

    #[test]
    fn opaque_wall_passes_din_4108_suite() {
        let report = check_opaque_wall(
            part_2::BuildingCategory::Residential,
            &sample_wall(),
            ClimateZoneDe::Zone2,
            2.5,
        )
        .expect("inputs complete");
        assert!(report.all_pass(), "checks: {:?}", report.checks);
    }

    #[test]
    fn part_4_mineral_wool_lambda() {
        let r = part_4::check_design_lambda("mineral_wool", 0.039).unwrap();
        assert_eq!(r.status, norm_core::CheckStatus::Pass);
    }
}
