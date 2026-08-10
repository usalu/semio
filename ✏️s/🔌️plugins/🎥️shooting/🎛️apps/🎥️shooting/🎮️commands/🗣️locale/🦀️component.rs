//! 🗣️ Shooting play app command — the host-pushed locale switch. Config-only.

use crate::apps::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{dispatch, shooting_app};
    use crate::apps::shooting::ShootingCommand;

    #[test]
    fn set_locale_switches_the_resolved_label_locale() {
        use crate::apps::shooting::testkit::render;
        use crate::apps::shooting::SHOOTING_PLAY_BODY_DOCUMENT;

        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        assert!(result.mutations.is_empty(), "locale is config-only");
        assert!(render(&mut app, SHOOTING_PLAY_BODY_DOCUMENT).contains("Aufnahmen"), "the document panel now resolves German labels");
    }
}
//#endregion 🧪️Tests
