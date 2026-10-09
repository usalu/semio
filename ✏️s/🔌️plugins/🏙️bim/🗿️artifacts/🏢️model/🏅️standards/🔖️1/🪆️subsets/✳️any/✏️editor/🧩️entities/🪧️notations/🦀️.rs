//! 🪧️ The annotation rows of the entity table: how a dimension, a tag, a text note, a leader and an annotation style read off the snapshot, how an edited value becomes a `set-*` mutation, what a new
//! annotation is, and the pickers (the annotation styles, the tag categories, the line ends and units) the properties panel offers for the reference and choice rows. The values a dimension or a tag
//! shows (its measured length, its printed text) are inferred rows of the `annotation-layout` inference.

use super::{number, parse_count, parse_number, parse_point, parse_text, partial, point, variant, Created, EntityKind, FieldRow, InferredRow};
use crate::editor::bim::terminology::BimLabels;
use crate::{AnchorEnd, AnnotationAnchor, AnnotationStyle, Assigned, Dimension, DimensionUnit, Leader, ModelMutation, ModelSnapshot, Point2, Tag, TagCategory, Terminator, TextNote, WallSide};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

//#region 🔖️Text
/// ⚓️ The text of an anchor: `point x, y`, `face wall left|right`, `axis wall`, `end wall start|end`, `opening id`, `grid id` or `column id`.
pub fn anchor_text(anchor: &AnnotationAnchor) -> String {
    match anchor {
        AnnotationAnchor::Point { point: at } => format!("point {}", point(at.x, at.y)),
        AnnotationAnchor::WallFace { wall, side } => format!("face {wall} {}", format!("{side:?}").to_ascii_lowercase()),
        AnnotationAnchor::WallAxis { wall } => format!("axis {wall}"),
        AnnotationAnchor::WallEnd { wall, end } => format!("end {wall} {}", format!("{end:?}").to_ascii_lowercase()),
        AnnotationAnchor::OpeningCentre { opening } => format!("opening {opening}"),
        AnnotationAnchor::Grid { grid } => format!("grid {grid}"),
        AnnotationAnchor::ColumnCentre { column } => format!("column {column}"),
    }
}

/// ⚓️ The anchor a text names (see [`anchor_text`]).
pub fn parse_anchor(text: &str) -> Option<AnnotationAnchor> {
    let text = text.trim();
    let (keyword, rest) = text.split_once(' ')?;
    let rest = rest.trim();
    let words: Vec<&str> = rest.split_whitespace().collect();
    match (keyword.to_ascii_lowercase().as_str(), words.as_slice()) {
        ("point", _) => parse_point(rest).map(|at| AnnotationAnchor::Point { point: at }),
        ("face", [wall, side]) => Some(AnnotationAnchor::WallFace { wall: (*wall).to_string(), side: variant(side, &[WallSide::Left, WallSide::Right])? }),
        ("axis", [wall]) => Some(AnnotationAnchor::WallAxis { wall: (*wall).to_string() }),
        ("end", [wall, end]) => Some(AnnotationAnchor::WallEnd { wall: (*wall).to_string(), end: variant(end, &[AnchorEnd::Start, AnchorEnd::End])? }),
        ("opening", [opening]) => Some(AnnotationAnchor::OpeningCentre { opening: (*opening).to_string() }),
        ("grid", [grid]) => Some(AnnotationAnchor::Grid { grid: (*grid).to_string() }),
        ("column", [column]) => Some(AnnotationAnchor::ColumnCentre { column: (*column).to_string() }),
        _ => None,
    }
}

/// ⚓️ The text of the anchors of a dimension, joined by ` | `.
pub fn anchors_text(anchors: &[AnnotationAnchor]) -> String {
    anchors.iter().map(anchor_text).collect::<Vec<_>>().join(" | ")
}

/// ⚓️ The anchors a text names, at least two, joined by `|`.
pub fn parse_anchors(text: &str) -> Option<Vec<AnnotationAnchor>> {
    let anchors = text.split('|').map(parse_anchor).collect::<Option<Vec<_>>>()?;
    (anchors.len() >= 2).then_some(anchors)
}

