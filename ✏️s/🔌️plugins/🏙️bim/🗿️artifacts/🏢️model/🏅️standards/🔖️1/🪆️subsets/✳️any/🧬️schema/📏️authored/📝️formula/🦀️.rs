//! 📝️ `formula`: the authored text of a family formula. A formula is stored as the canonical text of `semio_framework_expression::print`; this module parses and canonicalises it (the editor does so before
//! it emits a mutation, a diff refuses text that does not parse) and reads the parameters it refers to, all pure and without any evaluation. It also owns the grammar of parameter names, the key
//! of a parameter record, `family.name`, and the list of every formula slot of a solid.

use crate::{ExprPoint, FamilySolid, ParametricProfile, SolidShape};
use semio_framework_expression::{dependencies, parse, print, Expr, ParseError};
use std::collections::BTreeSet;

/// 🔤️ Whether `text` is a usable parameter name: letters, digits and underscores, not starting with a digit, and not a keyword of the expression language.
pub fn is_name(text: &str) -> bool {
    let mut chars = text.chars();
    chars.next().is_some_and(|first| first.is_alphabetic() || first == '_') && chars.all(|c| c.is_alphanumeric() || c == '_') && !semio_framework_expression::syntax::KEYWORDS.contains(&text)
}

/// 🔑️ The key of the parameter `name` of `family` in `family_parameters`.
pub fn parameter_id(family: &str, name: &str) -> String {
    format!("{family}.{name}")
}

/// 🔑️ The family and the name a parameter key is made of; names never contain a dot, so the last dot splits.
pub fn split_parameter_id(id: &str) -> Option<(&str, &str)> {
    id.rsplit_once('.').filter(|(family, name)| !family.is_empty() && is_name(name))
}

/// 📖️ The expression a formula text stands for.
pub fn parse_formula(text: &str) -> Result<Expr, ParseError> {
    parse(text)
}

/// 🖨️ The canonical text of a formula: parsed and printed again, so equal formulas have equal text.
pub fn canonical(text: &str) -> Result<String, ParseError> {
    parse(text).map(|expr| print(&expr))
}

/// ✔️ Whether `text` already is the canonical text of a formula.
pub fn is_canonical(text: &str) -> bool {
    canonical(text).is_ok_and(|canon| canon == text)
}

/// 🔗️ The parameter names a formula text refers to (every name, taken branches or not); empty for text that does not parse.
pub fn references(text: &str) -> BTreeSet<String> {
    parse(text).map(|expr| dependencies(&expr)).unwrap_or_default()
}

fn point_slots<'a>(prefix: &str, points: &'a [ExprPoint], out: &mut Vec<(String, &'a str)>) {
    for (index, point) in points.iter().enumerate() {
        out.push((format!("{prefix}[{index}].x"), point.x.as_str()));
        out.push((format!("{prefix}[{index}].y"), point.y.as_str()));
    }
}

fn profile_slots<'a>(prefix: &str, profile: &'a ParametricProfile, out: &mut Vec<(String, &'a str)>) {
    match profile {
        ParametricProfile::Rectangle { width, depth } => out.extend([(format!("{prefix}.width"), width.as_str()), (format!("{prefix}.depth"), depth.as_str())]),
        ParametricProfile::Circle { diameter } => out.push((format!("{prefix}.diameter"), diameter.as_str())),
        ParametricProfile::IShape { width, depth, web, flange } => out.extend([(format!("{prefix}.width"), width.as_str()), (format!("{prefix}.depth"), depth.as_str()), (format!("{prefix}.web"), web.as_str()), (format!("{prefix}.flange"), flange.as_str())]),
        ParametricProfile::Polygon { points } => point_slots(&format!("{prefix}.points"), points, out),
    }
}

