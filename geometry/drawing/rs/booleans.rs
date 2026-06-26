//! 🔀 Planar path boolean operations (optional `booleans` feature).

use geometry_drawing_engine::{DrawingError, PathSegment};
use geo::{BooleanOps, Coord, LineString, Polygon};

fn segments_to_polygon(segments: &[PathSegment]) -> Result<Polygon<f64>, DrawingError> {
    let mut coords: Vec<Coord<f64>> = Vec::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } | PathSegment::Line { to } => coords.push(Coord { x: to[0], y: to[1] }),
            PathSegment::Close => {}
            _ => {}
        }
    }
    if coords.len() < 3 {
        return Err(DrawingError::InvalidInput("boolean input needs a closed polygon".into()));
    }
    if coords.first() != coords.last() {
        coords.push(coords[0]);
    }
    Ok(Polygon::new(LineString(coords), vec![]))
}

fn polygon_to_segments(polygon: &Polygon<f64>) -> Vec<PathSegment> {
    let mut segments = Vec::new();
    let mut first = true;
    for coord in polygon.exterior().0.iter() {
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

pub fn boolean_paths(a: &[PathSegment], b: &[PathSegment], op: &str) -> Result<Vec<PathSegment>, DrawingError> {
    let poly_a = segments_to_polygon(a)?;
    let poly_b = segments_to_polygon(b)?;
    let result = match op {
        "union" => poly_a.union(&poly_b),
        "difference" => poly_a.difference(&poly_b),
        "intersection" => poly_a.intersection(&poly_b),
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
}