fn parse_category(text: &str) -> Option<TagCategory> {
    variant(text, &[TagCategory::Name, TagCategory::Type, TagCategory::Number, TagCategory::Size])
}

fn parse_terminator(text: &str) -> Option<Terminator> {
    variant(text, &[Terminator::Tick, Terminator::Arrow, Terminator::Dot])
}

fn parse_unit(text: &str) -> Option<DimensionUnit> {
    variant(text, &[DimensionUnit::Metre, DimensionUnit::Centimetre, DimensionUnit::Millimetre])
}

fn parse_lock(text: &str) -> Option<Assigned<Option<f64>>> {
    let text = text.trim();
    if text.is_empty() {
        return Some(Assigned::new(None));
    }
    text.parse::<f64>().ok().map(|lock| Assigned::new(Some(lock)))
}

fn clip(text: &str) -> String {
    let short: String = text.chars().take(24).collect();
    if short.chars().count() < text.chars().count() { format!("{short}\u{2026}") } else { short }
}
//#endregion 🔖️Text

//#region 🔖️Choices
/// 🎨️ The annotation styles an annotation can use: every style of the library by its id and name.
pub fn style_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    snapshot.annotation_styles.iter().map(|(id, style)| (id.clone(), style.name.clone())).collect()
}

/// 🏷️ What a tag can read, by its stored name and its localized label.
pub fn category_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    vec![
        ("Name".to_string(), labels.choice_name.as_str().to_string()),
        ("Type".to_string(), labels.choice_type.as_str().to_string()),
        ("Number".to_string(), labels.choice_number.as_str().to_string()),
        ("Size".to_string(), labels.choice_size.as_str().to_string()),
    ]
}

/// 🔚️ The line ends of a style, by their stored name and their localized label.
pub fn terminator_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    vec![("Tick".to_string(), labels.choice_tick.as_str().to_string()), ("Arrow".to_string(), labels.choice_arrow.as_str().to_string()), ("Dot".to_string(), labels.choice_dot.as_str().to_string())]
}

/// 📏️ The units a style prints in, by their stored name and their localized label.
pub fn unit_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    vec![("Metre".to_string(), labels.choice_metre.as_str().to_string()), ("Centimetre".to_string(), labels.choice_centimetre.as_str().to_string()), ("Millimetre".to_string(), labels.choice_millimetre.as_str().to_string())]
}
//#endregion 🔖️Choices

//#region 🔖️Fields
/// 🧾️ The authored parameters of a dimension.
pub static DIMENSION_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.dimensions.get(id).map(|row| row.name.clone()), parse_text => set_dimension::SetDimension),
    field!("storey", field_storey, Text, |s, id| s.dimensions.get(id).map(|row| row.storey.clone())),
    field!("anchors", field_anchors, Text, |s, id| s.dimensions.get(id).map(|row| anchors_text(&row.anchors)), parse_anchors => set_dimension::SetDimension),
    field!("angle", field_direction_angle, Number, |s, id| s.dimensions.get(id).map(|row| number(row.angle)), parse_number => set_dimension::SetDimension),
    field!("offset", field_offset, Number, |s, id| s.dimensions.get(id).map(|row| number(row.offset)), parse_number => set_dimension::SetDimension),
    field!("style", field_style, Text, |s, id| s.dimensions.get(id).map(|row| row.style.clone()), choices: style_choices, parse_text => set_dimension::SetDimension),
    field!("lock", field_lock, Number, |s, id| s.dimensions.get(id).map(|row| row.lock.map_or_else(String::new, number)), |_, id, value| partial::<crate::mutations::set_dimension::SetDimension, _>(id, "lock", &parse_lock(value)?).map(ModelMutation::SetDimension)),
];

/// 🧾️ The authored parameters of a tag.
pub static TAG_FIELDS: &[FieldRow] = &[
    field!("storey", field_storey, Text, |s, id| s.tags.get(id).map(|row| row.storey.clone())),
    field!("element", field_element, Text, |s, id| s.tags.get(id).map(|row| row.element.clone()), parse_text => set_tag::SetTag),
    field!("category", field_reads, Text, |s, id| s.tags.get(id).map(|row| format!("{:?}", row.category)), choices: category_choices, parse_category => set_tag::SetTag),
    field!("offset", field_offset, Text, |s, id| s.tags.get(id).map(|row| point(row.offset.x, row.offset.y)), parse_point => set_tag::SetTag),
    field!("style", field_style, Text, |s, id| s.tags.get(id).map(|row| row.style.clone()), choices: style_choices, parse_text => set_tag::SetTag),
];

