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
    document.pages.iter().find_map(|page| {
        if let Some(frame) = page.frames.iter().find(|frame| frame.id() == id) {
            return Some((page.id.as_str(), frame));
        }
        let parent = page.parent_page_id.as_ref().and_then(|parent_id| document.parent_pages.iter().find(|parent| parent.id == *parent_id))?;
        parent.frames.iter().find(|frame| frame.id() == id).map(|frame| (page.id.as_str(), frame))
    })
}


fn layer_select(frame_id: &str, page_id: &str, layer_id: &str, document: &LayoutSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("field", ui_value_text("layerId")?), ("frameId", ui_value_text(frame_id)?), ("pageId", ui_value_text(page_id)?)])?;
    let (action, args) = layout_action("patchFrame", Some(args))?;
    let mut input = ui::select(text(layer_id)?).try_id("layout-play-inspector.patchFrame.layerId.input").map_err(|_| admit())?.try_label(labels.group_layer.as_str()).map_err(|_| admit())?;
    if let Some(page) = document.pages.iter().find(|page| page.id == page_id) {
        for layer in &page.layers {
            input = input.try_item(text(&layer.id)?, ui::Label(text(&layer.name)?)).map_err(|_| admit())?;
        }
    }
    let control = input.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(labels.group_layer.as_str())?)).try_id("layout-play-inspector.patchFrame.layerId").map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn selection_frame(document: &LayoutSnapshot, page_id: &str, frame: &Frame) -> Frame {
    let Some(page) = document.pages.iter().find(|page| page.id == page_id) else { return frame.clone() };
    if page.frames.iter().any(|item| item.id() == frame.id()) {
        return frame.clone();
    }
    let Some(page_override) = page.overrides.iter().find(|item| item.object_id == frame.id()) else { return frame.clone() };
    let mut resolved = frame.clone();
    crate::standards::v1::subsets::any::schema::apply_page_override(&mut resolved, page_override);
    resolved
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
        Field { key: "baselineGrid", label: labels.baseline_grid, value: number(document.grid.baseline_grid), kind: InputKind::Number },
        Field { key: "baselineOffset", label: labels.baseline_offset, value: number(document.grid.baseline_offset), kind: InputKind::Number },
        Field { key: "snapToBaseline", label: labels.snap_to_baseline, value: if document.grid.snap_to_baseline { "true".into() } else { "false".into() }, kind: InputKind::Text },
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
        Field { key: "locked", label: labels.locked, value: if frame.locked() { "true".into() } else { "false".into() }, kind: InputKind::Text },
        Field { key: "visible", label: labels.visible, value: if frame.visible() { "true".into() } else { "false".into() }, kind: InputKind::Text },
    ];
    match frame {
        Frame::Rect { fill, stroke, .. } => {
            rows.push(Field { key: "fill", label: labels.fill, value: rgba_to_hex(fill), kind: InputKind::Color });
            rows.push(Field { key: "stroke", label: labels.stroke, value: rgba_to_hex(stroke), kind: InputKind::Color });
        }
        Frame::Text { columns, story_id, inset, .. } => {
            let story = document.stories.iter().find(|story| story.id == *story_id);
            let content = story.map(|story| story.content.clone()).unwrap_or_default();
            rows.push(Field { key: "columns", label: labels.columns, value: number(columns), kind: InputKind::Number });
            rows.push(Field { key: "storyContent", label: labels.story, value: content.clone(), kind: InputKind::LongText });
            rows.push(Field { key: "insetX", label: labels.inset_x, value: number(inset.x), kind: InputKind::Number });
            rows.push(Field { key: "insetY", label: labels.inset_y, value: number(inset.y), kind: InputKind::Number });
            rows.push(Field { key: "insetWidth", label: labels.inset_width, value: number(inset.width), kind: InputKind::Number });
            rows.push(Field { key: "insetHeight", label: labels.inset_height, value: number(inset.height), kind: InputKind::Number });
            let runs = story.map(|story| story.style_runs.as_slice()).unwrap_or(&[]);
            let shown = if runs.is_empty() { 1 } else { runs.len().min(4) };
            for index in 0..shown {
                let (start, end) = runs.get(index).map(|run| (byte_chars(&content, run.start), byte_chars(&content, run.end))).unwrap_or((0, content.chars().count()));
                if let Some(key) = crate::editor::layout::commands::patch_frame::style_range_key(index, true) {
                    rows.push(Field { key, label: labels.style_start.clone(), value: number(start), kind: InputKind::Number });
                }
                if let Some(key) = crate::editor::layout::commands::patch_frame::style_range_key(index, false) {
                    rows.push(Field { key, label: labels.style_end.clone(), value: number(end), kind: InputKind::Number });
                }
            }
        }
        Frame::Image { link_id, .. } => {
            let link = document.links.iter().find(|link| link.id == *link_id);
            let path = link.map(|link| link.path.clone()).unwrap_or_default();
            let width = link.map(|link| number(link.width)).unwrap_or_default();
            let height = link.map(|link| number(link.height)).unwrap_or_default();
            let dpi = link.map(|link| number(link.dpi)).unwrap_or_default();
            let profile = link.and_then(|link| link.color_profile.clone()).unwrap_or_default();
            rows.push(Field { key: "linkPath", label: labels.link_path, value: path, kind: InputKind::Text });
            rows.push(Field { key: "linkWidth", label: labels.link_width, value: width, kind: InputKind::Number });
            rows.push(Field { key: "linkHeight", label: labels.link_height, value: height, kind: InputKind::Number });
            rows.push(Field { key: "dpi", label: labels.dpi, value: dpi, kind: InputKind::Number });
            rows.push(Field { key: "colorProfile", label: labels.color_profile, value: profile, kind: InputKind::Text });
            if crate::mutations::set_drawing_text::is_drawing_kind(&link.map(|link| link.artifact_kind.as_str()).unwrap_or("")) {
                for (index, text) in crate::mutations::set_drawing_text::drawing_labels(document).into_iter().enumerate() {
                    let Some(key) = crate::mutations::set_drawing_text::drawing_text_field(index) else { break };
                    rows.push(Field { key, label: labels.drawing_text.clone(), value: text, kind: InputKind::Text });
                }
            }
        }
    }
    rows
}


