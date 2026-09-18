//! 🧲️ CAD spatial pick/selection engine — the Rust twin of the `🧲️GeometryTargets` and
//! `🧲️GeometryInteraction` regions of `⚙️engine/📺️renderer/🟦️.tsx` (lines 647-1380 and 2277-2888).
//!
//! 🧭️ Target-neutral on purpose: these are the sub-object pick targets, visibility filters,
//! hover-key aliases and modifier-mode selection merges a spatial editor needs, computed from the
//! pane's own `(&[CadObject], Option<&CadGeometry>)` pair — the exact pair
//! `modes::edit::cad_pane_working_objects` already returns. React's `World3dHost` and the wgpu
//! `render_world3d_surface_step` both paint whatever the play app puts on the `World3d` wire, so
//! computing this here closes the gap on BOTH targets at once instead of twice.
//!
//! 🚫️ Deliberately NOT here: screen-space marquee/ray math
//! (`spatialPickTargetsFromRay`/`ScreenSelection`, `marqueeCoverageFromGesture`, `pointInPolygon`,
//! `projectPointToClient`). That is host geometry, already Rust in the framework
//! (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs`'s `marquee_is_crossing{,_from_path}`,
//! `point_in_polygon`) and consumed by the renderer, not by a guest that never sees client pixels.
//! Merge modes reuse `protocol::MergeMode` — the one wire vocabulary — instead of a bespoke
//! `SpatialSelectionMode`.

use crate::editor::cad::engine::typology::{default_model_definition_id, is_shape_model_definition, load_typology, model_definition_selection_entity_kinds, model_definition_typology_ids, model_definition_uses_geometry_picking, resolve_typology_style, spatial_typology_toggle_label, ModelEntityKind, ResolvedTypologyStyle, PRIMITIVE_MODEL_ENTITY_KINDS};
use crate::standards::v1::subsets::any::io::geometry_import::{CadEdge, CadFace, CadGeometry, CadObject, CadShell, CadSolid, CadVertex, CadWire};
use protocol::{DslValue, MergeMode};
use std::collections::{BTreeMap, BTreeSet, HashMap};

//#region 🔖️Types
/// 🐁️ The two pointer gestures a pick event can carry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialPickKind {
    PointerDown,
    PointerMove,
}

impl SpatialPickKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PointerDown => "pointer.down",
            Self::PointerMove => "pointer.move",
        }
    }
}

/// 🎯️ Renderer-facing pick kinds; kernel-private granularity rides on
/// [`SpatialPickTarget::geometry_kind`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpatialPickTargetKind {
    Object,
    Face,
    Edge,
    Vertex,
}

pub const SPATIAL_PICK_TARGET_KINDS: &[SpatialPickTargetKind] = &[SpatialPickTargetKind::Object, SpatialPickTargetKind::Face, SpatialPickTargetKind::Edge, SpatialPickTargetKind::Vertex];

impl SpatialPickTargetKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Object => "object",
            Self::Face => "face",
            Self::Edge => "edge",
            Self::Vertex => "vertex",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "object" => Self::Object,
            "face" => Self::Face,
            "edge" => Self::Edge,
            "vertex" => Self::Vertex,
            _ => return None,
        })
    }

    /// 🥽️ Pick generality, coarsest first — the `CanvasPickMenu` ordering React sorts by.
    pub fn generality(self) -> u8 {
        match self {
            Self::Object => 0,
            Self::Face => 1,
            Self::Edge => 2,
            Self::Vertex => 3,
        }
    }
}

/// 🧲️ One snap/select target the editor offers for a kernel entity or a typology object row.
#[derive(Clone, Debug, PartialEq)]
pub struct SpatialPickTarget {
    pub kind: SpatialPickTargetKind,
    pub id: String,
    pub point: [f64; 3],
    pub points: Vec<[f64; 3]>,
    pub geometry_kind: Option<ModelEntityKind>,
    pub typology_id: Option<String>,
}

/// 🖱️ A committed selection row on the document (`{kind, id, editable}` in React).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectionTarget {
    pub kind: ModelEntityKind,
    pub id: String,
    pub editable: bool,
}

/// 👁️ Persisted hide/lock flags for one entity id.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SpatialEntityFlags {
    pub hidden: bool,
    pub locked: bool,
}

impl SpatialEntityFlags {
    /// 👁️ `worldEntitySelectable` — a hidden or locked entity never takes a canvas pick.
    pub fn selectable(self) -> bool {
        !self.hidden && !self.locked
    }
}

/// 👁️ Per-kind on/off map for visibility filters or selection/hover gates (`Some(false)` disables;
/// `None` means "not set", which React spells as `!== false`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SpatialPickKindToggles {
    pub object: Option<bool>,
    pub face: Option<bool>,
    pub edge: Option<bool>,
    pub vertex: Option<bool>,
}

impl SpatialPickKindToggles {
    pub fn get(&self, kind: SpatialPickTargetKind) -> Option<bool> {
        match kind {
            SpatialPickTargetKind::Object => self.object,
            SpatialPickTargetKind::Face => self.face,
            SpatialPickTargetKind::Edge => self.edge,
            SpatialPickTargetKind::Vertex => self.vertex,
        }
    }

    pub fn set(&mut self, kind: SpatialPickTargetKind, value: Option<bool>) {
        match kind {
            SpatialPickTargetKind::Object => self.object = value,
            SpatialPickTargetKind::Face => self.face = value,
            SpatialPickTargetKind::Edge => self.edge = value,
            SpatialPickTargetKind::Vertex => self.vertex = value,
        }
    }

    /// 👁️ React's `toggles[kind] !== false`.
    pub fn enabled(&self, kind: SpatialPickTargetKind) -> bool {
        self.get(kind) != Some(false)
    }
}

/// 👁️ Per-typology on/off map for play chrome.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpatialTypologyToggles(pub BTreeMap<String, bool>);

impl SpatialTypologyToggles {
    pub fn enabled(&self, typology_id: &str) -> bool {
        self.0.get(typology_id) != Some(&false)
    }

    pub fn with(mut self, typology_id: impl Into<String>, value: bool) -> Self {
        self.0.insert(typology_id.into(), value);
        self
    }
}

/// 👁️ Per-primitive on/off map for play chrome (kernel entity kinds, not pick kinds).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpatialPrimitiveToggles(pub BTreeMap<ModelEntityKind, bool>);

impl SpatialPrimitiveToggles {
    pub fn enabled(&self, kind: ModelEntityKind) -> bool {
        self.0.get(&kind) != Some(&false)
    }

    pub fn with(mut self, kind: ModelEntityKind, value: bool) -> Self {
        self.0.insert(kind, value);
        self
    }
}

/// ☑️ Aggregate enabled state for a fixed-key boolean toggle group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialToggleGroupState {
    All,
    None,
    Partial,
}

/// ☑️ The owned tri-state checkbox value a toggle group maps onto.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialToggleCheckboxState {
    Checked,
    Unchecked,
    Indeterminate,
}

/// 👁️ Which scene layers stay visible for geometry edit vs typology object picking.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpatialSceneVisibility {
    pub show_factory_wireframe: bool,
    pub show_committed_faces: bool,
    pub show_committed_edges: bool,
}
//#endregion 🔖️Types

//#region 🔖️KindMapping
/// 🧭️ Kernel entity kind → the coarser pick kind that represents it on the canvas.
pub fn geometry_kind_to_object_pick(kind: ModelEntityKind) -> Option<SpatialPickTargetKind> {
    Some(match kind {
        ModelEntityKind::Vertex | ModelEntityKind::Anchor => SpatialPickTargetKind::Vertex,
        ModelEntityKind::Edge | ModelEntityKind::Wire => SpatialPickTargetKind::Edge,
        ModelEntityKind::Face | ModelEntityKind::Shell => SpatialPickTargetKind::Face,
        ModelEntityKind::Solid => SpatialPickTargetKind::Object,
        _ => return None,
    })
}

