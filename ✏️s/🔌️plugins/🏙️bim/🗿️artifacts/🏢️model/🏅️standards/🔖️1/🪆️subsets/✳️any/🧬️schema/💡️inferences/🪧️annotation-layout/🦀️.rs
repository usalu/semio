//! 🪧️ `annotation-layout`: what the dimensions, tags, text notes and leaders of a storey show, from the current geometry of what they are anchored to. The snapshot stores only what was authored (the
//! anchors, the measuring direction, the offsets, the style, an optional lock, the note text); every distance, every printed text, every extension line and every position that follows an
//! element is derived here and never stored, so moving an anchored wall changes the printed number of its dimension and moves its tags by inference.
//!
//! The `Annotation(storey)` node of the model graph has the `WallLayout` of every wall whose face a dimension or leader of the storey is anchored to as parents (the join-trimmed faces are the
//! layout's), and depends besides on the annotations of the storey, the styles they name and the authored records of the elements they name (see [`dependency`]). The plan of the storey draws
//! from its value and the diagnostics of the storey report its [`StoreyAnnotations::findings`]: anchors that name nothing or no longer have geometry, dimensions of zero length, violated locks, empty tags.
//! A lock is a check, never a driver (decision r9 section 1): it names the value a dimension must keep and a finding reports the difference, nothing moves.
//!
//! Related: <https://en.wikipedia.org/wiki/Technical_drawing#Dimensioning>.

use super::super::diagnostics::{Diagnostic, DiagnosticCode};
use super::super::element_solids::{dep_object, dep_value};
use super::super::wall_layout::WallLayout;
use crate::{AnnotationAnchor, DimensionUnit, ModelSnapshot, Point2, Terminator};
use anchors::{Reason, EPS};
use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet};

#[path = "📍️anchors/🦀️.rs"]
pub mod anchors;
#[path = "📏️dimensions/🦀️.rs"]
pub mod dimensions;
#[path = "🔤️texts/🦀️.rs"]
pub mod texts;

//#region 🔖️Values
/// 🎨️ The numbers of a style a drawing needs, copied into every layout so a drawing reads nothing but the layout: text height, line end mark and size, printed unit and decimals, extension gap and overshoot.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct StyleMarks {
    pub text_height: f64,
    pub terminator: Terminator,
    pub unit: DimensionUnit,
    pub precision: u32,
    pub mark_size: f64,
    pub gap: f64,
    pub overshoot: f64,
}

impl Default for StyleMarks {
    fn default() -> Self {
        Self { text_height: 0.25, terminator: Terminator::Tick, unit: DimensionUnit::Metre, precision: 2, mark_size: 0.15, gap: 0.1, overshoot: 0.2 }
    }
}

/// 〰️ A straight line from `start` to `end` in building coordinates.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Line {
    pub start: Point2,
    pub end: Point2,
}

/// 🔤️ Where a text sits: the middle of its baseline in building coordinates (metres), its rotation (counter-clockwise radians) and the estimated advance width in metres.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct TextAnchor {
    pub at: Point2,
    pub rotation: f64,
    pub width: f64,
}

/// ⚓️ One resolved anchor of a dimension: why it has no geometry (else `None`), the foot on the anchored geometry nearest to the dimension line, the mark on the dimension line, the position of the mark
/// along the measuring direction and the extension line from the foot to beyond the dimension line (absent when the anchor lies on it).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct AnchorLayout {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    pub foot: Point2,
    pub mark: Point2,
    pub position: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<Line>,
}

/// 📏️ One measured span between two consecutive anchors: its end marks on the dimension line, its length in metres and the text printed for it.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct DimensionSegment {
    pub from: Point2,
    pub to: Point2,
    pub length: f64,
    pub text: String,
    pub text_at: TextAnchor,
}

/// 📏️ The derived layout of a dimension. `complete` is false while an anchor has no geometry; the layout then has no segment and no total. `lock_difference` is the total minus the lock, when locked.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct DimensionLayout {
    pub style: StyleMarks,
    pub angle: f64,
    pub anchors: Vec<AnchorLayout>,
    pub segments: Vec<DimensionSegment>,
    pub total: f64,
    pub total_text: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lock_difference: Option<f64>,
    pub complete: bool,
}

/// 🏷️ The derived layout of a tag: the reference point of its element, the text and where it sits. `complete` is false when the element is gone.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct TagLayout {
    pub style: StyleMarks,
    pub reference: Point2,
    pub at: TextAnchor,
    pub text: String,
    pub complete: bool,
}

/// 🗒️ The derived layout of a text note.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct NoteLayout {
    pub style: StyleMarks,
    pub at: TextAnchor,
    pub text: String,
}

