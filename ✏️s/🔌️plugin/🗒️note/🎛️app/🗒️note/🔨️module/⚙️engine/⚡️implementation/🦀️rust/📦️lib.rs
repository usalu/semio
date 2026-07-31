//! ⚙️ Note app — headless compute (constitutional: engine).

use note::{NoteBlockNode, NoteDocument, NoteImageAsset, NoteTableCell, NoteTextParagraph, NoteTextRun, NOTE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{DwgDrawing, DwgGeometry};
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};

//#region 🔖️Constants
/// 📄️ The `semio` example document, handcrafted in the `.note` DSL — {@link semio_example_document}/
/// {@link semio_example_json} are the only ways it should be consumed.
const SEMIO_NOTE_EXAMPLE_TEXT: &str = note_dsl::SEMIO_NOTE_EXAMPLE_TEXT;

static NOTE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
pub fn create_note_id(prefix: &str) -> String {
    let next = NOTE_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

/// 📄️ The `semio` example, parsed once from {@link SEMIO_NOTE_EXAMPLE_TEXT} — the source of truth for
/// every "semio" example call site (`setActiveExample`, tests). Falls back to the empty document if the
/// fixture ever fails to parse, matching the old JSON fixture's failure behavior.
pub fn semio_example_document() -> NoteDocument {
    <NoteDocument as store::DocumentDsl>::parse_dsl(SEMIO_NOTE_EXAMPLE_TEXT).unwrap_or_else(|_| empty_note_document())
}

/// 📄️ JSON re-serialization of {@link semio_example_document}, for the framework-generic call sites that
/// contractually require JSON text (`PluginApp::render`'s `projection_override_json`, `App::example`'s
/// manifest `document_json`).
pub fn semio_example_json() -> String {
    serde_json::to_string(&semio_example_document()).expect("serialize semio example document")
}

pub fn empty_note_document() -> NoteDocument {
    NoteDocument {
        schema: NOTE_DOCUMENT_SCHEMA.into(),
        id: "empty".into(),
        title: None,
        blocks: Vec::new(),
        grid_visible: Some(true),
        grid_spacing: Some(32.0),
        grid_subdivisions: Some(4.0),
        grid_opacity: Some(0.35),
        snap_enabled: Some(false),
        snap_grid_spacing: Some(8.0),
        pencil_width: Some(3.0),
        eraser_radius: Some(12.0),
        assets: BTreeMap::new(),
    }
}

pub fn block_id(block: &NoteBlockNode) -> &str {
    match block {
        NoteBlockNode::Text { id, .. }
        | NoteBlockNode::Image { id, .. }
        | NoteBlockNode::Table { id, .. }
        | NoteBlockNode::Math { id, .. }
        | NoteBlockNode::Ink { id, .. }
        | NoteBlockNode::Group { id, .. } => id,
    }
}

pub fn block_name(block: &NoteBlockNode) -> &str {
    match block {
        NoteBlockNode::Text { name, .. }
        | NoteBlockNode::Image { name, .. }
        | NoteBlockNode::Table { name, .. }
        | NoteBlockNode::Math { name, .. }
        | NoteBlockNode::Ink { name, .. }
        | NoteBlockNode::Group { name, .. } => name,
    }
}

pub fn block_kind(block: &NoteBlockNode) -> &str {
    match block {
        NoteBlockNode::Text { .. } => "text",
        NoteBlockNode::Image { .. } => "image",
        NoteBlockNode::Table { .. } => "table",
        NoteBlockNode::Math { .. } => "math",
        NoteBlockNode::Ink { .. } => "stroke",
        NoteBlockNode::Group { .. } => "group",
    }
}

pub fn block_visible(block: &NoteBlockNode) -> bool {
    match block {
        NoteBlockNode::Text { visible, .. }
        | NoteBlockNode::Image { visible, .. }
        | NoteBlockNode::Table { visible, .. }
        | NoteBlockNode::Math { visible, .. }
        | NoteBlockNode::Ink { visible, .. }
        | NoteBlockNode::Group { visible, .. } => *visible,
    }
}

pub fn block_icon(kind: &str) -> &str {
    match kind {
        "text" => "type",
        "image" => "image",
        "table" => "table",
        "math" => "sigma",
        "stroke" => "pencil",
        _ => "folder",
    }
}

pub fn block_tree_row_id(block: &NoteBlockNode) -> String {
    format!("note-play-block:{}", block_id(block))
}

pub fn block_id_from_tree_row_id(row_id: &str) -> Option<String> {
    row_id.strip_prefix("note-play-block:").map(str::to_string)
}

pub fn find_block<'a>(blocks: &'a [NoteBlockNode], target_id: &str) -> Option<&'a NoteBlockNode> {
    for block in blocks {
        if block_id(block) == target_id {
            return Some(block);
        }
        if let NoteBlockNode::Group { children, .. } = block {
            if let Some(found) = find_block(children, target_id) {
                return Some(found);
            }
        }
    }
    None
}

