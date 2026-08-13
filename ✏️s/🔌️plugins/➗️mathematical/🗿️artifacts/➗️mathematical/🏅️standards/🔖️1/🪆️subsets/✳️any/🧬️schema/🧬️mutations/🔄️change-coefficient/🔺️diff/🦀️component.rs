//! 🔺️ `change-coefficient` — computed from `(payload, base)`, never apply-then-capture. A no-op
//! (returns `base.equation` unchanged) when `payload.label` doesn't resolve to a numeric leaf in
//! `base` — a stale or foreign label is silently ignored, matching every other triad's
//! "missing target ⇒ no-op" convention in this file's siblings (`🏷️change-node-label`'s diff).

use super::mutation::ChangeCoefficient;
use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::EquationNodeKind;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCoefficient, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut equation = base.equation.clone();
    let targets_numeric_leaf = matches!(equation.find(payload.label).map(|node| &node.kind), Some(EquationNodeKind::Integer { .. }) | Some(EquationNodeKind::Rational { .. }));
    if targets_numeric_leaf {
        let new_kind = if payload.denom == "1" {
            EquationNodeKind::Integer { lexeme: payload.numer.clone() }
        } else {
            EquationNodeKind::Rational { numer: payload.numer.clone(), denom: payload.denom.clone() }
        };
        equation.replace(payload.label, new_kind);
    }
    MathematicalDiff { equation: Some(equation), ..Default::default() }
}
//#endregion 🔖️Diff
