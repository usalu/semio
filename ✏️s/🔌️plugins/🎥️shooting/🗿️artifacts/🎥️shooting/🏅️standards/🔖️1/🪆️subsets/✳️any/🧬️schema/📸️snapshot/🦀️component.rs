//! 🧬️ Shooting snapshot schema — artifact-lane fields only.
//!
//! `emblem: Option<ShootingEmblemChild>` is the ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`
//! composed `s.stdio.semio.image` child (see `🗿️artifacts/🎥️shooting/🦀️component.rs`'s `🔖️Composition`
//! region for the full design/converters). `store::ArtifactChild<S>` has no `dsl::DslField` impl (the
//! same wall every wave-4 exemplar hit), so it cannot join `ShootingSnapshotDsl`'s `#[derive(dsl::DslRecord)]`
//! fields directly — it is instead carried through that derive as an opaque hex/bracket-encoded
//! `Option<String>` (`🔖️ChildCodecPrimitives` below), letting the REST of the document (the readable
//! `assets`/`shots`/`savedCameras` table grammar) keep its existing derive-generated codec untouched.

use crate::artifacts::shooting::{ShootingAsset, ShootingEmblemChild, ShootingSavedCamera, ShootingSceneLighting, ShootingShot, SHOOTING_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted shooting document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub assets: Vec<ShootingAsset>,
    #[state(artifact)]
    #[serde(default)]
    pub saved_cameras: Vec<ShootingSavedCamera>,
    #[state(artifact)]
    #[serde(default)]
    pub scene: ShootingSceneLighting,
    #[state(artifact)]
    #[serde(default)]
    pub shots: Vec<ShootingShot>,
    #[state(artifact)]
    #[serde(default)]
    pub active_shot_id: String,
    #[state(artifact)]
    #[serde(default)]
    pub active_asset_id: String,
    /// 🕸️ Composed `s.stdio.semio.image` child — the scene's emblem overlay, genuinely absent for
    /// most documents (no default fixture sets one). See `🔖️Composition` in the artifact root.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.image")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emblem: Option<ShootingEmblemChild>,
}

impl Default for ShootingSnapshot {
    fn default() -> Self {
        Self { schema: SHOOTING_DOCUMENT_SCHEMA.into(), assets: Vec::new(), saved_cameras: Vec::new(), scene: ShootingSceneLighting::default(), shots: Vec::new(), active_shot_id: String::new(), active_asset_id: String::new(), emblem: None }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️DslMirror
/// 📄️ Local mirror of [`ShootingSnapshot`] — the real struct's `assets: Vec<ShootingAsset>` (etc.)
/// can't carry `#[dsl(statements, block)]` directly (that needs `Vec<T: DslVariants>`, an enum bound;
/// `ShootingAsset` is a plain record), so this document-shaped twin swaps each collection's element type
/// for its wrapper node and converts at the boundary.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(extension = "shooting")]
#[dsl(layout = "lines")]
struct ShootingSnapshotDsl {
    schema: String,
    active_shot_id: String,
    active_asset_id: String,
    #[dsl(block)]
    scene: ShootingSceneLighting,
    #[dsl(table)]
    assets: Vec<ShootingAsset>,
    #[dsl(table)]
    shots: Vec<ShootingShot>,
    #[dsl(table)]
    saved_cameras: Vec<ShootingSavedCamera>,
    /// 🕸️ Opaque hex/bracket-encoded `emblem: Option<ShootingEmblemChild>` handle — see this file's
    /// module doc comment and `🔖️ChildCodecPrimitives` below.
    emblem: Option<String>,
}

async fn shooting_snapshot_to_dsl(snapshot: &ShootingSnapshot) -> ShootingSnapshotDsl {
    ShootingSnapshotDsl {
        schema: snapshot.schema.clone(),
        active_shot_id: snapshot.active_shot_id.clone(),
        active_asset_id: snapshot.active_asset_id.clone(),
        scene: snapshot.scene.clone(),
        assets: snapshot.assets.clone(),
        shots: snapshot.shots.clone(),
        saved_cameras: snapshot.saved_cameras.clone(),
        emblem: snapshot.emblem.as_ref().map(enc_child),
    }
}

async fn shooting_snapshot_from_dsl(dsl_snapshot: ShootingSnapshotDsl) -> Result<ShootingSnapshot, String> {
    let emblem = dsl_snapshot.emblem.as_deref().map(dec_child).transpose()?;
    Ok(ShootingSnapshot {
        schema: dsl_snapshot.schema,
        assets: dsl_snapshot.assets,
        saved_cameras: dsl_snapshot.saved_cameras,
        scene: dsl_snapshot.scene,
        shots: dsl_snapshot.shots,
        active_shot_id: dsl_snapshot.active_shot_id,
        active_asset_id: dsl_snapshot.active_asset_id,
        emblem,
    })
}
//#endregion 🔖️DslMirror

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Hex/bracket child-handle codec — same convention every wave-4 exemplar uses
/// (`process3d`/`gismap`'s own `enc_child`/`dec_child`), reduced to exactly the two strings a
/// `ShootingEmblemChild` carries (`child_id`, `target` as its URI form) and packed into ONE opaque
/// string so it can ride through `ShootingSnapshotDsl`'s existing derive-generated `Option<String>`
/// field handling untouched.
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn enc_hex_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
async fn dec_hex_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
async fn enc_child(c: &ShootingEmblemChild) -> String {
    format!("[{},{}]", enc_hex_str(&c.child_id), enc_hex_str(&c.target.to_uri()))
}
async fn dec_child(s: &str) -> Result<ShootingEmblemChild, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("child handle: expected [child_id,target], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    let target_uri = dec_hex_str(target)?;
    let target = store::os_io::ArtifactRef::parse_uri(&target_uri).map_err(|e| e.to_string())?;
    Ok(store::ArtifactChild::new(dec_hex_str(child_id)?, target))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for ShootingSnapshotDsl {
    const EXTENSION: &'static str = "shooting";
    async fn envelope_id() -> &'static str {
        "shooting.shooting"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ShootingSnapshotDsl {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ArtifactDsl for ShootingSnapshot {
    const EXTENSION: &'static str = "shooting";
    async fn envelope_id() -> &'static str {
        "shooting.shooting"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <ShootingSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?;
        shooting_snapshot_from_dsl(parsed).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        <ShootingSnapshotDsl as store::ArtifactDsl>::print_dsl(&shooting_snapshot_to_dsl(self))
    }
}

impl store::ArtifactPack for ShootingSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <ShootingSnapshotDsl as store::ArtifactPack>::encode_pack_with(&shooting_snapshot_to_dsl(self), options)
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <ShootingSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        shooting_snapshot_from_dsl(parsed).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
