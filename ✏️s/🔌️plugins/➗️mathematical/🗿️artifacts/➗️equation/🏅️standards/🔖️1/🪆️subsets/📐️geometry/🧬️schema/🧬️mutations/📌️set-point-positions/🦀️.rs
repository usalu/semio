//! 📌️ `set-point-positions` — absolute positions of a set of points in the geometry playground's point cloud in ONE row,
//! addressed by BASE-state index: the exact undo of a `move-points` drag (every moved point back at its base position) and
//! of itself.

use crate::{EquationMutation, EquationSnapshot};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
/// 📍️ One point's absolute canvas position, addressed by its BASE-state index.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct EquationPointPosition {
    pub index: usize,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPointPositions {
    pub positions: Vec<EquationPointPosition>,
}

impl SetPointPositions {
    /// 🔢️ The positioned point indices, in payload order.
    pub fn indices(&self) -> Vec<usize> {
        self.positions.iter().map(|position| position.index).collect()
    }
}

/// 🧱️ The payload-intrinsic target law every multi-point leaf states in its schema: at least one point, each once.
pub fn equation_point_targets_invariant(indices: &[usize]) -> Result<(), &'static str> {
    if indices.is_empty() {
        return Err("a multi-point leaf names at least one point");
    }
    if indices.iter().enumerate().any(|(at, index)| indices[..at].contains(index)) {
        return Err("a multi-point leaf names each point once");
    }
    Ok(())
}

/// 🏷️ The message and target addresses of a set of point indices.
pub fn equation_point_targets(indices: &[usize]) -> Vec<String> {
    indices.iter().map(usize::to_string).collect()
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for SetPointPositions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "point-positions", kind: "set-point-positions", record: "SetPointPositions" };

    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set the positions of {} point(s)", self.positions.len()), &format!("Positionen von {} Punkt(en) setzen", self.positions.len()))
    }
    fn target(&self) -> Vec<String> {
        equation_point_targets(&self.indices())
    }
}
//#endregion 🔖️Payload
