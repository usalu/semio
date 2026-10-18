//! 🧬️ Remodeling snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `assets` carries real
//! `store::os_store::artifact_child::ArtifactChild<SemioImageSnapshot>` handles (composed `s.stdio.semio.image` children, one
//! per asset id — see `🗿️artifacts/📸️remodeling/🦀️.rs`'s `🧩️Composition` region) instead of
//! embedded `ImageAsset` bytes. `ArtifactChild<S>: dsl::DslField` is now real
//! (`🏪️store/🦀️.rs:523`), and `BTreeMap<String, T: DslField>: DslField` already exists
//! generically (`🗣️dsl/🦀️.rs:178`), so this struct's `#[derive(dsl::DslRecord)]` keeps
//! working unmodified for `assets` — no hand-rolled codec needed here (unlike `🖨️raster`'s own
//! identical migration, authored before this generic `BTreeMap` impl was confirmed reachable).
//! `child_slots()` is honestly EMPTY for `assets` regardless: the derive's `#[child(kind=...)]`
//! mechanism only recognizes a bare `ArtifactChild<T>`/`Vec<ArtifactChild<T>>` field, not a
//! `BTreeMap` value — kept as `BTreeMap<String, ArtifactChild<S>>` (not reshaped to a `Vec`) to
//! preserve the same id-keyed addressing every existing `create-asset`/`delete-asset` mutation and
//! `MediaStream.frames`/`RemodelingMesh.texture_asset_id`/`GeoProducts.*_asset_id` lookup already
//! assumes — the type/mutation/persistence layer is fully real, only the derive-generated SCHEMA
//! INTROSPECTION table is incomplete for this one field (matches `🖨️raster`'s/`💠️lowpoly`'s own
//! already-accepted gap for the identical shape).

use semio_framework::MeshData;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Snapshot

/// 📸️ Persisted remodeling document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "remodeling")]
#[artifact_schema(id = "s.remodel.remodeling")]
pub struct RemodelingSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub streams: Vec<MediaStream>,
    #[value(default)]
    #[state(artifact)]
    pub assets: BTreeMap<String, RemodelingAssetChild>,
    #[value(default)]
    #[state(artifact)]
    pub durable_artifacts: RemodelingDurableArtifactStore,
    #[value(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub calibration: CalibrationState,
    #[value(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub params: ReconstructionParams,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub gcps: Vec<GroundControlPoint>,
    #[value(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub results: ReconstructionResults,
}
//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for RemodelingSnapshot {
    fn default() -> Self {
        Self {
            schema: REMODELING_DOCUMENT_SCHEMA.into(),
            id: "remodeling".into(),
            streams: Vec::new(),
            assets: BTreeMap::new(),
            durable_artifacts: RemodelingDurableArtifactStore::new(),
            calibration: CalibrationState::default(),
            params: ReconstructionParams::default(),
            gcps: Vec::new(),
            results: ReconstructionResults::default(),
        }
    }
}
//#endregion 🔖️Snapshot

/// 🧾️ Document schema id every remodeling snapshot carries.
pub const REMODELING_DOCUMENT_SCHEMA: &str = "remodeling.scene";

/// 🪪️ Ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` contract §1 canonical surface id
/// grammar (`<artifact_kind>@<standard>/<subset>#<role>`). Lives at the ARTIFACT level (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the sibling editor
/// module. `artifact_kind = "s.remodel.remodeling"` is `s.<plugin-id>.<artifact-name>`, the fleet-wide
/// dialect grammar (`s.trinity.jack`, `s.puzzle.puzzle3d`, `s.block.block2d`, `s.procedural.generation2d`)
/// and this artifact's own identity (`ArtifactIdentity::parse("s.remodel.remodeling")` plus the
/// `composer.native` capability row above, and the `🚪️io`/`🧬️schema` `DIALECT` constants). It is NOT
/// `artifact_kind()`'s OS-level `"3d.remodeling"` kind id (a different, unrelated namespace).
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
pub const REMODELING_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.remodel.remodeling", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };

//#region 🧩️Composition
/// 🧩️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (design map §4: "remodeling→C:mesh R:image").
/// Two content-duplication shapes, both verified against real code (not assumed from the one-line
/// design summary):
///
/// 1. **`results.mesh.mesh` (was `MeshData`, the reconstructed/placeholder mesh's flat buffers) is now
///    a composed `s.stdio.semio/v1/mesh` CHILD** (`RemodelingMeshChild = store::os_store::artifact_child::ArtifactChild<
///    SemioMeshSnapshot>`) — exactly `💠️lowpoly`'s own pattern (this ticket's closest precedent for
///    opaque mesh composition), extended with a REAL bidirectional converter
///    (`crate::standards::v1::subsets::any::io::{mesh_data_to_semio_mesh,
///    semio_mesh_to_mesh_data}`, already real — the export path already built the forward direction).
///
/// 2. **`assets: BTreeMap<String, ImageAsset>` (embedded mime+base64 pixel bytes: video frames,
///    baked mesh textures, DSM/DTM/ortho rasters) is now `BTreeMap<String, RemodelingAssetChild>`
///    (`RemodelingAssetChild = store::os_store::artifact_child::ArtifactChild<SemioImageSnapshot>`)** — the design line's "R:image"
///    literally means `ArtifactLink` (an INDEPENDENT-lifecycle reference, `store::ArtifactLink`'s own
///    doc comment: "renders as a chip, never nests inline"). `remodeling`'s assets are NOT independent
///    documents referenced from elsewhere — they are embedded content OWNED by this exact document
///    (keyed by an id that only this document's own `MediaStream.frames`/`RemodelingMesh.
///    texture_asset_id`/`GeoProducts` fields ever address), the identical shape `🖨️raster`'s own
///    `assets: BTreeMap<String, RasterImageAsset>` had — and raster's own migration (this ticket,
///    same design map, "raster→C:image layers R:drawing") converted that shape to a composed CHILD,
///    not a link, documenting exactly this reasoning in place. Followed here for the same reason:
///    composing (owned CHILD) is the honest verb for content this document owns and mutates through
///    its own `create-asset`/`delete-asset` triad; `ArtifactLink` would be dishonest (there is no
///    independent target document to pin/reference).
///
/// **Schema-introspection gap, documented and accepted** (matches raster's/lowpoly's own identical
/// gap): `#[derive(ArtifactSchema)]`'s `#[child(kind=...)]` mechanism only recognizes a bare
/// `ArtifactChild<T>`/`Vec<ArtifactChild<T>>` field declared DIRECTLY on the struct it derives — not a
/// `BTreeMap` value (`assets`) and not a field nested two levels deep inside `results.mesh.mesh`. Kept
/// as-is (not reshaped) to preserve the exact addressing every existing mutation already assumes
/// (`image_key`-equivalent lookups by asset id; `ReplaceMeshResult`'s whole-`RemodelingMesh` payload
/// shape) — the type/mutation/persistence layer is fully real, only the derive's SCHEMA
/// INTROSPECTION table is incomplete for these two fields.
pub type RemodelingAssetChild = store::os_store::artifact_child::ArtifactChild<SemioImageSnapshot>;
pub type RemodelingMeshChild = store::os_store::artifact_child::ArtifactChild<SemioMeshSnapshot>;