/// 🧭️ Pick kind → the kernel entity kind a snap/selection payload must name.
pub fn kernel_geometry_kind_for_object_pick(kind: SpatialPickTargetKind, geometry_kind: Option<ModelEntityKind>) -> ModelEntityKind {
    if let Some(kind) = geometry_kind {
        return kind;
    }
    match kind {
        SpatialPickTargetKind::Vertex => ModelEntityKind::Vertex,
        SpatialPickTargetKind::Edge => ModelEntityKind::Edge,
        SpatialPickTargetKind::Face => ModelEntityKind::Face,
        SpatialPickTargetKind::Object => ModelEntityKind::Solid,
    }
}

/// 🎯️ Pick kinds an interaction's `selection.accept` list admits, or `None` for "accepts anything".
pub fn spatial_pick_kinds_for_selection_accept(accept: &[ModelEntityKind]) -> Option<BTreeSet<SpatialPickTargetKind>> {
    if accept.is_empty() {
        return None;
    }
    let mut out = BTreeSet::new();
    for kind in accept {
        if let Some(direct) = match kind {
            ModelEntityKind::Object => Some(SpatialPickTargetKind::Object),
            ModelEntityKind::Face => Some(SpatialPickTargetKind::Face),
            ModelEntityKind::Edge => Some(SpatialPickTargetKind::Edge),
            ModelEntityKind::Vertex => Some(SpatialPickTargetKind::Vertex),
            _ => None,
        } {
            out.insert(direct);
            continue;
        }
        if let Some(mapped) = geometry_kind_to_object_pick(*kind) {
            out.insert(mapped);
        }
    }
    Some(out)
}

/// 🧭️ The primitive entity kind of a pick target (typology object rows have none).
pub fn pick_target_primitive_kind(target: &SpatialPickTarget) -> Option<ModelEntityKind> {
    if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() {
        return None;
    }
    Some(kernel_geometry_kind_for_object_pick(target.kind, target.geometry_kind))
}
//#endregion 🔖️KindMapping

//#region 🔖️Keys
/// 🪪️ Stable `kind:id` key for a pick target.
pub fn spatial_pick_target_key(target: &SpatialPickTarget) -> String {
    format!("{}:{}", target.kind.as_str(), target.id)
}

/// 🪪️ Stable hover/selection key for a committed selection row.
pub fn selection_target_hover_key(target: &SelectionTarget) -> String {
    format!("{}:{}", target.kind.as_str(), target.id)
}

/// 🪪️ Stable kernel-geometry member key for object-reveal lookup.
pub fn spatial_pick_target_member_key(target: &SpatialPickTarget) -> String {
    if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() {
        return format!("object:{}", target.id);
    }
    format!("{}:{}", kernel_geometry_kind_for_object_pick(target.kind, target.geometry_kind).as_str(), target.id)
}

fn pinned_pick_target_keys<'a>(keys: impl IntoIterator<Item = &'a str>) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for key in keys {
        out.insert(key.to_string());
        let Some(colon) = key.find(':') else { continue };
        let (kind, id) = (&key[..colon], &key[colon + 1..]);
        if let Some(mapped) = ModelEntityKind::parse(kind).and_then(geometry_kind_to_object_pick) {
            out.insert(format!("{}:{id}", mapped.as_str()));
        }
        if kind == "object" {
            out.insert(format!("solid:{id}"));
        }
    }
    out
}

/// 🪪️ Expands a hover/selection key across geometry pick aliases (`solid:foo` ↔ `object:foo`).
pub fn spatial_hover_key_aliases(key: Option<&str>) -> BTreeSet<String> {
    match key.filter(|key| !key.is_empty()) {
        None => BTreeSet::new(),
        Some(key) => pinned_pick_target_keys([key]),
    }
}

/// 🪪️ True when two hover/selection keys name the same thing, aliases included.
pub fn spatial_hover_keys_match(left: Option<&str>, right: Option<&str>) -> bool {
    let (Some(left), Some(right)) = (left.filter(|k| !k.is_empty()), right.filter(|k| !k.is_empty())) else {
        return false;
    };
    left == right || spatial_hover_key_aliases(Some(left)).contains(right)
}
//#endregion 🔖️Keys

//#region 🔖️Buckets
/// ⚓️ One anchor row, read out of `CadGeometry::anchors` — that field is untyped `DslValue` on the
/// import struct (`🚪️io/🗺️geometry-import/🦀️.rs`), so the shape is read here rather than by
/// widening a `pub(crate)` struct other lanes are editing.
#[derive(Clone, Debug, PartialEq)]
pub struct AnchorRow {
    pub id: String,
    pub position: [f64; 3],
    pub attachment: Option<(String, String)>,
}

fn read_vec3(value: Option<&DslValue>) -> Option<[f64; 3]> {
    let items = value?.as_array()?;
    if items.len() < 3 {
        return None;
    }
    Some([items[0].as_f64()?, items[1].as_f64()?, items[2].as_f64()?])
}

fn read_anchor_rows(anchors: &[DslValue]) -> Vec<AnchorRow> {
    anchors
        .iter()
        .filter_map(|row| {
            let id = row.get("id")?.as_str()?.to_string();
            let position = read_vec3(row.get("position"))?;
            let attachment = row.get("attachment").and_then(|attachment| Some((attachment.get("kind")?.as_str()?.to_string(), attachment.get("id")?.as_str()?.to_string())));
            Some(AnchorRow { id, position, attachment })
        })
        .collect()
}

/// 🧲️ Record-shaped views over a pane's ephemeral `CadGeometry` — React's `geometryBuckets`.
pub struct GeometryBuckets<'a> {
    pub anchors: Vec<AnchorRow>,
    pub vertices: HashMap<&'a str, &'a CadVertex>,
    pub edges: HashMap<&'a str, &'a CadEdge>,
    pub wires: HashMap<&'a str, &'a CadWire>,
    pub faces: HashMap<&'a str, &'a CadFace>,
    pub shells: HashMap<&'a str, &'a CadShell>,
    pub solids: HashMap<&'a str, &'a CadSolid>,
    order: GeometryOrder<'a>,
}

struct GeometryOrder<'a> {
    vertices: Vec<&'a CadVertex>,
    edges: Vec<&'a CadEdge>,
    wires: Vec<&'a CadWire>,
    faces: Vec<&'a CadFace>,
    shells: Vec<&'a CadShell>,
    solids: Vec<&'a CadSolid>,
}

/// 🧲️ Builds the buckets, preserving authored asset order so pick-target lists stay deterministic.
pub fn geometry_buckets(geometry: &CadGeometry) -> GeometryBuckets<'_> {
    GeometryBuckets {
        anchors: read_anchor_rows(&geometry.anchors),
        vertices: geometry.vertices.iter().map(|row| (row.id.as_str(), row)).collect(),
        edges: geometry.edges.iter().map(|row| (row.id.as_str(), row)).collect(),
        wires: geometry.wires.iter().map(|row| (row.id.as_str(), row)).collect(),
        faces: geometry.faces.iter().map(|row| (row.id.as_str(), row)).collect(),
        shells: geometry.shells.iter().map(|row| (row.id.as_str(), row)).collect(),
        solids: geometry.solids.iter().map(|row| (row.id.as_str(), row)).collect(),
        order: GeometryOrder {
            vertices: geometry.vertices.iter().collect(),
            edges: geometry.edges.iter().collect(),
            wires: geometry.wires.iter().collect(),
            faces: geometry.faces.iter().collect(),
            shells: geometry.shells.iter().collect(),
            solids: geometry.solids.iter().collect(),
        },
    }
}