/// 🧾️ The authored parameters of a text note.
pub static NOTE_FIELDS: &[FieldRow] = &[
    field!("storey", field_storey, Text, |s, id| s.text_notes.get(id).map(|row| row.storey.clone())),
    field!("text", field_text, Text, |s, id| s.text_notes.get(id).map(|row| row.text.clone()), parse_text => set_text_note::SetTextNote),
    field!("position", field_position, Text, |s, id| s.text_notes.get(id).map(|row| point(row.position.x, row.position.y)), parse_point => set_text_note::SetTextNote),
    field!("rotation", field_rotation, Number, |s, id| s.text_notes.get(id).map(|row| number(row.rotation)), parse_number => set_text_note::SetTextNote),
    field!("style", field_style, Text, |s, id| s.text_notes.get(id).map(|row| row.style.clone()), choices: style_choices, parse_text => set_text_note::SetTextNote),
];

/// 🧾️ The authored parameters of a leader.
pub static LEADER_FIELDS: &[FieldRow] = &[
    field!("storey", field_storey, Text, |s, id| s.leaders.get(id).map(|row| row.storey.clone())),
    field!("anchor", field_anchor, Text, |s, id| s.leaders.get(id).map(|row| anchor_text(&row.anchor)), parse_anchor => set_leader::SetLeader),
    field!("offset", field_offset, Text, |s, id| s.leaders.get(id).map(|row| point(row.offset.x, row.offset.y)), parse_point => set_leader::SetLeader),
    field!("text", field_text, Text, |s, id| s.leaders.get(id).map(|row| row.text.clone()), parse_text => set_leader::SetLeader),
    field!("style", field_style, Text, |s, id| s.leaders.get(id).map(|row| row.style.clone()), choices: style_choices, parse_text => set_leader::SetLeader),
];

/// 🧾️ The authored parameters of an annotation style.
pub static STYLE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.annotation_styles.get(id).map(|row| row.name.clone()), parse_text => set_annotation_style::SetAnnotationStyle),
    field!("text_height", field_text_height, Number, |s, id| s.annotation_styles.get(id).map(|row| number(row.text_height)), parse_number => set_annotation_style::SetAnnotationStyle),
    field!("terminator", field_terminator, Text, |s, id| s.annotation_styles.get(id).map(|row| format!("{:?}", row.terminator)), choices: terminator_choices, parse_terminator => set_annotation_style::SetAnnotationStyle),
    field!("unit", field_unit, Text, |s, id| s.annotation_styles.get(id).map(|row| format!("{:?}", row.unit)), choices: unit_choices, parse_unit => set_annotation_style::SetAnnotationStyle),
    field!("precision", field_precision, Number, |s, id| s.annotation_styles.get(id).map(|row| row.precision.to_string()), parse_count => set_annotation_style::SetAnnotationStyle),
    field!("mark_size", field_mark_size, Number, |s, id| s.annotation_styles.get(id).map(|row| number(row.mark_size)), parse_number => set_annotation_style::SetAnnotationStyle),
    field!("gap", field_extension_gap, Number, |s, id| s.annotation_styles.get(id).map(|row| number(row.gap)), parse_number => set_annotation_style::SetAnnotationStyle),
    field!("overshoot", field_extension_overshoot, Number, |s, id| s.annotation_styles.get(id).map(|row| number(row.overshoot)), parse_number => set_annotation_style::SetAnnotationStyle),
];
//#endregion 🔖️Fields

//#region 🔖️Inferred
fn layout<'a>(inference: &'a crate::ModelInference, storey: &str) -> Option<&'a crate::standards::v1::subsets::any::schema::inferences::annotation_layout::StoreyAnnotations> {
    inference.annotations.get(storey)
}

