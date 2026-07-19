//! 🧱 Construction thermal properties: U-value, R-value, and thermal mass.

use crate::model::Material;

// #region 🔖FilmResistance
/// 🌬️ Standard interior film resistance [m²·K/W] (vertical surface, still air).
pub const R_FILM_INTERIOR_M2K_W: f64 = 0.13;
/// 🌬️ Standard exterior film resistance [m²·K/W] (outdoor, low wind).
pub const R_FILM_EXTERIOR_M2K_W: f64 = 0.04;
// #endregion 🔖FilmResistance

// #region 🔖Resistance
/// 🧊 Layer thermal resistance R = d/λ [m²·K/W].
pub fn layer_resistance_m2k_w(layer: &Material) -> f64 {
    layer.thickness_m / layer.conductivity_w_m_k
}

/// 🧊 Total effective resistance including film resistances [m²·K/W].
pub fn effective_resistance(layers: &[Material], r_interior: f64, r_exterior: f64) -> f64 {
    r_interior + r_exterior + layers.iter().map(layer_resistance_m2k_w).sum::<f64>()
}
// #endregion 🔖Resistance

// #region 🔖UValue
/// 🔥 Construction U-value [W/(m²·K)] = 1/R_total.
pub fn construction_u_value(layers: &[Material], r_interior: f64, r_exterior: f64) -> f64 {
    let r = effective_resistance(layers, r_interior, r_exterior);
    if r <= 0.0 {
        return f64::INFINITY;
    }
    1.0 / r
}
// #endregion 🔖UValue

// #region 🔖ThermalMass
/// 🪨 Area-normalized thermal capacitance [J/(m²·K)] = Σ ρ·c·d.
pub fn construction_thermal_mass(layers: &[Material]) -> f64 {
    layers.iter().map(|m| m.density_kg_m3 * m.specific_heat_j_kg_k * m.thickness_m).sum()
}

/// 🪨 Volumetric heat capacity of a single layer [J/(m³·K)].
pub fn layer_volumetric_heat_capacity(layer: &Material) -> f64 {
    layer.density_kg_m3 * layer.specific_heat_j_kg_k
}
// #endregion 🔖ThermalMass

// #region 🔖Equivalent
/// 🧱 Equivalent single-layer properties for multi-layer constructions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EquivalentLayer {
    pub thickness_m: f64,
    pub conductivity_w_m_k: f64,
    pub density_kg_m3: f64,
    pub specific_heat_j_kg_k: f64,
}

/// 🧱 Collapse layers into one equivalent slab preserving R and thermal mass.
pub fn equivalent_layer(layers: &[Material], r_interior: f64, r_exterior: f64) -> EquivalentLayer {
    let r_solid: f64 = layers.iter().map(layer_resistance_m2k_w).sum();
    let _r_total = r_interior + r_exterior + r_solid;
    let thickness_m: f64 = layers.iter().map(|l| l.thickness_m).sum();
    let thermal_mass = construction_thermal_mass(layers);
    let conductivity_w_m_k = if r_solid > 0.0 { thickness_m / r_solid } else { 1.0 };
    let volumetric = if thickness_m > 0.0 { thermal_mass / thickness_m } else { 0.0 };
    EquivalentLayer { thickness_m, conductivity_w_m_k, density_kg_m3: volumetric / 1000.0, specific_heat_j_kg_k: 1000.0 }
}
// #endregion 🔖Equivalent

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::EntityId;

    fn brick() -> Material {
        Material { id: EntityId(1), name: "Brick".into(), thickness_m: 0.1, conductivity_w_m_k: 0.72, density_kg_m3: 1920.0, specific_heat_j_kg_k: 840.0, thermal_absorptance: 0.9, solar_absorptance: 0.6, visible_absorptance: 0.6 }
    }

    fn insulation() -> Material {
        Material { id: EntityId(2), name: "EPS".into(), thickness_m: 0.14, conductivity_w_m_k: 0.035, density_kg_m3: 30.0, specific_heat_j_kg_k: 1400.0, thermal_absorptance: 0.9, solar_absorptance: 0.4, visible_absorptance: 0.4 }
    }

    #[test]
    fn wall_u_value_reasonable() {
        let layers = vec![brick(), insulation()];
        let u = construction_u_value(&layers, R_FILM_INTERIOR_M2K_W, R_FILM_EXTERIOR_M2K_W);
        assert!(u > 0.15 && u < 0.35);
    }

    #[test]
    fn resistance_adds_film_terms() {
        let layers = vec![insulation()];
        let r = effective_resistance(&layers, R_FILM_INTERIOR_M2K_W, R_FILM_EXTERIOR_M2K_W);
        assert!(r > layer_resistance_m2k_w(&insulation()));
    }

    #[test]
    fn thermal_mass_positive() {
        let mass = construction_thermal_mass(&[brick(), insulation()]);
        assert!(mass > 10_000.0);
    }
}
