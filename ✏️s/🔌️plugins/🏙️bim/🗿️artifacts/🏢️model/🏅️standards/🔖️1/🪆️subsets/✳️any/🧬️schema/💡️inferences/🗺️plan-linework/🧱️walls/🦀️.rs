//! 🧱️ Walls, curtain walls and the openings they host in the plan: poché with opening gaps, layer lines, door and window symbols.

use super::{classify, outline_style, Context, Cutting, PlanKind, PlanStyle, PlanVertex, Sheet};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged as corners, extents_of, seg};
use crate::standards::v1::subsets::any::schema::inferences::opening_frames::{OpeningFrame, PlanRole as Stroke, PlanShape};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::WallLayout;
use crate::{CurtainWall, ModelSnapshot, OpeningKind, Phase, Point2, Wall};
use semio_framework_geometry::bulge::{band_loop, BulgeSeg};
use semio_framework_geometry::loops::Vertex as Corner;
use semio_framework_geometry::Point;

const EPS: f64 = 1e-9;

/// 🪟️ An opening of a host with its resolved frame and the absolute height span of its cut.
struct Placed<'a> {
    id: &'a str,
    kind: &'a OpeningKind,
    frame: &'a OpeningFrame,
    low: f64,
    high: f64,
}

fn placed_openings<'a>(cx: &'a Context<'_>, host_id: &str, base_z: f64) -> Vec<Placed<'a>> {
    cx.snapshot
        .openings
        .iter()
        .filter(|(_, opening)| opening.host == host_id)
        .filter_map(|(id, opening)| {
            let frame = *cx.inputs.frames.get(id.as_str())?;
            let (low, high) = (base_z + frame.cut.z_min, base_z + frame.cut.z_max);
            Some(Placed { id, kind: &opening.kind, frame, low, high })
        })
        .filter(|placed| placed.frame.width > EPS && placed.frame.height > EPS)
        .collect()
}

fn stroke_vertices(shape: &PlanShape) -> Vec<PlanVertex> {
    match shape {
        PlanShape::Line { from, to } => vec![PlanVertex { x: from.x, y: from.y, bulge: 0.0 }, PlanVertex { x: to.x, y: to.y, bulge: 0.0 }],
        PlanShape::Arc { centre, radius, start_angle, sweep } => {
            let at = |angle: f64| (centre.x + radius * angle.cos(), centre.y + radius * angle.sin());
            let ((x0, y0), (x1, y1)) = (at(*start_angle), at(start_angle + sweep));
            vec![PlanVertex { x: x0, y: y0, bulge: (sweep / 4.0).tan() }, PlanVertex { x: x1, y: y1, bulge: 0.0 }]
        }
    }
}

fn pieces(layout: &WallLayout, axis: &BulgeSeg, gaps: &[(f64, f64)]) -> Vec<Vec<Corner>> {
    let length = axis.length();
    let trim = |left: Point2, right: Point2| (Point::new(left.x, left.y), Point::new(right.x, right.y));
    free_runs(length, gaps)
        .into_iter()
        .filter_map(|(from, to)| {
            let start = (from <= EPS).then(|| trim(layout.left_face.start, layout.right_face.start));
            let end = (to >= length - EPS).then(|| trim(layout.left_face.end, layout.right_face.end));
            band_loop(&axis.subsegment(from / length, to / length), layout.offset_left, layout.offset_right, start, end)
        })
        .map(|rows| rows.iter().map(|(p, b)| Corner::new(*p, *b)).collect())
        .collect()
}

fn free_runs(length: f64, gaps: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut sorted: Vec<(f64, f64)> = gaps.iter().map(|(a, b)| (a.max(0.0), b.min(length))).filter(|(a, b)| b > a).collect();
    sorted.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (mut runs, mut at) = (Vec::new(), 0.0);
    for (from, to) in sorted {
        if from > at + EPS {
            runs.push((at, from));
        }
        at = at.max(to);
    }
    if length > at + EPS {
        runs.push((at, length));
    }
    runs
}

fn layer_lines(sheet: &mut Sheet, id: &str, layout: &WallLayout, axis: &BulgeSeg, gaps: &[(f64, f64)]) {
    let rows = &layout.layer_offsets;
    if rows.len() < 3 || layout.thickness <= EPS {
        return;
    }
    let length = axis.length();
    for &offset in &rows[1..rows.len() - 1] {
        let Some(base) = axis.offset(offset) else { continue };
        let fraction = (layout.offset_left - offset) / layout.thickness;
        let pick = |left: Point2, right: Point2| Point::new(left.x + (right.x - left.x) * fraction, left.y + (right.y - left.y) * fraction);
        let curve = base.retarget(pick(layout.left_face.start, layout.right_face.start), pick(layout.left_face.end, layout.right_face.end));
        let at = |s: f64| curve.closest(axis.point_at_length(s)).t;
        for (from, to) in free_runs(length, gaps) {
            let (t0, t1) = (if from <= EPS { 0.0 } else { at(from) }, if to >= length - EPS { 1.0 } else { at(to) });
            if t1 > t0 {
                sheet.segment(id, PlanKind::WallLayer, PlanStyle::Projection, &curve.subsegment(t0, t1));
            }
        }
    }
}

