//! 🧬️ Lowpoly artifact schema — every field of the artifact with its state class.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolyPaintLayer, LowpolySelection, LOWPOLY_PAINT_TEXTURE_SIZE};
use schema::ArtifactSchema;
use semio_framework_3d::mesh::HalfedgeMesh;
use semio_framework_plugin::MeshData;
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Artifact
/// 🧬️ Full lowpoly artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolyArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub objects: Vec<LowpolyObject>,
    #[state(presence)]
    pub active_object_id: Option<String>,
    #[state(presence)]
    pub selection: LowpolySelection,
    #[state(presence)]
    pub selected_object_ids: Vec<String>,
    #[state(presence)]
    pub paint_utility: String,
    #[state(presence)]
    pub active_paint_layer: u32,
    #[state(presence)]
    pub active_utility_id: String,
    #[state(config)]
    pub show_edges: bool,
    #[state(config)]
    pub sun_enabled: bool,
    #[state(config)]
    pub sun_azimuth: f64,
    #[state(config)]
    pub sun_elevation: f64,
    #[state(config)]
    pub sun_intensity: f64,
    #[state(config)]
    pub sun_color: String,
    #[state(config)]
    pub world_camera_position_x: f64,
    #[state(config)]
    pub world_camera_position_y: f64,
    #[state(config)]
    pub world_camera_position_z: f64,
    #[state(config)]
    pub world_camera_target_x: f64,
    #[state(config)]
    pub world_camera_target_y: f64,
    #[state(config)]
    pub world_camera_target_z: f64,
    #[state(config)]
    pub world_camera_fov: f64,
    #[state(config)]
    pub utility_params_json: String,
    #[state(config)]
    pub paint_color_r: u32,
    #[state(config)]
    pub paint_color_g: u32,
    #[state(config)]
    pub paint_color_b: u32,
    #[state(config)]
    pub paint_color_a: u32,
    #[state(config)]
    pub selection_method: String,
    #[state(config)]
    pub selection_mode_default: String,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub locale: String,
    #[state(artifact)]
    pub hovered_object_id: Option<String>,
    #[state(artifact)]
    pub hovered_target_object_id: Option<String>,
    #[state(artifact)]
    pub hovered_target_mode: Option<String>,
    #[state(artifact)]
    pub hovered_target_id: Option<u32>,
    #[state(artifact)]
    pub stroke_drag_active: bool,
    #[state(artifact)]
    pub transform_drag_active: bool,
    #[state(artifact)]
    pub preview_seq: i64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for LowpolyArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA.into(),
            objects: Vec::new(),
            active_object_id: None,
            selection: LowpolySelection::default(),
            selected_object_ids: Vec::new(),
            paint_utility: "brush".into(),
            active_paint_layer: 0,
            active_utility_id: "move".into(),
            show_edges: true,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            world_camera_position_x: 18.0,
            world_camera_position_y: -18.0,
            world_camera_position_z: 12.0,
            world_camera_target_x: 0.0,
            world_camera_target_y: 0.0,
            world_camera_target_z: 0.0,
            world_camera_fov: 45.0,
            utility_params_json: String::new(),
            paint_color_r: 255,
            paint_color_g: 64,
            paint_color_b: 64,
            paint_color_a: 255,
            selection_method: "rectangle".into(),
            selection_mode_default: "default".into(),
            engagement_input: String::new(),
            locale: "en-US".into(),
            hovered_object_id: None,
            hovered_target_object_id: None,
            hovered_target_mode: None,
            hovered_target_id: None,
            stroke_drag_active: false,
            transform_drag_active: false,
            preview_seq: 0,
        }
    }
}

