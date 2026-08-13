//! ↩️ `change-coefficient` — undo reconstructed from BASE's own value at `label`; missing or
//! non-numeric target ⇒ `Vec::new()` (nothing to undo).

use super::mutation::ChangeCoefficient;
use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::EquationNodeKind;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeCoefficient, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    match base.equation.find(payload.label).map(|node| &node.kind) {
        Some(EquationNodeKind::Integer { lexeme }) => vec![MathematicalMutation::ChangeCoefficient(ChangeCoefficient { label: payload.label, numer: lexeme.clone(), denom: "1".to_string() })],
        Some(EquationNodeKind::Rational { numer, denom }) => vec![MathematicalMutation::ChangeCoefficient(ChangeCoefficient { label: payload.label, numer: numer.clone(), denom: denom.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
