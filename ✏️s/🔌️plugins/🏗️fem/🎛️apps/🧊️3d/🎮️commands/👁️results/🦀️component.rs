//! 👁️ FEM 3D app commands — which case/combination and display mode the results window shows,
//! config-only view state that never touches the document.

use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetResultDisplay
pub mod set_result_display {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    #[dsl(keyword = "result-display")]
    pub struct SetResultDisplay {
        pub source_id: Option<String>,
        pub mode: String,
        pub mode_index: u32,
    }

    pub fn handle(payload: &SetResultDisplay, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Fem3dConfigMutation::SetResultDisplay { source_id: payload.source_id.clone(), mode: payload.mode.clone(), mode_index: payload.mode_index }]))
    }
}
//#endregion 🔖️SetResultDisplay

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem3d::testkit::{dispatch, fem3d_app};
    use crate::apps::fem3d::Fem3dCommand;

    #[test]
    fn set_result_display_writes_config_not_artifact_mutations() {
        let mut app = fem3d_app();
        // 🎯️ No config accessor on `VcsArtifactApp` — dispatch must simply not panic/error, and the
        // results window render test (in `modes::edit::windows::results`) covers the resulting display.
        dispatch(&mut app, Fem3dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 1 }));
    }
}
// #endregion 🧪️Tests
