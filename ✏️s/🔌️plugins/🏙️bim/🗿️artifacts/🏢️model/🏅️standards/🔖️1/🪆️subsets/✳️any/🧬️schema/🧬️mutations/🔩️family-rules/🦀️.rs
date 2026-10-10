//! 🧩️ The rules every family leaf shares: what makes a family, a family parameter and a family solid valid in a base, who uses a family and who refers to a parameter. Pure, computed from the authored
//! records and the text of their formulas only (`semio_framework_expression::dependencies` on the parsed text), never from an inference. A leaf prefixes the field of the [`Fault`] with the record name
//! on create (`["solid", "height"]`) and uses it as is on set (`["height"]`).

use crate::standards::v1::subsets::any::schema::authored::formula;
use crate::{Family, FamilyParameter, FamilySolid, ModelSnapshot, ParametricProfile, Profile, SolidShape};
use protocol::OutcomeCode;

/// 🚫️ A rule broken: the outcome code, the message and the field of the record that breaks it.
pub struct Fault {
    pub code: OutcomeCode,
    pub message: String,
    pub field: String,
}

impl Fault {
    /// 🔎️ The fault of a record `noun` named `id` that does not exist, reported on `field`.
    pub fn missing(noun: &str, id: &str, field: &str) -> Self {
        Self { code: OutcomeCode::TargetMissing, message: format!("{noun} \"{id}\" does not exist."), field: field.to_string() }
    }

    /// 🚫️ The fault of a broken invariant, reported on `field`.
    pub fn invalid(message: impl Into<String>, field: &str) -> Self {
        Self { code: OutcomeCode::Invariant, message: message.into(), field: field.to_string() }
    }

    /// 🧭️ The target path of the refusal under `record` (empty for a set leaf).
    pub fn path(&self, record: Option<&'static str>) -> Vec<String> {
        record.into_iter().map(str::to_string).chain([self.field.clone()]).collect()
    }
}

/// 🧩 Why a family record is not valid: its name is blank.
pub fn family_record_fault(record: &Family) -> Option<Fault> {
    record.name.trim().is_empty().then(|| Fault::invalid("A family needs a name.", "name"))
}

/// 📝️ Why `text` is not a formula the expression language reads, none when it parses.
pub fn formula_fault(text: &str, field: &str) -> Option<Fault> {
    formula::parse_formula(text).err().map(|error| Fault::invalid(format!("The formula \"{text}\" of {field} does not parse: {}.", error.message()), field))
}

/// 🔎️ Why a formula refers to a name that is no parameter of the family, none when every name it uses exists. Authored references always resolve: a parameter is created before the formulas that use it.
pub fn reference_fault(base: &ModelSnapshot, family: &str, text: &str, field: &str) -> Option<Fault> {
    let unknown: Vec<String> = formula::references(text).into_iter().filter(|name| !base.family_parameters.contains_key(&formula::parameter_id(family, name))).collect();
    (!unknown.is_empty()).then(|| Fault::missing("Parameter", &unknown.join("\", \""), field))
}

/// ♻️ Why giving parameter `name` the formula `text` would make parameters depend on each other in a circle, none when the dependencies stay acyclic.
pub fn cycle_fault(base: &ModelSnapshot, family: &str, name: &str, text: &str) -> Option<Fault> {
    let mut graph: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> = base.family_parameters.values().filter(|row| row.family == family).map(|row| (row.name.clone(), formula::references(&row.value))).collect();
    graph.insert(name.to_string(), formula::references(text));
    let plan = semio_framework_expression::plan(&graph);
    plan.cycles.iter().find(|members| members.iter().any(|member| member == name)).map(|members| Fault::invalid(format!("The formula would make {} depend on each other in a circle.", members.join(", ")), "value"))
}

/// 🔤️ Why a parameter name is not usable.
pub fn name_fault(name: &str) -> Option<Fault> {
    (!formula::is_name(name)).then(|| Fault::invalid(format!("\"{name}\" is not a parameter name: use letters, digits and underscores, not starting with a digit."), "name"))
}

/// 🧊️ Why a family solid is not valid in `base`: its family is missing, its name is blank, a formula slot does not parse or names an unknown parameter, or its shape has too few points.
pub fn solid_fault(base: &ModelSnapshot, solid: &FamilySolid) -> Option<Fault> {
    if !base.families.contains_key(&solid.family) {
        return Some(Fault::missing("Family", &solid.family, "family"));
    }
    if solid.name.trim().is_empty() {
        return Some(Fault::invalid("A family solid needs a name.", "name"));
    }
    if let Some(fault) = formula::solid_slots(solid).into_iter().find_map(|(field, text)| formula_fault(text, &field).or_else(|| reference_fault(base, &solid.family, text, &field))) {
        return Some(fault);
    }
    shape_fault(&solid.shape)
}

