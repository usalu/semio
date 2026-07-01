//! 🖊️ In-process [`kernel_2d_engine::DrawingKernel`] store with SVG/PDF export.

#[cfg(feature = "booleans")]
pub mod booleans {
// #region booleans
//! 🔀 Planar path boolean operations (optional `booleans` feature).

use kernel_2d_engine::{DrawingError, PathSegment};
use geo::{BooleanOps, Coord, LineString, MultiPolygon, Polygon};

fn close_polygon(coords: &mut Vec<Coord<f64>>) -> Result<Polygon<f64>, DrawingError> {
    if coords.len() < 3 {
        return Err(DrawingError::InvalidInput("boolean input needs a closed polygon".into()));
    }
    if coords.first() != coords.last() {
        coords.push(coords[0]);
    }
    Ok(Polygon::new(LineString(std::mem::take(coords)), vec![]))
}

fn segments_to_multipolygon(segments: &[PathSegment]) -> Result<MultiPolygon<f64>, DrawingError> {
    let mut polygons = Vec::new();
    let mut coords = Vec::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => {
                if !coords.is_empty() {
                    polygons.push(close_polygon(&mut coords)?);
                }
                coords.push(Coord { x: to[0], y: to[1] });
            }
            PathSegment::Line { to } => coords.push(Coord { x: to[0], y: to[1] }),
            PathSegment::Close => {
                if !coords.is_empty() {
                    polygons.push(close_polygon(&mut coords)?);
                }
            }
            _ => {}
        }
    }
    if !coords.is_empty() {
        polygons.push(close_polygon(&mut coords)?);
    }
    if polygons.is_empty() {
        return Err(DrawingError::InvalidInput("boolean input needs a closed polygon".into()));
    }
    Ok(MultiPolygon::new(polygons))
}

fn ring_to_segments(ring: &LineString<f64>) -> Vec<PathSegment> {
    let mut segments = Vec::new();
    let mut first = true;
    for coord in &ring.0 {
        let point = [coord.x, coord.y];
        if first {
            segments.push(PathSegment::Move { to: point });
            first = false;
        } else {
            segments.push(PathSegment::Line { to: point });
        }
    }
    if !segments.is_empty() {
        segments.push(PathSegment::Close);
    }
    segments
}

fn polygon_to_segments(polygon: &Polygon<f64>) -> Vec<PathSegment> {
    let mut segments = ring_to_segments(polygon.exterior());
    for interior in polygon.interiors() {
        segments.extend(ring_to_segments(interior));
    }
    segments
}

pub fn boolean_paths(a: &[PathSegment], b: &[PathSegment], op: &str) -> Result<Vec<PathSegment>, DrawingError> {
    let poly_a = segments_to_multipolygon(a)?;
    let poly_b = segments_to_multipolygon(b)?;
    let result = match op {
        "union" => poly_a.union(&poly_b),
        "difference" => poly_a.difference(&poly_b),
        "intersection" => poly_a.intersection(&poly_b),
        "xor" => poly_a.xor(&poly_b),
        _ => return Err(DrawingError::InvalidInput(format!("unknown boolean op: {op}"))),
    };
    let mut segments = Vec::new();
    for polygon in result {
        segments.extend(polygon_to_segments(&polygon));
    }
    if segments.is_empty() {
        return Err(DrawingError::Operation("boolean produced empty path".into()));
    }
    Ok(segments)
}

pub fn boolean_paths_many(inputs: &[Vec<PathSegment>], op: &str) -> Result<Vec<PathSegment>, DrawingError> {
    if inputs.is_empty() {
        return Err(DrawingError::InvalidInput("boolean op needs at least one path".into()));
    }
    let mut acc = segments_to_multipolygon(&inputs[0])?;
    for next in inputs.iter().skip(1) {
        let poly_b = segments_to_multipolygon(next)?;
        acc = match op {
            "union" => acc.union(&poly_b),
            "difference" => acc.difference(&poly_b),
            "intersection" => acc.intersection(&poly_b),
            "xor" => acc.xor(&poly_b),
            _ => return Err(DrawingError::InvalidInput(format!("unknown boolean op: {op}"))),
        };
    }
    let mut segments = Vec::new();
    for polygon in acc.into_iter() {
        segments.extend(polygon_to_segments(&polygon));
    }
    if segments.is_empty() {
        return Err(DrawingError::Operation("boolean produced empty path".into()));
    }
    Ok(segments)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(origin: [f64; 2], size: f64) -> Vec<PathSegment> {
        vec![
            PathSegment::Move { to: origin },
            PathSegment::Line { to: [origin[0] + size, origin[1]] },
            PathSegment::Line { to: [origin[0] + size, origin[1] + size] },
            PathSegment::Line { to: [origin[0], origin[1] + size] },
            PathSegment::Close,
        ]
    }

    #[test]
    fn union_overlapping_rects() {
        let merged = boolean_paths(&square([0.0, 0.0], 10.0), &square([5.0, 5.0], 10.0), "union").expect("union");
        assert!(!merged.is_empty());
    }

    #[test]
    fn union_preserves_disconnected_input_contours() {
        let mut disconnected = square([20.0, 0.0], 5.0);
        disconnected.extend(square([30.0, 0.0], 5.0));
        let merged = boolean_paths(&square([0.0, 0.0], 5.0), &disconnected, "union").expect("union");
        assert_eq!(merged.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count(), 3);
    }
}
// #endregion booleans
}

