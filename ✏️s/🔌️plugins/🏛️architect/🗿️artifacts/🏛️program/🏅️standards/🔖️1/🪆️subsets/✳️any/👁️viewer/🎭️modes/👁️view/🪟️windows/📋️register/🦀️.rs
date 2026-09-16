//! 📋️ Architect viewer — the Register window: a read-only, document-wide register overview (entity
//! counts + draft/approved split per non-empty register). Built from the SAME artifact-level pure
//! `status_summary` inference the sibling editor surface's own document panel uses — this file itself
//! imports nothing from that sibling surface (`policyViewerPurityBreaches` forbids it outright). A
//! viewer has no per-session config (`Config = NoConfig`, contract §2.2), so unlike the editor's own
//! Register window (which reads its exact `ArchitectRegisterWindowConfig::active_register` to show ONE selected register) this
//! window shows every register at once — a genuinely useful, config-free read-only equivalent, not a
//! narrower stand-in for the one it mirrors.

use crate::standards::v1::subsets::any::schema::inferences::status_summary;
use crate::ProgramSnapshot;
use semio_framework_plugin::{tree_item_desc, tree_window_item, Label, LocalizedLabel, PanelTreeBuilder, PluginAssemblyError, SurfaceKind, TreeWindows, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::HasBase;

//#region 🔖️Constants
pub const ARCHITECT_VIEW_WINDOW_REGISTER: &str = "architect-view-register";
pub const ARCHITECT_VIEW_BODY_REGISTER: &str = "architect.view.register";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 👁️ Stitched into the viewer manifest by `crate::viewer::architect::create_architect_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: ARCHITECT_VIEW_WINDOW_REGISTER.into(),
        label: LocalizedLabel::native("Register Overview", "Register-Übersicht"),
        body_key: ARCHITECT_VIEW_BODY_REGISTER.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "list".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 👁️ Read-only overview — no `.window_kind_interactions(..)` reference for this window.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    value.as_ref().try_into().map_err(|_| PluginAssemblyError::new("architect.viewer.label.capacity", "register label admission failed"))
}

/// 👁️ Renders the artifact's non-empty registers and their total, draft and approved counts.
///
/// 🪟️ One windowed `registers` section spans the whole roster and each register is a windowed group
/// row over its three counts, so neither the number of registers nor the rows below one of them is
/// bounded by the fixed-children cap the old section-per-register fan-out was.
pub fn render(program: &ProgramSnapshot, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let summary = status_summary(program);
    if summary.total_entities == 0 {
        return semio_framework_plugin::built_text_node(Label::data("No entities in this program yet.")).map_err(|_| PluginAssemblyError::new("architect.viewer.label.capacity", "register placeholder admission failed"));
    }
    let registers: Vec<_> = summary.by_register.iter().filter(|register| register.count > 0).collect();
    PanelTreeBuilder::new("architect-view-register")?
        .window_section(windows, "architect-view-register.registers", Some(ui_label("Registers")?), true, &registers, |register| {
            let id = format!("architect-view-register.{}", register.register);
            let group = semio_framework_ui_contract::tree_item(ui_label(&register.register)?).try_id(&id).map_err(|_| PluginAssemblyError::new("architect.viewer.items.capacity", "register group admission failed"))?;
            let counts = [("total", "Total", register.count), ("draft", "Draft", register.draft_count), ("approved", "Approved", register.approved_count)];
            tree_window_item(windows, group, &id, true, &counts, |(key, label, count)| tree_item_desc(format!("{id}.{key}"), ui_label(format!("{label}: {count}"))?, None))
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
