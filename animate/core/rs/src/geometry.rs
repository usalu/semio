//! 📐 Two-dimensional shape catalog as VSobjects.

use crate::color::Color;
use crate::sobject::{Group, Sobject, VSobject};
use mathematical_geometry::{append_shape_to_path, Arc, BezPath, Circle, Line, Point, Rect, RoundedRect, RoundedRectRadii, Vec2};
use std::f64::consts::PI;

fn styled_path(path: BezPath, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let mut v = VSobject::from_path(path);
    v.style.fill = Some(fill);
    v.style.stroke = stroke;
    v.style.stroke_width = stroke_width;
    v
}

/// · Point marker.
pub fn point(at: Point, radius: f64, color: Color) -> VSobject {
    styled_path(circle_path(at, radius), color, None, 0.0)
}

/// ● Dot (small filled circle).
pub fn dot(at: Point, radius: f64, color: Color) -> VSobject {
    point(at, radius, color)
}

/// ─ Line segment.
pub fn line(start: Point, end: Point, color: Color, width: f64) -> VSobject {
    styled_path(line_path(start, end), Color::TRANSPARENT, Some(color), width)
}

/// ➡️ Arrow from start to end.
pub fn arrow(start: Point, end: Point, color: Color, width: f64, tip_len: f64) -> VSobject {
    let mut path = line_path(start, end);
    let dir = end - start;
    let len = dir.hypot().max(1e-9);
    let u = dir / len;
    let perp = Vec2::new(-u.y(), u.x());
    let tip = end;
    let base = tip - u * tip_len;
    path.move_to(base + perp * tip_len * 0.35);
    path.line_to(tip);
    path.line_to(base - perp * tip_len * 0.35);
    styled_path(path, Color::TRANSPARENT, Some(color), width)
}

/// ○ Circle outline or fill.
pub fn circle(center: Point, radius: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    styled_path(circle_path(center, radius), fill, stroke, stroke_width)
}

/// ◠ Circular arc.
pub fn arc(center: Point, radius: f64, start_angle: f64, sweep: f64, color: Color, width: f64) -> VSobject {
    let a = Arc::new(center, (radius, radius), start_angle, sweep, 0.0);
    let mut path = BezPath::new();
    append_shape_to_path(&mut path, &a, 0.01);
    styled_path(path, Color::TRANSPARENT, Some(color), width)
}

/// ■ Axis-aligned square.
pub fn square(side: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    rectangle(side, side, center, fill, stroke, stroke_width)
}

/// ▭ Rectangle.
pub fn rectangle(width: f64, height: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let r = Rect::new(center.x() - width / 2.0, center.y() - height / 2.0, center.x() + width / 2.0, center.y() + height / 2.0);
    let mut path = BezPath::new();
    append_shape_to_path(&mut path, &r, 0.01);
    styled_path(path, fill, stroke, stroke_width)
}

/// ▢ Rounded rectangle.
pub fn rounded_rectangle(width: f64, height: f64, radius: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let rect = Rect::new(center.x() - width / 2.0, center.y() - height / 2.0, center.x() + width / 2.0, center.y() + height / 2.0);
    let r = RoundedRect::new(rect, RoundedRectRadii::new(radius, radius, radius, radius));
    let mut path = BezPath::new();
    append_shape_to_path(&mut path, &r, 0.01);
    styled_path(path, fill, stroke, stroke_width)
}

/// ⬠ Regular polygon.
pub fn polygon(vertices: &[Point], fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let mut path = BezPath::new();
    if let Some(first) = vertices.first() {
        path.move_to(*first);
        for p in vertices.iter().skip(1) {
            path.line_to(*p);
        }
        path.close_path();
    }
    styled_path(path, fill, stroke, stroke_width)
}

/// △ Equilateral triangle.
pub fn triangle(side: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let h = side * 3.0_f64.sqrt() / 2.0;
    let verts = [Point::new(center.x(), center.y() + 2.0 * h / 3.0), Point::new(center.x() - side / 2.0, center.y() - h / 3.0), Point::new(center.x() + side / 2.0, center.y() - h / 3.0)];
    polygon(&verts, fill, stroke, stroke_width)
}

/// ★ Star polygon.
pub fn star(points: u32, outer: f64, inner: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let n = points.max(3) as usize;
    let mut verts = Vec::with_capacity(n * 2);
    for i in 0..(n * 2) {
        let angle = PI / 2.0 + (i as f64) * PI / (n as f64);
        let r = if i % 2 == 0 { outer } else { inner };
        verts.push(Point::new(center.x() + r * angle.cos(), center.y() + r * angle.sin()));
    }
    polygon(&verts, fill, stroke, stroke_width)
}

/// ◎ Annulus (ring).
pub fn annulus(center: Point, inner: f64, outer: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let mut path = circle_path(center, outer);
    let hole = circle_path(center, inner);
    for el in hole.elements() {
        path.push(el);
    }
    styled_path(path, fill, stroke, stroke_width)
}