fn point_centroid(points: &[[f64; 3]]) -> Option<[f64; 3]> {
    if points.is_empty() {
        return None;
    }
    let sum = points.iter().fold([0.0; 3], |acc, p| [acc[0] + p[0], acc[1] + p[1], acc[2] + p[2]]);
    let n = points.len() as f64;
    Some([sum[0] / n, sum[1] / n, sum[2] / n])
}

/// 🔑️ Dedup key for a point. React keys on `p.join(",")`, so equal values collapse; the bit pattern
/// does the same in O(1) once `-0.0` is folded onto `0.0` (JS prints both as `"0"`), and keeps a
/// solid's point set linear instead of the quadratic string scan a `Vec::contains` would cost on a
/// pane with thousands of kernel members.
fn point_key(point: [f64; 3]) -> [u64; 3] {
    [(point[0] + 0.0).to_bits(), (point[1] + 0.0).to_bits(), (point[2] + 0.0).to_bits()]
}

fn unique_points(points: Vec<[f64; 3]>) -> Vec<[f64; 3]> {
    let mut seen: std::collections::HashSet<[u64; 3]> = std::collections::HashSet::new();
    points.into_iter().filter(|point| seen.insert(point_key(*point))).collect()
}

impl<'a> GeometryBuckets<'a> {
    /// 📍️ `edgeSamplePoints` for the only curve family the Rust import carries: straight edges, so
    /// the samples ARE the endpoints. Curved edges (`arc`/`circle`/`nurbs`) have no pole/centre data
    /// on `CadEdgeCurve`, so they degrade to their endpoints instead of a tessellated polyline —
    /// tracked as a gap in the packet report §5.
    pub fn edge_points(&self, edge: &CadEdge) -> Vec<[f64; 3]> {
        edge.vertex_ids.iter().filter_map(|id| self.vertices.get(id.as_str()).map(|vertex| vertex.position)).collect()
    }

    pub fn wire_points(&self, wire: &CadWire) -> Vec<[f64; 3]> {
        unique_points(wire.edge_ids.iter().filter_map(|id| self.edges.get(id.as_str())).flat_map(|edge| self.edge_points(edge)).collect())
    }

    pub fn face_points(&self, face: &CadFace) -> Vec<[f64; 3]> {
        let edge_ids: Vec<&str> = face.wire_ids.iter().filter_map(|id| self.wires.get(id.as_str())).flat_map(|wire| wire.edge_ids.iter().map(String::as_str)).collect();
        unique_points(edge_ids.iter().filter_map(|id| self.edges.get(*id)).flat_map(|edge| self.edge_points(edge)).collect())
    }

    pub fn shell_points(&self, shell: &CadShell) -> Vec<[f64; 3]> {
        unique_points(shell.face_ids.iter().filter_map(|id| self.faces.get(id.as_str())).flat_map(|face| self.face_points(face)).collect())
    }

    pub fn solid_points(&self, solid: &CadSolid) -> Vec<[f64; 3]> {
        unique_points(solid.shell_ids.iter().filter_map(|id| self.shells.get(id.as_str())).flat_map(|shell| self.shell_points(shell)).collect())
    }

    pub fn all_vertex_points(&self) -> Vec<[f64; 3]> {
        self.order.vertices.iter().map(|vertex| vertex.position).collect()
    }