#[cfg(feature = "trace")]
pub mod trace {
// #region trace
//! 🔍 Bitmap autotrace: marching squares contours + Douglas-Peucker simplification.

use kernel_2d_engine::{DrawingError, PathSegment, Vec2};
use std::collections::HashMap;

fn pixel_on(mask: &[u8], width: u32, x: i32, y: i32, threshold: u8) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    let ux = x as u32;
    let uy = y as u32;
    if ux >= width {
        return false;
    }
    let idx = (uy as usize) * (width as usize) + (ux as usize);
    mask.get(idx).copied().unwrap_or(0) >= threshold
}

fn marching_squares_contours(mask: &[u8], width: u32, height: u32, threshold: u8) -> Vec<Vec<Vec2>> {
    fn direction(start: [i32; 2], end: [i32; 2]) -> i32 {
        match [end[0] - start[0], end[1] - start[1]] {
            [1, 0] => 0,
            [0, 1] => 1,
            [-1, 0] => 2,
            _ => 3,
        }
    }

    let w = width as i32;
    let h = height as i32;
    let mut edges: Vec<([i32; 2], [i32; 2])> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if !pixel_on(mask, width, x, y, threshold) {
                continue;
            }
            if !pixel_on(mask, width, x, y - 1, threshold) {
                edges.push(([x, y], [x + 1, y]));
            }
            if !pixel_on(mask, width, x + 1, y, threshold) {
                edges.push(([x + 1, y], [x + 1, y + 1]));
            }
            if !pixel_on(mask, width, x, y + 1, threshold) {
                edges.push(([x + 1, y + 1], [x, y + 1]));
            }
            if !pixel_on(mask, width, x - 1, y, threshold) {
                edges.push(([x, y + 1], [x, y]));
            }
        }
    }

    let mut outgoing: HashMap<[i32; 2], Vec<usize>> = HashMap::new();
    for (index, (start, _)) in edges.iter().enumerate() {
        outgoing.entry(*start).or_default().push(index);
    }
    let mut used = vec![false; edges.len()];
    let mut contours: Vec<Vec<Vec2>> = Vec::new();
    for first in 0..edges.len() {
        if used[first] {
            continue;
        }
        let start = edges[first].0;
        let mut edge_index = first;
        let mut contour = vec![[start[0] as f64, start[1] as f64]];
        loop {
            used[edge_index] = true;
            let end = edges[edge_index].1;
            if end == start {
                break;
            }
            contour.push([end[0] as f64, end[1] as f64]);
            let incoming_direction = direction(edges[edge_index].0, end);
            let Some(next) = outgoing.get(&end).and_then(|indices| {
                indices
                    .iter()
                    .copied()
                    .filter(|index| !used[*index])
                    .min_by_key(|index| match (direction(end, edges[*index].1) - incoming_direction).rem_euclid(4) {
                        1 => 0,
                        0 => 1,
                        3 => 2,
                        _ => 3,
                    })
            }) else {
                contour.clear();
                break;
            };
            edge_index = next;
        }
        if contour.len() >= 3 {
            contours.push(contour);
        }
    }
    contours
}

fn perpendicular_distance(point: Vec2, line_start: Vec2, line_end: Vec2) -> f64 {
    let dx = line_end[0] - line_start[0];
    let dy = line_end[1] - line_start[1];
    if dx.abs() < f64::EPSILON && dy.abs() < f64::EPSILON {
        let px = point[0] - line_start[0];
        let py = point[1] - line_start[1];
        return (px * px + py * py).sqrt();
    }
    let t = ((point[0] - line_start[0]) * dx + (point[1] - line_start[1]) * dy) / (dx * dx + dy * dy);
    let proj = [line_start[0] + t * dx, line_start[1] + t * dy];
    let ox = point[0] - proj[0];
    let oy = point[1] - proj[1];
    (ox * ox + oy * oy).sqrt()
}

