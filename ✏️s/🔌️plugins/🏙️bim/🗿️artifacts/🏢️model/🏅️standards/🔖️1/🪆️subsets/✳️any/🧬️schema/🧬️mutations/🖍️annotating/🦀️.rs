//! 🖍️ The rules every annotation leaf shares: what makes a dimension, a tag, a text note, a leader and an annotation style valid in a base. Pure; a leaf prefixes the field of the
//! [`Fault`] with the record name on create (`["dimension", "anchors"]`) and uses it as is on set (`["anchors"]`).

use crate::{AnnotationAnchor, AnnotationStyle, Dimension, Leader, ModelSnapshot, Point2, Tag, TextNote};
use protocol::OutcomeCode;

/// 🚫️ A rule broken: the outcome code, the message and the field of the record that breaks it.
pub struct Fault {
    pub code: OutcomeCode,
    pub message: String,
    pub field: &'static str,
}

impl Fault {
    fn missing(noun: &str, id: &str, field: &'static str) -> Self {
        Self { code: OutcomeCode::TargetMissing, message: format!("{noun} \"{id}\" does not exist."), field }
    }

    fn invalid(message: impl Into<String>, field: &'static str) -> Self {
        Self { code: OutcomeCode::Invariant, message: message.into(), field }
    }

    /// 🧭️ The target path of the refusal under `record` (empty for a set leaf).
    pub fn path(&self, record: Option<&'static str>) -> Vec<&'static str> {
        record.into_iter().chain([self.field]).collect()
    }
}

fn finite(point: Point2) -> bool {
    point.x.is_finite() && point.y.is_finite()
}

fn storey_fault(base: &ModelSnapshot, storey: &str) -> Option<Fault> {
    (!base.storeys.contains_key(storey)).then(|| Fault::missing("Storey", storey, "storey"))
}

fn style_fault(base: &ModelSnapshot, style: &str) -> Option<Fault> {
    (!base.annotation_styles.contains_key(style)).then(|| Fault::missing("Annotation style", style, "style"))
}

fn anchor_fault(base: &ModelSnapshot, anchor: &AnnotationAnchor, field: &'static str) -> Option<Fault> {
    match anchor {
        AnnotationAnchor::Point { point } => (!finite(*point)).then(|| Fault::invalid("An anchor point must have finite coordinates.", field)),
        AnnotationAnchor::WallFace { wall, .. } | AnnotationAnchor::WallAxis { wall } | AnnotationAnchor::WallEnd { wall, .. } => (!base.walls.contains_key(wall)).then(|| Fault::missing("Wall", wall, field)),
        AnnotationAnchor::OpeningCentre { opening } => (!base.openings.contains_key(opening)).then(|| Fault::missing("Opening", opening, field)),
        AnnotationAnchor::Grid { grid } => (!base.grids.contains_key(grid)).then(|| Fault::missing("Grid line", grid, field)),
        AnnotationAnchor::ColumnCentre { column } => (!base.columns.contains_key(column)).then(|| Fault::missing("Column", column, field)),
    }
}

fn taggable(base: &ModelSnapshot, element: &str) -> bool {
    base.walls.contains_key(element)
        || base.curtain_walls.contains_key(element)
        || base.columns.contains_key(element)
        || base.beams.contains_key(element)
        || base.slabs.contains_key(element)
        || base.roofs.contains_key(element)
        || base.openings.contains_key(element)
        || base.stairs.contains_key(element)
        || base.railings.contains_key(element)
        || base.spaces.contains_key(element)
        || base.grids.contains_key(element)
}

/// 📏️ The first rule a dimension breaks: its storey and style exist, it has at least two anchors that name existing elements or finite points, its direction and offset are finite and a lock is a positive length.
pub fn dimension_fault(base: &ModelSnapshot, dimension: &Dimension) -> Option<Fault> {
    if let Some(fault) = storey_fault(base, &dimension.storey).or_else(|| style_fault(base, &dimension.style)) {
        return Some(fault);
    }
    if dimension.anchors.len() < 2 {
        return Some(Fault::invalid("A dimension needs at least two anchors.", "anchors"));
    }
    if let Some(fault) = dimension.anchors.iter().find_map(|anchor| anchor_fault(base, anchor, "anchors")) {
        return Some(fault);
    }
    if !dimension.angle.is_finite() {
        return Some(Fault::invalid("A dimension direction must be a finite angle.", "angle"));
    }
    if !dimension.offset.is_finite() {
        return Some(Fault::invalid("A dimension offset must be a finite length.", "offset"));
    }
    dimension.lock.filter(|lock| !(lock.is_finite() && *lock > 0.0)).map(|_| Fault::invalid("A dimension lock must be a positive length.", "lock"))
}

/// 🏷️ The first rule a tag breaks: its storey and style exist, it names an element that can be tagged and its offset is finite.
pub fn tag_fault(base: &ModelSnapshot, tag: &Tag) -> Option<Fault> {
    if let Some(fault) = storey_fault(base, &tag.storey).or_else(|| style_fault(base, &tag.style)) {
        return Some(fault);
    }
    if !taggable(base, &tag.element) {
        return Some(Fault::missing("Element", &tag.element, "element"));
    }
    (!finite(tag.offset)).then(|| Fault::invalid("A tag offset must have finite coordinates.", "offset"))
}

/// 🗒️ The first rule a text note breaks: its storey and style exist, its position and rotation are finite and its text is not blank.
pub fn note_fault(base: &ModelSnapshot, note: &TextNote) -> Option<Fault> {
    if let Some(fault) = storey_fault(base, &note.storey).or_else(|| style_fault(base, &note.style)) {
        return Some(fault);
    }
    if !finite(note.position) {
        return Some(Fault::invalid("A text note position must have finite coordinates.", "position"));
    }
    if !note.rotation.is_finite() {
        return Some(Fault::invalid("A text note rotation must be a finite angle.", "rotation"));
    }
    note.text.trim().is_empty().then(|| Fault::invalid("A text note must not be blank.", "text"))
}

/// ↗️ The first rule a leader breaks: its storey and style exist, its anchor names an existing element or a finite point, its offset is finite and its text is not blank.
pub fn leader_fault(base: &ModelSnapshot, leader: &Leader) -> Option<Fault> {
    if let Some(fault) = storey_fault(base, &leader.storey).or_else(|| style_fault(base, &leader.style)) {
        return Some(fault);
    }
    if let Some(fault) = anchor_fault(base, &leader.anchor, "anchor") {
        return Some(fault);
    }
    if !finite(leader.offset) {
        return Some(Fault::invalid("A leader offset must have finite coordinates.", "offset"));
    }
    leader.text.trim().is_empty().then(|| Fault::invalid("A leader text must not be blank.", "text"))
}

/// 🎨️ The first rule an annotation style breaks: a name, a positive text height, non-negative mark size, gap and overshoot, and at most six printed decimals.
pub fn style_record_fault(style: &AnnotationStyle) -> Option<Fault> {
    if style.name.trim().is_empty() {
        return Some(Fault::invalid("An annotation style needs a name.", "name"));
    }
    if !(style.text_height.is_finite() && style.text_height > 0.0) {
        return Some(Fault::invalid("A text height must be a positive length.", "text_height"));
    }
    for (value, field) in [(style.mark_size, "mark_size"), (style.gap, "gap"), (style.overshoot, "overshoot")] {
        if !(value.is_finite() && value >= 0.0) {
            return Some(Fault::invalid("A mark size, gap and overshoot must be lengths of zero or more.", field));
        }
    }
    (style.precision > 6).then(|| Fault::invalid("A dimension prints at most six decimals.", "precision"))
}