pub fn flatten_blocks(blocks: &[NoteBlockNode]) -> Vec<&NoteBlockNode> {
    let mut out = Vec::new();
    fn visit<'a>(blocks: &'a [NoteBlockNode], out: &mut Vec<&'a NoteBlockNode>) {
        for block in blocks {
            out.push(block);
            if let NoteBlockNode::Group { children, .. } = block {
                visit(children, out);
            }
        }
    }
    visit(blocks, &mut out);
    out
}

pub fn create_block_by_kind(kind: &str, x: f64, y: f64) -> NoteBlockNode {
    let id = create_note_id(kind);
    match kind {
        "image" => NoteBlockNode::Image {
            id,
            name: "Image".into(),
            x,
            y,
            width: 240.0,
            height: 160.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            image_key: "placeholder".into(),
        },
        "table" => NoteBlockNode::Table {
            id,
            name: "Table".into(),
            x,
            y,
            width: 320.0,
            height: 160.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            columns: vec!["A".into(), "B".into(), "C".into()],
            rows: vec![
                vec![
                    NoteTableCell { content: String::new() },
                    NoteTableCell { content: String::new() },
                    NoteTableCell { content: String::new() },
                ],
                vec![
                    NoteTableCell { content: String::new() },
                    NoteTableCell { content: String::new() },
                    NoteTableCell { content: String::new() },
                ],
            ],
        },
        "math" => NoteBlockNode::Math {
            id,
            name: "Math".into(),
            x,
            y,
            width: 200.0,
            height: 80.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            tex: "E = mc^2".into(),
            display_mode: true,
        },
        "stroke" => NoteBlockNode::Ink {
            id,
            name: "Ink".into(),
            x,
            y,
            width: 1.0,
            height: 1.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            points: Vec::new(),
            stroke_width: 3.0,
            color: [0.0, 0.0, 0.0, 1.0],
        },
        "group" => NoteBlockNode::Group {
            id,
            name: "Group".into(),
            x,
            y,
            width: 280.0,
            height: 120.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            children: Vec::new(),
        },
        _ => NoteBlockNode::Text {
            id,
            name: "Text".into(),
            x,
            y,
            width: 280.0,
            height: 120.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            paragraphs: vec![NoteTextParagraph {
                runs: vec![NoteTextRun {
                    text: String::new(),
                    bold: None,
                    italic: None,
                    underline: None,
                    link: None,
                }],
            }],
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        },
    }
}

pub fn remove_block_from_tree(blocks: &mut Vec<NoteBlockNode>, target_id: &str) -> bool {
    if let Some(index) = blocks.iter().position(|block| block_id(block) == target_id) {
        blocks.remove(index);
        return true;
    }
    for block in blocks.iter_mut() {
        if let NoteBlockNode::Group { children, .. } = block {
            if remove_block_from_tree(children, target_id) {
                return true;
            }
        }
    }
    false
}

pub fn reid_block_tree(block: &mut NoteBlockNode, rename_top: bool) {
    let kind = block_kind(block).to_string();
    match block {
        NoteBlockNode::Text { id, name, .. }
        | NoteBlockNode::Image { id, name, .. }
        | NoteBlockNode::Table { id, name, .. }
        | NoteBlockNode::Math { id, name, .. }
        | NoteBlockNode::Ink { id, name, .. }
        | NoteBlockNode::Group { id, name, .. } => {
            *id = create_note_id(&kind);
            if rename_top {
                *name = format!("{name} copy");
            }
        }
    }
    if let NoteBlockNode::Group { children, .. } = block {
        for child in children.iter_mut() {
            reid_block_tree(child, false);
        }
    }
}