fn byte_chars(content: &str, byte: usize) -> usize {
    content.get(..byte.min(content.len())).unwrap_or(content).chars().count()
}

fn field_row(command: &str, field: &Field, frame_id: Option<&str>, page_id: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    let args = match (frame_id, page_id) {
        (Some(frame_id), Some(page_id)) => ui_value_map([("field", ui_value_text(field.key)?), ("frameId", ui_value_text(frame_id)?), ("pageId", ui_value_text(page_id)?)])?,
        (None, Some(page_id)) => ui_value_map([("field", ui_value_text(field.key)?), ("pageId", ui_value_text(page_id)?)])?,
        _ => ui_value_map([("field", ui_value_text(field.key)?)])?,
    };
    let (action, args) = layout_action(command, Some(args))?;
    let id = format!("layout-play-inspector.{command}.{}.input", field.key);
    if matches!(field.key, "snapToBaseline" | "locked" | "visible") {
        let control = ui::toggle(field.value == "true").try_id(&id).map_err(|_| admit())?.try_label(field.label.as_str()).map_err(|_| admit())?.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
        return ui::tree_item(ui::Label(text(field.label.as_str())?)).try_id(format!("layout-play-inspector.{command}.{}", field.key)).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit());
    }
    let mut input = ui::input(field.kind).value(text(&field.value)?).try_id(&id).map_err(|_| admit())?.try_label(field.label.as_str()).map_err(|_| admit())?;
    let trigger = if matches!(field.kind, InputKind::Color) { Trigger::Change } else { Trigger::Commit };
    if !matches!(field.kind, InputKind::Color) {
        input = input.commit(text("blur")?);
    }
    if field.kind == InputKind::Number {
        input = input.step(if matches!(field.key, "columns" | "columnsCount" | "dpi" | "linkWidth" | "linkHeight") { 1.0 } else { 0.1 });
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

fn paragraph_input(style_id: &str, key: &str, label: LabelText, value: String, kind: InputKind) -> UiAssemblyResult<BuiltNode> {
    let field = format!("{style_id}.{key}");
    let args = ui_value_map([("field", ui_value_text(&field)?)])?;
    let (action, args) = layout_action("patchDocument", Some(args))?;
    let id = format!("layout-play-inspector.patchDocument.{field}.input");
    let mut input = ui::input(kind).value(text(&value)?).try_id(&id).map_err(|_| admit())?.try_label(label.as_str()).map_err(|_| admit())?;
    let trigger = if matches!(kind, InputKind::Color) { Trigger::Change } else { Trigger::Commit };
    if kind == InputKind::Number {
        input = input.step(if key == "fontWeight" { 100.0 } else { 0.1 });
    } else if kind != InputKind::Color {
        input = input.commit(text("blur")?);
    }
    let control = input.try_on_with(trigger, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchDocument.{field}")).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn paragraph_alignment(style_id: &str, alignment: &str, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let field = format!("{style_id}.alignment");
    let args = ui_value_map([("field", ui_value_text(&field)?)])?;
    let (action, args) = layout_action("patchDocument", Some(args))?;
    let mut input = ui::select(text(alignment)?).try_id(format!("layout-play-inspector.patchDocument.{field}.input")).map_err(|_| admit())?.try_label(labels.alignment.as_str()).map_err(|_| admit())?;
    for (value, label) in [("left", labels.align_left), ("center", labels.align_center), ("right", labels.align_right), ("justify", labels.align_justify)] {
        input = input.try_item(text(value)?, ui::Label(text(label.as_str())?)).map_err(|_| admit())?;
    }
    let control = input.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(labels.alignment.as_str())?)).try_id(format!("layout-play-inspector.patchDocument.{field}")).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn paragraph_rows(document: &LayoutSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<semio_framework_plugin::UiFixedList<BuiltNode>> {
    let mut nodes = Vec::new();
    for style in &document.paragraph_styles {
        nodes.push(paragraph_input(&style.id, "name", labels.name, style.name.clone(), InputKind::Text)?);
        nodes.push(paragraph_input(&style.id, "fontFamily", labels.font_family, style.font_family.clone(), InputKind::Text)?);
        nodes.push(paragraph_input(&style.id, "fontSize", labels.font_size, number(style.font_size), InputKind::Number)?);
        nodes.push(paragraph_input(&style.id, "fontWeight", labels.font_weight, number(style.font_weight), InputKind::Number)?);
        nodes.push(paragraph_input(&style.id, "leading", labels.leading, number(style.leading), InputKind::Number)?);
        nodes.push(paragraph_input(&style.id, "tracking", labels.tracking, number(style.tracking), InputKind::Number)?);
        nodes.push(paragraph_alignment(&style.id, &style.alignment, labels)?);
    }
    ui_node_list(nodes.into_iter().map(Ok))
}

fn story_select(frame_id: &str, page_id: &str, story_id: &str, document: &LayoutSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("field", ui_value_text("storyId")?), ("frameId", ui_value_text(frame_id)?), ("pageId", ui_value_text(page_id)?)])?;
    let (action, args) = layout_action("patchFrame", Some(args))?;
    let mut input = ui::select(text(story_id)?).try_id("layout-play-inspector.patchFrame.storyId.input").map_err(|_| admit())?.try_label(labels.story.as_str()).map_err(|_| admit())?;
    for story in &document.stories {
        input = input.try_item(text(&story.id)?, ui::Label(text(&story.id)?)).map_err(|_| admit())?;
    }
    let control = input.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(labels.story.as_str())?)).try_id("layout-play-inspector.patchFrame.storyId").map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}


