//! 📏️ The analysis.measure widget computes: volume, area, length, centroid, bounds, mass properties, distances, point queries and angles of B-Rep shapes.
//!
//! Every measure reads the owned [`ShapeValue`] of its input directly through the kernel's pure analysis queries, so no kernel session is
//! needed except where two shapes must share one (distance) or a solid is built (the bounding box). Work that adds up over independent
//! members (the solids of a compound, the faces of a shape) is a stepped [`UnitJob`]: one member per unit of fuel, so a large input never
//! outruns the interactive ceiling. A query the kernel answers atomically (the distance between two shapes, the closest point) is one unit.
//!
//! 🔗️ [Divergence theorem](https://en.wikipedia.org/wiki/Divergence_theorem) · [Moment of inertia](https://en.wikipedia.org/wiki/Moment_of_inertia) · [Axis-aligned bounding box](https://en.wikipedia.org/wiki/Minimum_bounding_box)

use super::math_arithmetic::{finite, out_of_range};
use super::math_vector::{angle_between, sub};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::brep::engine::{BrepError, GeometryHandle, PointClassification, ShapeRoot, ShapeValue};
use semio_framework_3d::brep::queries::analysis::{bounding_box, edge_table, face_table, mass_properties, point_distance, CurveKind, ShapeScope, SurfaceKind};
use semio_framework_3d::brep::queries::classification::point_in_solid;
use semio_framework_3d::brep::queries::mass_properties::{face_area, solid_mass_properties, solid_volume};
use semio_framework_3d::brep::representation::arena::{FaceId, SolidId};
use semio_framework_3d::brep::representation::error::KernelError;
use semio_framework_3d::brep::representation::vector::Pnt3;
use semio_framework_3d::inertia::{combine, principal_inertia, InertiaPiece};
use std::collections::BTreeSet;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

const TOLERANCE: f64 = 1e-6;
const CLASSIFY_TOLERANCE: f64 = 1e-6;

//#region 🔖️Faults
/// 🛠️ A kernel analysis query that refused its input, as a widget fault.
pub(crate) fn query_fault(error: KernelError) -> WidgetFault {
    kernel_fault(&BrepError::Operation(error.to_string()))
}

/// 🚫️ The measure does not apply to this kind of shape, attributed to the shape port.
pub(crate) fn unsupported(port: &str, en: &str, de: &str) -> WidgetFault {
    WidgetFault::new("generation3d.geometry.measure-unsupported", en, de).at(port)
}

/// 🚫️ A bare curve or surface has no topology to analyse.
pub(crate) fn no_topology(port: &str) -> WidgetFault {
    WidgetFault::new("generation3d.geometry.no-topology", "A bare curve or surface has no topology to analyse; use a shape built from edges or faces.", "Eine reine Kurve oder Fläche hat keine Topologie zum Analysieren; verwende eine Form aus Kanten oder Flächen.").at(port)
}

/// 🚫️ The job lost the state it needs; it cannot happen unless the job is stepped after it ended.
pub(crate) fn lost_state() -> WidgetFault {
    WidgetFault::new("generation3d.geometry.pipeline", "The computation was set up without the value it needs.", "Die Berechnung wurde ohne den benötigten Wert aufgesetzt.")
}

/// 🛑️ The fault of a job that was cancelled before it finished.
pub(crate) fn cancelled() -> WidgetFault {
    WidgetFault::new("generation3d.geometry.cancelled", "The computation was cancelled.", "Die Berechnung wurde abgebrochen.")
}

fn panicked() -> WidgetFault {
    WidgetFault::new("generation3d.geometry.kernel", "The geometry kernel stopped unexpectedly while computing this widget.", "Der Geometriekern wurde beim Berechnen dieses Widgets unerwartet beendet.")
}

fn stale(port: &str) -> WidgetFault {
    WidgetFault::new("generation3d.geometry.selection-stale", "The selected faces no longer exist on the input shape; select them again.", "Die gewählten Flächen existieren an der Eingabeform nicht mehr; wähle sie erneut.").at(port)
}

