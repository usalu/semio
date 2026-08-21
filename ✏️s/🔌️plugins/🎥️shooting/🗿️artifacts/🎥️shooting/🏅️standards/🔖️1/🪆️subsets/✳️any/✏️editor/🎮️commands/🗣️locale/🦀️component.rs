//! 🗣️ Shooting play app command — the host-pushed locale switch. Config-only.

use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub async fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{dispatch, shooting_app};
    use crate::editor::shooting::ShootingCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_locale_switches_the_resolved_label_locale() {
        use crate::editor::shooting::testkit::render;
        use crate::editor::shooting::SHOOTING_PLAY_BODY_DOCUMENT;

        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        assert!(result.mutations.is_empty(), "locale is config-only");
        assert!(render(&mut app, SHOOTING_PLAY_BODY_DOCUMENT).contains("Aufnahmen"), "the document panel now resolves German labels");
    }
}
//#endregion 🧪️Tests
