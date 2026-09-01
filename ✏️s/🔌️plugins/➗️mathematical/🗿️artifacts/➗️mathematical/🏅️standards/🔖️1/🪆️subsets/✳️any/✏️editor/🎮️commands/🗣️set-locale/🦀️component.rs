//! 🗣️ 🗣️ Mathematical play app commands command — `set-locale`.

use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::MathematicalSnapshot;
use crate::editor::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub async fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
    Ok(Emit::config(vec![MathematicalConfigMutation::SetLocale { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::mathematical::testkit::math_app;
    use crate::editor::mathematical::MathematicalCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_locale_writes_config_not_mutations() {
        let mut app = math_app();
        let result = app.dispatch_typed(MathematicalCommand::SetLocale(SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).expect("locale");
        assert!(result.mutations.is_empty(), "setLocale must not emit a VCS operation");
    }
}
//#endregion 🧪️Tests
