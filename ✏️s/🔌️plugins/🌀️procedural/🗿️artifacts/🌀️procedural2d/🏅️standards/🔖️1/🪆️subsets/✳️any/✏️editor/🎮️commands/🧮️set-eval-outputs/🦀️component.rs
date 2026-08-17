//! 🧮️ 🧮️ Procedural2d play app commands command — `set-eval-outputs`.

use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::schema::host_from_fixture_with_session;
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-eval-outputs")]
pub struct SetEvalOutputs {
    pub outputs_json: String}

pub fn handle(payload: &SetEvalOutputs, _doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    session.set_eval_json(payload.outputs_json.clone());
    Ok(Emit::default())
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, dispatch};
    use crate::editor::procedural2d::Procedural2dCommand;

    #[test]
    fn set_eval_outputs_does_not_mutate_the_document() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Procedural2dCommand::SetEvalOutputs(SetEvalOutputs { outputs_json: "{}".into() }));
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
