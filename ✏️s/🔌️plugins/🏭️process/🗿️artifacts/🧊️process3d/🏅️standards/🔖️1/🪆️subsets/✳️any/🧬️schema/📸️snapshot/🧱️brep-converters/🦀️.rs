//! 🧱️ B-Rep converters of the process3d snapshot: the analytic B-Rep per working solid, the shipped reference solids and the box, cylinder and sphere topology builders. See [`super`].

use super::document_records::WorkingSolid;
use semio_s_artifact_stdio_semio_brep::schema::snapshot::{BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioBrepSnapshot, STDIO_SEMIOBREP_DOCUMENT_SCHEMA};

//#region 🔖️Document
//#region 🔖️WorkingScene
//#region 🔖️BrepConverters
/// 🌉️ WRITE direction, real (not a stub): analytic B-Rep topology for each `WorkingSolid` variant.
/// `Box`/`Cylinder`/`Sphere` are exact (no tessellation/approximation — `BrepSurface::Cylinder`/
/// `Sphere` are themselves analytic surface primitives, and a `BrepLoop` with zero edges is the
/// correct "untrimmed, spans the whole surface" shape for the sphere's single closed face and the
/// cylinder's lateral face). `ImportedMesh`/`ImportedSolid` reference content the app's own kernel
/// session must resolve (mesh URL / kernel-session-local handle) — this plugin-only migration cannot
/// tessellate/import that here, so it mints an honest EMPTY-topology placeholder rather than
/// fabricating fake geometry; the doc comment on `SolidSpec`'s old `ImportedMesh`/`ImportedSolid`
/// variants already flagged this as kernel-session-resolved, never persisted-content-resolved.
pub fn brep_snapshot_for_working_solid(solid: &WorkingSolid) -> SemioBrepSnapshot {
    match solid {
        WorkingSolid::Box { width, depth, height } => brep_snapshot_for_box(*width, *depth, *height),
        WorkingSolid::Cylinder { radius, height } => brep_snapshot_for_cylinder(*radius, *height),
        WorkingSolid::Sphere { radius } => brep_snapshot_for_sphere(*radius),
        WorkingSolid::Reference { reference_id } => reference_solid(reference_id).and_then(|reference| reference.brep_snapshot()).unwrap_or_else(empty_brep_snapshot),
        WorkingSolid::ImportedMesh { .. } | WorkingSolid::ImportedSolid { .. } => empty_brep_snapshot(),
    }
}
//#endregion 🔖️BrepConverters
//#endregion 🔖️WorkingScene
//#endregion 🔖️Document

//#region 🔖️Document
//#region 🔖️WorkingScene
//#region 🔖️BrepConverters
//#region 🔖️ReferenceSolids
/// 🏛️ One shipped reference solid: its exact STEP text (`🖼️assets/<slug>/📐️.stp`) plus the authored
/// axis-aligned extent the capability rules read without a kernel (`stock_extent`) — pinned against
/// the kernel's own bounding box by `reference_solid_extents_match_the_kernel`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceSolid {
    pub id: &'static str,
    pub label: &'static str,
    pub step: &'static str,
    pub extent: [f64; 3],
}

impl ReferenceSolid {
    /// 🧊️ The reference rebuilt in a fresh kernel session — one real solid handle, or `None` when
    /// the shipped STEP does not read (a build-time asset defect, never a user-facing state).
    pub fn import(&self, kernel: &mut semio_framework_3d_brep_engine::Brep) -> Option<semio_framework_3d_brep_engine::GeometryHandle> {
        semio_s_artifact_stdio_step::geometry::import_step(kernel,self.step).ok()?.into_iter().next()
    }

    /// 🌉️ The reference's real, lossless B-Rep content — the composed `stock_solid` child this
    /// stock mints.
    pub fn brep_snapshot(&self) -> Option<SemioBrepSnapshot> {
        let mut kernel = semio_framework_3d_brep_engine::Brep::new();
        self.import(&mut kernel)?;
        Some(semio_s_artifact_stdio_semio_brep::schema::snapshot::body::snapshot_from_body(kernel.tessellation_body()))
    }
}

