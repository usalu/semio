//! 📄️ Architect report window — one selected authored `ReportRecord`, rendered from the document.

use crate::editor::architect::ui_label;
use crate::ProgramSnapshot;
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelTreeBuilder, PluginAssemblyError, SurfaceKind, UiFixedList, ViewModel, WindowKindDefinition, WindowOptions};

#[path = "🎚️config/🦀️.rs"]
pub mod config;

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
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: authored reports are
        // read-only document records in this surface, so it has no selectable entities.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn localized(view: &ViewModel, en: impl AsRef<str>, de: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    let label = LocalizedLabel::native(en.as_ref(), de.as_ref());
    ui_label(label.resolve(view.terminology, view.locale))
}

pub(crate) fn render(program: &ProgramSnapshot, cfg: &config::ArchitectReportWindowConfig, view: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(selected_report_id) = cfg.selected_report_id.as_ref() else {
        return crate::editor::architect::ui_node(semio_framework_ui_contract::text(localized(view, "Generate a report in this window.", "Erstellen Sie in diesem Fenster einen Bericht.")?), "architect-report.empty");
    };
    let Some(report) = program.reports.iter().find(|report| report.header.id == *selected_report_id) else {
        return crate::editor::architect::ui_node(
            semio_framework_ui_contract::text(localized(
                view,
                format!("The selected report '{selected_report_id}' is unavailable."),
                format!("Der ausgewählte Bericht '{selected_report_id}' ist nicht verfügbar."),
            )?),
            "architect-report.missing",
        );
    };
    let mut tree = PanelTreeBuilder::new("architect-report")?;
    let mut meta = UiFixedList::default();
    let generated_en = report.generated_at.as_deref().unwrap_or("unknown");
    let generated_de = report.generated_at.as_deref().unwrap_or("unbekannt");
    for item in [
        tree_item_desc("architect-report.kind", localized(view, format!("Type: {:?}", report.kind), format!("Art: {:?}", report.kind))?, None)?,
        tree_item_desc("architect-report.generated", localized(view, format!("Created: {generated_en}"), format!("Erstellt: {generated_de}"))?, None)?,
        tree_item_desc("architect-report.version", localized(view, format!("Version: {}", report.version), format!("Version: {}", report.version))?, None)?,
    ] {
        meta.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect report metadata admission failed"))?;
    }
    tree = tree.section("architect-report.meta", Some(ui_label(report.title.clone())?), true, meta)?;
    for (index, section) in report.sections.iter().enumerate() {
        tree = tree.section(format!("architect-report.section.{index}"), Some(ui_label(section)?), true, UiFixedList::default())?;
    }
    tree.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
