//! 🔍️ Layout inspector: document summary, then editable document, page, and frame fields.

use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::config::LayoutWindowConfig;
use crate::terminology::LayoutLabels;
use semio_s_artifact_layout_layout_editor_commands::{layout_action, ui_label, ui_value_map, ui_value_text, LayoutInteractionSnapshot};
use semio_s_artifact_layout_layout_model::rgba_to_hex;
use semio_s_artifact_layout_layout_editor_commands::patch_document::text_chunks;
use semio_s_artifact_layout_layout_model::{Frame, Page, LAYOUT_DOCUMENT_SCHEMA};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::{InputKind, Trigger};
use semio_framework_plugin::{Buildable, HasBase, HasChildren};
use semio_framework_plugin::tree_item_desc;
use semio_framework_plugin::ui_node_list;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::LabelText;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiText;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
use semio_framework_ui_contract as ui;

#[path = "🧾️data-fields/🦀️.rs"]
mod data_fields;

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
    key: String,
    label: LabelText,
    value: String,
    kind: InputKind,
}

fn number(value: impl ToString) -> String {
    value.to_string()
}

fn locate_frame<'a>(document: &'a LayoutSnapshot, id: &str, active_page_id: &str) -> Option<(&'a str, &'a Frame)> {
    document.pages.iter().filter(|page| page.id == active_page_id).chain(document.pages.iter().filter(|page| page.id != active_page_id)).find_map(|page| {
        if let Some(frame) = page.frames.iter().find(|frame| frame.id() == id) {
            return Some((page.id.as_str(), frame));
        }
        let parent = page.parent_page_id.as_ref().and_then(|parent_id| document.parent_pages.iter().find(|parent| parent.id == *parent_id))?;
        parent.frames.iter().find(|frame| frame.id() == id).map(|frame| (page.id.as_str(), frame))
    })
}


fn layer_select(windows: &semio_framework_plugin::TreeWindows<'_>, frame_id: &str, page_id: &str, layer_id: &str, document: &LayoutSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let choices = document.pages.iter().find(|page| page.id == page_id).into_iter().flat_map(|page| &page.layers).map(|layer| (layer.id.as_str(), layer.name.as_str()));
    choice_row(windows, "patchFrame", "layerId", labels.group_layer, layer_id, choices, Some(frame_id), Some(page_id))
}

fn selection_frame(document: &LayoutSnapshot, page_id: &str, frame: &Frame) -> Frame {
    let Some(page) = document.pages.iter().find(|page| page.id == page_id) else { return frame.clone() };
    if page.frames.iter().any(|item| item.id() == frame.id()) {
        return frame.clone();
    }
    let Some(page_override) = page.overrides.iter().find(|item| item.object_id == frame.id()) else { return frame.clone() };
    let mut resolved = frame.clone();
    semio_s_artifact_layout_layout_model::apply_page_override(&mut resolved, page_override);
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
        Field { key: "name".into(), label: labels.name, value: document.name.clone(), kind: InputKind::Text },
        Field { key: "printTarget".into(), label: labels.print_target, value: document.print_target.clone().unwrap_or_default(), kind: InputKind::Text },
        Field { key: "baselineGrid".into(), label: labels.baseline_grid, value: number(document.grid.baseline_grid), kind: InputKind::Number },
        Field { key: "baselineOffset".into(), label: labels.baseline_offset, value: number(document.grid.baseline_offset), kind: InputKind::Number },
        Field { key: "snapToBaseline".into(), label: labels.snap_to_baseline, value: if document.grid.snap_to_baseline { "true".into() } else { "false".into() }, kind: InputKind::Text },
    ]
}

fn page_fields(page: &Page, labels: &LayoutLabels) -> Vec<Field> {
    vec![
        Field { key: "name".into(), label: labels.name, value: page.name.clone(), kind: InputKind::Text },
        Field { key: "width".into(), label: labels.width, value: number(page.width), kind: InputKind::Number },
        Field { key: "height".into(), label: labels.height, value: number(page.height), kind: InputKind::Number },
        Field { key: "marginTop".into(), label: labels.margin_top, value: number(page.margins.top), kind: InputKind::Number },
        Field { key: "marginRight".into(), label: labels.margin_right, value: number(page.margins.right), kind: InputKind::Number },
        Field { key: "marginBottom".into(), label: labels.margin_bottom, value: number(page.margins.bottom), kind: InputKind::Number },
        Field { key: "marginLeft".into(), label: labels.margin_left, value: number(page.margins.left), kind: InputKind::Number },
        Field { key: "columnsCount".into(), label: labels.columns, value: number(page.columns.count), kind: InputKind::Number },
        Field { key: "columnsGutter".into(), label: labels.gutter, value: number(page.columns.gutter), kind: InputKind::Number },
    ]
}