/// 🧩️ Restart-stable content authority. Every literal leaf contains at most 4 KiB; values carry
/// only bounded metadata and content-addressed leaves, never a whole image or mesh object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelingDurableArtifact {
    pub kind: String,
    pub mime: Option<String>,
    pub width: u32,
    pub height: u32,
    pub chunks: Vec<ByteBuffer>,
}

pub type RemodelingDurableArtifactStore = BTreeMap<String, RemodelingDurableArtifact>;

pub const REMODELING_DURABLE_CHUNK_RAW_BYTES: usize = 4_096;
const REMODELING_SPARSE_CONTENT_BYTES: usize = 512 * 3 * 4;
const REMODELING_SPARSE_CONTENT_CHUNKS: usize = 2;
const REMODELING_RASTER_CONTENT_BYTES: usize = 1_114_112;
const REMODELING_RASTER_CONTENT_CHUNKS: usize = 272;
const REMODELING_MESH_CONTENT_BYTES: usize = 87_552 + 30;
const REMODELING_MESH_CONTENT_CHUNKS: usize = 30;
const REMODELING_EMPTY_MESH_CHILD_ID: &str = "remodeling-mesh-constant-empty";
const REMODELING_BOX_MESH_CHILD_ID: &str = "remodeling-mesh-constant-box";
const REMODELING_BOUNDED_MESH_VERTICES: usize = 512;
const REMODELING_BOUNDED_MESH_TRIANGLES: usize = 512;

//#region 🔖️AssetHandles
fn mint_asset_child_handle(asset_id: &str, child_id: String) -> RemodelingAssetChild {
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: format!("{asset_id}-image"), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

pub fn committed_remodeling_asset_handle(asset_id: &str, content_id: &str) -> RemodelingAssetChild {
    mint_asset_child_handle(asset_id, content_id.into())
}

/// 🕸️ Deterministic content-addressed CHILD handle for one bounded durable asset.
pub fn image_asset_child_handle(asset_id: &str, asset: &ImageAsset) -> RemodelingAssetChild {
    mint_asset_child_handle(asset_id, store::content_id("remodeling-asset", format!("{}\u{1f}{}", asset.mime, asset.data).as_bytes()))
}

/// 🧩️ Admits a normal imported asset as independently bounded raw leaves. No whole `ImageAsset`
/// value is retained outside the typed mutation currently being reduced.
pub fn store_remodeling_asset(asset_id: &str, asset: &ImageAsset) -> RemodelingAssetChild {
    image_asset_child_handle(asset_id, asset)
}

/// 🧶️ Snapshot-owned decoded-leaf view for bounded consumers. Identity and display metadata stay
/// separate from the compressed payload, and no leaf can exceed 4 KiB.
#[derive(Clone)]
pub struct RemodelingAssetChunkSource {
    pub identity: String,
    pub mime: String,
    pub width: u32,
    pub height: u32,
    pub leaves: Vec<Arc<Vec<u8>>>,
    pub byte_len: usize,
}

pub fn remodeling_asset_chunk_source(snapshot: &RemodelingSnapshot, asset_id: &str) -> Option<RemodelingAssetChunkSource> {
    let handle = snapshot.assets.get(asset_id)?;
    let artifact = snapshot.durable_artifacts.get(&handle.child_id).filter(|artifact| artifact.kind == "image")?;
    let mime = artifact.mime.clone()?;
    let mut leaves = Vec::with_capacity(artifact.chunks.len());
    let mut byte_len = 0usize;
    for encoded in &artifact.chunks {
        let chunk = remodeling_durable_chunk(encoded)?;
        byte_len = byte_len.checked_add(chunk.len())?;
        if byte_len > REMODELING_RASTER_CONTENT_BYTES {
            return None;
        }
        leaves.push(Arc::new(chunk.into()));
    }
    (!leaves.is_empty()).then(|| RemodelingAssetChunkSource { identity: handle.child_id.clone(), mime, width: artifact.width, height: artifact.height, leaves, byte_len })
}

pub fn remodeling_asset_dimensions(snapshot: &RemodelingSnapshot, asset_id: &str) -> Option<(u32, u32)> {
    let handle = snapshot.assets.get(asset_id)?;
    let artifact = snapshot.durable_artifacts.get(&handle.child_id).filter(|artifact| artifact.kind == "image")?;
    Some((artifact.width, artifact.height))
}

/// 🖼️ Reconstructs a bounded API value for export and UI presentation only. Active reconstruction
/// consumes `remodeling_asset_chunk_source` directly and never passes through this whole-value facade.
pub fn remodeling_asset(snapshot: &RemodelingSnapshot, asset_id: &str) -> Option<ImageAsset> {
    let source = remodeling_asset_chunk_source(snapshot, asset_id)?;
    let mut bytes = Vec::with_capacity(source.byte_len);
    for leaf in source.leaves {
        bytes.extend_from_slice(&leaf);
    }
    Some(ImageAsset { mime: source.mime, data: base64_codec::base64_standard_encode(bytes), width: source.width, height: source.height })
}

pub fn durable_remodeling_asset(asset: &ImageAsset) -> Option<RemodelingDurableArtifact> {
    let bytes = base64_codec::base64_standard_decode(asset.data.as_bytes()).ok()?;
    (bytes.len() <= REMODELING_RASTER_CONTENT_BYTES).then_some(RemodelingDurableArtifact {
        kind: "image".into(),
        mime: Some(asset.mime.clone()),
        width: asset.width,
        height: asset.height,
        chunks: bytes.chunks(REMODELING_DURABLE_CHUNK_RAW_BYTES).map(ByteBuffer::from_u8_slice).collect(),
    })
}

//#region 🔖️DurableContent
/// 🏷️ Durable content lanes a reconstruction publishes through `append-content`, each with its exact
/// bounded envelope. The wire spelling is the `RemodelingDurableArtifact.kind` it writes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum RemodelingContentKind {
    #[default]
    Sparse,
    Mesh,
    Image,
}

impl RemodelingContentKind {
    pub const ALL: [Self; 3] = [Self::Sparse, Self::Mesh, Self::Image];