fn douglas_peucker(points: &[Vec2], epsilon: f64) -> Vec<Vec2> {
    if points.len() < 3 || epsilon <= 0.0 {
        return points.to_vec();
    }
    let start = points[0];
    let end = points[points.len() - 1];
    let mut max_dist = 0.0_f64;
    let mut index = 0_usize;
    for (i, point) in points.iter().enumerate().skip(1).take(points.len().saturating_sub(2)) {
        let dist = perpendicular_distance(*point, start, end);
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
    }
    if max_dist > epsilon {
        let mut left = douglas_peucker(&points[..=index], epsilon);
        let right = douglas_peucker(&points[index..], epsilon);
        left.pop();
        left.extend(right);
        left
    } else {
        vec![start, end]
    }
}

fn contour_to_segments(points: &[Vec2]) -> Vec<PathSegment> {
    if points.is_empty() {
        return Vec::new();
    }
    let mut segments = vec![PathSegment::Move { to: points[0] }];
    for point in points.iter().skip(1) {
        segments.push(PathSegment::Line { to: *point });
    }
    if points.len() > 2 {
        segments.push(PathSegment::Close);
    }
    segments
}

pub fn trace_bitmap_paths(width: u32, height: u32, mask_or_luma: &[u8], threshold: f64, simplify_epsilon: f64) -> Result<Vec<PathSegment>, DrawingError> {
    if width == 0 || height == 0 {
        return Err(DrawingError::InvalidInput("trace bitmap needs non-zero dimensions".into()));
    }
    let expected = (width as usize).saturating_mul(height as usize);
    if mask_or_luma.len() < expected {
        return Err(DrawingError::InvalidInput(format!("trace bitmap expects {expected} bytes, got {}", mask_or_luma.len())));
    }
    let threshold_u8 = (threshold.clamp(0.0, 1.0) * 255.0).round() as u8;
    let contours = marching_squares_contours(&mask_or_luma[..expected], width, height, threshold_u8);
    if contours.is_empty() {
        return Err(DrawingError::Operation("trace produced no contours".into()));
    }
    let mut segments: Vec<PathSegment> = Vec::new();
    for contour in contours {
        let simplified = douglas_peucker(&contour, simplify_epsilon.max(0.0));
        segments.extend(contour_to_segments(&simplified));
    }
    if segments.is_empty() {
        return Err(DrawingError::Operation("trace produced no segments".into()));
    }
    Ok(segments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traces_filled_square() {
        let width = 8_u32;
        let height = 8_u32;
        let mut mask = vec![0_u8; (width * height) as usize];
        for y in 2..6 {
            for x in 2..6 {
                mask[(y * width + x) as usize] = 255;
            }
        }
        let segments = trace_bitmap_paths(width, height, &mask, 0.5, 0.5).expect("trace");
        assert_eq!(segments.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count(), 1);
        assert!(segments.len() <= 6, "a solid square must trace its boundary without interior scanlines");
    }

    #[test]
    fn traces_disjoint_regions_with_move_per_contour() {
        let width = 10_u32;
        let height = 10_u32;
        let mut mask = vec![0_u8; (width * height) as usize];
        for y in 1..3 {
            for x in 1..3 {
                mask[(y * width + x) as usize] = 255;
            }
        }
        for y in 6..8 {
            for x in 6..8 {
                mask[(y * width + x) as usize] = 255;
            }
        }
        let segments = trace_bitmap_paths(width, height, &mask, 0.5, 0.5).expect("trace");
        let moves = segments
            .iter()
            .filter(|segment| matches!(segment, PathSegment::Move { .. }))
            .count();
        assert!(moves >= 2, "each disjoint contour must start with its own move");
    }

    #[test]
    fn traces_corner_touching_regions_as_separate_contours() {
        let mask = [255_u8, 0, 0, 255];
        let segments = trace_bitmap_paths(2, 2, &mask, 0.5, 0.0).expect("trace");
        assert_eq!(segments.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count(), 2);
    }
}
// #endregion trace
}


use async_trait::async_trait;
use kernel_2d_engine::{
    Affine2D, DrawingError, DrawingHandle, DrawingKernel, DrawingKind, DrawingNode, DrawingScene, FillStyle, GradientStop, PathSegment, SceneNode, StrokeStyle, Vec2,
};

