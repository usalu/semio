//! 🧵️ Element vocabulary shared by the multi-element and data leaves: which ids name an element, the placement fields of the
//! placed kinds (the one record a move, a rotation and their exact inverse all speak), and the name slot of every element kind.

use crate::{
    Assigned, Axis, BeamPatch, CeilingPatch, ColumnPatch, CurtainWallPatch, Entry, GridLinePatch, KeyedDelta, ModelDiff, ModelSnapshot, OpeningPatch, Patch, Point2, RailingPatch, RampPatch, RoofPatch, RoofShape, SlabPatch, Slope, SpaceBoundary, SpacePatch, StairFlight, StairPatch, Vertex, WallPatch,
};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeMap;

//#region 🔖️Placement
/// 📍️ The absolute placement fields of one placed element, tagged by its kind: exactly what a move or a rotation may change and
/// nothing else. A slab carries its slope and a roof its shape because a rotation turns their fall and ridge directions.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Placement {
    Wall {
        axis: Axis,
    },
    CurtainWall {
        axis: Axis,
    },
    Column {
        position: Point2,
        rotation: f64,
        #[value(default, skip_serializing_if = "Option::is_none")]
        tilt: Option<Slope>,
    },
    Beam {
        axis: Axis,
    },
    Slab {
        boundary: Vec<Vertex>,
        holes: Vec<Vec<Vertex>>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        slope: Option<Slope>,
    },
    Ceiling {
        boundary: Vec<Vertex>,
        holes: Vec<Vec<Vertex>>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        slope: Option<Slope>,
    },
    Roof {
        footprint: Vec<Vertex>,
        shape: RoofShape,
    },
    Stair {
        start: Point2,
        direction: f64,
        #[value(default, skip_serializing_if = "Option::is_none")]
        flight: Option<StairFlight>,
    },
    Railing {
        path: Vec<Point2>,
    },
    Ramp {
        path: Vec<Vertex>,
    },
    Space {
        boundary: SpaceBoundary,
    },
    Grid {
        start: Point2,
        end: Point2,
    },
    Opening {
        offset: f64,
        flip_hand: bool,
        flip_facing: bool,
    },
}

fn snap(value: f64) -> f64 {
    (value * 1e12).round() / 1e12 + 0.0
}

fn turned(direction: f64, turn: f64) -> f64 {
    if turn == 0.0 {
        direction
    } else {
        snap(direction + turn)
    }
}

fn axis_map(axis: &Axis, point: &impl Fn(Point2) -> Point2) -> Axis {
    match axis {
        Axis::Line { start, end } => Axis::Line { start: point(*start), end: point(*end) },
        Axis::Arc { start, end, bulge } => Axis::Arc { start: point(*start), end: point(*end), bulge: *bulge },
    }
}

fn loop_map(vertices: &[Vertex], point: &impl Fn(Point2) -> Point2) -> Vec<Vertex> {
    vertices.iter().map(|vertex| Vertex { point: point(vertex.point), bulge: vertex.bulge }).collect()
}

fn axis_numbers(axis: &Axis) -> Vec<f64> {
    match axis {
        Axis::Line { start, end } => vec![start.x, start.y, end.x, end.y],
        Axis::Arc { start, end, bulge } => vec![start.x, start.y, end.x, end.y, *bulge],
    }
}

fn loop_numbers(vertices: &[Vertex]) -> Vec<f64> {
    vertices.iter().flat_map(|vertex| [vertex.point.x, vertex.point.y, vertex.bulge]).collect()
}

