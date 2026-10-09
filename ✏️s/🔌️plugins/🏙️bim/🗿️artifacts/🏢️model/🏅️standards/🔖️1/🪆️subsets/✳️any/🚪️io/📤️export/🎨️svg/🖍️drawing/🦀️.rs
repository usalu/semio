//! 🖍️ One view (plan, ceiling plan, section or elevation) as an SVG group: the title, the filled regions (poché of cut walls, columns and mullions, the silhouettes of a projection), the stroked lines and the texts (space tags, grid and datum labels), each
//! primitive a `path` or `text` that names its style class, what it depicts and the model element it belongs to.
//! 📎 https://www.w3.org/TR/SVG11/struct.html#Groups

use super::codec::{attr, element, text, CommonAttrs, SvgElement, TransformOp};
use super::path::{commands, snap};
use super::sheet::{Slot, TITLE_BAND};
use super::style::{annotated, kind_class, paint_rank, style_class, view_class, TAG_SIZE, TITLE_SIZE};
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanLinework, PlanText};

/// 🔑️ An XML id built from a prefix and a model id: characters outside `[A-Za-z0-9_.-]` become `_`.
pub fn xml_id(prefix: &str, raw: &str) -> String {
    let tail: String = raw.chars().map(|letter| if letter.is_ascii_alphanumeric() || matches!(letter, '_' | '-' | '.') { letter } else { '_' }).collect();
    format!("{prefix}-{tail}")
}

pub fn lines_of(text: &PlanText) -> Vec<String> {
    let mut lines = vec![text.label.clone()];
    if !text.detail.is_empty() {
        lines.push(text.detail.clone());
    }
    if let Some(measure) = text.measure {
        lines.push(format!("{measure:.2} m²"));
    }
    lines
}

fn classed(class: String, element: &str, id: &str) -> CommonAttrs {
    let mut common = CommonAttrs::new().with_class(class);
    common.extra_attrs = vec![attr("data-element", element), attr("data-id", id)];
    common
}

fn text_element(plan_text: &PlanText, slot: &Slot) -> SvgElement {
    let (x, y) = slot.frame.point(plan_text.x, plan_text.y);
    let mut common = classed(format!("{} {}", style_class(plan_text.style), kind_class(plan_text.kind)), &plan_text.element, &plan_text.id);
    if annotated(plan_text.kind) {
        common.extra_attrs.push(attr("text-anchor", "middle"));
        if plan_text.height > 0.0 {
            common.extra_attrs.push(attr("font-size", snap(plan_text.height * slot.frame.mm)));
        }
    }
    if plan_text.rotation.abs() > 1e-9 {
        common = common.with_transform(vec![TransformOp::Rotate { angle: snap(-plan_text.rotation.to_degrees()), center: Some((x, y)) }]);
    }
    let lines = lines_of(plan_text);
    let children = if lines.len() == 1 {
        vec![text(&lines[0])]
    } else {
        let lift = snap(-(lines.len() as f64 - 1.0) * TAG_SIZE * 1.15 / 2.0);
        let step = snap(TAG_SIZE * 1.15);
        lines
            .iter()
            .enumerate()
            .map(|(index, line)| {
                let mut tspan = CommonAttrs::new();
                tspan.extra_attrs = vec![attr("dy", if index == 0 { lift } else { step })];
                SvgElement::Tspan { common: tspan, x: Some(x), y: None, children: vec![text(line)] }
            })
            .collect()
    };
    SvgElement::Text { common, x: Some(x), y: Some(y), children }
}

fn layer(class: &str, children: Vec<SvgElement>) -> SvgElement {
    SvgElement::Group { common: CommonAttrs::new().with_class(class), children }
}

/// 🖍️ The layers of the drawing of one view placed in `slot`, in paint order: the filled regions, the stroked lines, the texts and, when the view is annotated, the annotation layer. A sheet viewport wraps these in its own clipped group.
pub fn view_layers(slot: &Slot, plan: &PlanLinework) -> Vec<SvgElement> {
    let mut regions: Vec<_> = plan.regions.iter().collect();
    regions.sort_by_key(|region| paint_rank(region.style));
    let mut lines: Vec<_> = plan.polylines.iter().filter(|line| !annotated(line.kind)).collect();
    lines.sort_by_key(|line| paint_rank(line.style));
    let region_paths = regions
        .into_iter()
        .map(|region| SvgElement::Path {
            common: classed(format!("region {} {}", style_class(region.style), kind_class(region.kind)), &region.element, &region.id),
            d: std::iter::once(&region.outer).chain(region.holes.iter()).flat_map(|ring| commands(ring, true, &slot.frame)).collect(),
        })
        .collect();
    let line_paths = lines
        .into_iter()
        .map(|line| SvgElement::Path { common: classed(format!("line {} {}", style_class(line.style), kind_class(line.kind)), &line.element, &line.id), d: commands(&line.vertices, line.closed, &slot.frame) })
        .collect();
    let mut layers = vec![
        layer("layer regions", region_paths),
        layer("layer lines", line_paths),
        layer("layer texts", plan.texts.iter().filter(|plan_text| !annotated(plan_text.kind)).map(|plan_text| text_element(plan_text, slot)).collect()),
    ];
    let notation = notation_layer(slot, plan);
    if !notation.is_empty() {
        layers.push(layer("layer annotations", notation));
    }
    layers
}

/// 🖍️ The group of the drawing of one view placed in `slot`.
pub fn view_group(slot: &Slot, plan: &PlanLinework) -> SvgElement {
    let title = if slot.name.is_empty() { slot.view.as_str() } else { slot.name.as_str() };
    let mut common = CommonAttrs::new().with_id(xml_id("view", &slot.view)).with_class(format!("view {}", view_class(slot.kind))).with_transform(vec![TransformOp::Translate { x: snap(slot.x), y: Some(snap(slot.y)) }]);
    common.extra_attrs = vec![attr("data-view", &slot.view), attr("data-kind", view_class(slot.kind)), attr("data-building", &slot.building), attr("data-name", &slot.name), attr("data-scale", slot.scale)];
    if let Some(storey) = &slot.storey {
        common.extra_attrs.push(attr("data-storey", storey));
    }
    common.extra_attrs.push(attr("aria-label", title));
    let heading = SvgElement::Text { common: CommonAttrs::new().with_class("title"), x: Some(snap(slot.frame.left)), y: Some(snap(TITLE_BAND - (TITLE_BAND - TITLE_SIZE) / 2.0)), children: vec![text(title)] };
    let mut children = vec![element("title", vec![], vec![text(title)]), heading];
    children.extend(view_layers(slot, plan));
    SvgElement::Group { common, children }
}

/// 🪧️ The annotation layer of a view: the dimension lines, extension lines and marks, the leaders and the texts of dimensions, tags, notes and leaders, in drawing order. Empty for a plan without annotations.
fn notation_layer(slot: &Slot, plan: &PlanLinework) -> Vec<SvgElement> {
    let strokes = plan
        .polylines
        .iter()
        .filter(|line| annotated(line.kind))
        .map(|line| SvgElement::Path { common: classed(format!("line {} {}", style_class(line.style), kind_class(line.kind)), &line.element, &line.id), d: commands(&line.vertices, line.closed, &slot.frame) });
    strokes.chain(plan.texts.iter().filter(|plan_text| annotated(plan_text.kind)).map(|plan_text| text_element(plan_text, slot))).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
