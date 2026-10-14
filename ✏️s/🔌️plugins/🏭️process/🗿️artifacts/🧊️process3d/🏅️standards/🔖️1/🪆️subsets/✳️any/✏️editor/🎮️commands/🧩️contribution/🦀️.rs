//! 🧩️ Process 3d play app commands — host-pushed plugin contributions (machine catalogs).

use semio_s_artifact_process_process3d_config::{Process3dConfig, Process3dConfigMutation, Process3dConfigSetContributions};
use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetContributions
pub mod set_contributions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "contributions")]
    pub struct SetContributions {
        pub json: String,
    }

    pub fn handle(
        payload: &SetContributions,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut semio_s_artifact_process_process3d_kit::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let installable = semio_s_artifact_process_process3d_kit::installable_contributions(&payload.json, semio_s_artifact_process_process3d_kit::PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES);
        Ok(Emit::config(vec![Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json: installable })]))
    }
}
//#endregion 🔖️SetContributions
