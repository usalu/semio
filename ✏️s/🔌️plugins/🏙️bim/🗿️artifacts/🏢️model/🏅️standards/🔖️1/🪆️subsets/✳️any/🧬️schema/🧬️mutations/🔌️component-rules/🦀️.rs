//! 🔌️ The rules every component and MEP leaf shares: what makes a component, a component override and an MEP element valid in a base. Pure, computed from the authored records and the text of their formulas
//! only (through the family rules), never from an inference. A leaf prefixes the field of the [`Fault`] with the record name on create (`["component", "family"]`) and uses it as is on set (`["family"]`).

use super::elements;
use super::family_rules::{self, Fault};
use crate::{Component, FamilyCategory, MepElement, MepShape, ModelSnapshot, Point3};
use std::collections::{BTreeMap, BTreeSet};

fn finite(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite())
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

/// 🧱️ Why `host` cannot carry a component standing on `storey`, none when it is a wall of that storey.
pub fn host_fault(base: &ModelSnapshot, storey: &str, host: &str) -> Option<Fault> {
    match base.walls.get(host) {
        None if elements::exists(base, host) => Some(Fault::invalid(format!("Element \"{host}\" is no wall: a component is mounted on a wall."), "host")),
        None => Some(Fault::missing("Wall", host, "host")),
        Some(wall) if wall.storey != storey => Some(Fault::invalid(format!("Wall \"{host}\" stands on storey \"{}\": a mounted component stands on the storey of its wall.", wall.storey), "host")),
        Some(_) => None,
    }
}

/// 🪑️ Why a component is not valid in `base`: its storey or family is missing, its family is a profile, a number is not finite or its host is no wall of its storey.
pub fn component_fault(base: &ModelSnapshot, component: &Component) -> Option<Fault> {
    if !base.storeys.contains_key(&component.storey) {
        return Some(Fault::missing("Storey", &component.storey, "storey"));
    }
    match base.families.get(&component.family) {
        None => return Some(Fault::missing("Family", &component.family, "family")),
        Some(family) if family.category == FamilyCategory::Profile => return Some(Fault::invalid(format!("Family \"{}\" is a profile and cannot be placed as a component.", component.family), "family")),
        Some(_) => {}
    }
    if !finite(&[component.position.x, component.position.y]) {
        return Some(Fault::invalid("A component position must be finite.", "position"));
    }
    if !finite(&[component.elevation]) {
        return Some(Fault::invalid("A component elevation must be a finite length.", "elevation"));
    }
    if !finite(&[component.rotation]) {
        return Some(Fault::invalid("A component rotation must be a finite angle.", "rotation"));
    }
    component.host.as_deref().and_then(|host| host_fault(base, &component.storey, host))
}

/// 🔩️ Why a cross-section is not valid: a width, height or diameter that is not a positive length.
pub fn shape_fault(shape: &MepShape) -> Option<Fault> {
    let sizes: Vec<f64> = match shape {
        MepShape::Duct { width, height } | MepShape::Tray { width, height } => vec![*width, *height],
        MepShape::Pipe { diameter } => vec![*diameter],
    };
    sizes.into_iter().any(|size| !positive(size)).then(|| Fault::invalid("A section needs positive dimensions.", "shape"))
}

fn apart(first: &Point3, second: &Point3) -> bool {
    (first.x - second.x).hypot(first.y - second.y).hypot(first.z - second.z) > 1e-9
}

/// 🌀️ Why a path cannot be routed: fewer than two points, a point that is not finite or two consecutive points that coincide.
pub fn path_fault(path: &[Point3]) -> Option<Fault> {
    if path.len() < 2 {
        return Some(Fault::invalid("A routed element needs at least two path points.", "path"));
    }
    if !path.iter().all(|point| finite(&[point.x, point.y, point.z])) {
        return Some(Fault::invalid("Every path point must be finite.", "path"));
    }
    path.windows(2).any(|pair| !apart(&pair[0], &pair[1])).then(|| Fault::invalid("Consecutive path points must differ.", "path"))
}