fn frame_fields(frame: &Frame, document: &LayoutSnapshot, labels: &LayoutLabels) -> Vec<Field> {
    let bounds = frame.bounds();
    let mut rows = vec![
        Field { key: "x".into(), label: labels.x, value: number(bounds.x), kind: InputKind::Number },
        Field { key: "y".into(), label: labels.y, value: number(bounds.y), kind: InputKind::Number },
        Field { key: "width".into(), label: labels.width, value: number(bounds.width), kind: InputKind::Number },
        Field { key: "height".into(), label: labels.height, value: number(bounds.height), kind: InputKind::Number },
        Field { key: "rotation".into(), label: labels.rotation, value: number(bounds.rotation), kind: InputKind::Number },
        Field { key: "locked".into(), label: labels.locked, value: if frame.locked() { "true".into() } else { "false".into() }, kind: InputKind::Text },
        Field { key: "visible".into(), label: labels.visible, value: if frame.visible() { "true".into() } else { "false".into() }, kind: InputKind::Text },
    ];
    match frame {
        Frame::Rect { fill, stroke, .. } => {
            rows.push(Field { key: "fillEnabled".into(), label: labels.fill_enabled, value: fill.is_some().to_string(), kind: InputKind::Text });
            rows.push(Field { key: "fillAlpha".into(), label: labels.fill_alpha, value: number(fill.map(|color| color[3]).unwrap_or(1.0)), kind: InputKind::Number });
            rows.push(Field { key: "strokeEnabled".into(), label: labels.stroke_enabled, value: stroke.is_some().to_string(), kind: InputKind::Text });
            rows.push(Field { key: "strokeAlpha".into(), label: labels.stroke_alpha, value: number(stroke.map(|color| color[3]).unwrap_or(1.0)), kind: InputKind::Number });
            rows.push(Field { key: "fill".into(), label: labels.fill, value: rgba_to_hex(fill), kind: InputKind::Color });
            rows.push(Field { key: "stroke".into(), label: labels.stroke, value: rgba_to_hex(stroke), kind: InputKind::Color });
        }
        Frame::Text { columns, inset, .. } => {
            rows.push(Field { key: "columns".into(), label: labels.columns, value: number(columns), kind: InputKind::Number });

            rows.push(Field { key: "insetX".into(), label: labels.inset_x, value: number(inset.x), kind: InputKind::Number });
            rows.push(Field { key: "insetY".into(), label: labels.inset_y, value: number(inset.y), kind: InputKind::Number });
            rows.push(Field { key: "insetWidth".into(), label: labels.inset_width, value: number(inset.width), kind: InputKind::Number });
            rows.push(Field { key: "insetHeight".into(), label: labels.inset_height, value: number(inset.height), kind: InputKind::Number });

        }
        Frame::Image { link_id, .. } => {
            let link = document.links.iter().find(|link| link.id == *link_id);
            let path = link.map(|link| link.path.clone()).unwrap_or_default();
            let width = link.map(|link| number(link.width)).unwrap_or_default();
            let height = link.map(|link| number(link.height)).unwrap_or_default();
            let dpi = link.map(|link| number(link.dpi)).unwrap_or_default();
            let profile = link.and_then(|link| link.color_profile.clone()).unwrap_or_default();
            rows.push(Field { key: "linkPath".into(), label: labels.link_path, value: path, kind: InputKind::Text });
            rows.push(Field { key: "linkWidth".into(), label: labels.link_width, value: width, kind: InputKind::Number });
            rows.push(Field { key: "linkHeight".into(), label: labels.link_height, value: height, kind: InputKind::Number });
            rows.push(Field { key: "dpi".into(), label: labels.dpi, value: dpi, kind: InputKind::Number });
            rows.push(Field { key: "colorProfile".into(), label: labels.color_profile, value: profile, kind: InputKind::Text });
        }
    }
    rows
}


fn byte_chars(content: &str, byte: u64) -> usize {
    let byte=usize::try_from(byte).unwrap_or(content.len());
    content.get(..byte.min(content.len())).unwrap_or(content).chars().count()
}

fn field_row(command: &str, field: &Field, frame_id: Option<&str>, page_id: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    let args = match (frame_id, page_id) {
        (Some(frame_id), Some(page_id)) => ui_value_map([("field", ui_value_text(&field.key)?), ("frameId", ui_value_text(frame_id)?), ("pageId", ui_value_text(page_id)?)])?,
        (None, Some(page_id)) => ui_value_map([("field", ui_value_text(&field.key)?), ("pageId", ui_value_text(page_id)?)])?,
        _ => ui_value_map([("field", ui_value_text(&field.key)?)])?,
    };
    let (action, args) = layout_action(command, Some(args))?;
    let id = format!("layout-play-inspector.{command}.{}.input", field.key);
    let key = field.key.rsplit('.').next().unwrap_or(&field.key);
    if matches!(key, "snapToBaseline" | "locked" | "visible" | "fillEnabled" | "strokeEnabled") {
        let control = ui::toggle(field.value == "true").try_id(&id).map_err(|_| admit())?.try_label(field.label.as_str()).map_err(|_| admit())?.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
        return ui::tree_item(ui::Label(text(field.label.as_str())?)).try_id(format!("layout-play-inspector.{command}.{}", field.key)).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit());
    }
    let mut input = ui::input(field.kind).value(text(&field.value)?).try_id(&id).map_err(|_| admit())?.try_label(field.label.as_str()).map_err(|_| admit())?;
    let trigger = if matches!(field.kind, InputKind::Color) { Trigger::Change } else { Trigger::Commit };
    if !matches!(field.kind, InputKind::Color) {
        input = input.commit(text("blur")?);
    }
    if field.kind == InputKind::Number {
        input = input.step(if matches!(key, "columns" | "columnsCount" | "dpi" | "linkWidth" | "linkHeight") || field.key.starts_with("run.") { 1.0 } else { 0.1 });
        if field.key.ends_with("Alpha") { input = input.min(0.0).max(1.0).step(0.01); }
        if matches!(key, "width" | "height" | "columns" | "columnsCount" | "dpi" | "linkWidth" | "linkHeight") { input = input.min(1.0); }
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
    document_input(&format!("layout-play-inspector.patchDocument.{field}"), &field, label, value, kind)
}

