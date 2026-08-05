//! 📤️ Process 3d play app commands — shell effects: model export/import round-trips through the host.

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigOperation};
use crate::artifacts::process3d::engine::{export_process3d_model, import_process3d_model};
use crate::artifacts::process3d::{op::Process3dOperation, Process3dDocument};
use semio_framework_core::kernel::HostEffect;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
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

    pub fn handle(payload: &ExportModel, doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        match export_process3d_model(doc.projection, &payload.format) {
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

    pub fn handle(_payload: &LoadModelRequest, _doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
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

    pub fn handle(payload: &ImportModelFile, _doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        match import_process3d_model(&payload.name.to_ascii_lowercase(), &payload.payload) {
            Some(document) => Ok(Emit { document_operations: vec![Process3dOperation::SetDocument { document }], config_operations: vec![Process3dConfigOperation::SetSelectedId { value: None }], ..Default::default() }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ImportModelFile
