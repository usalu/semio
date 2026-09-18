//! 🌐️ Scene projection internals — the pure document→`World3dScene` payload functions BOTH surfaces
//! read (`✏️editor`'s grid and preview windows, `👁️viewer`'s preview window). They live in the schema
//! tree, not under either surface, so the viewer never imports through the mutation-capable editor
//! (`policyViewerPurityBreaches`) and neither surface owns a second copy of the geometry.
//!
//! 🧊️ Every payload is the `World3dScene` wire shape: a SMALL `meshes_json` catalogue of distinct
//! geometry (`{id, data}`) and a LARGE `instances_json` of `{id, meshId, position, rotation, scale,
//! label}` records — one instanced draw call per distinct mesh, never one mesh per cell.

use crate::schema::inferences::Grid3dAssignment;
use crate::schema::snapshot::{axis_offset, axis_size, cell_key, Grid3dColor, Grid3dSnapshot, Grid3dTile, Grid3dTileMedia};
use dsl::json;

//#region 🔖️Ids
/// 🧊️ The mesh id of the neutral, unpinned grid cell box.
pub const GRID_CELL_MESH: &str = "cell";
/// 🚫️ The mesh id of a masked-out cell box.
pub const GRID_MASKED_MESH: &str = "cell-masked";
/// 📌️ The mesh id prefix of a pinned cell box, tinted with the pinned tile's own colour.
pub const GRID_PINNED_MESH_PREFIX: &str = "cell-pin:";
/// 🗿️ The mesh id prefix of one tile's preview geometry.
pub const TILE_MESH_PREFIX: &str = "tile:";
//#endregion 🔖️Ids

//#region 📐️Geometry
/// 📐️ Cell `(x, y, z)`'s lower world corner on a non-uniform grid — the cumulative sum of every cell
/// size before it on each axis.
pub fn cell_origin(snapshot: &Grid3dSnapshot, x: u32, y: u32, z: u32) -> [f64; 3] {
    [axis_offset(&snapshot.cell_sizes_x, x as usize), axis_offset(&snapshot.cell_sizes_y, y as usize), axis_offset(&snapshot.cell_sizes_z, z as usize)]
}

/// 📐️ Cell `(x, y, z)`'s own box extent.
pub fn cell_extent(snapshot: &Grid3dSnapshot, x: u32, y: u32, z: u32) -> [f64; 3] {
    [axis_size(&snapshot.cell_sizes_x, x as usize), axis_size(&snapshot.cell_sizes_y, y as usize), axis_size(&snapshot.cell_sizes_z, z as usize)]
}

/// 📐️ The whole grid's world extent, for camera framing.
pub fn grid_extent(snapshot: &Grid3dSnapshot) -> [f64; 3] {
    [
        (0..snapshot.width).map(|index| axis_size(&snapshot.cell_sizes_x, index as usize)).sum(),
        (0..snapshot.height).map(|index| axis_size(&snapshot.cell_sizes_y, index as usize)).sum(),
        (0..snapshot.depth).map(|index| axis_size(&snapshot.cell_sizes_z, index as usize)).sum(),
    ]
}
//#endregion 📐️Geometry

//#region 🧊️BoxMesh
const BOX_CORNERS: [[f32; 3]; 8] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
const BOX_TRIANGLES: [u32; 36] = [0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3, 1, 2, 6, 1, 6, 5];
const BOX_EDGES: [[usize; 2]; 12] = [[0, 1], [1, 2], [2, 3], [3, 0], [4, 5], [5, 6], [6, 7], [7, 4], [0, 4], [1, 5], [2, 6], [3, 7]];

/// 🧊️ The unit box in tile space `0..1`, tinted with one RGBA colour — the geometry every grid cell
/// and every media-less tile instances. `edge_positions` carries the twelve wireframe edges, so a
/// window that shows a translucent cage costs no second mesh.
pub fn unit_box_mesh(color: [f32; 4]) -> semio_framework::MeshData {
    let mut mesh = semio_framework::MeshData::default();
    for corner in BOX_CORNERS {
        mesh.positions.extend_from_slice(&corner);
        let normal = [corner[0] * 2.0 - 1.0, corner[1] * 2.0 - 1.0, corner[2] * 2.0 - 1.0];
        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt().max(f32::EPSILON);
        mesh.normals.extend_from_slice(&[normal[0] / length, normal[1] / length, normal[2] / length]);
        mesh.colors.extend_from_slice(&color);
    }
    mesh.indices.extend_from_slice(&BOX_TRIANGLES);
    for [from, to] in BOX_EDGES {
        mesh.edge_positions.extend_from_slice(&BOX_CORNERS[from]);
        mesh.edge_positions.extend_from_slice(&BOX_CORNERS[to]);
    }
    mesh
}