fn draw_opening(sheet: &mut Sheet, cx: &Context<'_>, axis: &BulgeSeg, placed: &Placed<'_>, left: f64, right: f64) {
    let crossing = classify(placed.low, placed.high, cx.cut);
    let style = match crossing {
        Cutting::Cut => PlanStyle::Cut,
        Cutting::Below => PlanStyle::Projection,
        Cutting::Above => PlanStyle::Hidden,
    };
    let thin = if crossing == Cutting::Cut { PlanStyle::Projection } else { style };
    let length = axis.length();
    let span = (placed.frame.cut.s_min.max(0.0) / length, (placed.frame.cut.s_max.min(length)) / length);
    let section = (span.1 > span.0).then(|| axis.subsegment(span.0, span.1));
    match placed.kind {
        OpeningKind::Window { window_type } => {
            let depth = cx.snapshot.window_types.get(window_type).map_or(0.0, |kind| kind.frame_depth);
            let shift = (left - right) / 2.0;
            if let Some(section) = section {
                if let Some(rows) = band_loop(&section, depth / 2.0 + shift, depth / 2.0 - shift, None, None) {
                    sheet.loop_polyline(placed.id, PlanKind::WindowFrame, style, &rows.iter().map(|(p, b)| Corner::new(*p, *b)).collect::<Vec<_>>());
                }
                if crossing == Cutting::Cut {
                    for offset in [left, -right] {
                        if let Some(face) = section.offset(offset) {
                            sheet.segment(placed.id, PlanKind::WindowSill, PlanStyle::Projection, &face);
                        }
                    }
                }
            }
            for stroke in placed.frame.plan.iter().filter(|stroke| stroke.role == Stroke::Glazing) {
                sheet.polyline(placed.id, PlanKind::WindowGlazing, thin, false, stroke_vertices(&stroke.shape));
            }
        }
        OpeningKind::Door { .. } => {
            for stroke in &placed.frame.plan {
                let (kind, line) = if stroke.role == Stroke::Leaf { (PlanKind::DoorLeaf, style) } else { (PlanKind::DoorSwing, thin) };
                if stroke.role != Stroke::Glazing {
                    sheet.polyline(placed.id, kind, line, false, stroke_vertices(&stroke.shape));
                }
            }
        }
        OpeningKind::Void { .. } => {}
    }
}

fn draw_wall(sheet: &mut Sheet, cx: &Context<'_>, id: &str, wall: &Wall) {
    let Some(layout) = cx.inputs.layouts.get(id).copied() else { return };
    let outline = corners(&layout.footprint);
    if outline.len() < 3 {
        return;
    }
    let (left, right) = (layout.offset_left, layout.offset_right);
    let axis = seg(&wall.axis);
    let cutting = classify(layout.base_z, layout.top_z, cx.cut);
    let openings = placed_openings(cx, id, layout.base_z);
    let demolished = wall.phase == Phase::Demolished;
    if cutting != Cutting::Cut || demolished {
        let style = if demolished && cutting == Cutting::Cut { PlanStyle::Hidden } else { outline_style(cutting) };
        sheet.loop_polyline(id, PlanKind::WallOutline, style, &outline);
    } else {
        let ranges: Vec<(f64, f64)> = openings.iter().filter(|placed| classify(placed.low, placed.high, cx.cut) == Cutting::Cut).map(|placed| (placed.frame.cut.s_min, placed.frame.cut.s_max)).collect();
        for piece in pieces(layout, &axis, &ranges) {
            sheet.loop_region(id, PlanKind::WallCut, PlanStyle::Cut, &piece, &[]);
        }
        layer_lines(sheet, id, layout, &axis, &ranges);
    }
    if !demolished {
        for placed in &openings {
            draw_opening(sheet, cx, &axis, placed, left, right);
        }
    }
}

fn draw_curtain(sheet: &mut Sheet, cx: &Context<'_>, id: &str, curtain: &CurtainWall) {
    let Some(layout) = cx.inputs.curtains.get(id).copied() else { return };
    let cutting = classify(layout.base_z, layout.top_z, cx.cut);
    let axis = seg(&curtain.axis);
    sheet.segment(id, PlanKind::CurtainAxis, outline_style(cutting), &axis);
    if cutting == Cutting::Cut {
        let Some(kind) = cx.snapshot.curtain_wall_types.get(&curtain.curtain_wall_type) else { return };
        let last = layout.u_edges.len().saturating_sub(1);
        for (k, s) in layout.u_edges.iter().copied().enumerate().take(10_001) {
            let (width, depth) = extents_of(if k == 0 || k == last { &kind.border_mullion } else { &kind.interior_mullion });
            let (centre, tangent) = (axis.point_at_length(s), axis.tangent_at_length(s));
            let (half, normal) = (Point::new(tangent.x * width / 2.0, tangent.y * width / 2.0), Point::new(-tangent.y * depth / 2.0, tangent.x * depth / 2.0));
            let corner = |a: f64, b: f64| Corner::corner(centre.x + a * half.x + b * normal.x, centre.y + a * half.y + b * normal.y);
            sheet.loop_region(id, PlanKind::CurtainMullion, PlanStyle::Cut, &[corner(-1.0, -1.0), corner(1.0, -1.0), corner(1.0, 1.0), corner(-1.0, 1.0)], &[]);
        }
    }
}

/// 🧱️ Draws the walls, curtain walls and openings of the storey.
pub fn draw(sheet: &mut Sheet, cx: &Context<'_>) {
    let ModelSnapshot { walls, curtain_walls, .. } = cx.snapshot;
    for (id, wall) in walls.iter().filter(|(_, wall)| wall.storey == cx.storey) {
        draw_wall(sheet, cx, id, wall);
    }
    for (id, curtain) in curtain_walls.iter().filter(|(_, curtain)| curtain.storey == cx.storey) {
        draw_curtain(sheet, cx, id, curtain);
    }
}
