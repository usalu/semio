//! 📄️ BIM sheet window: one authored sheet on a `Canvas2d` surface in paper millimetres, y downward, at actual size: the paper, the frame, every viewport (the `view-linework` of its view mapped through the `sheet-layout` of the sheet), the captions,
//! the title block and the revision table, the selection outlined in the accent colour and the findings of the layout (a window beyond the frame, over the title block or on another window) outlined in red. Nothing here computes geometry: a sheet
//! without a layout paints nothing, and the position, the scale and the crop of a viewport are authored in the viewport, never in this window.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use self::config::BimSheetWindowConfig;
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::{canvas_surface, meta_record, path, path_record, text_record, Paint, Rgba};
use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::io::export::sheets::{self, ink::{sheet_marks, Mark, TitleLabels}};
use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::{PaperRect, PlacedViewport, SheetLayout};
use crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
use crate::{ModelInference, ModelSnapshot};
use semio_framework_plugin::DslValue;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "bim-edit-sheet";
pub const BODY_KEY: &str = "bim.edit.sheet";
const SURFACE_ID: &str = "bim.edit.sheet2d/sheet";
/// 🖼️ The air around the paper when the window frames a sheet, in pixels.
const FRAMING_PADDING: f64 = 32.0;
const PAPER: Rgba = [1.0, 1.0, 1.0, 1.0];
const EDGE: Rgba = [0.55, 0.57, 0.62, 1.0];
const INK: Rgba = [0.05, 0.05, 0.08, 1.0];
const MUTED: Rgba = [0.27, 0.27, 0.3, 1.0];
const ACCENT: Rgba = [0.1, 0.45, 0.95, 1.0];
const WARNING: Rgba = [0.9, 0.2, 0.2, 1.0];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: crate::editor::bim::utilities::initial(),
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native(BimLabels::NATIVE_EN.window_sheet.as_str(), BimLabels::NATIVE_DE.window_sheet.as_str()),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "file-text".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: crate::editor::bim::utilities::for_window(WINDOW_KIND_ID),
        interactions: vec![InteractionRef::new(BIM_ELEMENT_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Sheet
/// 📄️ Every sheet of the model in print order: by number, then id.
pub fn sheets_in_order(snapshot: &ModelSnapshot) -> Vec<String> {
    sheets::ordered(snapshot)
}

/// 📄️ The sheet a sheet window shows: its configured one while it still exists, else the first one in print order; none when the model has no sheet.
pub fn active_sheet(snapshot: &ModelSnapshot, config: &BimSheetWindowConfig) -> Option<String> {
    if snapshot.sheets.contains_key(&config.sheet) {
        return Some(config.sheet.clone());
    }
    sheets_in_order(snapshot).into_iter().next()
}

/// 🗣️ The headings a sheet prints, in the language of `labels`.
pub fn title_labels(labels: &BimLabels) -> TitleLabels {
    let text = |value: semio_framework_ui_locale::LabelText| value.as_str().to_string();
    TitleLabels {
        project: text(labels.sheet_project),
        number: text(labels.sheet_number),
        title: text(labels.sheet_title),
        scale: text(labels.sheet_scale),
        drawn_by: text(labels.sheet_drawn_by),
        checked_by: text(labels.sheet_checked_by),
        date: text(labels.sheet_date),
        revision: text(labels.sheet_revision),
        revision_mark: text(labels.sheet_rev_mark),
        revision_date: text(labels.sheet_rev_date),
        revision_description: text(labels.sheet_rev_description),
        revision_author: text(labels.sheet_rev_author),
    }
}

/// 🎯️ The corner of a window that scales its viewport: the bottom right.
pub fn handle_of(window: &PaperRect) -> [f64; 2] {
    [window.right(), window.bottom()]
}

/// 🖱️ The topmost viewport (the last one drawn) whose window holds the paper point `at`, grown by `tolerance` millimetres.
pub fn pick(layout: &SheetLayout, at: [f64; 2], tolerance: f64) -> Option<&PlacedViewport> {
    layout.viewports.iter().rev().find(|placed| at[0] >= placed.window.x - tolerance && at[0] <= placed.window.right() + tolerance && at[1] >= placed.window.y - tolerance && at[1] <= placed.window.bottom() + tolerance)
}
//#endregion 🔖️Sheet

//#region 🔖️Records
fn rect_points(rect: &PaperRect) -> [[f64; 2]; 4] {
    [[rect.x, rect.y], [rect.right(), rect.y], [rect.right(), rect.bottom()], [rect.x, rect.bottom()]]
}

fn outline(id: &str, rect: &PaperRect, colour: Rgba, width: f64) -> DslValue {
    path_record(id, "overlay", path(&rect_points(rect), true), Paint { fill: None, stroke: Some((colour, width)), dash: None })
}

fn composed(inner: &DslValue, matrix: [f64; 6]) -> DslValue {
    let m = inner.get("transform").and_then(DslValue::as_array).map(|items| items.iter().filter_map(DslValue::as_f64).collect::<Vec<f64>>()).filter(|items| items.len() == 6).unwrap_or_else(|| vec![1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    let [a, b, c, d, e, f] = matrix;
    DslValue::Array([a * m[0] + c * m[1], b * m[0] + d * m[1], a * m[2] + c * m[3], b * m[2] + d * m[3], a * m[4] + c * m[5] + e, b * m[4] + d * m[5] + f].iter().map(|value| DslValue::float(*value)).collect())
}

/// 🧭️ The records of a view's drawing moved into the window of its viewport: every record is renamed below the viewport and its transform composed with the map from view metres (canvas `(x, -y)`) to paper millimetres.
pub fn placed_records(records: Vec<DslValue>, placed: &PlacedViewport) -> Vec<DslValue> {
    let mm = placed.map.mm;
    let matrix = [mm, 0.0, 0.0, mm, placed.window.x - placed.map.min_x * mm, placed.window.y + placed.map.max_y * mm];
    records
        .into_iter()
        .map(|record| {
            let transform = composed(&record, matrix);
            let DslValue::Object(entries) = record else { return record };
            DslValue::Object(
                entries
                    .into_iter()
                    .map(|(key, value)| {
                        if key == "transform" {
                            return (key, transform.clone());
                        }
                        let renamed = match (key == "id", &value) {
                            (true, DslValue::String(id)) => DslValue::String(format!("{}:{id}", placed.viewport)),
                            _ => value,
                        };
                        (key, renamed)
                    })
                    .collect(),
            )
        })
        .collect()
}

/// ⬜️ The paper that lies in the canvas outside the window but inside `bounds` (the paper rectangle of the whole drawing), as up to four white strips: a cropped viewport shows only its window.
fn masks(placed: &PlacedViewport, drawing: &ViewLinework) -> Vec<DslValue> {
    let mm = placed.map.mm;
    let b = drawing.lines.bounds;
    let reach = PaperRect { x: placed.window.x + (b.min_x - placed.map.min_x) * mm, y: placed.window.y + (placed.map.max_y - b.max_y) * mm, width: (b.max_x - b.min_x) * mm, height: (b.max_y - b.min_y) * mm };
    let w = &placed.window;
    let strips = [
        PaperRect { x: reach.x, y: reach.y, width: reach.width, height: (w.y - reach.y).max(0.0) },
        PaperRect { x: reach.x, y: w.bottom(), width: reach.width, height: (reach.bottom() - w.bottom()).max(0.0) },
        PaperRect { x: reach.x, y: w.y, width: (w.x - reach.x).max(0.0), height: w.height },
        PaperRect { x: w.right(), y: w.y, width: (reach.right() - w.right()).max(0.0), height: w.height },
    ];
    strips
        .iter()
        .enumerate()
        .filter(|(_, strip)| strip.width > 0.0 && strip.height > 0.0)
        .map(|(index, strip)| path_record(&format!("{}:mask:{index}", placed.viewport), "node", path(&rect_points(strip), true), Paint { fill: Some(PAPER), stroke: None, dash: None }))
        .collect()
}

fn mark_record(index: usize, mark: &Mark) -> DslValue {
    match mark {
        Mark::Rect { class, rect, width } => path_record(&format!("ink:{class}:{index}"), "node", path(&rect_points(rect), true), Paint { fill: None, stroke: Some((INK, *width)), dash: None }),
        Mark::Line { class, from, to, width } => path_record(&format!("ink:{class}:{index}"), "node", path(&[[from.0, from.1], [to.0, to.1]], false), Paint { fill: None, stroke: Some((INK, *width)), dash: None }),
        Mark::Text { class, x, y, size, text, .. } => text_record(&format!("ink:{class}:{index}"), (*x, *y - *size), text, *size, if matches!(*class, "cell-label" | "revision-heading") { MUTED } else { INK }),
    }
}

/// 🎨️ The records of the sheet: the paper, the viewports (drawing and crop masks), the marks, the outlines of the selected viewports and of the viewports with findings, then the utility meta record. A sheet that is not there is a text.
pub fn records(layout: Option<&SheetLayout>, inference: &ModelInference, selection: &[String], utility: &str, labels: &BimLabels) -> Vec<DslValue> {
    let mut records = vec![meta_record(utility)];
    let Some(layout) = layout else {
        records.push(text_record("empty", (0.0, 0.0), labels.empty_sheet.as_str(), 6.0, MUTED));
        return records;
    };
    let paper = PaperRect { x: 0.0, y: 0.0, width: layout.width, height: layout.height };
    records.push(path_record("paper", "node", path(&rect_points(&paper), true), Paint { fill: Some(PAPER), stroke: Some((EDGE, 0.4)), dash: None }));
    for placed in &layout.viewports {
        let Some(drawing) = inference.view_linework.get(&placed.view) else { continue };
        records.extend(placed_records(crate::render::plan::layers(&drawing.lines, &[]), placed));
        if placed.cropped {
            records.extend(masks(placed, drawing));
        }
    }
    let marks = sheet_marks(layout, &title_labels(labels));
    for (index, mark) in marks.frame.iter().chain(&marks.captions).chain(&marks.title_block).chain(&marks.revisions).enumerate() {
        records.push(mark_record(index, mark));
    }
    for finding in &layout.findings {
        for id in &finding.viewports {
            if let Some(placed) = layout.viewports.iter().find(|placed| &placed.viewport == id) {
                records.push(outline(&format!("finding:{id}"), &placed.window, WARNING, 0.5));
            }
        }
    }
    for placed in layout.viewports.iter().filter(|placed| selection.contains(&placed.viewport)) {
        records.push(outline(&format!("selected:{}", placed.viewport), &placed.window, ACCENT, 0.8));
    }
    records
}
//#endregion 🔖️Records

//#region 🔖️Render
/// 🎨️ Renders the sheet window.
#[allow(clippy::too_many_arguments)]
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimSheetWindowConfig, selection: &[String], utility: &str, revision: u32, labels: &BimLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    render_over(snapshot, inference, config, selection, utility, revision, labels, &[])
}

/// 🎨️ [`render`] with the records of the authoring overlay (the handles of the selected viewports, the gesture preview) painted last, over everything.
#[allow(clippy::too_many_arguments)]
pub fn render_over(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimSheetWindowConfig, selection: &[String], utility: &str, revision: u32, labels: &BimLabels, overlay: &[DslValue]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let layout = active_sheet(snapshot, config).and_then(|id| inference.sheet_layouts.get(&id));
    let framing = layout.filter(|_| !config.framed).map(|layout| semio_framework_plugin::Canvas2dFraming { revision, bounds: [0.0, 0.0, layout.width, layout.height], padding: FRAMING_PADDING });
    let mut records = records(layout, inference, selection, utility, labels);
    records.extend(overlay.iter().cloned());
    canvas_surface(SURFACE_ID, config.viewport, framing, &records)
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