// #region 🔖Store
#[derive(Clone)]
struct StoredNode {
    kind: DrawingKind,
    node: DrawingNode,
    transform: Affine2D,
    fill: Option<FillStyle>,
    stroke: Option<StrokeStyle>,
    clip: Option<Vec<PathSegment>>,
    opacity: f64,
}

/// 🗄️ Scene-graph drawing store with `drawing-*` handles.
pub struct DrawingStore {
    seq: u32,
    registry: std::collections::HashMap<String, StoredNode>,
}

impl Default for DrawingStore {
    fn default() -> Self {
        Self::new()
    }
}

impl DrawingStore {
    pub fn new() -> Self {
        Self { seq: 0, registry: std::collections::HashMap::new() }
    }

    fn register(&mut self, kind: DrawingKind, node: DrawingNode) -> DrawingHandle {
        self.seq += 1;
        let handle = DrawingHandle::new(kind, self.seq);
        self.registry.insert(
            handle.as_str().to_string(),
            StoredNode { kind, node, transform: Affine2D::identity(), fill: None, stroke: None, clip: None, opacity: 1.0 },
        );
        handle
    }

    fn fork(&mut self, source: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        let entry = self.entry(source)?.clone();
        self.seq += 1;
        let handle = DrawingHandle::new(entry.kind, self.seq);
        self.registry.insert(handle.as_str().to_string(), entry);
        Ok(handle)
    }

    fn entry(&self, handle: &DrawingHandle) -> Result<&StoredNode, DrawingError> {
        self.registry.get(handle.as_str()).ok_or_else(|| DrawingError::MissingHandle(handle.as_str().to_string()))
    }

    fn entry_mut(&mut self, handle: &DrawingHandle) -> Result<&mut StoredNode, DrawingError> {
        self.registry.get_mut(handle.as_str()).ok_or_else(|| DrawingError::MissingHandle(handle.as_str().to_string()))
    }

    pub fn registry_len(&self) -> usize {
        self.registry.len()
    }

    pub fn dispose_sync(&mut self, handle: &DrawingHandle) {
        self.registry.remove(handle.as_str());
    }

    pub fn retain_sync(&mut self, live: &std::collections::HashSet<String>) {
        self.registry.retain(|handle, _| live.contains(handle));
    }

    fn node_to_segments(node: &DrawingNode) -> Vec<PathSegment> {
        match node {
            DrawingNode::Rect { x, y, width, height } => rect_segments(*x, *y, *width, *height),
            DrawingNode::Ellipse { cx, cy, rx, ry } => ellipse_segments(*cx, *cy, *rx, *ry),
            DrawingNode::Circle { cx, cy, r } => ellipse_segments(*cx, *cy, *r, *r),
            DrawingNode::Line { x1, y1, x2, y2 } => vec![PathSegment::Move { to: [*x1, *y1] }, PathSegment::Line { to: [*x2, *y2] }],
            DrawingNode::Polygon { points } => polygon_segments(points),
            DrawingNode::Path { segments } => segments.clone(),
            DrawingNode::Text { .. } => Vec::new(),
            DrawingNode::Group { .. } => Vec::new(),
        }
    }

    fn flatten_handle(&self, handle: &DrawingHandle, parent: Affine2D) -> Result<Vec<SceneNode>, DrawingError> {
        let entry = self.entry(handle)?;
        let transform = parent.multiply(entry.transform);
        match &entry.node {
            DrawingNode::Group { children } => children.iter().try_fold(Vec::new(), |mut acc, child| {
                let nested = self.flatten_handle(&DrawingHandle(child.clone()), transform)?;
                acc.extend(nested);
                Ok::<_, DrawingError>(acc)
            }),
            _ => Ok(vec![SceneNode {
                transform,
                node: entry.node.clone(),
                fill: entry.fill.clone(),
                stroke: entry.stroke.clone(),
                opacity: entry.opacity,
                clip: entry.clip.clone(),
            }]),
        }
    }

    pub fn flatten_scene_sync(&self, handle: &DrawingHandle) -> Result<DrawingScene, DrawingError> {
        let nodes = self.flatten_handle(handle, Affine2D::identity())?;
        let (width, height) = scene_bounds(&nodes);
        Ok(DrawingScene { width, height, nodes })
    }

    pub fn export_svg_sync(&self, handle: &DrawingHandle) -> Result<String, DrawingError> {
        let scene = self.flatten_scene_sync(handle)?;
        Ok(serialize_svg(&scene))
    }

    pub fn export_pdf_sync(&self, handle: &DrawingHandle) -> Result<Vec<u8>, DrawingError> {
        let scene = self.flatten_scene_sync(handle)?;
        Ok(serialize_pdf(&scene))
    }
}
// #endregion 🔖Store

