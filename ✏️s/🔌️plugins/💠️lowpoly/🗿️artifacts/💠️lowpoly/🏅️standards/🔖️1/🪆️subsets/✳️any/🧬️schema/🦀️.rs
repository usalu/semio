//! 🧬️ Lowpoly artifact schema — every field of the artifact with its state class.

use crate::{LOWPOLY_PAINT_TEXTURE_SIZE};
use framework_schema::ArtifactSchema;
use semio_framework_3d::mesh::HalfedgeMesh;
use semio_framework_plugin::MeshData;

//#region 🔖️Artifact
/// 🧬️ Full lowpoly artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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
            schema: crate::LOWPOLY_DOCUMENT_SCHEMA.into(),
            objects: Vec::new(),
            active_object_id: None,
            selection: LowpolySelection::default(),
            selected_object_ids: Vec::new(),
            paint_utility: "brush".into(),
            active_paint_layer: 0,
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
    pub fn to_snapshot(&self) -> crate::LowpolySnapshot {
        crate::LowpolySnapshot { schema: self.schema.clone(), objects: self.objects.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::LowpolySnapshot) -> Self {
        Self { schema: snapshot.schema, objects: snapshot.objects, ..Self::default() }
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
pub fn lowpoly_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.lowpoly.lowpoly",
        artifact: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
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
    pub snapshot: crate::LowpolySnapshot,
    pub mesh_workspace: std::collections::HashMap<String, String>,
}

/// 🧱️ Builds the deterministic primitive without UV repacking, so every owner gets a fresh matching
/// handle/payload pair and no process-global child payload cache is required.
pub fn default_owned_document() -> LowpolyOwnedDefaultDocument {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim");
    let mesh_json = mesh.to_json().expect("mesh json");
    let snapshot = crate::snapshot_from_mesh_json(&mesh_json, "obj-1", "Unit Box");
    let mesh_workspace = std::collections::HashMap::from([("obj-1".to_string(), mesh_json)]);
    LowpolyOwnedDefaultDocument { snapshot, mesh_workspace }
}

/// 🎞️ Default document projection used by tests and the play app.
pub fn default_snapshot() -> crate::LowpolySnapshot {
    default_owned_document().snapshot
}

/// 🕸️ Fresh app-owned companion payload for `default_snapshot()`'s exact child handle.
pub fn default_mesh_workspace() -> std::collections::HashMap<String, String> {
    default_owned_document().mesh_workspace
}

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
    ::framework_schema::artifact_schema_descriptor_registered("s.lowpoly.lowpoly")
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️MediaConversion
/// @emoji 🧵️ Builds a `MeshData` transfer payload from a raw tessellation-transfer `DslValue` (as
/// produced by the app's `LowpolyDocument::tessellate_transfer_json`), attaching a composited paint
/// texture when one is supplied. Shared by the app's live 3D scene builder and media export. Relocated
/// from `⚙️engine/🧵️media` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a pure
/// `Value` → `MeshData` conversion, not engine behaviour. Every field read routes through
/// `dsl::FromValue` directly — no `serde_json` bridge anywhere in this call chain.
pub fn mesh_data_from_transfer(transfer: &dsl::DslValue, paint_texture: Option<String>) -> MeshData {
    let read_f32 = |key: &str| -> Vec<f32> { transfer.get(key).and_then(|value| dsl::FromValue::from_value(value.clone()).ok()).unwrap_or_default() };
    let read_u32 = |key: &str| -> Vec<u32> { transfer.get(key).and_then(|value| dsl::FromValue::from_value(value.clone()).ok()).unwrap_or_default() };
    let read_u8 = |key: &str| -> Vec<u8> { transfer.get(key).and_then(|value| dsl::FromValue::from_value(value.clone()).ok()).unwrap_or_default() };
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
pub fn lowpoly_document_from_mesh(mesh: &MeshData) -> Result<serde_json::Value, String> {
    let halfedge = HalfedgeMesh::from_indexed_triangles(&mesh.positions, &mesh.indices).map_err(|err| format!("{err:?}"))?;
    let mesh_json = halfedge.to_json().map_err(|err| format!("{err:?}"))?;
    let snapshot = crate::snapshot_from_mesh_json(&mesh_json, "obj-1", "Imported Mesh");
    Ok(dsl::ToValue::to_value(&snapshot).into())
}

/// 🧊️ Minimal document wrapper for `3d.mesh` resources — no dedicated schema exists yet. Relocated
/// from `⚙️engine/🧵️media`. `MeshData` implements `dsl::ToValue` first-party (hand-written in
/// `🏗️mesh-engine/🦀️.rs`, since `serde`'s `Serialize` on it is `#[cfg(test)]`-only per the same
/// ticket), so this bridges through that instead of `serde_json::to_value(mesh)`.
pub fn mesh_document_from_mesh(mesh: &MeshData) -> Result<serde_json::Value, String> {
    let document = dsl::DslValue::object([("schema".to_string(), dsl::DslValue::String("mesh.document".to_string())), ("mesh".to_string(), dsl::ToValue::to_value(mesh))]);
    Ok(document.into())
}

/// 🔺️ Relocated from `⚙️engine/🧵️media`. Mirrors `mesh_document_from_mesh`'s `dsl::ToValue` bridge in
/// reverse (`dsl::FromValue`), since `MeshData: Deserialize` is likewise `#[cfg(test)]`-only.
pub fn mesh_from_mesh_document(doc: &serde_json::Value) -> Result<MeshData, String> {
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
    use crate::schema::diff::LowpolyDiff;
    use crate::schema::mutations::LowpolyMutation;
    use crate::schema::snapshot::LowpolySnapshot;
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
    use crate::LowpolySnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct LowpolyParts {
        pub snapshot: Option<LowpolySnapshot>,
    }

    pub struct LowpolyAnalyzerAnalysis;

    impl ArtifactAnalysis for LowpolyAnalyzerAnalysis {
        type Parts = LowpolyParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.lowpoly.lowpoly", standard: StandardId("1"), subset: SubsetId("*") };

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
