//! 🪨️ Effective (cracked) stiffness of concrete and masonry elements for the lateral analysis (EN 1998-1 4.3.1(6)/(7)): the stiffness the periods, drifts and stiffness centre are computed with.

use semio_s_artifact_norm_en1998_model::{En1998Building, En1998Member, En1998Storey, StiffnessBasis};

/// 🪨 Registry key of the simplified cracked-stiffness factor, 0.5 of the gross flexural and shear stiffness (unverified, see the NDP registry).
pub const SIMPLIFIED_FACTOR_KEY: &str = "ndp.en1998.en1.cracked-stiffness-factor";

/// 🪨 The simplified-method factor read from the registry, 0.5 of the gross stiffness (EN 1998-1 4.3.1(6), unverified).
pub fn simplified_factor() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common(SIMPLIFIED_FACTOR_KEY)
}

/// 🪨 Whether a declared effective share is usable: it lies in (0, 1].
pub fn is_valid_ratio(declared_ratio: f64) -> bool {
    declared_ratio > 0.0 && declared_ratio <= 1.0
}

/// 🪨 Factor on the gross stiffness: the registry factor for the simplified basis, else the declared share (at most 1).
pub fn factor(basis: StiffnessBasis, declared_ratio: f64) -> f64 {
    match basis {
        StiffnessBasis::Simplified => simplified_factor(),
        StiffnessBasis::Declared => declared_ratio.min(1.0),
    }
}

/// 🪨 Factor on the entered storey stiffnesses of a building.
pub fn building_factor(building: &En1998Building) -> f64 {
    factor(building.storey_stiffness_basis, building.storey_stiffness_ratio)
}

/// 🪨 Factor on the stiffness of a wall row.
pub fn member_factor(member: &En1998Member) -> f64 {
    factor(member.effective_stiffness_basis, member.effective_stiffness_ratio)
}

/// 🏬 Effective lateral stiffness `K_x` [N/m] of a storey: the entered (gross) value times the building's factor.
pub fn storey_x(building: &En1998Building, storey: &En1998Storey) -> f64 {
    storey.stiffness_x * building_factor(building)
}

/// 🏬 Effective lateral stiffness `K_y` [N/m] of a storey: the entered (gross) value times the building's factor.
pub fn storey_y(building: &En1998Building, storey: &En1998Storey) -> f64 {
    storey.stiffness_y * building_factor(building)
}

/// 🏬 Effective storey stiffnesses `(K_x, K_y)` of all storeys of a building, bottom up.
pub fn storeys_xy(building: &En1998Building) -> (Vec<f64>, Vec<f64>) {
    (building.storeys.iter().map(|s| storey_x(building, s)).collect(), building.storeys.iter().map(|s| storey_y(building, s)).collect())
}

/// 🏬 Gross (uncracked) lateral stiffnesses `(K_x, K_y)` of all storeys of a building as entered, bottom up: the stiffness behind the upper bound of the fundamental period.
pub fn gross_storeys_xy(building: &En1998Building) -> (Vec<f64>, Vec<f64>) {
    (building.storeys.iter().map(|s| s.stiffness_x).collect(), building.storeys.iter().map(|s| s.stiffness_y).collect())
}

/// 🏷️ The stiffness basis as text, EN and DE: `(en, de)`, e.g. "cracked 50 % (simplified, 4.3.1(6))" or "effective 80 % (declared)".
pub fn basis_text(basis: StiffnessBasis, declared_ratio: f64) -> (String, String) {
    let percent = factor(basis, declared_ratio) * 100.0;
    match basis {
        StiffnessBasis::Simplified => (format!("cracked {percent:.0} % (simplified, 4.3.1(6))"), format!("gerissen {percent:.0} % (vereinfacht, 4.3.1(6))")),
        StiffnessBasis::Declared => (format!("effective {percent:.0} % (declared)"), format!("wirksam {percent:.0} % (angegeben)")),
    }
}

/// 🏷️ The stiffness basis of a building as text, EN and DE.
pub fn building_basis_text(building: &En1998Building) -> (String, String) {
    basis_text(building.storey_stiffness_basis, building.storey_stiffness_ratio)
}