    /// 📐️ Points of one kernel entity by kind + id — React's `geometryEntityPoints`.
    pub fn entity_points(&self, kind: ModelEntityKind, id: &str) -> Vec<[f64; 3]> {
        match kind {
            ModelEntityKind::Anchor => self.anchors.iter().find(|anchor| anchor.id == id).map(|anchor| vec![anchor.position]).unwrap_or_default(),
            ModelEntityKind::Vertex => self.vertices.get(id).map(|vertex| vec![vertex.position]).unwrap_or_default(),
            ModelEntityKind::Edge => self.edges.get(id).map(|edge| self.edge_points(edge)).unwrap_or_default(),
            ModelEntityKind::Wire => self.wires.get(id).map(|wire| self.wire_points(wire)).unwrap_or_default(),
            ModelEntityKind::Face => self.faces.get(id).map(|face| self.face_points(face)).unwrap_or_default(),
            ModelEntityKind::Shell => self.shells.get(id).map(|shell| self.shell_points(shell)).unwrap_or_default(),
            ModelEntityKind::Solid => self.solids.get(id).map(|solid| self.solid_points(solid)).unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    /// 📐️ Consecutive segment pairs along every straight edge of one entity.
    pub fn entity_wire_segments(&self, kind: ModelEntityKind, id: &str) -> Vec<([f64; 3], [f64; 3])> {
        let polyline = |points: Vec<[f64; 3]>| -> Vec<([f64; 3], [f64; 3])> { points.windows(2).map(|pair| (pair[0], pair[1])).collect() };
        match kind {
            ModelEntityKind::Edge => self.edges.get(id).map(|edge| polyline(self.edge_points(edge))).unwrap_or_default(),
            ModelEntityKind::Wire => self.wires.get(id).map(|wire| wire.edge_ids.iter().filter_map(|id| self.edges.get(id.as_str())).flat_map(|edge| polyline(self.edge_points(edge))).collect()).unwrap_or_default(),
            ModelEntityKind::Face => self
                .faces
                .get(id)
                .map(|face| face.wire_ids.iter().filter_map(|id| self.wires.get(id.as_str())).flat_map(|wire| wire.edge_ids.iter().filter_map(|id| self.edges.get(id.as_str())).flat_map(|edge| polyline(self.edge_points(edge))).collect::<Vec<_>>()).collect())
                .unwrap_or_default(),
            ModelEntityKind::Shell => self.shells.get(id).map(|shell| shell.face_ids.iter().flat_map(|face_id| self.entity_wire_segments(ModelEntityKind::Face, face_id)).collect()).unwrap_or_default(),
            ModelEntityKind::Solid => self.solids.get(id).map(|solid| solid.shell_ids.iter().flat_map(|shell_id| self.entity_wire_segments(ModelEntityKind::Shell, shell_id)).collect()).unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    /// 📐️ Every straight-edge segment in the pane, for the factory wireframe layer.
    pub fn all_edge_segments(&self) -> Vec<([f64; 3], [f64; 3])> {
        self.order.edges.iter().flat_map(|edge| self.edge_points(edge).windows(2).map(|pair| (pair[0], pair[1])).collect::<Vec<_>>()).collect()
    }

    /// 📐️ Straight-edge segments limited to revealed kernel members (`edge:<id>` keys).
    pub fn edge_segments_for_members(&self, revealed_member_keys: &BTreeSet<String>) -> Vec<([f64; 3], [f64; 3])> {
        self.order
            .edges
            .iter()
            .filter(|edge| revealed_member_keys.contains(&format!("edge:{}", edge.id)))
            .flat_map(|edge| self.edge_points(edge).windows(2).map(|pair| (pair[0], pair[1])).collect::<Vec<_>>())
            .collect()
    }

    /// 🧲️ Every `kind:id` member key reachable from a solid, the solid itself included.
    pub fn solid_primitive_member_ids(&self, solid_id: &str) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        let Some(solid) = self.solids.get(solid_id) else { return out };
        out.insert(format!("solid:{solid_id}"));
        let mut shell_queue: Vec<&str> = solid.shell_ids.iter().map(String::as_str).collect();
        while let Some(shell_id) = shell_queue.pop() {
            if !out.insert(format!("shell:{shell_id}")) {
                continue;
            }
            let Some(shell) = self.shells.get(shell_id) else { continue };
            for face_id in &shell.face_ids {
                if !out.insert(format!("face:{face_id}")) {
                    continue;
                }
                let Some(face) = self.faces.get(face_id.as_str()) else { continue };
                for wire_id in &face.wire_ids {
                    if !out.insert(format!("wire:{wire_id}")) {
                        continue;
                    }
                    let Some(wire) = self.wires.get(wire_id.as_str()) else { continue };
                    for edge_id in &wire.edge_ids {
                        if !out.insert(format!("edge:{edge_id}")) {
                            continue;
                        }
                        let Some(edge) = self.edges.get(edge_id.as_str()) else { continue };
                        for vertex_id in &edge.vertex_ids {
                            out.insert(format!("vertex:{vertex_id}"));
                        }
                    }
                }
            }
        }
        out
    }
}
//#endregion 🔖️Buckets

//#region 🔖️Indices
fn object_primitive_entries(object: &CadObject) -> Vec<(ModelEntityKind, &str)> {
    object.primitives.iter().filter_map(|slot| ModelEntityKind::parse(&slot.kind).map(|kind| (kind, slot.primitive_id.as_str()))).collect()
}

fn object_primary_primitive_ref(object: &CadObject) -> Option<&str> {
    object.primitives.first().map(|slot| slot.primitive_id.as_str())
}

/// 🧭️ Object rows whose typology is declared under a model definition.
pub fn list_model_objects_for_model_definition<'a>(objects: &'a [CadObject], model_definition_id: &str) -> Vec<&'a CadObject> {
    let typologies = model_definition_typology_ids(Some(model_definition_id));
    objects.iter().filter(|object| typologies.contains(&object.typology)).collect()
}

/// 🏷️ Maps kernel member keys (`face:f0`, `solid:s0`, …) to the typology of the owning object row.
pub fn build_geometry_typology_index(objects: &[CadObject], geometry: &CadGeometry, model_definition_id: &str) -> BTreeMap<String, String> {
    let buckets = geometry_buckets(geometry);
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    for object in list_model_objects_for_model_definition(objects, model_definition_id) {
        for (kind, primitive_ref) in object_primitive_entries(object) {
            if kind == ModelEntityKind::Solid {
                for key in buckets.solid_primitive_member_ids(primitive_ref) {
                    out.insert(key, object.typology.clone());
                }
                continue;
            }
            out.insert(format!("{}:{primitive_ref}", kind.as_str()), object.typology.clone());
        }
    }
    for anchor in &buckets.anchors {
        let Some((kind, id)) = anchor.attachment.as_ref() else { continue };
        if let Some(mapped) = out.get(&format!("{kind}:{id}")).cloned() {
            out.insert(format!("anchor:{}", anchor.id), mapped);
        }
    }
    out
}

/// 🧭️ Maps kernel member keys to the owning object id, for hover-reveal gating.
pub fn build_geometry_object_index(objects: &[CadObject], geometry: &CadGeometry, model_definition_id: &str) -> BTreeMap<String, String> {
    let buckets = geometry_buckets(geometry);
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    for object in list_model_objects_for_model_definition(objects, model_definition_id) {
        out.insert(format!("object:{}", object.id), object.id.clone());
        for (kind, primitive_ref) in object_primitive_entries(object) {
            out.insert(format!("{}:{primitive_ref}", kind.as_str()), object.id.clone());
            if kind == ModelEntityKind::Solid {
                out.insert(format!("object:{primitive_ref}"), object.id.clone());
                for key in buckets.solid_primitive_member_ids(primitive_ref) {
                    out.insert(key, object.id.clone());
                }
            }
        }
    }
    for solid in &geometry.solids {
        let key = format!("solid:{}", solid.id);
        if out.contains_key(&key) {
            continue;
        }
        out.insert(key, solid.id.clone());
        out.insert(format!("object:{}", solid.id), solid.id.clone());
        for member in buckets.solid_primitive_member_ids(&solid.id) {
            out.entry(member).or_insert_with(|| solid.id.clone());
        }
    }
    for anchor in &buckets.anchors {
        let Some((kind, id)) = anchor.attachment.as_ref() else { continue };
        if let Some(mapped) = out.get(&format!("{kind}:{id}")).cloned() {
            out.insert(format!("anchor:{}", anchor.id), mapped);
        }
    }
    out
}
//#endregion 🔖️Indices

//#region 🔖️Targets
fn object_pick_points(object: &CadObject, buckets: &GeometryBuckets<'_>) -> Vec<[f64; 3]> {
    let Some(primitive_ref) = object_primary_primitive_ref(object) else { return Vec::new() };
    buckets.solids.get(primitive_ref).map(|solid| buckets.solid_points(solid)).unwrap_or_default()
}

fn model_object_pick_targets(objects: &[CadObject], buckets: &GeometryBuckets<'_>, model_definition_id: &str) -> Vec<SpatialPickTarget> {
    let mut out = Vec::new();
    for object in list_model_objects_for_model_definition(objects, model_definition_id) {
        let points = object_pick_points(object, buckets);
        let Some(point) = point_centroid(&points) else { continue };
        out.push(SpatialPickTarget { kind: SpatialPickTargetKind::Object, id: object.id.clone(), point, points, geometry_kind: None, typology_id: Some(object.typology.clone()) });
    }
    out
}

fn append_primitive_pick_targets(out: &mut Vec<SpatialPickTarget>, buckets: &GeometryBuckets<'_>, entity_kinds: &[ModelEntityKind], typology_index: &BTreeMap<String, String>, skip_solid_ids: &BTreeSet<String>) {
    let with_typology = |kind: SpatialPickTargetKind, geometry_kind: ModelEntityKind, id: &str, point: [f64; 3], points: Vec<[f64; 3]>| SpatialPickTarget {
        kind,
        id: id.to_string(),
        point,
        points,
        geometry_kind: Some(geometry_kind),
        typology_id: typology_index.get(&format!("{}:{id}", geometry_kind.as_str())).cloned(),
    };
    if entity_kinds.contains(&ModelEntityKind::Anchor) {
        for anchor in &buckets.anchors {
            out.push(with_typology(SpatialPickTargetKind::Vertex, ModelEntityKind::Anchor, &anchor.id, anchor.position, Vec::new()));
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Vertex) {
        for vertex in &buckets.order.vertices {
            out.push(with_typology(SpatialPickTargetKind::Vertex, ModelEntityKind::Vertex, &vertex.id, vertex.position, Vec::new()));
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Edge) {
        for edge in &buckets.order.edges {
            let points = buckets.edge_points(edge);
            if let Some(point) = point_centroid(&points) {
                out.push(with_typology(SpatialPickTargetKind::Edge, ModelEntityKind::Edge, &edge.id, point, points));
            }
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Wire) {
        for wire in &buckets.order.wires {
            let points = buckets.wire_points(wire);
            if let Some(point) = point_centroid(&points) {
                out.push(with_typology(SpatialPickTargetKind::Edge, ModelEntityKind::Wire, &wire.id, point, points));
            }
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Face) {
        for face in &buckets.order.faces {
            let points = buckets.face_points(face);
            if let Some(point) = point_centroid(&points) {
                out.push(with_typology(SpatialPickTargetKind::Face, ModelEntityKind::Face, &face.id, point, points));
            }
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Shell) {
        for shell in &buckets.order.shells {
            let segments = buckets.entity_wire_segments(ModelEntityKind::Shell, &shell.id);
            let points: Vec<[f64; 3]> = segments.into_iter().flat_map(|(a, b)| [a, b]).collect();
            if let Some(point) = point_centroid(&points) {
                out.push(with_typology(SpatialPickTargetKind::Face, ModelEntityKind::Shell, &shell.id, point, points));
            }
        }
    }
    if entity_kinds.contains(&ModelEntityKind::Solid) {
        let all = buckets.all_vertex_points();
        let all_center = point_centroid(&all);
        for solid in &buckets.order.solids {
            if skip_solid_ids.contains(&solid.id) {
                continue;
            }
            let points = buckets.solid_points(solid);
            let Some(point) = point_centroid(&points).or(all_center) else { continue };
            let points = if points.is_empty() { all.clone() } else { points };
            out.push(with_typology(SpatialPickTargetKind::Object, ModelEntityKind::Solid, &solid.id, point, points));
        }
    }
}

/// 🧲️ Builds every snap/select target for a pane from its kernel geometry and typology object rows.
pub fn create_spatial_pick_targets(objects: &[CadObject], geometry: Option<&CadGeometry>, active_model_definition_id: Option<&str>) -> Vec<SpatialPickTarget> {
    let Some(geometry) = geometry else { return Vec::new() };
    let fallback: &str = default_model_definition_id();
    let md_id = active_model_definition_id.unwrap_or(fallback);
    let buckets = geometry_buckets(geometry);
    let entity_kinds = model_definition_selection_entity_kinds(md_id);
    let scoped_objects = if is_shape_model_definition(Some(md_id)) { Vec::new() } else { list_model_objects_for_model_definition(objects, md_id) };
    let typology_index = build_geometry_typology_index(objects, geometry, md_id);
    let mut targets: Vec<SpatialPickTarget> = Vec::new();
    if entity_kinds.contains(&ModelEntityKind::Object) && !scoped_objects.is_empty() {
        targets.extend(model_object_pick_targets(objects, &buckets, md_id));
    }
    if model_definition_uses_geometry_picking(md_id) {
        let skip_solid_ids: BTreeSet<String> = scoped_objects.iter().filter_map(|object| object_primary_primitive_ref(object)).map(str::to_string).collect();
        append_primitive_pick_targets(&mut targets, &buckets, &entity_kinds, &typology_index, &skip_solid_ids);
    }
    targets
}

/// 🎯️ Keeps targets admitted by an interaction's `selection.accept` list and the enabled pick kinds.
pub fn filter_spatial_pick_targets(targets: Vec<SpatialPickTarget>, accept: &[ModelEntityKind], toggles: SpatialPickKindToggles) -> Vec<SpatialPickTarget> {
    let accept_kinds = spatial_pick_kinds_for_selection_accept(accept);
    targets
        .into_iter()
        .filter(|target| {
            if !toggles.enabled(target.kind) {
                return false;
            }
            let Some(accept_kinds) = accept_kinds.as_ref() else { return true };
            if accept_kinds.contains(&target.kind) {
                return true;
            }
            pick_target_primitive_kind(target).is_some_and(|primitive| accept.contains(&primitive))
        })
        .collect()
}

/// 🧲️ The statechart event payload a canvas pick dispatches (snap point + selection metadata).
pub fn create_spatial_pick_event(kind: SpatialPickKind, point: [f64; 3], target: Option<&SpatialPickTarget>, modifiers: &[(&str, bool)]) -> DslValue {
    let vec3 = |p: [f64; 3]| DslValue::Array(p.iter().map(|v| DslValue::float(*v)).collect());
    let mut entries: Vec<(String, DslValue)> = vec![
        ("kind".to_string(), DslValue::String(kind.as_str().to_string())),
        ("point".to_string(), vec3(point)),
        ("modifiers".to_string(), DslValue::object(modifiers.iter().map(|(name, value)| ((*name).to_string(), DslValue::Bool(*value))))),
    ];
    if let Some(target) = target {
        let geometry_kind = if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() { ModelEntityKind::Object } else { kernel_geometry_kind_for_object_pick(target.kind, target.geometry_kind) };
        entries.push(("snap".to_string(), DslValue::object([("kind".to_string(), DslValue::String(geometry_kind.as_str().to_string())), ("id".to_string(), DslValue::String(target.id.clone())), ("point".to_string(), vec3(target.point))])));
        entries.push(("selection".to_string(), DslValue::object([("kind".to_string(), DslValue::String(geometry_kind.as_str().to_string())), ("id".to_string(), DslValue::String(target.id.clone()))])));
    }
    DslValue::object(entries)
}
//#endregion 🔖️Targets

//#region 🔖️Toggles
/// 👁️ All geometry pick kinds enabled.
pub fn default_spatial_pick_kind_toggles() -> SpatialPickKindToggles {
    SpatialPickKindToggles { object: Some(true), face: Some(true), edge: Some(true), vertex: Some(true) }
}

/// 👁️ Filters pick targets by visibility (show/hide highlights); never affects ray pick or selection.
pub fn filter_spatial_pick_targets_for_visibility(targets: Vec<SpatialPickTarget>, filter_kind_toggles: SpatialPickKindToggles) -> Vec<SpatialPickTarget> {
    targets.into_iter().filter(|target| filter_kind_toggles.enabled(target.kind)).collect()
}

/// 👁️ Hide/lock flags authored on a pane object row.
pub fn spatial_entity_flags_for_object(objects: &[CadObject], entity_id: &str) -> SpatialEntityFlags {
    objects.iter().find(|object| object.id == entity_id).map_or(SpatialEntityFlags::default(), |object| SpatialEntityFlags { hidden: !object.visible, locked: object.locked })
}

/// 👁️ A kernel member inherits the hide/lock state of the object row that owns it — the owner is
/// looked up through the same fixed alias order React uses (`object`, `solid`, `face`, `edge`,
/// `vertex`, `anchor`), and only a `true` flag on the owner is inherited.
pub fn resolve_spatial_entity_flags(objects: &[CadObject], geometry: &CadGeometry, model_definition_id: &str, entity_id: &str) -> SpatialEntityFlags {
    let direct = spatial_entity_flags_for_object(objects, entity_id);
    if direct.hidden || direct.locked {
        return direct;
    }
    let index = build_geometry_object_index(objects, geometry, model_definition_id);
    let Some(owner) = ["object", "solid", "face", "edge", "vertex", "anchor"].into_iter().find_map(|kind| index.get(&format!("{kind}:{entity_id}"))) else {
        return direct;
    };
    let owner_flags = spatial_entity_flags_for_object(objects, owner);
    if !owner_flags.hidden && !owner_flags.locked {
        return direct;
    }
    SpatialEntityFlags { hidden: direct.hidden || owner_flags.hidden, locked: direct.locked || owner_flags.locked }
}

/// 👁️ Excludes hidden/locked entities from canvas pick and selection.
pub fn filter_spatial_pick_targets_for_entity_flags(targets: Vec<SpatialPickTarget>, flags_for_id: &impl Fn(&str) -> SpatialEntityFlags) -> Vec<SpatialPickTarget> {
    targets.into_iter().filter(|target| flags_for_id(&target.id).selectable()).collect()
}

/// 👁️ Drops locked/hidden entities from committed selection targets.
pub fn prune_selection_targets_for_entity_flags(targets: Vec<SelectionTarget>, flags_for_id: &impl Fn(&str) -> SpatialEntityFlags) -> Vec<SelectionTarget> {
    targets.into_iter().filter(|target| flags_for_id(&target.id).selectable()).collect()
}

/// 👁️ Effective pick kinds must be both visible AND enabled for selection/hover.
pub fn intersect_spatial_pick_kind_toggles(visible: SpatialPickKindToggles, selection: SpatialPickKindToggles) -> SpatialPickKindToggles {
    let mut merged = SpatialPickKindToggles::default();
    for kind in SPATIAL_PICK_TARGET_KINDS {
        if visible.get(*kind) == Some(false) || selection.get(*kind) == Some(false) {
            merged.set(*kind, Some(false));
        }
    }
    merged
}

/// 👁️ Pick kinds the active model definition's entity kinds map onto.
pub fn model_definition_pick_target_kinds(model_definition_id: Option<&str>) -> Vec<SpatialPickTargetKind> {
    let fallback: &str = default_model_definition_id();
    let md_id = model_definition_id.unwrap_or(fallback);
    let mut out: Vec<SpatialPickTargetKind> = Vec::new();
    for kind in model_definition_selection_entity_kinds(md_id) {
        let mapped = match kind {
            ModelEntityKind::Vertex | ModelEntityKind::Anchor => Some(SpatialPickTargetKind::Vertex),
            ModelEntityKind::Edge | ModelEntityKind::Wire => Some(SpatialPickTargetKind::Edge),
            ModelEntityKind::Face | ModelEntityKind::Shell => Some(SpatialPickTargetKind::Face),
            ModelEntityKind::Solid | ModelEntityKind::Geometry | ModelEntityKind::Object => Some(SpatialPickTargetKind::Object),
            _ => None,
        };
        if let Some(mapped) = mapped {
            if !out.contains(&mapped) {
                out.push(mapped);
            }
        }
    }
    if !out.is_empty() {
        return out;
    }
    if is_shape_model_definition(model_definition_id) {
        SPATIAL_PICK_TARGET_KINDS.to_vec()
    } else {
        vec![SpatialPickTargetKind::Object]
    }
}

/// 👁️ Default visibility/selection toggles for the kinds the active model definition allows.
pub fn default_spatial_pick_kind_toggles_for_model_definition(model_definition_id: Option<&str>) -> SpatialPickKindToggles {
    let allowed = model_definition_pick_target_kinds(model_definition_id);
    let mut toggles = SpatialPickKindToggles::default();
    for kind in SPATIAL_PICK_TARGET_KINDS {
        toggles.set(*kind, Some(allowed.contains(kind)));
    }
    toggles
}

/// 👁️ Every typology on the active model definition enabled for show/selection.
pub fn default_spatial_typology_toggles_for_model_definition(model_definition_id: Option<&str>) -> SpatialTypologyToggles {
    SpatialTypologyToggles(model_definition_typology_ids(model_definition_id).into_iter().map(|id| (id, true)).collect())
}

fn typology_toggle_allows_target(target: &SpatialPickTarget, toggles: &SpatialTypologyToggles, typology_ids: &[String]) -> bool {
    match target.typology_id.as_deref() {
        Some(typology_id) => toggles.enabled(typology_id),
        None => typology_ids.iter().any(|id| toggles.enabled(id)),
    }
}

/// 👁️ Filters pick targets by typology show/selection toggles.
pub fn filter_spatial_pick_targets_for_typology_toggles(targets: Vec<SpatialPickTarget>, toggles: &SpatialTypologyToggles, typology_ids: &[String]) -> Vec<SpatialPickTarget> {
    targets.into_iter().filter(|target| typology_toggle_allows_target(target, toggles, typology_ids)).collect()
}

/// 👁️ Derives per-kind toggles from typology-filtered targets (scene layers + legacy gates).
pub fn spatial_pick_kind_toggles_from_typology_filtered_targets(model_definition_id: Option<&str>, visible_targets: &[SpatialPickTarget]) -> SpatialPickKindToggles {
    let allowed = model_definition_pick_target_kinds(model_definition_id);
    let mut merged = SpatialPickKindToggles::default();
    for kind in SPATIAL_PICK_TARGET_KINDS {
        merged.set(*kind, Some(allowed.contains(kind) && visible_targets.iter().any(|target| target.kind == *kind)));
    }
    merged
}

/// 🧱️ Kernel primitive kinds toggled in play chrome.
pub const SPATIAL_PRIMITIVE_KINDS: &[ModelEntityKind] = PRIMITIVE_MODEL_ENTITY_KINDS;

/// 👁️ All kernel primitive kinds enabled for show/filter.
pub fn default_spatial_primitive_toggles() -> SpatialPrimitiveToggles {
    SpatialPrimitiveToggles(SPATIAL_PRIMITIVE_KINDS.iter().map(|kind| (*kind, true)).collect())
}

/// 👁️ Scene-layer pick-kind toggles from model definition + primitive show toggles.
pub fn spatial_scene_kind_toggles_for_model_definition(model_definition_id: Option<&str>, primitive_toggles: &SpatialPrimitiveToggles) -> SpatialPickKindToggles {
    let mut toggles = default_spatial_pick_kind_toggles_for_model_definition(model_definition_id);
    let off = |kind: ModelEntityKind| primitive_toggles.0.get(&kind) == Some(&false);
    if off(ModelEntityKind::Vertex) && off(ModelEntityKind::Anchor) {
        toggles.vertex = Some(false);
    }
    if off(ModelEntityKind::Edge) && off(ModelEntityKind::Wire) {
        toggles.edge = Some(false);
    }
    if off(ModelEntityKind::Face) && off(ModelEntityKind::Shell) {
        toggles.face = Some(false);
    }
    if off(ModelEntityKind::Solid) {
        toggles.object = Some(false);
    }
    toggles
}

/// 👁️ Filters pick targets by primitive show/filter toggles (typology object rows pass through).
pub fn filter_spatial_pick_targets_for_primitive_toggles(targets: Vec<SpatialPickTarget>, toggles: &SpatialPrimitiveToggles) -> Vec<SpatialPickTarget> {
    targets
        .into_iter()
        .filter(|target| match pick_target_primitive_kind(target) {
            None => true,
            Some(primitive) => toggles.enabled(primitive),
        })
        .collect()
}

/// ☑️ Whether every key in a toggle group is on, every key is off, or the group is mixed.
pub fn spatial_toggle_group_state(keys: &[String], toggles: &BTreeMap<String, bool>) -> SpatialToggleGroupState {
    if keys.is_empty() {
        return SpatialToggleGroupState::None;
    }
    let on = keys.iter().filter(|key| toggles.get(*key) != Some(&false)).count();
    if on == 0 {
        SpatialToggleGroupState::None
    } else if on == keys.len() {
        SpatialToggleGroupState::All
    } else {
        SpatialToggleGroupState::Partial
    }
}

/// ☑️ Sets every key in a chrome toggle group on or off.
pub fn spatial_toggle_group_fill(keys: &[String], enabled: bool) -> BTreeMap<String, bool> {
    keys.iter().map(|key| (key.clone(), enabled)).collect()
}

/// ☑️ Maps a chrome group aggregate onto the owned tri-state checkbox value.
pub fn spatial_toggle_checkbox_state(state: SpatialToggleGroupState) -> SpatialToggleCheckboxState {
    match state {
        SpatialToggleGroupState::Partial => SpatialToggleCheckboxState::Indeterminate,
        SpatialToggleGroupState::All => SpatialToggleCheckboxState::Checked,
        SpatialToggleGroupState::None => SpatialToggleCheckboxState::Unchecked,
    }
}

/// 👁️ Resolves which scene layers stay visible for geometry edit vs typology object picking.
pub fn resolve_spatial_scene_visibility(active_model_definition_id: Option<&str>, filter_kind_toggles: SpatialPickKindToggles) -> SpatialSceneVisibility {
    let fallback: &str = default_model_definition_id();
    let md_id = active_model_definition_id.unwrap_or(fallback);
    if model_definition_uses_geometry_picking(md_id) {
        return SpatialSceneVisibility {
            show_factory_wireframe: filter_kind_toggles.enabled(SpatialPickTargetKind::Edge),
            show_committed_faces: filter_kind_toggles.enabled(SpatialPickTargetKind::Face) || filter_kind_toggles.enabled(SpatialPickTargetKind::Object),
            show_committed_edges: filter_kind_toggles.enabled(SpatialPickTargetKind::Edge),
        };
    }
    SpatialSceneVisibility { show_factory_wireframe: false, show_committed_faces: false, show_committed_edges: false }
}

/// 👁️ Keeps pick targets the active model definition allows (primitives + typology objects).
pub fn filter_spatial_pick_targets_for_active_view(targets: Vec<SpatialPickTarget>, active_model_definition_id: Option<&str>) -> Vec<SpatialPickTarget> {
    let fallback: &str = default_model_definition_id();
    let md_id = active_model_definition_id.unwrap_or(fallback);
    let allowed_pick_kinds = model_definition_pick_target_kinds(Some(md_id));
    let entity_kinds = model_definition_selection_entity_kinds(md_id);
    let is_shape = is_shape_model_definition(Some(md_id));
    targets
        .into_iter()
        .filter(|target| {
            if !allowed_pick_kinds.contains(&target.kind) {
                return false;
            }
            if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() {
                return entity_kinds.contains(&ModelEntityKind::Object) && !is_shape;
            }
            entity_kinds.contains(&kernel_geometry_kind_for_object_pick(target.kind, target.geometry_kind))
        })
        .collect()
}
//#endregion 🔖️Toggles

//#region 🔖️Reveal
/// 👁️ Object ids whose kernel primitives should draw (hover/selection on the object or its topology).
pub fn revealed_object_ids_from_pick_keys(object_index: &BTreeMap<String, String>, hovered_target_key: Option<&str>, selected_target_keys: &BTreeSet<String>) -> BTreeSet<String> {
    let mut revealed = BTreeSet::new();
    let mut consider = |key: Option<&str>| {
        for alias in spatial_hover_key_aliases(key) {
            if let Some(owner) = object_index.get(&alias) {
                revealed.insert(owner.clone());
            }
        }
    };
    consider(hovered_target_key);
    for key in selected_target_keys {
        consider(Some(key));
    }
    revealed
}

fn spatial_pick_target_object_revealed(target: &SpatialPickTarget, object_index: &BTreeMap<String, String>, revealed_object_ids: &BTreeSet<String>) -> bool {
    if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() {
        return true;
    }
    object_index.get(&spatial_pick_target_member_key(target)).is_some_and(|owner| revealed_object_ids.contains(owner))
}

/// 📌️ Visibility-enabled pick highlights plus pinned hover/selection targets (pinned win over hidden).
pub fn resolve_spatial_pick_targets_to_render(view_targets: &[SpatialPickTarget], filter_kind_toggles: SpatialPickKindToggles, pinned_target_keys: &BTreeSet<String>, flags_for_id: &impl Fn(&str) -> SpatialEntityFlags, object_reveal: Option<(&BTreeMap<String, String>, &BTreeSet<String>)>) -> Vec<SpatialPickTarget> {
    let pinned_keys = pinned_pick_target_keys(pinned_target_keys.iter().map(String::as_str));
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut out: Vec<SpatialPickTarget> = Vec::new();
    for target in view_targets.iter().filter(|target| filter_kind_toggles.enabled(target.kind)) {
        let key = spatial_pick_target_key(target);
        if flags_for_id(&target.id).hidden && !pinned_keys.contains(&key) {
            continue;
        }
        if let Some((object_index, revealed)) = object_reveal {
            if !pinned_keys.contains(&key) && !spatial_pick_target_object_revealed(target, object_index, revealed) {
                continue;
            }
        }
        if !seen.insert(key) {
            continue;
        }
        out.push(target.clone());
    }
    for target in view_targets {
        let key = spatial_pick_target_key(target);
        if !pinned_keys.contains(&key) || seen.contains(&key) {
            continue;
        }
        seen.insert(key);
        out.push(target.clone());
    }
    out
}
//#endregion 🔖️Reveal

//#region 🔖️Selection
/// 🖱️ The selection merge a pointer's modifiers ask for — the framework's one wire vocabulary.
pub fn spatial_merge_mode_from_modifiers(shift: bool, ctrl: bool, meta: bool) -> MergeMode {
    let ctrl = ctrl || meta;
    match (shift, ctrl) {
        (true, true) => MergeMode::Invertive,
        (true, false) => MergeMode::Additive,
        (false, true) => MergeMode::Subtractive,
        (false, false) => MergeMode::Replace,
    }
}

/// 🖱️ Deduplicates selection rows by `kind:id`, first occurrence wins.
pub fn unique_selection_targets(targets: Vec<SelectionTarget>) -> Vec<SelectionTarget> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    targets.into_iter().filter(|target| seen.insert(selection_target_hover_key(target))).collect()
}

/// 🖱️ Applies a merge mode to the current selection. `Range` has no ordered topology on a spatial
/// canvas, so it behaves as `Replace` — the same choice `marqueeModeFromModifiers` makes by never
/// producing it.
pub fn merge_selection_targets(current: &[SelectionTarget], next: Vec<SelectionTarget>, mode: MergeMode) -> Vec<SelectionTarget> {
    let unique_next = unique_selection_targets(next);
    let next_keys: BTreeSet<String> = unique_next.iter().map(selection_target_hover_key).collect();
    match mode {
        MergeMode::Replace | MergeMode::Range => unique_next,
        MergeMode::Additive => {
            let mut seen: BTreeSet<String> = current.iter().map(selection_target_hover_key).collect();
            let mut merged = current.to_vec();
            for target in unique_next {
                if seen.insert(selection_target_hover_key(&target)) {
                    merged.push(target);
                }
            }
            merged
        }
        MergeMode::Subtractive => current.iter().filter(|target| !next_keys.contains(&selection_target_hover_key(target))).cloned().collect(),
        MergeMode::Invertive => {
            let current_keys: BTreeSet<String> = current.iter().map(selection_target_hover_key).collect();
            let mut merged: Vec<SelectionTarget> = current.iter().filter(|target| !next_keys.contains(&selection_target_hover_key(target))).cloned().collect();
            merged.extend(unique_next.into_iter().filter(|target| !current_keys.contains(&selection_target_hover_key(target))));
            merged
        }
    }
}

/// 🖱️ The committed selection row a pick target resolves to.
pub fn spatial_selection_target(target: &SpatialPickTarget) -> SelectionTarget {
    if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() {
        return SelectionTarget { kind: ModelEntityKind::Object, id: target.id.clone(), editable: false };
    }
    SelectionTarget { kind: kernel_geometry_kind_for_object_pick(target.kind, target.geometry_kind), id: target.id.clone(), editable: true }
}

/// 🖱️ Maps a committed selection row to the canvas hover key (the typology object when one owns it).
pub fn canvas_hover_key_for_selection_target(objects: &[CadObject], geometry: &CadGeometry, model_definition_id: &str, target: &SelectionTarget) -> String {
    if target.kind == ModelEntityKind::Object {
        return selection_target_hover_key(target);
    }
    let buckets = geometry_buckets(geometry);
    let member_key = format!("{}:{}", target.kind.as_str(), target.id);
    for object in list_model_objects_for_model_definition(objects, model_definition_id) {
        for (kind, primitive_ref) in object_primitive_entries(object) {
            if target.kind == kind && target.id == primitive_ref {
                return selection_target_hover_key(&SelectionTarget { kind: ModelEntityKind::Object, id: object.id.clone(), editable: false });
            }
            if kind == ModelEntityKind::Solid && buckets.solid_primitive_member_ids(primitive_ref).contains(&member_key) {
                return selection_target_hover_key(&SelectionTarget { kind: ModelEntityKind::Object, id: object.id.clone(), editable: false });
            }
        }
    }
    selection_target_hover_key(target)
}
//#endregion 🔖️Selection

//#region 🔖️Style
/// 🎨️ A colour slot on the spatial scene palette. The plugin never resolves theme hex — W1k's rule
/// is that token ids cross the wire and the renderer resolves them, so a style names its slot and
/// only an authored typology style carries a literal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpatialColorRef {
    Token(&'static str),
    Hex(String),
}

/// 🎨️ Palette slots the spatial canvas materials read, as the CSS variables React resolves.
pub struct SpatialScenePalette;

impl SpatialScenePalette {
    pub const SELECTED: &'static str = "--color-changed-selected";
    pub const SELECTED_EMISSIVE: &'static str = "--active-base";
    pub const HOVERED: &'static str = "--color-changed-hovered";
    pub const HOVERED_EMISSIVE: &'static str = "--hover-panel";
    pub const VERTEX: &'static str = "--foreground";
    pub const VERTEX_EMISSIVE: &'static str = "--hover-base";
    pub const EDGE: &'static str = "--border-normal-color";
    pub const EDGE_EMISSIVE: &'static str = "--hover-base";
    pub const OBJECT: &'static str = "--muted-foreground";
    pub const OBJECT_EMISSIVE: &'static str = "--hover-panel";
    pub const FACE: &'static str = "--accent-secondary";
    pub const FACE_EMISSIVE: &'static str = "--hover-window";
}

/// 🌫️ `WORLD_LOCKED_OPACITY_SCALE` (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:178`) — how far a locked
/// entity's highlight is dimmed.
pub const WORLD_LOCKED_OPACITY_SCALE: f64 = 0.35;

/// 🎨️ The resolved highlight material for one pick target in one interaction state.
#[derive(Clone, Debug, PartialEq)]
pub struct SpatialTargetStyle {
    pub color: SpatialColorRef,
    pub emissive: SpatialColorRef,
    pub opacity: f64,
    pub line_width: f64,
}

/// 🎨️ Material props a typology style contributes to a committed mesh.
pub fn typology_style_to_material_props(style: &ResolvedTypologyStyle) -> (String, String, f64) {
    (style.color.clone(), style.color.clone(), style.opacity)
}

/// 🎨️ Per-solid typology style from object-row membership (React's `createSolidTypologyStyleResolver`).
pub fn solid_typology_style(objects: &[CadObject], geometry: &CadGeometry, model_definition_id: &str, solid_id: &str) -> Option<ResolvedTypologyStyle> {
    let index = build_geometry_typology_index(objects, geometry, model_definition_id);
    index.get(&format!("solid:{solid_id}")).map(|typology| resolve_typology_style(typology))
}

/// 🎨️ The exact selected/hovered/locked/typology/default cascade React's `targetStyle` applies.
pub fn target_style(target: &SpatialPickTarget, hovered: bool, selected: bool, typology_style: Option<&ResolvedTypologyStyle>, locked: bool) -> SpatialTargetStyle {
    let is_vertex = target.kind == SpatialPickTargetKind::Vertex;
    let is_typology_object = target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none();
    if selected {
        return SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::SELECTED), emissive: SpatialColorRef::Token(SpatialScenePalette::SELECTED_EMISSIVE), opacity: if is_vertex { 1.0 } else { 0.34 }, line_width: 9.0 };
    }
    if hovered {
        return SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::HOVERED), emissive: SpatialColorRef::Token(SpatialScenePalette::HOVERED_EMISSIVE), opacity: if is_vertex { 1.0 } else { 0.28 }, line_width: 8.0 };
    }
    if locked {
        let dim = (if is_vertex { 0.55 } else { 0.12 }) * WORLD_LOCKED_OPACITY_SCALE;
        if let Some(style) = typology_style {
            return SpatialTargetStyle { color: SpatialColorRef::Hex(style.color.clone()), emissive: SpatialColorRef::Hex(style.color.clone()), opacity: dim, line_width: 4.0 };
        }
        let (color, emissive, line_width) = if is_vertex {
            (SpatialScenePalette::VERTEX, SpatialScenePalette::VERTEX_EMISSIVE, 4.0)
        } else if target.kind == SpatialPickTargetKind::Edge {
            (SpatialScenePalette::EDGE, SpatialScenePalette::EDGE_EMISSIVE, 4.0)
        } else if is_typology_object {
            (SpatialScenePalette::OBJECT, SpatialScenePalette::OBJECT_EMISSIVE, 5.0)
        } else {
            (SpatialScenePalette::FACE, SpatialScenePalette::FACE_EMISSIVE, 4.0)
        };
        return SpatialTargetStyle { color: SpatialColorRef::Token(color), emissive: SpatialColorRef::Token(emissive), opacity: dim, line_width };
    }
    if let Some(style) = typology_style {
        if is_vertex {
            return SpatialTargetStyle { color: SpatialColorRef::Hex(style.color.clone()), emissive: SpatialColorRef::Hex(style.color.clone()), opacity: 1.0, line_width: 5.0 };
        }
        if target.kind == SpatialPickTargetKind::Edge {
            return SpatialTargetStyle { color: SpatialColorRef::Hex(style.edge_color.clone()), emissive: SpatialColorRef::Hex(style.edge_color.clone()), opacity: 0.85, line_width: 5.0 };
        }
        if is_typology_object {
            return SpatialTargetStyle { color: SpatialColorRef::Hex(style.color.clone()), emissive: SpatialColorRef::Hex(style.color.clone()), opacity: style.opacity, line_width: 7.0 };
        }
        return SpatialTargetStyle { color: SpatialColorRef::Hex(style.color.clone()), emissive: SpatialColorRef::Hex(style.color.clone()), opacity: style.opacity.min(0.42), line_width: 5.0 };
    }
    if is_vertex {
        return SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::VERTEX), emissive: SpatialColorRef::Token(SpatialScenePalette::VERTEX_EMISSIVE), opacity: 1.0, line_width: 5.0 };
    }
    if target.kind == SpatialPickTargetKind::Edge {
        return SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::EDGE), emissive: SpatialColorRef::Token(SpatialScenePalette::EDGE_EMISSIVE), opacity: 0.8, line_width: 5.0 };
    }
    if is_typology_object {
        return SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::OBJECT), emissive: SpatialColorRef::Token(SpatialScenePalette::OBJECT_EMISSIVE), opacity: 0.28, line_width: 7.0 };
    }
    SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::FACE), emissive: SpatialColorRef::Token(SpatialScenePalette::FACE_EMISSIVE), opacity: 0.16, line_width: 5.0 }
}

/// 🏷️ Chrome label for one typology toggle row, authored label first.
pub fn typology_toggle_row_label(typology_id: &str) -> String {
    spatial_typology_toggle_label(typology_id, load_typology(typology_id).map(|spec| spec.label.as_str()))
}
//#endregion 🔖️Style

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