    pub fn wire(self) -> &'static str {
        match self {
            Self::Sparse => "sparse",
            Self::Mesh => "mesh",
            Self::Image => "image",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.wire() == value)
    }

    pub fn max_bytes(self) -> usize {
        match self {
            Self::Sparse => REMODELING_SPARSE_CONTENT_BYTES,
            Self::Mesh => REMODELING_MESH_CONTENT_BYTES,
            Self::Image => REMODELING_RASTER_CONTENT_BYTES,
        }
    }

    pub fn max_chunks(self) -> usize {
        match self {
            Self::Sparse => REMODELING_SPARSE_CONTENT_CHUNKS,
            Self::Mesh => REMODELING_MESH_CONTENT_CHUNKS,
            Self::Image => REMODELING_RASTER_CONTENT_CHUNKS,
        }
    }

    fn digest_prefix(self) -> &'static str {
        match self {
            Self::Mesh => "remodeling-mesh",
            Self::Sparse | Self::Image => "remodeling-asset",
        }
    }
}

/// #️⃣️ Streaming content digest a run folds over every raw leaf it publishes; the resulting id names
/// the durable content, so identical reconstruction output lands on the identical durable entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct RemodelingContentDigest {
    digest: [u64; 4],
    len: u64,
}

impl Default for RemodelingContentDigest {
    fn default() -> Self {
        Self { digest: [0x6c62272e07bb0142, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f], len: 0 }
    }
}

impl RemodelingContentDigest {
    pub fn record(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.len = self.len.wrapping_add(1);
            self.digest[0] = (self.digest[0] ^ u64::from(*byte)).wrapping_mul(0x00000100000001b3);
            self.digest[1] = (self.digest[1] ^ self.digest[0].rotate_left(17) ^ self.len).wrapping_mul(0x9e3779b185ebca87);
            self.digest[2] = (self.digest[2] ^ self.digest[1].rotate_left(29) ^ u64::from(*byte)).wrapping_mul(0xc2b2ae3d27d4eb4f);
            self.digest[3] = (self.digest[3] ^ self.digest[2].rotate_left(41) ^ self.len.rotate_left(7)).wrapping_mul(0x165667b19e3779f9);
        }
    }

    pub fn byte_len(&self) -> u64 {
        self.len
    }

    pub fn parts(&self) -> ([u64; 4], u64) {
        (self.digest, self.len)
    }

    pub fn from_parts(digest: [u64; 4], len: u64) -> Self {
        Self { digest, len }
    }

    pub fn content_id(&self, kind: RemodelingContentKind) -> String {
        format!("{}-{:016x}{:016x}{:016x}{:016x}-{:016x}", kind.digest_prefix(), self.digest[0], self.digest[1], self.digest[2], self.digest[3], self.len)
    }
}

/// 🧱️ Raw bytes of one durable leaf, refusing anything above the 4 KiB leaf envelope.
pub fn remodeling_durable_chunk(chunk: &ByteBuffer) -> Option<&[u8]> {
    (chunk.0.len() <= REMODELING_DURABLE_CHUNK_RAW_BYTES).then_some(chunk.0.as_slice())
}

/// ✅️ Whether `content_id` names complete durable content of `kind` with exactly `chunk_count` leaves
/// inside the kind's envelope; a mesh must additionally resolve inside the 512/512 resolution envelope.
pub fn remodeling_content_is_complete(store: &RemodelingDurableArtifactStore, content_id: &str, kind: RemodelingContentKind, chunk_count: u64) -> bool {
    let Some(artifact) = store.get(content_id).filter(|artifact| artifact.kind == kind.wire()) else { return false };
    if chunk_count == 0 || u64::try_from(artifact.chunks.len()).ok() != Some(chunk_count) {
        return false;
    }
    match kind {
        RemodelingContentKind::Mesh => mesh_from_durable_chunks(&artifact.chunks).is_some(),
        RemodelingContentKind::Sparse | RemodelingContentKind::Image => {
            let mut bytes = 0usize;
            artifact.chunks.iter().all(|encoded| remodeling_durable_chunk(encoded).and_then(|chunk| bytes.checked_add(chunk.len())).is_some_and(|total| {
                bytes = total;
                total <= kind.max_bytes()
            }))
        }
    }
}
//#endregion 🔖️DurableContent
//#endregion 🔖️AssetHandles

