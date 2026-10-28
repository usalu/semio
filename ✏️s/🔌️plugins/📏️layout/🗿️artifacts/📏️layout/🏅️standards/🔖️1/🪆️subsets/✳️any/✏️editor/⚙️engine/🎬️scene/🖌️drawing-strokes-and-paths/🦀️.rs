//! 🖌️ Drawing strokes and paths of the layout scene: the raw drawing strokes, their kinds and placement in a frame, the stroke paint plan, the draw node collection, the curve point helpers and the display stroke painting. See [`super`].

use super::*;

//#region ⚙️Scene
pub(super) fn raw_drawing_strokes(scene: Option<&VectorScene>) -> Vec<DisplayStroke> {
    let Some(drawing) = scene else { return Vec::new() };
    let mut raw = Vec::new();
    for layer in &drawing.layers {
        if !layer.visible {
            continue;
        }
        collect_draw_node(&layer.root, &drawing.styles, 0.0, 0.0, &mut raw);
    }
    raw
}

pub(super) fn drawing_strokes(raw: &[DisplayStroke], page_width: f64, page_height: f64) -> Vec<DisplayStroke> {
    if page_width <= 0.0 || page_height <= 0.0 {
        return Vec::new();
    }
    fit_strokes(raw, page_width, page_height)
}

/// 🖊️ Classifies first-party vector artifacts for shared Drawing placement.
pub fn drawing_kind(kind: &str) -> bool {
    matches!(kind, "drawing" | "dwg" | "dxf" | "svg" | "cad" | "s.draw.drawing" | "s.stdio.dwg" | "s.stdio.dxf" | "s.stdio.svg" | "s.cad.cad")
}

pub(super) fn place_drawing_in_frame(raw: &[DisplayStroke], bounds: &LayoutBounds) -> Vec<DisplayStroke> {
    if raw.is_empty() || bounds.width <= 0.0 || bounds.height <= 0.0 {
        return Vec::new();
    }
    let fitted = fit_strokes(raw, bounds.width, bounds.height);
    let center_x = (bounds.x + bounds.width * 0.5) as f32;
    let center_y = (bounds.y + bounds.height * 0.5) as f32;
    let radians = bounds.rotation as f32;
    let (sin, cos) = radians.sin_cos();
    fitted
        .into_iter()
        .map(|stroke| DisplayStroke {
            closed: stroke.closed,
            color: stroke.color,
            width: stroke.width,
            fill: stroke.fill,
            stack: stroke.stack,
            points: stroke
                .points
                .into_iter()
                .map(|(x, y)| {
                    let page_x = x + bounds.x as f32;
                    let page_y = y + bounds.y as f32;
                    if bounds.rotation.abs() < 1.0e-6 {
                        (page_x, page_y)
                    } else {
                        let dx = page_x - center_x;
                        let dy = page_y - center_y;
                        (center_x + dx * cos - dy * sin, center_y + dx * sin + dy * cos)
                    }
                })
                .collect(),
        })
        .collect()
}

fn plan_stroke_paint(styles: &[VectorStyle], name: &Option<String>) -> (Option<[f32; 4]>, [f32; 4], f32) {
    let mut color = [0.15_f32, 0.2, 0.28, 1.0];
    let mut width = 1.0_f32;
    let mut fill = None;
    if let Some(style) = name.as_ref().and_then(|name| styles.iter().find(|style| style.name == *name)) {
        if let Some(stroke) = &style.stroke {
            color = [stroke.r, stroke.g, stroke.b, stroke.a];
        }
        if let Some(paint) = &style.fill {
            fill = Some([paint.r, paint.g, paint.b, paint.a]);
        }
        if let Some(opacity) = style.opacity {
            color[3] *= opacity;
            if let Some(fill_color) = fill.as_mut() {
                fill_color[3] *= opacity;
            }
        }
        if let Some(stroke_width) = style.stroke_width {
            width = stroke_width.clamp(0.1, 64.0) as f32;
        }
    }
    (fill, color, width)
}

