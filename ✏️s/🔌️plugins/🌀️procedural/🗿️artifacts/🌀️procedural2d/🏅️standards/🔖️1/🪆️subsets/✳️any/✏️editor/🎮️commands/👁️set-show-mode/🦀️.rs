//! 👁️ 👁️ Procedural2d play app commands command — `set-show-mode`.

use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-show-mode")]
pub struct SetShowMode {
    pub value: String,
}

pub fn handle(payload: &SetShowMode, _doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Procedural2dConfigMutation::SetShowMode { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, dispatch};
    use crate::editor::procedural2d::Procedural2dCommand;

    #[test]
    fn set_show_mode_is_config_only() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Procedural2dCommand::SetShowMode(SetShowMode { value: "wire".into() }));
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