//#region 🔖️MeshHandle
fn mesh_child_handle(child_id: String, artifact_id: String) -> RemodelingMeshChild {
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id, dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

#[cfg(test)]
fn mesh_digest_bytes(chunks: impl IntoIterator<Item = impl AsRef<[u8]>>) -> String {
    let mut digest = RemodelingContentDigest::default();
    for chunk in chunks {
        digest.record(chunk.as_ref());
    }
    digest.content_id(RemodelingContentKind::Mesh)
}

/// 🧱️ Folds one field-tagged mesh leaf (`[field, values…]`, fields in ascending order) into `mesh`.
pub fn apply_mesh_chunk(mesh: &mut MeshData, last_field: Option<u8>, bytes: &[u8]) -> bool {
    let Some((&field, values)) = bytes.split_first() else { return false };
    if field > 11 || last_field.is_some_and(|previous| field < previous) {
        return false;
    }
    let component_count = values.len() / 4;
    match field {
        0 | 1 | 2 | 4 | 7 | 9 if values.len() % 4 == 0 => {
            let target = match field {
                0 => &mut mesh.positions,
                1 => &mut mesh.normals,
                2 => &mut mesh.colors,
                4 => &mut mesh.uvs,
                7 => &mut mesh.edge_positions,
                _ => &mut mesh.edge_uvs,
            };
            let limit = match field {
                0 | 1 => REMODELING_BOUNDED_MESH_VERTICES * 3,
                2 => REMODELING_BOUNDED_MESH_VERTICES * 4,
                4 => REMODELING_BOUNDED_MESH_VERTICES * 2,
                7 => REMODELING_BOUNDED_MESH_TRIANGLES * 6,
                _ => REMODELING_BOUNDED_MESH_TRIANGLES * 4,
            };
            if target.len().checked_add(component_count).is_none_or(|count| count > limit) {
                return false;
            }
            target.extend(values.as_chunks::<4>().0.iter().map(|value| f32::from_le_bytes(*value)));
        }
        3 | 5 | 6 | 8 if values.len() % 4 == 0 => {
            let target = match field {
                3 => &mut mesh.indices,
                5 => &mut mesh.face_ids,
                6 => &mut mesh.vertex_ids,
                _ => &mut mesh.edge_ids,
            };
            let limit = match field {
                3 => REMODELING_BOUNDED_MESH_TRIANGLES * 3,
                5 => REMODELING_BOUNDED_MESH_TRIANGLES,
                6 => REMODELING_BOUNDED_MESH_VERTICES,
                _ => REMODELING_BOUNDED_MESH_TRIANGLES * 3,
            };
            if target.len().checked_add(component_count).is_none_or(|count| count > limit) {
                return false;
            }
            target.extend(values.as_chunks::<4>().0.iter().map(|value| u32::from_le_bytes(*value)));
        }
        10 => {
            if mesh.edge_is_seam.len().checked_add(values.len()).is_none_or(|count| count > REMODELING_BOUNDED_MESH_TRIANGLES * 3) {
                return false;
            }
            mesh.edge_is_seam.extend_from_slice(values);
        }
        11 => {
            let Ok(value) = std::str::from_utf8(values) else { return false };
            if mesh.paint_texture_base64.as_ref().map_or(0, String::len).checked_add(value.len()).is_none_or(|count| count > 24_576) {
                return false;
            }
            mesh.paint_texture_base64.get_or_insert_with(String::new).push_str(value);
        }
        _ => return false,
    }
    true
}

fn mesh_from_durable_chunks(chunks: &[ByteBuffer]) -> Option<MeshData> {
    let mut mesh = MeshData::default();
    let mut last_field = None;
    let mut bytes = 0usize;
    for encoded in chunks {
        let chunk = remodeling_durable_chunk(encoded)?;
        bytes = bytes.checked_add(chunk.len()).filter(|total| *total <= REMODELING_MESH_CONTENT_BYTES)?;
        if !apply_mesh_chunk(&mut mesh, last_field, &chunk) {
            return None;
        }
        last_field = chunk.first().copied();
    }
    mesh_is_within_resolution_envelope(&mesh).then_some(mesh)
}

/// 🔗️ The composed mesh child a durable reconstructed or imported mesh publishes.
pub fn remodeling_mesh_content_handle(content_id: &str, chunk_count: u64) -> RemodelingMeshChild {
    mesh_child_handle(content_id.into(), format!("remodeling-mesh-content:{chunk_count}"))
}

pub fn remodeling_mesh_content_handle_parts(handle: &RemodelingMeshChild) -> Option<(&str, u64)> {
    let chunk_count = handle.target.artifact_id.strip_prefix("remodeling-mesh-content:")?;
    Some((&handle.child_id, chunk_count.parse().ok()?))
}

/// 🧊️ Stable empty-mesh handle with no process-owned payload.
pub fn empty_remodeling_mesh_handle() -> RemodelingMeshChild {
    mesh_child_handle(REMODELING_EMPTY_MESH_CHILD_ID.into(), "remodeling-mesh-constant:empty".into())
}

/// 📦️ Stable bounded placeholder handle with no process-owned payload.
pub fn placeholder_remodeling_mesh_handle() -> RemodelingMeshChild {
    mesh_child_handle(REMODELING_BOX_MESH_CHILD_ID.into(), "remodeling-mesh-constant:box".into())
}

pub fn mesh_is_within_resolution_envelope(mesh: &MeshData) -> bool {
    let vertices = mesh.positions.len().checked_div(3);
    let triangles = mesh.indices.len().checked_div(3);
    let Some(vertices) = vertices.filter(|_| mesh.positions.len().is_multiple_of(3)) else { return false };
    let Some(triangles) = triangles.filter(|_| mesh.indices.len().is_multiple_of(3)) else { return false };
    vertices <= REMODELING_BOUNDED_MESH_VERTICES
        && triangles <= REMODELING_BOUNDED_MESH_TRIANGLES
        && mesh.indices.iter().all(|index| usize::try_from(*index).ok().is_some_and(|index| index < vertices))
        && (mesh.normals.is_empty() || vertices.checked_mul(3) == Some(mesh.normals.len()))
        && (mesh.colors.is_empty() || vertices.checked_mul(3) == Some(mesh.colors.len()) || vertices.checked_mul(4) == Some(mesh.colors.len()))
        && (mesh.uvs.is_empty() || vertices.checked_mul(2) == Some(mesh.uvs.len()))
        && (mesh.face_ids.is_empty() || mesh.face_ids.len() == triangles)
        && (mesh.vertex_ids.is_empty() || mesh.vertex_ids.len() == vertices)
        && mesh.normals.len() <= REMODELING_BOUNDED_MESH_VERTICES * 3
        && mesh.colors.len() <= REMODELING_BOUNDED_MESH_VERTICES * 4
        && mesh.uvs.len() <= REMODELING_BOUNDED_MESH_VERTICES * 2
        && mesh.face_ids.len() <= REMODELING_BOUNDED_MESH_TRIANGLES
        && mesh.vertex_ids.len() <= REMODELING_BOUNDED_MESH_VERTICES
        && mesh.edge_positions.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 6
        && mesh.edge_ids.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 3
        && mesh.edge_uvs.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 4
        && mesh.edge_is_seam.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 3
        && mesh.paint_texture_base64.as_ref().is_none_or(|value| value.len() <= 24_576)
}

/// 🧱️ Returns the durable raw chunk count without reconstructing or cloning mesh fields.
pub fn bounded_remodeling_mesh_chunk_count(store: &RemodelingDurableArtifactStore, handle: &RemodelingMeshChild) -> Option<u64> {
    let (content_id, chunk_count) = remodeling_mesh_content_handle_parts(handle)?;
    let artifact = store.get(content_id).filter(|artifact| artifact.kind == "mesh")?;
    (u64::try_from(artifact.chunks.len()).ok() == Some(chunk_count)).then_some(chunk_count)
}

/// 🧱️ Resolves one admitted durable mesh chunk without cloning the reconstructed mesh.
pub fn bounded_remodeling_mesh_chunk(store: &RemodelingDurableArtifactStore, handle: &RemodelingMeshChild, index: u64) -> Option<Arc<[u8]>> {
    let chunk_count = bounded_remodeling_mesh_chunk_count(store, handle)?;
    if index >= chunk_count {
        return None;
    }
    let artifact = store.get(&handle.child_id)?;
    Some(Arc::from(remodeling_durable_chunk(artifact.chunks.get(usize::try_from(index).ok()?)?)?))
}

/// 🧱️ Resolves only fixed constants or durable content admitted by the 512/512 envelope.
pub fn resolve_bounded_remodeling_mesh(store: &RemodelingDurableArtifactStore, handle: &RemodelingMeshChild) -> Option<MeshData> {
    match handle.child_id.as_str() {
        REMODELING_EMPTY_MESH_CHILD_ID => Some(MeshData::default()),
        REMODELING_BOX_MESH_CHILD_ID => Some(semio_framework::mesh_from_kind("box")),
        _ => {
            let chunk_count = bounded_remodeling_mesh_chunk_count(store, handle)?;
            let artifact = store.get(&handle.child_id)?;
            (u64::try_from(artifact.chunks.len()).ok() == Some(chunk_count)).then(|| mesh_from_durable_chunks(&artifact.chunks)).flatten()
        }
    }
}

#[cfg(test)]
static REMODELING_TEST_MESHES: std::sync::OnceLock<std::sync::Mutex<BTreeMap<String, MeshData>>> = std::sync::OnceLock::new();

#[cfg(test)]
fn remodeling_test_meshes() -> &'static std::sync::Mutex<BTreeMap<String, MeshData>> {
    REMODELING_TEST_MESHES.get_or_init(|| std::sync::Mutex::new(BTreeMap::new()))
}

