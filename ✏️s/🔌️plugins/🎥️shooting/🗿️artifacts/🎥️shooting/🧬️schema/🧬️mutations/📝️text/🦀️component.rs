//! ⚡️ Shooting artifact — OpText/OpBinary codecs + grammar for serializing `ShootingMutation`.
//! Mutation apply/inverse live in `🧬️mutations`.

pub use crate::artifacts::shooting::schema::mutations::{
    apply_shooting_mutation, inverse_shooting_mutation, ShootingMutation,
};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::shooting::{ShootingAsset, ShootingAssetPatch, ShootingCamera, ShootingSnapshot, ShootingSavedCamera, ShootingSavedCameraPatch, ShootingScenePatch, ShootingShot, ShootingShotPatch};
use protocol::CollectionMutation;

//#region 🔖️OpText
/// 🌿️ `ShootingSnapshot`'s three collections (`assets`/`shots`/`saved_cameras`) are `Vec<T>` of a
/// plain `#[derive(dsl::DslRecord)]` struct, but `#[dsl(statements, block)]` needs its element type
/// to implement `dsl::DslVariants` (enum-only) — these one-variant newtype-tuple wrappers close
/// that gap without duplicating any field: the newtype-tuple codegen delegates entirely to the
/// wrapped type's own `RecordSpec` (including its `#[dsl(keyword = "...")]`), so `ShootingAsset`
/// prints/parses byte-identically whether reached through `ShootingAssetNode` or on its own.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum ShootingAssetNode {
    Asset(ShootingAsset),
}

#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum ShootingShotNode {
    Shot(ShootingShot),
}

/// 🔡️ The variant key must stay textually in sync with `ShootingSavedCamera`'s own
/// `#[dsl(keyword = "saved-camera")]` — this newtype delegates its `RecordSpec` entirely to the
/// wrapped type (see the doc comment above), so the `Shape::Statements` dispatcher's outer tag
/// (this key) and `parse_record_body`'s inner leading-keyword check (the struct's `keyword`) must
/// agree, or parsing that variant would require two different tokens where only one is written.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum ShootingSavedCameraNode {
    #[dsl(key = "saved-camera")]
    SavedCamera(ShootingSavedCamera),
}

