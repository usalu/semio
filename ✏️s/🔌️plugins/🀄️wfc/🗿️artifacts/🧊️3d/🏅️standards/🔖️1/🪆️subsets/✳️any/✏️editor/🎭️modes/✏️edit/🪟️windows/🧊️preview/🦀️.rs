//! 🧊️ WFC `wfc-3d-preview` window — the SOLVED assembly, on `SurfaceKind::World3d`.
//!
//! The solve is an inference, so nothing this window draws is persisted: the assignment arrives
//! through the app transient (`Wfc3dTransient`), which the editor refreshes from
//! `crate::inferences::solve_assignments`. The window itself is read-only.
//!
//! Instancing contract (`📓️explore-editor-window-and-media-patterns.md` §6.4): `meshes_json` is the
//! SMALL catalogue — one entry per distinct TILE, whatever the slot count — and `instances_json` is
//! the many-record lane, one record per SOLVED slot referencing its tile's `meshId`. Every instance
//! sharing a mesh key is drawn with ONE hardware instanced draw call, so a hundred slots over three
//! tiles costs three draws, not a hundred.
//!
//! An UNSOLVED slot is OMITTED, never drawn as a placeholder box — the same strict contract the
//! sibling `grid3d` preview keeps, and the honest one: this window paints the SOLUTION, so a slot the
//! solve did not reach has nothing to show and the `status_json` verdict is what says why. The shared
//! `tile:__placeholder` mesh survives for a different case: a SOLVED slot whose tile's media resolves
//! to no geometry (an unhydrated `MeshChild`) still gets a body rather than vanishing.
//!
//! Tile space is the unit box `0..1` on every axis; a slot's `x`/`y`/`z` is its box's MINIMUM corner
//! and `width`/`height`/`depth` its extent, so an instance is `position = slot origin`,
//! `scale = slot extent`. That is what lets `tower-stack`'s differently-sized storeys share one mesh.