/// 💡️ What a dimension shows now, read off the `annotation-layout` inference: its measured total, the text it prints and the difference to its lock.
pub static DIMENSION_INFERRED: &[InferredRow] = &[
    inferred!("measured", field_measured, |s, inference, id| s.dimensions.get(id).and_then(|row| layout(inference, &row.storey)).and_then(|set| set.dimensions.get(id)).filter(|row| row.complete).map(|row| number(row.total))),
    inferred!("printed", field_printed, |s, inference, id| s.dimensions.get(id).and_then(|row| layout(inference, &row.storey)).and_then(|set| set.dimensions.get(id)).filter(|row| row.complete).map(|row| row.segments.iter().map(|segment| segment.text.clone()).collect::<Vec<_>>().join(" | "))),
    inferred!("lock_difference", field_lock_difference, |s, inference, id| s.dimensions.get(id).and_then(|row| layout(inference, &row.storey)).and_then(|set| set.dimensions.get(id)).and_then(|row| row.lock_difference).map(number)),
];

/// 💡️ What a tag prints now, read off the `annotation-layout` inference.
pub static TAG_INFERRED: &[InferredRow] = &[inferred!("printed", field_printed, |s, inference, id| s.tags.get(id).and_then(|row| layout(inference, &row.storey)).and_then(|set| set.tags.get(id)).map(|row| row.text.clone()))];
//#endregion 🔖️Inferred

//#region 🔖️Create
fn style_of(snapshot: &ModelSnapshot) -> Result<String, &'static str> {
    snapshot.annotation_styles.keys().next().cloned().ok_or("bim.create.annotation-style-missing")
}

fn container(snapshot: &ModelSnapshot, parent: &str) -> Result<String, &'static str> {
    snapshot.storeys.contains_key(parent).then(|| parent.to_string()).ok_or("bim.create.storey-missing")
}

/// 📏️ A new dimension: half a metre of free points on the storey, in the first annotation style.
pub fn create_dimension(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let (storey, style) = (container(snapshot, parent)?, style_of(snapshot)?);
    let anchors = vec![AnnotationAnchor::Point { point: Point2 { x: 0.0, y: 0.0 } }, AnnotationAnchor::Point { point: Point2 { x: 1.0, y: 0.0 } }];
    Ok(ModelMutation::CreateDimension(crate::mutations::create_dimension::CreateDimension { id: id.into(), dimension: Dimension { storey, anchors, angle: 0.0, offset: -0.5, style, lock: None, name: name.into() } }))
}

/// 🏷️ A new tag: on the first wall of the storey, reading its name.
pub fn create_tag(snapshot: &ModelSnapshot, id: &str, parent: &str, _name: &str) -> Created {
    let (storey, style) = (container(snapshot, parent)?, style_of(snapshot)?);
    let element = snapshot.walls.iter().find(|(_, wall)| wall.storey == storey).map(|(wall, _)| wall.clone()).ok_or("bim.create.wall-missing")?;
    Ok(ModelMutation::CreateTag(crate::mutations::create_tag::CreateTag { id: id.into(), tag: Tag { storey, element, category: TagCategory::Name, offset: Point2 { x: 0.0, y: 0.5 }, style } }))
}

/// 🗒️ A new text note at the origin of the storey, named by its text.
pub fn create_text_note(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let (storey, style) = (container(snapshot, parent)?, style_of(snapshot)?);
    Ok(ModelMutation::CreateTextNote(crate::mutations::create_text_note::CreateTextNote { id: id.into(), text_note: TextNote { storey, position: Point2 { x: 0.0, y: 0.0 }, text: name.into(), rotation: 0.0, style } }))
}

/// ↗️ A new leader from the origin of the storey to a metre beside it, named by its text.
pub fn create_leader(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let (storey, style) = (container(snapshot, parent)?, style_of(snapshot)?);
    Ok(ModelMutation::CreateLeader(crate::mutations::create_leader::CreateLeader { id: id.into(), leader: Leader { storey, anchor: AnnotationAnchor::Point { point: Point2 { x: 0.0, y: 0.0 } }, offset: Point2 { x: 1.0, y: 1.0 }, text: name.into(), style } }))
}

