//! 📐 Two-dimensional shape catalog as VSobjects.

use crate::color::Color;
use crate::sobject::VSobject;
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
    append_shape_to_path(&mut path, a, 0.01);
    styled_path(path, Color::TRANSPARENT, Some(color), width)
}

/// ■ Axis-aligned square.
pub fn square(side: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    rectangle(side, side, center, fill, stroke, stroke_width)
}

/// ▭ Rectangle.
pub fn rectangle(width: f64, height: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let r = Rect::new(
        center.x() - width / 2.0,
        center.y() - height / 2.0,
        center.x() + width / 2.0,
        center.y() + height / 2.0,
    );
    let mut path = BezPath::new();
    append_shape_to_path(&mut path, r, 0.01);
    styled_path(path, fill, stroke, stroke_width)
}

/// ▢ Rounded rectangle.
pub fn rounded_rectangle(width: f64, height: f64, radius: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
    let rect = Rect::new(
        center.x() - width / 2.0,
        center.y() - height / 2.0,
        center.x() + width / 2.0,
        center.y() + height / 2.0,
    );
    let r = RoundedRect::new(rect, RoundedRectRadii::new(radius, radius, radius, radius));
    let mut path = BezPath::new();
    append_shape_to_path(&mut path, r, 0.01);
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
    let verts = [
        Point::new(center.x(), center.y() + 2.0 * h / 3.0),
        Point::new(center.x() - side / 2.0, center.y() - h / 3.0),
        Point::new(center.x() + side / 2.0, center.y() - h / 3.0),
    ];
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
        path.push(*el);
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
    let dir = if direction.hypot() < 1e-9 {
        Vec2::new(0.0, -1.0)
    } else {
        direction / direction.hypot()
    };
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
    append_shape_to_path(&mut path, Circle::new(center, radius), 0.01);
    path
}

fn line_path(start: Point, end: Point) -> BezPath {
    let mut path = BezPath::new();
    append_shape_to_path(&mut path, Line::new(start, end), 0.01);
    path
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
}
