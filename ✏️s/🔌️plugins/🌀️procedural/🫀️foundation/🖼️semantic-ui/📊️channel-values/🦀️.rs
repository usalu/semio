//! 📊️ Windowed readout of evaluated graph channels using the framework's tree and graph targets.
use super::*;
use semio_framework_pack_json::Value;

#[path = "text-pages.rs"]
mod text_pages;
use text_pages::TextPages;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🔌️ One evaluated direction of a graph channel.
pub struct ChannelValueRow<'a> {
    pub id: String,
    pub label: String,
    pub input: bool,
    pub value: &'a Value,
}

impl ChannelValueRow<'_> {
    pub fn summary(&self) -> String {
        let arrow = if self.input { "←" } else { "→" };
        format!("{arrow} {}", channel_value_text(self.value, 2))
    }

}

/// 🏷️ Labels for the channel tree and its two evaluated directions.
pub struct ChannelValueLabels<'a> {
    values: &'static str,
    input: &'a str,
    output: &'a str,
    key: &'static str,
    value: &'static str,
    image: &'static str,
    image_unavailable: &'static str,
}

impl<'a> ChannelValueLabels<'a> {
    /// 🌍️ Uses the same locale as the surrounding preview window.
    pub fn new(is_de: bool, input: &'a str, output: &'a str) -> Self {
        let (values, key, value, image, image_unavailable) = if is_de {
            ("Werte", "Schlüssel", "Wert", "Bildvorschau", "Bildvorschau nicht verfügbar")
        } else {
            ("Values", "Key", "Value", "Image preview", "Image preview unavailable")
        };
        Self { values, input, output, key, value, image, image_unavailable }
    }
}

fn graph_value_row(factory: &ActionFactory, id: &str, label: &str, value: &str, target_id: &str, highlighted: bool) -> UiAssemblyResult<TreeItemBuilder> {
    let (hover, args) = factory.action(semio_plugin::INTERACTION_HOVER_ACTION_ID, Some(graph_hover_args(target_id, "handle")?))?;
    ui_id(tree_item_builder(Label(UiText::clipped(label))), id)?
        .description(UiText::clipped(value))
        .highlighted(highlighted)
        .try_on_with(Trigger::HoverPreview, hover, args.ok_or_else(|| ui_assembly_error("ui.value-row.args"))?)
        .map_err(|_| ui_assembly_error("ui.value-row.hover"))
}

struct ChannelValueNodes<'a, 'view> {
    windows: &'a TreeWindows<'view>,
    factory: &'a ActionFactory,
    labels: &'a ChannelValueLabels<'a>,
    target_id: &'a str,
    target_label: &'a str,
    highlighted: bool,
}

impl ChannelValueNodes<'_, '_> {
    fn row(&self, id: &str, label: &str, description: &str) -> UiAssemblyResult<TreeItemBuilder> {
        graph_value_row(self.factory, id, label, description, self.target_id, self.highlighted)
    }

    fn value(&self, id: &str, label: &str, value: &Value) -> UiAssemblyResult<BuiltNode> {
        let row = self.row(id, label, &channel_value_text(value, 0))?;
        match value {
            Value::Array(items) if !items.is_empty() => semio_plugin::tree_window_indexed_item(self.windows, row, id, false, items.len(), |index| {
                let child = (index + 1).to_string();
                self.value(&child, &child, &items[index])
            }),
            Value::Object(fields) if !fields.is_empty() => {
                let image_source = channel_image_source(value);
                let preview_rows = usize::from(image_source.is_some());
                semio_plugin::tree_window_indexed_item(self.windows, row, id, false, fields.len() + preview_rows, |index| {
                    if let Some(source) = image_source.filter(|_| index == 0) {
                        return self.image(source);
                    }
                    let member = index - preview_rows;
                    let (key, item) = fields.iter().nth(member).expect("window bounds match field count");
                    self.member(&(member + 1).to_string(), key, item)
                })
            }
            Value::String(text) if !text.is_empty() => self.text(row, id, text),
            _ => ui_build(row),
        }
    }

    fn member(&self, id: &str, key: &str, value: &Value) -> UiAssemblyResult<BuiltNode> {
        if !key_requires_detail(key) {
            return self.value(id, key, value);
        }
        let preview = quoted_channel_text(key);
        let row = self.row(id, &preview, &channel_value_text(value, 0))?;
        semio_plugin::tree_window_indexed_item(self.windows, row, id, false, 2, |index| {
            if index == 0 {
                let row = self.row("key", self.labels.key, &preview)?;
                if key.is_empty() { ui_build(row) } else { self.text(row, "key", key) }
            } else {
                self.value("value", self.labels.value, value)
            }
        })
    }

    fn image(&self, source: &str) -> UiAssemblyResult<BuiltNode> {
        let id = "image-preview";
        let alt = Label(UiText::clipped(&format!("{} · {}", self.labels.image, self.target_label)));
        match self.windows.retained_image(id, source, alt)? {
            Some(image) => ui_build(ui_child(self.row(id, self.labels.image, "")?.default_open(true), image)?),
            None => ui_build(self.row(id, self.labels.image, self.labels.image_unavailable)?),
        }
    }

    fn text(&self, row: TreeItemBuilder, id: &str, text: &str) -> UiAssemblyResult<BuiltNode> {
        windowed_read_only_text(self.windows, row, id, text, |page| self.row(page, page, ""))
    }
}

