//! 🧬️ Lowpoly artifact schema — every field of the artifact with its state class.

use crate::LOWPOLY_PAINT_TEXTURE_SIZE;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::MeshData;
use semio_framework_3d::mesh::HalfedgeMesh;

//#region 🔖️Artifact
/// 🧬️ lowpoly document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolyArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub objects: Vec<LowpolyObject>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for LowpolyArtifact {
    fn default() -> Self {
        Self { schema: crate::LOWPOLY_DOCUMENT_SCHEMA.into(), objects: Vec::new() }
    }
}

impl LowpolyArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::LowpolySnapshot {
        crate::LowpolySnapshot { schema: self.schema.clone(), objects: self.objects.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::LowpolySnapshot) -> Self {
        Self { schema: snapshot.schema, objects: snapshot.objects }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::LowpolySnapshot) {
        self.schema = snapshot.schema;
        self.objects = snapshot.objects;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.lowpoly.lowpoly` — twenty handcrafted schema leaves.
pub fn lowpoly_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.lowpoly.lowpoly",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️DocumentHelpers
pub const LOWPOLY_DEFAULT_EXAMPLE_ID: &str = "hexagonal-cut-concrete-forest-left";

pub const LOWPOLY_DEFAULT_EXAMPLE_LABEL: &str = "Hexagonal Cut Concrete Forest Left";

/// 🔧️ Shared by the app's compute session and the `edit-paint-layer`/`insert-paint-layer` mutation
/// leaves — a mutable lookup of an object by id within a projection. Relocated from `⚙️engine`.
pub fn object_mut<'a>(projection: &'a mut crate::LowpolySnapshot, object_id: &str) -> Option<&'a mut LowpolyObject> {
    projection.objects.iter_mut().find(|object| object.id == object_id)
}

/// 🔧️ Shared by the app's compute session and `edit-paint-layer`'s `↩️inverse` leaf (which reads the
/// currently-stored bytes at each run's offset to compute the undo runs). Relocated from `⚙️engine`.
pub fn layer_pixels_at<'a>(projection: &'a crate::LowpolySnapshot, object_id: &str, layer_index: usize) -> Option<&'a [u8]> {
    projection.objects.iter().find(|object| object.id == object_id).and_then(|object| object.paint_layers.get(layer_index)).map(|layer| layer.pixels.as_slice())
}

/// 🔎 Returns whether `s.lowpoly.lowpoly` is present in the process-local schema registry. Relocated
/// from `⚙️engine` alongside `default_snapshot` (same rule; mirrors `s.space.home`'s identical move).
pub fn artifact_schema_registered() -> bool {
    ::semio_framework_schema_registry::artifact_schema_descriptor_registered("s.lowpoly.lowpoly")
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️MediaConversion
/// 🧵️ Builds a `MeshData` transfer payload from a raw tessellation-transfer `DslValue` (as
/// produced by the app's `LowpolyDocument::tessellate_transfer_json`), attaching a composited paint
/// texture when one is supplied. Shared by the app's live 3D scene builder and media export. Relocated
/// from `⚙️engine/🧵️media` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a pure
/// `Value` → `MeshData` conversion, not engine behaviour. Every field read routes through
/// `semio_framework_value::FromValue` directly — no `serde_json` bridge anywhere in this call chain.
pub fn mesh_data_from_transfer(transfer: &semio_framework_value::DslValue, paint_texture: Option<String>) -> MeshData {
    let read_f32 = |key: &str| -> Vec<f32> { transfer.get(key).and_then(|value| semio_framework_value::FromValue::from_value(value.clone()).ok()).unwrap_or_default() };
    let read_u32 = |key: &str| -> Vec<u32> { transfer.get(key).and_then(|value| semio_framework_value::FromValue::from_value(value.clone()).ok()).unwrap_or_default() };
    let read_u8 = |key: &str| -> Vec<u8> { transfer.get(key).and_then(|value| semio_framework_value::FromValue::from_value(value.clone()).ok()).unwrap_or_default() };
    MeshData {
        positions: read_f32("positions"),
        normals: read_f32("normals"),
        indices: read_u32("indices"),
        uvs: read_f32("uvs"),
        face_ids: read_u32("faceIds"),
        vertex_ids: read_u32("vertexIds"),
        edge_positions: read_f32("edgePositions"),
        edge_ids: read_u32("edgeIds"),
        edge_uvs: read_f32("edgeUvs"),
        edge_is_seam: read_u8("edgeIsSeam"),
        paint_texture_base64: paint_texture,
        ..MeshData::default()
    }
}

/// 🧱️ Builds a typed managed mesh document directly from a decoded triangle mesh.
pub fn lowpoly_snapshot_from_mesh(mesh: &MeshData) -> Result<crate::LowpolySnapshot, String> {
    let halfedge = HalfedgeMesh::from_indexed_triangles(&mesh.positions, &mesh.indices).map_err(|error| format!("{error:?}"))?;
    let state = crate::LowpolyMeshState::from_mesh(halfedge);
    let child = crate::managed_mesh_child_handle("obj-1", &state);
    Ok(crate::LowpolySnapshot { schema: crate::LOWPOLY_DOCUMENT_SCHEMA.into(), objects: vec![crate::LowpolyObject {
        id: "obj-1".into(), name: "Imported Mesh".into(), transform: crate::LowpolyTransform::default(), smooth_shading: false,
        mesh: Some(child), paint_layers: vec![crate::LowpolyPaintLayer::new("Base")], mesh_content: String::new(), mesh_state: Some(state),
    }] })
}

/// 🧬️ Projects a mesh document into its intrinsic value contract.
pub fn mesh_document_value(mesh: &MeshData) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::object([("schema".to_string(), semio_framework_value::DslValue::String("mesh.document".to_string())), ("mesh".to_string(), semio_framework_value::ToValue::to_value(mesh))])
}

/// 🧩️ Binds a nonempty triangle mesh from the intrinsic document value.
pub fn mesh_from_document_value(document: &semio_framework_value::DslValue) -> Result<MeshData, String> {
    let value = document.get("mesh").ok_or_else(|| "mesh document requires its typed mesh field".to_string())?;
    let mesh: MeshData = semio_framework_value::FromValue::from_value(value.clone()).map_err(|error| error.to_string())?;
    if mesh.positions.is_empty() || mesh.indices.is_empty() { return Err("mesh document requires nonempty positions and indices".into()); }
    Ok(mesh)
}
//#endregion 🔖️MediaConversion

//#region 🔖️PixelCompute
/// 🎨️ Alpha-composites an object's paint layers into one RGBA buffer (bottom to top). Relocated
/// from `⚙️engine/🎨️paint` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a pure
/// pixel-buffer algorithm, not engine behaviour.
pub fn composite_layer_pixels(layers: &[LowpolyPaintLayer]) -> Vec<u8> {
    let mut out = vec![0u8; LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4];
    for layer in layers.iter() {
        if !layer.visible {
            continue;
        }
        let default_pixels;
        let pixels = if layer.pixels.is_empty() {
            default_pixels = crate::empty_paint_pixels();
            default_pixels.as_slice()
        } else {
            layer.pixels.as_slice()
        };
        let opacity = layer.opacity.clamp(0.0, 1.0);
        for (dst, src) in out.chunks_mut(4).zip(pixels.chunks(4)) {
            let sa = (src.get(3).copied().unwrap_or(255) as f32 / 255.0) * opacity;
            let da = dst[3] as f32 / 255.0;
            let out_a = sa + da * (1.0 - sa);
            if out_a < 1e-6 {
                continue;
            }
            for (c, dst_c) in dst.iter_mut().enumerate().take(3) {
                let sc = src.get(c).copied().unwrap_or(0) as f32 / 255.0;
                let dc = *dst_c as f32 / 255.0;
                *dst_c = ((sc * sa + dc * da * (1.0 - sa)) / out_a * 255.0).round().clamp(0.0, 255.0) as u8;
            }
            dst[3] = (out_a * 255.0).round().clamp(0.0, 255.0) as u8;
        }
    }
    out
}

/// 🖌️ Stamps a soft round brush (or eraser) into a raw RGBA buffer of the paint texture's size in place. Shared by
/// the app's compute session and the `apply-paint-stroke` leaf. Relocated from `⚙️engine/🎨️paint`.
#[allow(clippy::too_many_arguments, reason = "one brush stamp per call site; a params struct would only move the same 8 fields around for this single leaf fn")]
pub fn stamp_brush(pixels: &mut [u8], u: f32, v: f32, radius: f32, color: [u8; 4], hardness: f32, opacity: f32, eraser: bool) {
    stamp_brush_on(pixels, LOWPOLY_PAINT_TEXTURE_SIZE, u, v, radius, color, hardness, opacity, eraser);
}

/// 🖌️ [`stamp_brush`] on a square RGBA buffer `side` pixels wide: the dab lands at UV `(u, v)` (v up), every pixel
/// within `radius` gets the brush colour and gains `falloff · opacity` alpha (an eraser loses it instead), the falloff
/// running from `1` at the centre to `hardness` at the rim.
#[allow(clippy::too_many_arguments, reason = "one brush stamp per call site; a params struct would only move the same 9 fields around for this single leaf fn")]
pub fn stamp_brush_on(pixels: &mut [u8], side: usize, u: f32, v: f32, radius: f32, color: [u8; 4], hardness: f32, opacity: f32, eraser: bool) {
    let size = side as f32;
    let cx = (u.clamp(0.0, 1.0) * (size - 1.0)).round() as i32;
    let cy = ((1.0 - v.clamp(0.0, 1.0)) * (size - 1.0)).round() as i32;
    let r = radius.max(0.5);
    let r_i = r.ceil() as i32;
    let hard = hardness.clamp(0.0, 1.0);
    let alpha_scale = opacity.clamp(0.0, 1.0);
    for y in (cy - r_i)..=(cy + r_i) {
        for x in (cx - r_i)..=(cx + r_i) {
            if x < 0 || y < 0 || x >= size as i32 || y >= size as i32 {
                continue;
            }
            let dx = x as f32 - cx as f32;
            let dy = y as f32 - cy as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > r {
                continue;
            }
            let t = 1.0 - dist / r;
            let falloff = hard + (1.0 - hard) * t;
            let stamp = (falloff * alpha_scale * 255.0).round().clamp(0.0, 255.0) as u8;
            let offset = (y as usize * side + x as usize) * 4;
            if eraser {
                let current = pixels[offset + 3];
                pixels[offset + 3] = current.saturating_sub(stamp);
            } else {
                pixels[offset..(3 + offset)].copy_from_slice(&color[..3]);
                let current = pixels[offset + 3];
                pixels[offset + 3] = current.saturating_add(stamp);
            }
        }
    }
}

/// 🪣️ Flood-fills a contiguous same-color region of a raw RGBA buffer in place. Relocated from
/// `⚙️engine/🎨️paint`.
pub fn flood_fill(pixels: &mut [u8], u: f32, v: f32, color: [u8; 4]) {
    let size = LOWPOLY_PAINT_TEXTURE_SIZE;
    let sx = ((u.clamp(0.0, 1.0) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let sy = (((1.0 - v.clamp(0.0, 1.0)) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let start = (sy * size + sx) * 4;
    let target = [pixels[start], pixels[start + 1], pixels[start + 2], pixels[start + 3]];
    let mut stack = vec![(sx, sy)];
    let mut visited = vec![false; size * size];
    while let Some((x, y)) = stack.pop() {
        let pi = y * size + x;
        if visited[pi] {
            continue;
        }
        visited[pi] = true;
        let offset = pi * 4;
        let pixel = [pixels[offset], pixels[offset + 1], pixels[offset + 2], pixels[offset + 3]];
        if pixel != target {
            continue;
        }
        pixels[offset..(4 + offset)].copy_from_slice(&color);
        if x > 0 {
            stack.push((x - 1, y));
        }
        if x + 1 < size {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y + 1 < size {
            stack.push((x, y + 1));
        }
    }
}

/// 💧️ Reads one RGBA sample from a composited buffer at UV. Relocated from `⚙️engine/🎨️paint`.
pub fn sample_pixel_from(composite: &[u8], u: f32, v: f32) -> [u8; 4] {
    let size = LOWPOLY_PAINT_TEXTURE_SIZE;
    let x = ((u.clamp(0.0, 1.0) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let y = (((1.0 - v.clamp(0.0, 1.0)) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let offset = (y * size + x) * 4;
    [composite[offset], composite[offset + 1], composite[offset + 2], composite[offset + 3]]
}

/// 🧮️ Coalesces a `before`/`after` layer-buffer pair into the minimal contiguous pixel runs
/// (`(offset, bytes)`) that turn `before` into `after`; the seam where a mutated scratch buffer becomes
/// a `PaintStroke` operation. Returns raw `(offset, bytes)` tuples — `op` wraps each into its own
/// `PixelRun`. Relocated from `⚙️engine/🎨️paint`.
/// 🩸 Runs are formed per RGBA PIXEL, never per byte: a fill of (255,64,64,255) over opaque white leaves
/// bytes 0 and 3 of every pixel equal, and a byte-wise diff then minted one two-byte run PER PIXEL —
/// 65 536 runs for one 256² layer against the 4 096-run retained envelope (`paintFill` refused with
/// `exceeds its fixed run envelope`, 2026-09-18). A run now spans consecutive changed pixels whole.
pub fn pixel_runs_from_diff(before: &[u8], after: &[u8]) -> Vec<(u32, Vec<u8>)> {
    let mut runs = Vec::new();
    let pixels = before.len().min(after.len()) / 4;
    let changed = |pixel: usize| before[pixel * 4..pixel * 4 + 4] != after[pixel * 4..pixel * 4 + 4];
    let mut pixel = 0;
    while pixel < pixels {
        if !changed(pixel) {
            pixel += 1;
            continue;
        }
        let start = pixel;
        while pixel < pixels && changed(pixel) {
            pixel += 1;
        }
        runs.push(((start * 4) as u32, after[start * 4..pixel * 4].to_vec()));
    }
    runs
}
//#endregion 🔖️PixelCompute

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ExportConcreteForestMeshTests
#[cfg(all(test, feature = "cad-fixtures"))]
#[path = "🧪️tests/🔬️export-concrete-forest-mesh/🦀️.rs"]
mod export_concrete_forest_mesh_tests;
//#endregion 🔖️ExportConcreteForestMeshTests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::LowpolyObject;
pub use crate::LowpolyPaintLayer;
pub use crate::LowpolySelection;
//#endregion 🔁️Re-exports

/// 🩸 Contiguous RGBA octets shared by mutations and sparse deltas.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PixelRun {
    pub offset: u32,
    #[value(with = "semio_framework_value::bytes")]
    pub bytes: Vec<u8>,
}
