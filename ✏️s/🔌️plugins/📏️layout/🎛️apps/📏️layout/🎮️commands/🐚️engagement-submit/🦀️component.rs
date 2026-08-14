//! 🐚️ 🐚️ Layout play app commands command — `engagement-submit`.

use crate::apps::layout::commands::{export_package, export_pdf, export_png, export_svg};
use crate::apps::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use semio_framework_plugin::{engagement_token_matches, ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: String,
}

/// 🐚️ Routes typed export intents from the engagement bar to the matching export handler.
pub fn handle(payload: &EngagementSubmit, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let typed = payload.value.trim();
    if engagement_token_matches(typed, "export png") || engagement_token_matches(typed, "png") {
        return export_png::handle(&export_png::ExportPng { page_id: None }, doc, cfg);
    }
    if engagement_token_matches(typed, "export svg") || engagement_token_matches(typed, "svg") {
        return export_svg::handle(&export_svg::ExportSvg { page_id: None }, doc, cfg);
    }
    if engagement_token_matches(typed, "export pdf") || engagement_token_matches(typed, "pdf") {
        return export_pdf::handle(&export_pdf::ExportPdf { page_id: None }, doc, cfg);
    }
    if engagement_token_matches(typed, "export package") || engagement_token_matches(typed, "package") {
        return export_package::handle(&export_package::ExportPackage {}, doc, cfg);
    }
    Ok(Emit::default())
}
