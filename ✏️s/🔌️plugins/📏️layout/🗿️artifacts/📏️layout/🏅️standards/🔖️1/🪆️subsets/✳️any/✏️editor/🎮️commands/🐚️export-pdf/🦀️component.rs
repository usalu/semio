//! 🐚️ 🐚️ Layout play app commands command — `export-pdf`.

use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::editor::layout::panels::preflight::run_layout_preflight;
use crate::editor::layout::terminology::layout_labels;
use crate::editor::layout::engine::scene::{export_document_pdf, export_document_png_cpu, export_document_svg, export_package_zip};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use base64::Engine;
use semio_framework::kernel::Effect;
use semio_framework_plugin::{engagement_token_matches, ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "export-pdf")]
pub struct ExportPdf {
    pub page_id: Option<String>,
}

pub async fn handle(payload: &ExportPdf, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let page_id = payload.page_id.clone().unwrap_or_else(|| cfg.snapshot.active_page_id.clone());
    match export_document_pdf(doc.snapshot, &page_id) {
        Ok(bytes) => Ok(Emit::effect(Effect::DownloadMediaExport { filename: format!("{page_id}.pdf"), mime_type: "application/pdf".into(), data: base64::engine::general_purpose::STANDARD.encode(bytes), encoding: Some("base64".into()) })),
        Err(_) => Ok(Emit::default()),
    }
}