/// 🔷️ Why a shape lacks the points it needs: a polygon has at least three, a sweep path at least two.
pub fn shape_fault(shape: &SolidShape) -> Option<Fault> {
    let polygon = |profile: &ParametricProfile| matches!(profile, ParametricProfile::Polygon { points } if points.len() < 3).then(|| Fault::invalid("A polygon profile needs at least three points.", "shape"));
    match shape {
        SolidShape::Extrusion { profile, .. } | SolidShape::Revolution { profile, .. } => polygon(profile),
        SolidShape::Sweep { profile, path } => polygon(profile).or_else(|| (path.len() < 2).then(|| Fault::invalid("A sweep path needs at least two points.", "shape"))),
        SolidShape::Cuboid { .. } => None,
    }
}

/// 🔗️ Who uses family `family` as a profile, none when nobody does: a column type, a beam type, a curtain wall type mullion, a wall sweep or a railing section.
pub fn profile_user(base: &ModelSnapshot, family: &str) -> Option<&'static str> {
    let is = |profile: &Profile| matches!(profile, Profile::Family { family: used } if used == family);
    [
        ("column types", base.column_types.values().any(|row| is(&row.profile))),
        ("beam types", base.beam_types.values().any(|row| is(&row.profile))),
        ("curtain wall types", base.curtain_wall_types.values().any(|row| is(&row.interior_mullion) || is(&row.border_mullion))),
        ("wall sweeps", base.wall_sweeps.values().any(|row| is(&row.profile))),
        ("railings", base.railings.values().any(|row| is(&row.profile) || is(&row.post_profile) || row.baluster.as_ref().is_some_and(|baluster| is(&baluster.profile)))),
    ]
    .into_iter()
    .find_map(|(noun, used)| used.then_some(noun))
}

/// 🪑️ Who places family `family`, none when nobody does: a component is an instance of it and its solids are the evaluated solids of the family.
pub fn component_user(base: &ModelSnapshot, family: &str) -> Option<&'static str> {
    base.components.values().any(|row| row.family == family).then_some("components")
}

/// 🔑️ The key of the parameter or override `name` of the owner `owner` (a family or a component) in `family_parameters` or `component_overrides`.
pub fn parameter_key(owner: &str, name: &str) -> String {
    formula::parameter_id(owner, name)
}

/// 🔗️ The names a formula text refers to, empty when it does not parse.
pub fn references(text: &str) -> std::collections::BTreeSet<String> {
    formula::references(text)
}

/// 🔗️ Who refers to parameter `name` of `family` in a formula, none when nobody does: another parameter of the family or one of its solids.
pub fn parameter_user(base: &ModelSnapshot, family: &str, name: &str) -> Option<String> {
    let refers = |text: &str| formula::references(text).contains(name);
    let other = base.family_parameters.values().find(|row| row.family == family && row.name != name && refers(&row.value));
    if let Some(row) = other {
        return Some(format!("the formula of parameter \"{}\"", row.name));
    }
    base.family_solids.iter().filter(|(_, row)| row.family == family).find_map(|(id, row)| formula::solid_slots(row).into_iter().find(|(_, text)| refers(text)).map(|(field, _)| format!("the {field} of solid \"{id}\"")))
}

/// 🧭️ The parameters of `family` with the ones other formulas use first, so that creating them one after the other never meets a name that does not exist yet; parameters in a circle (only an import can bring
/// one) come last, by name.
pub fn parameters_in_order<'a>(base: &'a ModelSnapshot, family: &str) -> Vec<&'a FamilyParameter> {
    let rows: std::collections::BTreeMap<&str, &FamilyParameter> = base.family_parameters.values().filter(|row| row.family == family).map(|row| (row.name.as_str(), row)).collect();
    let graph: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> = rows.iter().map(|(name, row)| ((*name).to_string(), formula::references(&row.value))).collect();
    let mut names = semio_framework_expression::plan(&graph).order;
    let rest: Vec<String> = rows.keys().map(|name| (*name).to_string()).filter(|name| !names.contains(name)).collect();
    names.extend(rest);
    names.iter().filter_map(|name| rows.get(name.as_str()).copied()).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
