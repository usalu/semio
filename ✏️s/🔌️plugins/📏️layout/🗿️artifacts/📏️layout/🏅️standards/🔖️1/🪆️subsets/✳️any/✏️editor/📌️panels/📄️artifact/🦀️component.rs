//! 📄️ Layout play app panel — the document tree: spreads, pages, frames, parent pages, layers,
//! stories, links and styles of the current layout document.

use crate::artifacts::layout::{Frame, LayoutSnapshot, LAYOUT_DOCUMENT_SCHEMA};
use crate::editor::layout::config::LayoutConfig;
use crate::editor::layout::terminology::LayoutLabels;
use crate::editor::layout::{layout_action, ui_node_list, ui_value_map, ui_value_text, LAYOUT_GRANULARITY_ELEMENT, LAYOUT_INTERACTION_ELEMENTS};
use semio_framework_plugin::{
    tree_item_desc, tree_item_with_action, InteractionTarget, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiText, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, INTERACTION_SELECT_ACTION_ID,
};

//#region 🔖️Constants
pub const LAYOUT_PLAY_BODY_DOCUMENT: &str = "layout.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
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
async fn frame_icon(kind: &str) -> &str {
    match kind {
        "rect" => "square",
        "text" => "type",
        "image" => "image",
        _ => "square",
    }
}

async fn page_row_id(page_id: &str) -> String {
    format!("layout-document.page.{page_id}")
}

async fn layer_row_id(page_id: &str, layer_id: &str) -> String {
    format!("layout-document.layer.{page_id}.{layer_id}")
}

async fn spread_row_id(spread_id: &str) -> String {
    format!("layout-document.spread.{spread_id}")
}

async fn parent_page_row_id(parent_page_id: &str) -> String {
    format!("layout-document.parentPage.{parent_page_id}")
}

async fn story_row_id(story_id: &str) -> String {
    format!("layout-document.story.{story_id}")
}

async fn link_row_id(link_id: &str) -> String {
    format!("layout-document.link.{link_id}")
}

async fn style_row_id(style_id: &str) -> String {
    format!("layout-document.style.{style_id}")
}
//#endregion 🔖️RowIds

//#region 🔖️Render
/// 🌳️ Layout's row shape (id/label/description/icon/optional-action) over the SDK's
/// `tree_item_desc`/`tree_item_with_action` — the icon assignment is the only bit the SDK helpers
/// don't cover, since not every plugin's rows carry one.
fn layout_tree_item(
    id: impl Into<String>,
    label: impl TryInto<Label>,
    description: Option<String>,
    icon_id: Option<String>,
    action: Option<(semio_framework_plugin::ActionId, Option<UiValue>)>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut item = match action {
        Some(action) => tree_item_with_action(id, label, description.clone(), action)?,
        None => tree_item_desc(id, label, description.clone())?,
    };
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        if props.description.is_none() {
            props.description = match description {
                Some(value) => Some(UiText::try_from_string(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "layout row description admission failed"))?),
                None => None,
            };
        }
        props.icon = match icon_id {
            Some(value) => Some(UiText::try_from_string(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "layout row icon admission failed"))?),
            None => None,
        };
    }
    Ok(item)
}

fn selection_args(ids: impl IntoIterator<Item = String>, merge: &str) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut targets = UiFixedList::default();
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

/// 🕹️ `_config` is unused now — page/frame selection moved into the framework-owned "elements"
/// interaction domain; `.interaction_domain(LAYOUT_INTERACTION_ELEMENTS)?` below has the framework's
/// renderer translate row hover into `interactionHover` and stamp presence from `InteractionState`,
/// replacing the deleted `.selected()?`/`.highlighted()?`/`.selection_change()` calls.
pub async fn render(doc: &LayoutSnapshot, _config: &LayoutConfig, labels: &LayoutLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
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
        let mut referencing_ids = UiFixedList::default();
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
            Some(labels.document.into()),
            true,
            crate::editor::layout::ui_node_list([layout_tree_item("layout-document.document.root", Label::data(doc.name.clone()), Some(LAYOUT_DOCUMENT_SCHEMA.into()), Some("file-text".into()), None)])?,
        )?
        .section("layout-document.spreads", Some(labels.spreads.into()), false, spread_items)?
        .section("layout-document.pages", Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, page_items)?
        .section("layout-document.frames", Some(labels.frames.into()), true, frame_items)?
        .section("layout-document.parentPages", Some(labels.parent_pages.into()), false, parent_page_items)?
        .section("layout-document.layers", Some(labels.layers.into()), false, layer_items)?
        .section("layout-document.stories", Some(labels.stories.into()), false, story_items)?
        .section("layout-document.links", Some(labels.links.into()), false, link_items)?
        .section("layout-document.styles", Some(labels.styles.into()), false, style_items)?
        .interaction_domain(LAYOUT_INTERACTION_ELEMENTS)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::layout::testkit::{layout_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn document_lists_sample_pages() {
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("layout-document.page.page-1"));
        assert!(json.contains("Page 1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn document_tree_has_nine_sections() {
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        for section_id in [
            "layout-document.document",
            "layout-document.spreads",
            "layout-document.pages",
            "layout-document.frames",
            "layout-document.parentPages",
            "layout-document.layers",
            "layout-document.stories",
            "layout-document.links",
            "layout-document.styles",
        ] {
            assert!(json.contains(section_id), "missing section {section_id}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn layout_labels_resolve_native_english_by_default() {
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("\"Frames\""));
        assert!(json.contains("\"Layers\""));
        assert!(!json.contains("Rahmen"));
    }

    #[semio_framework_async_macros::async_test]
    async fn layout_labels_translate_document_tree_in_german() {
        use crate::editor::layout::commands::set_locale;
        use crate::editor::layout::testkit::dispatch;
        use crate::editor::layout::LayoutCommand;
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("\"Rahmen\""));
        assert!(json.contains("\"Ebenen\""));
        assert!(!json.contains("\"Frames\""));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