#[cfg(any(test, feature = "testkit"))]
pub fn mint_and_stash_mesh(mesh: MeshData) -> RemodelingMeshChild {
    let bytes = semio_framework_pack_json::to_json_string(&mesh).into_bytes();
    let child_id = mesh_digest_bytes([bytes.as_slice()]);
    remodeling_test_meshes().lock().expect("remodeling test mesh lock").insert(child_id.clone(), mesh);
    mesh_child_handle(child_id, "remodeling-mesh".into())
}

#[cfg(any(test, feature = "testkit"))]
pub fn remodeling_mesh_workspace(handle: &RemodelingMeshChild) -> Option<MeshData> {
    if handle.child_id == REMODELING_EMPTY_MESH_CHILD_ID {
        return Some(MeshData::default());
    }
    if handle.child_id == REMODELING_BOX_MESH_CHILD_ID {
        return Some(semio_framework::mesh_from_kind("box"));
    }
    remodeling_test_meshes().lock().expect("remodeling test mesh lock").get(&handle.child_id).cloned()
}
//#endregion 🔖️MeshHandle
//#endregion 🧩️Composition

//#region 🔖️Buffers
/// ☁️ Literal samples or an explicit reference to snapshot-owned raw content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Float32Buffer {
    #[dsl(key = "inline")]
    Inline { values: Vec<f32> },
    #[dsl(key = "content")]
    Content { content_id: String, chunk_count: u64 },
}

impl Default for Float32Buffer {
    fn default() -> Self { Self::Inline { values: Vec::new() } }
}

impl Float32Buffer {
    pub fn from_f32_slice(values: &[f32]) -> Self { Self::Inline { values: values.to_vec() } }
    pub fn to_f32_vec(&self) -> Vec<f32> { match self { Self::Inline { values } => values.clone(), Self::Content { .. } => Vec::new() } }
    pub fn content_reference(&self) -> Option<(&str, u64)> { match self { Self::Content { content_id, chunk_count } => Some((content_id, *chunk_count)), Self::Inline { .. } => None } }
    pub fn to_f32_vec_from(&self, store: &RemodelingDurableArtifactStore) -> Vec<f32> {
        let Some((content_id, chunk_count)) = self.content_reference() else { return self.to_f32_vec() };
        let Some(artifact) = store.get(content_id).filter(|artifact| artifact.kind == "sparse" && u64::try_from(artifact.chunks.len()).ok() == Some(chunk_count)) else { return Vec::new() };
        let mut values = Vec::new();
        for leaf in &artifact.chunks {
            let Some(bytes) = remodeling_durable_chunk(leaf) else { return Vec::new() };
            let (chunks, remainder) = bytes.as_chunks::<4>();
            if !remainder.is_empty() { return Vec::new(); }
            values.extend(chunks.iter().map(|chunk| f32::from_le_bytes(*chunk)));
        }
        values
    }
    pub fn is_empty(&self) -> bool { match self { Self::Inline { values } => values.is_empty(), Self::Content { chunk_count, .. } => *chunk_count == 0 } }
}

impl semio_framework_dsl_record::BorrowedDslField for Float32Buffer { const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Value; }

impl semio_framework_dsl_record::DslField for Float32Buffer {
    fn shape() -> semio_framework_dsl_record::Shape { semio_framework_dsl_record::Shape::Block(Box::new(semio_framework_dsl_record::Shape::Statements(<Self as semio_framework_dsl_record::DslVariants>::variants()))) }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, semio_framework_value::ValueError> {
        let shape = semio_framework_dsl_record::Shape::Statements(<Self as semio_framework_dsl_record::DslVariants>::variants_controlled(control)?);
        Ok(semio_framework_dsl_record::Shape::Block(semio_framework_dsl_record::producer::boxed(shape, control)?))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue { semio_framework_dsl_record::FieldValue::Block(Box::new(semio_framework_dsl_record::FieldValue::Statements(vec![<Self as semio_framework_dsl_record::DslVariants>::to_named_record(self)]))) }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, semio_framework_value::ValueError> {
        let mut statements = control.allocate_vec(1)?;
        control.charge(size_of::<semio_framework_dsl_record::FieldValue>())?;
        statements.push(<Self as semio_framework_dsl_record::DslVariants>::to_named_record_controlled(self, control)?);
        Ok(semio_framework_dsl_record::FieldValue::Block(Box::new(semio_framework_dsl_record::FieldValue::Statements(statements))))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Block(value) = value else { return Err("expected a float-buffer block".into()) };
        let semio_framework_dsl_record::FieldValue::Statements(statements) = value.as_ref() else { return Err("expected a float-buffer variant".into()) };
        let [(keyword, record)] = statements.as_slice() else { return Err("expected one float-buffer variant".into()) };
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, record).map_err(|error| error.to_string())
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        control.checkpoint()?;
        let semio_framework_dsl_record::FieldValue::Block(value) = value else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a float-buffer block")) };
        let semio_framework_dsl_record::FieldValue::Statements(statements) = value.as_ref() else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a float-buffer variant")) };
        let [(keyword, record)] = statements.as_slice() else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected one float-buffer variant")) };
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record_controlled(keyword, record, control)
    }
}

/// 🧱 Literal octets owned by a snapshot or a typed reconstruction operation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(transparent)]
#[serde(transparent)]
pub struct ByteBuffer(pub Vec<u8>);

