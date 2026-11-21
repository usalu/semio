//! 🎥️ Process 3d play app commands — the 3D viewport camera (config-only, ephemeral view state).

use semio_s_artifact_process_process3d_config::{Process3dConfig, Process3dConfigMutation, Process3dConfigSetCamera};
use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
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
        pub fov: Option<f64>,
    }

    impl SetCamera {
        /// 🧿️ Reads the renderer's camera record with an optional field of view.
        pub fn from_viewport(camera: &semio_framework_value::DslValue) -> Option<Self> {
            let vector = |key: &str| {
                let [x, y, z] = camera.get(key)?.as_array()? else { return None };
                let point = [x.as_f64()?, y.as_f64()?, z.as_f64()?];
                point.iter().all(|coordinate| coordinate.is_finite()).then_some(point)
            };
            let fov = match camera.get("fov") {
                Some(value) => Some(value.as_f64().filter(|fov| *fov > 0.0 && *fov < 180.0)?),
                None => None,
            };
            Some(Self { position: vector("position")?, target: vector("target")?, fov })
        }
    }

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut semio_s_artifact_process_process3d_kit::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fov = payload.fov.unwrap_or(cfg.snapshot.camera_fov);
        Ok(Emit::config(vec![Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ position: payload.position, target: payload.target, fov })]))
    }
}
//#endregion 🔖️SetCamera
