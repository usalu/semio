//! 🔺️ `change-coefficient` — computed from `(payload, base)`, never apply-then-capture. Error
//! `target-missing` when `payload.label` doesn't resolve to a numeric leaf in `base` — a stale or
//! foreign label, or a label that resolves to a non-numeric node, cannot be changed.

use super::mutation::ChangeCoefficient;
use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::EquationNodeKind;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCoefficient, base: &MathematicalSnapshot) -> protocol::MutationOutcome<MathematicalDiff> {
    let mut equation = base.equation.clone();
    let Some(node) = equation.find(payload.label) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Equation node {} does not exist.", payload.label.0), [payload.label.0.to_string()]);
    };
    let current = match &node.kind {
        EquationNodeKind::Integer { lexeme } => Some((lexeme.clone(), "1".to_string())),
        EquationNodeKind::Rational { numer, denom } => Some((numer.clone(), denom.clone())),
        _ => None,
    };
    let Some((current_numer, current_denom)) = current else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Equation node {} is not a numeric leaf.", payload.label.0), [payload.label.0.to_string()]);
    };
    if payload.denom == "0" {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Coefficient {} cannot have a zero denominator.", payload.label.0), [payload.label.0.to_string()]);
    }
    if current_numer == payload.numer && current_denom == payload.denom {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Coefficient {} is already {}/{}.", payload.label.0, payload.numer, payload.denom));
    }
    let new_kind = if payload.denom == "1" {
        EquationNodeKind::Integer { lexeme: payload.numer.clone() }
    } else {
        EquationNodeKind::Rational { numer: payload.numer.clone(), denom: payload.denom.clone() }
    };
    equation.replace(payload.label, new_kind);
    protocol::MutationOutcome::new(MathematicalDiff { equation: Some(equation), ..Default::default() })
}
//#endregion 🔖️Diff
