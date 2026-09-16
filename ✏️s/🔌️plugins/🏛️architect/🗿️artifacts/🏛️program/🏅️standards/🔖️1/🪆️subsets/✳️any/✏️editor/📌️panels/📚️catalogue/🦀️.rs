//! 📚️ Architect catalogue panel — the action shortcuts and the register index.
//!
//! 🪟️ Both sections are **virtualised**. The 66-entry `REGISTER_IDS` roster is ONE `registers` section
//! spanning the whole index — the `.chunks(UI_FIXED_LIST_ITEMS)`-into-"Registers 1–32"/"33–64"/"65–66"
//! idiom is deleted — and the shortcut roster is windowed the same way, so neither depends on the
//! fixed-children cap.

use crate::editor::architect::catalog::REGISTER_IDS;
use crate::editor::architect::ui_label;
use crate::editor::architect::{architect_action, ui_value_map, ui_value_text};
use semio_framework_plugin::{tree_item_with_action, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, UiAssemblyResult, UiValue, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const ARCHITECT_BODY_CATALOGUE: &str = "architect.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(ARCHITECT_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 📚️ One shortcut's argument value, kept literal so the whole roster is a `const` slice a window can
/// index — the `UiValue`s themselves are arena-credited and built per materialised row only.
enum ShortcutArg {
    Text(&'static str),
    Null,
}

/// 📚️ One catalogue shortcut: the app action a click dispatches, with its own argument map — a
/// catalogue row is never a pick target, so it keeps its binding.
struct Shortcut {
    id: &'static str,
    label: &'static str,
    action: &'static str,
    args: &'static [(&'static str, ShortcutArg)],
}

const SHORTCUTS: &[Shortcut] = &[
    Shortcut { id: "architect-catalogue.add-item", label: "Add Register Item", action: "addRegisterItem", args: &[("registerId", ShortcutArg::Text("elements")), ("template", ShortcutArg::Null)] },
    Shortcut { id: "architect-catalogue.validate", label: "Run Validation", action: "runValidation", args: &[] },
    Shortcut { id: "architect-catalogue.analysis", label: "Run Analysis", action: "runAnalysis", args: &[("analysisKind", ShortcutArg::Text("gap"))] },
    Shortcut { id: "architect-catalogue.report", label: "Run Report", action: "runReport", args: &[("reportKind", ShortcutArg::Text("executiveSummary"))] },
    Shortcut { id: "architect-catalogue.export", label: "Export ProgramSnapshot", action: "exportProgram", args: &[] },
    Shortcut { id: "architect-catalogue.import", label: "Import ProgramSnapshot", action: "importProgramRequest", args: &[] },
    Shortcut { id: "architect-catalogue.export-csv", label: "Export Registers CSV", action: "exportRegistersCsv", args: &[] },
    Shortcut { id: "architect-catalogue.import-csv", label: "Import Registers CSV", action: "importRegistersCsv", args: &[("csv", ShortcutArg::Text("")), ("strategy", ShortcutArg::Text("upsert"))] },
    Shortcut { id: "architect-catalogue.apply-template", label: "Apply Template", action: "applyTemplate", args: &[("templateId", ShortcutArg::Text(""))] },
    Shortcut { id: "architect-catalogue.search", label: "Search ProgramSnapshot", action: "search", args: &[("query", ShortcutArg::Text(""))] },
];

fn shortcut_row(shortcut: &Shortcut) -> UiAssemblyResult<BuiltNode> {
    let args = if shortcut.args.is_empty() {
        None
    } else {
        let mut entries = Vec::with_capacity(shortcut.args.len());
        for (key, value) in shortcut.args {
            entries.push((*key, match value {
                ShortcutArg::Text(text) => ui_value_text(text)?,
                ShortcutArg::Null => UiValue::Null,
            }));
        }
        Some(ui_value_map(entries)?)
    };
    tree_item_with_action(shortcut.id, ui_label(shortcut.label)?, None, architect_action(shortcut.action, args)?)
}

pub fn render(windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    PanelTreeBuilder::new("architect-catalogue")?
        .window_section(windows, "architect-catalogue.actions", Some(ui_label(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)?), true, SHORTCUTS, shortcut_row)?
        .window_section(windows, "architect-catalogue.registers", Some(ui_label("Registers")?), true, REGISTER_IDS, |register| {
            let args = ui_value_map([("registerId", ui_value_text(register)?)])?;
            tree_item_with_action(format!("architect-catalogue.register.{register}"), ui_label(*register)?, None, architect_action("selectRegister", Some(args))?)
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
