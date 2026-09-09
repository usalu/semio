//! 📄️ Architect report window — the last generated `ProgramReport`, rendered as a section tree.

use crate::editor::architect::config::{parse_active_report, ArchitectConfig};
use crate::editor::architect::ui_label;
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelTreeBuilder, PluginAssemblyError, SurfaceKind, UiFixedList, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub(crate) const ARCHITECT_WINDOW_REPORT: &str = "architect-report";
pub(crate) const ARCHITECT_BODY_REPORT: &str = "architect.report";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub(crate) fn definition() -> WindowKindDefinition {
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
pub(crate) fn render(cfg: &ArchitectConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(report) = parse_active_report(cfg) else {
        return crate::editor::architect::ui_node(semio_framework_ui_contract::text(ui_label("Run validation, analysis, or report to populate this panel.")?), "architect-report.empty");
    };
    let mut tree = PanelTreeBuilder::new("architect-report")?;
    let mut meta = UiFixedList::default();
    for item in [tree_item_desc("architect-report.kind", ui_label(format!("Kind: {:?}", report.kind))?, None)?, tree_item_desc("architect-report.generated", ui_label(format!("Generated: {}", report.generated_at))?, None)?] {
        meta.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect report metadata admission failed"))?;
    }
    tree = tree.section("architect-report.meta", Some(ui_label(report.title.clone())?), true, meta)?;
    for (index, section) in report.sections.iter().enumerate() {
        let mut items = UiFixedList::default();
        if !section.body.is_empty() {
            let item = tree_item_desc(format!("architect-report.section.{index}.body"), ui_label(&section.body)?, None)?;
            items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect report body admission failed"))?;
        }
        for (bullet_index, bullet) in section.bullets.iter().enumerate() {
            let item = tree_item_desc(format!("architect-report.section.{index}.bullet.{bullet_index}"), ui_label(format!("• {bullet}"))?, None)?;
            items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect report bullet admission failed"))?;
        }
        tree = tree.section(format!("architect-report.section.{index}"), Some(ui_label(section.heading.clone())?), true, items)?;
    }
    tree.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
