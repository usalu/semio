//! 📤️ Process 3d play app commands — shell effects: model export/import round-trips through the host.

use semio_s_artifact_process_process3d_config::{Process3dConfig, Process3dConfigMutation};
use semio_s_artifact_process_process3d_io::{export_process3d_model, import_process3d_model};
use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use semio_framework::kernel::Effect;
use semio_framework::DslValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ExportModel
pub mod export_model {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "export-model")]
    pub struct ExportModel {
        pub format: String,
    }

    pub fn handle(
        payload: &ExportModel,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut semio_s_artifact_process_process3d_kit::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let scene = semio_s_artifact_process_process3d_model::snapshot::process_working_scene_from_snapshot(doc.snapshot);
        match export_process3d_model(&scene, cfg.snapshot.resolved_up_to, &payload.format).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("process3d.media.export"), error))? {
            Some(export) => Ok(Emit::effect(Effect::DownloadMediaExport {
                filename: export.filename,
                mime_type: export.mime_type,
                data: match export.data {
                    DslValue::String(text) => text,
                    other => semio_framework_pack_json::to_json_string(&other),
                },
                encoding: export.encoding,
            })),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ExportModel

//#region 🔖️LoadModelRequest
pub mod load_model_request {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "load-model-request")]
    pub struct LoadModelRequest {}

    pub fn handle(
        _payload: &LoadModelRequest,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut semio_s_artifact_process_process3d_kit::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(111), accept: ".stp,.step,.obj,.stl,.glb".into(), read_as: Some("dataUrl".into()), import_action: "importModelFile".into(), multiple: false, args: None }))
    }
}
//#endregion 🔖️LoadModelRequest

//#region 🔖️ImportModelFile
pub mod import_model_file {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "import-model-file")]
    pub struct ImportModelFile {
        pub name: String,
        pub payload: String,
    }

    /// 📤️ Importing a model file replaces the whole document (stock geometry + a cleared timeline),
    /// which has no in-history mutation (see `📓️taxonomy.md`'s forbidden vocabulary), so this routes
    /// through `editor::process3d::reset_process3d_document_effect` (a `Effect::LoadDocument`).
    pub fn handle(
        payload: &ImportModelFile,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut semio_s_artifact_process_process3d_kit::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        match import_process3d_model(&payload.name.to_ascii_lowercase(), &payload.payload) {
            Some(snapshot) => Ok(Emit { effects: vec![semio_s_artifact_process_process3d_kit::reset_process3d_document_effect(&snapshot)], ..Default::default() }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ImportModelFile
