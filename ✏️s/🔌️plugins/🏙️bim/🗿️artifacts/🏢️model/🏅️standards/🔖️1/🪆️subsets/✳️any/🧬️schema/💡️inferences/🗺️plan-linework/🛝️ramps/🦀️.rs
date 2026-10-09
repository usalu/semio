//! 🛝️ Ramps in the plan: the mitred outline of the slab (arcs stay arcs), a line across the width at every end of a landing, the up arrow along the centre line, and the slope tag (percent and ratio) at its middle.

use super::{classify, outline_style, Context, PlanKind, PlanStyle, PlanVertex, Sheet};
use crate::standards::v1::subsets::any::schema::inferences::ramp_runs::{strip_of, LENGTH_EPS};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::Point;

const ARROW_HEAD: f64 = 0.25;

fn head(sheet: &mut Sheet, id: &str, tip: Point, heading: f64) {
    let barb = |side: f64| {
        let (sin, cos) = heading.sin_cos();
        let (along, aside) = (-ARROW_HEAD, side * ARROW_HEAD * 0.4);
        Point::new(tip.x + along * cos - aside * sin, tip.y + along * sin + aside * cos)
    };
    sheet.path(id, PlanKind::RampArrow, PlanStyle::Annotation, &[barb(1.0), tip, barb(-1.0)]);
}

fn chain(segments: &[BulgeSeg]) -> Vec<PlanVertex> {
    let mut vertices: Vec<PlanVertex> = segments.iter().map(|segment| PlanVertex { x: segment.start.x, y: segment.start.y, bulge: segment.bulge }).collect();
    if let Some(last) = segments.last() {
        vertices.push(PlanVertex { x: last.end.x, y: last.end.y, bulge: 0.0 });
    }
    vertices
}

/// 🛝️ Draws the ramps of the storey.
pub fn draw(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, ramp) in cx.snapshot.ramps.iter().filter(|(_, ramp)| ramp.storey == cx.storey) {
        let Some(run) = cx.inputs.ramp_runs.get(id.as_str()).copied() else { continue };
        let strip = strip_of(ramp);
        if strip.is_empty() {
            continue;
        }
        let (low, high) = (run.base_z.min(run.top_z) - ramp.thickness, run.base_z.max(run.top_z));
        let style = outline_style(classify(low, high, cx.cut));
        sheet.loop_polyline(id, PlanKind::RampOutline, style, &strip.outline());
        for landing in &run.landings {
            for s in [landing.from, landing.to] {
                if s > LENGTH_EPS && s < run.length - LENGTH_EPS {
                    let (left, right, _) = strip.section_at(s);
                    sheet.path(id, PlanKind::RampLanding, style, &[left, right]);
                }
            }
        }
        let walk: Vec<BulgeSeg> = if run.rise >= 0.0 { strip.centre.clone() } else { strip.centre.iter().rev().map(BulgeSeg::reversed).collect() };
        sheet.polyline(id, PlanKind::RampArrow, PlanStyle::Annotation, false, chain(&walk));
        if let Some(last) = walk.last() {
            let tangent = last.tangent_at(1.0);
            head(sheet, id, last.end, tangent.y.atan2(tangent.x));
        }
        let (_, _, middle) = strip.section_at(run.length / 2.0);
        let (index, t) = strip.locate(run.length / 2.0);
        let tangent = strip.centre[index].tangent_at(t);
        let label = format!("{:.1} %", run.slope * 100.0);
        let detail = if run.slope > LENGTH_EPS { format!("1:{:.1}", 1.0 / run.slope) } else { String::new() };
        sheet.text(id, PlanKind::RampTag, PlanStyle::Annotation, middle, tangent.y.atan2(tangent.x), &label, &detail, Some(run.slope));
    }
}