fn document_input(location: &str, field: &str, label: LabelText, value: String, kind: InputKind) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("field", ui_value_text(field)?)])?;
    let (action, args) = layout_action("patchDocument", Some(args))?;
    let mut input = ui::input(kind).value(text(&value)?).try_id(format!("{location}.input")).map_err(|_| admit())?.try_label(label.as_str()).map_err(|_| admit())?;
    let trigger = if matches!(kind, InputKind::Color) { Trigger::Change } else { Trigger::Commit };
    if kind == InputKind::Number {
        let key = field.rsplit('.').next().unwrap_or(field);
        input = input.step(if key == "fontWeight" { 100.0 } else { 0.1 });
        if key == "colorAlpha" { input = input.min(0.0).max(1.0).step(0.01); }
    }
    if kind != InputKind::Color { input = input.commit(text("blur")?); }
    let control = input.try_on_with(trigger, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(location).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
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

fn grouped(id: &str, label: &str, children: impl IntoIterator<Item = BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    let mut row = ui::tree_item(ui::Label(text(label)?)).try_id(id).map_err(|_| admit())?;
    for child in children { row = row.try_child(child).map_err(|_| admit())?; }
    row.try_build().map_err(|_| admit())
}

fn paragraph_row(style: &semio_s_artifact_layout_layout_model::ParagraphStyle, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    grouped(&format!("layout-play-inspector.paragraph.{}", style.id), &style.name, [
        paragraph_input(&style.id, "name", labels.name, style.name.clone(), InputKind::Text)?,
        paragraph_input(&style.id, "fontFamily", labels.font_family, style.font_family.clone(), InputKind::Text)?,
        paragraph_input(&style.id, "fontSize", labels.font_size, number(style.font_size), InputKind::Number)?,
        paragraph_input(&style.id, "fontWeight", labels.font_weight, number(style.font_weight), InputKind::Number)?,
        paragraph_input(&style.id, "leading", labels.leading, number(style.leading), InputKind::Number)?,
        paragraph_input(&style.id, "tracking", labels.tracking, number(style.tracking), InputKind::Number)?,
        paragraph_alignment(&style.id, &style.alignment, labels)?,
        document_button(&format!("{}.delete", style.id), "true", labels.delete_paragraph_style)?,
    ])
}

fn character_row(style: &semio_s_artifact_layout_layout_model::CharacterStyle, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    grouped(&format!("layout-play-inspector.character.{}", style.id), style.name.as_deref().unwrap_or(&style.id), [
        paragraph_input(&style.id, "name", labels.name, style.name.clone().unwrap_or_default(), InputKind::Text)?,
        paragraph_input(&style.id, "fontFamily", labels.font_family, style.font_family.clone().unwrap_or_default(), InputKind::Text)?,
        paragraph_input(&style.id, "fontSize", labels.font_size, style.font_size.map(number).unwrap_or_default(), InputKind::Number)?,
        paragraph_input(&style.id, "fontWeight", labels.font_weight, style.font_weight.map(number).unwrap_or_default(), InputKind::Number)?,
        select_row("patchDocument", &format!("{}.italic", style.id), labels.italic, style.italic.map(|flag| if flag { "true" } else { "false" }).unwrap_or(""), [("", labels.inherit.as_str()), ("true", labels.on.as_str()), ("false", labels.off.as_str())], None, None)?,
        document_toggle(&format!("{}.colorEnabled", style.id), style.color.is_some(), labels.color_enabled)?,
        paragraph_input(&style.id, "color", labels.color, rgba_to_hex(&style.color), InputKind::Color)?,
        paragraph_input(&style.id, "colorAlpha", labels.opacity, number(style.color.map(|color| color[3]).unwrap_or(1.0)), InputKind::Number)?,
        paragraph_input(&style.id, "tracking", labels.tracking, style.tracking.map(number).unwrap_or_default(), InputKind::Number)?,
        document_button(&format!("{}.delete", style.id), "true", labels.delete_character_style)?,
    ])
}

fn story_select(windows: &semio_framework_plugin::TreeWindows<'_>, frame_id: &str, page_id: &str, story_id: &str, document: &LayoutSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    choice_row(windows, "patchFrame", "storyId", labels.story, story_id, document.stories.iter().map(|story| (story.id.as_str(), story.id.as_str())), Some(frame_id), Some(page_id))
}

fn style_run(windows: &semio_framework_plugin::TreeWindows<'_>, frame_id: &str, page_id: &str, index: usize, run: &semio_s_artifact_layout_layout_model::TextStyleRun, content: &str, document: &LayoutSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let prefix = format!("run.{index}");
    let mut row = ui::tree_item(ui::Label(text(&format!("{} {}", labels.style_run.as_str(), index + 1))?)).try_id(format!("layout-play-inspector.patchFrame.{prefix}")).map_err(|_| admit())?;
    for (key, label, value) in [("start", labels.style_start, byte_chars(content, run.start)), ("end", labels.style_end, byte_chars(content, run.end))] {
        row = row.try_child(field_row("patchFrame", &Field { key: format!("{prefix}.{key}"), label, value: number(value), kind: InputKind::Number }, Some(frame_id), Some(page_id))?).map_err(|_| admit())?;
    }
    row=row.try_child(choice_row(windows,"patchFrame",&format!("{prefix}.paragraphStyle"),labels.paragraph_style,run.paragraph_style_id.as_deref().unwrap_or(""),std::iter::once(("",labels.thread_none.as_str())).chain(document.paragraph_styles.iter().map(|style|(style.id.as_str(),style.name.as_str()))),Some(frame_id),Some(page_id))?).map_err(|_|admit())?;
    row=row.try_child(choice_row(windows,"patchFrame",&format!("{prefix}.characterStyle"),labels.character_style,run.character_style_id.as_deref().unwrap_or(""),std::iter::once(("",labels.thread_none.as_str())).chain(document.character_styles.iter().map(|style|(style.id.as_str(),style.name.as_deref().unwrap_or(&style.id)))),Some(frame_id),Some(page_id))?).map_err(|_|admit())?;
    row.try_child(frame_button(frame_id, page_id, &format!("{prefix}.delete"), labels.delete_style_run)?).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn select_row<'a>(command: &str, field: &str, label: LabelText, value: &str, choices: impl IntoIterator<Item = (&'a str, &'a str)>, frame_id: Option<&str>, page_id: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    select_row_at(&format!("layout-play-inspector.{command}.{field}"), command, field, label, value, choices, frame_id, page_id)
}

fn select_row_at<'a>(location: &str, command: &str, field: &str, label: LabelText, value: &str, choices: impl IntoIterator<Item = (&'a str, &'a str)>, frame_id: Option<&str>, page_id: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    let mut args = vec![("field", ui_value_text(field)?)];
    if let Some(id) = frame_id { args.push(("frameId", ui_value_text(id)?)); }
    if let Some(id) = page_id { args.push(("pageId", ui_value_text(id)?)); }
    let (action, args) = layout_action(command, Some(ui_value_map(args)?))?;
    let id = location;
    let mut input = ui::select(text(value)?).try_id(format!("{id}.input")).map_err(|_| admit())?.try_label(label.as_str()).map_err(|_| admit())?;
    for (value, label) in choices { input = input.try_item(text(value)?, ui::Label(text(label)?)).map_err(|_| admit())?; }
    let control = input.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(id).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn choice_row<'a>(windows: &semio_framework_plugin::TreeWindows<'_>, command: &str, field: &str, label: LabelText, current: &str, choices: impl Iterator<Item = (&'a str, &'a str)>+Clone, frame_id: Option<&str>, page_id: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    let id = format!("layout-play-inspector.{command}.{field}");
    let selected = choices.clone().find(|(value, _)| *value == current).map_or(current, |(_, label)| label);
    let item = ui::tree_item(ui::Label(text(label.as_str())?)).try_id(&id).map_err(|_| admit())?.description(text(selected)?);
    semio_framework_plugin::tree_window_indexed_item(windows, item, &id, false, choices.clone().count(), |index| {
        let (value,title)=choices.clone().nth(index).ok_or_else(admit)?;
        let mut args = vec![("field", ui_value_text(field)?), ("value", ui_value_text(value)?)];
        if let Some(frame_id) = frame_id { args.push(("frameId", ui_value_text(frame_id)?)); }
        if let Some(page_id) = page_id { args.push(("pageId", ui_value_text(page_id)?)); }
        let mut row = semio_framework_plugin::tree_item_with_action(format!("{id}.choice.{index}"), ui_label(title)?, None, layout_action(command, Some(ui_value_map(args)?))?)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut row.component { props.selected = Some(value == current); }
        Ok(row)
    })
}