fn wrong_component(port: &str) -> WidgetFault {
    WidgetFault::new("generation3d.geometry.selection-component", "This input takes faces but the selection holds other elements.", "Dieser Eingang erwartet Flächen, die Auswahl enthält aber andere Elemente.").at(port)
}

/// 🔎️ The `index`-th of `items`, or the lost-state fault.
pub(crate) fn nth<T: Copy>(items: &[T], index: usize) -> Result<T, WidgetFault> {
    items.get(index).copied().ok_or_else(lost_state)
}
//#endregion 🔖️Faults

//#region 🔖️Job
type Work<S> = fn(&mut S, usize) -> Result<(), WidgetFault>;
type Conclude<S> = fn(S, Quality) -> Result<(Outputs, Quality), WidgetFault>;

/// 🪜️ A widget job of `total` independent units over one state: each call to `step` runs up to `fuel` units, the last one concludes into outputs.
///
/// A panic inside a unit becomes a `generation3d.geometry.kernel` fault instead of tearing the engine down.
pub(crate) struct UnitJob<S: Send> {
    quality: Quality,
    state: Option<S>,
    total: usize,
    done: usize,
    work: Work<S>,
    conclude: Conclude<S>,
    finished: Option<WidgetEvaluation>,
}

impl<S: Send> UnitJob<S> {
    fn end(&mut self, result: Result<(Outputs, Quality), WidgetFault>) -> WidgetStep {
        self.state = None;
        let evaluation = match result {
            Ok((outputs, quality)) => WidgetEvaluation::ok(outputs, quality),
            Err(fault) => WidgetEvaluation::faulted(fault, self.quality),
        };
        self.finished = Some(evaluation.clone());
        WidgetStep::Done(evaluation)
    }
}

impl<S: Send> WidgetJob for UnitJob<S> {
    fn step(&mut self, fuel: usize) -> WidgetStep {
        if let Some(evaluation) = &self.finished {
            return WidgetStep::Done(evaluation.clone());
        }
        let mut budget = fuel.max(1);
        while self.done < self.total && budget > 0 {
            let (work, index) = (self.work, self.done);
            let Some(state) = self.state.as_mut() else { return self.end(Err(lost_state())) };
            if let Err(fault) = catch_unwind(AssertUnwindSafe(|| work(state, index))).unwrap_or_else(|_| Err(panicked())) {
                return self.end(Err(fault));
            }
            self.done += 1;
            budget -= 1;
        }
        if self.done < self.total {
            return WidgetStep::Working { progress: (self.done as f32 / self.total as f32).min(0.999) };
        }
        let (conclude, quality) = (self.conclude, self.quality);
        let Some(state) = self.state.take() else { return self.end(Err(lost_state())) };
        let result = catch_unwind(AssertUnwindSafe(|| conclude(state, quality))).unwrap_or_else(|_| Err(panicked()));
        self.end(result)
    }

    fn cancel(&mut self) {
        if self.finished.is_none() {
            self.end(Err(cancelled()));
        }
    }
}

/// 🚀️ Starts a [`UnitJob`] over `prepared` (the state and its unit count), or answers with the fault that stopped it from being prepared.
pub(crate) fn staged<S: Send + 'static>(kind: &Kind, prepared: Result<(S, usize), WidgetFault>, work: Work<S>, conclude: Conclude<S>) -> Box<dyn WidgetJob> {
    match prepared {
        Ok((state, total)) => Box::new(UnitJob { quality: kind.quality, state: Some(state), total, done: 0, work, conclude, finished: None }),
        Err(fault) => failed(fault, kind.quality),
    }
}
//#endregion 🔖️Job