/// 🌲️ The left hexagonal-cut concrete forest piece (`♻️mit-bestand/🖼️asset/🏚️abbau-aufbau`): a
/// 8.1 m × 4.68 m parallelogram slab on a triangular beam grid over two hexagonal columns, 3 m tall.
pub const REFERENCE_SOLID_CONCRETE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";

/// 🏛️ Every reference solid this artifact ships, by stable id.
pub const REFERENCE_SOLIDS: &[ReferenceSolid] = &[ReferenceSolid {
    id: REFERENCE_SOLID_CONCRETE_FOREST_LEFT,
    label: "Hexagonal Cut Concrete Forest Left",
    step: semio_s_artifact_process_process3d_reference_solids::CONCRETE_FOREST_LEFT_STEP,
    extent: [10.800_010_282_482_4, 4.676_537_180_435_97, 3.0],
}];

pub fn reference_solid(reference_id: &str) -> Option<&'static ReferenceSolid> {
    REFERENCE_SOLIDS.iter().find(|reference| reference.id == reference_id)
}
//#endregion 🔖️ReferenceSolids
//#endregion 🔖️BrepConverters
//#endregion 🔖️WorkingScene
//#endregion 🔖️Document

//#region 🔖️Document
//#region 🔖️WorkingScene
//#region 🔖️BrepConverters
fn empty_brep_snapshot() -> SemioBrepSnapshot {
    SemioBrepSnapshot { schema: STDIO_SEMIOBREP_DOCUMENT_SCHEMA.into(), vertices: Vec::new(), edges: Vec::new(), loops: Vec::new(), faces: Vec::new(), shells: Vec::new(), solids: Vec::new(), coedges: Vec::new(), next_label: 0 }
}

fn point3(p: [f64; 3]) -> semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3 {
    semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3 { x: p[0], y: p[1], z: p[2] }
}

/// 📦️ Corner-at-local-origin box, spanning `[0,w]×[0,d]×[0,h]` — 8 vertices, 12 straight edges, 6
/// planar faces, 1 shell, 1 solid.
fn brep_snapshot_for_box(width: f64, depth: f64, height: f64) -> SemioBrepSnapshot {
    let (w, d, h) = (width, depth, height);
    let corners = [[0.0, 0.0, 0.0], [w, 0.0, 0.0], [w, d, 0.0], [0.0, d, 0.0], [0.0, 0.0, h], [w, 0.0, h], [w, d, h], [0.0, d, h]];
    let vertices: Vec<BrepVertex> = corners.iter().enumerate().map(|(i, c)| BrepVertex { id: format!("v{i}"), point: point3(*c), tol: 0.0 }).collect();
    let line_edge = |id: &str, a: usize, b: usize| -> BrepEdge {
        let origin = corners[a];
        let direction = [corners[b][0] - corners[a][0], corners[b][1] - corners[a][1], corners[b][2] - corners[a][2]];
        BrepEdge { id: id.into(), start_vertex: format!("v{a}"), end_vertex: format!("v{b}"), curve: BrepCurve::Line { origin: point3(origin), direction: point3(direction) }, tol: 0.0 }
    };
    let edges = vec![
        line_edge("e0", 0, 1),
        line_edge("e1", 1, 2),
        line_edge("e2", 2, 3),
        line_edge("e3", 3, 0),
        line_edge("e4", 4, 5),
        line_edge("e5", 5, 6),
        line_edge("e6", 6, 7),
        line_edge("e7", 7, 4),
        line_edge("e8", 0, 4),
        line_edge("e9", 1, 5),
        line_edge("e10", 2, 6),
        line_edge("e11", 3, 7),
    ];
    let loop_of = |id: &str, edges: &[(&str, bool)]| -> BrepLoop { BrepLoop { id: id.into(), edges: edges.iter().map(|(e, o)| BrepLoopEdge { edge: (*e).into(), orientation: *o }).collect() } };
    let loops = vec![
        loop_of("l0", &[("e0", true), ("e1", true), ("e2", true), ("e3", true)]),
        loop_of("l1", &[("e4", true), ("e5", true), ("e6", true), ("e7", true)]),
        loop_of("l2", &[("e0", true), ("e9", true), ("e4", false), ("e8", false)]),
        loop_of("l3", &[("e1", true), ("e10", true), ("e5", false), ("e9", false)]),
        loop_of("l4", &[("e2", true), ("e11", true), ("e6", false), ("e10", false)]),
        loop_of("l5", &[("e3", true), ("e8", true), ("e7", false), ("e11", false)]),
    ];
    let plane_face = |id: &str, loop_id: &str, origin: [f64; 3], normal: [f64; 3], orientation: bool| -> BrepFace {
        BrepFace { id: id.into(), outer_loop: loop_id.into(), inner_loops: Vec::new(), surface: BrepSurface::Plane { origin: point3(origin), normal: point3(normal) }, orientation, tol: 0.0 }
    };
    let faces = vec![
        plane_face("f0", "l0", [0.0, 0.0, 0.0], [0.0, 0.0, -1.0], true),
        plane_face("f1", "l1", [0.0, 0.0, h], [0.0, 0.0, 1.0], true),
        plane_face("f2", "l2", [0.0, 0.0, 0.0], [0.0, -1.0, 0.0], true),
        plane_face("f3", "l3", [w, 0.0, 0.0], [1.0, 0.0, 0.0], true),
        plane_face("f4", "l4", [0.0, d, 0.0], [0.0, 1.0, 0.0], true),
        plane_face("f5", "l5", [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], true),
    ];
    let shells = vec![BrepShell { id: "s0".into(), faces: (0..6).map(|i| BrepShellFace { face: format!("f{i}"), orientation: true }).collect() }];
    let solids = vec![BrepSolid { id: "so0".into(), shells: vec![BrepSolidShell { shell: "s0".into(), is_void: false }] }];
    SemioBrepSnapshot { schema: STDIO_SEMIOBREP_DOCUMENT_SCHEMA.into(), vertices, edges, loops, faces, shells, solids, coedges: Vec::new(), next_label: 0 }
}