fn story_style_select(frame_id: &str, page_id: &str, story_id: &str, document: &LayoutSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let current = document.stories.iter().find(|story| story.id == story_id).and_then(|story| story.style_runs.iter().find_map(|run| run.character_style_id.clone())).unwrap_or_default();
    let args = ui_value_map([("field", ui_value_text("characterStyle")?), ("frameId", ui_value_text(frame_id)?), ("pageId", ui_value_text(page_id)?)])?;
    let (action, args) = layout_action("patchFrame", Some(args))?;
    let mut input = ui::select(text(&current)?).try_id("layout-play-inspector.patchFrame.characterStyle.input").map_err(|_| admit())?.try_label(labels.character_style.as_str()).map_err(|_| admit())?;
    input = input.try_item(text("")?, ui::Label(text(labels.thread_none.as_str())?)).map_err(|_| admit())?;
    for style in &document.character_styles {
        let name = style.name.clone().unwrap_or_else(|| style.id.clone());
        input = input.try_item(text(&style.id)?, ui::Label(text(&name)?)).map_err(|_| admit())?;
    }
    let control = input.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(labels.character_style.as_str())?)).try_id("layout-play-inspector.patchFrame.characterStyle").map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn thread_select(frame_id: &str, page_id: &str, thread_next: &Option<String>, document: &LayoutSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let current = thread_next.clone().unwrap_or_default();
    let args = ui_value_map([("field", ui_value_text("threadNext")?), ("frameId", ui_value_text(frame_id)?), ("pageId", ui_value_text(page_id)?)])?;
    let (action, args) = layout_action("patchFrame", Some(args))?;
    let mut input = ui::select(text(&current)?).try_id("layout-play-inspector.patchFrame.threadNext.input").map_err(|_| admit())?.try_label(labels.thread_next.as_str()).map_err(|_| admit())?;
    input = input.try_item(text("")?, ui::Label(text(labels.thread_none.as_str())?)).map_err(|_| admit())?;
    for page in &document.pages {
        for frame in &page.frames {
            if let Frame::Text { id, .. } = frame {
                if id != frame_id {
                    input = input.try_item(text(id)?, ui::Label(text(id)?)).map_err(|_| admit())?;
                }
            }
        }
    }
    let control = input.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(labels.thread_next.as_str())?)).try_id("layout-play-inspector.patchFrame.threadNext").map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn layer_name(page_id: &str, layer_id: &str, name: &str, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let field = format!("{layer_id}.name");
    let args = ui_value_map([("field", ui_value_text(&field)?), ("pageId", ui_value_text(page_id)?), ("value", ui_value_text(name)?)])?;
    let (action, args) = layout_action("patchPage", Some(args))?;
    let control = ui::input(InputKind::Text).value(text(name)?).try_id(format!("layout-play-inspector.patchPage.{field}.input")).map_err(|_| admit())?.try_label(labels.name.as_str()).map_err(|_| admit())?.commit(text("blur")?).try_on_with(Trigger::Commit, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(&format!("{} {}", labels.group_layer.as_str(), name))?)) .try_id(format!("layout-play-inspector.patchPage.{field}")).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn layer_flag(page_id: &str, layer_id: &str, key: &str, on: bool, label: LabelText) -> UiAssemblyResult<BuiltNode> {
    let field = format!("{layer_id}.{key}");
    let args = ui_value_map([("field", ui_value_text(&field)?), ("pageId", ui_value_text(page_id)?)])?;
    let (action, args) = layout_action("patchPage", Some(args))?;
    let control = ui::toggle(on).try_id(format!("layout-play-inspector.patchPage.{field}.input")).map_err(|_| admit())?.try_label(label.as_str()).map_err(|_| admit())?.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchPage.{field}")).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn document_button(field: &str, value: &str, label: LabelText) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("field", ui_value_text(field)?), ("value", ui_value_text(value)?)])?;
    let (action, args) = layout_action("patchDocument", Some(args))?;
    let control = ui::button(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchDocument.{field}.input")).map_err(|_| admit())?.try_on_with(Trigger::Activate, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchDocument.{field}")).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn document_toggle(field: &str, on: bool, label: LabelText) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("field", ui_value_text(field)?)])?;
    let (action, args) = layout_action("patchDocument", Some(args))?;
    let control = ui::toggle(on).try_id(format!("layout-play-inspector.patchDocument.{field}.input")).map_err(|_| admit())?.try_label(label.as_str()).map_err(|_| admit())?.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchDocument.{field}")).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn parent_select(page_id: &str, parent_page_id: &Option<String>, document: &LayoutSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let current = parent_page_id.clone().unwrap_or_default();
    let args = ui_value_map([("field", ui_value_text("parentPageId")?), ("pageId", ui_value_text(page_id)?)])?;
    let (action, args) = layout_action("patchPage", Some(args))?;
    let mut input = ui::select(text(&current)?).try_id("layout-play-inspector.patchPage.parentPageId.input").map_err(|_| admit())?.try_label(labels.parent_page.as_str()).map_err(|_| admit())?;
    input = input.try_item(text("")?, ui::Label(text(labels.thread_none.as_str())?)).map_err(|_| admit())?;
    for parent in &document.parent_pages {
        input = input.try_item(text(&parent.id)?, ui::Label(text(&parent.name)?)).map_err(|_| admit())?;
    }
    let control = input.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(labels.parent_page.as_str())?)).try_id("layout-play-inspector.patchPage.parentPageId").map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}