fn collect_draw_node(node: &VectorNode, styles: &[VectorStyle], tx: f64, ty: f64, out: &mut Vec<DisplayStroke>) {
    match node {
        VectorNode::Group { transform, children } => {
            let next_x = tx + transform.translation.x;
            let next_y = ty + transform.translation.y;
            for child in children {
                collect_draw_node(child, styles, next_x, next_y, out);
            }
        }
        VectorNode::Path { segments, style } => {
            let (fill, color, width) = plan_stroke_paint(styles, style);
            let mut current: Option<(f64, f64)> = None;
            let mut points = Vec::new();
            let mut closed = false;
            let flush = |points: &mut Vec<(f32, f32)>, closed: &mut bool, out: &mut Vec<DisplayStroke>| {
                if points.len() >= 2 && out.len() < 128 {
                    out.push(DisplayStroke { points: std::mem::take(points), closed: *closed, color, width, fill, stack: 0 });
                } else {
                    points.clear();
                }
                *closed = false;
            };
            for segment in segments {
                match segment {
                    VectorSegment::MoveTo { to } => {
                        flush(&mut points, &mut closed, out);
                        current = Some((to.x + tx, to.y + ty));
                        points.push(((to.x + tx) as f32, (to.y + ty) as f32));
                    }
                    VectorSegment::LineTo { to } => {
                        let point = (to.x + tx, to.y + ty);
                        if points.is_empty() {
                            if let Some(start) = current {
                                points.push((start.0 as f32, start.1 as f32));
                            }
                        }
                        push_point(&mut points, point);
                        current = Some(point);
                    }
                    VectorSegment::CubicTo { c1, c2, to } => {
                        let start = current.unwrap_or((c1.x + tx, c1.y + ty));
                        for step in 1..=4 {
                            let t = step as f64 / 4.0;
                            push_point(&mut points, cubic_point(start, (c1.x + tx, c1.y + ty), (c2.x + tx, c2.y + ty), (to.x + tx, to.y + ty), t));
                        }
                        current = Some((to.x + tx, to.y + ty));
                    }
                    VectorSegment::QuadTo { c, to } => {
                        let start = current.unwrap_or((c.x + tx, c.y + ty));
                        for step in 1..=4 {
                            let t = step as f64 / 4.0;
                            push_point(&mut points, quad_point(start, (c.x + tx, c.y + ty), (to.x + tx, to.y + ty), t));
                        }
                        current = Some((to.x + tx, to.y + ty));
                    }
                    VectorSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => {
                        let end = (to.x + tx, to.y + ty);
                        let start = current.unwrap_or(end);
                        let sampled = arc_points(start, *rx, *ry, *x_rotation, *large_arc, *sweep, end);
                        if sampled.is_empty() {
                            push_point(&mut points, end);
                        } else {
                            for point in sampled {
                                push_point(&mut points, point);
                            }
                        }
                        current = Some(end);
                    }
                    VectorSegment::Close => closed = true,
                }
            }
            flush(&mut points, &mut closed, out);
        }
        VectorNode::Text { .. } | VectorNode::Image { .. } => {}
    }
}

fn push_point(points: &mut Vec<(f32, f32)>, point: (f64, f64)) {
    if points.len() < 128 {
        points.push((point.0 as f32, point.1 as f32));
    }
}

fn cubic_point(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), t: f64) -> (f64, f64) {
    let u = 1.0 - t;
    let x = u * u * u * p0.0 + 3.0 * u * u * t * p1.0 + 3.0 * u * t * t * p2.0 + t * t * t * p3.0;
    let y = u * u * u * p0.1 + 3.0 * u * u * t * p1.1 + 3.0 * u * t * t * p2.1 + t * t * t * p3.1;
    (x, y)
}

fn quad_point(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), t: f64) -> (f64, f64) {
    let u = 1.0 - t;
    ((u * u * p0.0) + (2.0 * u * t * p1.0) + (t * t * p2.0), (u * u * p0.1) + (2.0 * u * t * p1.1) + (t * t * p2.1))
}

fn arc_points(from: (f64, f64), rx: f64, ry: f64, x_rotation_degrees: f64, large_arc: bool, sweep: bool, to: (f64, f64)) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let mut start = from;
    for [c1, c2, end] in arc_cubics(from, rx, ry, x_rotation_degrees, large_arc, sweep, to) {
        let end_point = (end[0], end[1]);
        for step in 1..=4 {
            let t = step as f64 / 4.0;
            points.push(cubic_point(start, (c1[0], c1[1]), (c2[0], c2[1]), end_point, t));
        }
        start = end_point;
    }
    points
}

