//! 🔗️ Partner surfaces. A partition between two spaces is a surface in each of them (`adjacent` names the neighbour); gbXML and the energy model want the two to point at each other. The partners are found among the
//! surfaces of the two spaces that face each other: the two sides of one wall (same wall element), or a floor and the ceiling under it (the same outline, so the same centre and area). The closest centres pair first, so
//! a wall cut in several pieces by its neighbours pairs piece by piece.

/// 🧱️ What a surface is for pairing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Wall,
    Floor,
    Ceiling,
}

/// 🔗️ One surface that has a neighbour.
#[derive(Clone, Debug, PartialEq)]
pub struct Face {
    pub space: String,
    pub adjacent: String,
    pub part: Part,
    pub element: String,
    pub centre: [f64; 3],
    pub area: f64,
}

/// 📐️ The centre of a polygon (the mean of its vertices).
pub fn centre_of(polygon: &[[f64; 3]]) -> [f64; 3] {
    let count = polygon.len().max(1) as f64;
    let mut sum = [0.0; 3];
    for vertex in polygon {
        for (total, value) in sum.iter_mut().zip(vertex) {
            *total += value;
        }
    }
    sum.map(|total| total / count)
}

fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.iter().zip(b).map(|(left, right)| (left - right).powi(2)).sum::<f64>().sqrt()
}

fn fits(a: &Face, b: &Face) -> bool {
    if a.adjacent != b.space || b.adjacent != a.space {
        return false;
    }
    match (a.part, b.part) {
        (Part::Wall, Part::Wall) => a.element == b.element,
        (Part::Floor, Part::Ceiling) | (Part::Ceiling, Part::Floor) => distance([a.centre[0], a.centre[1], 0.0], [b.centre[0], b.centre[1], 0.0]) <= 1e-3 && (a.area - b.area).abs() <= 1e-6 * a.area.abs().max(1.0),
        _ => false,
    }
}

/// 🔗️ The partners among `faces` as pairs of indices `(low, high)`, in ascending order; a face without a partner is in no pair.
pub fn pair(faces: &[Face]) -> Vec<(usize, usize)> {
    let mut candidates: Vec<(f64, usize, usize)> = Vec::new();
    for (i, a) in faces.iter().enumerate() {
        for (j, b) in faces.iter().enumerate().skip(i + 1) {
            if fits(a, b) {
                candidates.push((distance(a.centre, b.centre) + (a.area - b.area).abs(), i, j));
            }
        }
    }
    candidates.sort_by(|left, right| left.0.total_cmp(&right.0).then(left.1.cmp(&right.1)).then(left.2.cmp(&right.2)));
    let mut taken = vec![false; faces.len()];
    let mut pairs = Vec::new();
    for (_, i, j) in candidates {
        if !taken[i] && !taken[j] {
            taken[i] = true;
            taken[j] = true;
            pairs.push((i, j));
        }
    }
    pairs.sort_unstable();
    pairs
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
