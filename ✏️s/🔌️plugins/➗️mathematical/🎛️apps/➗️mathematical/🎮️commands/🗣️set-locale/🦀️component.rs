//! 🗣️ 🗣️ Mathematical play app commands command — `set-locale`.

use crate::apps::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::MathematicalSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
    Ok(Emit::config(vec![MathematicalConfigMutation::SetLocale { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
        use crate::apps::mathematical::testkit::math_app;
    use crate::apps::mathematical::MathematicalCommand;

    #[test]
    fn set_locale_writes_config_not_mutations() {
        let mut app = math_app();
        let result = app.dispatch_typed(MathematicalCommand::SetLocale(SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).expect("locale");
        assert!(result.mutations.is_empty(), "setLocale must not emit a VCS operation");
    }
}
//#endregion 🧪️Tests
