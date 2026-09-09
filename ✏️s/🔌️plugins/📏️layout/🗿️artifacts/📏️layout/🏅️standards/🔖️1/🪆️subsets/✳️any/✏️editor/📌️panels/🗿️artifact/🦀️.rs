//! 📄️ Layout play app panel — the document tree: spreads, pages, frames, parent pages, layers,
//! stories, links and styles of the current layout document.

use crate::editor::layout::config::LayoutConfig;
use crate::editor::layout::terminology::LayoutLabels;
use crate::editor::layout::{layout_action, ui_node_list, ui_value_map, ui_value_text, LAYOUT_GRANULARITY_ELEMENT, LAYOUT_INTERACTION_ELEMENTS};
use crate::{Frame, LayoutSnapshot, LAYOUT_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    tree_item_desc, tree_item_with_action, InteractionTarget, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiText, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, INTERACTION_SELECT_ACTION_ID,
};

//#region 🔖️Constants
pub const LAYOUT_PLAY_BODY_DOCUMENT: &str = "layout.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(LAYOUT_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️RowIds
fn frame_icon(kind: &str) -> &str {
    match kind {
        "rect" => "square",
        "text" => "type",
        "image" => "image",
        _ => "square",
    }
}

fn page_row_id(page_id: &str) -> String {
    format!("layout-document.page.{page_id}")
}

fn layer_row_id(page_id: &str, layer_id: &str) -> String {
    format!("layout-document.layer.{page_id}.{layer_id}")
}

fn spread_row_id(spread_id: &str) -> String {
    format!("layout-document.spread.{spread_id}")
}

fn parent_page_row_id(parent_page_id: &str) -> String {
    format!("layout-document.parentPage.{parent_page_id}")
}

fn story_row_id(story_id: &str) -> String {
    format!("layout-document.story.{story_id}")
}

fn link_row_id(link_id: &str) -> String {
    format!("layout-document.link.{link_id}")
}

fn style_row_id(style_id: &str) -> String {
    format!("layout-document.style.{style_id}")
}
//#endregion 🔖️RowIds