impl Placement {
    /// 🔀️ The placement with every point mapped and every direction turned by `turn` radians.
    pub fn map(&self, point: impl Fn(Point2) -> Point2, turn: f64) -> Placement {
        match self {
            Placement::Wall { axis } => Placement::Wall { axis: axis_map(axis, &point) },
            Placement::CurtainWall { axis } => Placement::CurtainWall { axis: axis_map(axis, &point) },
            Placement::Column { position, rotation, tilt } => Placement::Column { position: point(*position), rotation: turned(*rotation, turn), tilt: tilt.map(|tilt| Slope { direction: turned(tilt.direction, turn), angle: tilt.angle }) },
            Placement::Beam { axis } => Placement::Beam { axis: axis_map(axis, &point) },
            Placement::Slab { boundary, holes, slope } => Placement::Slab {
                boundary: loop_map(boundary, &point),
                holes: holes.iter().map(|hole| loop_map(hole, &point)).collect(),
                slope: slope.map(|slope| Slope { direction: turned(slope.direction, turn), angle: slope.angle }),
            },
            Placement::Ceiling { boundary, holes, slope } => Placement::Ceiling {
                boundary: loop_map(boundary, &point),
                holes: holes.iter().map(|hole| loop_map(hole, &point)).collect(),
                slope: slope.map(|slope| Slope { direction: turned(slope.direction, turn), angle: slope.angle }),
            },
            Placement::Roof { footprint, shape } => Placement::Roof {
                footprint: loop_map(footprint, &point),
                shape: match shape {
                    RoofShape::Shed { pitch, direction } => RoofShape::Shed { pitch: *pitch, direction: turned(*direction, turn) },
                    RoofShape::Gable { pitch, ridge_direction } => RoofShape::Gable { pitch: *pitch, ridge_direction: turned(*ridge_direction, turn) },
                    other => other.clone(),
                },
            },
            Placement::Stair { start, direction, flight } => Placement::Stair { start: point(*start), direction: turned(*direction, turn), flight: flight.clone() },
            Placement::Railing { path } => Placement::Railing { path: path.iter().map(|vertex| point(*vertex)).collect() },
            Placement::Ramp { path } => Placement::Ramp { path: loop_map(path, &point) },
            Placement::Space { boundary } => Placement::Space {
                boundary: match boundary {
                    SpaceBoundary::Bounded { seed } => SpaceBoundary::Bounded { seed: point(*seed) },
                    SpaceBoundary::Explicit { outline } => SpaceBoundary::Explicit { outline: loop_map(outline, &point) },
                },
            },
            Placement::Grid { start, end } => Placement::Grid { start: point(*start), end: point(*end) },
            Placement::Opening { .. } => self.clone(),
        }
    }

    /// ➡️ The placement translated by `vector`.
    pub fn translated(&self, vector: Point2) -> Placement {
        self.map(|point| Point2 { x: point.x + vector.x, y: point.y + vector.y }, 0.0)
    }

    /// 🔄️ The placement turned counter-clockwise by `angle` radians about `pivot`; points are snapped to 1e-12 m so quarter turns stay exact.
    pub fn rotated(&self, pivot: Point2, angle: f64) -> Placement {
        let (sin, cos) = angle.sin_cos();
        self.map(
            |point| {
                let (dx, dy) = (point.x - pivot.x, point.y - pivot.y);
                Point2 { x: snap(pivot.x + dx * cos - dy * sin), y: snap(pivot.y + dx * sin + dy * cos) }
            },
            angle,
        )
    }