/// ◔ Circular sector.
pub fn sector(center: Point, radius: f64, start_angle: f64, sweep: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let mut path = BezPath::new();
    path.move_to(center);
    let steps = 64;
    for i in 0..=steps {
        let t = start_angle + sweep * (i as f64 / steps as f64);
        path.line_to(Point::new(center.x() + radius * t.cos(), center.y() + radius * t.sin()));
    }
    path.close_path();
    styled_path(path, fill, stroke, stroke_width)
}

/// { } Brace under content.
pub fn brace(start: Point, end: Point, direction: Vec2, color: Color, width: f64) -> VSobject {
    let mid = Point::new((start.x() + end.x()) / 2.0, (start.y() + end.y()) / 2.0);
    let dir = if direction.hypot() < 1e-9 { Vec2::new(0.0, -1.0) } else { direction / direction.hypot() };
    let depth = (end - start).hypot() * 0.15;
    let tip = mid + dir * depth;
    let mut path = BezPath::new();
    path.move_to(start);
    path.quad_to(tip, end);
    styled_path(path, Color::TRANSPARENT, Some(color), width)
}

/// ∠ Angle arc between two rays from vertex.
pub fn angle(vertex: Point, ray_a: Point, ray_b: Point, radius: f64, color: Color, width: f64) -> VSobject {
    let va = ray_a - vertex;
    let vb = ray_b - vertex;
    let a1 = va.y().atan2(va.x());
    let a2 = vb.y().atan2(vb.x());
    let mut sweep = a2 - a1;
    while sweep <= -PI {
        sweep += 2.0 * PI;
    }
    while sweep > PI {
        sweep -= 2.0 * PI;
    }
    arc(vertex, radius, a1, sweep, color, width)
}

fn circle_path(center: Point, radius: f64) -> BezPath {
    let mut path = BezPath::new();
    append_shape_to_path(&mut path, &Circle::new(center, radius), 0.01);
    path
}

fn line_path(start: Point, end: Point) -> BezPath {
    let mut path = BezPath::new();
    append_shape_to_path(&mut path, &Line::new(start, end), 0.01);
    path
}

/// ╌ Dashed stroke style built from multiple segment paths.
#[derive(Clone)]
pub struct DashedVSobject {
    pub inner: VSobject,
}

impl DashedVSobject {
    pub fn from_segments(paths: Vec<BezPath>, color: Color, width: f64) -> Self {
        let mut inner = VSobject::new();
        inner.set_paths(paths);
        inner.style.fill = None;
        inner.style.stroke = Some(color);
        inner.style.stroke_width = width;
        Self { inner }
    }

    pub fn as_vobject(&self) -> &VSobject {
        &self.inner
    }
}

/// ╌ Dashed line via repeated stroke segments.
pub fn dashed_line(start: Point, end: Point, color: Color, width: f64, dash_len: f64, gap_len: f64) -> VSobject {
    let dir = end - start;
    let total = dir.hypot();
    if total < 1e-9 {
        return line(start, end, color, width);
    }
    let u = dir / total;
    let step = (dash_len + gap_len).max(1e-9);
    let mut paths = Vec::new();
    let mut dist = 0.0;
    while dist < total {
        let seg_start = start + u * dist;
        let seg_end = start + u * (dist + dash_len).min(total);
        let mut path = BezPath::new();
        path.move_to(seg_start);
        path.line_to(seg_end);
        paths.push(path);
        dist += step;
    }
    DashedVSobject::from_segments(paths, color, width).inner
}

/// ⬭ Axis-aligned ellipse.
pub fn ellipse(center: Point, width: f64, height: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let rx = width / 2.0;
    let ry = height / 2.0;
    let steps = 64;
    let mut path = BezPath::new();
    for i in 0..=steps {
        let t = (i as f64 / steps as f64) * std::f64::consts::TAU;
        let p = Point::new(center.x() + rx * t.cos(), center.y() + ry * t.sin());
        if i == 0 {
            path.move_to(p);
        } else {
            path.line_to(p);
        }
    }
    path.close_path();
    styled_path(path, fill, stroke, stroke_width)
}

/// ⬡ Regular polygon with `n` sides inscribed in a circle.
pub fn regular_polygon(n: u32, radius: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let sides = n.max(3) as usize;
    let verts: Vec<Point> = (0..sides)
        .map(|i| {
            let angle = PI / 2.0 + (i as f64) * std::f64::consts::TAU / sides as f64;
            Point::new(center.x() + radius * angle.cos(), center.y() + radius * angle.sin())
        })
        .collect();
    polygon(&verts, fill, stroke, stroke_width)
}

/// ▢ Rectangle around an Sobject's bounds.
pub fn surrounding_rectangle(mobject: &dyn Sobject, buff: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let b = mobject.bounds();
    rectangle(b.width() + buff * 2.0, b.height() + buff * 2.0, b.center(), fill, stroke, stroke_width)
}

