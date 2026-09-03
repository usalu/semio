//! 🗣️ 🗣️ Generation3d play app commands command — `set-locale`.

use crate::artifacts::generation3d::op::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Generation3dConfigMutation::SetLocale { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation3d::testkit::{app, dispatch};
    use crate::editor::generation3d::Generation3dCommand;

    #[test]
    fn set_locale_updates_config_locale() {
        let _serial = crate::editor::generation3d::test_support::lock();
        let mut app = app();
        dispatch(&mut app, Generation3dCommand::SetLocale(SetLocale { value: "de-DE".into() }));
    }
}
//#endregion 🧪️Tests