    /// 🔢️ Every number of the placement, to prove it finite.
    pub fn numbers(&self) -> Vec<f64> {
        match self {
            Placement::Wall { axis } | Placement::CurtainWall { axis } | Placement::Beam { axis } => axis_numbers(axis),
            Placement::Column { position, rotation, tilt } => [position.x, position.y, *rotation].into_iter().chain(tilt.iter().flat_map(|tilt| [tilt.direction, tilt.angle])).collect(),
            Placement::Grid { start, end } => vec![start.x, start.y, end.x, end.y],
            Placement::Opening { offset, .. } => vec![*offset],
            Placement::Slab { boundary, holes, slope } | Placement::Ceiling { boundary, holes, slope } => loop_numbers(boundary).into_iter().chain(holes.iter().flat_map(|hole| loop_numbers(hole))).chain(slope.iter().flat_map(|slope| [slope.direction, slope.angle])).collect(),
            Placement::Roof { footprint, shape } => loop_numbers(footprint)
                .into_iter()
                .chain(match shape {
                    RoofShape::Flat => Vec::new(),
                    RoofShape::Shed { pitch, direction } => vec![*pitch, *direction],
                    RoofShape::Gable { pitch, ridge_direction } => vec![*pitch, *ridge_direction],
                    RoofShape::Hip { pitch } => vec![*pitch],
                    RoofShape::Mansard { lower_pitch, upper_pitch, break_height } => vec![*lower_pitch, *upper_pitch, *break_height],
                })
                .collect(),
            Placement::Stair { start, direction, .. } => vec![start.x, start.y, *direction],
            Placement::Railing { path } => path.iter().flat_map(|point| [point.x, point.y]).collect(),
            Placement::Ramp { path } => loop_numbers(path),
            Placement::Space { boundary } => match boundary {
                SpaceBoundary::Bounded { seed } => vec![seed.x, seed.y],
                SpaceBoundary::Explicit { outline } => loop_numbers(outline),
            },
        }
    }