fn chunked_text(windows: &semio_framework_plugin::TreeWindows<'_>, location: &str, field: &str, content: &str, label: LabelText, stamp: &str) -> UiAssemblyResult<BuiltNode> {
    let ranges = text_chunks::ranges(content);
    let item = ui::tree_item(ui::Label(text(label.as_str())?)).try_id(location).map_err(|_| admit())?;
    semio_framework_plugin::tree_window_indexed_item(windows, item, location, true, ranges.len(), |index| {
        let range=ranges.get(index).ok_or_else(admit)?;
        document_input(&format!("{location}.span.{}.{}", range.start, range.end), &format!("{field}.chunk.{}.{}.{stamp}", range.start, range.end), label, content[range.bytes.clone()].into(), InputKind::LongText)
    })
}

fn chunked_bytes(windows: &semio_framework_plugin::TreeWindows<'_>, location: &str, field: &str, bytes: &[u8], label: LabelText, stamp: &str) -> UiAssemblyResult<BuiltNode> {
    let ranges = text_chunks::byte_ranges(bytes.len());
    let item = ui::tree_item(ui::Label(text(label.as_str())?)).try_id(location).map_err(|_| admit())?;
    semio_framework_plugin::tree_window_indexed_item(windows, item, location, true, ranges.len(), |index| {
        let range=ranges.get(index).ok_or_else(admit)?;
        let value = bytes[range.clone()].iter().map(|byte| format!("{byte:02x}")).collect();
        document_input(&format!("{location}.span.{}.{}", range.start, range.end), &format!("{field}.bytes.{}.{}.{stamp}", range.start, range.end), label, value, InputKind::Text)
    })
}