use crate::editor::wfc3d::modes::edit::tools::fill::{self, Wfc3dFillTickPayload};
use crate::editor::wfc3d::transient::{assigned_tile, Wfc3dTransient};
use semio_framework_plugin::ToolRunView;
use crate::schema::snapshot::{Slot3d, Tile, TileMedia3d, Wfc3dSnapshot};
use semio_framework_plugin::{
    scene_surface, BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions, World3dScene, world3d_camera_projection_json, world3d_selection_json, WorldProjectionConfig,
};
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WFC_3D_PREVIEW_WINDOW: &str = "wfc-3d-preview";
pub const WFC_3D_PREVIEW_BODY: &str = "wfc.3d.preview";
const WFC_3D_PREVIEW_SURFACE: &str = "wfc.3d.preview";
/// 🥽️ The mesh id one tile resolves to in the scene catalogue.
pub fn tile_mesh_id(tile_id: &str) -> String {
    format!("tile:{tile_id}")
}
/// 📦️ The mesh a SOLVED slot borrows when its own tile's media resolves to no geometry, so an
/// unhydrated `MeshChild` still has a body on screen instead of silently disappearing.
pub const WFC_3D_PLACEHOLDER_MESH: &str = "tile:__placeholder";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::wfc3d::create_wfc3d_editor`. The window
/// declares no actions: it paints a derived value and authors nothing.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WFC_3D_PREVIEW_WINDOW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: WFC_3D_PREVIEW_BODY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "box".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Meshes
/// 🥽️ One tile's resolved geometry: flat xyz `positions`, triangle `indices`, and the straight RGBA
/// tint its media declared, if any.
pub struct TileGeometry {
    pub positions: Vec<f64>,
    pub indices: Vec<u32>,
    pub color: Option<[f64; 4]>,
}

/// 🥽️ One tile's geometry, whatever its media says: inline `Mesh` verbatim, a `MeshChild` through its
/// HYDRATED local owner (a wire-decoded child the host has not materialized yet resolves to nothing
/// and falls soft), and nothing at all otherwise.
fn tile_geometry(tile: &Tile) -> Option<TileGeometry> {
    match &tile.media {
        TileMedia3d::Mesh { positions, indices, color } => {
            if positions.len() < 9 || indices.is_empty() {
                return None;
            }
            Some(TileGeometry {
                positions: positions.clone(),
                indices: indices.clone(),
                color: color.map(|color| [f64::from(color.r) / 255.0, f64::from(color.g) / 255.0, f64::from(color.b) / 255.0, f64::from(color.a) / 255.0]),
            })
        }
        TileMedia3d::MeshChild { child } => {
            let owner = child.local_owner::<semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot>()?;
            let mut positions = Vec::new();
            let mut indices = Vec::new();
            for mesh in &owner.meshes {
                for primitive in &mesh.primitives {
                    let base = u32::try_from(positions.len() / 3).ok()?;
                    for point in &primitive.positions {
                        positions.push(point.x);
                        positions.push(point.y);
                        positions.push(point.z);
                    }
                    for index in &primitive.indices {
                        indices.push(base + index);
                    }
                }
            }
            (positions.len() >= 9 && !indices.is_empty()).then_some(TileGeometry { positions, indices, color: None })
        }
    }
}

/// 📦️ The unit box every unresolvable tile borrows, so the catalogue always has something to place.
fn placeholder_geometry() -> (Vec<f64>, Vec<u32>) {
    match crate::unit_box_media(None) {
        TileMedia3d::Mesh { positions, indices, .. } => (positions, indices),
        TileMedia3d::MeshChild { .. } => (Vec::new(), Vec::new()),
    }
}

/// 📐️ Area-weighted vertex normals over the triangle list — ONE per position, so the buffer the host
/// binds has the same vertex count as `positions`.
///
/// 🩹 A catalogue entry that shipped an EMPTY `normals` array bound a zero-length `normal` attribute
/// at item size 3, and every instanced draw of that mesh died in the GL with
/// `glDrawElements: Vertex buffer is not big enough for the draw call` — the solved boxes reached the
/// pane as bare edges and never as bodies. A degenerate mesh (all triangles collinear) falls back to
/// `+Z` rather than to zeros, because a zero normal is the same crash with a different cause.
pub fn vertex_normals(positions: &[f64], indices: &[u32]) -> Vec<f64> {
    let vertices = positions.len() / 3;
    let mut normals = vec![0.0f64; vertices * 3];
    for triangle in indices.chunks(3) {
        let (Some(a), Some(b), Some(c)) = (triangle.first(), triangle.get(1), triangle.get(2)) else { continue };
        let corner = |index: &u32| -> Option<[f64; 3]> {
            let base = (*index as usize).checked_mul(3)?;
            Some([*positions.get(base)?, *positions.get(base + 1)?, *positions.get(base + 2)?])
        };
        let (Some(pa), Some(pb), Some(pc)) = (corner(a), corner(b), corner(c)) else { continue };
        let u = [pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]];
        let v = [pc[0] - pa[0], pc[1] - pa[1], pc[2] - pa[2]];
        let face = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        for index in [a, b, c] {
            let base = (*index as usize) * 3;
            if base + 2 < normals.len() {
                normals[base] += face[0];
                normals[base + 1] += face[1];
                normals[base + 2] += face[2];
            }
        }
    }
    for normal in normals.chunks_mut(3) {
        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if length > f64::EPSILON {
            normal[0] /= length;
            normal[1] /= length;
            normal[2] /= length;
        } else {
            normal[2] = 1.0;
        }
    }
    normals
}

/// 🥽️ One catalogue entry. `colors` is RGB, three floats per vertex: the host binds the `color`
/// attribute at item size 3, so a straight RGBA copy shifts every vertex' colour by one channel.
fn mesh_entry(id: &str, positions: &[f64], indices: &[u32], color: Option<[f64; 4]>) -> Value {
    let colors: Vec<f64> = match color {
        Some(rgba) => positions.chunks(3).flat_map(|_| [rgba[0], rgba[1], rgba[2]]).collect(),
        None => Vec::new(),
    };
    json!({ "id": id, "data": { "positions": positions, "normals": vertex_normals(positions, indices), "colors": colors, "indices": indices, "uvs": Vec::<f64>::new() } })
}

/// 🥽️ The SMALL mesh catalogue: one entry per distinct tile plus the shared placeholder. Never one
/// entry per slot — that is what `instances_json` is for.
pub fn meshes_json(document: &Wfc3dSnapshot) -> String {
    let (placeholder_positions, placeholder_indices) = placeholder_geometry();
    let mut meshes = vec![mesh_entry(WFC_3D_PLACEHOLDER_MESH, &placeholder_positions, &placeholder_indices, None)];
    for tile in &document.tiles {
        if let Some(geometry) = tile_geometry(tile) {
            meshes.push(mesh_entry(&tile_mesh_id(&tile.id), &geometry.positions, &geometry.indices, geometry.color));
        }
    }
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖️Meshes

//#region 🔖️Instances
/// 🧱️ ONE instance record for one solved slot: the tile's mesh placed at the slot's own box.
fn instance_record(slot: &Slot3d, mesh_id: &str, label: &str) -> Value {
    json!({
        "id": slot.id,
        "meshId": mesh_id,
        "position": [slot.x, slot.y, slot.z],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [slot.width.max(f64::EPSILON), slot.height.max(f64::EPSILON), slot.depth.max(f64::EPSILON)],
        "label": label,
    })
}

/// 🪪️ The record ONE slot contributes, or `None` when the solve did not assign it a declared tile —
/// an unsolved slot is omitted from the lane, not drawn as a placeholder. A solved slot whose tile has
/// no resolvable geometry borrows the shared placeholder mesh so it still has a body.
fn solved_instance(document: &Wfc3dSnapshot, transient: &Wfc3dTransient, slot: &Slot3d) -> Option<Value> {
    let tile_id = assigned_tile(transient, &slot.id)?;
    let tile = document.tiles.iter().find(|entry| entry.id == tile_id)?;
    let mesh_id = if tile_geometry(tile).is_some() { tile_mesh_id(&tile.id) } else { WFC_3D_PLACEHOLDER_MESH.to_string() };
    Some(instance_record(slot, &mesh_id, &format!("{} · {}", slot.id, tile.id)))
}

/// 🚚️ The AUTHORITATIVE instance set — one record per SOLVED slot, in document order.
pub fn instances_json(document: &Wfc3dSnapshot, transient: &Wfc3dTransient) -> String {
    let records: Vec<Value> = document.slots.iter().filter_map(|slot| solved_instance(document, transient, slot)).collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

/// 🚚️ The delta lane that rides ALONGSIDE the authoritative set. This window rebuilds its projection
/// per render rather than keeping residency, so every publication is a full generation: `base` 0,
/// `changed` the whole set, `removed` empty. A consumer that merges deltas gets a correct (if
/// unhelpful) answer; one that ignores the lane reads `instances_json`, which is authoritative.
pub fn instances_delta_json(document: &Wfc3dSnapshot, transient: &Wfc3dTransient) -> String {
    let changed: Vec<Value> = document.slots.iter().filter_map(|slot| solved_instance(document, transient, slot)).collect();
    let count = changed.len();
    serde_json::to_string(&json!({ "base": 0, "revision": 1, "count": count, "changed": changed, "removed": Vec::<String>::new() })).unwrap_or_else(|_| "{}".into())
}
//#endregion 🔖️Instances

//#region 🔖️Camera
/// 📷️ The axis-aligned centre and largest span of every slot box — what the opening pose frames.
fn framing_bounds(document: &Wfc3dSnapshot) -> ([f64; 3], f64) {
    let mut minimum = [f64::INFINITY; 3];
    let mut maximum = [f64::NEG_INFINITY; 3];
    for slot in &document.slots {
        let low = [slot.x, slot.y, slot.z];
        let high = [slot.x + slot.width, slot.y + slot.height, slot.z + slot.depth];
        for axis in 0..3 {
            minimum[axis] = minimum[axis].min(low[axis]);
            maximum[axis] = maximum[axis].max(high[axis]);
        }
    }
    if document.slots.is_empty() {
        return ([0.0, 0.0, 0.0], 1.0);
    }
    let centre = [(minimum[0] + maximum[0]) * 0.5, (minimum[1] + maximum[1]) * 0.5, (minimum[2] + maximum[2]) * 0.5];
    let span = (maximum[0] - minimum[0]).max(maximum[1] - minimum[1]).max(maximum[2] - minimum[2]).max(1.0);
    (centre, span)
}

/// 📷️ The pose this pane opens on before any user gesture, framed on what the document holds. Without
/// it the published camera sits at all zeros and the world lane carries no view direction at boot.
pub fn camera_json(document: &Wfc3dSnapshot, zoom: f64) -> String {
    let (target, span) = framing_bounds(document);
    let projection = WorldProjectionConfig::default();
    let distance = (span * 2.2).max(1.4);
    let (position, up) = semio_framework_plugin::world3d_projection_pose(&projection, target, distance);
    world3d_camera_projection_json(position, target, Some(up), if zoom > 0.0 { zoom } else { 1.0 }, &projection)
}
//#endregion 🔖️Camera

//#region 🔖️Render
/// 🧊️ Renders the solved assembly. Read-only: `selection_json` carries no ids, because picking a
/// solved instance would address a derived value, not a document row.
/// 🏃️ The live fill tick while the run is non-terminal and carries a decodable payload.
pub fn live_fill_payload(tool_run: Option<&ToolRunView>) -> Option<Wfc3dFillTickPayload> {
    let run = tool_run.filter(|run| run.tool_id == fill::TOOL_ID && !run.state.is_terminal())?;
    let bytes = run.payload.as_ref()?;
    fill::decode_fill_payload(bytes)
}

pub fn render(document: &Wfc3dSnapshot, transient: &Wfc3dTransient, zoom: f64) -> UiAssemblyResult<BuiltNode> {
    let scene = World3dScene {
        instances_delta_json: Some(instances_delta_json(document, transient)),
        status_json: Some(status_json(document, transient)),
        ..World3dScene::base(camera_json(document, zoom), meshes_json(document), instances_json(document, transient), world3d_selection_json("replace", &[], None))
    };
    scene_surface(WFC_3D_PREVIEW_SURFACE, ContractSurfaceKind::World3d, &scene)
}

/// 🩺 The one-line verdict the preview banner shows. Deliberately short: a window body's owned text
/// is capacity-bounded, and one oversized admission fails the WHOLE surface refresh.
pub fn status_json(document: &Wfc3dSnapshot, transient: &Wfc3dTransient) -> String {
    let message = if document.slots.is_empty() {
        "No slots authored yet.".to_string()
    } else if transient.contradiction {
        "Contradiction: no assignment satisfies these rules.".to_string()
    } else if transient.assignments.is_empty() {
        "Not solved yet.".to_string()
    } else {
        format!("Solved {} of {} slots.", transient.assignments.len(), document.slots.len())
    };
    serde_json::to_string(&json!({ "message": message })).unwrap_or_else(|_| "{}".into())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
