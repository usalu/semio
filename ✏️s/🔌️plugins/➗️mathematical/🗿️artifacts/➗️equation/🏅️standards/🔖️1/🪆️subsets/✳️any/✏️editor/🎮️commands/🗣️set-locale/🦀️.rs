//! 🗣️ 🗣️ Equation play app commands command — `set-locale`.

use crate::op::EquationMutation;
use crate::EquationSnapshot;
use crate::editor::equation::config::{EquationConfig, EquationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, EquationConfig>) -> Result<Emit<EquationMutation, EquationConfigMutation>, Fault> {
    Ok(Emit::config(vec![EquationConfigMutation::SetLocale(crate::editor::equation::config::SetLocale { value: payload.value.clone() })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
