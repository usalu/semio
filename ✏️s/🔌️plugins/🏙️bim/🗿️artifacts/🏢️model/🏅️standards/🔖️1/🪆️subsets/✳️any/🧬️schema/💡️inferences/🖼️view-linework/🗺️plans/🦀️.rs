//! 🗺️ The plan and ceiling plan of a view: the `plan-linework` of its storey cut at the height the view resolves, looked at from above (a plan) or from below (a ceiling plan, which draws what lies above the
//! cut as projection and what lies below it dashed), then filtered by the categories, phase and detail level of the view and cropped to its crop rectangle.

use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{plan_at, Inputs, PlanLinework, PlanStyle, PlanVertex, Sheet};
use super::clip::Rect;
use super::filters::{crop_plan, lets_through};
use super::ViewLinework;
use crate::{view_cut_height, ModelSnapshot, View, ViewKind};
use semio_framework_geometry::Point;

fn mirrored(style: PlanStyle) -> PlanStyle {
    match style {
        PlanStyle::Projection => PlanStyle::Hidden,
        PlanStyle::Hidden => PlanStyle::Projection,
        other => other,
    }
}

fn vertices(rows: &[PlanVertex]) -> Vec<PlanVertex> {
    rows.to_vec()
}

/// 🗺️ The linework of a plan or ceiling plan view; an empty drawing when the storey it cuts is not there.
pub fn plan_view(snapshot: &ModelSnapshot, id: &str, view: &View, inputs: &Inputs<'_>) -> ViewLinework {
    let Some((storey, cut)) = view.storey.as_deref().zip(view_cut_height(snapshot, view)) else {
        return ViewLinework::empty(id, view);
    };
    let plan = plan_at(snapshot, storey, inputs, cut);
    let flip = view.kind == ViewKind::CeilingPlan;
    let style = |style: PlanStyle| if flip { mirrored(style) } else { style };
    let mut sheet = Sheet::default();
    for region in plan.regions.iter().filter(|region| lets_through(snapshot, view, &region.element, region.kind, style(region.style))) {
        sheet.region(&region.element, region.kind, style(region.style), vertices(&region.outer), region.holes.iter().map(|hole| vertices(hole)).collect());
    }
    for line in plan.polylines.iter().filter(|line| lets_through(snapshot, view, &line.element, line.kind, style(line.style))) {
        sheet.polyline(&line.element, line.kind, style(line.style), line.closed, vertices(&line.vertices));
    }
    for text in plan.texts.iter().filter(|text| lets_through(snapshot, view, &text.element, text.kind, style(text.style))) {
        sheet.text(&text.element, text.kind, style(text.style), Point::new(text.x, text.y), text.rotation, &text.label, &text.detail, text.measure);
    }
    let lines = sheet.finish(storey, plan.cut_elevation);
    let lines = PlanLinework { cut_height: cut, ..lines };
    let lines = view.crop.map_or(lines.clone(), |crop| crop_plan(lines, &Rect { x0: crop.min.x, y0: crop.min.y, x1: crop.max.x, y1: crop.max.y }));
    ViewLinework { view: id.to_string(), kind: view.kind, scale: view.scale, detail: view.detail, lines }
}
