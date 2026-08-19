//! 🎞️ Animate app engine facet: 📐️geometry (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
//! relocated verbatim from the deleted artifact-tree `⚙️engine/📐️geometry`).

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod geometry {
    //! 📐️ Two-dimensional shape catalog as VSobjects.

    use crate::editor::animate::engine::text::color::Color;
    use crate::editor::animate::engine::scene::sobject::{Group, Sobject, Sobjects, VSobject};
    use geometry::{append_shape_to_path, Arc, BezPath, Circle, Line, Point, Rect, RoundedRect, RoundedRectRadii, Vec2};
    use std::f64::consts::PI;

    async fn styled_path(path: BezPath, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let mut v = VSobject::from_path(path);
        v.style.fill = Some(fill);
        v.style.stroke = stroke;
        v.style.stroke_width = stroke_width;
        v
    }

    /// · Point marker.
    pub async fn point(at: Point, radius: f64, color: Color) -> VSobject {
        styled_path(circle_path(at, radius), color, None, 0.0)
    }

    /// ●️ Dot (small filled circle).
    pub async fn dot(at: Point, radius: f64, color: Color) -> VSobject {
        point(at, radius, color)
    }

    /// ─️ Line segment.
    pub async fn line(start: Point, end: Point, color: Color, width: f64) -> VSobject {
        styled_path(line_path(start, end), Color::TRANSPARENT, Some(color), width)
    }

    /// ➡️ Arrow from start to end.
    pub async fn arrow(start: Point, end: Point, color: Color, width: f64, tip_len: f64) -> VSobject {
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

    /// ○️ Circle outline or fill.
    pub async fn circle(center: Point, radius: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        styled_path(circle_path(center, radius), fill, stroke, stroke_width)
    }

    /// ◠️ Circular arc.
    pub async fn arc(center: Point, radius: f64, start_angle: f64, sweep: f64, color: Color, width: f64) -> VSobject {
        let a = Arc::new(center, (radius, radius), start_angle, sweep, 0.0);
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &a, 0.01);
        styled_path(path, Color::TRANSPARENT, Some(color), width)
    }

    /// ■️ Axis-aligned square.
    pub async fn square(side: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        rectangle(side, side, center, fill, stroke, stroke_width)
    }

    /// ▭️ Rectangle.
    pub async fn rectangle(width: f64, height: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let r = Rect::new(center.x() - width / 2.0, center.y() - height / 2.0, center.x() + width / 2.0, center.y() + height / 2.0);
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &r, 0.01);
        styled_path(path, fill, stroke, stroke_width)
    }

    /// ▢️ Rounded rectangle.
    pub async fn rounded_rectangle(width: f64, height: f64, radius: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let rect = Rect::new(center.x() - width / 2.0, center.y() - height / 2.0, center.x() + width / 2.0, center.y() + height / 2.0);
        let r = RoundedRect::new(rect, RoundedRectRadii::new(radius, radius, radius, radius));
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &r, 0.01);
        styled_path(path, fill, stroke, stroke_width)
    }

    /// ⬠️ Regular polygon.
    pub async fn polygon(vertices: &[Point], fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
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

    /// △️ Equilateral triangle.
    pub async fn triangle(side: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let h = side * 3.0_f64.sqrt() / 2.0;
        let verts = [Point::new(center.x(), center.y() + 2.0 * h / 3.0), Point::new(center.x() - side / 2.0, center.y() - h / 3.0), Point::new(center.x() + side / 2.0, center.y() - h / 3.0)];
        polygon(&verts, fill, stroke, stroke_width)
    }

    /// ★️ Star polygon.
    pub async fn star(points: u32, outer: f64, inner: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let n = points.max(3) as usize;
        let mut verts = Vec::with_capacity(n * 2);
        for i in 0..(n * 2) {
            let angle = PI / 2.0 + (i as f64) * PI / (n as f64);
            let r = if i % 2 == 0 { outer } else { inner };
            verts.push(Point::new(center.x() + r * angle.cos(), center.y() + r * angle.sin()));
        }
        polygon(&verts, fill, stroke, stroke_width)
    }

    /// ◎️ Annulus (ring).
    pub async fn annulus(center: Point, inner: f64, outer: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let mut path = circle_path(center, outer);
        let hole = circle_path(center, inner);
        for el in hole.elements() {
            path.push(el);
        }
        styled_path(path, fill, stroke, stroke_width)
    }

    /// ◔️ Circular sector.
    pub async fn sector(center: Point, radius: f64, start_angle: f64, sweep: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
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
    pub async fn brace(start: Point, end: Point, direction: Vec2, color: Color, width: f64) -> VSobject {
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
    pub async fn angle(vertex: Point, ray_a: Point, ray_b: Point, radius: f64, color: Color, width: f64) -> VSobject {
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

    async fn circle_path(center: Point, radius: f64) -> BezPath {
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &Circle::new(center, radius), 0.01);
        path
    }

    async fn line_path(start: Point, end: Point) -> BezPath {
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &Line::new(start, end), 0.01);
        path
    }

    /// ╌️ Dashed stroke style built from multiple segment paths.
    #[derive(Clone)]
    pub struct DashedVSobject {
        pub inner: VSobject,
    }

    impl DashedVSobject {
        pub async fn from_segments(paths: Vec<BezPath>, color: Color, width: f64) -> Self {
            let mut inner = VSobject::new();
            inner.set_paths(paths);
            inner.style.fill = None;
            inner.style.stroke = Some(color);
            inner.style.stroke_width = width;
            Self { inner }
        }

        pub async fn as_vobject(&self) -> &VSobject {
            &self.inner
        }
    }

    /// ╌️ Dashed line via repeated stroke segments.
    pub async fn dashed_line(start: Point, end: Point, color: Color, width: f64, dash_len: f64, gap_len: f64) -> VSobject {
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

    /// ⬭️ Axis-aligned ellipse.
    pub async fn ellipse(center: Point, width: f64, height: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
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

    /// ⬡️ Regular polygon with `n` sides inscribed in a circle.
    pub async fn regular_polygon(n: u32, radius: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let sides = n.max(3) as usize;
        let verts: Vec<Point> = (0..sides)
            .map(|i| {
                let angle = PI / 2.0 + (i as f64) * std::f64::consts::TAU / sides as f64;
                Point::new(center.x() + radius * angle.cos(), center.y() + radius * angle.sin())
            })
            .collect();
        polygon(&verts, fill, stroke, stroke_width)
    }

    /// ▢️ Rectangle around an Sobject's bounds.
    // 🔀️ R11 "trivially generic" case: a single erased receiver, no heterogeneous storage — stays
    // generic over `S: Sobject` rather than routed through `Sobjects` (matches `AnimateBuilder`).
    pub async fn surrounding_rectangle<S: Sobject>(mobject: &S, buff: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let b = mobject.bounds();
        rectangle(b.width() + buff * 2.0, b.height() + buff * 2.0, b.center(), fill, stroke, stroke_width)
    }

    /// ⊎ Simple path union by concatenating subpaths.
    pub async fn boolean_union(a: &VSobject, b: &VSobject, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
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
    pub async fn boolean_difference(a: &VSobject, b: &VSobject, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
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
    pub async fn arrow_vector_field<F>(x_range: (f64, f64), y_range: (f64, f64), cols: u32, rows: u32, field: F, color: Color, arrow_scale: f64) -> Group
    where
        F: Fn(f64, f64) -> Vec2,
    {
        let cols = cols.max(1);
        let rows = rows.max(1);
        let dx = (x_range.1 - x_range.0) / cols as f64;
        let dy = (y_range.1 - y_range.0) / rows as f64;
        let mut children: Vec<Sobjects> = Vec::new();
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
                children.push((arrow(start, tip, color, 1.5, arrow_scale * 0.2)).into());
            }
        }
        Group::new(children)
    }

    /// 〰 Stream lines traced through a vector field from seed points.
    pub async fn stream_lines<F>(seeds: &[(f64, f64)], field: F, color: Color, steps: u32, step_size: f64) -> Group
    where
        F: Fn(f64, f64) -> Vec2,
    {
        let steps = steps.max(2);
        let mut children: Vec<Sobjects> = Vec::new();
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
            children.push((v).into());
        }
        Group::new(children)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn shapes_produce_paths() {
            let c = circle(Point::ZERO, 1.0, Color::BLUE, None, 0.0);
            assert!(!c.paths.is_empty());
            let a = arrow(Point::ZERO, Point::new(2.0, 0.0), Color::RED, 2.0, 0.3);
            assert!(!a.paths.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn star_has_vertices() {
            let s = star(5, 1.0, 0.4, Point::ZERO, Color::YELLOW, None, 0.0);
            assert!(s.paths[0].elements().len() > 4);
        }

        #[semio_framework_async_macros::async_test]
        async fn ellipse_and_regular_polygon_build() {
            let e = ellipse(Point::ZERO, 2.0, 1.0, Color::BLUE, None, 0.0);
            assert!(!e.paths.is_empty());
            let p = regular_polygon(6, 1.0, Point::ZERO, Color::GREEN, None, 0.0);
            assert!(!p.paths.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn dashed_line_has_multiple_segments() {
            let d = dashed_line(Point::ZERO, Point::new(4.0, 0.0), Color::WHITE, 2.0, 0.3, 0.2);
            assert!(d.paths.len() > 1);
        }

        #[semio_framework_async_macros::async_test]
        async fn boolean_ops_combine_paths() {
            let a = circle(Point::ZERO, 1.0, Color::BLUE, None, 0.0);
            let b = circle(Point::new(0.5, 0.0), 1.0, Color::RED, None, 0.0);
            let u = boolean_union(&a, &b, Color::PURPLE, None, 0.0);
            assert!(u.paths.len() >= 2);
            let diff = boolean_difference(&a, &b, Color::YELLOW, None, 0.0);
            assert!(diff.paths.len() >= 2);
        }

        #[semio_framework_async_macros::async_test]
        async fn vector_field_helpers_build() {
            let vf = arrow_vector_field((-1.0, 1.0), (-1.0, 1.0), 3, 3, |x, _| Vec2::new(x, 1.0), Color::TEAL, 0.2);
            assert!(!vf.children.is_empty());
            let sl = stream_lines(&[(0.0, 0.0)], |_, y| Vec2::new(1.0, y), Color::WHITE, 8, 0.1);
            assert!(!sl.children.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn vector_field_skips_zero_length_vectors() {
            let vf = arrow_vector_field((-1.0, 1.0), (-1.0, 1.0), 2, 2, |_, _| Vec2::new(0.0, 0.0), Color::TEAL, 0.2);
            assert!(vf.children.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn stream_lines_stops_on_zero_length_field() {
            let sl = stream_lines(&[(0.0, 0.0)], |_, _| Vec2::new(0.0, 0.0), Color::WHITE, 8, 0.1);
            assert_eq!(sl.children.len(), 1);
        }

        #[semio_framework_async_macros::async_test]
        async fn point_dot_and_line_build_paths() {
            let p = point(Point::ZERO, 0.1, Color::RED);
            assert!(!p.paths.is_empty());
            let d = dot(Point::ZERO, 0.1, Color::RED);
            assert!(!d.paths.is_empty());
            let l = line(Point::ZERO, Point::new(1.0, 1.0), Color::BLUE, 1.0);
            assert!(!l.paths.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn square_triangle_and_polygon_build() {
            let sq = square(2.0, Point::ZERO, Color::RED, None, 0.0);
            assert!(!sq.paths.is_empty());
            let tri = triangle(2.0, Point::ZERO, Color::GREEN, None, 0.0);
            assert!(!tri.paths.is_empty());
            let empty_poly = polygon(&[], Color::WHITE, None, 0.0);
            assert!(empty_poly.paths[0].elements().is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn annulus_and_sector_build() {
            let a = annulus(Point::ZERO, 0.5, 1.0, Color::BLUE, None, 0.0);
            assert!(!a.paths.is_empty());
            let s = sector(Point::ZERO, 1.0, 0.0, PI / 2.0, Color::YELLOW, None, 0.0);
            assert!(!s.paths.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn brace_and_angle_build() {
            let b = brace(Point::new(-1.0, 0.0), Point::new(1.0, 0.0), Vec2::new(0.0, -1.0), Color::WHITE, 1.0);
            assert!(!b.paths.is_empty());
            let b_default_dir = brace(Point::new(-1.0, 0.0), Point::new(1.0, 0.0), Vec2::new(0.0, 0.0), Color::WHITE, 1.0);
            assert!(!b_default_dir.paths.is_empty());
            let ang = angle(Point::ZERO, Point::new(1.0, 0.0), Point::new(0.0, 1.0), 0.3, Color::ORANGE, 1.0);
            assert!(!ang.paths.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn surrounding_rectangle_pads_bounds() {
            let c = circle(Point::ZERO, 1.0, Color::BLUE, None, 0.0);
            let r = surrounding_rectangle(&c, 0.5, Color::TRANSPARENT, Some(Color::WHITE), 1.0);
            assert!(!r.paths.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn dashed_line_degenerate_endpoints_falls_back_to_line() {
            let d = dashed_line(Point::new(1.0, 1.0), Point::new(1.0, 1.0), Color::WHITE, 2.0, 0.3, 0.2);
            assert_eq!(d.paths.len(), 1);
        }
    }
}

pub mod three_d {
    //! 🧊️ Three-dimensional Sobjects projected into the scene plane.

    use crate::editor::animate::engine::text::color::Color;
    use crate::editor::animate::engine::geometry::geometry::{circle, line, polygon, rectangle};
    use crate::editor::animate::engine::scene::sobject::{Bounds, Group, Sobject, Sobjects, Style, VSobject};
    use crate::editor::animate::engine::rate::updater::Updater;
    use geometry::{Affine, BezPath, Point};

    /// 📦️ Base 3D Sobject with yaw/pitch and projection scale.
    #[derive(Clone)]
    pub struct ThreeDVSobject {
        pub inner: VSobject,
        pub yaw: f64,
        pub pitch: f64,
        pub depth: f64,
    }

    impl ThreeDVSobject {
        pub async fn new(inner: VSobject) -> Self {
            Self { inner, yaw: 0.0, pitch: 0.0, depth: 0.0 }
        }

        pub async fn project_point(&self, p: (f64, f64, f64)) -> Point {
            let (x, y, z) = p;
            let cy = self.yaw.cos();
            let sy = self.yaw.sin();
            let cp = self.pitch.cos();
            let sp = self.pitch.sin();
            let x1 = x * cy - z * sy;
            let z1 = x * sy + z * cy;
            let y1 = y * cp - z1 * sp;
            let z2 = y * sp + z1 * cp + self.depth;
            let scale = 1.0 / (1.0 + z2 * 0.1);
            Point::new(x1 * scale, y1 * scale)
        }
    }

    impl Sobject for ThreeDVSobject {
        async fn id(&self) -> u64 {
            self.inner.id()
        }
        async fn name(&self) -> &str {
            self.inner.name()
        }
        async fn set_name(&mut self, name: String) {
            self.inner.set_name(name);
        }
        async fn style(&self) -> &Style {
            self.inner.style()
        }
        async fn style_mut(&mut self) -> &mut Style {
            self.inner.style_mut()
        }
        async fn opacity(&self) -> f64 {
            self.inner.opacity()
        }
        async fn set_opacity(&mut self, opacity: f64) {
            self.inner.set_opacity(opacity);
        }
        async fn effective_opacity(&self) -> f64 {
            self.inner.effective_opacity()
        }
        async fn set_parent_opacity(&mut self, parent: f64) {
            self.inner.set_parent_opacity(parent);
        }
        async fn transform(&self) -> Affine {
            self.inner.transform()
        }
        async fn transform_mut(&mut self) -> &mut Affine {
            self.inner.transform_mut()
        }
        async fn bounds(&self) -> Bounds {
            self.inner.bounds()
        }
        async fn paths(&self) -> Vec<BezPath> {
            self.inner.paths()
        }
        async fn children(&self) -> Vec<&Sobjects> {
            self.inner.children()
        }
        async fn visit_children_mut(&mut self, f: &mut dyn FnMut(&mut Sobjects)) {
            self.inner.visit_children_mut(f);
        }
        async fn add_child(&mut self, child: Sobjects) {
            self.inner.add_child(child);
        }
        async fn updaters(&self) -> &[Updater] {
            self.inner.updaters()
        }
        async fn updaters_mut(&mut self) -> &mut Vec<Updater> {
            self.inner.updaters_mut()
        }
        async fn save_state(&mut self) {
            self.inner.save_state();
        }
        async fn restore(&mut self) {
            self.inner.restore();
        }
        async fn generate_target(&mut self) {
            self.inner.generate_target();
        }
        async fn has_target(&self) -> bool {
            self.inner.has_target()
        }
        async fn apply_target(&mut self) {
            self.inner.apply_target();
        }
        async fn clone_box(&self) -> Sobjects {
            self.clone().into()
        }
        async fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        async fn z_order(&self) -> i64 {
            self.inner.z_order()
        }
        async fn set_z_order(&mut self, z: i64) {
            self.inner.set_z_order(z);
        }
        async fn point_ratio(&self) -> f64 {
            self.inner.point_ratio()
        }
    }

    /// 🌐️ Parametric surface wireframe.
    pub struct Surface {
        pub group: Group,
        pub resolution: u32,
    }

    impl Surface {
        pub async fn paraboloid(radius: f64, color: Color) -> Self {
            let steps = 12;
            let mut children: Vec<Sobjects> = Vec::new();
            for i in 0..steps {
                let t = i as f64 / steps as f64 * std::f64::consts::TAU;
                let mut prev = None;
                for j in 0..=steps {
                    let r = radius * j as f64 / steps as f64;
                    let x = r * t.cos();
                    let z = r * t.sin();
                    let y = (x * x + z * z) * 0.2;
                    let td = ThreeDVSobject::new(VSobject::new());
                    let p = td.project_point((x, y, z));
                    if let Some(prev_p) = prev {
                        children.push((line(prev_p, p, color.with_alpha(0.5), 1.0)).into());
                    }
                    prev = Some(p);
                }
            }
            Self { group: Group::new(children), resolution: steps as u32 }
        }
    }

    /// ⚪️ Sphere wireframe.
    pub async fn sphere(radius: f64, center: (f64, f64, f64), color: Color) -> Group {
        let steps = 16;
        let mut children: Vec<Sobjects> = Vec::new();
        let td = ThreeDVSobject::new(VSobject::new());
        for i in 0..steps {
            let phi = i as f64 / steps as f64 * std::f64::consts::PI;
            let mut prev = None;
            for j in 0..=steps {
                let theta = j as f64 / steps as f64 * std::f64::consts::TAU;
                let x = center.0 + radius * phi.sin() * theta.cos();
                let y = center.1 + radius * phi.cos();
                let z = center.2 + radius * phi.sin() * theta.sin();
                let p = td.project_point((x, y, z));
                if let Some(prev_p) = prev {
                    children.push((line(prev_p, p, color.with_alpha(0.6), 1.0)).into());
                }
                prev = Some(p);
            }
        }
        Group::new(children)
    }

    /// 🧊️ Cube wireframe.
    pub async fn cube(side: f64, center: (f64, f64, f64), color: Color) -> Group {
        let h = side / 2.0;
        let corners = [(-h, -h, -h), (h, -h, -h), (h, h, -h), (-h, h, -h), (-h, -h, h), (h, -h, h), (h, h, h), (-h, h, h)];
        let td = ThreeDVSobject::new(VSobject::new());
        let pts: Vec<Point> = corners.iter().map(|(x, y, z)| td.project_point((center.0 + x, center.1 + y, center.2 + z))).collect();
        let edges = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
        let children: Vec<Sobjects> = edges.iter().map(|(a, b)| (line(pts[*a], pts[*b], color, 2.0)).into()).collect();
        Group::new(children)
    }

    /// 🟦️ Solid cube with filled projected faces.
    pub async fn solid_cube(side: f64, center: (f64, f64, f64), fill: Color, stroke: Option<Color>, stroke_width: f64) -> Group {
        let h = side / 2.0;
        let corners = [(-h, -h, -h), (h, -h, -h), (h, h, -h), (-h, h, -h), (-h, -h, h), (h, -h, h), (h, h, h), (-h, h, h)];
        let td = ThreeDVSobject::new(VSobject::new());
        let pts: Vec<Point> = corners.iter().map(|(x, y, z)| td.project_point((center.0 + x, center.1 + y, center.2 + z))).collect();
        let faces: [(&[usize], f64); 6] = [(&[0, 1, 2, 3], 0.85), (&[4, 5, 6, 7], 0.85), (&[0, 1, 5, 4], 0.7), (&[2, 3, 7, 6], 0.7), (&[1, 2, 6, 5], 0.55), (&[0, 3, 7, 4], 0.55)];
        let mut children: Vec<Sobjects> = Vec::new();
        for (indices, alpha) in faces {
            let verts: Vec<Point> = indices.iter().map(|&i| pts[i]).collect();
            children.push((polygon(&verts, fill.with_alpha(alpha), stroke, stroke_width)).into());
        }
        let edges = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
        for (a, b) in edges {
            children.push((line(pts[a], pts[b], stroke.unwrap_or(fill), stroke_width)).into());
        }
        Group::new(children)
    }

    /// 🟦️ Filled face proxy for 3D objects (projected rectangle).
    pub async fn face(width: f64, height: f64, center: Point, fill: Color) -> VSobject {
        rectangle(width, height, center, fill, None, 0.0)
    }

    /// 🔮️ Disc cross-section helper.
    pub async fn disc(radius: f64, center: Point, fill: Color) -> VSobject {
        circle(center, radius, fill, None, 0.0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn cube_has_twelve_edges() {
            let g = cube(2.0, (0.0, 0.0, 0.0), Color::WHITE);
            assert_eq!(g.children.len(), 12);
        }

        #[semio_framework_async_macros::async_test]
        async fn projection_moves_points() {
            let td = ThreeDVSobject::new(VSobject::new());
            let p = td.project_point((1.0, 0.0, 0.0));
            assert!(p.x().is_finite());
        }

        #[semio_framework_async_macros::async_test]
        async fn three_d_vobject_is_sobject() {
            let td = ThreeDVSobject::new(VSobject::new());
            assert_eq!(td.opacity(), 1.0);
        }

        #[semio_framework_async_macros::async_test]
        async fn solid_cube_has_faces() {
            let g = solid_cube(2.0, (0.0, 0.0, 0.0), Color::BLUE, Some(Color::WHITE), 1.0);
            assert!(g.children.len() >= 6);
        }

        #[semio_framework_async_macros::async_test]
        async fn sphere_builds_wireframe_lines() {
            let g = sphere(1.0, (0.0, 0.0, 0.0), Color::WHITE);
            assert!(!g.children.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn face_and_disc_build_projected_shapes() {
            let f = face(2.0, 1.0, Point::ZERO, Color::RED);
            assert!(!f.paths.is_empty());
            let d = disc(1.0, Point::ZERO, Color::BLUE);
            assert!(!d.paths.is_empty());
        }
    }
}

pub mod axes {
    //! 📊️ Coordinate axes, number planes, and complex planes.

    use crate::editor::animate::engine::text::color::Color;
    use crate::editor::animate::engine::geometry::geometry::{arrow, dot, line};
    use crate::editor::animate::engine::scene::sobject::{Group, Sobject, Sobjects, VSobject};
    use crate::editor::animate::engine::text::text::Text;
    use geometry::{BezPath, Point};

    /// 📈️ Cartesian axes with optional labels.
    pub struct Axes {
        pub group: Group,
        pub x_length: f64,
        pub y_length: f64,
        pub origin: Point,
    }

    impl Axes {
        pub async fn new(x_length: f64, y_length: f64, origin: Point, color: Color) -> Self {
            let x_axis = arrow(origin, Point::new(origin.x() + x_length, origin.y()), color, 3.0, 0.2);
            let y_axis = arrow(origin, Point::new(origin.x(), origin.y() + y_length), color, 3.0, 0.2);
            let group = Group::new(vec![(x_axis).into(), (y_axis).into()]);
            Self { group, x_length, y_length, origin }
        }

        pub async fn with_tick_labels(mut self, x_ticks: &[f64], y_ticks: &[f64], color: Color) -> Self {
            for &x in x_ticks {
                let p = self.coords_to_point(x, 0.0);
                let mut label = Text::new(format!("{x:.1}"), color);
                label.inner.move_to(Point::new(p.x(), p.y() - 0.25));
                self.group.add_child((label.inner).into());
                self.group.add_child((line(Point::new(p.x(), p.y() - 0.08), Point::new(p.x(), p.y() + 0.08), color.with_alpha(0.6), 1.0)).into());
            }
            for &y in y_ticks {
                let p = self.coords_to_point(0.0, y);
                let mut label = Text::new(format!("{y:.1}"), color);
                label.inner.move_to(Point::new(p.x() - 0.35, p.y()));
                self.group.add_child((label.inner).into());
                self.group.add_child((line(Point::new(p.x() - 0.08, p.y()), Point::new(p.x() + 0.08, p.y()), color.with_alpha(0.6), 1.0)).into());
            }
            self
        }

        pub async fn coords_to_point(&self, x: f64, y: f64) -> Point {
            Point::new(self.origin.x() + x, self.origin.y() + y)
        }

        pub async fn as_group(&self) -> &Group {
            &self.group
        }
    }

    /// 📉️ Function graph y = f(x) sampled over a range.
    pub struct FunctionGraph {
        pub inner: VSobject,
    }

    impl FunctionGraph {
        pub async fn new<F>(x_range: (f64, f64), axes: &Axes, f: F, samples: u32, color: Color, width: f64) -> Self
        where
            F: Fn(f64) -> f64,
        {
            let samples = samples.max(2);
            let mut path = BezPath::new();
            for i in 0..samples {
                let t = i as f64 / (samples - 1) as f64;
                let x = x_range.0 + t * (x_range.1 - x_range.0);
                let y = f(x);
                let p = axes.coords_to_point(x, y);
                if i == 0 {
                    path.move_to(p);
                } else {
                    path.line_to(p);
                }
            }
            let mut inner = VSobject::from_path(path);
            inner.style.fill = None;
            inner.style.stroke = Some(color);
            inner.style.stroke_width = width;
            Self { inner }
        }
    }

    /// 🌀️ Parametric curve (x(t), y(t)) sampled over a parameter range.
    pub struct ParametricFunction {
        pub inner: VSobject,
    }

    impl ParametricFunction {
        pub async fn new<F>(t_range: (f64, f64), axes: &Axes, f: F, samples: u32, color: Color, width: f64) -> Self
        where
            F: Fn(f64) -> (f64, f64),
        {
            let samples = samples.max(2);
            let mut path = BezPath::new();
            for i in 0..samples {
                let t = t_range.0 + (i as f64 / (samples - 1) as f64) * (t_range.1 - t_range.0);
                let (x, y) = f(t);
                let p = axes.coords_to_point(x, y);
                if i == 0 {
                    path.move_to(p);
                } else {
                    path.line_to(p);
                }
            }
            let mut inner = VSobject::from_path(path);
            inner.style.fill = None;
            inner.style.stroke = Some(color);
            inner.style.stroke_width = width;
            Self { inner }
        }
    }

    /// 🔲️ Number plane with grid lines.
    pub struct NumberPlane {
        pub axes: Axes,
        pub group: Group,
        pub unit_size: f64,
    }

    impl NumberPlane {
        pub async fn new(x_range: (f64, f64), y_range: (f64, f64), unit_size: f64, color: Color) -> Self {
            let origin = Point::new(-x_range.0 * unit_size, -y_range.0 * unit_size);
            let x_len = (x_range.1 - x_range.0) * unit_size;
            let y_len = (y_range.1 - y_range.0) * unit_size;
            let axes = Axes::new(x_len, y_len, origin, color);
            let mut children: Vec<Sobjects> = vec![(arrow(origin, Point::new(origin.x() + x_len, origin.y()), color, 3.0, 0.2)).into(), (arrow(origin, Point::new(origin.x(), origin.y() + y_len), color, 3.0, 0.2)).into()];
            let grid_color = color.with_alpha(0.25);
            let x_steps = ((x_range.1 - x_range.0) as i32).abs().max(1);
            let y_steps = ((y_range.1 - y_range.0) as i32).abs().max(1);
            for i in 0..=x_steps {
                let x = origin.x() + i as f64 * unit_size;
                children.push((line(Point::new(x, origin.y()), Point::new(x, origin.y() + y_len), grid_color, 1.0)).into());
            }
            for j in 0..=y_steps {
                let y = origin.y() + j as f64 * unit_size;
                children.push((line(Point::new(origin.x(), y), Point::new(origin.x() + x_len, y), grid_color, 1.0)).into());
            }
            let group = Group::new(children);
            Self { axes, group, unit_size }
        }
    }

    /// ➖️ One-dimensional number line.
    pub struct NumberLine {
        pub group: Group,
        pub start: Point,
        pub length: f64,
    }

    impl NumberLine {
        pub async fn new(start: Point, length: f64, color: Color) -> Self {
            let axis = line(start, Point::new(start.x() + length, start.y()), color, 3.0);
            let tick_count = 10;
            let mut children: Vec<Sobjects> = vec![(axis).into()];
            for i in 0..=tick_count {
                let x = start.x() + length * i as f64 / tick_count as f64;
                children.push((line(Point::new(x, start.y() - 0.1), Point::new(x, start.y() + 0.1), color, 1.5)).into());
            }
            Self { group: Group::new(children), start, length }
        }

        pub async fn number_to_point(&self, n: f64) -> Point {
            Point::new(self.start.x() + n, self.start.y())
        }
    }

    /// 🔢️ Integer-only number line with unit ticks.
    pub struct IntegerLine {
        pub group: Group,
        pub start: Point,
        pub unit_size: f64,
        pub min: i32,
        pub max: i32,
    }

    impl IntegerLine {
        pub async fn new(start: Point, min: i32, max: i32, unit_size: f64, color: Color) -> Self {
            let span = (max - min).max(1) as f64;
            let length = span * unit_size;
            let axis = line(start, Point::new(start.x() + length, start.y()), color, 3.0);
            let mut children: Vec<Sobjects> = vec![(axis).into()];
            for value in min..=max {
                let x = start.x() + (value - min) as f64 * unit_size;
                children.push((line(Point::new(x, start.y() - 0.12), Point::new(x, start.y() + 0.12), color, 1.5)).into());
                if value % 5 == 0 {
                    children.push((dot(Point::new(x, start.y()), 0.04, color)).into());
                }
            }
            Self { group: Group::new(children), start, unit_size, min, max }
        }

        pub async fn integer_to_point(&self, n: i32) -> Point {
            Point::new(self.start.x() + (n - self.min) as f64 * self.unit_size, self.start.y())
        }
    }

    /// ℂ Complex plane (axes with imaginary vertical axis).
    pub struct ComplexPlane {
        pub plane: NumberPlane,
    }

    impl ComplexPlane {
        pub async fn new(range: f64, unit_size: f64, color: Color) -> Self {
            let plane = NumberPlane::new((-range, range), (-range, range), unit_size, color);
            Self { plane }
        }

        pub async fn complex_to_point(&self, re: f64, im: f64) -> Point {
            self.plane.axes.coords_to_point(re * self.plane.unit_size, im * self.plane.unit_size)
        }

        pub async fn plot_point(&self, re: f64, im: f64, color: Color) -> VSobject {
            dot(self.complex_to_point(re, im), 0.06, color)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn axes_map_coordinates() {
            let axes = Axes::new(4.0, 3.0, Point::ZERO, Color::WHITE);
            let p = axes.coords_to_point(1.0, 2.0);
            assert!((p.x() - 1.0).abs() < 1e-9);
            assert!((p.y() - 2.0).abs() < 1e-9);
        }

        #[semio_framework_async_macros::async_test]
        async fn number_line_maps_values() {
            let nl = NumberLine::new(Point::ZERO, 10.0, Color::WHITE);
            assert!((nl.number_to_point(5.0).x() - 5.0).abs() < 1e-9);
        }

        #[semio_framework_async_macros::async_test]
        async fn integer_line_maps_values() {
            let il = IntegerLine::new(Point::ZERO, 0, 10, 1.0, Color::WHITE);
            assert!((il.integer_to_point(5).x() - 5.0).abs() < 1e-9);
        }

        #[semio_framework_async_macros::async_test]
        async fn axes_tick_labels_and_graphs() {
            let axes = Axes::new(4.0, 3.0, Point::ZERO, Color::WHITE).with_tick_labels(&[1.0, 2.0], &[1.0], Color::WHITE);
            assert!(axes.group.children.len() > 2);
            let fg = FunctionGraph::new((0.0, 2.0), &axes, |x| x * x, 16, Color::YELLOW, 2.0);
            assert!(!fg.inner.paths.is_empty());
            let pf = ParametricFunction::new((0.0, std::f64::consts::TAU), &axes, |t| (t.cos(), t.sin()), 32, Color::GREEN, 2.0);
            assert!(!pf.inner.paths.is_empty());
        }
    }
}