/// 📑️ Opens one exact UTF-8 text page at a time under a retained tree row.
pub fn windowed_read_only_text(
    windows: &TreeWindows<'_>,
    row: TreeItemBuilder,
    id: &str,
    text: &str,
    mut page_row: impl FnMut(&str) -> UiAssemblyResult<TreeItemBuilder>,
) -> UiAssemblyResult<BuiltNode> {
    let pages = TextPages(text);
    let page_count = pages.len();
    let surface = |index| {
        let page = pages.get(index).ok_or_else(|| ui_assembly_error("ui.value-text.page"))?;
        TextWindowKit::render_read_only("content", &TextView { text: page.to_owned(), language: None })
    };
    semio_plugin::tree_window_indexed_item(windows, row, id, false, page_count, |index| {
        if page_count == 1 {
            return surface(index);
        }
        let page = (index + 1).to_string();
        let row = page_row(&page)?;
        semio_plugin::tree_window_indexed_item(windows, row, &page, false, 1, |_| surface(index))
    })
}

/// 🖼️ Reads the canonical Flow image dictionary serialized by the channel evaluator.
fn channel_image_source(value: &Value) -> Option<&str> {
    if value.get("schema").and_then(Value::as_str) != Some("image") {
        return None;
    }
    let source = value.get("dataUrl")?.as_str()?;
    let (media, content) = source.split_once(',')?;
    let subtype = media.strip_prefix("data:image/")?.split(';').next()?;
    (!subtype.is_empty() && !content.is_empty()).then_some(source)
}

const CHANNEL_SUMMARY_LIMIT: usize = 80;

fn key_requires_detail(key: &str) -> bool {
    key.len() > CHANNEL_SUMMARY_LIMIT || key.is_empty() || key.trim() != key || key.contains("  ")
        || key.chars().any(|character| !character.is_alphanumeric() && !matches!(character, '_' | '-' | '.' | ':' | '$' | ' '))
}

fn quoted_channel_text(text: &str) -> String {
    let mut chars = text.chars();
    let mut sample: String = chars.by_ref().take(CHANNEL_SUMMARY_LIMIT).collect();
    if chars.next().is_some() { sample.push('…'); }
    semio_framework_pack_json::to_string(&Value::String(sample))
}

fn channel_value_tree(windows: &TreeWindows<'_>, controller: &'static str, namespace: &str, labels: &ChannelValueLabels<'_>, rows: &[ChannelValueRow<'_>], hovered_ports: &[String]) -> UiAssemblyResult<BuiltNode> {
    let factory = ActionFactory::new(controller);
    let hovered: std::collections::BTreeSet<&str> = hovered_ports.iter().map(String::as_str).collect();
    semio_plugin::PanelTreeBuilder::new(namespace)?
        .window_section(windows, "values", Some(ui_label(labels.values)?), true, rows, |channel| {
            let highlighted = hovered.contains(channel.id.as_str());
            let row = graph_value_row(&factory, &channel.id, &channel.label, &channel.summary(), &channel.id, highlighted)?
                .granularity(ui_text("handle")?);
            let expandable = match channel.value {
                Value::Object(fields) => !fields.is_empty(),
                Value::Array(items) => !items.is_empty(),
                Value::String(text) => !text.is_empty(),
                _ => false,
            };
            if !expandable {
                return ui_build(row);
            }
            let nodes = ChannelValueNodes { windows, factory: &factory, labels, target_id: &channel.id, target_label: &channel.label, highlighted };
            let (id, label) = if channel.input { ("in", labels.input) } else { ("out", labels.output) };
            semio_plugin::tree_window_indexed_item(windows, row, &channel.id, false, 1, |_| nodes.value(id, label, channel.value))
        })?
        .interaction_domain(controller, "graph")?
        .build()
}

/// 🪟️ Places the evaluated channel tree beside the scene.
pub fn scene_with_channel_values(scene: BuiltNode, windows: &TreeWindows<'_>, controller: &'static str, namespace: &str, labels: &ChannelValueLabels<'_>, rows: &[ChannelValueRow<'_>], hovered_ports: &[String]) -> UiAssemblyResult<BuiltNode> {
    if rows.is_empty() { return Ok(scene); }
    let mut tree = channel_value_tree(windows, controller, namespace, labels, rows, hovered_ports)?;
    tree.layout = LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::Vertical, sizing: Sizing::Fill, ..Default::default() });
    let scene = ui_build(ui_child(ui_id(column().grow(true), "preview.scene")?, scene)?)?;
    let tree = ui_build(ui_child(ui_id(column().grow(true), "preview.channels")?, tree)?)?;
    ui_build(ui_child(ui_child(column().grow(true), scene)?, tree)?)
}

/// 🧾️ Bounded display of a channel value, including structured and empty values.
fn channel_value_text(value: &semio_framework_pack_json::Value, depth: usize) -> String {
    use semio_framework_pack_json::Value;
    const ITEMS: usize = 4;
    match value {
        Value::String(text) => quoted_channel_text(text),
        Value::Array(items) => {
            let mut parts = Vec::new();
            if depth > 0 {
                parts.extend(items.iter().take(ITEMS).map(|v| channel_value_text(v, depth - 1)));
            }
            if items.len() > parts.len() { parts.push("…".into()); }
            format!("[{}]", parts.join(", "))
        }
        Value::Object(items) => {
            let mut parts = Vec::new();
            if depth > 0 {
                parts.extend(items.iter().take(ITEMS).map(|(key, v)| format!("{}: {}", quoted_channel_text(key), channel_value_text(v, depth - 1))));
            }
            if items.len() > parts.len() { parts.push("…".into()); }
            format!("{{{}}}", parts.join(", "))
        }
        _ => semio_framework_pack_json::to_string(value),
    }
}

