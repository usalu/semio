//! 🪵️ Process 3d play app commands — swap the stock kind (resets the process timeline).

use semio_s_artifact_process_process3d_config::{Process3dConfig, Process3dConfigMutation};
use semio_s_artifact_process_process3d_kit::editor::process3d::terminology::process3d_labels;
use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_s_artifact_process_process3d_model::{snapshot::process_working_scene_to_snapshot, snapshot::Pose, snapshot::Process3dSnapshot, snapshot::ProcessWorkingScene, snapshot::Stock, snapshot::WorkingSolid};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetStock
pub mod set_stock {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "stock")]
    pub struct SetStock {
        pub kind: String,
    }

    /// 🪵️ Swapping the stock kind resets the whole process timeline (steps + cursor), which is a
    /// whole-document replace, not a targeted edit — no in-history mutation exists for that (see
    /// `📓️taxonomy.md`'s forbidden vocabulary), so this routes through
    /// `editor::process3d::reset_process3d_document_effect` (a `Effect::LoadDocument`) instead.
    pub fn handle(payload: &SetStock, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, ctx: &mut semio_s_artifact_process_process3d_kit::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let solid = match payload.kind.as_str() {
            "cylinder" => WorkingSolid::Cylinder { radius: 0.3, height: 1.0 },
            "sphere" => WorkingSolid::Sphere { radius: 0.5 },
            _ => WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 },
        };
        let stock = Stock { id: fixture.stock_id.clone(), label: process3d_labels(ctx.view_state()?).stock.into(), solid, pose: Pose::default() };
        let scene = ProcessWorkingScene { stock, steps: Vec::new() };
        let snapshot = process_working_scene_to_snapshot(&scene, fixture.workshop.clone());
        Ok(Emit { effects: vec![semio_s_artifact_process_process3d_kit::reset_process3d_document_effect(&snapshot)], ..Default::default() })
    }
}
//#endregion 🔖️SetStock
