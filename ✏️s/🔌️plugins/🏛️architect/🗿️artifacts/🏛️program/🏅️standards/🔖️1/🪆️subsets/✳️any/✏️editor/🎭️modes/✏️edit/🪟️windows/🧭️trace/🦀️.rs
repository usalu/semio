//! 🧭️ Architect trace window — the document-wide audit trail.

use crate::standards::v1::subsets::any::schema::inferences::audit_trail;
use crate::ProgramSnapshot;
use crate::editor::architect::ui_label;
use semio_framework_plugin::{tree_item_desc,  LocalizedLabel, PanelTreeBuilder, PluginAssemblyError, SurfaceKind, UiFixedList, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const ARCHITECT_WINDOW_TRACE: &str = "architect-trace";
pub const ARCHITECT_BODY_TRACE: &str = "architect.trace";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: ARCHITECT_WINDOW_TRACE.into(),
        label: LocalizedLabel::native("Trace", "Nachverfolgung"),
        body_key: ARCHITECT_BODY_TRACE.into(),
        surface_kind: SurfaceKind::TextEditor,
        icon_id: "file-code".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: this window degrades to a
        // document-wide audit feed (see `render`'s doc comment) — its rows are informational, not
        // selectable.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactEditor::render` carries no
/// `InteractionView` (matches `note`'s/`gis2d`'s inspection panel precedent), so this window can no
/// longer scope trace chain/impact to a selected entity — both sections needed a root id and are
/// gone with it; the audit trail degrades to the document-wide feed (`audit_trail(program, None)`)
/// instead of one scoped to a selection.
pub fn render(program: &ProgramSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let trail = audit_trail(program, None);
    let mut items = UiFixedList::default();
    for (index, event) in trail.events.iter().take(12).enumerate() {
        let item = tree_item_desc(format!("architect-trace.audit.{index}"), ui_label(format!("{:?} @ {} — {}", event.action, event.timestamp, event.header.name))?, None)?;
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect trace event admission failed"))?;
    }
    if items.is_empty() {
        let item = tree_item_desc("architect-trace.audit.empty", ui_label("(no events)")?, None)?;
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect trace empty row admission failed"))?;
    }
    PanelTreeBuilder::new("architect-trace")?.section("architect-trace.audit", Some(ui_label(format!("Audit Trail ({})", trail.events.len()))?), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