/// 🥫️ Axis-aligned cylinder, base centered at local origin, axis along +Z, spanning `[0,height]`.
/// Two circular cap edges + two planar cap faces + one analytic `BrepSurface::Cylinder` lateral
/// face (untrimmed — its loop has no bounding edges, since the lateral surface is naturally closed
/// around the axis and only bounded top/bottom by the shared circular edges via the loop's implicit
/// parametric domain, matching the same "loop may be empty" convention the sphere face uses).
fn brep_snapshot_for_cylinder(radius: f64, height: f64) -> SemioBrepSnapshot {
    let (r, h) = (radius, height);
    let vertices = vec![BrepVertex { id: "v0".into(), point: point3([r, 0.0, 0.0]), tol: 0.0 }, BrepVertex { id: "v1".into(), point: point3([r, 0.0, h]), tol: 0.0 }];
    let edges = vec![
        BrepEdge { id: "e0".into(), start_vertex: "v0".into(), end_vertex: "v0".into(), curve: BrepCurve::Circle { center: point3([0.0, 0.0, 0.0]), axis: point3([0.0, 0.0, 1.0]), radius: r }, tol: 0.0 },
        BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Circle { center: point3([0.0, 0.0, h]), axis: point3([0.0, 0.0, 1.0]), radius: r }, tol: 0.0 },
    ];
    let loops = vec![
        BrepLoop { id: "l0".into(), edges: vec![BrepLoopEdge { edge: "e0".into(), orientation: true }] },
        BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] },
        BrepLoop { id: "l2".into(), edges: Vec::new() },
    ];
    let faces = vec![
        BrepFace { id: "f0".into(), outer_loop: "l0".into(), inner_loops: Vec::new(), surface: BrepSurface::Plane { origin: point3([0.0, 0.0, 0.0]), normal: point3([0.0, 0.0, -1.0]) }, orientation: true, tol: 0.0 },
        BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: Vec::new(), surface: BrepSurface::Plane { origin: point3([0.0, 0.0, h]), normal: point3([0.0, 0.0, 1.0]) }, orientation: true, tol: 0.0 },
        BrepFace { id: "f2".into(), outer_loop: "l2".into(), inner_loops: Vec::new(), surface: BrepSurface::Cylinder { origin: point3([0.0, 0.0, 0.0]), axis: point3([0.0, 0.0, 1.0]), radius: r }, orientation: true, tol: 0.0 },
    ];
    let shells = vec![BrepShell { id: "s0".into(), faces: vec![BrepShellFace { face: "f0".into(), orientation: true }, BrepShellFace { face: "f1".into(), orientation: true }, BrepShellFace { face: "f2".into(), orientation: true }] }];
    let solids = vec![BrepSolid { id: "so0".into(), shells: vec![BrepSolidShell { shell: "s0".into(), is_void: false }] }];
    SemioBrepSnapshot { schema: STDIO_SEMIOBREP_DOCUMENT_SCHEMA.into(), vertices, edges, loops, faces, shells, solids, coedges: Vec::new(), next_label: 0 }
}

