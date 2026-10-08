//! 📏️ Equation editor command — `edit-points`: the edited point list as the concrete point kinds of every change
//! (`set-point-positions`, `insert-point`, `remove-point`), never a whole-geometry replace.

use crate::op::EquationMutation;
use crate::{EquationGeometry, EquationSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "edit-points")]
pub struct EditPoints {
    #[dsl(block)]
    pub geometry: EquationGeometry,
}

pub fn handle(payload: &EditPoints, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(crate::editor::equation::commands::edit_equation::equation_point_edit_leaves(&doc.snapshot.geometry.points, &payload.geometry.points)))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