/// 📄️ Op-local mirror of `ShootingSnapshot` for `SetSnapshot`'s `#[dsl(block)]` payload — reuses the
/// derive-generated shape, independent of the artifact's own fixture DSL mirror.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(extension = "shooting")]
#[dsl(layout = "lines")]
struct ShootingSnapshotDsl {
    schema: String,
    active_shot_id: String,
    active_asset_id: String,
    #[dsl(block)]
    scene: crate::artifacts::shooting::ShootingSceneLighting,
    #[dsl(table)]
    assets: Vec<ShootingAsset>,
    #[dsl(table)]
    shots: Vec<ShootingShot>,
    #[dsl(table)]
    saved_cameras: Vec<ShootingSavedCamera>,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for ShootingSnapshotDsl {
    const EXTENSION: &'static str = "shooting";
    fn envelope_id() -> &'static str { "shooting" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for ShootingSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs




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

fn shooting_snapshot_from_dsl(dsl_fixture: ShootingSnapshotDsl) -> ShootingSnapshot {
    ShootingSnapshot {
        schema: dsl_fixture.schema,
        assets: dsl_fixture.assets,
        saved_cameras: dsl_fixture.saved_cameras,
        scene: dsl_fixture.scene,
        shots: dsl_fixture.shots,
        active_shot_id: dsl_fixture.active_shot_id,
        active_asset_id: dsl_fixture.active_asset_id,
    }
}

/// ⚡️ Local mirror of `ShootingMutation` — the real enum's `Assets`/`Shots`/`SavedCameras` variants
/// each wrap a single `protocol::CollectionMutation<..>` field, a foreign generic type (orphan rule:
/// can't `impl dsl::DslField` for it here) that also isn't the tagged-enum shape `#[derive(dsl::DslEnum)]`
/// needs anyway — so each `CollectionMutation` variant (`Add`/`Remove`/`Move`/`Patch`) is flattened
/// into its own DSL-facing operation variant instead, exactly the `imperative::ImperativeOperationDsl`
/// idiom.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
#[allow(clippy::large_enum_variant, reason = "mirror-only enum used solely at the print_op/parse_op boundary, never stored or passed around")]
enum ShootingMutationDsl {
    AssetsAdd {
        index: usize,
        #[dsl(statements)]
        item: Box<ShootingAssetNode>,
    },
    AssetsRemove {
        id: String,
    },
    AssetsMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    AssetsPatch {
        id: String,
        #[dsl(block)]
        patch: ShootingAssetPatch,
    },
    ShotsAdd {
        index: usize,
        #[dsl(statements)]
        item: Box<ShootingShotNode>,
    },
    ShotsRemove {
        id: String,
    },
    ShotsMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    ShotsPatch {
        id: String,
        #[dsl(block)]
        patch: ShootingShotPatch,
    },
    SavedCamerasAdd {
        index: usize,
        #[dsl(statements)]
        item: Box<ShootingSavedCameraNode>,
    },
    SavedCamerasRemove {
        id: String,
    },
    SavedCamerasMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    SavedCamerasPatch {
        id: String,
        #[dsl(block)]
        patch: ShootingSavedCameraPatch,
    },
    #[dsl(key = "active-shot")]
    SetActiveShot {
        shot_id: Option<String>,
    },
    #[dsl(key = "active-asset")]
    SetActiveAsset {
        asset_id: Option<String>,
    },
    #[dsl(key = "shot-camera")]
    SetShotCamera {
        shot_id: String,
        #[dsl(block)]
        camera: ShootingCamera,
    },
    #[dsl(key = "scene-patch")]
    PatchScene {
        #[dsl(block)]
        patch: ShootingScenePatch,
    },
    #[dsl(key = "translate")]
    TranslateAssets {
        asset_ids: Vec<String>,
        dx: f64,
        dy: f64,
        dz: f64,
    },
    #[dsl(key = "rotate")]
    RotateAssets {
        asset_ids: Vec<String>,
        ax: f64,
        ay: f64,
        az: f64,
        angle: f64,
    },
    #[dsl(key = "scale")]
    ScaleAssets {
        asset_ids: Vec<String>,
        sx: f64,
        sy: f64,
        sz: f64,
    },
    #[dsl(key = "fixture")]
    SetSnapshot {
        #[dsl(block)]
        snapshot: ShootingSnapshotDsl,
    },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for ShootingMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for ShootingMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




fn shooting_mutation_to_dsl(operation: &ShootingMutation) -> ShootingMutationDsl {
    match operation {
        ShootingMutation::Assets(op) => match op {
            CollectionMutation::Add { index: at, item } => ShootingMutationDsl::AssetsAdd { index: *at, item: Box::new(ShootingAssetNode::Asset(item.clone())) },
            CollectionMutation::Remove { id } => ShootingMutationDsl::AssetsRemove { id: id.clone() },
            CollectionMutation::Move { id, to_index: to } => ShootingMutationDsl::AssetsMove { id: id.clone(), to_index: *to },
            CollectionMutation::Patch { id, patch } => ShootingMutationDsl::AssetsPatch { id: id.clone(), patch: patch.clone() },
        },
        ShootingMutation::Shots(op) => match op {
            CollectionMutation::Add { index: at, item } => ShootingMutationDsl::ShotsAdd { index: *at, item: Box::new(ShootingShotNode::Shot(item.clone())) },
            CollectionMutation::Remove { id } => ShootingMutationDsl::ShotsRemove { id: id.clone() },
            CollectionMutation::Move { id, to_index: to } => ShootingMutationDsl::ShotsMove { id: id.clone(), to_index: *to },
            CollectionMutation::Patch { id, patch } => ShootingMutationDsl::ShotsPatch { id: id.clone(), patch: patch.clone() },
        },
        ShootingMutation::SavedCameras(op) => match op {
            CollectionMutation::Add { index: at, item } => ShootingMutationDsl::SavedCamerasAdd { index: *at, item: Box::new(ShootingSavedCameraNode::SavedCamera(item.clone())) },
            CollectionMutation::Remove { id } => ShootingMutationDsl::SavedCamerasRemove { id: id.clone() },
            CollectionMutation::Move { id, to_index: to } => ShootingMutationDsl::SavedCamerasMove { id: id.clone(), to_index: *to },
            CollectionMutation::Patch { id, patch } => ShootingMutationDsl::SavedCamerasPatch { id: id.clone(), patch: patch.clone() },
        },
        ShootingMutation::SetActiveShot { shot_id } => ShootingMutationDsl::SetActiveShot { shot_id: shot_id.clone() },
        ShootingMutation::SetActiveAsset { asset_id } => ShootingMutationDsl::SetActiveAsset { asset_id: asset_id.clone() },
        ShootingMutation::SetShotCamera { shot_id, camera } => ShootingMutationDsl::SetShotCamera { shot_id: shot_id.clone(), camera: camera.clone() },
        ShootingMutation::PatchScene { patch } => ShootingMutationDsl::PatchScene { patch: patch.clone() },
        ShootingMutation::TranslateAssets { asset_ids, dx, dy, dz } => ShootingMutationDsl::TranslateAssets { asset_ids: asset_ids.clone(), dx: *dx, dy: *dy, dz: *dz },
        ShootingMutation::RotateAssets { asset_ids, ax, ay, az, angle } => ShootingMutationDsl::RotateAssets { asset_ids: asset_ids.clone(), ax: *ax, ay: *ay, az: *az, angle: *angle },
        ShootingMutation::ScaleAssets { asset_ids, sx, sy, sz } => ShootingMutationDsl::ScaleAssets { asset_ids: asset_ids.clone(), sx: *sx, sy: *sy, sz: *sz },
        ShootingMutation::SetSnapshot { snapshot } => ShootingMutationDsl::SetSnapshot { snapshot: shooting_snapshot_to_dsl(snapshot) },
    }
}

fn shooting_mutation_from_dsl(dsl_op: ShootingMutationDsl) -> ShootingMutation {
    match dsl_op {
        ShootingMutationDsl::AssetsAdd { index, item } => {
            let ShootingAssetNode::Asset(asset) = *item;
            ShootingMutation::Assets(CollectionMutation::Add { index: index, item: asset })
        }
        ShootingMutationDsl::AssetsRemove { id } => ShootingMutation::Assets(CollectionMutation::Remove { id }),
        ShootingMutationDsl::AssetsMove { id, to_index } => ShootingMutation::Assets(CollectionMutation::Move { id, to_index: to_index }),
        ShootingMutationDsl::AssetsPatch { id, patch } => ShootingMutation::Assets(CollectionMutation::Patch { id, patch }),
        ShootingMutationDsl::ShotsAdd { index, item } => {
            let ShootingShotNode::Shot(shot) = *item;
            ShootingMutation::Shots(CollectionMutation::Add { index: index, item: shot })
        }
        ShootingMutationDsl::ShotsRemove { id } => ShootingMutation::Shots(CollectionMutation::Remove { id }),
        ShootingMutationDsl::ShotsMove { id, to_index } => ShootingMutation::Shots(CollectionMutation::Move { id, to_index: to_index }),
        ShootingMutationDsl::ShotsPatch { id, patch } => ShootingMutation::Shots(CollectionMutation::Patch { id, patch }),
        ShootingMutationDsl::SavedCamerasAdd { index, item } => {
            let ShootingSavedCameraNode::SavedCamera(entry) = *item;
            ShootingMutation::SavedCameras(CollectionMutation::Add { index: index, item: entry })
        }
        ShootingMutationDsl::SavedCamerasRemove { id } => ShootingMutation::SavedCameras(CollectionMutation::Remove { id }),
        ShootingMutationDsl::SavedCamerasMove { id, to_index } => ShootingMutation::SavedCameras(CollectionMutation::Move { id, to_index: to_index }),
        ShootingMutationDsl::SavedCamerasPatch { id, patch } => ShootingMutation::SavedCameras(CollectionMutation::Patch { id, patch }),
        ShootingMutationDsl::SetActiveShot { shot_id } => ShootingMutation::SetActiveShot { shot_id },
        ShootingMutationDsl::SetActiveAsset { asset_id } => ShootingMutation::SetActiveAsset { asset_id },
        ShootingMutationDsl::SetShotCamera { shot_id, camera } => ShootingMutation::SetShotCamera { shot_id, camera },
        ShootingMutationDsl::PatchScene { patch } => ShootingMutation::PatchScene { patch },
        ShootingMutationDsl::TranslateAssets { asset_ids, dx, dy, dz } => ShootingMutation::TranslateAssets { asset_ids, dx, dy, dz },
        ShootingMutationDsl::RotateAssets { asset_ids, ax, ay, az, angle } => ShootingMutation::RotateAssets { asset_ids, ax, ay, az, angle },
        ShootingMutationDsl::ScaleAssets { asset_ids, sx, sy, sz } => ShootingMutation::ScaleAssets { asset_ids, sx, sy, sz },
        ShootingMutationDsl::SetSnapshot { snapshot } => ShootingMutation::SetSnapshot { snapshot: shooting_snapshot_from_dsl(snapshot) },
    }
}

impl protocol::OpText for ShootingMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(shooting_mutation_from_dsl(<ShootingMutationDsl as protocol::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <ShootingMutationDsl as protocol::OpText>::print_op(&shooting_mutation_to_dsl(self))
    }
}

/// 🎞️ Binary mirror of the `OpText` bridge above — `ShootingMutationDsl` already derives `OpBinary`
/// via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for ShootingMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        shooting_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(shooting_mutation_from_dsl(ShootingMutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