/// ↗️ The derived layout of a leader: the tip on its anchor and the text. `reason` says why the anchor has no geometry, when it has none.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct LeaderLayout {
    pub style: StyleMarks,
    pub tip: Point2,
    pub at: TextAnchor,
    pub text: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
}

/// 🪧️ Everything the annotations of one storey show, by annotation id, and the findings about them.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct StoreyAnnotations {
    pub storey: String,
    pub dimensions: BTreeMap<String, DimensionLayout>,
    pub tags: BTreeMap<String, TagLayout>,
    pub notes: BTreeMap<String, NoteLayout>,
    pub leaders: BTreeMap<String, LeaderLayout>,
    pub findings: Vec<Diagnostic>,
}

/// 🧾️ The inferred values the annotations of a storey are laid out from (the parents of its `Annotation` node): the wall layouts by wall id.
#[derive(Default)]
pub struct Inputs<'a> {
    pub layouts: BTreeMap<&'a str, &'a WallLayout>,
}
//#endregion 🔖️Values

//#region 🔖️Printing
/// 🔢️ The numbers of a style: its own, or the standard when the style is missing (a finding reports the missing style).
pub fn marks_of(snapshot: &ModelSnapshot, style: &str) -> StyleMarks {
    snapshot.annotation_styles.get(style).map_or_else(StyleMarks::default, |row| StyleMarks { text_height: row.text_height, terminator: row.terminator, unit: row.unit, precision: row.precision, mark_size: row.mark_size, gap: row.gap, overshoot: row.overshoot })
}

/// 🔢️ A length in metres printed in a unit with a number of decimals; negative zero prints as zero.
pub fn print(metres: f64, unit: DimensionUnit, precision: u32) -> String {
    let scaled = metres
        * match unit {
            DimensionUnit::Metre => 1.0,
            DimensionUnit::Centimetre => 100.0,
            DimensionUnit::Millimetre => 1000.0,
        };
    let text = format!("{:.*}", precision as usize, scaled);
    match text.strip_prefix('-') {
        Some(rest) if rest.chars().all(|letter| letter == '0' || letter == '.') => rest.to_string(),
        _ => text,
    }
}

/// 🔤️ The estimated advance width in metres of a text of the given height: the plan font is proportional, 0.55 em per character is its measured mean.
pub fn text_width(text: &str, height: f64) -> f64 {
    text.chars().count() as f64 * height * 0.55
}
//#endregion 🔖️Printing

//#region 🔖️Layout
/// 🔎️ Whether the storey has any annotation (a storey without one has no `Annotation` node).
pub fn is_annotated(snapshot: &ModelSnapshot, storey: &str) -> bool {
    snapshot.dimensions.values().any(|row| row.storey == storey) || snapshot.tags.values().any(|row| row.storey == storey) || snapshot.text_notes.values().any(|row| row.storey == storey) || snapshot.leaders.values().any(|row| row.storey == storey)
}

/// 🧱️ The walls whose join-trimmed faces the dimensions and leaders of a storey are anchored to: the `WallLayout` parents of its node.
pub fn face_walls(snapshot: &ModelSnapshot, storey: &str) -> BTreeSet<String> {
    let anchors = snapshot.dimensions.values().filter(|row| row.storey == storey).flat_map(|row| row.anchors.iter()).chain(snapshot.leaders.values().filter(|row| row.storey == storey).map(|row| &row.anchor));
    anchors.filter_map(|anchor| if let AnnotationAnchor::WallFace { wall, .. } = anchor { snapshot.walls.contains_key(wall).then(|| wall.clone()) } else { None }).collect()
}

fn finding(code: DiagnosticCode, id: &str, storey: &str) -> Diagnostic {
    Diagnostic::new(code, &[id]).on(storey)
}

fn reasons(storey: &str, id: &str, anchors: impl Iterator<Item = (Option<Reason>, Option<String>)>, findings: &mut Vec<Diagnostic>) {
    for (reason, element) in anchors {
        match (reason, element) {
            (Some(Reason::Missing), element) => findings.push(finding(DiagnosticCode::AnnotationAnchorMissing, id, storey).lacking(element.as_deref().unwrap_or_default())),
            (Some(_), _) => findings.push(finding(DiagnosticCode::AnnotationAnchorUnresolved, id, storey)),
            (None, _) => {}
        }
    }
}