/// 🎨️ An authored colour as the renderer's straight-alpha float RGBA, clamped into `0..1`.
pub fn float_color(color: Option<&Grid3dColor>, fallback: [f32; 4]) -> [f32; 4] {
    match color {
        Some(color) => [color.r.min(255) as f32 / 255.0, color.g.min(255) as f32 / 255.0, color.b.min(255) as f32 / 255.0, color.a.min(255) as f32 / 255.0],
        None => fallback,
    }
}
//#endregion 🧊️BoxMesh

//#region 🗿️TileMesh
/// 🗿️ One tile's preview geometry. An INLINE mesh becomes real triangles; a `MeshChild` handle whose
/// child the host has not hydrated falls back to a unit-box placeholder rather than failing the
/// surface, which is the same "fail soft" rule raster's asset children follow.
pub fn tile_mesh(tile: &Grid3dTile) -> semio_framework::MeshData {
    match &tile.media {
        Grid3dTileMedia::Mesh { mesh } if !mesh.indices.is_empty() && mesh.positions.len() >= 9 => {
            let color = float_color(mesh.color.as_ref(), [0.72, 0.74, 0.78, 1.0]);
            let mut data = semio_framework::MeshData { positions: mesh.positions.iter().map(|value| *value as f32).collect(), indices: mesh.indices.clone(), ..Default::default() };
            let vertices = data.positions.len() / 3;
            for _ in 0..vertices {
                data.normals.extend_from_slice(&[0.0, 0.0, 1.0]);
                data.colors.extend_from_slice(&color);
            }
            data
        }
        Grid3dTileMedia::Mesh { mesh } => unit_box_mesh(float_color(mesh.color.as_ref(), [0.72, 0.74, 0.78, 1.0])),
        Grid3dTileMedia::MeshChild { .. } => unit_box_mesh([0.62, 0.66, 0.72, 1.0]),
    }
}

/// 🎨️ The tint a tile paints its pinned grid cells with.
pub fn tile_tint(tile: &Grid3dTile) -> [f32; 4] {
    match &tile.media {
        Grid3dTileMedia::Mesh { mesh } => float_color(mesh.color.as_ref(), [0.35, 0.62, 0.92, 0.55]),
        Grid3dTileMedia::MeshChild { .. } => [0.35, 0.62, 0.92, 0.55],
    }
}

fn mesh_entry(id: String, data: semio_framework::MeshData) -> json::Value {
    json::object([("id".to_string(), json::Value::String(id)), ("data".to_string(), json::Value::from(data))])
}

fn vector3(values: [f64; 3]) -> json::Value {
    json::Value::Array(values.into_iter().map(json::Value::from).collect())
}

fn instance_record(id: String, mesh_id: String, position: [f64; 3], scale: [f64; 3], label: String) -> json::Value {
    json::object([
        ("id".to_string(), json::Value::String(id)),
        ("meshId".to_string(), json::Value::String(mesh_id)),
        ("position".to_string(), vector3(position)),
        ("rotation".to_string(), json::Value::Array(vec![json::Value::from(0.0), json::Value::from(0.0), json::Value::from(0.0), json::Value::from(1.0)])),
        ("scale".to_string(), vector3(scale)),
        ("label".to_string(), json::Value::String(label)),
    ])
}
//#endregion 🗿️TileMesh

//#region ▦️GridWindow
/// ▦️ The grid window's mesh catalogue: the neutral cell cage, the masked-cell cage, and one tinted
/// cage per tile that some cell is pinned to. Never one mesh per cell.
pub fn grid_meshes_json(snapshot: &Grid3dSnapshot) -> String {
    let mut meshes = vec![mesh_entry(GRID_CELL_MESH.to_string(), unit_box_mesh([0.58, 0.62, 0.68, 0.18])), mesh_entry(GRID_MASKED_MESH.to_string(), unit_box_mesh([0.16, 0.17, 0.2, 0.35]))];
    for tile in &snapshot.tiles {
        if snapshot.pinned.iter().any(|cell| cell.tile_id == tile.id) {
            meshes.push(mesh_entry(format!("{GRID_PINNED_MESH_PREFIX}{}", tile.id), unit_box_mesh(tile_tint(tile))));
        }
    }
    json::to_string(&json::Value::Array(meshes))
}