//#region 🔖️Scope
/// 🎯️ The analysis scope a shape value's root names; a bare curve or surface has none.
pub(crate) fn scope_of(shape: &ShapeValue, port: &str) -> Result<ShapeScope, WidgetFault> {
    match &shape.root {
        ShapeRoot::Vertex(id) => Ok(ShapeScope::vertex(*id)),
        ShapeRoot::Edge(id) => Ok(ShapeScope::edge(*id)),
        ShapeRoot::Wire(wire) => Ok(ShapeScope::wire(&wire.members.iter().map(|(edge, _)| *edge).collect::<Vec<_>>(), &wire.vertices)),
        ShapeRoot::Face(id) => Ok(ShapeScope::face(*id)),
        ShapeRoot::Shell(id) => Ok(ShapeScope::shell(*id)),
        ShapeRoot::Solid(id) => Ok(ShapeScope::solid(*id)),
        ShapeRoot::Compound { solids, .. } => Ok(ShapeScope::compound(solids)),
        ShapeRoot::Curve { .. } | ShapeRoot::Surface { .. } => Err(no_topology(port)),
    }
}

/// 🧱️ The solids a shape reaches, ascending by arena index.
fn solids_of(shape: &ShapeValue, port: &str) -> Result<Vec<SolidId>, WidgetFault> {
    Ok(scope_of(shape, port)?.members(&shape.body).map_err(query_fault)?.solids)
}

/// 🧱️ The faces a shape reaches, ascending by arena index.
fn faces_of(shape: &ShapeValue, port: &str) -> Result<Vec<FaceId>, WidgetFault> {
    Ok(scope_of(shape, port)?.members(&shape.body).map_err(query_fault)?.faces)
}
//#endregion 🔖️Scope

//#region 🔖️Volume
struct Volume {
    shape: Arc<ShapeValue>,
    solids: Vec<SolidId>,
    total: f64,
}

/// 🧊️ The enclosed volume, one solid per unit; the volumes of a compound's solids are added, overlaps are not merged.
fn volume(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    let prepared = (|| {
        let shape = inputs.shape("solid")?.clone();
        let solids = solids_of(&shape, "solid")?;
        let total = solids.len();
        Ok((Volume { shape, solids, total: 0.0 }, total))
    })();
    staged(
        kind,
        prepared,
        |state, index| {
            state.total += solid_volume(&state.shape.body, nth(&state.solids, index)?, TOLERANCE).map_err(query_fault)?;
            Ok(())
        },
        |state, quality| Ok((outputs([("volume", GeometryValue::Number(finite(state.total)?))]), quality)),
    )
}
//#endregion 🔖️Volume

//#region 🔖️Area
struct Area {
    shape: Arc<ShapeValue>,
    faces: Vec<FaceId>,
    total: f64,
}

/// 🧱️ The state of an area sum over `faces` of `shape`, one face per unit.
fn area_state(shape: Arc<ShapeValue>, faces: Vec<FaceId>) -> Result<(Area, usize), WidgetFault> {
    let total = faces.len();
    Ok((Area { shape, faces, total: 0.0 }, total))
}

/// 🧮️ Adds the area of the face a unit names.
fn add_face_area(state: &mut Area, index: usize) -> Result<(), WidgetFault> {
    state.total += face_area(&state.shape.body, nth(&state.faces, index)?, TOLERANCE).map_err(query_fault)?;
    Ok(())
}

/// 🧮️ Concludes an area sum into the `area` output.
fn area_output(state: Area, quality: Quality) -> Result<(Outputs, Quality), WidgetFault> {
    Ok((outputs([("area", GeometryValue::Number(finite(state.total)?))]), quality))
}

/// 🧊️ The total area of all faces of a shape, one face per unit.
fn area(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    let prepared = (|| {
        let shape = inputs.shape("shape")?.clone();
        let faces = faces_of(&shape, "shape")?;
        if faces.is_empty() {
            return Err(unsupported("shape", "A vertex, edge, wire or curve has no area; use a face, shell, solid or compound.", "Ein Punkt, eine Kante, ein Kantenzug oder eine Kurve hat keine Fläche; verwende eine Fläche, eine Schale, einen Körper oder eine Gruppe."));
        }
        area_state(shape, faces)
    })();
    staged(kind, prepared, add_face_area, area_output)
}