/// 🌀️ Why an MEP element is not valid in `base`: its storey is missing, its section has a dimension that is not positive or its path cannot be routed.
pub fn mep_fault(base: &ModelSnapshot, mep: &MepElement) -> Option<Fault> {
    if !base.storeys.contains_key(&mep.storey) {
        return Some(Fault::missing("Storey", &mep.storey, "storey"));
    }
    shape_fault(&mep.shape).or_else(|| path_fault(&mep.path))
}

/// ♻️ Why giving the override of parameter `name` of `component` the formula `text` would make parameters depend on each other in a circle, none when the dependencies stay acyclic. Every other override of the
/// component replaces the formula of its parameter in the dependency graph, so a circle that only exists under the overrides is found.
pub fn override_cycle_fault(base: &ModelSnapshot, component: &str, family: &str, name: &str, text: &str) -> Option<Fault> {
    let mut graph: BTreeMap<String, BTreeSet<String>> = base
        .family_parameters
        .values()
        .filter(|row| row.family == family)
        .map(|row| {
            let value = base.component_overrides.get(&family_rules::parameter_key(component, &row.name)).map_or(row.value.as_str(), |over| over.value.as_str());
            (row.name.clone(), family_rules::references(value))
        })
        .collect();
    graph.insert(name.to_string(), family_rules::references(text));
    let plan = semio_framework_expression::plan(&graph);
    plan.cycles.iter().find(|members| members.iter().any(|member| member == name)).map(|members| Fault::invalid(format!("The formula would make {} depend on each other in a circle.", members.join(", ")), "value"))
}

/// 🎚️ Why the formula `text` cannot override parameter `name` of `component`: the component or the parameter of its family does not exist, the formula does not parse, uses a name that is no parameter of the family
/// or closes a circle under the overrides of the component.
pub fn override_fault(base: &ModelSnapshot, component: &str, name: &str, text: &str) -> Option<Fault> {
    let Some(row) = base.components.get(component) else {
        return Some(Fault::missing("Component", component, "component"));
    };
    if !base.family_parameters.contains_key(&family_rules::parameter_key(&row.family, name)) {
        return Some(Fault::missing("Parameter", name, "name"));
    }
    family_rules::formula_fault(text, "value")
        .or_else(|| family_rules::reference_fault(base, &row.family, text, "value"))
        .or_else(|| override_cycle_fault(base, component, &row.family, name, text))
}

/// 🧬️ Why the component `component` cannot become an instance of `family`: one of its overrides names a parameter the new family lacks, uses a name it lacks or closes a circle under the new formulas.
pub fn family_swap_fault(base: &ModelSnapshot, component: &str, family: &str) -> Option<Fault> {
    let overrides: Vec<_> = base.component_overrides.values().filter(|row| row.component == component).collect();
    for row in &overrides {
        if !base.family_parameters.contains_key(&family_rules::parameter_key(family, &row.name)) {
            return Some(Fault::invalid(format!("Override \"{}\" of component \"{component}\" names no parameter of family \"{family}\"; remove it first.", row.name), "family"));
        }
        if let Some(fault) = family_rules::reference_fault(base, family, &row.value, "family") {
            return Some(Fault::invalid(format!("Override \"{}\" of component \"{component}\" cannot be evaluated in family \"{family}\": {}", row.name, fault.message), "family"));
        }
    }
    let graph: BTreeMap<String, BTreeSet<String>> = base
        .family_parameters
        .values()
        .filter(|row| row.family == family)
        .map(|row| {
            let value = overrides.iter().find(|over| over.name == row.name).map_or(row.value.as_str(), |over| over.value.as_str());
            (row.name.clone(), family_rules::references(value))
        })
        .collect();
    let plan = semio_framework_expression::plan(&graph);
    plan.cycles.first().map(|members| Fault::invalid(format!("The overrides of component \"{component}\" would make {} depend on each other in a circle in family \"{family}\".", members.join(", ")), "family"))
}

/// 🧱️ The components mounted on the wall `wall`, in id order.
pub fn mounted_on<'a>(base: &'a ModelSnapshot, wall: &'a str) -> impl Iterator<Item = &'a String> {
    base.components.iter().filter(move |(_, row)| row.host.as_deref() == Some(wall)).map(|(id, _)| id)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
