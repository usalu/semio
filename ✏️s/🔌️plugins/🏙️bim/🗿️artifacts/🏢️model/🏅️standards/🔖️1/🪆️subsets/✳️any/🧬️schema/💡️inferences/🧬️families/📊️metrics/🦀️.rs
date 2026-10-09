//! 📊️ `metrics`: the canonical JSON table the family oracle compares, by family id: the kind, formula and value of every parameter, the issues as `code|owner|subject|field`, and per solid its
//! visibility, material, volume, area, triangle count and bounds, plus the area and vertex count of the outline of a profile family. The python oracle re-derives the whole table from the same committed snapshot
//! with its own evaluator and numpy/shapely geometry; the committed expectation is written by that file, never by hand.

use super::{FamilyValue, ParameterValue, ResolvedParameter};
use semio_framework_geometry::loops;
use std::collections::BTreeMap;

/// 🧊️ What the oracle compares of one solid.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct SolidMetrics {
    pub visible: bool,
    pub material: String,
    pub volume: f64,
    pub area: f64,
    pub triangles: u32,
    pub min: Vec<f64>,
    pub max: Vec<f64>,
}

/// 🧬️ What the oracle compares of one family.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FamilyMetrics {
    pub category: String,
    pub parameters: BTreeMap<String, ResolvedParameter>,
    pub issues: Vec<String>,
    pub solids: BTreeMap<String, SolidMetrics>,
    pub outline_area: f64,
    pub outline_vertices: u32,
}

/// 📐️ The metrics of one family.
pub fn metrics_of(family: &FamilyValue) -> FamilyMetrics {
    let issues = {
        let mut rows: Vec<String> = family.issues.iter().map(|issue| format!("{}|{:?}|{}|{}", issue.code.slug(), issue.owner, issue.subject, issue.field).to_lowercase()).collect();
        rows.sort();
        rows
    };
    let solids = family
        .solids
        .iter()
        .map(|(id, solid)| (id.clone(), SolidMetrics { visible: solid.visible, material: solid.material.clone(), volume: solid.volume, area: solid.area, triangles: (solid.indices.len() / 3) as u32, min: vec![solid.bounds.min.x, solid.bounds.min.y, solid.bounds.min.z], max: vec![solid.bounds.max.x, solid.bounds.max.y, solid.bounds.max.z] }))
        .collect();
    let outline: Vec<loops::Vertex> = family.outline.iter().map(|vertex| loops::Vertex::new(semio_framework_geometry::Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect();
    FamilyMetrics { category: family.category.map(|category| format!("{category:?}")).unwrap_or_default(), parameters: family.parameters.clone(), issues, solids, outline_area: loops::area(&outline), outline_vertices: family.outline.len() as u32 }
}

/// 📏️ The canonical JSON table `family → metrics` the family oracle compares.
pub fn table_json(families: &BTreeMap<String, FamilyValue>) -> String {
    let table: BTreeMap<String, FamilyMetrics> = families.iter().map(|(id, family)| (id.clone(), metrics_of(family))).collect();
    semio_framework_pack_json::to_json_string(&table)
}

/// 🔢️ The magnitude of a numeric parameter value in SI base units, none for a truth value or a text.
pub fn magnitude(value: &ParameterValue) -> Option<f64> {
    match value {
        ParameterValue::Number { value } | ParameterValue::Length { value } | ParameterValue::Angle { value } => Some(*value),
        ParameterValue::Boolean { .. } | ParameterValue::Text { .. } => None,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