/// 🧾️ Every formula slot of a solid as `(field, text)`, the fields named like the issues of the inference (`profile.width`, `path[1].x`, `offset.z`, `height`, `material`, `visible`).
pub fn solid_slots(solid: &FamilySolid) -> Vec<(String, &str)> {
    let mut out: Vec<(String, &str)> = Vec::new();
    match &solid.shape {
        SolidShape::Extrusion { profile, base, height } => {
            profile_slots("profile", profile, &mut out);
            out.extend([("base".to_string(), base.as_str()), ("height".to_string(), height.as_str())]);
        }
        SolidShape::Revolution { profile, angle, .. } => {
            profile_slots("profile", profile, &mut out);
            out.push(("angle".to_string(), angle.as_str()));
        }
        SolidShape::Sweep { profile, path } => {
            profile_slots("profile", profile, &mut out);
            point_slots("path", path, &mut out);
        }
        SolidShape::Cuboid { x, y, z, width, depth, height } => out.extend([("x", x), ("y", y), ("z", z), ("width", width), ("depth", depth), ("height", height)].map(|(field, text)| (field.to_string(), text.as_str()))),
    }
    out.extend([("material".to_string(), solid.material.as_str()), ("visible".to_string(), solid.visible.as_str()), ("offset.x".to_string(), solid.offset.x.as_str()), ("offset.y".to_string(), solid.offset.y.as_str()), ("offset.z".to_string(), solid.offset.z.as_str())]);
    out
}

fn profile_slots_mut<'a>(prefix: &str, profile: &'a mut ParametricProfile, out: &mut Vec<(String, &'a mut String)>) {
    match profile {
        ParametricProfile::Rectangle { width, depth } => out.extend([(format!("{prefix}.width"), width), (format!("{prefix}.depth"), depth)]),
        ParametricProfile::Circle { diameter } => out.push((format!("{prefix}.diameter"), diameter)),
        ParametricProfile::IShape { width, depth, web, flange } => out.extend([(format!("{prefix}.width"), width), (format!("{prefix}.depth"), depth), (format!("{prefix}.web"), web), (format!("{prefix}.flange"), flange)]),
        ParametricProfile::Polygon { points } => {
            for (index, point) in points.iter_mut().enumerate() {
                out.push((format!("{prefix}.points[{index}].x"), &mut point.x));
                out.push((format!("{prefix}.points[{index}].y"), &mut point.y));
            }
        }
    }
}

/// 🖊️ Replaces the formula in the slot `field` of a solid (the names of [`solid_slots`]); `false` when the solid has no such slot.
pub fn set_slot(solid: &mut FamilySolid, field: &str, text: &str) -> bool {
    let FamilySolid { shape, material, visible, offset, .. } = solid;
    let mut slots: Vec<(String, &mut String)> = Vec::new();
    match shape {
        SolidShape::Extrusion { profile, base, height } => {
            profile_slots_mut("profile", profile, &mut slots);
            slots.extend([("base".to_string(), base), ("height".to_string(), height)]);
        }
        SolidShape::Revolution { profile, angle, .. } => {
            profile_slots_mut("profile", profile, &mut slots);
            slots.push(("angle".to_string(), angle));
        }
        SolidShape::Sweep { profile, path } => {
            profile_slots_mut("profile", profile, &mut slots);
            for (index, point) in path.iter_mut().enumerate() {
                slots.push((format!("path[{index}].x"), &mut point.x));
                slots.push((format!("path[{index}].y"), &mut point.y));
            }
        }
        SolidShape::Cuboid { x, y, z, width, depth, height } => slots.extend([("x", x), ("y", y), ("z", z), ("width", width), ("depth", depth), ("height", height)].map(|(name, slot)| (name.to_string(), slot))),
    }
    slots.extend([("material".to_string(), material), ("visible".to_string(), visible), ("offset.x".to_string(), &mut offset.x), ("offset.y".to_string(), &mut offset.y), ("offset.z".to_string(), &mut offset.z)]);
    match slots.into_iter().find(|(name, _)| name == field) {
        Some((_, slot)) => {
            *slot = text.to_string();
            true
        }
        None => false,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
