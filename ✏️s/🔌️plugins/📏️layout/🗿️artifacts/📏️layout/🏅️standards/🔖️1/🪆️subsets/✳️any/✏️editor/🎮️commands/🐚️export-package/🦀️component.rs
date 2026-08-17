//! 🐚️ 🐚️ Layout play app commands command — `export-package`.

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
#[dsl(keyword = "export-package")]
pub struct ExportPackage {}

pub fn handle(_payload: &ExportPackage, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let document = doc.snapshot;
    let preflight_json = serde_json::to_string(&run_layout_preflight(document, layout_labels(cfg.snapshot))).unwrap_or_else(|_| "[]".into());
    let doc_json = serde_json::to_string(document).unwrap_or_default();
    match export_package_zip(&doc_json, &preflight_json) {
        Ok(bytes) => Ok(Emit::effect(Effect::DownloadMediaExport {
            filename: format!("{}.layout-package.zip", document.name),
            mime_type: "application/zip".into(),
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            encoding: Some("base64".into()),
        })),
        Err(_) => Ok(Emit::default()),
    }
}