/// 🧲️ The summed area of the selected faces, one face per unit; the selection holds persistent labels of the shape's faces.
fn selection_area(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    let prepared = (|| {
        let shape = inputs.shape("shape")?.clone();
        let selection = inputs.selection("faces")?;
        if selection.component != SelectionKind::Face {
            return Err(wrong_component("faces"));
        }
        let wanted: BTreeSet<u64> = selection.ids.iter().copied().collect();
        let faces: Vec<FaceId> = shape.body.faces.iter().filter(|(_, face)| wanted.contains(&face.label.0)).map(|(id, _)| id).collect();
        if faces.len() != wanted.len() || faces.is_empty() {
            return Err(stale("faces"));
        }
        area_state(shape, faces)
    })();
    staged(kind, prepared, add_face_area, area_output)
}
//#endregion 🔖️Area

//#region 🔖️Length
/// 📏️ The length of a curve, wire or edge: a shape imported into a fresh session and measured there, where bare curves are measured too.
fn length(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let mut session = KernelSession::new();
            let imported = session.import(inputs.shape("curve")?)?;
            let length = session.brep().length_sync(&imported.handle).map_err(|error| kernel_fault(&error))?;
            Ok(outputs([("length", GeometryValue::Number(finite(length)?))]))
        })(),
    )
}
//#endregion 🔖️Length

//#region 🔖️Mass
struct Pieces {
    shape: Arc<ShapeValue>,
    solids: Vec<SolidId>,
    pieces: Vec<InertiaPiece>,
}

/// 🧱️ The state of a mass sum over the solids of `shape`, one solid per unit.
fn pieces_state(shape: &Arc<ShapeValue>, port: &str) -> Result<(Pieces, usize), WidgetFault> {
    let solids = solids_of(shape, port)?;
    if solids.is_empty() {
        return Err(unsupported(port, "The shape holds no solid, so it has no volume.", "Die Form enthält keinen Körper und hat damit kein Volumen."));
    }
    let total = solids.len();
    Ok((Pieces { shape: shape.clone(), solids, pieces: Vec::new() }, total))
}

/// 🧮️ Integrates the solid a unit names into one rigid piece at unit density.
fn add_piece(state: &mut Pieces, index: usize) -> Result<(), WidgetFault> {
    let mass = solid_mass_properties(&state.shape.body, nth(&state.solids, index)?, TOLERANCE).map_err(query_fault)?;
    state.pieces.push(InertiaPiece { mass: mass.volume, centroid: [mass.centroid.x, mass.centroid.y, mass.centroid.z], about_centroid: mass.inertia });
    Ok(())
}

/// 🛞️ The pieces combined into one rigid body, or the `measure-unsupported` fault of a shape without volume.
fn combined(state: &Pieces, port: &str) -> Result<InertiaPiece, WidgetFault> {
    combine(&state.pieces).ok_or_else(|| unsupported(port, "The shape encloses no volume.", "Die Form schließt kein Volumen ein."))
}

/// 🎯️ The `centroid` output holding one point.
fn centroid_output(value: [f64; 3]) -> Result<Outputs, WidgetFault> {
    Ok(outputs([("centroid", GeometryValue::Point(value))]))
}