/// 🧮️ The annotations of one storey laid out over the wall layouts of `inputs`, with their findings in a stable order (dimensions, tags, notes, leaders; by id).
pub fn annotations_of(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>) -> StoreyAnnotations {
    let mut out = StoreyAnnotations { storey: storey.to_string(), ..StoreyAnnotations::default() };
    let styled = |id: &str, style: &str, findings: &mut Vec<Diagnostic>| {
        if !snapshot.annotation_styles.contains_key(style) {
            findings.push(finding(DiagnosticCode::AnnotationStyleMissing, id, storey).lacking(style));
        }
    };
    for (id, row) in snapshot.dimensions.iter().filter(|(_, row)| row.storey == storey) {
        let layout = dimensions::layout_of(snapshot, inputs, row);
        styled(id, &row.style, &mut out.findings);
        reasons(storey, id, layout.anchors.iter().zip(&row.anchors).map(|(anchor, source)| (anchor.reason, source.element().map(str::to_string))), &mut out.findings);
        if layout.complete && layout.segments.iter().any(|segment| segment.length <= EPS) {
            out.findings.push(finding(DiagnosticCode::DimensionZero, id, storey));
        }
        if let (Some(difference), Some(lock)) = (layout.lock_difference, row.lock) {
            if difference.abs() > LOCK_TOLERANCE {
                out.findings.push(finding(DiagnosticCode::DimensionLockViolated, id, storey).with("length", layout.total).with("lock", lock).with("difference", difference));
            }
        }
        out.dimensions.insert(id.clone(), layout);
    }
    for (id, row) in snapshot.tags.iter().filter(|(_, row)| row.storey == storey) {
        let layout = texts::tag_layout(snapshot, row);
        styled(id, &row.style, &mut out.findings);
        if !layout.complete {
            out.findings.push(finding(DiagnosticCode::AnnotationAnchorMissing, id, storey).lacking(&row.element));
        } else if layout.text.is_empty() {
            out.findings.push(finding(DiagnosticCode::TagEmpty, id, storey));
        }
        out.tags.insert(id.clone(), layout);
    }
    for (id, row) in snapshot.text_notes.iter().filter(|(_, row)| row.storey == storey) {
        styled(id, &row.style, &mut out.findings);
        out.notes.insert(id.clone(), texts::note_layout(snapshot, row));
    }
    for (id, row) in snapshot.leaders.iter().filter(|(_, row)| row.storey == storey) {
        let layout = texts::leader_layout(snapshot, inputs, row);
        styled(id, &row.style, &mut out.findings);
        reasons(storey, id, std::iter::once((layout.reason, row.anchor.element().map(str::to_string))), &mut out.findings);
        out.leaders.insert(id.clone(), layout);
    }
    out
}

/// ⚖️ The difference in metres below which a locked dimension still keeps its value: a micrometre.
pub const LOCK_TOLERANCE: f64 = 1e-6;
//#endregion 🔖️Layout

//#region 🔖️Dependency
fn element_input(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let wall = snapshot.walls.get(id).cloned();
    let column = snapshot.columns.get(id).cloned();
    let beam = snapshot.beams.get(id).cloned();
    let slab = snapshot.slabs.get(id).cloned();
    let roof = snapshot.roofs.get(id).cloned();
    let opening = snapshot.openings.get(id).cloned();
    dep_object([
        ("wall", dep_value(&wall)),
        ("wall_type", dep_value(&wall.as_ref().and_then(|row| snapshot.wall_types.get(&row.wall_type).cloned()))),
        ("curtain_wall", dep_value(&snapshot.curtain_walls.get(id).cloned())),
        ("column", dep_value(&column)),
        ("column_type", dep_value(&column.as_ref().and_then(|row| snapshot.column_types.get(&row.column_type).cloned()))),
        ("beam", dep_value(&beam)),
        ("beam_type", dep_value(&beam.as_ref().and_then(|row| snapshot.beam_types.get(&row.beam_type).cloned()))),
        ("slab", dep_value(&slab)),
        ("slab_type", dep_value(&slab.as_ref().and_then(|row| snapshot.slab_types.get(&row.slab_type).cloned()))),
        ("roof", dep_value(&roof)),
        ("roof_type", dep_value(&roof.as_ref().and_then(|row| snapshot.roof_types.get(&row.roof_type).cloned()))),
        ("opening", dep_value(&opening)),
        ("host", dep_value(&opening.as_ref().and_then(|row| snapshot.walls.get(&row.host).map(|host| host.axis.clone()).or_else(|| snapshot.curtain_walls.get(&row.host).map(|host| host.axis.clone()))))),
        ("window_type", dep_value(&opening.as_ref().and_then(|row| if let crate::OpeningKind::Window { window_type } = &row.kind { snapshot.window_types.get(window_type).cloned() } else { None }))),
        ("door_type", dep_value(&opening.as_ref().and_then(|row| if let crate::OpeningKind::Door { door_type } = &row.kind { snapshot.door_types.get(door_type).cloned() } else { None }))),
        ("stair", dep_value(&snapshot.stairs.get(id).cloned())),
        ("railing", dep_value(&snapshot.railings.get(id).cloned())),
        ("space", dep_value(&snapshot.spaces.get(id).cloned())),
        ("grid", dep_value(&snapshot.grids.get(id).cloned())),
    ])
}