pub fn clone_block(block: &NoteBlockNode) -> NoteBlockNode {
    let mut cloned: NoteBlockNode = serde_json::from_value(serde_json::to_value(block).unwrap()).unwrap();
    reid_block_tree(&mut cloned, true);
    cloned
}

pub fn offset_block_tree(block: &mut NoteBlockNode, dx: f64, dy: f64) {
    match block {
        NoteBlockNode::Text { x, y, .. }
        | NoteBlockNode::Image { x, y, .. }
        | NoteBlockNode::Table { x, y, .. }
        | NoteBlockNode::Math { x, y, .. }
        | NoteBlockNode::Ink { x, y, .. }
        | NoteBlockNode::Group { x, y, .. } => {
            *x += dx;
            *y += dy;
        }
    }
    if let NoteBlockNode::Group { children, .. } = block {
        for child in children.iter_mut() {
            offset_block_tree(child, dx, dy);
        }
    }
}

pub fn insert_after(blocks: &mut Vec<NoteBlockNode>, target_id: &str, block: NoteBlockNode) -> bool {
    if let Some(index) = blocks.iter().position(|entry| block_id(entry) == target_id) {
        blocks.insert(index + 1, block);
        return true;
    }
    for entry in blocks.iter_mut() {
        if let NoteBlockNode::Group { children, .. } = entry {
            if insert_after(children, target_id, block.clone()) {
                return true;
            }
        }
    }
    false
}

pub fn insert_block(blocks: &mut Vec<NoteBlockNode>, parent_id: Option<&str>, index: usize, block: NoteBlockNode) {
    if let Some(parent_id) = parent_id {
        for node in blocks.iter_mut() {
            if let NoteBlockNode::Group { id, children, .. } = node {
                if id == parent_id {
                    let index = index.min(children.len());
                    children.insert(index, block);
                    return;
                }
                insert_block(children, Some(parent_id), index, block.clone());
            }
        }
        return;
    }
    let index = index.min(blocks.len());
    blocks.insert(index, block);
}

pub fn update_block_in_tree(blocks: &mut [NoteBlockNode], target_id: &str, next_block: NoteBlockNode) -> bool {
    for block in blocks.iter_mut() {
        if block_id(block) == target_id {
            *block = next_block;
            return true;
        }
        if let NoteBlockNode::Group { children, .. } = block {
            if update_block_in_tree(children, target_id, next_block.clone()) {
                return true;
            }
        }
    }
    false
}

pub fn mutate_block_in_tree(
    blocks: &mut [NoteBlockNode],
    target_id: &str,
    mutator: &mut impl FnMut(&NoteBlockNode) -> NoteBlockNode,
) -> bool {
    for block in blocks.iter_mut() {
        if block_id(block) == target_id {
            *block = mutator(block);
            return true;
        }
        if let NoteBlockNode::Group { children, .. } = block {
            if mutate_block_in_tree(children, target_id, mutator) {
                return true;
            }
        }
    }
    false
}

pub fn block_bounds(block: &NoteBlockNode) -> (f64, f64, f64, f64) {
    match block {
        NoteBlockNode::Text { x, y, width, height, .. }
        | NoteBlockNode::Image { x, y, width, height, .. }
        | NoteBlockNode::Table { x, y, width, height, .. }
        | NoteBlockNode::Math { x, y, width, height, .. }
        | NoteBlockNode::Ink { x, y, width, height, .. }
        | NoteBlockNode::Group { x, y, width, height, .. } => (*x, *y, *width, *height),
    }
}