impl LowpolyArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::lowpoly::LowpolySnapshot {
        crate::artifacts::lowpoly::LowpolySnapshot { schema: self.schema.clone(), objects: self.objects.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::lowpoly::LowpolySnapshot) -> Self {
        Self { schema: snapshot.schema, objects: snapshot.objects, ..Self::default() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::lowpoly::LowpolySnapshot) {
        self.schema = snapshot.schema;
        self.objects = snapshot.objects;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.lowpoly.lowpoly` — twenty handcrafted schema leaves.
pub fn lowpoly_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.lowpoly.lowpoly",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
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
/// 🧺 One caller-owned default document pair. The parent snapshot owns only the exact
/// `ArtifactChild` handle while the app session owns its matching mesh payload.
pub struct LowpolyOwnedDefaultDocument {
    pub snapshot: crate::artifacts::lowpoly::LowpolySnapshot,
    pub mesh_workspace: std::collections::HashMap<String, String>,
}

/// 🧱️ Builds the deterministic primitive without UV repacking, so every owner gets a fresh matching
/// handle/payload pair and no process-global child payload cache is required.
pub fn default_owned_document() -> LowpolyOwnedDefaultDocument {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim");
    let mesh_json = mesh.to_json().expect("mesh json");
    let snapshot = crate::artifacts::lowpoly::snapshot_from_mesh_json(&mesh_json, "obj-1", "Unit Box");
    let mesh_workspace = std::collections::HashMap::from([("obj-1".to_string(), mesh_json)]);
    LowpolyOwnedDefaultDocument { snapshot, mesh_workspace }
}

/// 🎞️ Default document projection used by tests and the play app.
pub fn default_snapshot() -> crate::artifacts::lowpoly::LowpolySnapshot {
    default_owned_document().snapshot
}

/// 🕸️ Fresh app-owned companion payload for `default_snapshot()`'s exact child handle.
pub fn default_mesh_workspace() -> std::collections::HashMap<String, String> {
    default_owned_document().mesh_workspace
}

/// 🔧️ Shared by the app's compute session and the `edit-paint-layer`/`insert-paint-layer` mutation
/// leaves — a mutable lookup of an object by id within a projection. Relocated from `⚙️engine`.
pub fn object_mut<'a>(projection: &'a mut crate::artifacts::lowpoly::LowpolySnapshot, object_id: &str) -> Option<&'a mut LowpolyObject> {
    projection.objects.iter_mut().find(|object| object.id == object_id)
}

/// 🔧️ Shared by the app's compute session and `edit-paint-layer`'s `↩️inverse` leaf (which reads the
/// currently-stored bytes at each run's offset to compute the undo runs). Relocated from `⚙️engine`.
pub fn layer_pixels_at<'a>(projection: &'a crate::artifacts::lowpoly::LowpolySnapshot, object_id: &str, layer_index: usize) -> Option<&'a [u8]> {
    projection.objects.iter().find(|object| object.id == object_id).and_then(|object| object.paint_layers.get(layer_index)).map(|layer| layer.pixels.as_slice())
}

/// 🔎 Returns whether `s.lowpoly.lowpoly` is present in the process-local schema registry. Relocated
/// from `⚙️engine` alongside `default_snapshot` (same rule; mirrors `s.space.home`'s identical move).
pub fn artifact_schema_registered() -> bool {
    ::schema::artifact_schema_descriptor_registered("s.lowpoly.lowpoly")
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️MediaConversion
/// @emoji 🧵️ Builds a `MeshData` transfer payload from a raw tessellation-transfer JSON value (as
/// produced by the app's `LowpolyDocument::tessellate_transfer_json`), attaching a composited paint
/// texture when one is supplied. Shared by the app's live 3D scene builder and media export. Relocated
/// from `⚙️engine/🧵️media` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a pure
/// `Value` → `MeshData` conversion, not engine behaviour.
pub fn mesh_data_from_transfer(transfer: &Value, paint_texture: Option<String>) -> MeshData {
    let read_f32 = |key: &str| -> Vec<f32> { transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default() };
    let read_u32 = |key: &str| -> Vec<u32> { transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default() };
    let read_u8 = |key: &str| -> Vec<u8> { transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default() };
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

/// 🔺️ Rebuilds a fresh single-object lowpoly projection from a DWG-imported mesh. Relocated from
/// `⚙️engine/🧵️media`. Routes through `dsl::ToValue` (not `serde_json::to_value` on `LowpolySnapshot`
/// directly) since the snapshot transitively carries `LowpolyObject.mesh:
/// Option<store::ArtifactChild<SemioMeshSnapshot>>`, whose `Serialize` is `#[cfg(test)]`-only
/// (ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`) — `dsl::ToValue`
/// stays available unconditionally, and the `DslValue`→`serde_json::Value` bridge (`🌱️value/🦀️.rs`)
/// yields the identical JSON shape.
pub fn lowpoly_document_from_mesh(mesh: &MeshData) -> Result<Value, String> {
    let halfedge = HalfedgeMesh::from_indexed_triangles(&mesh.positions, &mesh.indices).map_err(|err| format!("{err:?}"))?;
    let mesh_json = halfedge.to_json().map_err(|err| format!("{err:?}"))?;
    let snapshot = crate::artifacts::lowpoly::snapshot_from_mesh_json(&mesh_json, "obj-1", "Imported Mesh");
    Ok(dsl::ToValue::to_value(&snapshot).into())
}

/// 🧊️ Minimal document wrapper for `3d.mesh` resources — no dedicated schema exists yet. Relocated
/// from `⚙️engine/🧵️media`. `MeshData` implements `dsl::ToValue` first-party (hand-written in
/// `🔺️mesh-engine/🦀️.rs`, since `serde`'s `Serialize` on it is `#[cfg(test)]`-only per the same
/// ticket), so this bridges through that instead of `serde_json::to_value(mesh)`.
pub fn mesh_document_from_mesh(mesh: &MeshData) -> Result<Value, String> {
    let mesh_value: Value = dsl::ToValue::to_value(mesh).into();
    Ok(serde_json::json!({ "schema": "mesh.document", "mesh": mesh_value }))
}

/// 🔺️ Relocated from `⚙️engine/🧵️media`. Mirrors `mesh_document_from_mesh`'s `dsl::ToValue` bridge in
/// reverse (`dsl::FromValue`), since `MeshData: Deserialize` is likewise `#[cfg(test)]`-only.
pub fn mesh_from_mesh_document(doc: &Value) -> Result<MeshData, String> {
    doc.get("mesh")
        .and_then(|value| dsl::FromValue::from_value(dsl::DslValue::from(value.clone())).ok())
        .filter(|mesh: &MeshData| !mesh.positions.is_empty() && !mesh.indices.is_empty())
        .map_or_else(|| Ok(semio_framework_plugin::mesh_from_kind("box")), Ok)
}
//#endregion 🔖️MediaConversion

//#region 🔖️PixelCompute
/// @emoji 🎨️ Alpha-composites an object's paint layers into one RGBA buffer (bottom to top). Relocated
/// from `⚙️engine/🎨️paint` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a pure
/// pixel-buffer algorithm, not engine behaviour.
pub fn composite_layer_pixels(layers: &[LowpolyPaintLayer]) -> Vec<u8> {
    let mut out = vec![0u8; LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4];
    for layer in layers.iter() {
        if !layer.visible {
            continue;
        }
        let pixels = layer.pixels.as_slice();
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

/// @emoji 🖌️ Stamps a soft round brush (or eraser) into a raw RGBA buffer in place. Shared by the
/// app's compute session and the plugin's mid-drag scratch buffer. Relocated from `⚙️engine/🎨️paint`.
#[allow(clippy::too_many_arguments, reason = "one brush stamp per call site; a params struct would only move the same 8 fields around for this single leaf fn")]
pub fn stamp_brush(pixels: &mut [u8], u: f32, v: f32, radius: f32, color: [u8; 4], hardness: f32, opacity: f32, eraser: bool) {
    let size = LOWPOLY_PAINT_TEXTURE_SIZE as f32;
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
            let offset = (y as usize * LOWPOLY_PAINT_TEXTURE_SIZE + x as usize) * 4;
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

/// @emoji 🪣️ Flood-fills a contiguous same-color region of a raw RGBA buffer in place. Relocated from
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

/// @emoji 💧️ Reads one RGBA sample from a composited buffer at UV. Relocated from `⚙️engine/🎨️paint`.
pub fn sample_pixel_from(composite: &[u8], u: f32, v: f32) -> [u8; 4] {
    let size = LOWPOLY_PAINT_TEXTURE_SIZE;
    let x = ((u.clamp(0.0, 1.0) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let y = (((1.0 - v.clamp(0.0, 1.0)) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let offset = (y * size + x) * 4;
    [composite[offset], composite[offset + 1], composite[offset + 2], composite[offset + 3]]
}

/// @emoji 🧮️ Coalesces a `before`/`after` layer-buffer pair into the minimal contiguous pixel runs
/// (`(offset, bytes)`) that turn `before` into `after`; the seam where a mutated scratch buffer becomes
/// a `PaintStroke` operation. Returns raw `(offset, bytes)` tuples — `op` wraps each into its own
/// `PixelRun`. Relocated from `⚙️engine/🎨️paint`.
pub fn pixel_runs_from_diff(before: &[u8], after: &[u8]) -> Vec<(u32, Vec<u8>)> {
    let mut runs = Vec::new();
    let len = before.len().min(after.len());
    let mut index = 0;
    while index < len {
        if before[index] == after[index] {
            index += 1;
            continue;
        }
        let start = index;
        while index < len && before[index] != after[index] {
            index += 1;
        }
        runs.push((start as u32, after[start..index].to_vec()));
    }
    runs
}
//#endregion 🔖️PixelCompute

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::lowpoly::schema::diff::LowpolyDiff;
    use crate::artifacts::lowpoly::schema::mutations::LowpolyMutation;
    use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct LowpolyBuilderConstruction {
        snapshot: LowpolySnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for LowpolyBuilderConstruction {
        type Snapshot = LowpolySnapshot;
        type Mutation = LowpolyMutation;
        type Diff = LowpolyDiff;
        fn empty() -> Self {
            Self { snapshot: LowpolySnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<LowpolySnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<LowpolySnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::MutationDiff::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <LowpolyDiff as protocol::MutationDiff<LowpolySnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::lowpoly::LowpolySnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct LowpolyParts {
        pub snapshot: Option<LowpolySnapshot>,
    }

    pub struct LowpolyAnalyzerAnalysis;

    impl ArtifactAnalysis for LowpolyAnalyzerAnalysis {
        type Parts = LowpolyParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.lowpoly", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = LowpolyParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <LowpolySnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <LowpolySnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec LowpolyBuilderFacets {
        construction: LowpolyBuilderConstruction,
        analysis: LowpolyAnalyzerAnalysis,
        composition: super::super::io::derived_composition::LowpolyComposerComposition,
    }
    builder: LowpolyBuilder,
    analyzer: LowpolyAnalyzer,
    composer: LowpolyComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::lowpoly::empty_paint_pixels;

    #[semio_framework_async_macros::async_test]
    async fn default_snapshot_has_unit_box_object() {
        let projection = default_snapshot();
        assert_eq!(projection.schema, crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA);
        assert_eq!(projection.objects.len(), 1);
        assert_eq!(projection.objects[0].id, "obj-1");
        assert_eq!(projection.objects[0].name, "Unit Box");
        assert_eq!(projection.objects[0].paint_layers.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn default_unit_box_mesh_parses_and_has_faces() {
        let projection = default_snapshot();
        let workspace = default_mesh_workspace();
        let mesh_json = workspace.get(&projection.objects[0].id).expect("workspace entry for default object");
        let mesh = HalfedgeMesh::from_json(mesh_json).expect("default mesh");
        assert!(mesh.face_count() >= 6, "unit box should expose six faces");
        assert!(mesh.vertex_count() >= 8, "unit box should expose eight vertices");
    }

    #[semio_framework_async_macros::async_test]
    async fn projection_round_trips_paint_pixels_through_base64_json() {
        let mut projection = default_snapshot();
        projection.objects[0].paint_layers[0].pixels[0] = 7;
        projection.objects[0].paint_layers[0].pixels[1] = 9;
        let json = serde_json::to_string(&projection).unwrap();
        let restored: crate::artifacts::lowpoly::LowpolySnapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, projection);
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_engine_apply_and_inverse_round_trip() {
        // 🩹 Was `protocol::ArtifactEngine`-based (that trait doesn't exist anywhere in the
        // codebase — a stale reference predating 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL — and
        // `LowpolyEngine` never exposed a `snapshot()`/apply-mutation API either) and constructed
        // the since-removed `LowpolyMutation::ObjectsPatch` bag variant. Rewritten against the real
        // `protocol::Mutation` diff/apply/inverse contract and the new `rename-object` mutation.
        use crate::artifacts::lowpoly::mutations::rename_object;
        use crate::artifacts::lowpoly::LowpolyMutation;
        use protocol::{Mutation, MutationDiff};
        let base = default_snapshot();
        let object_id = base.objects[0].id.clone();
        let mutation = LowpolyMutation::RenameObject(rename_object::RenameObject { id: object_id, new_name: "Renamed".into() });
        let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
        assert_eq!(after.objects[0].name, "Renamed");
        let inverse = mutation.inverse(&base);
        let mut state = after;
        for step in &inverse {
            state = step.diff(&base).diff().apply(&state).expect("valid mutation diff");
        }
        assert_eq!(state.objects[0].name, "Unit Box");
    }

    #[semio_framework_async_macros::async_test]
    async fn pixel_runs_from_diff_captures_only_changed_bytes() {
        let mut before = vec![0u8; 16];
        let mut after = before.clone();
        after[4] = 9;
        after[5] = 9;
        after[10] = 3;
        let runs = pixel_runs_from_diff(&before, &after);
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[0].0, 4);
        assert_eq!(runs[0].1, vec![9, 9]);
        assert_eq!(runs[1].0, 10);
        assert_eq!(runs[1].1, vec![3]);
        before[4] = 9;
        before[5] = 9;
        before[10] = 3;
        assert!(pixel_runs_from_diff(&before, &after).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn composite_layer_pixels_skips_invisible_layers() {
        let mut layer = LowpolyPaintLayer::new("Hidden");
        layer.visible = false;
        layer.pixels = vec![255, 0, 0, 255];
        let out = composite_layer_pixels(&[layer]);
        assert_eq!(&out[0..4], &[0, 0, 0, 0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn composite_layer_pixels_blends_partial_opacity_over_transparent_base() {
        let mut layer = LowpolyPaintLayer::new("Half");
        layer.opacity = 0.5;
        layer.pixels = vec![200, 100, 50, 255];
        let out = composite_layer_pixels(&[layer]);
        assert_eq!(&out[0..4], &[200, 100, 50, 128]);
    }

    #[semio_framework_async_macros::async_test]
    async fn composite_layer_pixels_blends_stacked_opaque_and_translucent_layers() {
        let base = LowpolyPaintLayer { name: "Base".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), pixels: vec![255, 0, 0, 255] };
        let top = LowpolyPaintLayer { name: "Top".into(), visible: true, opacity: 0.5, blend_mode: "normal".into(), pixels: vec![0, 0, 255, 255] };
        let out = composite_layer_pixels(&[base, top]);
        assert_eq!(&out[0..4], &[128, 0, 128, 255]);
    }

    #[semio_framework_async_macros::async_test]
    async fn stamp_brush_eraser_reduces_alpha_at_center() {
        let mut pixels = empty_paint_pixels();
        stamp_brush(&mut pixels, 0.5, 0.5, 4.0, [0, 0, 0, 0], 1.0, 1.0, true);
        let size = LOWPOLY_PAINT_TEXTURE_SIZE;
        let center = (size / 2 * size + size / 2) * 4;
        assert!(pixels[center + 3] < 255);
    }

    #[semio_framework_async_macros::async_test]
    async fn flood_fill_only_affects_contiguous_matching_region() {
        let mut pixels = empty_paint_pixels();
        let size = LOWPOLY_PAINT_TEXTURE_SIZE;
        for y in 0..10 {
            for x in 0..10 {
                let offset = (y * size + x) * 4;
                pixels[offset..offset + 4].copy_from_slice(&[0, 255, 0, 255]);
            }
        }
        flood_fill(&mut pixels, 0.99, 0.01, [255, 0, 0, 255]);
        assert_eq!(&pixels[0..4], &[0, 255, 0, 255]);
        let far_offset = (500 * size + 500) * 4;
        assert_eq!(&pixels[far_offset..far_offset + 4], &[255, 0, 0, 255]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️ExportConcreteForestMeshTests
#[cfg(all(test, feature = "cad-fixtures"))]
mod export_concrete_forest_mesh_tests {
    use cad_plugin::artifacts::cad::io::geometry_import::{objects_from_fixture_model, parse_geometry};
    use semio_framework_3d::mesh::{FaceId, HalfedgeMesh, Vec3 as MeshVec3, VertexId};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{Brep, GeometryHandle};
    use serde_json::Value;
    use std::collections::HashMap;

    /// Asserts every directed edge (by vertex id, after welding) has an opposite-winding counterpart, i.e. the
    /// mesh has no open boundary loops.
    fn assert_watertight(mesh: &HalfedgeMesh) {
        let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
        for fi in 0..mesh.face_count() {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).expect("face verts");
            let n = verts.len();
            for i in 0..n {
                *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
            }
        }
        let open: Vec<(u32, u32)> = directed.keys().copied().filter(|&(a, b)| !directed.contains_key(&(b, a))).collect();
        assert!(open.is_empty(), "mesh is not watertight: {} open boundary edges, e.g. {:?}", open.len(), &open[..open.len().min(5)]);
    }

    fn open_boundary_count(mesh: &HalfedgeMesh) -> usize {
        let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
        for fi in 0..mesh.face_count() {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).expect("face verts");
            let n = verts.len();
            for i in 0..n {
                *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
            }
        }
        directed.keys().filter(|&&(a, b)| !directed.contains_key(&(b, a))).count()
    }

    /// Spurious `fill_holes` caps on this solid spanned the open gap between vertical supports: large X
    /// extent *and* large Z extent on one face. Real CAD faces are either horizontal slabs (small Δz) or
    /// vertical support sides (small Δx).
    fn assert_no_spanning_face_across_support_gap(mesh: &HalfedgeMesh) {
        for fi in 0..mesh.face_count() {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).expect("face verts");
            let mut min_x = f32::MAX;
            let mut max_x = f32::MIN;
            let mut min_z = f32::MAX;
            let mut max_z = f32::MIN;
            for vid in verts {
                let p = mesh.vertex_position(vid).expect("vertex");
                min_x = min_x.min(p.x());
                max_x = max_x.max(p.x());
                min_z = min_z.min(p.z());
                max_z = max_z.max(p.z());
            }
            let dx = max_x - min_x;
            let dz = max_z - min_z;
            assert!(!(dx > 4.0 && dz > 1.0), "face {fi} spans the support gap (dx={dx:.3}, dz={dz:.3}) — likely a filled hole, not a CAD face");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn export_concrete_forest_left_lowpoly_mesh_json() {
        if std::env::var("EXPORT_LOWPOLY_FOREST_MESH").ok().as_deref() != Some("1") {
            return;
        }
        let source = include_str!("../../../../../../../../📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🎮️play/🔣️.json");
        let root: Value = serde_json::from_str(source).expect("fixture");
        let geometry = parse_geometry(root.pointer("/models/0/model/geometry"));
        let objects = root.pointer("/models/0/model/objects").and_then(|value| value.as_array()).cloned().unwrap_or_default();
        let mut kernel = Brep::new();
        let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
        let handle = GeometryHandle(imported[0].solid_handle.clone().expect("handle"));
        let (positions, face_loops) = kernel.solid_face_loops_sync(&handle).expect("CAD face loops");
        let holed = face_loops.iter().filter(|(_, holes)| !holes.is_empty()).count();
        let mut mesh = HalfedgeMesh::from_face_loops(&positions, &face_loops).expect("halfedge from CAD wires");
        let flips = mesh.orient_faces_consistently().expect("orient faces");
        let before_merge = mesh.face_count();
        let merges = mesh.merge_coplanar_faces().expect("merge coplanar faces");
        assert!(mesh.face_count() <= before_merge, "coplanar merge must not increase face count");
        assert!(merges > 0 || before_merge == mesh.face_count(), "expected coplanar merge to join adjacent CAD faces on the plate/supports");
        assert!((0..mesh.face_count()).any(|fi| mesh.face_vertex_ids(FaceId(fi as u32)).map_or(0, |v| v.len()) > 3), "expected at least one non-triangle CAD face");
        assert_watertight(&mesh);
        assert_no_spanning_face_across_support_gap(&mesh);
        let mut min = MeshVec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = MeshVec3::new(f32::MIN, f32::MIN, f32::MIN);
        for index in 0..mesh.vertex_count() {
            let position = mesh.vertex_position(VertexId(index as u32)).expect("vertex");
            min = MeshVec3([min.x().min(position.x()), min.y().min(position.y()), min.z().min(position.z())]);
            max = MeshVec3([max.x().max(position.x()), max.y().max(position.y()), max.z().max(position.z())]);
        }
        let center = min.add(max).scale(0.5);
        mesh.translate(center.scale(-1.0)).expect("center mesh");
        let _ = mesh.unwrap_uv();
        let json = mesh.to_json().expect("mesh json");
        eprintln!("LOWPOLY_FOREST_MESH_JSON_START");
        eprintln!("{json}");
        eprintln!("LOWPOLY_FOREST_MESH_JSON_END");
    }
}
//#endregion 🔖️ExportConcreteForestMeshTests
