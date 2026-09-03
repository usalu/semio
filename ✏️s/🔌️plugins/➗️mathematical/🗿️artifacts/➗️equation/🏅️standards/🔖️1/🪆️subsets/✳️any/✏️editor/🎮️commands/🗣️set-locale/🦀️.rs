//! 🗣️ 🗣️ Equation play app commands command — `set-locale`.

use crate::artifacts::equation::op::EquationMutation;
use crate::artifacts::equation::EquationSnapshot;
use crate::editor::equation::config::{EquationConfig, EquationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub async fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, EquationConfig>) -> Result<Emit<EquationMutation, EquationConfigMutation>, Fault> {
    Ok(Emit::config(vec![EquationConfigMutation::SetLocale { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::equation::testkit::math_app;
    use crate::editor::equation::EquationCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_locale_writes_config_not_mutations() {
        let mut app = math_app();
        let result = app.dispatch_typed(EquationCommand::SetLocale(SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).expect("locale");
        assert!(result.mutations.is_empty(), "setLocale must not emit a VCS operation");
    }
}
//#endregion 🧪️Tests
