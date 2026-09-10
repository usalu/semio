//! 🎮️ Plugin-scope command facet — the commands that apply whenever ANY procedural app is focused,
//! as opposed to the app- and mode-scoped rosters each surface owns under its own `🎮️commands/`.
//!
//! `PluginManifest::commands` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`) is the field this facet
//! fills, and `Plugin::plugin_command` the only way to fill it — so every entry here carries its own
//! program-level handler and is dispatched through `handle_plugin_command`, never through an app.

use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{DslValue, LocalizedLabel};

/// 🪪️ The command id, addressed as `{owner: Plugin{"procedural"}, command_id: LIST_FLOW_EXTENSIONS}`.
pub(crate) const LIST_FLOW_EXTENSIONS: &str = "listFlowExtensions";

/// 🌊️ Declares the one plugin-scope command procedural publishes: a read-only roster of the flow
/// extensions this plugin installs, which is plugin-scoped precisely because both the 2D and the 3D
/// editor evaluate through the same nine extensions and neither owns them.
pub(crate) fn list_flow_extensions_command() -> CommandDefinition {
    CommandDefinition { in_palette: true, ..CommandDefinition::bounded_catalog(LIST_FLOW_EXTENSIONS, LocalizedLabel::native("List Flow Extensions", "Flow-Erweiterungen auflisten"), "plugin", ActionKind::View) }
}

/// 🧾️ Projects `super::FLOW_EXTENSIONS` as `[{ id, extension, label, version }]`, in declaration
/// order. `ActionKind::View` forbids mutations, so the result carries output only.
pub(crate) fn list_flow_extensions(_invocation: &ManifestCommandInvocation, _meta: &ActionMeta) -> Result<InvocationResult, Fault> {
    let rows = super::FLOW_EXTENSIONS
        .iter()
        .map(|(slug, extension, label, version)| {
            DslValue::Object(vec![
                ("id".to_string(), DslValue::String(super::flow_extension_declaration_id(slug))),
                ("extension".to_string(), DslValue::String((*extension).to_string())),
                ("label".to_string(), DslValue::String((*label).to_string())),
                ("version".to_string(), DslValue::String((*version).to_string())),
            ])
        })
        .collect::<Vec<_>>();
    Ok(InvocationResult {
        output: DslValue::Array(rows),
        mutations: Vec::new(),
        inverse_group: UndoGroup { invocation_id: InvocationId(String::new()), mutations: Vec::new(), inverse_mutations: Vec::new(), member_edits: Vec::new() },
        diagnostics: Vec::new(),
        requested_effects: Vec::new(),
        events: Vec::new(),
        ui_scope: UiDirtyScope::None,
        history_patch: None,
    })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