/// ⊎ Simple path union by concatenating subpaths.
pub fn boolean_union(a: &VSobject, b: &VSobject, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let mut paths = a.paths.clone();
    paths.extend(b.paths.clone());
    let mut v = VSobject::new();
    v.set_paths(paths);
    v.style.fill = Some(fill);
    v.style.stroke = stroke;
    v.style.stroke_width = stroke_width;
    v
}

/// ⊖ Simple path difference via compound path (outer + hole subpaths).
pub fn boolean_difference(a: &VSobject, b: &VSobject, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let mut paths = a.paths.clone();
    paths.extend(b.paths.clone());
    let mut v = VSobject::new();
    v.set_paths(paths);
    v.style.fill = Some(fill);
    v.style.stroke = stroke;
    v.style.stroke_width = stroke_width;
    v
}

/// ➡️ Grid of small arrows sampling a vector field.
pub fn arrow_vector_field<F>(x_range: (f64, f64), y_range: (f64, f64), cols: u32, rows: u32, field: F, color: Color, arrow_scale: f64) -> Group
where
    F: Fn(f64, f64) -> Vec2,
{
    let cols = cols.max(1);
    let rows = rows.max(1);
    let dx = (x_range.1 - x_range.0) / cols as f64;
    let dy = (y_range.1 - y_range.0) / rows as f64;
    let mut children: Vec<Box<dyn Sobject>> = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let x = x_range.0 + (col as f64 + 0.5) * dx;
            let y = y_range.0 + (row as f64 + 0.5) * dy;
            let start = Point::new(x, y);
            let v = field(x, y);
            let len = v.hypot();
            if len < 1e-9 {
                continue;
            }
            let u = v / len;
            let tip = start + u * arrow_scale;
            children.push(Box::new(arrow(start, tip, color, 1.5, arrow_scale * 0.2)));
        }
    }
    Group::new(children)
}

/// 〰️ Stream lines traced through a vector field from seed points.
pub fn stream_lines<F>(seeds: &[(f64, f64)], field: F, color: Color, steps: u32, step_size: f64) -> Group
where
    F: Fn(f64, f64) -> Vec2,
{
    let steps = steps.max(2);
    let mut children: Vec<Box<dyn Sobject>> = Vec::new();
    for &(sx, sy) in seeds {
        let mut path = BezPath::new();
        let mut x = sx;
        let mut y = sy;
        path.move_to(Point::new(x, y));
        for _ in 0..steps {
            let v = field(x, y);
            let len = v.hypot();
            if len < 1e-9 {
                break;
            }
            let u = v / len;
            x += u.x() * step_size;
            y += u.y() * step_size;
            path.line_to(Point::new(x, y));
        }
        let mut v = VSobject::from_path(path);
        v.style.fill = None;
        v.style.stroke = Some(color);
        v.style.stroke_width = 1.5;
        children.push(Box::new(v));
    }
    Group::new(children)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shapes_produce_paths() {
        let c = circle(Point::ZERO, 1.0, Color::BLUE, None, 0.0);
        assert!(!c.paths.is_empty());
        let a = arrow(Point::ZERO, Point::new(2.0, 0.0), Color::RED, 2.0, 0.3);
        assert!(!a.paths.is_empty());
    }

    #[test]
    fn star_has_vertices() {
        let s = star(5, 1.0, 0.4, Point::ZERO, Color::YELLOW, None, 0.0);
        assert!(s.paths[0].elements().len() > 4);
    }

    #[test]
    fn ellipse_and_regular_polygon_build() {
        let e = ellipse(Point::ZERO, 2.0, 1.0, Color::BLUE, None, 0.0);
        assert!(!e.paths.is_empty());
        let p = regular_polygon(6, 1.0, Point::ZERO, Color::GREEN, None, 0.0);
        assert!(!p.paths.is_empty());
    }

    #[test]
    fn dashed_line_has_multiple_segments() {
        let d = dashed_line(Point::ZERO, Point::new(4.0, 0.0), Color::WHITE, 2.0, 0.3, 0.2);
        assert!(d.paths.len() > 1);
    }

    #[test]
    fn boolean_ops_combine_paths() {
        let a = circle(Point::ZERO, 1.0, Color::BLUE, None, 0.0);
        let b = circle(Point::new(0.5, 0.0), 1.0, Color::RED, None, 0.0);
        let u = boolean_union(&a, &b, Color::PURPLE, None, 0.0);
        assert!(u.paths.len() >= 2);
        let diff = boolean_difference(&a, &b, Color::YELLOW, None, 0.0);
        assert!(diff.paths.len() >= 2);
    }

    #[test]
    fn vector_field_helpers_build() {
        let vf = arrow_vector_field((-1.0, 1.0), (-1.0, 1.0), 3, 3, |x, _| Vec2::new(x, 1.0), Color::TEAL, 0.2);
        assert!(!vf.children.is_empty());
        let sl = stream_lines(&[(0.0, 0.0)], |_, y| Vec2::new(1.0, y), Color::WHITE, 8, 0.1);
        assert!(!sl.children.is_empty());
    }
}
