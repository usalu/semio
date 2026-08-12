//! 📤️ Process 3d play app commands — shell effects: model export/import round-trips through the host.

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::engine::{export_process3d_model, import_process3d_model};
use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot};
use semio_framework::kernel::HostEffect;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️ExportModel
pub mod export_model {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-model")]
    pub struct ExportModel {
        pub format: String,
    }

    pub fn handle(payload: &ExportModel, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        match export_process3d_model(doc.snapshot, &payload.format) {
            Some(export) => Ok(Emit::effect(HostEffect::DownloadMediaExport {
                filename: export.filename,
                mime_type: export.mime_type,
                data: match export.data {
                    Value::String(text) => text,
                    other => serde_json::to_string(&other).unwrap_or_default(),
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "load-model-request")]
    pub struct LoadModelRequest {}

    pub fn handle(_payload: &LoadModelRequest, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit::effect(HostEffect::RequestFileOpen { accept: ".stp,.step,.obj,.stl,.glb".into(), read_as: Some("dataUrl".into()), import_action: "importModelFile".into(), multiple: false }))
    }
}
//#endregion 🔖️LoadModelRequest

//#region 🔖️ImportModelFile
pub mod import_model_file {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-model-file")]
    pub struct ImportModelFile {
        pub name: String,
        pub payload: String,
    }

    /// 📤️ Importing a model file replaces the whole document (stock geometry + a cleared timeline),
    /// which has no in-history mutation (see `📓️taxonomy.md`'s forbidden vocabulary), so this routes
    /// through `apps::process3d::reset_process3d_document_effect` (a `HostEffect::LoadDocument`).
    pub fn handle(payload: &ImportModelFile, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        match import_process3d_model(&payload.name.to_ascii_lowercase(), &payload.payload) {
            Some(snapshot) => Ok(Emit {
                effects: vec![crate::apps::process3d::reset_process3d_document_effect(&snapshot)],
                config_mutations: vec![Process3dConfigMutation::SetSelectedId { value: None }],
                ..Default::default()
            }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ImportModelFile