    /// 🧬️ Whether both placements belong to the same element kind.
    pub fn same_kind(&self, other: &Placement) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

/// 📍️ The current placement of a placed element, `None` for an id that is no placed element.
pub fn placement(base: &ModelSnapshot, id: &str) -> Option<Placement> {
    if let Some(wall) = base.walls.get(id) {
        return Some(Placement::Wall { axis: wall.axis.clone() });
    }
    if let Some(wall) = base.curtain_walls.get(id) {
        return Some(Placement::CurtainWall { axis: wall.axis.clone() });
    }
    if let Some(column) = base.columns.get(id) {
        return Some(Placement::Column { position: column.position, rotation: column.rotation, tilt: column.tilt });
    }
    if let Some(beam) = base.beams.get(id) {
        return Some(Placement::Beam { axis: beam.axis.clone() });
    }
    if let Some(slab) = base.slabs.get(id) {
        return Some(Placement::Slab { boundary: slab.boundary.clone(), holes: slab.holes.clone(), slope: slab.slope });
    }
    if let Some(ceiling) = base.ceilings.get(id) {
        return Some(Placement::Ceiling { boundary: ceiling.boundary.clone(), holes: ceiling.holes.clone(), slope: ceiling.slope });
    }
    if let Some(roof) = base.roofs.get(id) {
        return Some(Placement::Roof { footprint: roof.footprint.clone(), shape: roof.shape.clone() });
    }
    if let Some(stair) = base.stairs.get(id) {
        return Some(Placement::Stair { start: stair.start, direction: stair.direction, flight: Some(stair.flight.clone()) });
    }
    if let Some(railing) = base.railings.get(id) {
        return Some(Placement::Railing { path: railing.path.clone() });
    }
    if let Some(ramp) = base.ramps.get(id) {
        return Some(Placement::Ramp { path: ramp.path.clone() });
    }
    if let Some(space) = base.spaces.get(id) {
        return Some(Placement::Space { boundary: space.boundary.clone() });
    }
    base.grids.get(id).map(|grid| Placement::Grid { start: grid.start, end: grid.end })
}

/// 📍️ The placement of any element that has a state a placement can restore: a placed element, or an opening with its offset along the
/// host and its two flips. Openings follow their host by inference, so a move and a rotation never read this; the exact inverse of a
/// mirror writes it back in one diff, because restoring openings one by one could collide with openings that are still mirrored.
pub fn state(base: &ModelSnapshot, id: &str) -> Option<Placement> {
    placement(base, id).or_else(|| base.openings.get(id).map(|opening| Placement::Opening { offset: opening.offset, flip_hand: opening.flip_hand, flip_facing: opening.flip_facing }))
}

fn patched<T: Clone + PartialEq, P: Patch<T>>(slot: Option<KeyedDelta<T, P>>, id: &str, patch: P) -> Option<KeyedDelta<T, P>> {
    let mut delta = slot.unwrap_or_default();
    delta.0.insert(id.to_string(), Entry::Patched(patch));
    Some(delta)
}

/// 🔺️ `next` when it differs from `prior`, nothing otherwise: a patch carries exactly the fields that move.
fn moved<T: Clone + PartialEq>(next: &T, prior: Option<&T>) -> Option<T> {
    (prior != Some(next)).then(|| next.clone())
}

/// 🔺️ The sparse diff that writes every placement of `after` onto its element: one patch per element holding exactly the placement
/// fields that differ from the same element in `before`.
pub fn placement_diff(before: &BTreeMap<String, Placement>, after: &BTreeMap<String, Placement>) -> ModelDiff {
    let mut diff = ModelDiff::default();
    for (id, place) in after {
        let prior = before.get(id);
        match place {
            Placement::Wall { axis } => {
                let was = if let Some(Placement::Wall { axis }) = prior { Some(axis) } else { None };
                diff.walls = patched(diff.walls.take(), id, WallPatch { axis: moved(axis, was), ..Default::default() });
            }
            Placement::CurtainWall { axis } => {
                let was = if let Some(Placement::CurtainWall { axis }) = prior { Some(axis) } else { None };
                diff.curtain_walls = patched(diff.curtain_walls.take(), id, CurtainWallPatch { axis: moved(axis, was), ..Default::default() });
            }
            Placement::Column { position, rotation, tilt } => {
                let (was_position, was_rotation, was_tilt) = if let Some(Placement::Column { position, rotation, tilt }) = prior { (Some(position), Some(rotation), Some(tilt)) } else { (None, None, None) };
                diff.columns = patched(diff.columns.take(), id, ColumnPatch { position: moved(position, was_position), rotation: moved(rotation, was_rotation), tilt: moved(tilt, was_tilt).map(Assigned::new), ..Default::default() });
            }
            Placement::Beam { axis } => {
                let was = if let Some(Placement::Beam { axis }) = prior { Some(axis) } else { None };
                diff.beams = patched(diff.beams.take(), id, BeamPatch { axis: moved(axis, was), ..Default::default() });
            }
            Placement::Slab { boundary, holes, slope } => {
                let (was_boundary, was_holes, was_slope) = if let Some(Placement::Slab { boundary, holes, slope }) = prior { (Some(boundary), Some(holes), Some(slope)) } else { (None, None, None) };
                diff.slabs = patched(diff.slabs.take(), id, SlabPatch { boundary: moved(boundary, was_boundary), holes: moved(holes, was_holes), slope: moved(slope, was_slope).map(Assigned::new), ..Default::default() });
            }
            Placement::Ceiling { boundary, holes, slope } => {
                let (was_boundary, was_holes, was_slope) = if let Some(Placement::Ceiling { boundary, holes, slope }) = prior { (Some(boundary), Some(holes), Some(slope)) } else { (None, None, None) };
                diff.ceilings = patched(diff.ceilings.take(), id, CeilingPatch { boundary: moved(boundary, was_boundary), holes: moved(holes, was_holes), slope: moved(slope, was_slope).map(Assigned::new), ..Default::default() });
            }
            Placement::Roof { footprint, shape } => {
                let (was_footprint, was_shape) = if let Some(Placement::Roof { footprint, shape }) = prior { (Some(footprint), Some(shape)) } else { (None, None) };
                diff.roofs = patched(diff.roofs.take(), id, RoofPatch { footprint: moved(footprint, was_footprint), shape: moved(shape, was_shape), ..Default::default() });
            }
            Placement::Stair { start, direction, flight } => {
                let (was_start, was_direction, was_flight) = if let Some(Placement::Stair { start, direction, flight }) = prior { (Some(start), Some(direction), flight.as_ref()) } else { (None, None, None) };
                diff.stairs = patched(diff.stairs.take(), id, StairPatch { start: moved(start, was_start), direction: moved(direction, was_direction), flight: flight.as_ref().and_then(|flight| moved(flight, was_flight)), ..Default::default() });
            }
            Placement::Railing { path } => {
                let was = if let Some(Placement::Railing { path }) = prior { Some(path) } else { None };
                diff.railings = patched(diff.railings.take(), id, RailingPatch { path: moved(path, was), ..Default::default() });
            }
            Placement::Ramp { path } => {
                let was = if let Some(Placement::Ramp { path }) = prior { Some(path) } else { None };
                diff.ramps = patched(diff.ramps.take(), id, RampPatch { path: moved(path, was), ..Default::default() });
            }
            Placement::Space { boundary } => {
                let was = if let Some(Placement::Space { boundary }) = prior { Some(boundary) } else { None };
                diff.spaces = patched(diff.spaces.take(), id, SpacePatch { boundary: moved(boundary, was), ..Default::default() });
            }
            Placement::Grid { start, end } => {
                let (was_start, was_end) = if let Some(Placement::Grid { start, end }) = prior { (Some(start), Some(end)) } else { (None, None) };
                diff.grids = patched(diff.grids.take(), id, GridLinePatch { start: moved(start, was_start), end: moved(end, was_end), ..Default::default() });
            }
            Placement::Opening { offset, flip_hand, flip_facing } => {
                let (was_offset, was_hand, was_facing) = if let Some(Placement::Opening { offset, flip_hand, flip_facing }) = prior { (Some(offset), Some(flip_hand), Some(flip_facing)) } else { (None, None, None) };
                diff.openings = patched(diff.openings.take(), id, OpeningPatch { offset: moved(offset, was_offset), flip_hand: moved(flip_hand, was_hand), flip_facing: moved(flip_facing, was_facing), ..Default::default() });
            }
        }
    }
    diff
}

/// 🚫️ A refusal in the making: the outcome code, the human message and the target path.
pub struct Refusal {
    pub code: OutcomeCode,
    pub message: String,
    pub path: Vec<String>,
}

impl Refusal {
    /// 🏗️ A refusal with `code`, `message` and the given path segments.
    pub fn new<S: Into<String>>(code: OutcomeCode, message: impl Into<String>, path: impl IntoIterator<Item = S>) -> Self {
        Self { code, message: message.into(), path: path.into_iter().map(Into::into).collect() }
    }

