//! ↩️ `change-coefficient` — undo reconstructed from BASE's own value at `label`; missing or
//! non-numeric target ⇒ `Vec::new()` (nothing to undo).

use crate::standards::v1::subsets::any::schema::snapshot::EquationNodeKind;
use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeCoefficient, base: &EquationSnapshot) -> Vec<EquationMutation> {
    match base.equation.find(payload.label).map(|node| &node.kind) {
        Some(EquationNodeKind::Integer { lexeme }) => vec![EquationMutation::ChangeCoefficient(super::ChangeCoefficient { label: payload.label, numer: lexeme.clone(), denom: "1".to_string() })],
        Some(EquationNodeKind::Rational { numer, denom }) => vec![EquationMutation::ChangeCoefficient(super::ChangeCoefficient { label: payload.label, numer: numer.clone(), denom: denom.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
