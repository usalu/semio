//! 🔍️ Layout inspector: document summary, then editable document, page, and frame fields.

use crate::editor::layout::modes::edit::windows::blueprint::config::LayoutWindowConfig;
use crate::editor::layout::terminology::LayoutLabels;
use crate::editor::layout::{layout_action, ui_label, ui_value_map, ui_value_text, LayoutInteractionSnapshot};
use crate::standards::v1::subsets::any::schema::rgba_to_hex;
use crate::{Frame, LayoutSnapshot, Page, LAYOUT_DOCUMENT_SCHEMA};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{tree_item_desc, ui_node_list, BuiltNode, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract as ui;

pub const LAYOUT_PLAY_BODY_INSPECTION: &str = "layout.play.inspection";

pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(LAYOUT_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}

fn admit() -> PluginAssemblyError {
    PluginAssemblyError::new("layout.inspector.capacity", "Layout inspector exceeds its UI capacity")
}

fn text(value: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(admit)
}

struct Field {
    key: &'static str,
    label: LabelText,
    value: String,
    kind: InputKind,
}

fn number(value: impl ToString) -> String {
    value.to_string()
}

fn locate_frame<'a>(document: &'a LayoutSnapshot, id: &str) -> Option<(&'a str, &'a Frame)> {
    document.pages.iter().find_map(|page| page.frames.iter().find(|frame| frame.id() == id).map(|frame| (page.id.as_str(), frame)))
}

fn frame_kind_label<'a>(frame: &Frame, labels: &'a LayoutLabels) -> &'a str {
    match frame.kind_str() {
        "rect" => labels.kind_rect.as_str(),
        "text" => labels.kind_text.as_str(),
        "image" => labels.kind_image.as_str(),
        _ => labels.kind.as_str(),
    }
}

fn document_fields(document: &LayoutSnapshot, labels: &LayoutLabels) -> Vec<Field> {
    vec![
        Field { key: "name", label: labels.name, value: document.name.clone(), kind: InputKind::Text },
        Field { key: "printTarget", label: labels.print_target, value: document.print_target.clone().unwrap_or_default(), kind: InputKind::Text },
        Field { key: "dataFields", label: labels.data_fields, value: document.data_fields_json.clone().unwrap_or_default(), kind: InputKind::LongText },
    ]
}

fn page_fields(page: &Page, labels: &LayoutLabels) -> Vec<Field> {
    vec![
        Field { key: "name", label: labels.name, value: page.name.clone(), kind: InputKind::Text },
        Field { key: "width", label: labels.width, value: number(page.width), kind: InputKind::Number },
        Field { key: "height", label: labels.height, value: number(page.height), kind: InputKind::Number },
        Field { key: "marginTop", label: labels.margin_top, value: number(page.margins.top), kind: InputKind::Number },
        Field { key: "marginRight", label: labels.margin_right, value: number(page.margins.right), kind: InputKind::Number },
        Field { key: "marginBottom", label: labels.margin_bottom, value: number(page.margins.bottom), kind: InputKind::Number },
        Field { key: "marginLeft", label: labels.margin_left, value: number(page.margins.left), kind: InputKind::Number },
        Field { key: "columnsCount", label: labels.columns, value: number(page.columns.count), kind: InputKind::Number },
        Field { key: "columnsGutter", label: labels.gutter, value: number(page.columns.gutter), kind: InputKind::Number },
    ]
}

fn frame_fields(frame: &Frame, document: &LayoutSnapshot, labels: &LayoutLabels) -> Vec<Field> {
    let bounds = frame.bounds();
    let mut rows = vec![
        Field { key: "x", label: labels.x, value: number(bounds.x), kind: InputKind::Number },
        Field { key: "y", label: labels.y, value: number(bounds.y), kind: InputKind::Number },
        Field { key: "width", label: labels.width, value: number(bounds.width), kind: InputKind::Number },
        Field { key: "height", label: labels.height, value: number(bounds.height), kind: InputKind::Number },
        Field { key: "rotation", label: labels.rotation, value: number(bounds.rotation), kind: InputKind::Number },
    ];
    match frame {
        Frame::Rect { fill, stroke, .. } => {
            rows.push(Field { key: "fill", label: labels.fill, value: rgba_to_hex(fill), kind: InputKind::Color });
            rows.push(Field { key: "stroke", label: labels.stroke, value: rgba_to_hex(stroke), kind: InputKind::Color });
        }
        Frame::Text { columns, story_id, .. } => {
            let content = document.stories.iter().find(|story| story.id == *story_id).map(|story| story.content.clone()).unwrap_or_default();
            rows.push(Field { key: "columns", label: labels.columns, value: number(columns), kind: InputKind::Number });
            rows.push(Field { key: "storyContent", label: labels.story, value: content, kind: InputKind::LongText });
        }
        Frame::Image { link_id, .. } => {
            let path = document.links.iter().find(|link| link.id == *link_id).map(|link| link.path.clone()).unwrap_or_default();
            rows.push(Field { key: "linkPath", label: labels.link_path, value: path, kind: InputKind::Text });
        }
    }
    rows
}