fn guide_input(page_id: &str, index: usize, key: &str, label: LabelText, value: f64) -> UiAssemblyResult<BuiltNode> {
    let field = format!("guide.{index}.{key}");
    let args = ui_value_map([("field", ui_value_text(&field)?), ("pageId", ui_value_text(page_id)?)])?;
    let (action, args) = layout_action("patchPage", Some(args))?;
    let control = ui::input(InputKind::Number).value(text(&number(value))?).try_id(format!("layout-play-inspector.patchPage.{field}.input")).map_err(|_| admit())?.try_label(label.as_str()).map_err(|_| admit())?.step(0.1).commit(text("blur")?).try_on_with(Trigger::Commit, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchPage.{field}")).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn frame_button(frame_id: &str, page_id: &str, field: &str, label: LabelText) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("field", ui_value_text(field)?), ("frameId", ui_value_text(frame_id)?), ("pageId", ui_value_text(page_id)?), ("value", ui_value_text("true")?)])?;
    let (action, args) = layout_action("patchFrame", Some(args))?;
    let control = ui::button(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchFrame.{field}.input")).map_err(|_| admit())?.try_on_with(Trigger::Activate, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchFrame.{field}")).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn page_button(page_id: &str, field: &str, label: LabelText) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("field", ui_value_text(field)?), ("pageId", ui_value_text(page_id)?), ("value", ui_value_text("true")?)])?;
    let (action, args) = layout_action("patchPage", Some(args))?;
    let control = ui::button(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchPage.{field}.input")).map_err(|_| admit())?.try_on_with(Trigger::Activate, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchPage.{field}")).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
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
    if !document.paragraph_styles.is_empty() {
        builder = builder.section("layout-play-inspector.paragraph-style", Some(ui_label(labels.paragraph_style.as_str())?), true, paragraph_rows(document, labels)?)?;
    }
    let mut character_nodes = vec![document_button("addCharacterStyle", "Emphasis", labels.add_character_style)?];
    for style in &document.character_styles {
        character_nodes.push(paragraph_input(&style.id, "name", labels.name, style.name.clone().unwrap_or_default(), InputKind::Text)?);
        character_nodes.push(paragraph_input(&style.id, "fontFamily", labels.font_family, style.font_family.clone().unwrap_or_default(), InputKind::Text)?);
        character_nodes.push(paragraph_input(&style.id, "fontSize", labels.font_size, style.font_size.map(number).unwrap_or_default(), InputKind::Number)?);
        character_nodes.push(paragraph_input(&style.id, "fontWeight", labels.font_weight, style.font_weight.map(number).unwrap_or_default(), InputKind::Number)?);
        character_nodes.push(document_toggle(&format!("{}.italic", style.id), style.italic.unwrap_or(false), labels.italic)?);
        character_nodes.push(paragraph_input(&style.id, "color", labels.color, rgba_to_hex(&style.color), InputKind::Color)?);
        character_nodes.push(paragraph_input(&style.id, "tracking", labels.tracking, style.tracking.map(number).unwrap_or_default(), InputKind::Number)?);
        character_nodes.push(document_button(&format!("{}.delete", style.id), "true", labels.delete_character_style)?);
    }
    builder = builder.section("layout-play-inspector.character-style", Some(ui_label(labels.character_style.as_str())?), true, ui_node_list(character_nodes.into_iter().map(Ok))?)?;
    let mut structure = Vec::new();
    for spread in &document.spreads {
        structure.push(paragraph_input(&spread.id, "name", labels.spread, spread.name.clone(), InputKind::Text)?);
    }
    for parent in &document.parent_pages {
        structure.push(paragraph_input(&parent.id, "name", labels.parent_page, parent.name.clone(), InputKind::Text)?);
        structure.push(paragraph_input(&parent.id, "width", labels.width, number(parent.width), InputKind::Number)?);
        structure.push(paragraph_input(&parent.id, "height", labels.height, number(parent.height), InputKind::Number)?);
    }
    if !structure.is_empty() {
        builder = builder.section("layout-play-inspector.structure", Some(ui_label(labels.spread.as_str())?), true, ui_node_list(structure.into_iter().map(Ok))?)?;
    }
    if let Some(page) = document.pages.iter().find(|page| page.id == config.active_page_id) {
        let mut page_rows = rows(&page_fields(page, labels), "patchPage", None, Some(&page.id))?;
        page_rows.try_push(parent_select(&page.id, &page.parent_page_id, document, labels)?).map_err(|_| admit())?;
        page_rows.try_push(page_button(&page.id, "addGuide", labels.add_guide)?).map_err(|_| admit())?;
        for (index, guide) in page.guides.iter().enumerate() {
            page_rows.try_push(guide_input(&page.id, index, "x", labels.x, guide.x)?).map_err(|_| admit())?;
            page_rows.try_push(guide_input(&page.id, index, "y", labels.y, guide.y)?).map_err(|_| admit())?;
            page_rows.try_push(guide_input(&page.id, index, "width", labels.width, guide.width)?).map_err(|_| admit())?;
            page_rows.try_push(guide_input(&page.id, index, "height", labels.height, guide.height)?).map_err(|_| admit())?;
            page_rows.try_push(page_button(&page.id, &format!("guide.{index}.delete"), labels.delete_guide)?).map_err(|_| admit())?;
        }
        page_rows.try_push(page_button(&page.id, "moveEarlier", labels.move_earlier)?).map_err(|_| admit())?;
        page_rows.try_push(page_button(&page.id, "moveLater", labels.move_later)?).map_err(|_| admit())?;
        if document.pages.len() > 1 {
            page_rows.try_push(page_button(&page.id, "delete", labels.delete_page)?).map_err(|_| admit())?;
        }
        page_rows.try_push(page_button(&page.id, "addLayer", labels.add_layer)?).map_err(|_| admit())?;
        for layer in &page.layers {
            page_rows.try_push(layer_name(&page.id, &layer.id, &layer.name, labels)?).map_err(|_| admit())?;
            page_rows.try_push(layer_flag(&page.id, &layer.id, "visible", layer.visible, labels.visible)?).map_err(|_| admit())?;
            page_rows.try_push(layer_flag(&page.id, &layer.id, "locked", layer.locked, labels.locked)?).map_err(|_| admit())?;
        }
        builder = builder.section("layout-play-inspector.page", Some(ui_label(labels.group_page.as_str())?), true, page_rows)?;
    }
    let Some(frame_id) = interaction.ids.first() else { return builder.build() };
    let Some((page_id, source)) = locate_frame(document, frame_id) else {
        let missing = ui_node_list([tree_item_desc("layout-play-inspector.missing", ui_label(labels.selection_not_found.as_str())?, None)])?;
        return builder.section("layout-play-inspector.frame", Some(ui_label(labels.group_frame.as_str())?), true, missing)?.build();
    };
    let frame = selection_frame(document, page_id, source);
    let mut frame_rows = rows(&frame_fields(&frame, document, labels), "patchFrame", Some(frame_id), Some(page_id))?;
    if document.pages.iter().any(|page| page.id == page_id && page.frames.iter().any(|item| item.id() == frame.id())) {
        frame_rows.try_push(layer_select(frame_id, page_id, frame.layer_id(), document, labels)?).map_err(|_| admit())?;
        frame_rows.try_push(frame_button(frame_id, page_id, "forward", labels.bring_forward)?).map_err(|_| admit())?;
        frame_rows.try_push(frame_button(frame_id, page_id, "backward", labels.send_backward)?).map_err(|_| admit())?;
    }
    frame_rows.try_push(tree_item_desc("layout-play-inspector.frame.kind", ui_label(labels.kind.as_str())?, Some(frame_kind_label(&frame, labels).to_string()))?).map_err(|_| admit())?;
    if let Frame::Text { wrap_mode, story_id, thread_next, .. } = &frame {
        frame_rows.try_push(wrap_row(frame_id, page_id, wrap_mode, labels)?).map_err(|_| admit())?;
        frame_rows.try_push(story_select(frame_id, page_id, story_id, document, labels)?).map_err(|_| admit())?;
        frame_rows.try_push(story_style_select(frame_id, page_id, story_id, document, labels)?).map_err(|_| admit())?;
        frame_rows.try_push(thread_select(frame_id, page_id, thread_next, document, labels)?).map_err(|_| admit())?;
    }
    if let Frame::Image { link_id, .. } = &frame {
        if let Some(link) = document.links.iter().find(|link| link.id == *link_id) {
            if !link.artifact_kind.is_empty() {
                frame_rows.try_push(tree_item_desc("layout-play-inspector.frame.artifact", ui_label(labels.artifact.as_str())?, Some(format!("{} {}", link.artifact_kind, link.artifact_ref)))?).map_err(|_| admit())?;
            }
            if crate::editor::layout::panels::catalogue::native_open(&link.artifact_kind).is_some() && !link.artifact_ref.trim().is_empty() {
                frame_rows.try_push(frame_button(frame_id, page_id, "open", labels.open_source)?).map_err(|_| admit())?;
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