// #region 🔖Geometry
fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Vec<PathSegment> {
    vec![
        PathSegment::Move { to: [x, y] },
        PathSegment::Line { to: [x + width, y] },
        PathSegment::Line { to: [x + width, y + height] },
        PathSegment::Line { to: [x, y + height] },
        PathSegment::Close,
    ]
}

fn polygon_segments(points: &[Vec2]) -> Vec<PathSegment> {
    let mut segments = Vec::new();
    if let Some(first) = points.first() {
        segments.push(PathSegment::Move { to: *first });
        for point in points.iter().skip(1) {
            segments.push(PathSegment::Line { to: *point });
        }
        if points.len() > 2 {
            segments.push(PathSegment::Close);
        }
    }
    segments
}

fn ellipse_segments(cx: f64, cy: f64, rx: f64, ry: f64) -> Vec<PathSegment> {
    const K: f64 = 0.5522847498;
    let ox = rx * K;
    let oy = ry * K;
    vec![
        PathSegment::Move { to: [cx, cy - ry] },
        PathSegment::Cubic { ctrl1: [cx + ox, cy - ry], ctrl2: [cx + rx, cy - oy], to: [cx + rx, cy] },
        PathSegment::Cubic { ctrl1: [cx + rx, cy + oy], ctrl2: [cx + ox, cy + ry], to: [cx, cy + ry] },
        PathSegment::Cubic { ctrl1: [cx - ox, cy + ry], ctrl2: [cx - rx, cy + oy], to: [cx - rx, cy] },
        PathSegment::Cubic { ctrl1: [cx - rx, cy - oy], ctrl2: [cx - ox, cy - ry], to: [cx, cy - ry] },
        PathSegment::Close,
    ]
}

fn polyline_segments(points: &[Vec2]) -> Vec<PathSegment> {
    let mut segments = Vec::new();
    if let Some(first) = points.first() {
        segments.push(PathSegment::Move { to: *first });
        for point in points.iter().skip(1) {
            segments.push(PathSegment::Line { to: *point });
        }
    }
    segments
}

fn scene_bounds(nodes: &[SceneNode]) -> (f64, f64) {
    let mut max_x = 512.0_f64;
    let mut max_y = 512.0_f64;
    for node in nodes {
        match &node.node {
            DrawingNode::Rect { x, y, width, height } => {
                max_x = max_x.max(x + width);
                max_y = max_y.max(y + height);
            }
            DrawingNode::Circle { cx, cy, r } => {
                max_x = max_x.max(cx + r);
                max_y = max_y.max(cy + r);
            }
            DrawingNode::Ellipse { cx, cy, rx, ry } => {
                max_x = max_x.max(cx + rx);
                max_y = max_y.max(cy + ry);
            }
            DrawingNode::Text { x, y, size, .. } => {
                max_x = max_x.max(x + size * 4.0);
                max_y = max_y.max(y + size);
            }
            _ => {}
        }
    }
    (max_x.max(1.0), max_y.max(1.0))
}
// #endregion 🔖Geometry

// #region 🔖Export
fn color_css(color: [f64; 4]) -> String {
    let r = (color[0] * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = (color[1] * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = (color[2] * 255.0).round().clamp(0.0, 255.0) as u8;
    let a = color[3].clamp(0.0, 1.0);
    if (a - 1.0).abs() < f64::EPSILON {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        format!("rgba({r},{g},{b},{a:.3})")
    }
}

fn segments_to_svg_d(segments: &[PathSegment]) -> String {
    let mut out = String::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => out.push_str(&format!("M {} {} ", to[0], to[1])),
            PathSegment::Line { to } => out.push_str(&format!("L {} {} ", to[0], to[1])),
            PathSegment::Quad { ctrl, to } => out.push_str(&format!("Q {} {} {} {} ", ctrl[0], ctrl[1], to[0], to[1])),
            PathSegment::Cubic { ctrl1, ctrl2, to } => out.push_str(&format!("C {} {} {} {} {} {} ", ctrl1[0], ctrl1[1], ctrl2[0], ctrl2[1], to[0], to[1])),
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                out.push_str(&format!("A {} {} {} {} {} {} {} ", rx, ry, rotation, if *large_arc { 1 } else { 0 }, if *sweep { 1 } else { 0 }, to[0], to[1]));
            }
            PathSegment::Close => out.push('Z'),
        }
    }
    out.trim().to_string()
}