    /// 🚫️ The refused outcome (no diff).
    pub fn outcome(self) -> MutationOutcome<ModelDiff> {
        MutationOutcome::refuse(self.code, self.message, self.path)
    }
}

/// 🔀️ What a multi-element mutation changes: the base placement and the new placement of every element whose placement differs.
pub struct Change {
    pub before: BTreeMap<String, Placement>,
    pub after: BTreeMap<String, Placement>,
}

/// 🧲️ The selected elements of `ids` placed by `transform`: unknown ids are `TargetMissing`, elements that are no placed kind
/// (sites, buildings, storeys) `Invariant`, openings are skipped because they follow their host by inference, and elements whose
/// placement does not change are left out.
pub fn changes(base: &ModelSnapshot, ids: &[String], transform: impl Fn(&Placement) -> Placement) -> Result<Change, Refusal> {
    changes_by(base, ids, |_, placement| transform(placement))
}

/// 🧲️ [`changes`] with a transform that also sees the id it places, for a placement that differs per element (an alignment to a line).
pub fn changes_by(base: &ModelSnapshot, ids: &[String], transform: impl Fn(&str, &Placement) -> Placement) -> Result<Change, Refusal> {
    if ids.is_empty() {
        return Err(Refusal::new(OutcomeCode::Invariant, "No element is selected.", ["ids"]));
    }
    let mut change = Change { before: BTreeMap::new(), after: BTreeMap::new() };
    for id in ids {
        match placement(base, id) {
            Some(before) => {
                let after = transform(id, &before);
                if !after.numbers().iter().all(|number| number.is_finite()) {
                    return Err(Refusal::new(OutcomeCode::Invariant, format!("Element \"{id}\" would leave the finite plane."), [id.clone()]));
                }
                if after != before {
                    change.before.insert(id.clone(), before);
                    change.after.insert(id.clone(), after);
                }
            }
            None if base.openings.contains_key(id) => {}
            None if exists(base, id) => return Err(Refusal::new(OutcomeCode::Invariant, format!("Element \"{id}\" has no placement of its own."), [id.clone()])),
            None => return Err(Refusal::new(OutcomeCode::TargetMissing, format!("Element \"{id}\" does not exist."), [id.clone()])),
        }
    }
    Ok(change)
}

/// 🔀️ The outcome of a multi-element placement: the sparse placement diff, or `NoOp` when no placement changes.
pub fn placed(change: Result<Change, Refusal>, field: &str) -> MutationOutcome<ModelDiff> {
    match change {
        Err(refusal) => refusal.outcome(),
        Ok(change) if change.after.is_empty() => MutationOutcome::refuse(OutcomeCode::NoOp, "Every element already has this placement.", [field]),
        Ok(change) => MutationOutcome::new(placement_diff(&change.before, &change.after)),
    }
}
//#endregion 🔖️Placement

//#region 🔖️Identity
/// 🔎️ Whether `id` names an element: a site, building, storey, grid line, wall, curtain wall, column, beam, slab, roof, opening, stair, railing, ramp, space, zone or area scheme.
pub fn exists(base: &ModelSnapshot, id: &str) -> bool {
    base.sites.contains_key(id)
        || base.buildings.contains_key(id)
        || base.storeys.contains_key(id)
        || base.grids.contains_key(id)
        || base.walls.contains_key(id)
        || base.curtain_walls.contains_key(id)
        || base.columns.contains_key(id)
        || base.beams.contains_key(id)
        || base.slabs.contains_key(id)
        || base.ceilings.contains_key(id)
        || base.roofs.contains_key(id)
        || base.openings.contains_key(id)
        || base.wall_sweeps.contains_key(id)
        || base.stairs.contains_key(id)
        || base.railings.contains_key(id)
        || base.ramps.contains_key(id)
        || base.spaces.contains_key(id)
        || base.zones.contains_key(id)
        || base.area_schemes.contains_key(id)
        || base.views.contains_key(id)
        || base.sheets.contains_key(id)
        || base.viewports.contains_key(id)
        || base.sheet_revisions.contains_key(id)
}

/// 🏷️ Whether `id` names a holder of properties and classifications: an element (see [`exists`]) or a record of a type library, whose data its instances inherit.
pub fn holds_data(base: &ModelSnapshot, id: &str) -> bool {
    exists(base, id)
        || base.wall_types.contains_key(id)
        || base.slab_types.contains_key(id)
        || base.ceiling_types.contains_key(id)
        || base.roof_types.contains_key(id)
        || base.column_types.contains_key(id)
        || base.beam_types.contains_key(id)
        || base.window_types.contains_key(id)
        || base.door_types.contains_key(id)
}

macro_rules! noun {
    ($base:expr, $id:expr, $($collection:ident => $noun:literal),+ $(,)?) => {{
        $( if $base.$collection.contains_key($id) { return Some($noun); } )+
        None
    }};
}

/// 🪪 The kind of record that already uses the id `id`, `None` when it is free. Ids are unique across every collection of the model
/// (elements, materials and every type library), so every `create-*` diff and `split-wall` refuse `mutation.duplicate-id` on a hit
/// of any kind, never only of their own collection.
pub fn taken(base: &ModelSnapshot, id: &str) -> Option<&'static str> {
    noun!(
        base, id,
        sites => "Site",
        buildings => "Building",
        storeys => "Storey",
        grids => "Grid line",
        walls => "Wall",
        curtain_walls => "Curtain wall",
        curtain_panel_overrides => "Curtain panel override",
        columns => "Column",
        beams => "Beam",
        slabs => "Slab",
        ceilings => "Ceiling",
        roofs => "Roof",
        openings => "Opening",
        wall_sweeps => "Wall sweep",
        stairs => "Stair",
        railings => "Railing",
        ramps => "Ramp",
        spaces => "Space",
        zones => "Zone",
        area_schemes => "Area scheme",
        views => "View",
        sheets => "Sheet",
        viewports => "Viewport",
        sheet_revisions => "Sheet revision",
        materials => "Material",
        wall_types => "Wall type",
        slab_types => "Slab type",
        ceiling_types => "Ceiling type",
        roof_types => "Roof type",
        column_types => "Column type",
        beam_types => "Beam type",
        window_types => "Window type",
        door_types => "Door type",
        curtain_wall_types => "Curtain wall type",
        dimensions => "Dimension",
        tags => "Tag",
        text_notes => "Text note",
        leaders => "Leader",
        annotation_styles => "Annotation style",
        schedules => "Schedule",
        families => "Family",
        family_parameters => "Family parameter",
        family_solids => "Family solid",
        property_templates => "Property template",
        classification_systems => "Classification system",
    )
}

