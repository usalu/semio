//! 💧️ Material-dependent condensate limits for DIN 4108-3:2024-03 §5.2.2 and §5.2.3.

/// 🧾️ Verified material basis at the condensation locations of one envelope element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CondensateMaterials {
    pub limited_capillarity: bool,
    pub materials_resist_damage: bool,
    pub timber_increase_pp: f64,
    pub wood_product_increase_pp: f64,
}

/// 📏️ Governing mass limit and whether the condensation and material requirements are established.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CondensateLimits {
    pub mass_limit_kg_m2: f64,
    pub complies: Option<bool>,
}

/// 🧮 Condensate and material verification. Evaporation is assessed separately.
pub fn condensate_limits(mass_kg_m2: f64, materials: Option<CondensateMaterials>) -> CondensateLimits {
    let materials = materials.filter(|m| [m.timber_increase_pp, m.wood_product_increase_pp]
        .into_iter().all(|value| value.is_finite() && value >= 0.0));
    let limit = if materials.is_some_and(|m| m.limited_capillarity) { 0.5 } else { 1.0 };
    let complies = if !mass_kg_m2.is_finite() || mass_kg_m2 < 0.0 {
        None
    } else if mass_kg_m2 == 0.0 || mass_kg_m2 > limit {
        Some(mass_kg_m2 == 0.0)
    } else {
        materials.map(|m| m.materials_resist_damage && m.timber_increase_pp <= 5.0 && m.wood_product_increase_pp <= 3.0)
    };
    CondensateLimits { mass_limit_kg_m2: limit, complies }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_limits_follow_neutral_cases() {
        for line in include_str!("../../../../../🧪️tests/🧫️fixtures/💧️condensate-materials/🔣️.tsv").lines() {
            let values: Vec<f64> = line.split('\t').map(|value| value.parse().unwrap()).collect();
            let [mass, known, limited, damage_resistance, timber, products, limit, status] = values.try_into().unwrap();
            let materials = (known == 1.0).then_some(CondensateMaterials {
                limited_capillarity: limited == 1.0,
                materials_resist_damage: damage_resistance == 1.0,
                timber_increase_pp: timber,
                wood_product_increase_pp: products,
            });
            let result = condensate_limits(mass, materials);
            assert_eq!(result.mass_limit_kg_m2, limit, "{line}");
            assert_eq!(result.complies, (status >= 0.0).then_some(status == 1.0), "{line}");
        }
    }
}
