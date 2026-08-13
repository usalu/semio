//! 🗣️ 🗣️ Fem2d play app commands command — `set-locale`.

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::op::Fem2dMutation;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️SetLocale
//#endregion 🔖️SetLocale

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    use crate::apps::fem2d::testkit::{dispatch, fem2d_app};
    use crate::apps::fem2d::Fem2dCommand;

    #[test]
    fn set_locale_action_writes_config_not_artifact_mutations() {
        let mut app = fem2d_app();
        let before = app.snapshot().expect("snapshot");
        let result = dispatch(&mut app, Fem2dCommand::SetLocale(SetLocale { value: "de-DE".into() }));
        assert!(result.mutations.is_empty());
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
