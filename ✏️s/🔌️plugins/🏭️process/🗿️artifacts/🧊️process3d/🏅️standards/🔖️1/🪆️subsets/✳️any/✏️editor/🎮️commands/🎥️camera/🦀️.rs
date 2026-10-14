//! 🎥️ Process 3d play app commands — the 3D viewport camera (config-only, ephemeral view state).

use semio_s_artifact_process_process3d_config::{Process3dConfig, Process3dConfigMutation, Process3dConfigSetCamera};
use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(coord)]
        pub position: [f64; 3],
        #[dsl(coord)]
        pub target: [f64; 3],
        pub fov: f64,
    }

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut semio_s_artifact_process_process3d_kit::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ position: payload.position, target: payload.target, fov: payload.fov })]))
    }
}
//#endregion 🔖️SetCamera
