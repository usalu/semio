//! 🔺️ `change-coefficient` — one label-addressed kind patch. Error
//! `target-missing` when `payload.label` doesn't resolve to a numeric leaf in `base` — a stale or
//! foreign label, or a label that resolves to a non-numeric node, cannot be changed.

use crate::standards::v1::subsets::any::schema::snapshot::EquationNodeKind;
use crate::diff::{EquationExprDiff, EquationKindPatch};
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeCoefficient, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let Some(node) = base.equation.find(payload.label) else {
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
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Coefficient {} is already {}/{}.", payload.label.0, payload.numer, payload.denom));
    }
    let new_kind = if payload.denom == "1" { EquationNodeKind::Integer { lexeme: payload.numer.clone() } } else { EquationNodeKind::Rational { numer: payload.numer.clone(), denom: payload.denom.clone() } };
    let patch = EquationKindPatch { label: payload.label, kind: new_kind };
    protocol::MutationOutcome::new(EquationDiff { equation: Some(EquationExprDiff { kinds: vec![patch], next_label: None }), ..Default::default() })
}
//#endregion 🔖️Diff