macro_rules! named {
    ($base:expr, $id:expr, $name:expr, $($collection:ident => $patch:ident . $slot:ident),+ $(,)?) => {{
        $(
            if let Some(row) = $base.$collection.get($id) {
                let diff = ModelDiff::$collection($id.to_string(), Entry::Patched(crate::$patch { $slot: Some($name.to_string()), ..Default::default() }));
                return Some((row.$slot.clone(), diff));
            }
        )+
        None
    }};
}

/// 🏷️ The current name of the element `id` (a grid line's label) with the one-field diff that sets it to `name`; `None` for an unknown id.
pub fn rename(base: &ModelSnapshot, id: &str, name: &str) -> Option<(String, ModelDiff)> {
    named!(
        base, id, name,
        sites => SitePatch.name,
        buildings => BuildingPatch.name,
        storeys => StoreyPatch.name,
        grids => GridLinePatch.label,
        walls => WallPatch.name,
        curtain_walls => CurtainWallPatch.name,
        columns => ColumnPatch.name,
        beams => BeamPatch.name,
        slabs => SlabPatch.name,
        ceilings => CeilingPatch.name,
        roofs => RoofPatch.name,
        openings => OpeningPatch.name,
        wall_sweeps => WallSweepPatch.name,
        stairs => StairPatch.name,
        railings => RailingPatch.name,
        ramps => RampPatch.name,
        spaces => SpacePatch.name,
        zones => ZonePatch.name,
        area_schemes => AreaSchemePatch.name,
    )
}
/// 🏘️ The ids of the spaces that belong to the zone `zone`, in id order.
pub fn zone_members(base: &ModelSnapshot, zone: &str) -> Vec<String> {
    base.spaces.iter().filter(|(_, space)| space.zone.as_deref() == Some(zone)).map(|(id, _)| id.clone()).collect()
}

