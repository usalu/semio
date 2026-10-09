//! 🎨️ One sheet as an SVG 1.1 document (`s.stdio.svg@1.1/*`) in paper millimetres: the viewBox is the paper, the frame, the viewports, their captions, the title block and the revision table are groups of it. Every viewport is a group clipped to its window
//! that holds the layers of its view (regions, lines, texts, annotations) already mapped to paper millimetres at the scale of the viewport, so a CAD or a browser measures the drawing in millimetres of paper.
//! The tree is the typed SVG model of the `s.stdio.svg` artifact and the text is written by the XML writer of `s.stdio.xml`, both behind the SVG export's `codec`.
//! 📎 https://www.w3.org/TR/SVG11/masking.html#ClippingPaths

use super::ink::{sheet_marks, Anchor, Mark, TitleLabels};
use crate::standards::v1::subsets::any::io::export::svg::codec::{attr, document_text, element, text, CommonAttrs, SvgElement, ViewBox};
use crate::standards::v1::subsets::any::io::export::svg::drawing::{view_layers, xml_id};
use crate::standards::v1::subsets::any::io::export::svg::path::{snap, Frame};
use crate::standards::v1::subsets::any::io::export::svg::sheet::Slot;
use crate::standards::v1::subsets::any::io::export::svg::style::{self, view_class};
use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::{PaperRect, PlacedViewport, SheetLayout};
use crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
use crate::ModelSnapshot;
use std::collections::BTreeMap;

const NAMESPACE: &str = "http://www.w3.org/2000/svg";

const SHEET_STYLE: &str = "\n      .sheet-page{font-family:sans-serif}\n      rect{fill:none;stroke:#000}\n      line{stroke:#000}\n      text.cell-label,text.revision-heading{fill:#444}\n      text.caption{font-weight:bold}\n    ";

fn rect_element(common: CommonAttrs, rect: &PaperRect) -> SvgElement {
    SvgElement::Rect { common, x: snap(rect.x), y: snap(rect.y), width: snap(rect.width), height: snap(rect.height), rx: None, ry: None }
}

fn mark_element(mark: &Mark) -> SvgElement {
    match mark {
        Mark::Rect { class, rect, width } => {
            let mut common = CommonAttrs::new().with_class(*class);
            common.extra_attrs = vec![attr("stroke-width", snap(*width))];
            rect_element(common, rect)
        }
        Mark::Line { class, from, to, width } => {
            let mut common = CommonAttrs::new().with_class(*class);
            common.extra_attrs = vec![attr("stroke-width", snap(*width))];
            SvgElement::Line { common, x1: snap(from.0), y1: snap(from.1), x2: snap(to.0), y2: snap(to.1) }
        }
        Mark::Text { class, x, y, size, bold, anchor, text: content } => {
            let mut common = CommonAttrs::new().with_class(*class);
            common.extra_attrs = vec![attr("font-size", snap(*size)), attr("text-anchor", match anchor {
                Anchor::Start => "start",
                Anchor::Middle => "middle",
                Anchor::End => "end",
            })];
            if *bold {
                common.extra_attrs.push(attr("font-weight", "bold"));
            }
            SvgElement::Text { common, x: Some(snap(*x)), y: Some(snap(*y)), children: vec![text(content)] }
        }
    }
}

fn group(class: &str, children: Vec<SvgElement>) -> SvgElement {
    SvgElement::Group { common: CommonAttrs::new().with_class(class), children }
}

fn window_text(window: &PaperRect) -> String {
    format!("{} {} {} {}", snap(window.x), snap(window.y), snap(window.width), snap(window.height))
}

fn viewport_group(model: &ModelSnapshot, placed: &PlacedViewport, drawing: Option<&ViewLinework>) -> SvgElement {
    let view = model.views.get(&placed.view);
    let frame = Frame { min_x: placed.map.min_x, max_y: placed.map.max_y, left: placed.window.x, top: placed.window.y, mm: placed.map.mm };
    let slot = Slot {
        view: placed.view.clone(),
        building: view.map(|row| row.building.clone()).unwrap_or_default(),
        name: placed.label.clone(),
        kind: placed.kind,
        storey: view.and_then(|row| row.storey.clone()),
        scale: placed.scale,
        x: 0.0,
        y: 0.0,
        width: placed.window.width,
        height: placed.window.height,
        frame,
    };
    let mut common = CommonAttrs::new().with_id(xml_id("viewport", &placed.viewport)).with_class(format!("viewport {}", view_class(placed.kind)));
    common.extra_attrs = vec![
        attr("data-viewport", &placed.viewport),
        attr("data-view", &placed.view),
        attr("data-kind", view_class(placed.kind)),
        attr("data-scale", placed.scale),
        attr("data-label", &placed.label),
        attr("data-window", window_text(&placed.window)),
        attr("clip-path", format!("url(#{})", xml_id("clip", &placed.viewport))),
        attr("aria-label", &placed.label),
    ];
    let mut children = vec![element("title", vec![], vec![text(&placed.label)])];
    if let Some(drawing) = drawing {
        children.extend(view_layers(&slot, &drawing.lines));
    }
    SvgElement::Group { common, children }
}

fn clip_path(placed: &PlacedViewport) -> SvgElement {
    element("clipPath", vec![attr("id", xml_id("clip", &placed.viewport))], vec![rect_element(CommonAttrs::new(), &placed.window)])
}

/// 🎨️ The SVG text of the sheet `layout`: its viewports draw the linework of `drawings` (by view id), its marks use the headings of `labels`. `model` names the building and storey of every view for the data attributes.
pub fn sheet_svg(model: &ModelSnapshot, layout: &SheetLayout, drawings: &BTreeMap<&str, &ViewLinework>, labels: &TitleLabels) -> Result<String, String> {
    let marks = sheet_marks(layout, labels);
    let (width, height) = (snap(layout.width), snap(layout.height));
    let heading = format!("{} {}", layout.number, layout.name);
    let mut common = CommonAttrs::new().with_class("sheet-page");
    common.extra_attrs = vec![
        attr("version", "1.1"),
        attr("data-unit", "mm"),
        attr("data-sheet", &layout.sheet),
        attr("data-number", &layout.number),
        attr("data-name", &layout.name),
        attr("data-paper", &layout.paper),
        attr("data-width", width),
        attr("data-height", height),
        attr("aria-label", &heading),
    ];
    let mut children = vec![
        element("title", vec![], vec![text(&heading)]),
        element("desc", vec![], vec![text(&format!("Sheet {heading}: {} viewport(s) at paper size {}, paper millimetres.", layout.viewports.len(), layout.paper))]),
        element("style", vec![], vec![text(&format!("{}{SHEET_STYLE}", style::sheet()))]),
        SvgElement::Defs { common: CommonAttrs::new(), children: layout.viewports.iter().map(clip_path).collect() },
        group("frame", marks.frame.iter().map(mark_element).collect()),
    ];
    children.extend(layout.viewports.iter().map(|placed| viewport_group(model, placed, drawings.get(placed.view.as_str()).copied())));
    children.push(group("captions", marks.captions.iter().map(mark_element).collect()));
    children.push(group("title-block", marks.title_block.iter().map(mark_element).collect()));
    if !marks.revisions.is_empty() {
        children.push(group("revision-table", marks.revisions.iter().map(mark_element).collect()));
    }
    let root = SvgElement::Svg {
        common,
        view_box: Some(ViewBox { min_x: 0.0, min_y: 0.0, width, height }),
        width: Some(format!("{width}mm")),
        height: Some(format!("{height}mm")),
        xmlns: Some(NAMESPACE.into()),
        children,
    };
    document_text(&root)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
