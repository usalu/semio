//! 🗂️ Generate-mode window — the generation list.

use crate::editor::flow::config::FlowConfig;
use crate::editor::flow::{flow_action, ui_value_map, ui_value_text};
use crate::playbook::FormGeneration;
use semio_framework_plugin::plugin_app_close_prelude::Label;
use semio_framework_plugin::{
    tree_item_with_action, ActionBinding, Buildable, BuiltNode, HasBase, Locale, LocalizedLabel, PanelTreeBuilder, PluginAssemblyError, RowAction, RowActionPlacement, SurfaceKind, Terminology, Trigger, UiAssemblyResult, UiFixedList, UiText,
    WindowKindDefinition, WindowOptions,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const FLOW_PLAY_WINDOW_GENERATIONS: &str = "flow-generations";
pub const FLOW_PLAY_BODY_GENERATIONS: &str = "flow.play.generations";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
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

/// 🎬️ One remove/rename row action, surfaced in the row's overflow menu (mirrors the retired
/// `UiTreeItemAction { placement: Menu }` pair — `📓️recipe-plugin.md` §2's `TreeItem` row).
fn generation_row_action(icon: &str, label: String, action: &str, args: Option<semio_framework_plugin::UiValue>) -> UiAssemblyResult<RowAction> {
    let (action, args) = flow_action(action, args)?;
    Ok(RowAction { icon: ui_text(icon)?, label: Some(ui_label(label)?), action: ActionBinding { trigger: Trigger::Activate, action, args, capability: None }, placement: RowActionPlacement::Menu })
}

/// 🌳️ One generation row: primary click selects it, the overflow menu carries rename/remove.
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM / `📓️recipe-plugin.md` §6: the old
/// `presence.selected` stamp has no build-time equivalent this wave — dropped rather than approximated,
/// mirroring this crate's `main`/`inspection` windows' identical documented gap.
fn generation_item(generation: &FormGeneration, surface_prefix: &str, locale: Locale, terminology: Terminology) -> UiAssemblyResult<BuiltNode> {
    let remove_args = ui_value_map([("id", ui_value_text(&generation.id)?)])?;
    let rename_args = ui_value_map([("id", ui_value_text(&generation.id)?), ("name", ui_value_text(format!("{} copy", generation.name))?)])?;
    let select_args = ui_value_map([("id", ui_value_text(&generation.id)?)])?;
    let (select_action, select_args) = flow_action("selectGeneration", Some(select_args))?;
    let mut builder = ui::tree_item(ui_label(&generation.name)?).description(ui_text(format!("{} values", generation.values.len()))?).icon(ui_text("layers")?);
    builder = builder.try_id(format!("{surface_prefix}.generation.{}", generation.id)).map_err(|_| generation_error("item-id"))?;
    builder = match select_args {
        Some(args) => builder.try_on_with(Trigger::Activate, select_action, args).map_err(|_| generation_error("item-select"))?,
        None => builder.try_on(Trigger::Activate, select_action).map_err(|_| generation_error("item-select"))?,
    };
    builder = builder.try_row_action(generation_row_action("pencil", generation_tree_label("rename", locale, terminology), "renameGeneration", Some(rename_args))?).map_err(|_| generation_error("item-row-actions"))?;
    builder = builder.try_row_action(generation_row_action("trash-2", generation_tree_label("remove", locale, terminology), "removeGeneration", Some(remove_args))?).map_err(|_| generation_error("item-row-actions"))?;
    builder.try_build().map_err(|_| generation_error("item-build"))
}

pub fn render(config: &FlowConfig, locale: Locale, terminology: Terminology) -> UiAssemblyResult<BuiltNode> {
    let generation = config.generation();
    let surface_prefix = "flow-play-generate";
    let mut items = UiFixedList::default();
    for entry in &generation.generations {
        items.try_push(generation_item(entry, surface_prefix, locale, terminology)?).map_err(|_| generation_error("items"))?;
    }
    let mut builder = PanelTreeBuilder::new(surface_prefix)?.section_or_placeholder(
        format!("{surface_prefix}.generations"),
        Some(ui_label(generation_tree_label("generations", locale, terminology))?),
        true,
        items,
        generation_tree_label("empty", locale, terminology),
    )?;
    let mut add_items = UiFixedList::default();
    add_items
        .try_push(tree_item_with_action(format!("{surface_prefix}.add-generation"), generation_tree_label("add", locale, terminology), None, flow_action("addGeneration", None)?)?)
        .map_err(|_| generation_error("actions"))?;
    builder = builder.section(format!("{surface_prefix}.actions"), Some(ui_label(generation_tree_label("actions", locale, terminology))?), true, add_items)?;
    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{flow_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn the_empty_generation_list_still_offers_the_add_action() {
        let mut app = flow_app().await;
        assert!(render_body(&mut app, FLOW_PLAY_BODY_GENERATIONS).await.contains("addGeneration"));
    }
}
//#endregion 🧪️Tests