/// 🔮️ Sphere centered at local origin — one closed, untrimmed analytic `BrepSurface::Sphere` face
/// (no boundary curves at all: zero vertices/edges, one loop with zero edges, matching the shared
/// "a `BrepLoop` with no edges means the whole surface" convention).
fn brep_snapshot_for_sphere(radius: f64) -> SemioBrepSnapshot {
    let loops = vec![BrepLoop { id: "l0".into(), edges: Vec::new() }];
    let faces = vec![BrepFace { id: "f0".into(), outer_loop: "l0".into(), inner_loops: Vec::new(), surface: BrepSurface::Sphere { center: point3([0.0, 0.0, 0.0]), radius }, orientation: true, tol: 0.0 }];
    let shells = vec![BrepShell { id: "s0".into(), faces: vec![BrepShellFace { face: "f0".into(), orientation: true }] }];
    let solids = vec![BrepSolid { id: "so0".into(), shells: vec![BrepSolidShell { shell: "s0".into(), is_void: false }] }];
    SemioBrepSnapshot { schema: STDIO_SEMIOBREP_DOCUMENT_SCHEMA.into(), vertices: Vec::new(), edges: Vec::new(), loops, faces, shells, solids, coedges: Vec::new(), next_label: 0 }
}

/// 🌉️ READ direction, documented gap: recovering a `WorkingSolid` (a small parametric vocabulary)
/// from an arbitrary resolved `SemioBrepSnapshot` (general topology) is not generally invertible —
/// nothing tags a brep as "this originated from a box/cylinder/sphere". Mirrors `📐️cad`'s own
/// documented read-side gap for its per-pane object lists. Callers that only need SOME extent
/// (rendering/bounds) should read `stock_bounding_box`, which works directly off the brep's own
/// vertex list instead of trying to recover a `WorkingSolid`.
pub fn working_solid_from_brep_snapshot(_brep: &SemioBrepSnapshot) -> WorkingSolid {
    WorkingSolid::default()
}

/// 🪪️ Mint a deterministic, content-addressed `s.stdio.semio.brep` CHILD HANDLE from `content`
/// (mirrors `📐️cad`'s `cad_model_child_handle`/`💠️lowpoly`'s `mesh_child_handle` — same
/// `store::os_store::artifact_child::ArtifactChild::new`/`ArtifactDialect` shape). Two callers with byte-identical content
/// mint the same handle.
pub fn brep_child_handle(slug: &str, content: &SemioBrepSnapshot) -> store::os_store::artifact_child::ArtifactChild<SemioBrepSnapshot> {
    brep_child_handle_for_text(slug, &serde_json::to_string(&semio_framework_value::ToValue::to_value(content)).unwrap_or_default())
}

/// 🪪️ The `s.stdio.semio.brep` child handle of a `WorkingSolid`: analytic solids are addressed by
/// their minted snapshot, a `Reference` by the shipped STEP text it is — the same content-addressing,
/// without materialising a 57-face snapshot's JSON inside the guest on every step mutation.
pub fn working_solid_child_handle(slug: &str, solid: &WorkingSolid) -> store::os_store::artifact_child::ArtifactChild<SemioBrepSnapshot> {
    match solid {
        WorkingSolid::Reference { reference_id } => brep_child_handle_for_text(slug, reference_solid(reference_id).map_or(reference_id.as_str(), |reference| reference.step)),
        _ => brep_child_handle(slug, &brep_snapshot_for_working_solid(solid)),
    }
}

fn brep_child_handle_for_text(slug: &str, content: &str) -> store::os_store::artifact_child::ArtifactChild<SemioBrepSnapshot> {
    let child_id = store::content_id(&format!("{slug}-brep"), content.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "brep".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️BrepConverters
//#endregion 🔖️WorkingScene
//#endregion 🔖️Document
