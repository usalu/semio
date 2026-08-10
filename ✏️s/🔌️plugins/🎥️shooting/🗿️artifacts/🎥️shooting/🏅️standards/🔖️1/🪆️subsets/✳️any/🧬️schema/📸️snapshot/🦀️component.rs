//! 🧬️ Shooting snapshot schema — persistent fields only.

use crate::artifacts::shooting::{
    ShootingAsset, ShootingSavedCamera, ShootingSceneLighting, ShootingShot, SHOOTING_DOCUMENT_SCHEMA,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted shooting document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub assets: Vec<ShootingAsset>,
    #[state(persistent)]
    #[serde(default)]
    pub saved_cameras: Vec<ShootingSavedCamera>,
    #[state(persistent)]
    #[serde(default)]
    pub scene: ShootingSceneLighting,
    #[state(persistent)]
    #[serde(default)]
    pub shots: Vec<ShootingShot>,
    #[state(persistent)]
    #[serde(default)]
    pub active_shot_id: String,
    #[state(persistent)]
    #[serde(default)]
    pub active_asset_id: String,
}

impl Default for ShootingSnapshot {
    fn default() -> Self {
        Self {
            schema: SHOOTING_DOCUMENT_SCHEMA.into(),
            assets: Vec::new(),
            saved_cameras: Vec::new(),
            scene: ShootingSceneLighting::default(),
            shots: Vec::new(),
            active_shot_id: String::new(),
            active_asset_id: String::new(),
        }
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
}

fn shooting_snapshot_to_dsl(snapshot: &ShootingSnapshot) -> ShootingSnapshotDsl {
    ShootingSnapshotDsl {
        schema: snapshot.schema.clone(),
        active_shot_id: snapshot.active_shot_id.clone(),
        active_asset_id: snapshot.active_asset_id.clone(),
        scene: snapshot.scene.clone(),
        assets: snapshot.assets.clone(),
        shots: snapshot.shots.clone(),
        saved_cameras: snapshot.saved_cameras.clone(),
    }
}

fn shooting_snapshot_from_dsl(dsl_snapshot: ShootingSnapshotDsl) -> ShootingSnapshot {
    ShootingSnapshot {
        schema: dsl_snapshot.schema,
        assets: dsl_snapshot.assets,
        saved_cameras: dsl_snapshot.saved_cameras,
        scene: dsl_snapshot.scene,
        shots: dsl_snapshot.shots,
        active_shot_id: dsl_snapshot.active_shot_id,
        active_asset_id: dsl_snapshot.active_asset_id,
    }
}
//#endregion 🔖️DslMirror

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for ShootingSnapshotDsl {
    const EXTENSION: &'static str = "shooting";
    fn envelope_id() -> &'static str {
        "shooting.shooting"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions {
                limits: dsl::Limits::default(),
                mode: dsl::SourceMode::Document,
            },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ShootingSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) =
            store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ArtifactDsl for ShootingSnapshot {
    const EXTENSION: &'static str = "shooting";
    fn envelope_id() -> &'static str {
        "shooting.shooting"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <ShootingSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?;
        Ok(shooting_snapshot_from_dsl(parsed))
    }
    fn print_dsl(&self) -> String {
        <ShootingSnapshotDsl as store::ArtifactDsl>::print_dsl(&shooting_snapshot_to_dsl(self))
    }
}

impl store::ArtifactPack for ShootingSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <ShootingSnapshotDsl as store::ArtifactPack>::encode_pack_with(&shooting_snapshot_to_dsl(self), options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <ShootingSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        Ok(shooting_snapshot_from_dsl(parsed))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
