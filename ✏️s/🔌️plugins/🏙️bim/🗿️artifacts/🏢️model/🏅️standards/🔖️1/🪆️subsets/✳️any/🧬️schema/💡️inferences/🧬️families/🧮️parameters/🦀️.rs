//! 🧮️ `parameters`: the resolved parameters of one family. The authored formulas (and, for a placed instance, its override formulas) are parsed, ordered by their dependencies and evaluated by
//! `semio_framework_expression::evaluate_all_declared`; a parameter that does not parse, has the wrong kind, takes part in a cycle or depends on one that failed becomes an issue, never a stored value.

use super::issues::{FamilyIssue, FamilyIssueCode, IssueOwner};
use super::{ParameterValue, ResolvedParameter};
use crate::{FamilyParameter, ModelSnapshot, ParameterKind};
use semio_framework_expression::{evaluate_all_declared, parse, print, Expr, Kind, Value};
use std::collections::{BTreeMap, BTreeSet};

/// 🧭️ The kind a parameter kind computes: integers and reals are plain numbers, a material is the text of its id.
pub const fn kind_of(kind: ParameterKind) -> Kind {
    match kind {
        ParameterKind::Length => Kind::Length,
        ParameterKind::Angle => Kind::Angle,
        ParameterKind::Real | ParameterKind::Integer => Kind::Number,
        ParameterKind::Boolean => Kind::Bool,
        ParameterKind::Text | ParameterKind::Material => Kind::Text,
    }
}

/// 🔁️ The stored form of an evaluated value (SI base units: metres, radians).
pub fn stored(value: &Value) -> Option<ParameterValue> {
    Some(match value {
        Value::Number(value) => ParameterValue::Number { value: *value },
        Value::Length(value) => ParameterValue::Length { value: *value },
        Value::Angle(value) => ParameterValue::Angle { value: *value },
        Value::Bool(value) => ParameterValue::Boolean { value: *value },
        Value::Text(value) => ParameterValue::Text { value: value.clone() },
        Value::Area(_) | Value::Volume(_) => return None,
    })
}

/// 🎁️ Everything the evaluation of the parameters of one family gives.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Resolution {
    pub order: Vec<String>,
    pub parameters: BTreeMap<String, ResolvedParameter>,
    pub env: BTreeMap<String, Value>,
    pub names: BTreeSet<String>,
    pub issues: Vec<FamilyIssue>,
}

/// 📖️ The authored parameters of `family`, by name.
pub fn authored<'a>(snapshot: &'a ModelSnapshot, family: &str) -> BTreeMap<&'a str, &'a FamilyParameter> {
    snapshot.family_parameters.values().filter(|row| row.family == family).map(|row| (row.name.as_str(), row)).collect()
}

/// ▶️ Resolves the parameters of `family`; `overrides` are formula texts that replace the formulas of existing parameters (placed instances).
pub fn resolve(snapshot: &ModelSnapshot, family: &str, overrides: &BTreeMap<String, String>) -> Resolution {
    let rows = authored(snapshot, family);
    let mut resolution = Resolution { names: rows.keys().map(|name| (*name).to_string()).collect(), ..Resolution::default() };
    let mut formulas: BTreeMap<String, Expr> = BTreeMap::new();
    let mut replaced: BTreeMap<String, Expr> = BTreeMap::new();
    let mut texts: BTreeMap<String, String> = BTreeMap::new();
    let mut unparsed: BTreeSet<String> = BTreeSet::new();
    for (name, row) in &rows {
        texts.insert((*name).to_string(), row.value.clone());
        match parse(&row.value) {
            Ok(expr) => {
                formulas.insert((*name).to_string(), expr);
            }
            Err(error) => {
                unparsed.insert((*name).to_string());
                resolution.issues.push(FamilyIssue::of_parse(IssueOwner::Parameter, name, "value", &error));
            }
        }
    }
    for (name, text) in overrides {
        match parse(text) {
            Ok(expr) => {
                texts.insert(name.clone(), print(&expr));
                replaced.insert(name.clone(), expr);
            }
            Err(error) => {
                unparsed.insert(name.clone());
                resolution.issues.push(FamilyIssue::of_parse(IssueOwner::Parameter, name, "value", &error));
            }
        }
    }
    let declared: BTreeMap<String, Kind> = rows.iter().filter(|(name, _)| !unparsed.contains(**name)).map(|(name, row)| ((*name).to_string(), kind_of(row.kind))).collect();
    let live: BTreeMap<String, Expr> = formulas.into_iter().filter(|(name, _)| !unparsed.contains(name)).collect();
    let resolved = evaluate_all_declared(&live, &replaced, &declared);
    let known = |name: &str| resolution.names.contains(name);
    let found: Vec<FamilyIssue> = resolved.errors.iter().map(|(name, error)| FamilyIssue::of_error(IssueOwner::Parameter, name, "value", error, &known)).collect();
    resolution.issues.extend(found);
    resolution.order = resolved.order.clone();
    for (name, row) in &rows {
        let value = resolved.values.get(*name);
        let stored_value = value.and_then(stored);
        if let (Some(value), ParameterKind::Integer) = (value, row.kind) {
            if value.magnitude().is_some_and(|number| (number - number.round()).abs() > 1e-9) {
                resolution.issues.push(FamilyIssue::new(FamilyIssueCode::Kind, IssueOwner::Parameter, name, "value", "an integer parameter must be a whole number", Vec::new()));
            }
        }
        if let (Some(Value::Text(id)), ParameterKind::Material) = (value, row.kind) {
            if !snapshot.materials.contains_key(id) {
                resolution.issues.push(FamilyIssue::new(FamilyIssueCode::Unknown, IssueOwner::Parameter, name, "value", format!("material `{id}` does not exist"), vec![id.clone()]));
            }
        }
        resolution.parameters.insert((*name).to_string(), ResolvedParameter { kind: row.kind, formula: texts.get(*name).cloned().unwrap_or_default(), value: stored_value });
    }
    resolution.env = resolved.values;
    resolution
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