pub fn patch_block_field(document: &NoteDocument, block_id: &str, field: &str, value: &Value) -> NoteDocument {
    let Some(block) = find_block(&document.blocks, block_id).cloned() else {
        return document.clone();
    };
    let mut next = document.clone();
    match field {
        "name" => {
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { name, .. }
                    | NoteBlockNode::Image { name, .. }
                    | NoteBlockNode::Table { name, .. }
                    | NoteBlockNode::Math { name, .. }
                    | NoteBlockNode::Ink { name, .. }
                    | NoteBlockNode::Group { name, .. } => *name = value.as_str().unwrap_or("").into(),
                }
                cloned
            });
        }
        "visible" => {
            let pressed = value.as_bool().unwrap_or(true);
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { visible, .. }
                    | NoteBlockNode::Image { visible, .. }
                    | NoteBlockNode::Table { visible, .. }
                    | NoteBlockNode::Math { visible, .. }
                    | NoteBlockNode::Ink { visible, .. }
                    | NoteBlockNode::Group { visible, .. } => *visible = pressed,
                }
                cloned
            });
        }
        "locked" => {
            let pressed = value.as_bool().unwrap_or(false);
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { locked, .. }
                    | NoteBlockNode::Image { locked, .. }
                    | NoteBlockNode::Table { locked, .. }
                    | NoteBlockNode::Math { locked, .. }
                    | NoteBlockNode::Ink { locked, .. }
                    | NoteBlockNode::Group { locked, .. } => *locked = pressed,
                }
                cloned
            });
        }
        "x" | "y" | "width" | "height" => {
            let number = value.as_f64().unwrap_or(0.0);
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Image { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Table { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Math { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Ink { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Group { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                }
                cloned
            });
        }
        "textContent" => {
            if let NoteBlockNode::Text { .. } = block {
                let text = value.as_str().unwrap_or("");
                let paragraphs = vec![NoteTextParagraph {
                    runs: vec![NoteTextRun {
                        text: text.into(),
                        bold: None,
                        italic: None,
                        underline: None,
                        link: None,
                    }],
                }];
                let mut updated = block;
                if let NoteBlockNode::Text { paragraphs: p, .. } = &mut updated {
                    *p = paragraphs;
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "textSize" => {
            if let NoteBlockNode::Text { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Text { font_size, .. } = &mut updated {
                    *font_size = value.as_f64().unwrap_or(18.0);
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "mathTex" => {
            if let NoteBlockNode::Math { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Math { tex, .. } = &mut updated {
                    *tex = value.as_str().unwrap_or("").into();
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "inkWidth" => {
            if let NoteBlockNode::Ink { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Ink { stroke_width, .. } = &mut updated {
                    *stroke_width = value.as_f64().unwrap_or(3.0);
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "tableAddRow" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { columns, rows, .. } = &mut updated {
                    let width = columns.len();
                    rows.push((0..width).map(|_| NoteTableCell { content: String::new() }).collect());
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "tableRemoveRow" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { rows, .. } = &mut updated {
                    if rows.len() > 1 {
                        rows.pop();
                    }
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "tableAddColumn" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { columns, rows, .. } = &mut updated {
                    let next_letter = (b'A' + (columns.len() as u8 % 26)) as char;
                    columns.push(next_letter.to_string());
                    for row in rows.iter_mut() {
                        row.push(NoteTableCell { content: String::new() });
                    }
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "tableRemoveColumn" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { columns, rows, .. } = &mut updated {
                    if columns.len() > 1 {
                        columns.pop();
                        for row in rows.iter_mut() {
                            row.pop();
                        }
                    }
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        _ => {}
    }
    next
}

//#endregion 🔖️DocumentHelpers

//#region 🔖️MediaExport
fn escape_svg_text(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

pub fn note_document_bounds(document: &NoteDocument) -> (u32, u32) {
    let mut max_x = 1024.0_f64;
    let mut max_y = 1024.0_f64;
    for block in flatten_blocks(&document.blocks) {
        if !block_visible(block) {
            continue;
        }
        let (x, y, width, height) = match block {
            NoteBlockNode::Text { x, y, width, height, .. }
            | NoteBlockNode::Image { x, y, width, height, .. }
            | NoteBlockNode::Table { x, y, width, height, .. }
            | NoteBlockNode::Math { x, y, width, height, .. }
            | NoteBlockNode::Ink { x, y, width, height, .. }
            | NoteBlockNode::Group { x, y, width, height, .. } => (*x, *y, *width, *height),
        };
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
    }
    (max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32)
}

fn note_block_to_svg(block: &NoteBlockNode, document: &NoteDocument) -> String {
    let (x, y, rotation, width, height) = match block {
        NoteBlockNode::Text { x, y, rotation, width, height, .. }
        | NoteBlockNode::Image { x, y, rotation, width, height, .. }
        | NoteBlockNode::Table { x, y, rotation, width, height, .. }
        | NoteBlockNode::Math { x, y, rotation, width, height, .. }
        | NoteBlockNode::Ink { x, y, rotation, width, height, .. }
        | NoteBlockNode::Group { x, y, rotation, width, height, .. } => (*x, *y, *rotation, *width, *height),
    };
    let transform = format!("translate({x} {y}) rotate({rotation})");
    match block {
        NoteBlockNode::Text {
            paragraphs,
            font_size,
            font_weight,
            ..
        } => {
            let text = paragraphs
                .iter()
                .map(|paragraph| paragraph.runs.iter().map(|run| run.text.as_str()).collect::<Vec<_>>().join(""))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                r#"<g transform="{transform}"><text x="0" y="{font_size}" font-size="{font_size}" font-weight="{font_weight}" fill="black">{}</text></g>"#,
                escape_svg_text(&text)
            )
        }
        NoteBlockNode::Image { image_key, .. } => {
            if let Some(asset) = document.assets.get(image_key) {
                format!(
                    r#"<g transform="{transform}"><image href="{}" width="{width}" height="{height}"/></g>"#,
                    asset.data
                )
            } else {
                format!(
                    "<g transform=\"{transform}\"><rect width=\"{width}\" height=\"{height}\" fill=\"#ddd\" stroke=\"#888\"/></g>"
                )
            }
        }
        NoteBlockNode::Ink {
            points,
            stroke_width,
            color,
            ..
        } => {
            if points.len() < 2 {
                return String::new();
            }
            let mut d = format!("M {} {}", points[0][0], points[0][1]);
            for point in points.iter().skip(1) {
                d.push_str(&format!(" L {} {}", point[0], point[1]));
            }
            let stroke = format!(
                "rgba({},{},{},{})",
                (color[0] * 255.0).round() as u8,
                (color[1] * 255.0).round() as u8,
                (color[2] * 255.0).round() as u8,
                color[3]
            );
            format!(
                r#"<g transform="{transform}"><path d="{d}" fill="none" stroke="{stroke}" stroke-width="{stroke_width}" stroke-linecap="round" stroke-linejoin="round"/></g>"#
            )
        }
        _ => format!(
            "<g transform=\"{transform}\"><rect width=\"{width}\" height=\"{height}\" fill=\"none\" stroke=\"#888\"/></g>"
        ),
    }
}

pub fn note_document_to_svg(document: &NoteDocument) -> (String, u32, u32) {
    let (width, height) = note_document_bounds(document);
    let body = flatten_blocks(&document.blocks)
        .into_iter()
        .filter(|block| block_visible(block))
        .map(|block| note_block_to_svg(block, document))
        .collect::<Vec<_>>()
        .join("");
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">{body}</svg>"#
    );
    (svg, width, height)
}

pub fn note_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let document: NoteDocument = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    Ok(note_document_to_svg(&document))
}
//#endregion 🔖️MediaExport

//#region 🔖️MediaImport
fn ink_block_from_points(points: &[[f64; 2]]) -> NoteBlockNode {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for point in points {
        min_x = min_x.min(point[0]);
        min_y = min_y.min(point[1]);
        max_x = max_x.max(point[0]);
        max_y = max_y.max(point[1]);
    }
    let local_points = points.iter().map(|point| [point[0] - min_x, point[1] - min_y]).collect();
    NoteBlockNode::Ink {
        id: create_note_id("dwg-ink"),
        name: "Imported Stroke".into(),
        x: min_x,
        y: min_y,
        width: (max_x - min_x).max(1.0),
        height: (max_y - min_y).max(1.0),
        rotation: 0.0,
        visible: true,
        locked: false,
        points: local_points,
        stroke_width: 1.0,
        color: [0.0, 0.0, 0.0, 1.0],
    }
}

fn text_block_from_dwg(at: &[f64; 3], height: f64, rotation: f64, content: &str) -> NoteBlockNode {
    let font_size = if height > 0.0 { height } else { 12.0 };
    NoteBlockNode::Text {
        id: create_note_id("dwg-text"),
        name: "Imported Text".into(),
        x: at[0],
        y: at[1],
        width: (content.chars().count() as f64 * font_size * 0.6).max(font_size),
        height: font_size * 1.4,
        rotation,
        visible: true,
        locked: false,
        paragraphs: vec![NoteTextParagraph {
            runs: vec![NoteTextRun { text: content.to_string(), bold: None, italic: None, underline: None, link: None }],
        }],
        font_size,
        font_weight: "normal".into(),
        align: "left".into(),
    }
}

pub fn note_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
    let mut document = empty_note_document();
    document.id = create_note_id("dwg-import");
    document.title = Some("Imported Drawing".into());
    for entity in &drawing.entities {
        match &entity.geometry {
            DwgGeometry::Line { start, end } => {
                document.blocks.push(ink_block_from_points(&[[start[0], start[1]], [end[0], end[1]]]));
            }
            DwgGeometry::LwPolyline { closed, vertices, .. } => {
                if vertices.len() >= 2 {
                    let mut points = vertices.clone();
                    if *closed {
                        points.push(vertices[0]);
                    }
                    document.blocks.push(ink_block_from_points(&points));
                }
            }
            DwgGeometry::Text { at, height, rotation, content } => {
                document.blocks.push(text_block_from_dwg(at, *height, *rotation, content));
            }
            _ => {}
        }
    }
    serde_json::to_value(&document).map_err(|error| error.to_string())
}
//#endregion 🔖️MediaImport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{DwgColor, DwgEntity, DwgLayer};

    #[test]
    fn clone_block_reids_group_children() {
        let child = create_block_by_kind("text", 0.0, 0.0);
        let child_id = block_id(&child).to_string();
        let group = NoteBlockNode::Group {
            id: "group-1".into(),
            name: "Group".into(),
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            children: vec![child],
        };
        let cloned = clone_block(&group);
        if let NoteBlockNode::Group { children, .. } = &cloned {
            assert_ne!(block_id(&children[0]), child_id);
        } else {
            panic!("expected group block");
        }
    }

    #[test]
    fn imports_dwg_polyline_and_text_into_note_blocks() {
        let drawing = DwgDrawing {
            layers: vec![DwgLayer::default()],
            entities: vec![
                DwgEntity {
                    layer: 0,
                    color: DwgColor::ByLayer,
                    geometry: DwgGeometry::LwPolyline {
                        closed: true,
                        elevation: 0.0,
                        vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]],
                        bulges: vec![0.0, 0.5, 0.0],
                    },
                },
                DwgEntity {
                    layer: 0,
                    color: DwgColor::ByLayer,
                    geometry: DwgGeometry::Text { at: [1.0, 2.0, 0.0], height: 2.5, rotation: 0.0, content: "semio".into() },
                },
            ],
            extmin: [0.0, 0.0, 0.0],
            extmax: [10.0, 10.0, 0.0],
        };
        let value = note_document_json_from_dwg(&drawing).unwrap();
        let document: NoteDocument = serde_json::from_value(value).unwrap();
        assert_eq!(document.schema, NOTE_DOCUMENT_SCHEMA);
        assert_eq!(document.blocks.len(), 2);
        let ink_count = document.blocks.iter().filter(|block| matches!(block, NoteBlockNode::Ink { .. })).count();
        let text_count = document.blocks.iter().filter(|block| matches!(block, NoteBlockNode::Text { .. })).count();
        assert_eq!(ink_count, 1);
        assert_eq!(text_count, 1);
        if let Some(NoteBlockNode::Ink { points, .. }) = document.blocks.iter().find(|block| matches!(block, NoteBlockNode::Ink { .. })) {
            assert_eq!(points.len(), 4);
        } else {
            panic!("expected ink block");
        }
        if let Some(NoteBlockNode::Text { paragraphs, .. }) = document.blocks.iter().find(|block| matches!(block, NoteBlockNode::Text { .. })) {
            assert_eq!(paragraphs[0].runs[0].text, "semio");
        } else {
            panic!("expected text block");
        }
    }

    #[test]
    fn imports_empty_dwg_drawing_as_valid_empty_note_document() {
        let drawing = DwgDrawing::default();
        let value = note_document_json_from_dwg(&drawing).unwrap();
        let document: NoteDocument = serde_json::from_value(value).unwrap();
        assert_eq!(document.schema, NOTE_DOCUMENT_SCHEMA);
        assert!(document.blocks.is_empty());
    }
}
//#endregion 🧪️Tests