/// 🔑️ Everything the annotations of `storey` read of the snapshot besides the wall layouts they are parented to: the annotation records of the storey, the styles they name and the authored records
/// (with the types they name) of every element they are anchored to or read.
pub fn dependency(snapshot: &ModelSnapshot, storey: &str) -> DslValue {
    let dimensions: BTreeMap<&String, _> = snapshot.dimensions.iter().filter(|(_, row)| row.storey == storey).collect();
    let tags: BTreeMap<&String, _> = snapshot.tags.iter().filter(|(_, row)| row.storey == storey).collect();
    let notes: BTreeMap<&String, _> = snapshot.text_notes.iter().filter(|(_, row)| row.storey == storey).collect();
    let leaders: BTreeMap<&String, _> = snapshot.leaders.iter().filter(|(_, row)| row.storey == storey).collect();
    let styles: BTreeSet<&String> = dimensions.values().map(|row| &row.style).chain(tags.values().map(|row| &row.style)).chain(notes.values().map(|row| &row.style)).chain(leaders.values().map(|row| &row.style)).collect();
    let elements: BTreeSet<&str> = dimensions.values().flat_map(|row| row.elements()).chain(tags.values().map(|row| row.element.as_str())).chain(leaders.values().filter_map(|row| row.element())).collect();
    dep_object([
        ("dimensions", DslValue::object(dimensions.into_iter().map(|(id, row)| (id.clone(), dep_value(row))))),
        ("tags", DslValue::object(tags.into_iter().map(|(id, row)| (id.clone(), dep_value(row))))),
        ("notes", DslValue::object(notes.into_iter().map(|(id, row)| (id.clone(), dep_value(row))))),
        ("leaders", DslValue::object(leaders.into_iter().map(|(id, row)| (id.clone(), dep_value(row))))),
        ("styles", DslValue::object(styles.into_iter().map(|id| (id.clone(), dep_value(&snapshot.annotation_styles.get(id).cloned()))))),
        ("elements", DslValue::object(elements.into_iter().map(|id| (id.to_string(), element_input(snapshot, id))))),
    ])
}

/// 📖️ The snapshot collections the annotations read.
pub const READS: &[&str] = &[
    "dimensions", "tags", "text_notes", "leaders", "annotation_styles", "walls", "wall_types", "curtain_walls", "columns", "column_types", "beams", "beam_types", "slabs", "slab_types", "roofs", "roof_types", "openings", "window_types", "door_types", "stairs", "railings", "spaces", "grids",
];
//#endregion 🔖️Dependency

//#region 🔖️Metrics
#[derive(value_derive::ToValue)]
struct DimensionMetrics {
    segments: Vec<f64>,
    total: f64,
}

#[derive(value_derive::ToValue)]
struct StoreyMetrics {
    dimensions: BTreeMap<String, DimensionMetrics>,
    tags: BTreeMap<String, String>,
    findings: Vec<String>,
}

/// 📏️ The canonical JSON table `storey → { dimensions: id → { segments, total }, tags: id → text, findings: ["<slug>|<ids>"] }` the annotation oracle compares: only complete dimensions are listed.
pub fn metrics_json(annotations: &BTreeMap<String, StoreyAnnotations>) -> String {
    let table: BTreeMap<String, StoreyMetrics> = annotations
        .iter()
        .map(|(storey, set)| {
            let dimensions = set.dimensions.iter().filter(|(_, layout)| layout.complete).map(|(id, layout)| (id.clone(), DimensionMetrics { segments: layout.segments.iter().map(|segment| segment.length).collect(), total: layout.total })).collect();
            let tags = set.tags.iter().filter(|(_, layout)| layout.complete).map(|(id, layout)| (id.clone(), layout.text.clone())).collect();
            let mut findings: Vec<String> = set.findings.iter().map(|finding| format!("{}|{}", finding.code.slug(), finding.elements.join(","))).collect();
            findings.sort();
            findings.dedup();
            (storey.clone(), StoreyMetrics { dimensions, tags, findings })
        })
        .collect();
    semio_framework_pack_json::to_json_string(&table)
}
//#endregion 🔖️Metrics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