fn serialize_svg(scene: &DrawingScene) -> String {
    let mut body = String::new();
    for node in &scene.nodes {
        match &node.node {
            DrawingNode::Text { x, y, content, size } => {
                let [tx, ty] = node.transform.transform_point([*x, *y]);
                let fill = node.fill.as_ref().map(|f| match f {
                    FillStyle::Solid { color } => color_css(*color),
                    _ => "black".into(),
                }).unwrap_or_else(|| "black".into());
                body.push_str(&format!(r#"<text x="{tx}" y="{ty}" font-size="{size}" fill="{fill}">{content}</text>"#));
            }
            _ => {
                let segments = DrawingStore::node_to_segments(&node.node);
                if segments.is_empty() {
                    continue;
                }
                let d = segments_to_svg_d(&segments);
                let mut attrs = format!(r#"d="{d}""#);
                if let Some(fill) = &node.fill {
                    match fill {
                        FillStyle::Solid { color } => attrs.push_str(&format!(r#" fill="{}""#, color_css(*color))),
                        FillStyle::LinearGradient { x1, y1, x2, y2, stops } => {
                            let id = format!("lg{}", body.len());
                            body.push_str(&format!(r#"<defs><linearGradient id="{id}" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}">"#));
                            for stop in stops {
                                body.push_str(&format!(r#"<stop offset="{}" stop-color="{}"/>"#, stop.offset, color_css(stop.color)));
                            }
                            body.push_str("</linearGradient></defs>");
                            attrs.push_str(&format!(r#" fill="url(#{id})""#));
                        }
                        FillStyle::RadialGradient { cx, cy, r, stops } => {
                            let id = format!("rg{}", body.len());
                            body.push_str(&format!(r#"<defs><radialGradient id="{id}" cx="{cx}" cy="{cy}" r="{r}">"#));
                            for stop in stops {
                                body.push_str(&format!(r#"<stop offset="{}" stop-color="{}"/>"#, stop.offset, color_css(stop.color)));
                            }
                            body.push_str("</radialGradient></defs>");
                            attrs.push_str(&format!(r#" fill="url(#{id})""#));
                        }
                    }
                } else {
                    attrs.push_str(r#" fill="none""#);
                }
                if let Some(stroke) = &node.stroke {
                    attrs.push_str(&format!(r#" stroke="{}" stroke-width="{}""#, color_css(stroke.color), stroke.width));
                }
                body.push_str(&format!("<path {attrs}/>"));
            }
        }
    }
    format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">{body}</svg>"#, scene.width, scene.height, scene.width, scene.height)
}

fn segments_to_pdf_ops(segments: &[PathSegment]) -> String {
    let mut out = String::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => out.push_str(&format!("{} {} m\n", to[0], to[1])),
            PathSegment::Line { to } => out.push_str(&format!("{} {} l\n", to[0], to[1])),
            PathSegment::Close => out.push_str("h\n"),
            _ => {}
        }
    }
    out
}

fn serialize_pdf(scene: &DrawingScene) -> Vec<u8> {
    let mut stream = String::new();
    stream.push_str("0.1 0.1 0.1 rg\n");
    for node in &scene.nodes {
        let segments = DrawingStore::node_to_segments(&node.node);
        if segments.is_empty() {
            continue;
        }
        stream.push_str(&segments_to_pdf_ops(&segments));
        if node.fill.is_some() {
            stream.push_str("f\n");
        } else if node.stroke.is_some() {
            stream.push_str("S\n");
        }
    }
    let content = stream.as_bytes();
    let objects = [
        "1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n",
        "2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n",
        &format!("3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents 4 0 R /Resources << >> >>\nendobj\n", scene.width, scene.height),
        &format!("4 0 obj\n<< /Length {} >>\nstream\n", content.len()),
    ];
    let mut pdf = String::from("%PDF-1.4\n");
    let mut offsets = Vec::new();
    for object in &objects[..3] {
        offsets.push(pdf.len());
        pdf.push_str(object);
    }
    offsets.push(pdf.len());
    pdf.push_str(&objects[3]);
    pdf.push_str(std::str::from_utf8(content).unwrap_or(""));
    pdf.push_str("\nendstream\nendobj\n");
    let xref = pdf.len();
    pdf.push_str(&format!("xref\n0 5\n0000000000 65535 f \n"));
    for offset in offsets {
        pdf.push_str(&format!("{:010} 00000 n \n", offset));
    }
    pdf.push_str(&format!("trailer\n<< /Size 5 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n"));
    pdf.into_bytes()
}
// #endregion 🔖Export

// #region 🔖Kernel
#[async_trait(?Send)]
impl DrawingKernel for DrawingStore {
    async fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Rect, DrawingNode::Rect { x, y, width, height }))
    }

    async fn ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Ellipse, DrawingNode::Ellipse { cx, cy, rx, ry }))
    }

    async fn circle(&mut self, cx: f64, cy: f64, r: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Circle, DrawingNode::Circle { cx, cy, r }))
    }

    async fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Line, DrawingNode::Line { x1, y1, x2, y2 }))
    }

    async fn polygon(&mut self, points: &[Vec2]) -> Result<DrawingHandle, DrawingError> {
        if points.len() < 3 {
            return Err(DrawingError::InvalidInput("polygon needs at least 3 points".into()));
        }
        Ok(self.register(DrawingKind::Polygon, DrawingNode::Polygon { points: points.to_vec() }))
    }

    async fn polyline_path(&mut self, points: &[Vec2]) -> Result<DrawingHandle, DrawingError> {
        if points.len() < 2 {
            return Err(DrawingError::InvalidInput("polyline needs at least 2 points".into()));
        }
        Ok(self.register(DrawingKind::Path, DrawingNode::Path { segments: polyline_segments(points) }))
    }

    async fn rect_path(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Path, DrawingNode::Path { segments: rect_segments(x, y, width, height) }))
    }

    async fn set_fill(&mut self, handle: &DrawingHandle, fill: FillStyle) -> Result<DrawingHandle, DrawingError> {
        let next = self.fork(handle)?;
        self.entry_mut(&next)?.fill = Some(fill);
        Ok(next)
    }

    async fn set_stroke(&mut self, handle: &DrawingHandle, stroke: StrokeStyle) -> Result<DrawingHandle, DrawingError> {
        let next = self.fork(handle)?;
        self.entry_mut(&next)?.stroke = Some(stroke);
        Ok(next)
    }

    async fn linear_gradient_fill(&mut self, handle: &DrawingHandle, x1: f64, y1: f64, x2: f64, y2: f64, stops: &[GradientStop]) -> Result<DrawingHandle, DrawingError> {
        self.set_fill(handle, FillStyle::LinearGradient { x1, y1, x2, y2, stops: stops.to_vec() }).await
    }

    async fn translate(&mut self, handle: &DrawingHandle, dx: f64, dy: f64) -> Result<DrawingHandle, DrawingError> {
        let next = self.fork(handle)?;
        let transform = self.entry(&next)?.transform;
        self.entry_mut(&next)?.transform = transform.multiply(Affine2D::translate(dx, dy));
        Ok(next)
    }

    async fn rotate(&mut self, handle: &DrawingHandle, angle: f64) -> Result<DrawingHandle, DrawingError> {
        let next = self.fork(handle)?;
        let transform = self.entry(&next)?.transform;
        self.entry_mut(&next)?.transform = transform.multiply(Affine2D::rotate(angle));
        Ok(next)
    }

    async fn scale(&mut self, handle: &DrawingHandle, sx: f64, sy: f64) -> Result<DrawingHandle, DrawingError> {
        let next = self.fork(handle)?;
        let transform = self.entry(&next)?.transform;
        self.entry_mut(&next)?.transform = transform.multiply(Affine2D::scale(sx, sy));
        Ok(next)
    }

    async fn group(&mut self, children: &[DrawingHandle]) -> Result<DrawingHandle, DrawingError> {
        if children.is_empty() {
            return Err(DrawingError::InvalidInput("group needs children".into()));
        }
        let handles: Vec<String> = children.iter().map(|h| h.as_str().to_string()).collect();
        Ok(self.register(DrawingKind::Group, DrawingNode::Group { children: handles }))
    }

    async fn bool_union(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        self.bool_op(a, b, "union").await
    }

    async fn bool_difference(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        self.bool_op(a, b, "difference").await
    }

    async fn bool_intersection(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        self.bool_op(a, b, "intersection").await
    }

    async fn bool_xor(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        self.bool_op(a, b, "xor").await
    }

    async fn bool_op_many(&mut self, op: &str, handles: &[DrawingHandle]) -> Result<DrawingHandle, DrawingError> {
        if handles.is_empty() {
            return Err(DrawingError::InvalidInput("boolean op needs at least one handle".into()));
        }
        if handles.len() == 1 {
            return self.fork(&handles[0]);
        }
        let mut segments_list: Vec<Vec<PathSegment>> = Vec::new();
        for handle in handles {
            segments_list.push(DrawingStore::node_to_segments(&self.entry(handle)?.node));
        }
        #[cfg(feature = "booleans")]
        {
            let merged = booleans::boolean_paths_many(&segments_list, op)?;
            return Ok(self.register(DrawingKind::Path, DrawingNode::Path { segments: merged }));
        }
        #[cfg(not(feature = "booleans"))]
        {
            let _ = (segments_list, op);
            Err(DrawingError::Operation("boolean ops require booleans feature".into()))
        }
    }

    async fn boolean_segments(&self, a: &[PathSegment], b: &[PathSegment], op: &str) -> Result<Vec<PathSegment>, DrawingError> {
        #[cfg(feature = "booleans")]
        {
            return booleans::boolean_paths(a, b, op);
        }
        #[cfg(not(feature = "booleans"))]
        {
            let _ = (a, b, op);
            Err(DrawingError::Operation("boolean ops require booleans feature".into()))
        }
    }

    async fn trace_bitmap(&mut self, width: u32, height: u32, mask_or_luma: &[u8], threshold: f64, simplify_epsilon: f64) -> Result<DrawingHandle, DrawingError> {
        #[cfg(feature = "trace")]
        {
            let segments = trace::trace_bitmap_paths(width, height, mask_or_luma, threshold, simplify_epsilon)?;
            return Ok(self.register(DrawingKind::Path, DrawingNode::Path { segments }));
        }
        #[cfg(not(feature = "trace"))]
        {
            let _ = (width, height, mask_or_luma, threshold, simplify_epsilon);
            Err(DrawingError::Operation("trace requires trace feature".into()))
        }
    }

    async fn text(&mut self, x: f64, y: f64, content: &str, size: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Text, DrawingNode::Text { x, y, content: content.to_string(), size }))
    }

    async fn apply_clip(&mut self, target: &DrawingHandle, clip: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        let clip_segments = DrawingStore::node_to_segments(&self.entry(clip)?.node);
        let next = self.fork(target)?;
        self.entry_mut(&next)?.clip = Some(clip_segments);
        Ok(next)
    }

    async fn flatten_scene(&self, handle: &DrawingHandle) -> Result<DrawingScene, DrawingError> {
        self.flatten_scene_sync(handle)
    }

    async fn export_svg(&self, handle: &DrawingHandle) -> Result<String, DrawingError> {
        self.export_svg_sync(handle)
    }

    async fn export_pdf(&self, handle: &DrawingHandle) -> Result<Vec<u8>, DrawingError> {
        self.export_pdf_sync(handle)
    }

    async fn kind(&self, handle: &DrawingHandle) -> Result<DrawingKind, DrawingError> {
        Ok(self.entry(handle)?.kind)
    }

    async fn dispose(&mut self, handle: &DrawingHandle) {
        self.dispose_sync(handle);
    }

    fn retain_sync(&mut self, live: &std::collections::HashSet<String>) {
        DrawingStore::retain_sync(self, live);
    }
}

impl DrawingStore {
    async fn bool_op(&mut self, a: &DrawingHandle, b: &DrawingHandle, op: &str) -> Result<DrawingHandle, DrawingError> {
        let a_segments = DrawingStore::node_to_segments(&self.entry(a)?.node);
        let b_segments = DrawingStore::node_to_segments(&self.entry(b)?.node);
        #[cfg(feature = "booleans")]
        {
            let merged = booleans::boolean_paths(&a_segments, &b_segments, op)?;
            return Ok(self.register(DrawingKind::Path, DrawingNode::Path { segments: merged }));
        }
        #[cfg(not(feature = "booleans"))]
        {
            let _ = (a_segments, b_segments, op);
            Err(DrawingError::Operation("boolean ops require booleans feature".into()))
        }
    }
}
// #endregion 🔖Kernel

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use kernel_2d_engine::block_on;

    #[test]
    fn rect_exports_svg() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 10.0, 20.0)).expect("rect");
        let svg = store.export_svg_sync(&rect).expect("svg");
        assert!(svg.contains("<svg"));
        assert!(svg.contains("10"));
    }

    #[test]
    fn rect_exports_pdf() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 10.0, 20.0)).expect("rect");
        let pdf = store.export_pdf_sync(&rect).expect("pdf");
        assert!(pdf.starts_with(b"%PDF"));
    }

    #[test]
    fn group_flattens_children() {
        let mut store = DrawingStore::new();
        let a = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let b = block_on(store.circle(10.0, 10.0, 3.0)).unwrap();
        let group = block_on(store.group(&[a, b])).unwrap();
        let scene = store.flatten_scene_sync(&group).unwrap();
        assert_eq!(scene.nodes.len(), 2);
    }
}
// #endregion 🔖Tests