fn field_row(command: &str, field: &Field, frame_id: Option<&str>, page_id: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    let args = match (frame_id, page_id) {
        (Some(frame_id), Some(page_id)) => ui_value_map([("field", ui_value_text(field.key)?), ("frameId", ui_value_text(frame_id)?), ("pageId", ui_value_text(page_id)?)])?,
        (None, Some(page_id)) => ui_value_map([("field", ui_value_text(field.key)?), ("pageId", ui_value_text(page_id)?)])?,
        _ => ui_value_map([("field", ui_value_text(field.key)?)])?,
    };
    let (action, args) = layout_action(command, Some(args))?;
    let id = format!("layout-play-inspector.{command}.{}.input", field.key);
    let mut input = ui::input(field.kind).value(text(&field.value)?).try_id(&id).map_err(|_| admit())?.try_label(field.label.as_str()).map_err(|_| admit())?;
    let trigger = if matches!(field.kind, InputKind::Color) { Trigger::Change } else { Trigger::Commit };
    if !matches!(field.kind, InputKind::Color) {
        input = input.commit(text("blur")?);
    }
    if field.kind == InputKind::Number {
        input = input.step(if matches!(field.key, "columns" | "columnsCount") { 1.0 } else { 0.1 });
    }
    let control = input.try_on_with(trigger, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(field.label.as_str())?)).try_id(format!("layout-play-inspector.{}.{}", command, field.key)).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn wrap_row(frame_id: &str, page_id: &str, wrap_mode: &str, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("field", ui_value_text("wrapMode")?), ("frameId", ui_value_text(frame_id)?), ("pageId", ui_value_text(page_id)?)])?;
    let (action, args) = layout_action("patchFrame", Some(args))?;
    let mut input = ui::select(text(wrap_mode)?).try_id("layout-play-inspector.patchFrame.wrapMode").map_err(|_| admit())?.try_label(labels.wrap_mode.as_str()).map_err(|_| admit())?;
    for (value, label) in [("none", labels.wrap_none), ("box", labels.wrap_box), ("contour", labels.wrap_contour)] {
        input = input.try_item(text(value)?, ui::Label(text(label.as_str())?)).map_err(|_| admit())?;
    }
    let control = input.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(labels.wrap_mode.as_str())?)).try_id("layout-play-inspector.frame.wrapMode").map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn rows(fields: &[Field], command: &str, frame_id: Option<&str>, page_id: Option<&str>) -> UiAssemblyResult<semio_framework_plugin::UiFixedList<BuiltNode>> {
    ui_node_list(fields.iter().map(|field| field_row(command, field, frame_id, page_id)))
}

/// 🕹️ Summary stays a read-only tree. Document, active page, and the selected frame are inputs bound to `patchDocument`, `patchPage`, and `patchFrame`.
pub fn render(document: &LayoutSnapshot, config: &LayoutWindowConfig, interaction: &LayoutInteractionSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let items = ui_node_list([
        tree_item_desc("layout-play-inspector.schema", ui_label(labels.schema.as_str())?, Some(LAYOUT_DOCUMENT_SCHEMA.into())),
        tree_item_desc("layout-play-inspector.name", ui_label(labels.name.as_str())?, Some(document.name.clone())),
        tree_item_desc("layout-play-inspector.pages", ui_label(labels.pages.as_str())?, Some(document.pages.len().to_string())),
        tree_item_desc("layout-play-inspector.active-page", ui_label(labels.active_page.as_str())?, Some(config.active_page_id.clone())),
        tree_item_desc("layout-play-inspector.selected-count", ui_label(labels.selected.as_str())?, Some(interaction.ids.len().to_string())),
    ])?;
    let mut builder = PanelTreeBuilder::new("layout-play-inspector")?.section("layout-play-inspector.summary", Some(ui_label(labels.inspection.as_str())?), true, items)?;
    builder = builder.section("layout-play-inspector.document", Some(ui_label(labels.group_document.as_str())?), true, rows(&document_fields(document, labels), "patchDocument", None, None)?)?;
    if let Some(page) = document.pages.iter().find(|page| page.id == config.active_page_id) {
        builder = builder.section("layout-play-inspector.page", Some(ui_label(labels.group_page.as_str())?), true, rows(&page_fields(page, labels), "patchPage", None, Some(&page.id))?)?;
    }
    let Some(frame_id) = interaction.ids.first() else { return builder.build() };
    let Some((page_id, frame)) = locate_frame(document, frame_id) else {
        let missing = ui_node_list([tree_item_desc("layout-play-inspector.missing", ui_label(labels.selection_not_found.as_str())?, None)])?;
        return builder.section("layout-play-inspector.frame", Some(ui_label(labels.group_frame.as_str())?), true, missing)?.build();
    };
    let mut frame_rows = rows(&frame_fields(frame, document, labels), "patchFrame", Some(frame_id), Some(page_id))?;
    frame_rows.try_push(tree_item_desc("layout-play-inspector.frame.kind", ui_label(labels.kind.as_str())?, Some(frame_kind_label(frame, labels).to_string()))?).map_err(|_| admit())?;
    if let Frame::Text { wrap_mode, .. } = frame {
        frame_rows.try_push(wrap_row(frame_id, page_id, wrap_mode, labels)?).map_err(|_| admit())?;
    }
    if let Frame::Image { link_id, .. } = frame {
        if let Some(link) = document.links.iter().find(|link| link.id == *link_id) {
            if !link.artifact_kind.is_empty() {
                frame_rows.try_push(tree_item_desc("layout-play-inspector.frame.artifact", ui_label(labels.artifact.as_str())?, Some(format!("{} {}", link.artifact_kind, link.artifact_ref)))?).map_err(|_| admit())?;
            }
        }
    }
    builder.section("layout-play-inspector.frame", Some(ui_label(labels.group_frame.as_str())?), true, frame_rows)?.build()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️semantic-contract/🦀️.rs"]
mod semantic_contract;
