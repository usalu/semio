//! 🩺️ The findings about the MEP elements of one storey: elements without a usable section or without a run of positive length, coordinates that are not numbers, and the clashes between elements of different systems
//! (from the `MepClashes` node of the storey, which does the expensive scan once). Computed from the `Mep` values the `Diagnostics` node of the storey has as parents.

use super::MepIssueCode;
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::{Diagnostic, DiagnosticCode, Inputs};

/// 🩺️ The findings about the MEP elements of `storey`.
pub fn storey(storey: &str, inputs: &Inputs<'_>) -> Vec<Diagnostic> {
    let mut found = Vec::new();
    for (id, value) in inputs.meps.iter().filter(|(_, value)| value.storey == storey) {
        if value.issues.iter().any(|issue| issue.code == MepIssueCode::NonFinite) {
            found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
        } else if value.issues.iter().any(|issue| matches!(issue.code, MepIssueCode::SectionDegenerate | MepIssueCode::PathDegenerate)) {
            found.push(Diagnostic::new(DiagnosticCode::MepDegenerate, &[id]).on(storey).with("width", value.section.width).with("height", value.section.height).with("length", value.length));
        }
    }
    if let Some(clashes) = inputs.clashes {
        found.extend(clashes.pairs.iter().map(|pair| Diagnostic::new(DiagnosticCode::MepClash, &[&pair.a, &pair.b]).on(storey).with("distance", pair.distance).with("reach", pair.reach)));
    }
    found
}
