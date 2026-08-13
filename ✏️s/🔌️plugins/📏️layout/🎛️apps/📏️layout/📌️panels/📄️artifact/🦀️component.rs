//! 📄️ Layout play app panel — the document tree: spreads, pages, frames, parent pages, layers,
//! stories, links and styles of the current layout document.

use crate::apps::layout::config::LayoutConfig;
use crate::apps::layout::layout_action;
use crate::apps::layout::terminology::LayoutLabels;
use crate::artifacts::layout::{Frame, LayoutSnapshot, LAYOUT_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    tree_item_desc, tree_item_with_action, ActionDescriptor, IconName, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use serde_json::json;

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

fn frame_row_id(frame_id: &str) -> String {
    format!("layout-document.frame.{frame_id}")
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
fn layout_tree_item(id: impl Into<String>, label: impl Into<Label>, description: Option<String>, icon_id: Option<String>, action: Option<ActionDescriptor>) -> UiTreeItemNode {
    let mut item = match action {
        Some(action) => tree_item_with_action(id, label, description, action),
        None => tree_item_desc(id, label, description),
    };
    item.icon_id = icon_id.and_then(|id| IconName::from_str(&id));
    item
}

/// 🌳️ A `layout_tree_item` that additionally dispatches `setHover`/clear-hover on hover/unhover — used
/// by the document tree's page and frame rows to drive canvas hover highlighting.
fn layout_tree_item_hoverable(id: impl Into<String>, label: impl Into<Label>, description: Option<String>, icon_id: Option<String>, action: Option<ActionDescriptor>, hover_id: &str) -> UiTreeItemNode {
    let mut item = layout_tree_item(id, label, description, icon_id, action);
    item.hover_action = Some(layout_action("setHover", Some(json!({ "id": hover_id }))));
    item.unhover_action = Some(layout_action("setHover", Some(json!({ "id": serde_json::Value::Null }))));
    item
}

pub fn render(doc: &LayoutSnapshot, config: &LayoutConfig, labels: &LayoutLabels) -> UiNode {
    let spread_items: Vec<UiTreeItemNode> = doc.spreads.iter().map(|spread| layout_tree_item(spread_row_id(&spread.id), Label::data(spread.name.clone()), Some(spread.page_ids.join(", ")), Some("layout".into()), None)).collect();

    let page_items: Vec<UiTreeItemNode> = doc
        .pages
        .iter()
        .map(|page| {
            layout_tree_item_hoverable(
                page_row_id(&page.id),
                Label::data(page.name.clone()),
                page.parent_page_id.as_ref().map(|parent_id| format!("{}: {parent_id}", labels.parent.as_str())),
                Some("file".into()),
                Some(layout_action("setActivePage", Some(json!({ "pageId": page.id })))),
                &page.id,
            )
        })
        .collect();

    let frame_items: Vec<UiTreeItemNode> = doc
        .pages
        .iter()
        .flat_map(|page| {
            page.frames.iter().map(move |frame| {
                layout_tree_item_hoverable(
                    frame_row_id(frame.id()),
                    Label::data(frame.id()),
                    Some(format!("{} · {}", page.name, frame.kind_str())),
                    Some(frame_icon(frame.kind_str()).into()),
                    Some(layout_action("setSelection", Some(json!({ "ids": [frame.id()] })))),
                    frame.id(),
                )
            })
        })
        .collect();
    let frame_items = if frame_items.is_empty() { vec![layout_tree_item("layout-document.frames.empty", labels.drop_here, None, Some("inbox".into()), None)] } else { frame_items };

    let parent_page_items: Vec<UiTreeItemNode> =
        doc.parent_pages.iter().map(|parent| layout_tree_item(parent_page_row_id(&parent.id), Label::data(parent.name.clone()), Some(format!("{}×{}", parent.width as i64, parent.height as i64)), Some("copy".into()), None)).collect();

    let layer_items: Vec<UiTreeItemNode> = doc
        .pages
        .iter()
        .flat_map(|page| {
            page.layers
                .iter()
                .map(move |layer| layout_tree_item(layer_row_id(&page.id, &layer.id), Label::data(format!("{} · {}", page.name, layer.name)), Some(format!("{} {}", layer.object_ids.len(), labels.objects.as_str())), Some("layers".into()), None))
        })
        .collect();

    let story_items: Vec<UiTreeItemNode> =
        doc.stories.iter().map(|story| layout_tree_item(story_row_id(&story.id), Label::data(story.id.clone()), Some(format!("{} {}", story.content.chars().count(), labels.chars.as_str())), Some("file-text".into()), None)).collect();

    let link_items: Vec<UiTreeItemNode> = doc
        .links
        .iter()
        .map(|link| {
            let referencing_ids: Vec<String> = doc
                .pages
                .iter()
                .flat_map(|page| page.frames.iter())
                .filter_map(|frame| match frame {
                    Frame::Image { link_id, .. } if link_id == &link.id => Some(frame.id().to_string()),
                    _ => None,
                })
                .collect();
            layout_tree_item(
                link_row_id(&link.id),
                Label::data(link.path.clone()),
                Some(link.state.clone().unwrap_or_else(|| "ok".into())),
                Some("link".into()),
                (!referencing_ids.is_empty()).then(|| layout_action("setSelection", Some(json!({ "ids": referencing_ids })))),
            )
        })
        .collect();

    let mut style_items: Vec<UiTreeItemNode> =
        doc.paragraph_styles.iter().map(|style| layout_tree_item(style_row_id(&style.id), Label::data(style.name.clone()), Some(format!("{} · {}pt", style.font_family, style.font_size as i64)), Some("type".into()), None)).collect();
    style_items.extend(doc.character_styles.iter().map(|style| {
        let name = style.name.clone().unwrap_or_else(|| style.id.clone());
        let font_family = style.font_family.as_deref().unwrap_or("—");
        let description = match style.font_size {
            Some(size) => format!("{font_family} · {}pt", size as i64),
            None => font_family.to_string(),
        };
        layout_tree_item(style_row_id(&style.id), Label::data(name), Some(description), Some("type".into()), None)
    }));

    let highlighted_ids: Vec<String> = config.hovered_id.as_ref().map(|id| vec![page_row_id(id), frame_row_id(id)]).unwrap_or_default();
    let mut builder = PanelTreeBuilder::new("layout-document")
        .section("layout-document.document", Some(labels.document.into()), true, vec![layout_tree_item("layout-document.document.root", Label::data(doc.name.clone()), Some(LAYOUT_DOCUMENT_SCHEMA.into()), Some("file-text".into()), None)])
        .section("layout-document.spreads", Some(labels.spreads.into()), false, spread_items)
        .section("layout-document.pages", Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, page_items)
        .section("layout-document.frames", Some(labels.frames.into()), true, frame_items)
        .section("layout-document.parentPages", Some(labels.parent_pages.into()), false, parent_page_items)
        .section("layout-document.layers", Some(labels.layers.into()), false, layer_items)
        .section("layout-document.stories", Some(labels.stories.into()), false, story_items)
        .section("layout-document.links", Some(labels.links.into()), false, link_items)
        .section("layout-document.styles", Some(labels.styles.into()), false, style_items)
        .selected(config.selected_ids.iter().flat_map(|id| vec![page_row_id(id), frame_row_id(id), layer_row_id(&config.active_page_id, id)]).collect())
        .selection_change(layout_action("setSelection", None));
    if !highlighted_ids.is_empty() {
        builder = builder.highlighted(highlighted_ids);
    }
    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::layout::testkit::{layout_app, render as render_body};

    #[test]
    fn document_lists_sample_pages() {
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("layout-document.page.page-1"));
        assert!(json.contains("Page 1"));
    }

    #[test]
    fn document_tree_has_nine_sections() {
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

    #[test]
    fn layout_labels_resolve_native_english_by_default() {
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("\"Frames\""));
        assert!(json.contains("\"Layers\""));
        assert!(!json.contains("Rahmen"));
    }

    #[test]
    fn layout_labels_translate_document_tree_in_german() {
        use crate::apps::layout::testkit::dispatch;
        use crate::apps::layout::commands::set_locale;
        use crate::apps::layout::LayoutCommand;
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("\"Rahmen\""));
        assert!(json.contains("\"Ebenen\""));
        assert!(!json.contains("\"Frames\""));
    }

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