impl ByteBuffer {
    pub fn from_u8_slice(values: &[u8]) -> Self { Self(values.to_vec()) }
    pub fn to_u8_vec(&self) -> Vec<u8> { self.0.clone() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

impl semio_framework_dsl_record::BorrowedDslField for ByteBuffer { const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Bytes64; }

impl semio_framework_dsl_record::DslField for ByteBuffer {
    fn shape() -> semio_framework_dsl_record::Shape { semio_framework_dsl_record::Shape::Bytes64 }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, semio_framework_value::ValueError> { control.checkpoint()?; Ok(semio_framework_dsl_record::Shape::Bytes64) }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue { semio_framework_dsl_record::FieldValue::Bytes64(self.0.clone()) }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, semio_framework_value::ValueError> { control.copy_bytes(&self.0).map(semio_framework_dsl_record::FieldValue::Bytes64) }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> { match value { semio_framework_dsl_record::FieldValue::Bytes64(bytes) => Ok(Self(bytes.clone())), _ => Err("expected literal octets".into()) } }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> { match value { semio_framework_dsl_record::FieldValue::Bytes64(bytes) => control.copy_bytes(bytes).map(Self), _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected literal octets")) } }
}
//#endregion 🔖️Buffers

//#region 🔖️Domain
/// 🖼️ One embedded pixel asset (video frame, ortho tile, texture) referenced by id from
/// `RemodelingSnapshot::assets`, `MediaStream.frames`, `RemodelingMesh.texture_asset_id`, or
/// `GeoProducts.{dsm,dtm,ortho}_asset_id`. Sampled video frames use `image/jpeg` (~10x smaller than
/// PNG for photographic content); PNG stays reserved for exports/textures/rasters that need
/// lossless round trips.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ImageAsset {
    pub mime: String,
    pub data: String,
    pub width: u32,
    pub height: u32,
}

/// 🗂️ Which shape a `MediaStream`'s frames were captured as. Video input is always eagerly extracted
/// into individually-addressable `FrameRef`s before persistence (video bytes themselves are never
/// stored) — `MediaKind::Video` only records that provenance, `MediaStream.source` carries the detail.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum MediaKind {
    #[default]
    ImageSequence,
    Video,
}

/// 🎞️ Codec a `VideoSource` was demuxed from — a plain mirror of `remodeling_video::VideoCodec` without
/// its `FourCc` payload (an unrecognized four-character code collapses to `Unknown`, which is enough
/// provenance for a QC/diagnostic label).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum VideoCodec {
    Avc,
    Hevc,
    Vp9,
    Av1,
    Mjpeg,
    #[default]
    Unknown,
}

/// 🎥️ Provenance of a `MediaStream` that originated from an actual video file (as opposed to a raw
/// image-sequence import), populated from the owned `remodeling_video::VideoProbe` record
/// once at import time from `remodeling_video::probe`. "Video input = image sequence with timestamps":
/// by the time a stream reaches this document its frames are already individually-addressable
/// `ImageAsset`s with true media timestamps; this struct only records where they came from.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct VideoSource {
    pub name: String,
    pub container: String,
    pub codec: VideoCodec,
    pub duration_ms: f64,
    pub frame_count: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct FrameRef {
    pub index: u32,
    pub timestamp_ms: f64,
    pub asset_id: String,
}

/// 🎞️ One imported media source (an image sequence or a video), decoded into `FrameRef`s pointing at
/// `RemodelingSnapshot::assets`. Multiple cameras/angles are multiple streams, joined by `camera_id`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct MediaStream {
    pub id: String,
    pub name: String,
    pub kind: MediaKind,
    pub camera_id: Option<String>,
    pub sync_offset_ms: f64,
    pub fps_hint: f64,
    #[dsl(table)]
    pub frames: Vec<FrameRef>,
    #[dsl(block)]
    pub source: Option<VideoSource>,
}

/// 🎯️ Per-camera intrinsics/distortion, a plain-JSON mirror of `remodeling_camera::{Intrinsics,
/// Distortion}` rather than a direct reuse of those types: `Distortion` is a Rust enum tuned for the
/// solver's math (`BrownConrady{k1,k2,k3,p1,p2}` / `FisheyeEquidistant{k1,k2,k3,k4}`), which doesn't
/// serialize into a stable arg-form-editable shape — the document instead always carries a flat
/// 5-slot `distortion` array plus a `model` label the plugin uses to decide which slots are live,
/// matching the "pinhole|brownConrady|fisheye" UI select.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct CameraCalibration {
    pub id: String,
    pub label: String,
    pub model: String,
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub skew: f64,
    /// 🔢️ `[k1, k2, k3, p1, p2]`.
    pub distortion: [f32; 5],
    pub rms_reprojection_px: Option<f32>,
    pub locked: bool,
}

/// 🎯️ One rig member's pose relative to the rig origin — a plain mirror of `remodeling_camera`'s
/// `RigExtrinsic{camera_id, pose_in_rig: Se3}`, flattened to a quaternion + translation since `Se3`
/// (a `crate::lie` manifold type) is a plugin-runtime concern, not a document one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct RigExtrinsic {
    pub camera_id: String,
    pub rotation_wxyz: [f32; 4],
    #[dsl(coord)]
    pub translation_m: [f32; 3],
}

impl Default for RigExtrinsic {
    fn default() -> Self {
        Self { camera_id: String::new(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation_m: [0.0; 3] }
    }
}

/// 🎯️ Per-camera intrinsics/distortion plus rig extrinsics, refined by `remodeling_camera`/`remodeling_sfm`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct CalibrationState {
    #[dsl(table)]
    pub cameras: Vec<CameraCalibration>,
    #[dsl(table)]
    pub rig: Vec<RigExtrinsic>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct GcpObservation {
    pub stream_id: String,
    pub frame_index: u32,
    pub pixel: [f32; 2],
}

/// 📍️ A surveyed ground-control point used by `remodeling_geo` to georeference the reconstruction.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct GroundControlPoint {
    pub id: String,
    pub name: String,
    #[dsl(coord)]
    pub world_position: [f64; 3],
    #[dsl(table)]
    pub observations: Vec<GcpObservation>,
}

/// ⏭️ Frame sampling/decode limits `remodeling_engine` applies before feature extraction. `min_sharpness`
/// is the blur gate: a candidate frame is dropped when its sharpness falls below this fraction of the
/// rolling median sharpness of the last ~15 accepted frames.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct IngestParams {
    pub frame_sample_stride: u32,
    pub max_frames: u32,
    pub downscale_long_edge_px: u32,
    pub min_sharpness: f32,
}

