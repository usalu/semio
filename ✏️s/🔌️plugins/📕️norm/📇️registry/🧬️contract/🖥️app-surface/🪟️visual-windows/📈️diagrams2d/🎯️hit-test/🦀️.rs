//! 🎯️ Hit-testing of lowered diagrams: pointer and keyboard picking resolved to selection events. See [`super`].

use super::*;

/// 🧲️ What kind of diagram element a pick target stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementKind {
    Series,
    Marker,
    Note,
    Legend,
}

/// 📐️ Screen-space geometry of one pick target in canvas pixels.
#[derive(Clone, Debug, PartialEq)]
pub enum PickShape {
    Point { at: [f64; 2], radius: f64 },
    Path { points: Vec<[f64; 2]> },
    Area { ring: Vec<[f64; 2]> },
    Rect { min: [f64; 2], max: [f64; 2] },
}

/// 🎯️ One pickable element with the data coordinate it reports to the tooltip.
#[derive(Clone, Debug, PartialEq)]
pub struct PickTarget {
    pub element: String,
    pub kind: ElementKind,
    pub index: Option<usize>,
    pub shape: PickShape,
    pub data: Option<[f64; 2]>,
}

/// 🖱️ The closest target under a pointer.
#[derive(Clone, Debug, PartialEq)]
pub struct Hit {
    pub target: PickTarget,
    pub distance: f64,
}

/// ✅️ Selection event of a diagram: which element, check and entity the user picked.
#[derive(Clone, Debug, PartialEq)]
pub struct DiagramSelection {
    pub diagram_id: String,
    pub element: String,
    pub kind: ElementKind,
    pub index: Option<usize>,
    pub check_id: Option<String>,
    pub entity: Option<EntityRef>,
}

fn segment_distance(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let d = [b[0] - a[0], b[1] - a[1]];
    let len2 = d[0] * d[0] + d[1] * d[1];
    let t = if len2 < 1e-18 { 0.0 } else { (((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / len2).clamp(0.0, 1.0) };
    (p[0] - a[0] - t * d[0]).hypot(p[1] - a[1] - t * d[1])
}

/// ⭕️ Even-odd containment of `p` in a ring of screen points.
pub fn ring_contains(ring: &[[f64; 2]], p: [f64; 2]) -> bool {
    let n = ring.len();
    let mut inside = false;
    for i in 0..n {
        let (a, b) = (ring[i], ring[(i + 1) % n]);
        if (a[1] > p[1]) != (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0] {
            inside = !inside;
        }
    }
    inside
}

fn priority(target: &PickTarget) -> u8 {
    match (&target.kind, &target.shape) {
        (ElementKind::Marker, _) => 0,
        (_, PickShape::Point { .. }) => 1,
        (ElementKind::Legend, _) => 2,
        (_, PickShape::Path { .. }) => 3,
        _ => 4,
    }
}

impl PickShape {
    /// 📏️ Distance from `p` to the shape; zero inside areas and rectangles, `radius` is subtracted for points.
    pub fn distance(&self, p: [f64; 2]) -> f64 {
        match self {
            Self::Point { at, radius } => ((p[0] - at[0]).hypot(p[1] - at[1]) - radius).max(0.0),
            Self::Path { points } => points.windows(2).map(|w| segment_distance(p, w[0], w[1])).fold(f64::INFINITY, f64::min),
            Self::Area { ring } => {
                if ring_contains(ring, p) {
                    0.0
                } else {
                    (0..ring.len()).map(|i| segment_distance(p, ring[i], ring[(i + 1) % ring.len()])).fold(f64::INFINITY, f64::min)
                }
            }
            Self::Rect { min, max } => (min[0] - p[0]).max(p[0] - max[0]).max(0.0).hypot((min[1] - p[1]).max(p[1] - max[1]).max(0.0)),
        }
    }
}

/// 🖱️ The best target within `tolerance` pixels: markers beat points, points beat legend entries, legend entries beat lines, lines beat areas; ties by distance.
pub fn pick(targets: &[PickTarget], p: [f64; 2], tolerance: f64) -> Option<Hit> {
    targets
        .iter()
        .map(|t| Hit { target: t.clone(), distance: t.shape.distance(p) })
        .filter(|h| h.distance <= tolerance)
        .min_by(|a, b| priority(&a.target).cmp(&priority(&b.target)).then(a.distance.total_cmp(&b.distance)))
}

/// ✅️ The selection event for a hit; markers carry their bound check, every element carries the diagram focus.
pub fn selection_for(diagram: &Diagram2d, hit: &Hit) -> DiagramSelection {
    let t = &hit.target;
    let check_id = diagram.markers.iter().find(|m| m.id == t.element).and_then(|m| m.bound_check.clone());
    DiagramSelection { diagram_id: diagram.id.clone(), element: t.element.clone(), kind: t.kind, index: t.index, check_id, entity: diagram.focus.clone() }
}

/// ⌨️ Keyboard traversal order: series, then markers, then notes; each id once.
pub fn focus_order(diagram: &Diagram2d) -> Vec<String> {
    diagram.ids().into_iter().map(str::to_string).collect()
}

/// ⌨️ The element after (`step > 0`) or before (`step < 0`) `current` with wrap-around; the first one without a current element.
pub fn step_focus(order: &[String], current: Option<&str>, step: i32) -> Option<String> {
    if order.is_empty() {
        return None;
    }
    let n = order.len() as i32;
    let at = current.and_then(|c| order.iter().position(|o| o == c)).map_or(if step >= 0 { -1 } else { 0 }, |i| i as i32);
    Some(order[(at + step).rem_euclid(n) as usize].clone())
}

/// 💬️ Tooltip text of a hit in the locale: label, coordinates with axis units and utilisation if the element is a marker.
pub fn tooltip(diagram: &Diagram2d, hit: &Hit, locale: Locale) -> String {
    let label = element_label(diagram, &hit.target.element, locale);
    let mut text = label;
    if let Some([x, y]) = hit.target.data {
        let axis = |i: usize, v: f64| diagram.axes.get(i).map_or_else(|| format_number(v, locale), |a| format!("{} {}", format_number(v, locale), a.unit));
        text.push_str(&format!(" ({}; {})", axis(0, x), axis(1, y)));
    }
    if let Some(u) = diagram.markers.iter().find(|m| m.id == hit.target.element).and_then(|m| m.utilisation) {
        text.push_str(&format!(" · {}", format_percent(u, locale)));
    }
    text
}

/// 🏷️ The locale label of a series, marker or note id.
pub fn element_label(diagram: &Diagram2d, id: &str, locale: Locale) -> String {
    let pick = |c: &LocalizedCopy| copy_text(c, locale).to_string();
    diagram
        .series
        .iter()
        .find(|s| s.id() == id)
        .map(|s| pick(series_label(s)))
        .or_else(|| diagram.markers.iter().find(|m| m.id == id).map(|m| pick(&m.label)))
        .or_else(|| diagram.annotations.iter().find(|n| n.id == id).map(|n| pick(&n.text)))
        .unwrap_or_else(|| id.to_string())
}
