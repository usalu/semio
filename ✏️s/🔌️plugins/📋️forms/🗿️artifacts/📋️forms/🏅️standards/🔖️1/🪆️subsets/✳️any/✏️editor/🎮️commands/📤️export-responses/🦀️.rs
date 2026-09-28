//! 📤️ Download immutable submissions as JSON or a normalized CSV answer table.
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-responses")]
pub struct ExportResponses { pub format: String }

pub fn handle(payload: &ExportResponses, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let data = match payload.format.as_str() {
        "json" => crate::schema::response::export::export_responses_json(&doc.snapshot.responses),
        "csv" => crate::schema::response::export::export_responses_csv(&doc.snapshot.responses),
        _ => return Err(Fault::from("forms-export-format-invalid")),
    };
    download(payload, doc.snapshot, data)
}

pub fn download(payload: &ExportResponses, snapshot: &FormsSnapshot, data: String) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let mime = match payload.format.as_str() { "json" => "application/json", "csv" => "text/csv;charset=utf-8", _ => return Err(Fault::from("forms-export-format-invalid")) };
    Ok(Emit::effect(Effect::DownloadMediaExport { filename: format!("{}.responses.{}", snapshot.id, payload.format), mime_type: mime.into(), data, encoding: None }))
}
