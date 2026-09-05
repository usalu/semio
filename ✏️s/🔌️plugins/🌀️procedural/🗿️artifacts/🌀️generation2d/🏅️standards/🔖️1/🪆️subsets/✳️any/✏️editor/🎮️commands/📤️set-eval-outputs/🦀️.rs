//! 🧮️ 🧮️ Generation2d play app commands command — `set-eval-outputs`.

use crate::artifacts::generation2d::op::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-eval-outputs")]
pub struct SetEvalOutputs {
    pub outputs_json: String,
}

pub fn handle(payload: &SetEvalOutputs, _doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    session.set_eval_json(payload.outputs_json.clone());
    Ok(Emit::default())
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation2d::testkit::{app, dispatch};
    use crate::editor::generation2d::Generation2dCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_eval_outputs_does_not_mutate_the_document() {
        let mut app = app().await;
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Generation2dCommand::SetEvalOutputs(SetEvalOutputs { outputs_json: "{}".into() })).await;
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