/// 🎯️ The center of mass: of the enclosed volume for solids, compounds and closed shells, of the surface for open shells and faces, the position of a vertex.
fn centroid(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    let shape = match inputs.shape("shape") {
        Ok(shape) => shape.clone(),
        Err(fault) => return failed(fault, kind.quality),
    };
    match &shape.root {
        ShapeRoot::Solid(_) | ShapeRoot::Compound { .. } => staged(
            kind,
            pieces_state(&shape, "shape"),
            add_piece,
            |state, quality| {
                let whole = combined(&state, "shape")?;
                Ok((outputs([("centroid", GeometryValue::Point(whole.centroid))]), quality))
            },
        ),
        ShapeRoot::Vertex(id) => finish(kind, shape.body.vertices.get(*id).ok_or_else(lost_state).and_then(|vertex| centroid_output([vertex.position.x, vertex.position.y, vertex.position.z]))),
        ShapeRoot::Shell(_) | ShapeRoot::Face(_) => finish(
            kind,
            (|| {
                let mass = mass_properties(&shape.body, &scope_of(&shape, "shape")?, TOLERANCE).map_err(query_fault)?;
                let found = mass.volumetric.map(|volumetric| volumetric.centroid).or(mass.area_centroid);
                centroid_output(found.ok_or_else(|| unsupported("shape", "The shape has no area to take a center of.", "Die Form hat keine Fläche, aus der ein Schwerpunkt entsteht."))?)
            })(),
        ),
        _ => failed(unsupported("shape", "An edge, wire, curve or surface has no center of mass; use a vertex, face, shell, solid or compound.", "Eine Kante, ein Kantenzug, eine Kurve oder eine Fläche ohne Berandung hat keinen Schwerpunkt; verwende einen Punkt, eine Fläche, eine Schale, einen Körper oder eine Gruppe."), kind.quality),
    }
}

struct Massive {
    pieces: Pieces,
    density: f64,
}

/// 🛞️ Mass, volume, centroid, inertia tensor about the centroid and principal frame of a solid or compound of uniform density, one solid per unit.
fn mass_properties_compute(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    let prepared = (|| {
        let density = inputs.number("density")?;
        if !(density.is_finite() && density > 0.0) {
            return Err(out_of_range("density", "The density must be greater than zero.", "Die Dichte muss größer als null sein."));
        }
        let (pieces, total) = pieces_state(inputs.shape("solid")?, "solid")?;
        Ok((Massive { pieces, density }, total))
    })();
    staged(
        kind,
        prepared,
        |state, index| add_piece(&mut state.pieces, index),
        |state, quality| {
            let whole = combined(&state.pieces, "solid")?;
            let mut tensor = whole.about_centroid;
            tensor.iter_mut().flatten().for_each(|entry| *entry *= state.density);
            let principal = principal_inertia(tensor);
            let entries: Vec<GeometryValue> = tensor.iter().flatten().map(|entry| GeometryValue::Number(*entry)).collect();
            for entry in tensor.iter().flatten() {
                finite(*entry)?;
            }
            Ok((
                outputs([
                    ("mass", GeometryValue::Number(finite(whole.mass * state.density)?)),
                    ("volume", GeometryValue::Number(whole.mass)),
                    ("centroid", GeometryValue::Point(whole.centroid)),
                    ("inertia", GeometryValue::List(entries)),
                    ("principalMoments", GeometryValue::Vector(principal.values)),
                    ("principalAxes", GeometryValue::List(principal.axes.iter().map(|axis| GeometryValue::Vector(*axis)).collect())),
                ]),
                quality,
            ))
        },
    )
}
//#endregion 🔖️Mass

//#region 🔖️Bounds
/// 📦️ The bounding box of `min`..`max` as a solid, or `None` when the shape is flat along an axis and encloses no volume.
fn box_solid(min: [f64; 3], max: [f64; 3]) -> Result<Option<ShapeValue>, WidgetFault> {
    let size = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];
    if size.iter().any(|extent| !(extent.is_finite() && *extent > 0.0)) {
        return Ok(None);
    }
    let mut session = KernelSession::new();
    let solid = session.brep().box_prim_sync(size[0], size[1], size[2]).map_err(|error| kernel_fault(&error))?;
    let placed = if min == [0.0; 3] { solid } else { session.brep().translate_sync(&solid, min).map_err(|error| kernel_fault(&error))? };
    session.export(&placed).map(Some)
}

