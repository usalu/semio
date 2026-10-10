//! 🏷️ The words of the family window in a locale: the name of a category, of a parameter kind, of a solid shape and of an axis, and the text of an evaluated parameter value.

use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::inferences::families::ParameterValue;
use crate::{FamilyCategory, ParameterKind, SolidAxis};

/// 🏷️ The name of a family category.
pub fn category_label(labels: &BimLabels, category: FamilyCategory) -> String {
    let label = match category {
        FamilyCategory::Furniture => labels.fam_cat_furniture,
        FamilyCategory::Equipment => labels.fam_cat_equipment,
        FamilyCategory::Casework => labels.fam_cat_casework,
        FamilyCategory::Plumbing => labels.fam_cat_plumbing,
        FamilyCategory::Lighting => labels.fam_cat_lighting,
        FamilyCategory::Mechanical => labels.fam_cat_mechanical,
        FamilyCategory::Electrical => labels.fam_cat_electrical,
        FamilyCategory::Generic => labels.fam_cat_generic,
        FamilyCategory::Profile => labels.fam_cat_profile,
    };
    label.as_str().to_string()
}

/// 🏷️ The name of a parameter kind.
pub fn kind_label(labels: &BimLabels, kind: ParameterKind) -> String {
    let label = match kind {
        ParameterKind::Length => labels.fam_kind_length,
        ParameterKind::Angle => labels.fam_kind_angle,
        ParameterKind::Real => labels.fam_kind_real,
        ParameterKind::Integer => labels.fam_kind_integer,
        ParameterKind::Boolean => labels.fam_kind_boolean,
        ParameterKind::Text => labels.fam_kind_text,
        ParameterKind::Material => labels.fam_kind_material,
    };
    label.as_str().to_string()
}

/// 🏷️ The name of a solid shape token (`Cuboid`, `Extrusion`, `Revolution`, `Sweep`).
pub fn shape_label(labels: &BimLabels, token: &str) -> String {
    match token {
        "Cuboid" => labels.fam_shape_cuboid,
        "Extrusion" => labels.fam_shape_extrusion,
        "Revolution" => labels.fam_shape_revolution,
        _ => labels.fam_shape_sweep,
    }
    .as_str()
    .to_string()
}

/// 🏷️ The name of an axis.
pub fn axis_label(axis: SolidAxis) -> &'static str {
    match axis {
        SolidAxis::X => "X",
        SolidAxis::Y => "Y",
        SolidAxis::Z => "Z",
    }
}

fn trimmed(value: f64) -> String {
    let text = format!("{value:.6}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// 🔢️ The text of an evaluated value: a length in millimetres, an angle in degrees, a number as it is, a truth value and a text localized.
pub fn value_text(labels: &BimLabels, value: &ParameterValue) -> String {
    match value {
        ParameterValue::Length { value } => format!("{} mm", trimmed(value * 1000.0)),
        ParameterValue::Angle { value } => format!("{}°", trimmed(value.to_degrees())),
        ParameterValue::Number { value } => trimmed(*value),
        ParameterValue::Boolean { value } => (if *value { labels.fam_true } else { labels.fam_false }).as_str().to_string(),
        ParameterValue::Text { value } => value.clone(),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