fn thread_select(windows: &semio_framework_plugin::TreeWindows<'_>, frame_id: &str, page_id: &str, thread_next: &Option<String>, document: &LayoutSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let choices = document.pages.iter().flat_map(|page| &page.frames).filter_map(|frame| match frame { Frame::Text { id, .. } if id != frame_id => Some((id.as_str(), id.as_str())), _ => None });
    choice_row(windows, "patchFrame", "threadNext", labels.thread_next, thread_next.as_deref().unwrap_or(""), std::iter::once(("", labels.thread_none.as_str())).chain(choices), Some(frame_id), Some(page_id))
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
    document_button_at(&format!("layout-play-inspector.patchDocument.{field}"), field, value, label)
}

fn document_button_at(location: &str, field: &str, value: &str, label: LabelText) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("field", ui_value_text(field)?), ("value", ui_value_text(value)?)])?;
    let (action, args) = layout_action("patchDocument", Some(args))?;
    let control = ui::button(ui::Label(text(label.as_str())?)).try_id(format!("{location}.input")).map_err(|_| admit())?.try_on_with(Trigger::Activate, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(location).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn document_toggle(field: &str, on: bool, label: LabelText) -> UiAssemblyResult<BuiltNode> {
    document_toggle_at(&format!("layout-play-inspector.patchDocument.{field}"), field, on, label)
}

fn document_toggle_at(location: &str, field: &str, on: bool, label: LabelText) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("field", ui_value_text(field)?)])?;
    let (action, args) = layout_action("patchDocument", Some(args))?;
    let control = ui::toggle(on).try_id(format!("{location}.input")).map_err(|_| admit())?.try_label(label.as_str()).map_err(|_| admit())?.try_on_with(Trigger::Change, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(location).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn parent_select(windows: &semio_framework_plugin::TreeWindows<'_>, page_id: &str, parent_page_id: &Option<String>, document: &LayoutSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    choice_row(windows, "patchPage", "parentPageId", labels.parent_page, parent_page_id.as_deref().unwrap_or(""), std::iter::once(("", labels.thread_none.as_str())).chain(document.parent_pages.iter().map(|parent| (parent.id.as_str(), parent.name.as_str()))), None, Some(page_id))
}

fn parent_frame_fields(windows: &semio_framework_plugin::TreeWindows<'_>, document: &LayoutSnapshot, page_id: &str, frame: &Frame, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let page = document.pages.iter().find(|page| page.id == page_id).ok_or_else(admit)?;
    let parent = page.parent_page_id.as_ref().and_then(|id| document.parent_pages.iter().find(|parent| parent.id == *id)).ok_or_else(admit)?;
    let mut fields = frame_fields(frame, document, labels);
    fields.retain(|field| !matches!(field.key.as_str(), "linkPath" | "linkWidth" | "linkHeight" | "dpi" | "colorProfile") && !field.key.starts_with("drawingText"));
    for field in &mut fields { field.key = format!("parent.{}", field.key); }
    let mut nodes: Vec<_> = fields.iter().map(|field| field_row("patchFrame", field, Some(frame.id()), Some(page_id))).collect::<UiAssemblyResult<_>>()?;
    nodes.push(choice_row(windows, "patchFrame", "parent.layerId", labels.group_layer, frame.layer_id(), parent.layers.iter().map(|layer| (layer.id.as_str(), layer.name.as_str())), Some(frame.id()), Some(page_id))?);
    if let Frame::Text { story_id, thread_next, wrap_mode, .. } = frame {
        nodes.push(select_row("patchFrame", "parent.wrapMode", labels.wrap_mode, wrap_mode, [("none", labels.wrap_none.as_str()), ("box", labels.wrap_box.as_str()), ("contour", labels.wrap_contour.as_str())], Some(frame.id()), Some(page_id))?);
        nodes.push(choice_row(windows, "patchFrame", "parent.storyId", labels.story, story_id, document.stories.iter().map(|story| (story.id.as_str(), story.id.as_str())), Some(frame.id()), Some(page_id))?);
        let choices = parent.frames.iter().chain(document.pages.iter().flat_map(|page| &page.frames)).filter_map(|target| match target { Frame::Text { id, .. } if id != frame.id() => Some((id.as_str(), id.as_str())), _ => None });
        nodes.push(choice_row(windows, "patchFrame", "parent.threadNext", labels.thread_next, thread_next.as_deref().unwrap_or(""), std::iter::once(("", labels.thread_none.as_str())).chain(choices), Some(frame.id()), Some(page_id))?);
    }
    nodes.push(frame_button(frame.id(), page_id, "parent.forward", labels.bring_forward)?);
    nodes.push(frame_button(frame.id(), page_id, "parent.backward", labels.send_backward)?);
    grouped("layout-play-inspector.parent-frame", labels.shared_parent_frame.as_str(), nodes)
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
    page_button_value(page_id, field, "true", label)
}

fn page_button_value(page_id: &str, field: &str, value: &str, label: LabelText) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("field", ui_value_text(field)?), ("pageId", ui_value_text(page_id)?), ("value", ui_value_text(value)?)])?;
    let (action, args) = layout_action("patchPage", Some(args))?;
    let control = ui::button(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchPage.{field}.input")).map_err(|_| admit())?.try_on_with(Trigger::Activate, action, args.ok_or_else(admit)?).map_err(|_| admit())?.try_build().map_err(|_| admit())?;
    ui::tree_item(ui::Label(text(label.as_str())?)).try_id(format!("layout-play-inspector.patchPage.{field}")).map_err(|_| admit())?.try_child(control).map_err(|_| admit())?.try_build().map_err(|_| admit())
}

