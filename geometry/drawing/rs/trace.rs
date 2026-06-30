//! 🔍 Bitmap autotrace: marching squares contours + Douglas-Peucker simplification.

use geometry_drawing_engine::{DrawingError, PathSegment, Vec2};

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
    let w = width as i32;
    let h = height as i32;
    let mut contours: Vec<Vec<Vec2>> = Vec::new();
    let mut visited = vec![false; (width as usize).saturating_mul(height as usize)];
    for y in 0..h {
        for x in 0..w {
            let idx = (y as usize) * (width as usize) + (x as usize);
            if visited.get(idx).copied().unwrap_or(true) || !pixel_on(mask, width, x, y, threshold) {
                continue;
            }
            let mut contour: Vec<Vec2> = Vec::new();
            let mut cx = x;
            let mut cy = y;
            let start = [cx as f64 + 0.5, cy as f64 + 0.5];
            contour.push(start);
            visited[idx] = true;
            let mut guard = 0;
            while guard < 4096 {
                guard += 1;
                let mut moved = false;
                for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let nx = cx + dx;
                    let ny = cy + dy;
                    if nx < 0 || ny < 0 || nx >= w || ny >= h {
                        continue;
                    }
                    let nidx = (ny as usize) * (width as usize) + (nx as usize);
                    if pixel_on(mask, width, nx, ny, threshold) && !visited[nidx] {
                        cx = nx;
                        cy = ny;
                        visited[nidx] = true;
                        contour.push([cx as f64 + 0.5, cy as f64 + 0.5]);
                        moved = true;
                        break;
                    }
                }
                if !moved {
                    break;
                }
                if contour.len() > 3 {
                    let last = contour[contour.len() - 1];
                    if (last[0] - start[0]).abs() < f64::EPSILON && (last[1] - start[1]).abs() < f64::EPSILON {
                        break;
                    }
                }
            }
            if contour.len() >= 3 {
                contours.push(contour);
            }
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
    let mut merged: Vec<Vec2> = Vec::new();
    for contour in contours {
        let simplified = douglas_peucker(&contour, simplify_epsilon.max(0.0));
        if merged.is_empty() {
            merged = simplified;
        } else {
            for point in simplified.iter().skip(1) {
                merged.push(*point);
            }
        }
    }
    Ok(contour_to_segments(&merged))
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
        assert!(!segments.is_empty());
    }
}