/// ▦️ One instance per grid cell, placed at its own non-uniform origin and scaled to its own box.
/// The instance id IS the cell key, so a pick comes back as `x:y:z` and the editor's pin/mask
/// commands need no second lookup table.
pub fn grid_instances_json(snapshot: &Grid3dSnapshot) -> String {
    let mut instances = Vec::with_capacity(snapshot.width as usize * snapshot.height as usize * snapshot.depth as usize);
    for z in 0..snapshot.depth {
        for y in 0..snapshot.height {
            for x in 0..snapshot.width {
                let key = cell_key(x, y, z);
                let pinned = snapshot.pinned.iter().find(|cell| (cell.x, cell.y, cell.z) == (x, y, z));
                let masked = snapshot.masked.iter().any(|cell| (cell.x, cell.y, cell.z) == (x, y, z));
                let mesh_id = match (&pinned, masked) {
                    (_, true) => GRID_MASKED_MESH.to_string(),
                    (Some(cell), false) => format!("{GRID_PINNED_MESH_PREFIX}{}", cell.tile_id),
                    (None, false) => GRID_CELL_MESH.to_string(),
                };
                let label = match (&pinned, masked) {
                    (_, true) => format!("{key} masked"),
                    (Some(cell), false) => format!("{key} {}", cell.tile_id),
                    (None, false) => key.clone(),
                };
                instances.push(instance_record(key, mesh_id, cell_origin(snapshot, x, y, z), cell_extent(snapshot, x, y, z), label));
            }
        }
    }
    json::to_string(&json::Value::Array(instances))
}
//#endregion ▦️GridWindow

//#region 👁️PreviewWindow
/// 👁️ The preview window's mesh catalogue: exactly one entry per authored tile, so a solved grid of
/// hundreds of cells costs one instanced draw call per distinct tile.
pub fn preview_meshes_json(snapshot: &Grid3dSnapshot) -> String {
    let meshes: Vec<json::Value> = snapshot.tiles.iter().map(|tile| mesh_entry(format!("{TILE_MESH_PREFIX}{}", tile.id), tile_mesh(tile))).collect();
    json::to_string(&json::Value::Array(meshes))
}

/// 👁️ One instance per SOLVED cell, scaled into that cell's own box. `assignments` is the inference
/// commit's `[x, y, z, tileId]` list — never persisted state.
pub fn preview_instances_json(snapshot: &Grid3dSnapshot, assignments: &[Grid3dAssignment]) -> String {
    let instances: Vec<json::Value> = assignments.iter().filter(|row| in_grid(snapshot, row)).map(|row| preview_instance(snapshot, row)).collect();
    json::to_string(&json::Value::Array(instances))
}

fn in_grid(snapshot: &Grid3dSnapshot, row: &Grid3dAssignment) -> bool {
    row.x < snapshot.width && row.y < snapshot.height && row.z < snapshot.depth
}

fn preview_instance(snapshot: &Grid3dSnapshot, row: &Grid3dAssignment) -> json::Value {
    let key = cell_key(row.x, row.y, row.z);
    instance_record(key.clone(), format!("{TILE_MESH_PREFIX}{}", row.tile_id), cell_origin(snapshot, row.x, row.y, row.z), cell_extent(snapshot, row.x, row.y, row.z), format!("{key} {}", row.tile_id))
}

/// 🚚️ The incremental companion of [`preview_instances_json`]: `{base, revision, count, changed,
/// removed}`, where `changed` carries whole records so an applying consumer never diffs fields.
/// `instances_json` stays AUTHORITATIVE on every publication; a consumer not at `base` reads it.
pub fn preview_instances_delta_json(snapshot: &Grid3dSnapshot, previous: &[Grid3dAssignment], next: &[Grid3dAssignment], revision: u64) -> String {
    let changed: Vec<json::Value> = next.iter().filter(|row| in_grid(snapshot, row) && !previous.contains(row)).map(|row| preview_instance(snapshot, row)).collect();
    let removed: Vec<json::Value> = previous
        .iter()
        .filter(|row| !next.iter().any(|candidate| (candidate.x, candidate.y, candidate.z) == (row.x, row.y, row.z)))
        .map(|row| json::Value::String(cell_key(row.x, row.y, row.z)))
        .collect();
    json::to_string(&json::object([
        ("base".to_string(), json::Value::from(revision.saturating_sub(1))),
        ("revision".to_string(), json::Value::from(revision)),
        ("count".to_string(), json::Value::from(next.len() as u64)),
        ("changed".to_string(), json::Value::Array(changed)),
        ("removed".to_string(), json::Value::Array(removed)),
    ]))
}
//#endregion 👁️PreviewWindow

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