fn rows(fields: &[Field], command: &str, frame_id: Option<&str>, page_id: Option<&str>) -> UiAssemblyResult<semio_framework_plugin::UiFixedList<BuiltNode>> {
    ui_node_list(fields.iter().map(|field| field_row(command, field, frame_id, page_id)))
}

/// 🕹️ Summary stays a read-only tree. Document, active page, and the selected frame are inputs bound to `patchDocument`, `patchPage`, and `patchFrame`.
pub fn render(document: &LayoutSnapshot, config: &LayoutWindowConfig, interaction: &LayoutInteractionSnapshot, labels: &LayoutLabels, windows: &semio_framework_plugin::TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {render_qualified(document,config,interaction,labels,windows,None)}

fn render_qualified(document:&LayoutSnapshot,config:&LayoutWindowConfig,interaction:&LayoutInteractionSnapshot,labels:&LayoutLabels,windows:&semio_framework_plugin::TreeWindows<'_>,drawing:Option<&semio_s_artifact_layout_layout_editor_commands::patch_page::drawing::source::OriginalPageDrawingSource>)->UiAssemblyResult<BuiltNode>{
    let dictionary_stamp = text_chunks::dictionary_stamp(document);
    let items = ui_node_list([
        tree_item_desc("layout-play-inspector.schema", ui_label(labels.schema.as_str())?, Some(LAYOUT_DOCUMENT_SCHEMA.into())),
        tree_item_desc("layout-play-inspector.name", ui_label(labels.name.as_str())?, Some(document.name.clone())),
        tree_item_desc("layout-play-inspector.pages", ui_label(labels.pages.as_str())?, Some(document.pages.len().to_string())),
        tree_item_desc("layout-play-inspector.active-page", ui_label(labels.active_page.as_str())?, Some(config.active_page_id.clone())),
        tree_item_desc("layout-play-inspector.selected-count", ui_label(labels.selected.as_str())?, Some(interaction.ids.len().to_string())),
    ])?;
    let mut builder = PanelTreeBuilder::new("layout-play-inspector")?.section("layout-play-inspector.summary", Some(ui_label(labels.inspection.as_str())?), true, items)?;
    builder = builder.section("layout-play-inspector.document", Some(ui_label(labels.group_document.as_str())?), true, rows(&document_fields(document, labels), "patchDocument", None, None)?)?;
    builder = builder.section("layout-play-inspector.create", Some(ui_label(labels.create.as_str())?), true, ui_node_list([
        document_button("addStory", "true", labels.add_story),
        document_button("addDataField", "true", labels.add_data_member),
        document_button_at("layout-play-inspector.patchDocument.clearDataFields", &format!("clearDataFields.at.{dictionary_stamp}"), "true", labels.clear_data_fields),
        document_button("addParagraphStyle", labels.paragraph_style.as_str(), labels.add_paragraph_style),
        document_button("addCharacterStyle", labels.character_style.as_str(), labels.add_character_style),
        document_button("addParentPage", labels.parent_page.as_str(), labels.add_parent_page),
        document_button("addSpread", labels.spread.as_str(), labels.add_spread),
    ])?)?;
    builder = builder.window_section(windows, "layout-play-inspector.stories", Some(ui_label(labels.stories.as_str())?), true, &document.stories, |story| {
        let id = format!("story.{}", story.id);
        let mut nodes = vec![chunked_text(windows, &format!("layout-play-inspector.global.story.{}.content", story.id), &format!("{id}.content"), &story.content, labels.story, &text_chunks::fingerprint(story.content.as_bytes()))?];
        let referenced = document.pages.iter().flat_map(|page| &page.frames).chain(document.parent_pages.iter().flat_map(|page| &page.frames)).any(|frame| matches!(frame, Frame::Text { story_id, .. } if story_id == &story.id));
        if !referenced { nodes.push(document_button(&format!("{id}.delete"), "true", labels.delete_story)?); }
        grouped(&format!("layout-play-inspector.{id}"), &story.id, nodes)
    })?;
    let data_rows = data_fields::rows(document);
    builder = builder.window_section(windows, "layout-play-inspector.data", Some(ui_label(labels.data_fields.as_str())?), true, &data_rows, |row| data_fields::render(windows, row, labels, &dictionary_stamp))?;
    builder = builder.window_section(windows, "layout-play-inspector.paragraph-style", Some(ui_label(labels.paragraph_style.as_str())?), true, &document.paragraph_styles, |style| paragraph_row(style, labels))?;
    builder = builder.window_section(windows, "layout-play-inspector.character-style", Some(ui_label(labels.character_style.as_str())?), true, &document.character_styles, |style| character_row(style, labels))?;
    builder = builder.window_section(windows, "layout-play-inspector.spreads", Some(ui_label(labels.spreads.as_str())?), true, &document.spreads, |spread| grouped(&format!("layout-play-inspector.spread.{}", spread.id), &spread.name, [
        paragraph_input(&spread.id, "name", labels.name, spread.name.clone(), InputKind::Text)?,
        document_button(&format!("{}.delete", spread.id), "true", labels.delete_spread)?,
    ]))?;
    builder = builder.window_section(windows, "layout-play-inspector.parents", Some(ui_label(labels.parent_pages.as_str())?), true, &document.parent_pages, |parent| grouped(&format!("layout-play-inspector.parent.{}", parent.id), &parent.name, [
        paragraph_input(&parent.id, "name", labels.name, parent.name.clone(), InputKind::Text)?,
        paragraph_input(&parent.id, "width", labels.width, number(parent.width), InputKind::Number)?,
        paragraph_input(&parent.id, "height", labels.height, number(parent.height), InputKind::Number)?,
        document_button(&format!("{}.capture", parent.id), &config.active_page_id, labels.capture_parent)?,
        document_button(&format!("{}.delete", parent.id), "true", labels.delete_parent_page)?,
    ]))?;
    if let Some((page_index,page)) = document.pages.iter().enumerate().find(|(_,page)| page.id == config.active_page_id) {
        let mut page_rows = rows(&page_fields(page, labels), "patchPage", None, Some(&page.id))?;
        page_rows.try_push(choice_row(windows, "patchPage", "spreadId", labels.spread, &page.spread_id, document.spreads.iter().map(|spread| (spread.id.as_str(), spread.name.as_str())), None, Some(&page.id))?).map_err(|_| admit())?;
        page_rows.try_push(parent_select(windows, &page.id, &page.parent_page_id, document, labels)?).map_err(|_| admit())?;
        if document.background_drawing.is_some(){let control=ui::button(ui::Label(text(labels.copy_document_drawing.as_str())?)).disabled(true).try_id("layout-play-inspector.patchPage.copyDocumentDrawing.input").map_err(|_|admit())?.try_build().map_err(|_|admit())?;let row=ui::tree_item(ui::Label(text(labels.copy_document_drawing.as_str())?)).try_id("layout-play-inspector.patchPage.copyDocumentDrawing").map_err(|_|admit())?.description(text(labels.drawing_copy_unavailable.as_str())?).try_child(control).map_err(|_|admit())?.try_build().map_err(|_|admit())?;page_rows.try_push(row).map_err(|_|admit())?;}
        if page.background_drawing.is_some(){page_rows.try_push(page_button(&page.id,"clearPageDrawing",labels.clear_page_drawing)?).map_err(|_|admit())?;}
        page_rows.try_push(page_button(&page.id, "addGuide", labels.add_guide)?).map_err(|_| admit())?;
        page_rows.try_push(page_button(&page.id, "moveEarlier", labels.move_earlier)?).map_err(|_| admit())?;
        page_rows.try_push(page_button(&page.id, "moveLater", labels.move_later)?).map_err(|_| admit())?;
        if document.pages.len() > 1 {
            page_rows.try_push(page_button(&page.id, "delete", labels.delete_page)?).map_err(|_| admit())?;
        }
        page_rows.try_push(page_button_value(&page.id, "addLayer", labels.group_layer.as_str(), labels.add_layer)?).map_err(|_| admit())?;
        builder = builder.section("layout-play-inspector.page", Some(ui_label(labels.group_page.as_str())?), true, page_rows)?;
        if page.background_drawing.is_some(){
            let canvas=drawing.map(|drawing|drawing.canvas_at(document,semio_s_artifact_layout_layout_model::DrawingSourceAddress::Page(page_index))).transpose().map_err(|_|admit())?.flatten();
            let mut controls=Vec::new();
            for(key,label,value)in[("width",labels.width,canvas.map(|canvas|canvas.width)),("height",labels.height,canvas.map(|canvas|canvas.height))]{
                let id=format!("layout-play-inspector.page-drawing.{key}");
                if let Some(value)=value{controls.push(field_row("patchPage",&Field{key:if key=="width"{"pageDrawingWidth".into()}else{"pageDrawingHeight".into()},label,value:number(value),kind:InputKind::Number},None,Some(&page.id))?);continue}
                let control=ui::input(InputKind::Number).value(text("")?).disabled(true).try_id(format!("{id}.input")).map_err(|_|admit())?.try_label(label.as_str()).map_err(|_|admit())?.try_build().map_err(|_|admit())?;
                controls.push(ui::tree_item(ui::Label(text(label.as_str())?)).try_id(&id).map_err(|_|admit())?.description(text(labels.drawing_canvas_unavailable.as_str())?).try_child(control).map_err(|_|admit())?.try_build().map_err(|_|admit())?);
            }
            builder=builder.section("layout-play-inspector.page-drawing",Some(ui_label(labels.page_drawing.as_str())?),true,ui_node_list(controls.into_iter().map(Ok))?)?;
        }
        let guides: Vec<_> = page.guides.iter().enumerate().collect();
        builder = builder.window_section(windows, "layout-play-inspector.guides", Some(ui_label(labels.guide.as_str())?), true, &guides, |(index, guide)| grouped(&format!("layout-play-inspector.guide.{index}"), &format!("{} {}", labels.guide.as_str(), index + 1), [
            guide_input(&page.id, *index, "x", labels.x, guide.x)?,
            guide_input(&page.id, *index, "y", labels.y, guide.y)?,
            guide_input(&page.id, *index, "width", labels.width, guide.width)?,
            guide_input(&page.id, *index, "height", labels.height, guide.height)?,
            page_button(&page.id, &format!("guide.{index}.delete"), labels.delete_guide)?,
        ]))?;
        builder = builder.window_section(windows, "layout-play-inspector.layers", Some(ui_label(labels.layers.as_str())?), true, &page.layers, |layer| grouped(&format!("layout-play-inspector.layer.{}", layer.id), &layer.name, [
            layer_name(&page.id, &layer.id, &layer.name, labels)?,
            layer_flag(&page.id, &layer.id, "visible", layer.visible, labels.visible)?,
            layer_flag(&page.id, &layer.id, "locked", layer.locked, labels.locked)?,
            page_button(&page.id, &format!("{}.moveEarlier", layer.id), labels.move_earlier)?,
            page_button(&page.id, &format!("{}.moveLater", layer.id), labels.move_later)?,
            page_button(&page.id, &format!("{}.delete", layer.id), labels.delete_layer)?,
        ]))?;
    }
    let Some(frame_id) = interaction.ids.first() else { return builder.build() };
    let Some((page_id, source)) = locate_frame(document, frame_id, &config.active_page_id) else {
        let missing = ui_node_list([tree_item_desc("layout-play-inspector.missing", ui_label(labels.selection_not_found.as_str())?, None)])?;
        return builder.section("layout-play-inspector.frame", Some(ui_label(labels.group_frame.as_str())?), true, missing)?.build();
    };
    let frame = selection_frame(document, page_id, source);
    let local = document.pages.iter().any(|page| page.id == page_id && page.frames.iter().any(|item| item.id() == frame.id()));
    let mut fields = frame_fields(&frame, document, labels);
    if !local { fields.retain(|field| matches!(field.key.as_str(), "x" | "y" | "width" | "height" | "rotation" | "locked" | "visible" | "linkPath" | "linkWidth" | "linkHeight" | "dpi" | "colorProfile") || field.key.starts_with("drawingText")); }
    let mut frame_rows = rows(&fields, "patchFrame", Some(frame_id), Some(page_id))?;
    if !local {
        frame_rows.try_push(tree_item_desc("layout-play-inspector.parent-frame.scope", ui_label(labels.parent_frame_scope.as_str())?, None)?).map_err(|_| admit())?;
        frame_rows.try_push(parent_frame_fields(windows, document, page_id, source, labels)?).map_err(|_| admit())?;
    }
    if local {
        frame_rows.try_push(layer_select(windows, frame_id, page_id, frame.layer_id(), document, labels)?).map_err(|_| admit())?;
        frame_rows.try_push(frame_button(frame_id, page_id, "forward", labels.bring_forward)?).map_err(|_| admit())?;
        frame_rows.try_push(frame_button(frame_id, page_id, "backward", labels.send_backward)?).map_err(|_| admit())?;
    }
    frame_rows.try_push(tree_item_desc("layout-play-inspector.frame.kind", ui_label(labels.kind.as_str())?, Some(frame_kind_label(&frame, labels).to_string()))?).map_err(|_| admit())?;
    if let Frame::Text { wrap_mode, story_id, thread_next, .. } = &frame {
        if let Some(story) = document.stories.iter().find(|story| story.id == *story_id) { frame_rows.try_push(chunked_text(windows, &format!("layout-play-inspector.selected.story.{}.content", story.id), &format!("story.{}.content", story.id), &story.content, labels.story, &text_chunks::fingerprint(story.content.as_bytes()))?).map_err(|_| admit())?; }
        if local {
            frame_rows.try_push(wrap_row(frame_id, page_id, wrap_mode, labels)?).map_err(|_| admit())?;
            frame_rows.try_push(story_select(windows, frame_id, page_id, story_id, document, labels)?).map_err(|_| admit())?;
        }
        frame_rows.try_push(frame_button(frame_id, page_id, "addStyleRun", labels.add_style_run)?).map_err(|_| admit())?;
        frame_rows.try_push(frame_button(frame_id, page_id, "clearStyleRuns", labels.clear_style_runs)?).map_err(|_| admit())?;
        if local { frame_rows.try_push(thread_select(windows, frame_id, page_id, thread_next, document, labels)?).map_err(|_| admit())?; }
    }
    if let Frame::Image { link_id, .. } = &frame {
        if let Some(link) = document.links.iter().find(|link| link.id == *link_id) {
            if !link.artifact_kind.is_empty() {
                frame_rows.try_push(tree_item_desc("layout-play-inspector.frame.artifact", ui_label(labels.artifact.as_str())?, Some(format!("{} {}", link.artifact_kind, link.artifact_ref)))?).map_err(|_| admit())?;
            }
            if semio_s_artifact_layout_layout_editor_commands::native_open(&link.artifact_kind).is_some() && !link.artifact_ref.trim().is_empty() {
                frame_rows.try_push(frame_button(frame_id, page_id, "open", labels.open_source)?).map_err(|_| admit())?;
            }
        }
    }
    builder = builder.section("layout-play-inspector.frame", Some(ui_label(labels.group_frame.as_str())?), true, frame_rows)?;
    if let Frame::Text { story_id, .. } = &frame {
        if let Some(story) = document.stories.iter().find(|story| story.id == *story_id) {
            let runs: Vec<_> = story.style_runs.iter().enumerate().collect();
            builder = builder.window_section(windows, "layout-play-inspector.runs", Some(ui_label(labels.style_run.as_str())?), true, &runs, |(index, run)| style_run(windows, frame_id, page_id, *index, run, &story.content, document, labels))?;
        }
    }
    builder.build()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️semantic-contract/🦀️.rs"]
mod semantic_contract;

#[cfg(test)]#[path="🧪️tests/📄️drawing/🦀️.rs"]mod page_drawing_control_tests;

#[path="🖼️drawing-work/🦀️.rs"]pub mod drawing_work;
