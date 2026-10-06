//! 🗂️ Generate-mode window — the generation list.

use crate::editor::flow::modes::edit::windows::main::transient::FlowWindowTransient;
use crate::editor::flow::{flow_action, ui_value_map, ui_value_text};
use crate::playbook::FormGeneration;
use semio_framework_ui_contract::Label;
use semio_framework_plugin::activation_target;
use semio_framework_plugin::row_action;
use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::Buildable;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::HasBase;
use semio_framework_plugin::HasChildren;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::RowActionPlacement;
use semio_framework_plugin::SurfaceKind;
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::Trigger;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiFixedList;
use semio_framework_plugin::UiText;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const FLOW_PLAY_WINDOW_GENERATIONS: &str = "flow-generations";
pub const FLOW_PLAY_BODY_GENERATIONS: &str = "flow.play.generations";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: FLOW_PLAY_WINDOW_GENERATIONS.into(),
        label: LocalizedLabel::native("Generations", "Generationen"),
        body_key: FLOW_PLAY_BODY_GENERATIONS.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "sparkles".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn generation_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.generations", format!("fixed UI admission failed at {stage}"))
}

fn ui_label(value: impl AsRef<str>) -> UiAssemblyResult<Label> {
    Label::try_from(value.as_ref().to_string()).map_err(|error| PluginAssemblyError::new("ui.generations", error))
}

fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| generation_error("text"))
}

/// 🗣️ Chrome labels for the generations tree — localized at the call site via [`Locale`]/[`Terminology`].
fn generation_tree_label(key: &str, locale: Locale, terminology: Terminology) -> String {
    let localized = LocalizedLabel::from_fn(|_terminology, locale| match (key, locale) {
        ("remove", Locale::De) => "Entfernen".into(),
        ("remove", _) => "Remove".into(),
        ("rename", Locale::De) => "Umbenennen".into(),
        ("rename", _) => "Rename".into(),
        ("generations", Locale::De) => "Generierungen".into(),
        ("generations", _) => "Generations".into(),
        ("add", Locale::De) => "Generierung hinzufügen".into(),
        ("add", _) => "Add Generation".into(),
        ("empty", Locale::De) => "(keine Generierungen)".into(),
        ("empty", _) => "(no generations)".into(),
        ("actions", Locale::De) => "Aktionen".into(),
        ("actions", _) => "Actions".into(),
        _ => key.into(),
    });
    localized.resolve(terminology, locale).to_string()
}

/// ✏️ The selected generation's inline name editor: `renameGeneration{id}` committed on blur with the typed name — the one
/// reachable rename, as procedural's twin `generation_rename_field` (`🌀️procedural/🫀️core/🖼️semantic-ui`) renders it. The
/// former row action renamed to a HARDCODED `"{name} copy"`, so a user could reach the verb but never choose a name.
fn generation_rename_field(surface_prefix: &str, generation: &FormGeneration, placeholder: &str) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = flow_action("renameGeneration", Some(ui_value_map([("id", ui_value_text(&generation.id)?)])?))?;
    let editor = ui::input(ui::InputKind::Text).value(ui_text(&generation.name)?).placeholder(ui_label(placeholder)?).commit(ui_text("blur")?);
    let editor = editor.try_id(format!("{surface_prefix}.generation.{}.rename", generation.id)).map_err(|_| generation_error("rename-id"))?;
    let editor = match args {
        Some(args) => editor.try_on_with(Trigger::Commit, action, args).map_err(|_| generation_error("rename-binding"))?,
        None => editor.try_on(Trigger::Commit, action).map_err(|_| generation_error("rename-binding"))?,
    };
    editor.try_build().map_err(|_| generation_error("rename-build"))
}

/// 🌳️ One generation row: its ONE target `{id}` — activation `selectGeneration`, the overflow menu's `removeGeneration` —
/// and, while selected, the inline name editor.
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM / `📓️recipe-plugin.md` §6: the old
/// `presence.selected` stamp has no build-time equivalent this wave — dropped rather than approximated,
/// mirroring this crate's `main`/`inspection` windows' identical documented gap.
fn generation_item(generation: &FormGeneration, selected: bool, surface_prefix: &str, locale: Locale, terminology: Terminology) -> UiAssemblyResult<BuiltNode> {
    let select = flow_action("selectGeneration", Some(ui_value_map([("id", ui_value_text(&generation.id)?)])?))?;
    let mut builder = ui::tree_item(ui_label(&generation.name)?).description(ui_text(format!("{} values", generation.values.len()))?).icon(ui_text("layers")?);
    builder = builder.try_id(format!("{surface_prefix}.generation.{}", generation.id)).map_err(|_| generation_error("item-id"))?.target(activation_target(select)?);
    builder = builder.try_row_action(row_action("trash-2", &generation_tree_label("remove", locale, terminology), "removeGeneration", RowActionPlacement::Menu)?).map_err(|_| generation_error("item-row-actions"))?;
    if selected {
        builder = builder.try_child(generation_rename_field(surface_prefix, generation, &generation_tree_label("rename", locale, terminology))?).map_err(|_| generation_error("item-rename"))?;
    }
    builder.try_build().map_err(|_| generation_error("item-build"))
}

pub fn render(transient: &FlowWindowTransient, locale: Locale, terminology: Terminology, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut generation_owner = transient.generation().map_err(|error| PluginAssemblyError::new("ui.generations", error.into_message()))?;
    let generation = generation_owner.as_mut();
    let surface_prefix = "flow-play-generate";
    let mut builder = PanelTreeBuilder::new(surface_prefix)?.window_section_or_placeholder(
        windows,
        &format!("{surface_prefix}.generations"),
        Some(ui_label(generation_tree_label("generations", locale, terminology))?),
        true,
        &generation.generations,
        |entry| generation_item(entry, crate::playbook::selected_generation(&generation).is_some_and(|selected| selected.id == entry.id), surface_prefix, locale, terminology),
        generation_tree_label("empty", locale, terminology),
    )?;
    let mut add_items = UiFixedList::default();
    add_items.try_push(tree_item_with_action(format!("{surface_prefix}.add-generation"), generation_tree_label("add", locale, terminology), None, flow_action("addGeneration", None)?)?).map_err(|_| generation_error("actions"))?;
    builder = builder.section(format!("{surface_prefix}.actions"), Some(ui_label(generation_tree_label("actions", locale, terminology))?), true, add_items)?;
    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