/// 🎨️ The standard annotation style: a quarter-metre text at 1:50, slanted ticks, metres with two decimals.
pub fn standard_style(name: &str) -> AnnotationStyle {
    AnnotationStyle { name: name.into(), text_height: 0.25, terminator: Terminator::Tick, unit: DimensionUnit::Metre, precision: 2, mark_size: 0.15, gap: 0.1, overshoot: 0.2 }
}

/// 🎨️ A new annotation style: the standard one.
pub fn create_annotation_style(_: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateAnnotationStyle(crate::mutations::create_annotation_style::CreateAnnotationStyle { id: id.into(), annotation_style: standard_style(name) }))
}
//#endregion 🔖️Create

//#region 🔖️Kinds
macro_rules! notation_kind {
    ($kind:literal, $icon:literal, $library:literal, $label:ident, $group:ident, $coll:ident, name: $name:expr, parent: |$ps:pat_param, $pid:pat_param| $parent:expr, delete: $delete:expr, create: $create:expr, fields: $fields:expr, inferred: $inferred:expr, rename: $rename:expr) => {
        EntityKind {
            kind: $kind,
            icon: $icon,
            library: $library,
            label: |labels| labels.$label,
            group: |labels| labels.$group,
            ids: |snapshot| snapshot.$coll.keys().cloned().collect(),
            name: |snapshot, id| snapshot.$coll.get(id).map($name),
            parent: |$ps, $pid| $parent,
            delete: $delete,
            rename: $rename,
            create: Some($create),
            fields: $fields,
            inferred: $inferred,
        }
    };
}

/// 📏️ The entity row of dimensions.
pub const DIMENSION: EntityKind = notation_kind!("dimension", "move-horizontal", false, kind_dimension, group_dimensions, dimensions, name: |row: &Dimension| row.name.clone(), parent: |s, id| s.dimensions.get(id).map(|row| row.storey.clone()),
    delete: delete!(delete_dimension::DeleteDimension), create: create_dimension, fields: DIMENSION_FIELDS, inferred: DIMENSION_INFERRED, rename: renaming!(set_dimension::SetDimension));

/// 🏷️ The entity row of tags: named by what it reads and which element.
pub const TAG: EntityKind = notation_kind!("tag", "tag", false, kind_tag, group_tags, tags, name: |row: &Tag| format!("{:?} {}", row.category, row.element), parent: |s, id| s.tags.get(id).map(|row| row.storey.clone()),
    delete: delete!(delete_tag::DeleteTag), create: create_tag, fields: TAG_FIELDS, inferred: TAG_INFERRED, rename: None);

/// 🗒️ The entity row of text notes: named by their text.
pub const TEXT_NOTE: EntityKind = notation_kind!("text-note", "type", false, kind_text_note, group_text_notes, text_notes, name: |row: &TextNote| clip(&row.text), parent: |s, id| s.text_notes.get(id).map(|row| row.storey.clone()),
    delete: delete!(delete_text_note::DeleteTextNote), create: create_text_note, fields: NOTE_FIELDS, inferred: &[], rename: None);

/// ↗️ The entity row of leaders: named by their text.
pub const LEADER: EntityKind = notation_kind!("leader", "arrow-up-right", false, kind_leader, group_leaders, leaders, name: |row: &Leader| clip(&row.text), parent: |s, id| s.leaders.get(id).map(|row| row.storey.clone()),
    delete: delete!(delete_leader::DeleteLeader), create: create_leader, fields: LEADER_FIELDS, inferred: &[], rename: None);

/// 🎨️ The entity row of annotation styles (a library entry).
pub const ANNOTATION_STYLE: EntityKind = notation_kind!("annotation-style", "palette", true, kind_annotation_style, group_annotation_styles, annotation_styles, name: |row: &AnnotationStyle| row.name.clone(), parent: |_, _| None,
    delete: delete!(delete_annotation_style::DeleteAnnotationStyle), create: create_annotation_style, fields: STYLE_FIELDS, inferred: &[], rename: renaming!(set_annotation_style::SetAnnotationStyle));
//#endregion 🔖️Kinds
