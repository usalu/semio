//! 🔤️ Tags, text notes and leaders: the text a tag reads from its element (name, type, number or size) and where each text and leader sits. A tag and a leader are placed by an offset from the
//! reference point of what they name, so they follow the element when it moves; a text note is placed where it was put.

use super::anchors::{reference_point, resolve, Reason};
use super::{marks_of, print, text_width, Inputs, LeaderLayout, NoteLayout, StyleMarks, TagLayout, TextAnchor};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::extents_of;
use crate::standards::v1::subsets::any::schema::inferences::opening_frames::resolve_size;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::thickness_of;
use crate::{Leader, ModelSnapshot, OpeningKind, Point2, Tag, TagCategory, TextNote};

fn name_of(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    snapshot
        .walls
        .get(id)
        .map(|row| row.name.clone())
        .or_else(|| snapshot.curtain_walls.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.columns.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.beams.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.slabs.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.roofs.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.openings.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.stairs.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.railings.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.spaces.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.grids.get(id).map(|row| row.label.clone()))
}

fn type_of(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    if let Some(wall) = snapshot.walls.get(id) {
        return snapshot.wall_types.get(&wall.wall_type).map(|row| row.name.clone());
    }
    if let Some(column) = snapshot.columns.get(id) {
        return snapshot.column_types.get(&column.column_type).map(|row| row.name.clone());
    }
    if let Some(beam) = snapshot.beams.get(id) {
        return snapshot.beam_types.get(&beam.beam_type).map(|row| row.name.clone());
    }
    if let Some(slab) = snapshot.slabs.get(id) {
        return snapshot.slab_types.get(&slab.slab_type).map(|row| row.name.clone());
    }
    if let Some(roof) = snapshot.roofs.get(id) {
        return snapshot.roof_types.get(&roof.roof_type).map(|row| row.name.clone());
    }
    if let Some(opening) = snapshot.openings.get(id) {
        return match &opening.kind {
            OpeningKind::Window { window_type } => snapshot.window_types.get(window_type).map(|row| row.name.clone()),
            OpeningKind::Door { door_type } => snapshot.door_types.get(door_type).map(|row| row.name.clone()),
            OpeningKind::Void { .. } => None,
        };
    }
    snapshot.spaces.get(id).map(|space| space.usage.clone()).filter(|usage| !usage.is_empty())
}

fn across(width: f64, depth: f64, style: &StyleMarks) -> String {
    format!("{} \u{d7} {}", print(width, style.unit, style.precision), print(depth, style.unit, style.precision))
}

fn size_of(snapshot: &ModelSnapshot, id: &str, style: &StyleMarks) -> Option<String> {
    if let Some(opening) = snapshot.openings.get(id) {
        let size = resolve_size(snapshot, opening);
        return Some(across(size.width, size.height, style));
    }
    if let Some(wall) = snapshot.walls.get(id) {
        return Some(print(thickness_of(snapshot, wall), style.unit, style.precision));
    }
    let profile = snapshot.columns.get(id).and_then(|row| snapshot.column_types.get(&row.column_type)).map(|row| &row.profile).or_else(|| snapshot.beams.get(id).and_then(|row| snapshot.beam_types.get(&row.beam_type)).map(|row| &row.profile))?;
    let (width, depth) = extents_of(profile);
    Some(across(width, depth, style))
}

/// 🏷️ The text a tag prints: empty when the element has no such property (a number on a wall).
pub fn tag_text(snapshot: &ModelSnapshot, tag: &Tag, style: &StyleMarks) -> String {
    match tag.category {
        TagCategory::Name => name_of(snapshot, &tag.element),
        TagCategory::Type => type_of(snapshot, &tag.element),
        TagCategory::Number => snapshot.spaces.get(&tag.element).map(|space| space.number.clone()),
        TagCategory::Size => size_of(snapshot, &tag.element, style),
    }
    .unwrap_or_default()
}

fn anchored(at: Point2, rotation: f64, text: &str, height: f64) -> TextAnchor {
    TextAnchor { at, rotation, width: text_width(text, height) }
}

/// 🏷️ The layout of one tag: its text from the element and its position at the offset from the reference point of the element.
pub fn tag_layout(snapshot: &ModelSnapshot, tag: &Tag) -> TagLayout {
    let style = marks_of(snapshot, &tag.style);
    let reference = reference_point(snapshot, &tag.element);
    let origin = reference.unwrap_or(Point2 { x: 0.0, y: 0.0 });
    let text = tag_text(snapshot, tag, &style);
    let at = anchored(Point2 { x: origin.x + tag.offset.x, y: origin.y + tag.offset.y }, 0.0, &text, style.text_height);
    TagLayout { style, reference: origin, at, text, complete: reference.is_some() }
}

/// 🗒️ The layout of one text note.
pub fn note_layout(snapshot: &ModelSnapshot, note: &TextNote) -> NoteLayout {
    let style = marks_of(snapshot, &note.style);
    let at = anchored(note.position, note.rotation, &note.text, style.text_height);
    NoteLayout { style, at, text: note.text.clone() }
}

/// ↗️ The layout of one leader: the tip on its anchor and the text at the offset from the tip.
pub fn leader_layout(snapshot: &ModelSnapshot, inputs: &Inputs<'_>, leader: &Leader) -> LeaderLayout {
    let style = marks_of(snapshot, &leader.style);
    let resolved = resolve(snapshot, inputs, &leader.anchor);
    let tip = resolved.as_ref().map(|reference| reference.pick()).unwrap_or(Point2 { x: 0.0, y: 0.0 });
    let at = anchored(Point2 { x: tip.x + leader.offset.x, y: tip.y + leader.offset.y }, 0.0, &leader.text, style.text_height);
    LeaderLayout { style, tip, at, text: leader.text.clone(), reason: resolved.err().filter(|reason| *reason != Reason::Parallel) }
}