/// 🗃️ Why the rule lists of an area scheme cannot be authored: the outcome code, the message and the name of the offending list (a zone that does not exist, a blank usage).
pub fn scheme_rule_issue(base: &ModelSnapshot, usages: &[String], zones: &[String]) -> Option<(OutcomeCode, String, &'static str)> {
    if let Some(zone) = zones.iter().find(|zone| !base.zones.contains_key(*zone)) {
        return Some((OutcomeCode::TargetMissing, format!("Zone \"{zone}\" does not exist."), "zones"));
    }
    usages.iter().any(|usage| usage.trim().is_empty()).then(|| (OutcomeCode::Invariant, "A counted usage must not be blank.".to_string(), "usages"))
}
//#endregion 🔖️Identity

//#region 🔖️Storey
macro_rules! storeyed {
    ($base:expr, $id:expr, $($collection:ident),+ $(,)?) => {{
        $( if let Some(row) = $base.$collection.get($id) { return Some(row.storey.clone()); } )+
        None
    }};
}

macro_rules! restoreyed {
    ($base:expr, $id:expr, $storey:expr, $($collection:ident => $patch:ident),+ $(,)?) => {{
        $( if $base.$collection.contains_key($id) { return Some(ModelDiff::$collection($id.to_string(), Entry::Patched(crate::$patch { storey: Some($storey.to_string()), ..Default::default() }))); } )+
        None
    }};
}