fn arc_cubics(from: (f64, f64), mut rx: f64, mut ry: f64, x_rotation_degrees: f64, large_arc: bool, sweep: bool, to: (f64, f64)) -> Vec<[[f64; 2]; 3]> {
    let span = ((to.0 - from.0).powi(2) + (to.1 - from.1).powi(2)).sqrt();
    if span == 0.0 {
        return Vec::new();
    }
    rx = rx.abs();
    ry = ry.abs();
    if rx == 0.0 || ry == 0.0 {
        return vec![[[from.0, from.1], [to.0, to.1], [to.0, to.1]]];
    }
    let phi = x_rotation_degrees.to_radians();
    let (sin_phi, cos_phi) = phi.sin_cos();
    let dx = (from.0 - to.0) / 2.0;
    let dy = (from.1 - to.1) / 2.0;
    let x1 = cos_phi * dx + sin_phi * dy;
    let y1 = -sin_phi * dx + cos_phi * dy;
    let lambda = (x1 * x1) / (rx * rx) + (y1 * y1) / (ry * ry);
    if lambda > 1.0 {
        let scale = lambda.sqrt();
        rx *= scale;
        ry *= scale;
    }
    let numerator = (rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1).max(0.0);
    let denominator = rx * rx * y1 * y1 + ry * ry * x1 * x1;
    let root = if denominator > 0.0 { (numerator / denominator).sqrt() } else { 0.0 };
    let sign = if large_arc == sweep { -1.0 } else { 1.0 };
    let cx1 = sign * root * rx * y1 / ry;
    let cy1 = -sign * root * ry * x1 / rx;
    let cx = cos_phi * cx1 - sin_phi * cy1 + (from.0 + to.0) / 2.0;
    let cy = sin_phi * cx1 + cos_phi * cy1 + (from.1 + to.1) / 2.0;
    let angle = |ux: f64, uy: f64, vx: f64, vy: f64| {
        let turn = (ux * vy - uy * vx).signum();
        let cosine = ((ux * vx + uy * vy) / ((ux * ux + uy * uy).sqrt() * (vx * vx + vy * vy).sqrt())).clamp(-1.0, 1.0);
        if turn < 0.0 { -cosine.acos() } else { cosine.acos() }
    };
    let theta1 = angle(1.0, 0.0, (x1 - cx1) / rx, (y1 - cy1) / ry);
    let mut delta = angle((x1 - cx1) / rx, (y1 - cy1) / ry, (-x1 - cx1) / rx, (-y1 - cy1) / ry);
    if !sweep && delta > 0.0 {
        delta -= std::f64::consts::TAU;
    } else if sweep && delta < 0.0 {
        delta += std::f64::consts::TAU;
    }
    let pieces = (delta.abs() / std::f64::consts::FRAC_PI_2).ceil().max(1.0) as usize;
    let step = delta / pieces as f64;
    let handle = 4.0 / 3.0 * (step / 4.0).tan();
    let on_ellipse = |theta: f64| {
        let (sin, cos) = theta.sin_cos();
        ([cos_phi * rx * cos - sin_phi * ry * sin + cx, sin_phi * rx * cos + cos_phi * ry * sin + cy], [-cos_phi * rx * sin - sin_phi * ry * cos, -sin_phi * rx * sin + cos_phi * ry * cos])
    };
    (0..pieces)
        .map(|piece| {
            let (a, b) = (theta1 + step * piece as f64, theta1 + step * (piece + 1) as f64);
            let ((p0, d0), (p1, d1)) = (on_ellipse(a), on_ellipse(b));
            let end = if piece + 1 == pieces { [to.0, to.1] } else { p1 };
            [[p0[0] + handle * d0[0], p0[1] + handle * d0[1]], [p1[0] - handle * d1[0], p1[1] - handle * d1[1]], end]
        })
        .collect()
}

fn fit_strokes(strokes: &[DisplayStroke], page_width: f64, page_height: f64) -> Vec<DisplayStroke> {
    let Some((scale, ox, oy)) = fit_from_points(stroke_points(strokes), page_width, page_height) else { return Vec::new() };
    strokes.iter().map(|stroke| DisplayStroke { closed: stroke.closed, color: stroke.color, width: stroke.width, fill: stroke.fill, stack: stroke.stack, points: stroke.points.iter().map(|(x, y)| (x * scale + ox, y * scale + oy)).collect() }).collect()
}

pub(super) fn paint_display_stroke(scene: &mut Scene, transform: Affine, stroke: &DisplayStroke) {
    if let Some(fill) = stroke.fill.filter(|color| color[3] > 0.001) {
        if stroke.points.len() >= 3 {
            let mut path = BezPath::new();
            for (index, (x, y)) in stroke.points.iter().enumerate() {
                let point = Point::new(*x as f64, *y as f64);
                if index == 0 { path.move_to(point); } else { path.line_to(point); }
            }
            path.close_path();
            scene.fill(FillRule::NonZero, transform, Color::new(fill), None, &path);
        }
    }
    let mut points = stroke.points.clone();
    if stroke.closed && points.len() >= 2 {
        points.push(points[0]);
    }
    for pair in points.windows(2) {
        scene.stroke(&Stroke::new(stroke.width.max(0.1) as f64), transform, Color::new(stroke.color), None, &Line::new(Point::new(pair[0].0 as f64, pair[0].1 as f64), Point::new(pair[1].0 as f64, pair[1].1 as f64)));
    }
}
//#endregion ⚙️Scene
