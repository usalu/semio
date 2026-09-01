//! 🗣️ 🗣️ Fem2d play app commands command — `set-locale`.

use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️SetLocale
//#endregion 🔖️SetLocale

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Fem2dConfigMutation::SetLocale { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem2d::testkit::{dispatch, fem2d_app};
    use crate::editor::fem2d::Fem2dCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_locale_action_writes_config_not_artifact_mutations() {
        let mut app = fem2d_app();
        let before = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot");
        let result = dispatch(&mut app, Fem2dCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
        assert!(result.mutations.is_empty());
        assert_eq!(semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
