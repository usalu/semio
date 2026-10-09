//! 🏧️ `set-curtain-wall-grid` payload. Sets, per direction, the grid rule of one curtain wall that replaces the rule of its type: explicit grid lines (a whole list owned by the curtain wall) or a uniform spacing; assigning nothing clears the override and the wall follows its type again.

use crate::{Assigned, CurtainGrid, CurtainWallPatch, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCurtainWallGrid {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub u_grid: Option<Assigned<Option<CurtainGrid>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub v_grid: Option<Assigned<Option<CurtainGrid>>>,
}

impl SetCurtainWallGrid {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> CurtainWallPatch {
        CurtainWallPatch { u_grid: self.u_grid.clone(), v_grid: self.v_grid.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: CurtainWallPatch) -> Self {
        Self { id, u_grid: patch.u_grid, v_grid: patch.v_grid }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetCurtainWallGrid {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "curtain-wall", kind: "set-curtain-wall-grid", record: "SetCurtainWallGrid" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set the grid of curtain wall \"{}\"", self.id), &format!("Raster von Vorhangfassade \"{}\" setzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