/// 📦️ The tight axis-aligned bounds of a shape: the corners, the extent and, when it has volume, the box as a solid. Bounds that rest on a NURBS entity are reported as approximate.
fn bounding_box_compute(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish_with_quality(
        kind,
        (|| {
            let shape = inputs.shape("shape")?;
            let bounds = bounding_box(&shape.body, &scope_of(shape, "shape")?).map_err(query_fault)?;
            let size = sub(bounds.max, bounds.min);
            let mut found = outputs([("min", GeometryValue::Point(bounds.min)), ("max", GeometryValue::Point(bounds.max)), ("size", GeometryValue::Vector(size))]);
            if let Some(solid) = box_solid(bounds.min, bounds.max)? {
                found.insert("box".to_string(), GeometryValue::shape(solid));
            }
            Ok((found, if bounds.exact { kind.quality } else { Quality::Approximate }))
        })(),
    )
}
//#endregion 🔖️Bounds

//#region 🔖️Pairs
struct Pair {
    first: Arc<ShapeValue>,
    second: Arc<ShapeValue>,
    session: KernelSession,
    handles: Vec<GeometryHandle>,
    distance: f64,
}

/// 🛤️ The shortest distance between two shapes: both are imported into one fresh session (one unit each), then the kernel measures (one unit).
fn distance(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    let prepared = (|| Ok((Pair { first: inputs.shape("a")?.clone(), second: inputs.shape("b")?.clone(), session: KernelSession::new(), handles: Vec::new(), distance: 0.0 }, 3)))();
    staged(
        kind,
        prepared,
        |state, index| {
            if index < 2 {
                let source = if index == 0 { state.first.clone() } else { state.second.clone() };
                let imported = state.session.import(&source)?;
                state.handles.push(imported.handle);
                return Ok(());
            }
            let (a, b) = (nth_handle(&state.handles, 0)?, nth_handle(&state.handles, 1)?);
            state.distance = state.session.brep().shape_distance_sync(&a, &b).map_err(|error| kernel_fault(&error))?.distance;
            Ok(())
        },
        |state, quality| Ok((outputs([("distance", GeometryValue::Number(finite(state.distance)?))]), quality)),
    )
}

/// 🔎️ The `index`-th imported handle.
fn nth_handle(handles: &[GeometryHandle], index: usize) -> Result<GeometryHandle, WidgetFault> {
    handles.get(index).cloned().ok_or_else(lost_state)
}

/// 📌️ The point of a shape closest to a given point.
fn closest_point(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let shape = inputs.shape("shape")?;
            let [x, y, z] = inputs.point("point")?;
            let nearest = point_distance(&shape.body, &scope_of(shape, "shape")?, Pnt3::new(x, y, z)).map_err(query_fault)?;
            Ok(outputs([("point", GeometryValue::Point(nearest.closest))]))
        })(),
    )
}

struct Classification {
    shape: Arc<ShapeValue>,
    solids: Vec<SolidId>,
    point: Pnt3,
    verdict: PointClassification,
}

/// 🧭️ Where a point lies relative to a solid or compound, one solid per unit: inside any solid wins over the boundary, which wins over outside.
fn classify_point(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    let prepared = (|| {
        let shape = inputs.shape("solid")?.clone();
        let [x, y, z] = inputs.point("point")?;
        let solids = solids_of(&shape, "solid")?;
        let total = solids.len();
        Ok((Classification { shape, solids, point: Pnt3::new(x, y, z), verdict: PointClassification::Outside }, total))
    })();
    staged(
        kind,
        prepared,
        |state, index| {
            let found = point_in_solid(&state.shape.body, nth(&state.solids, index)?, state.point, CLASSIFY_TOLERANCE).map_err(query_fault)?;
            state.verdict = match (state.verdict, found) {
                (PointClassification::Inside, _) | (_, PointClassification::Inside) => PointClassification::Inside,
                (PointClassification::OnBoundary, _) | (_, PointClassification::OnBoundary) => PointClassification::OnBoundary,
                _ => PointClassification::Outside,
            };
            Ok(())
        },
        |state, quality| {
            let name = match state.verdict {
                PointClassification::Inside => "inside",
                PointClassification::Outside => "outside",
                PointClassification::OnBoundary => "boundary",
            };
            Ok((outputs([("classification", GeometryValue::Text(name.to_string()))]), quality))
        },
    )
}
//#endregion 🔖️Pairs

