//! 🔲️ Recover a silhouette when cancelling the oriented projected triangle edges leaves one simple ring.

use std::collections::{BTreeMap, BTreeSet};

type Point = [f64; 2];
type Key = [u64; 2];

fn key(point: Point) -> Key {
    point.map(|v| if v == 0.0 { 0 } else { v.to_bits() })
}

fn separated_edges(ring: &[Point]) -> bool {
    let n = ring.len();
    let mut boxes: Vec<_> = (0..n).map(|i| {
        let [a, b] = [ring[i], ring[(i + 1) % n]];
        (a[0].min(b[0]), a[0].max(b[0]), a[1].min(b[1]), a[1].max(b[1]), i)
    }).collect();
    boxes.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (k, &(_, right, low, high, i)) in boxes.iter().enumerate() {
        for &(left, _, bottom, top, j) in &boxes[k + 1..] {
            if left > right { break; }
            if (i + 1) % n != j && (j + 1) % n != i && low <= top && bottom <= high { return false; }
        }
    }
    true
}

/// 🧵 A single simple boundary has winding one inside and zero outside, exactly the union of its positively oriented input triangles.
pub(super) fn single_front_outline(triangles: impl Iterator<Item = [Point; 3]>) -> Option<Vec<Point>> {
    let mut edges = BTreeSet::new();
    for [a, mut b, mut c] in triangles {
        let cross = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
        if !cross.is_finite() || cross == 0.0 { return None; }
        if cross < 0.0 { std::mem::swap(&mut b, &mut c); }
        for (start, end) in [(a,b), (b,c), (c,a)] {
            if !edges.insert((key(start), key(end))) { return None; }
        }
    }
    let mut next = BTreeMap::new();
    for &(start, end) in &edges {
        if !edges.contains(&(end, start)) && next.insert(start, end).is_some() { return None; }
    }
    let start = *next.first_key_value()?.0;
    let mut current = start;
    let mut ring = Vec::with_capacity(next.len());
    loop {
        ring.push(current.map(f64::from_bits));
        current = next.remove(&current)?;
        if current == start { break; }
    }
    if !next.is_empty() || ring.len() < 3 || !separated_edges(&ring) { return None; }
    let first = (0..ring.len()).min_by(|&a, &b| ring[a][0].total_cmp(&ring[b][0]).then_with(|| ring[a][1].total_cmp(&ring[b][1])))?;
    ring.rotate_left(first);
    Some(ring)
}
