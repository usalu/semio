//! 🖌️ Drawing kinds and the stroke paint of the layout scene; embedded drawings stream through deferred drawing bounds, never through a materialized scene. See [`super`].

use super::*;

//#region ⚙️Scene
pub fn drawing_kind(kind: &str) -> bool {
    matches!(kind, "drawing" | "dwg" | "dxf" | "svg" | "cad" | "s.draw.drawing" | "s.stdio.dwg" | "s.stdio.dxf" | "s.stdio.svg" | "s.cad.cad")
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