//#region 🔖️Angle
/// 🧭️ What an angle is measured between: the outward normal of a planar face or the line of a straight edge.
enum Direction {
    Normal([f64; 3]),
    Line([f64; 3]),
}

fn not_planar(port: &str) -> WidgetFault {
    WidgetFault::new("generation3d.geometry.not-planar", "The face is not planar and has no single normal.", "Die Fläche ist nicht eben und hat keine einheitliche Normale.").at(port)
}

fn not_straight(port: &str) -> WidgetFault {
    WidgetFault::new("generation3d.geometry.not-straight", "The edge is not straight and has no single direction.", "Die Kante ist nicht gerade und hat keine einheitliche Richtung.").at(port)
}

/// 🧭️ The direction a face or edge shape stands for, refusing curved faces, curved edges and every other shape.
fn direction_of(shape: &ShapeValue, port: &str) -> Result<Direction, WidgetFault> {
    let scope = scope_of(shape, port)?;
    match &shape.root {
        ShapeRoot::Face(_) => {
            let row = face_table(&shape.body, &scope, TOLERANCE).map_err(query_fault)?.into_iter().next().ok_or_else(lost_state)?;
            match (row.surface_kind, row.normal) {
                (SurfaceKind::Plane, Some(normal)) => Ok(Direction::Normal(normal)),
                _ => Err(not_planar(port)),
            }
        }
        ShapeRoot::Edge(_) => {
            let row = edge_table(&shape.body, &scope).map_err(query_fault)?.into_iter().next().ok_or_else(lost_state)?;
            match row.curve_kind {
                CurveKind::Line => Ok(Direction::Line(sub(row.end, row.start))),
                _ => Err(not_straight(port)),
            }
        }
        _ => Err(unsupported(port, "Only a planar face or a straight edge has an angle.", "Nur eine ebene Fläche oder eine gerade Kante hat einen Winkel.")),
    }
}

/// 📐️ The angle between two planar faces (their outward normals, 0 to 180 degrees) or two straight edges (their lines, 0 to 90 degrees).
fn angle(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let (first, second) = (direction_of(inputs.shape("a")?, "a")?, direction_of(inputs.shape("b")?, "b")?);
            let zero = |port: &str| super::math_vector::zero_vector(port);
            let result = match (first, second) {
                (Direction::Normal(a), Direction::Normal(b)) => angle_between(a, b).ok_or_else(|| zero("a"))?,
                (Direction::Line(a), Direction::Line(b)) => {
                    let between = angle_between(a, b).ok_or_else(|| if super::math_vector::unit(a).is_none() { zero("a") } else { zero("b") })?;
                    between.min(std::f64::consts::PI - between)
                }
                _ => return Err(WidgetFault::new("generation3d.geometry.angle-mismatch", "Both shapes must be planar faces or both must be straight edges.", "Beide Formen müssen ebene Flächen oder beide müssen gerade Kanten sein.").at("b")),
            };
            Ok(outputs([("angle", GeometryValue::Number(result))]))
        })(),
    )
}
//#endregion 🔖️Angle

/// 🗃️ Every analysis.measure catalogue kind and the compute that starts it.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "analysis.volume", start: volume },
    ComputeEntry { id: "analysis.area", start: area },
    ComputeEntry { id: "analysis.length", start: length },
    ComputeEntry { id: "analysis.centroid", start: centroid },
    ComputeEntry { id: "analysis.boundingBox", start: bounding_box_compute },
    ComputeEntry { id: "analysis.massProperties", start: mass_properties_compute },
    ComputeEntry { id: "analysis.distance", start: distance },
    ComputeEntry { id: "analysis.closestPoint", start: closest_point },
    ComputeEntry { id: "analysis.classifyPoint", start: classify_point },
    ComputeEntry { id: "analysis.angle", start: angle },
    ComputeEntry { id: "analysis.selectionArea", start: selection_area },
];

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
