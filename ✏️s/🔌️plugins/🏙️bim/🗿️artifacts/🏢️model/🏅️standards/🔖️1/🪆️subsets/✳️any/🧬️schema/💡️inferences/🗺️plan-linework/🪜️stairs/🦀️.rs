//! 🪜️ Stairs in the plan: flight and landing outlines, one riser line per riser (projection below the cut, dashed above it), the cut line
//! where the plane passes through a riser, and the up arrow along the walking line.

use super::{classify, outline_style, Context, Cutting, PlanKind, PlanStyle, PlanVertex, Sheet};
use crate::standards::v1::subsets::any::schema::inferences::bodies::rectangle as footprint;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::point;
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::{StairFlightRun, StairRun, StairWinder};
use semio_framework_geometry::bulge::bulge_from_sweep;
use semio_framework_geometry::loops::Vertex as Corner;
use semio_framework_geometry::Point;
use std::f64::consts::PI;

const ARROW_HEAD: f64 = 0.25;
const EPS: f64 = 1e-9;

fn at(origin: Point, angle: f64, along: f64, aside: f64) -> Point {
    let (sin, cos) = angle.sin_cos();
    Point::new(origin.x + along * cos - aside * sin, origin.y + along * sin + aside * cos)
}

fn rectangle(centre: Point, direction: f64, depth: f64, width: f64) -> Vec<Corner> {
    footprint(centre, direction, depth / 2.0, depth / 2.0, width)
}

fn riser_style(run: &StairRun, flight: &StairFlightRun, riser: u32, cut: f64) -> Option<PlanStyle> {
    let low = flight.base_z + f64::from(riser) * run.riser_height;
    match classify(low, low + run.riser_height, cut) {
        Cutting::Below => Some(PlanStyle::Projection),
        Cutting::Above => Some(PlanStyle::Hidden),
        Cutting::Cut => None,
    }
}

fn straight_flight(sheet: &mut Sheet, id: &str, run: &StairRun, flight: &StairFlightRun, outline: PlanStyle, cut: f64) {
    let (start, width, direction) = (point(&flight.start), run.width, flight.direction);
    let depth = flight.length.max(EPS);
    sheet.loop_polyline(id, PlanKind::StairOutline, outline, &rectangle(at(start, direction, depth / 2.0, 0.0), direction, depth, width));
    for riser in 0..=flight.treads {
        let distance = f64::from(riser) * flight.tread;
        match riser_style(run, flight, riser, cut) {
            Some(style) => sheet.path(id, PlanKind::StairRiser, style, &[at(start, direction, distance, -width / 2.0), at(start, direction, distance, width / 2.0)]),
            None => {
                let step = flight.tread * 0.25;
                sheet.path(id, PlanKind::StairCutLine, PlanStyle::Cut, &[at(start, direction, distance, -width / 2.0), at(start, direction, distance + step, -width * 0.1), at(start, direction, distance - step, width * 0.1), at(start, direction, distance, width / 2.0)]);
            }
        }
    }
}

fn winding_flight(sheet: &mut Sheet, id: &str, run: &StairRun, flight: &StairFlightRun, winder: &StairWinder, outline: PlanStyle, cut: f64) {
    let around = |radius: f64, angle: f64| Point::new(winder.centre.x + radius * angle.cos(), winder.centre.y + radius * angle.sin());
    let (a0, a1) = (winder.start_angle, winder.start_angle + winder.sweep);
    let bulge = bulge_from_sweep(winder.sweep);
    let sector = [Corner::new(around(winder.outer_radius, a0), bulge), Corner::new(around(winder.outer_radius, a1), 0.0), Corner::new(around(winder.inner_radius, a1), -bulge), Corner::new(around(winder.inner_radius, a0), 0.0)];
    sheet.loop_polyline(id, PlanKind::StairOutline, outline, &sector);
    for riser in 0..=flight.treads {
        let angle = a0 + winder.sweep * f64::from(riser) / f64::from(flight.treads.max(1));
        let ends = [around(winder.inner_radius, angle), around(winder.outer_radius, angle)];
        match riser_style(run, flight, riser, cut) {
            Some(style) => sheet.path(id, PlanKind::StairRiser, style, &ends),
            None => sheet.path(id, PlanKind::StairCutLine, PlanStyle::Cut, &ends),
        }
    }
}

fn arrow(sheet: &mut Sheet, id: &str, run: &StairRun) {
    let Some(last) = run.flights.last() else { return };
    if let Some(winder) = last.winder {
        let radius = (winder.inner_radius + winder.outer_radius) / 2.0;
        let around = |angle: f64| Point::new(winder.centre.x + radius * angle.cos(), winder.centre.y + radius * angle.sin());
        let (a0, a1) = (winder.start_angle, winder.start_angle + winder.sweep);
        let (from, tip) = (around(a0), around(a1));
        sheet.polyline(id, PlanKind::StairArrow, PlanStyle::Annotation, false, vec![PlanVertex { x: from.x, y: from.y, bulge: bulge_from_sweep(winder.sweep) }, PlanVertex { x: tip.x, y: tip.y, bulge: 0.0 }]);
        let heading = a1 + winder.sweep.signum() * PI / 2.0;
        head(sheet, id, tip, heading);
        return;
    }
    let mut walk: Vec<Point> = Vec::new();
    for (index, flight) in run.flights.iter().enumerate() {
        let (start, end) = (point(&flight.start), at(point(&flight.start), flight.direction, flight.length, 0.0));
        if index > 0 {
            if let Some(landing) = run.landings.get(index - 1) {
                walk.push(point(&landing.centre));
            }
        }
        walk.extend([start, end]);
    }
    if let (Some(&tip), Some(&before)) = (walk.last(), walk.get(walk.len().wrapping_sub(2))) {
        sheet.path(id, PlanKind::StairArrow, PlanStyle::Annotation, &walk);
        head(sheet, id, tip, (tip.y - before.y).atan2(tip.x - before.x));
    }
}

fn head(sheet: &mut Sheet, id: &str, tip: Point, heading: f64) {
    let barb = |side: f64| at(tip, heading, -ARROW_HEAD, side * ARROW_HEAD * 0.4);
    sheet.path(id, PlanKind::StairArrow, PlanStyle::Annotation, &[barb(1.0), tip, barb(-1.0)]);
}

/// 🪜️ Draws the stairs of the storey.
pub fn draw(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, _) in cx.snapshot.stairs.iter().filter(|(_, stair)| stair.storey == cx.storey) {
        let Some(run) = cx.inputs.runs.get(id.as_str()).copied() else { continue };
        if run.riser_count == 0 {
            continue;
        }
        let outline = outline_style(classify(run.base_z, run.top_z, cx.cut));
        for flight in &run.flights {
            match &flight.winder {
                Some(winder) => winding_flight(sheet, id, &run, flight, winder, outline, cx.cut),
                None => straight_flight(sheet, id, &run, flight, outline, cx.cut),
            }
        }
        for landing in &run.landings {
            sheet.loop_polyline(id, PlanKind::StairLanding, outline, &rectangle(point(&landing.centre), landing.direction, landing.depth, landing.width));
        }
        arrow(sheet, id, &run);
    }
}