/// 🪜️ The storey the element `id` stands on, `None` for an id that is no storey-placed element (an opening stands on the storey of its host, a grid line, site, building or storey on none).
pub fn storey_of(base: &ModelSnapshot, id: &str) -> Option<String> {
    storeyed!(base, id, walls, curtain_walls, columns, beams, slabs, ceilings, roofs, stairs, railings, ramps, spaces)
}

/// 🪜️ The one-field diff that stands the storey-placed element `id` on `storey`, `None` for an id that is no storey-placed element.
pub fn restorey(base: &ModelSnapshot, id: &str, storey: &str) -> Option<ModelDiff> {
    restoreyed!(
        base, id, storey,
        walls => WallPatch,
        curtain_walls => CurtainWallPatch,
        columns => ColumnPatch,
        beams => BeamPatch,
        slabs => SlabPatch,
        ceilings => CeilingPatch,
        roofs => RoofPatch,
        stairs => StairPatch,
        railings => RailingPatch,
        ramps => RampPatch,
        spaces => SpacePatch,
    )
}

/// 🔝️ The base offset and the top constraint that resolve the height of the element `id`, `None` for an element whose extent no top constraint drives (a stair starts at its storey).
pub fn vertical_of<'a>(base: &'a ModelSnapshot, id: &str) -> Option<(f64, &'a crate::TopConstraint)> {
    base.walls
        .get(id)
        .map(|row| (row.base_offset, &row.top))
        .or_else(|| base.curtain_walls.get(id).map(|row| (row.base_offset, &row.top)))
        .or_else(|| base.columns.get(id).map(|row| (row.base_offset, &row.top)))
        .or_else(|| base.stairs.get(id).map(|row| (0.0, &row.top)))
        .or_else(|| base.ramps.get(id).map(|row| (row.base_offset, &row.top)))
}
//#endregion 🔖️Storey

//#region 🔖️Phase
macro_rules! phased {
    ($base:expr, $id:expr, $($collection:ident),+ $(,)?) => {{
        $( if let Some(row) = $base.$collection.get($id) { return Some(row.phase); } )+
        None
    }};
}

macro_rules! rephased {
    ($base:expr, $id:expr, $phase:expr, $($collection:ident => $patch:ident),+ $(,)?) => {{
        $( if $base.$collection.contains_key($id) { return Some(ModelDiff::$collection($id.to_string(), Entry::Patched(crate::$patch { phase: Some($phase), ..Default::default() }))); } )+
        None
    }};
}

/// 🕰️ The construction phase of the element `id`, `None` for an id that carries none (an opening takes the phase of its host, a grid line, site, building or storey has none).
pub fn phase_of(base: &ModelSnapshot, id: &str) -> Option<crate::Phase> {
    phased!(base, id, walls, curtain_walls, columns, beams, slabs, roofs, stairs, railings, spaces)
}

/// 🕰️ The one-field diff that puts the phasable element `id` into `phase`, `None` for an id that carries no phase.
pub fn rephase(base: &ModelSnapshot, id: &str, phase: crate::Phase) -> Option<ModelDiff> {
    rephased!(
        base, id, phase,
        walls => WallPatch,
        curtain_walls => CurtainWallPatch,
        columns => ColumnPatch,
        beams => BeamPatch,
        slabs => SlabPatch,
        roofs => RoofPatch,
        stairs => StairPatch,
        railings => RailingPatch,
        spaces => SpacePatch,
    )
}
//#endregion 🔖️Phase
