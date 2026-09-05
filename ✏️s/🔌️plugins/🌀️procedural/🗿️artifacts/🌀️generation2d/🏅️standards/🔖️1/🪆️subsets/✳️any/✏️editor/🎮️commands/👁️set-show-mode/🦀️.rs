//! 👁️ 👁️ Generation2d play app commands command — `set-show-mode`.

use crate::artifacts::generation2d::op::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-show-mode")]
pub struct SetShowMode {
    pub value: String,
}

pub fn handle(payload: &SetShowMode, _doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Generation2dConfigMutation::SetShowMode { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation2d::testkit::{app, dispatch};
    use crate::editor::generation2d::Generation2dCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_show_mode_is_config_only() {
        let mut app = app().await;
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Generation2dCommand::SetShowMode(SetShowMode { value: "wire".into() })).await;
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
