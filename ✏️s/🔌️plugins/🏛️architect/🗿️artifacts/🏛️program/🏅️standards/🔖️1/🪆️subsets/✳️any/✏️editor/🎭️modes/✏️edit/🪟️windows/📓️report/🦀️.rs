//! 📄️ Architect report window — the last generated `ProgramReport`, rendered as a section tree.

use crate::editor::architect::config::{parse_active_report, ArchitectConfig};
use semio_framework_plugin::{tree_item_desc, ui_text, Label, LocalizedLabel, PanelTreeBuilder, PluginAssemblyError, SurfaceKind, UiFixedList, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub(crate) const ARCHITECT_WINDOW_REPORT: &str = "architect-report";
pub(crate) const ARCHITECT_BODY_REPORT: &str = "architect.report";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub(crate) async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: ARCHITECT_WINDOW_REPORT.into(),
        label: LocalizedLabel::native("Report", "Bericht"),
        body_key: ARCHITECT_BODY_REPORT.into(),
        surface_kind: SurfaceKind::TextEditor,
        icon_id: "file-text".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the report is a read-only
        // rendering of the last generated `ProgramReport` — it has no selectable entities.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub(crate) async fn render(cfg: &ArchitectConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(report) = parse_active_report(cfg) else {
        return ui_text(Label::data("Run validation, analysis, or report to populate this panel."));
    };
    let mut tree = PanelTreeBuilder::new("architect-report")?;
    let mut meta = UiFixedList::default();
    for item in [tree_item_desc("architect-report.kind", Label::data(format!("Kind: {:?}", report.kind)), None)?, tree_item_desc("architect-report.generated", Label::data(format!("Generated: {}", report.generated_at)), None)?] {
        meta.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect report metadata admission failed"))?;
    }
    tree = tree.section("architect-report.meta", Some(Label::data(report.title.clone())), true, meta)?;
    for (index, section) in report.sections.iter()?.enumerate() {
        let mut items = UiFixedList::default();
        if !section.body.is_empty() {
            let item = tree_item_desc(format!("architect-report.section.{index}.body"), Label::data(&section.body), None)?;
            items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect report body admission failed"))?;
        }
        for (bullet_index, bullet) in section.bullets.iter().enumerate() {
            let item = tree_item_desc(format!("architect-report.section.{index}.bullet.{bullet_index}"), Label::data(format!("• {bullet}")), None)?;
            items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect report bullet admission failed"))?;
        }
        tree = tree.section(format!("architect-report.section.{index}"), Some(Label::data(section.heading.clone())), true, items)?;
    }
    tree.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::registers::ReportKind;
    use crate::artifacts::program::sample_plugin;
    use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::build_report;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_text_editor_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, ARCHITECT_BODY_REPORT);
        assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
    }

    #[semio_framework_async_macros::async_test]
    async fn a_report_in_the_config_renders_its_section_headings() {
        let report = build_report(&sample_plugin(), ReportKind::ExecutiveSummary);
        let cfg = ArchitectConfig { active_report_json: serde_json::to_string(&report).expect("json"), ..ArchitectConfig::default() };
        let json = serde_json::to_string(&render(&cfg)).expect("json");
        assert!(json.contains("Overview"));
        assert!(json.contains("architect-report.section"));
    }

    #[semio_framework_async_macros::async_test]
    async fn no_report_renders_the_placeholder() {
        let json = serde_json::to_string(&render(&ArchitectConfig::default())).expect("json");
        assert!(json.contains("Run validation, analysis, or report"));
    }
}
//#endregion 🧪️Tests