impl Default for IngestParams {
    fn default() -> Self {
        Self { frame_sample_stride: 5, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum FeatureDetector {
    #[default]
    Orb,
    Akaze,
    Harris,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct FeatureParams {
    pub detector: FeatureDetector,
    pub target_count: u32,
    pub octaves: u32,
    pub edge_threshold: f32,
}

impl Default for FeatureParams {
    fn default() -> Self {
        Self { detector: FeatureDetector::default(), target_count: 4000, octaves: 4, edge_threshold: 10.0 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum MatcherKind {
    #[default]
    BruteForce,
    KdTree,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct MatchParams {
    pub matcher: MatcherKind,
    pub ratio_test: f32,
    pub cross_check: bool,
    pub sequential_window: u32,
    pub max_pairs_per_frame: u32,
    pub loop_closure: bool,
}

impl Default for MatchParams {
    fn default() -> Self {
        Self { matcher: MatcherKind::default(), ratio_test: 0.8, cross_check: true, sequential_window: 8, max_pairs_per_frame: 16, loop_closure: true }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum RobustLossKind {
    L2,
    #[default]
    Huber,
    Cauchy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct SfmParams {
    pub ransac_iterations: u32,
    pub ransac_threshold_px: f32,
    pub min_track_length: u32,
    pub ba_max_iterations: u32,
    pub robust_loss: RobustLossKind,
    pub huber_delta_px: f32,
}

impl Default for SfmParams {
    fn default() -> Self {
        Self { ransac_iterations: 1000, ransac_threshold_px: 2.0, min_track_length: 3, ba_max_iterations: 50, robust_loss: RobustLossKind::default(), huber_delta_px: 1.5 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum DenseResolution {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct DenseParams {
    pub resolution: DenseResolution,
    pub window_radius_px: u32,
    pub min_view_consistency: u32,
    pub confidence_threshold: f32,
    pub max_points: u32,
}

impl Default for DenseParams {
    fn default() -> Self {
        Self { resolution: DenseResolution::default(), window_radius_px: 3, min_view_consistency: 3, confidence_threshold: 0.5, max_points: 500_000 }
    }
}

/// 🧊️ UI-facing meshing knobs `remodeling_engine` translates into `remodeling_mesh`'s own internal
/// `MeshParams`/`TsdfVolume` construction args (this document does not depend on `remodeling_mesh`, so
/// the two `MeshParams` types are intentionally separate). `guarantee_watertight`,
/// `hole_fill_max_boundary_verts`, and `self_intersection_check` are the watertight-guarantee knobs:
/// when `guarantee_watertight` is set and repair/hole-fill can't recover a closed 2-manifold, the
/// `🔖️Close` fallback triggers and re-validates until the result passes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct MeshParams {
    #[dsl(unit = "mm")]
    pub tsdf_voxel_size_mm: f32,
    #[dsl(unit = "mm")]
    pub tsdf_truncation_mm: f32,
    pub decimate_target_triangles: u32,
    pub smoothing_iterations: u32,
    pub texture_enabled: bool,
    pub texture_size: u32,
    pub guarantee_watertight: bool,
    pub hole_fill_max_boundary_verts: u32,
    pub self_intersection_check: bool,
}

impl Default for MeshParams {
    fn default() -> Self {
        Self {
            tsdf_voxel_size_mm: 5.0,
            tsdf_truncation_mm: 20.0,
            decimate_target_triangles: 200_000,
            smoothing_iterations: 2,
            texture_enabled: true,
            texture_size: 2048,
            guarantee_watertight: true,
            hole_fill_max_boundary_verts: 512,
            self_intersection_check: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct MotionParams {
    pub enabled: bool,
    pub max_tracks: u32,
    pub track_window_px: u32,
    pub min_track_quality: f32,
    pub min_track_length_frames: u32,
}

impl Default for MotionParams {
    fn default() -> Self {
        Self { enabled: false, max_tracks: 64, track_window_px: 21, min_track_quality: 0.3, min_track_length_frames: 5 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct GeoParams {
    pub enabled: bool,
    pub origin_lon: Option<f64>,
    pub origin_lat: Option<f64>,
    pub origin_alt: Option<f64>,
    #[dsl(unit = "m")]
    pub gsd_m: f32,
    #[dsl(unit = "m")]
    pub dsm_cell_m: f32,
    #[dsl(unit = "m")]
    pub dtm_filter_radius_m: f32,
    pub ortho_max_px: u32,
}

impl Default for GeoParams {
    fn default() -> Self {
        Self { enabled: false, origin_lon: None, origin_lat: None, origin_alt: None, gsd_m: 0.05, dsm_cell_m: 0.1, dtm_filter_radius_m: 2.0, ortho_max_px: 4096 }
    }
}

/// ⚙️ Full reconstruction parameter set, one sub-struct per pipeline stage — `remodeling_engine` reads
/// these directly to configure `remodeling_image`/`remodeling_video`/`remodeling_camera`/`remodeling_feature`/
/// `remodeling_sfm`/`remodeling_dense`/`remodeling_mesh`/`remodeling_motion`/`remodeling_geo` without this crate
/// depending on any of them.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionParams {
    #[dsl(block)]
    pub ingest: IngestParams,
    #[dsl(block)]
    pub feature: FeatureParams,
    #[dsl(block)]
    pub matching: MatchParams,
    #[dsl(block)]
    pub sfm: SfmParams,
    #[dsl(block)]
    pub dense: DenseParams,
    #[dsl(block)]
    pub mesh: MeshParams,
    #[dsl(block)]
    pub motion: MotionParams,
    #[dsl(block)]
    pub geo: GeoParams,
}

/// 📷️ A single recovered camera pose — one `CameraTrajectory.poses` entry a finalized reconstruction run
/// publishes; while the run is live the same poses stream as `reconstruction` tool-run trace records.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct CameraPosePreview {
    pub camera_id: String,
    pub rotation_wxyz: [f32; 4],
    #[dsl(coord)]
    pub translation: [f32; 3],
}

impl Default for CameraPosePreview {
    fn default() -> Self {
        Self { camera_id: String::new(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation: [0.0; 3] }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum MeshSource {
    #[default]
    Placeholder,
    Reconstructed,
    Imported,
}

/// ✅️ A plain-JSON mirror of `remodeling_mesh::WatertightReport`'s summary fields (all scalars — the
/// report itself carries no array data, so this is a snapshot only in the sense of avoiding a hard
/// dependency on `remodeling_mesh`, not in the sense of trimming size).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct WatertightReportSnapshot {
    pub vertex_count: u32,
    pub triangle_count: u32,
    pub boundary_edge_count: u32,
    pub boundary_loop_count: u32,
    pub non_manifold_edge_count: u32,
    pub non_manifold_vertex_count: u32,
    pub connected_components: u32,
    pub consistently_oriented: bool,
    pub euler_characteristic: i64,
    pub genus: Option<i64>,
    pub signed_volume: f64,
    pub self_intersection_pairs: Option<u32>,
    pub closed_fallback_used: bool,
    pub is_closed: bool,
    pub is_two_manifold: bool,
    pub is_watertight: bool,
}

/// 🧵️ The reconstructed (or placeholder/imported) mesh. Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `mesh` is now a composed `s.stdio.semio/v1/mesh`
/// CHILD handle (`RemodelingMeshChild`, `🧩️Composition` region above), never embedded `MeshData` —
/// reconstructed geometry resolves through a bounded durable chunk handle; the empty and box
/// placeholders are typed constants that require no process cache.
/// `source`/`texture_asset_id`/`watertight` are genuinely NOT part of the composed mesh's own content
/// (they describe THIS document's relationship to the mesh — provenance, a separate asset reference, a
/// derived QC summary — not geometry), so they stay sibling fields here rather than folding into the
/// child, matching `puzzle`'s own `*Extra`-sibling precedent for content a composed subset's shape
/// can't represent. Always present (never `Option`) so the 3D view always has something to render —
/// `default_remodeling_scene()` seeds it with a placeholder box.
///
/// `ArtifactChild<S>: dsl::DslField` is now real (`🏪️store/🦀️.rs:523`) so this struct keeps
/// its plain `#[derive(dsl::DslRecord)]` instead of hand-rolling — the former `🔖️MeshBridge` region
/// (a `MeshDataTwin` buffer-by-buffer bridge, needed only because `MeshData` is foreign and had no
/// `DslField` impl reachable from this crate) is gone entirely: every field left on this struct now has
/// a real `DslField` impl on its own.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
pub struct RemodelingMesh {
    #[dsl(block)]
    #[value(default = "empty_remodeling_mesh_handle")]
    pub mesh: RemodelingMeshChild,
    pub source: MeshSource,
    pub texture_asset_id: Option<String>,
    #[dsl(block)]
    pub watertight: Option<WatertightReportSnapshot>,
}

impl Default for RemodelingMesh {
    fn default() -> Self {
        Self { mesh: empty_remodeling_mesh_handle(), source: MeshSource::default(), texture_asset_id: None, watertight: None }
    }
}

/// ☁️ Sparse point cloud from bundle adjustment (`points` = flat xyz triples).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct SparseCloud {
    pub points: Float32Buffer,
    pub colors: Option<ByteBuffer>,
}

/// ☁️ Dense point cloud with optional per-point LAS-style classification codes (0 unclassified, 2
/// ground, 6 building, …) — `remodeling_dense::PointClass` is a bespoke enum without numeric LAS
/// discriminants, so `remodeling_engine` maps it to LAS codes when it distills this snapshot.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct DenseCloud {
    pub positions: Float32Buffer,
    pub colors: Option<ByteBuffer>,
    pub confidence: Option<Float32Buffer>,
    pub classification: Option<ByteBuffer>,
}

/// 🎥️ Recovered camera trajectory across all registered frames.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct CameraTrajectory {
    #[dsl(table)]
    pub poses: Vec<CameraPosePreview>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum TrackClass {
    #[default]
    Static,
    Moving,
}

/// 🏃️ A distilled summary of one `remodeling_motion` track — full per-frame keyframe paths
/// (`Track2d`/`Trajectory3d` in the motion crate) are plugin-runtime scratch, not durable document
/// state; only enough is kept here to list/label tracks and drive the report table.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct MotionTrackSummary {
    pub id: String,
    pub length: u32,
    pub class: TrackClass,
    #[dsl(unit = "m/s")]
    pub mean_speed_m_s: f32,
}

/// 🗺️ Georeferenced raster products, each stored as a pixel `ImageAsset` (DSM/DTM as 16-bit-encoded
/// PNG, ortho as an RGB PNG) rather than an embedded float grid — rasters are pixels, so they follow
/// the same persistence rule as every other image in this document instead of a bespoke height-grid
/// packed-array shape.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct GeoProducts {
    pub dsm_asset_id: Option<String>,
    pub dtm_asset_id: Option<String>,
    pub ortho_asset_id: Option<String>,
}

/// ✅️ A plain-JSON mirror of the QC-relevant fields of `remodeling_geo::QualityReport`, plus the
/// watertight snapshot (mirroring `QualityReport.watertight: Option<WatertightReport>`) and a few
/// cheap scalar summaries (`remodeling_engine` computes these once at the end of a run; the underlying
/// per-camera covariance/per-point-sigma arrays and density/overlap rasters stay plugin-runtime).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct QcReportSnapshot {
    pub reprojection_rms_px: f64,
    pub gcp_checkpoint_rmse: Option<f64>,
    #[dsl(block)]
    pub watertight: Option<WatertightReportSnapshot>,
    pub mean_track_length: f32,
    pub registered_frame_ratio: f32,
    pub dense_coverage_ratio: f32,
    pub warnings: Vec<String>,
}

/// 📦️ Everything a completed (or partially completed) reconstruction run has produced so far.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", default)]
pub struct ReconstructionResults {
    #[dsl(block)]
    pub sparse: Option<SparseCloud>,
    #[dsl(block)]
    pub dense: Option<DenseCloud>,
    #[dsl(block)]
    pub mesh: RemodelingMesh,
    #[dsl(block)]
    pub trajectory: Option<CameraTrajectory>,
    #[dsl(table)]
    pub tracks: Vec<MotionTrackSummary>,
    #[dsl(block)]
    pub geo: Option<GeoProducts>,
    #[dsl(block)]
    pub qc: Option<QcReportSnapshot>,
}

/// 🌱️ An empty scene seeded with a placeholder box mesh, so the 3D editor/preview always has
/// something to render before any media has been imported/reconstructed.
pub fn default_remodeling_scene() -> RemodelingSnapshot {
    RemodelingSnapshot {
        schema: REMODELING_DOCUMENT_SCHEMA.into(),
        id: "remodeling".into(),
        streams: Vec::new(),
        assets: BTreeMap::new(),
        durable_artifacts: RemodelingDurableArtifactStore::new(),
        calibration: CalibrationState::default(),
        params: ReconstructionParams::default(),
        gcps: Vec::new(),
        results: ReconstructionResults { mesh: RemodelingMesh { mesh: placeholder_remodeling_mesh_handle(), source: MeshSource::Placeholder, ..RemodelingMesh::default() }, ..ReconstructionResults::default() },
    }
}
//#endregion 🔖️Domain

//#region 🔖️CanonicalOrder
/// 🔢️ Insertion point that keeps a keyed collection in ascending key order. Every collection this
/// vocabulary addresses by key — `streams` and `gcps` by `id`, `calibration.cameras` by `id`,
/// `calibration.rig` by `camera_id`, a stream's `frames` by `(index, asset_id)`, a GCP's
/// `observations` by `(stream_id, frame_index)` — is a canonically ordered document invariant, so a
/// `create-*`/`add-*` puts its member exactly where a later `delete-*`/`remove-*` took it from and
/// `inverse(m, before)` applied to `after` restores `before` member positions included.
pub fn ordered_index<T, K: Ord>(items: &[T], key: &K, key_of: impl Fn(&T) -> K) -> usize {
    items.partition_point(|item| key_of(item) < *key)
}
//#endregion 🔖️CanonicalOrder

//#region 🔖️Leaves
/// 🧾️ The snapshot facet's five format leaves.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🔖️Leaves