//#region 🔖️Render
/// 🌳️ Layout's row shape (id/label/description/icon/optional-action) over the SDK's
/// `tree_item_desc`/`tree_item_with_action` — the icon assignment is the only bit the SDK helpers
/// don't cover, since not every plugin's rows carry one.
fn layout_tree_item(
    id: impl AsRef<str>,
    label: impl Into<Label>,
    description: Option<String>,
    icon_id: Option<String>,
    action: Option<(semio_framework_plugin::ActionId, Option<UiValue>)>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let label = label.into();
    let mut item = match action {
        Some(action) => tree_item_with_action(id, label.as_str(), description.clone(), action)?,
        None => tree_item_desc(id, label.as_str(), description.clone())?,
    };
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        if props.description.is_none() {
            props.description = match description {
                Some(value) => Some(UiText::try_from_string(value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout row description admission failed"))?),
                None => None,
            };
        }
        props.icon = match icon_id {
            Some(value) => Some(UiText::try_from_string(value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout row icon admission failed"))?),
            None => None,
        };
    }
    Ok(item)
}

fn selection_args(ids: impl IntoIterator<Item = String>, merge: &str) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut targets: UiFixedList<InteractionTarget> = UiFixedList::default();
    for id in ids {
        targets.try_push(InteractionTarget { granularity: LAYOUT_GRANULARITY_ELEMENT.into(), id }).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout selection target admission failed"))?;
    }
    let targets = serde_json::to_string(&targets).map_err(|error| PluginAssemblyError::new("ui.action-argument", error.to_string()))?;
    ui_value_map([("domainId", ui_value_text(LAYOUT_INTERACTION_ELEMENTS)?), ("targets", ui_value_text(targets)?), ("merge", ui_value_text(merge)?), ("method", ui_value_text("pick")?)])
}

/// 🕹️ Used to build a `layout_tree_item` that additionally dispatched `setHover`/clear-hover on
/// hover/unhover — deleted along with the framework-owned "elements" domain's presence stamping,
/// which now highlights any `.interaction_domain(LAYOUT_INTERACTION_ELEMENTS)?` row on hover
/// automatically (matching hover-source id against the row's own `id`), no per-row wiring needed
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
///
/// 🕹️ `_config` is unused now — page/frame selection moved into the framework-owned "elements"
/// interaction domain; `.interaction_domain(LAYOUT_INTERACTION_ELEMENTS)?` below has the framework's
/// renderer translate row hover into `interactionHover` and stamp presence from `InteractionState`,
/// replacing the deleted `.selected()?`/`.highlighted()?`/`.selection_change()` calls.
pub fn render(doc: &LayoutSnapshot, _config: &LayoutConfig, labels: &LayoutLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let spread_items = ui_node_list(doc.spreads.iter().map(|spread| layout_tree_item(spread_row_id(&spread.id), Label::data(spread.name.clone()), Some(spread.page_ids.join(", ")), Some("layout".into()), None)))?;

    let page_items = ui_node_list(doc.pages.iter().map(|page| {
        let action = ui_value_map([("pageId", ui_value_text(&page.id)?)])?;
        layout_tree_item(
            page_row_id(&page.id),
            Label::data(page.name.clone()),
            page.parent_page_id.as_ref().map(|parent_id| format!("{}: {parent_id}", labels.parent.as_str())),
            Some("file".into()),
            Some(layout_action("setActivePage", Some(action))?),
        )
    }))?;

    // 🕹️ Row `id` is the BARE frame id (not a `frame_row_id(...)`-prefixed row id) — the framework's
    // `.interaction_domain(LAYOUT_INTERACTION_ELEMENTS)?` presence stamping matches `state.selection`/
    // `.hover` ids against a row's own `id` verbatim, and canvas hit-testing (`DisplayList::hit_test`)
    // resolves those exact bare ids too; a prefixed row id would desync tree/canvas cross-highlighting.
    let mut frame_items = UiFixedList::default();
    for page in &doc.pages {
        for frame in &page.frames {
            let action = layout_action(INTERACTION_SELECT_ACTION_ID, Some(selection_args([frame.id().to_string()], "replace")?))?;
            let item = layout_tree_item(frame.id(), Label::data(frame.id()), Some(format!("{} · {}", page.name, frame.kind_str())), Some(frame_icon(frame.kind_str()).into()), Some(action))?;
            frame_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout frame row admission failed"))?;
        }
    }
    if frame_items.is_empty() {
        let item = layout_tree_item("layout-document.frames.empty", labels.drop_here, None, Some("inbox".into()), None)?;
        frame_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout empty frame row admission failed"))?;
    }

    let parent_page_items =
        ui_node_list(doc.parent_pages.iter().map(|parent| layout_tree_item(parent_page_row_id(&parent.id), Label::data(parent.name.clone()), Some(format!("{}×{}", parent.width as i64, parent.height as i64)), Some("copy".into()), None)))?;

    let layer_items = ui_node_list(doc.pages.iter().flat_map(|page| {
        page.layers
            .iter()
            .map(move |layer| layout_tree_item(layer_row_id(&page.id, &layer.id), Label::data(format!("{} · {}", page.name, layer.name)), Some(format!("{} {}", layer.object_ids.len(), labels.objects.as_str())), Some("layers".into()), None))
    }))?;

    let story_items =
        ui_node_list(doc.stories.iter().map(|story| layout_tree_item(story_row_id(&story.id), Label::data(story.id.clone()), Some(format!("{} {}", story.content.chars().count(), labels.chars.as_str())), Some("file-text".into()), None)))?;

    let mut link_items = UiFixedList::default();
    for link in &doc.links {
        let mut referencing_ids: UiFixedList<String> = UiFixedList::default();
        for frame in doc.pages.iter().flat_map(|page| page.frames.iter()) {
            if let Frame::Image { link_id, .. } = frame {
                if link_id == &link.id {
                    referencing_ids.try_push(frame.id().to_string()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout link reference admission failed"))?;
                }
            }
        }
        let action = if referencing_ids.is_empty() { None } else { Some(layout_action(INTERACTION_SELECT_ACTION_ID, Some(selection_args(referencing_ids, "replace")?))?) };
        let item = layout_tree_item(link_row_id(&link.id), Label::data(link.path.clone()), Some(link.state.clone().unwrap_or_else(|| "ok".into())), Some("link".into()), action)?;
        link_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout link row admission failed"))?;
    }

    let mut style_items =
        ui_node_list(doc.paragraph_styles.iter().map(|style| layout_tree_item(style_row_id(&style.id), Label::data(style.name.clone()), Some(format!("{} · {}pt", style.font_family, style.font_size as i64)), Some("type".into()), None)))?;
    for style in &doc.character_styles {
        let name = style.name.clone().unwrap_or_else(|| style.id.clone());
        let font_family = style.font_family.as_deref().unwrap_or("—");
        let description = match style.font_size {
            Some(size) => format!("{font_family} · {}pt", size as i64),
            None => font_family.to_string(),
        };
        let item = layout_tree_item(style_row_id(&style.id), Label::data(name), Some(description), Some("type".into()), None)?;
        style_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout character style admission failed"))?;
    }

    // 🕹️ `.selected()?`/`.highlighted()?`/`.selection_change()` deleted — the framework stamps this
    // tree's presence from the "elements" `InteractionState` post-render and would overwrite
    // whatever this function stamped anyway (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    PanelTreeBuilder::new("layout-document")?
        .section(
            "layout-document.document",
            Some(crate::editor::layout::ui_label(labels.document.as_str())?),
            true,
            ui_node_list([layout_tree_item("layout-document.document.root", Label::data(doc.name.clone()), Some(LAYOUT_DOCUMENT_SCHEMA.into()), Some("file-text".into()), None)])?,
        )?
        .section("layout-document.spreads", Some(crate::editor::layout::ui_label(labels.spreads.as_str())?), false, spread_items)?
        .section("layout-document.pages", Some(crate::editor::layout::ui_label(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)?), true, page_items)?
        .section("layout-document.frames", Some(crate::editor::layout::ui_label(labels.frames.as_str())?), true, frame_items)?
        .section("layout-document.parentPages", Some(crate::editor::layout::ui_label(labels.parent_pages.as_str())?), false, parent_page_items)?
        .section("layout-document.layers", Some(crate::editor::layout::ui_label(labels.layers.as_str())?), false, layer_items)?
        .section("layout-document.stories", Some(crate::editor::layout::ui_label(labels.stories.as_str())?), false, story_items)?
        .section("layout-document.links", Some(crate::editor::layout::ui_label(labels.links.as_str())?), false, link_items)?
        .section("layout-document.styles", Some(crate::editor::layout::ui_label(labels.styles.as_str())?), false, style_items)?
        .interaction_domain(LAYOUT_INTERACTION_ELEMENTS)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
