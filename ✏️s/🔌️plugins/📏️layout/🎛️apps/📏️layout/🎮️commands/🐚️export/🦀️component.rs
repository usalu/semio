//! 🐚️ Layout play app commands — shell effects: PNG/SVG/PDF/package export and the engagement-bar
//! submit that routes typed export intents to them. Export round-trips through the host, no operations
//! either way.

use crate::apps::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::apps::layout::panels::preflight::run_layout_preflight;
use crate::apps::layout::terminology::layout_labels;
use crate::apps::layout::engine::scene::{export_document_pdf, export_document_png_cpu, export_document_svg, export_package_zip};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use base64::Engine;
use semio_framework::kernel::HostEffect;
use semio_framework_plugin::{engagement_token_matches, ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️ExportPng
pub mod export_png {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-png")]
    pub struct ExportPng {
        pub page_id: Option<String>,
    }

    pub fn handle(payload: &ExportPng, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
        let page_id = payload.page_id.clone().unwrap_or_else(|| cfg.snapshot.active_page_id.clone());
        match export_document_png_cpu(doc.snapshot, &page_id) {
            Ok(bytes) => Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{page_id}.png"), mime_type: "image/png".into(), data: base64::engine::general_purpose::STANDARD.encode(bytes), encoding: Some("base64".into()) })),
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ExportPng

//#region 🔖️ExportSvg
pub mod export_svg {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-svg")]
    pub struct ExportSvg {
        pub page_id: Option<String>,
    }

    pub fn handle(payload: &ExportSvg, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
        let page_id = payload.page_id.clone().unwrap_or_else(|| cfg.snapshot.active_page_id.clone());
        match export_document_svg(doc.snapshot, &page_id) {
            Ok(svg) => Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{page_id}.svg"), mime_type: "image/svg+xml".into(), data: svg, encoding: None })),
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ExportSvg

//#region 🔖️ExportPdf
pub mod export_pdf {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-pdf")]
    pub struct ExportPdf {
        pub page_id: Option<String>,
    }

    pub fn handle(payload: &ExportPdf, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
        let page_id = payload.page_id.clone().unwrap_or_else(|| cfg.snapshot.active_page_id.clone());
        match export_document_pdf(doc.snapshot, &page_id) {
            Ok(bytes) => Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{page_id}.pdf"), mime_type: "application/pdf".into(), data: base64::engine::general_purpose::STANDARD.encode(bytes), encoding: Some("base64".into()) })),
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ExportPdf

//#region 🔖️ExportPackage
pub mod export_package {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-package")]
    pub struct ExportPackage {}

    pub fn handle(_payload: &ExportPackage, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
        let document = doc.snapshot;
        let preflight_json = serde_json::to_string(&run_layout_preflight(document, layout_labels(cfg.snapshot))).unwrap_or_else(|_| "[]".into());
        let doc_json = serde_json::to_string(document).unwrap_or_default();
        match export_package_zip(&doc_json, &preflight_json) {
            Ok(bytes) => Ok(Emit::effect(HostEffect::DownloadMediaExport {
                filename: format!("{}.layout-package.zip", document.name),
                mime_type: "application/zip".into(),
                data: base64::engine::general_purpose::STANDARD.encode(bytes),
                encoding: Some("base64".into()),
            })),
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ExportPackage

//#region 🔖️EngagementSubmit
pub mod engagement_submit {
    use super::*;

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
}
//#endregion 🔖️EngagementSubmit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::layout::testkit::{dispatch, layout_app};
    use crate::apps::layout::LayoutCommand;

    #[test]
    fn export_actions_wire_to_real_layout_exporters() {
        // 🌉️ SVG/PDF export routes through stdio's real `s.stdio.semio/v1/drawing`→svg bridge
        // (`io_dispatch`, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT
        // W5b) — under `cargo nextest`'s per-test process isolation this registration must happen in
        // THIS test's own process; found missing (pre-existing gap, unrelated to composition work),
        // fixed by mirroring the sibling registration call already used in
        // `⚙️engine/🎬️scene/🦀️component.rs`'s own SVG export test.
        crate::artifacts::layout::io::ensure_stdio_semio_drawing_registered();
        let mut app = layout_app();
        let exports: Vec<(LayoutCommand, &str)> = vec![
            (LayoutCommand::ExportPng(export_png::ExportPng { page_id: Some("page-1".into()) }), "image/png"),
            (LayoutCommand::ExportSvg(export_svg::ExportSvg { page_id: Some("page-1".into()) }), "image/svg+xml"),
            (LayoutCommand::ExportPdf(export_pdf::ExportPdf { page_id: Some("page-1".into()) }), "application/pdf"),
            (LayoutCommand::ExportPackage(export_package::ExportPackage {}), "application/zip"),
        ];
        for (command, mime_type) in exports {
            let result = dispatch(&mut app, command);
            match result.requested_effects.first() {
                Some(HostEffect::DownloadMediaExport { mime_type: mime, data, .. }) => {
                    assert_eq!(mime, mime_type);
                    assert!(!data.is_empty(), "export data");
                }
                other => panic!("expected DownloadMediaExport, got {other:?}"),
            }
        }
    }

    #[test]
    fn engagement_submit_triggers_export() {
        let mut app = layout_app();
        let result = dispatch(&mut app, LayoutCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "export png".into() }));
        assert!(matches!(result.requested_effects.first(), Some(HostEffect::DownloadMediaExport { mime_type, .. }) if mime_type == "image/png"));
    }

    #[test]
    fn engagement_submit_triggers_export_from_normalized_shell_draft() {
        // The React shell PascalCases and strips separators from every draft before submitting it
        // (`normalizeEngagementActionText`), so "export png" arrives as "ExportPng".
        let mut app = layout_app();
        let result = dispatch(&mut app, LayoutCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "ExportPng".into() }));
        assert!(matches!(result.requested_effects.first(), Some(HostEffect::DownloadMediaExport { mime_type, .. }) if mime_type == "image/png"));
    }

    #[test]
    fn registry_backed_engagement_submit_is_shell_effect_not_operation() {
        // 🧬️ engagementSubmit is declared `Shell`: through the real registry the kind-discipline
        // check must accept it because its handler only routes an export `HostEffect`, never operations.
        let mut app = crate::apps::layout::testkit::layout_app_with_registry();
        let result = dispatch(&mut app, LayoutCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "export png".into() }));
        assert!(result.mutations.is_empty(), "Shell action must not emit document operations");
        assert!(matches!(result.requested_effects.first(), Some(HostEffect::DownloadMediaExport { mime_type, .. }) if mime_type == "image/png"));
    }
}
//#endregion 🧪️Tests
