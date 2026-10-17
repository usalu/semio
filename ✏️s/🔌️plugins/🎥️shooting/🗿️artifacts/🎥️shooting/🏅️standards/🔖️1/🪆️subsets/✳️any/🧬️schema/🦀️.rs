//! 🧬️ Shooting artifact schema — every field of the artifact with its state class.

use crate::{ShootingEmblemChild, ShootingSnapshot};
use schema::ArtifactSchema;


//#region 🔖️Artifact
/// 🧬️ shooting document artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub assets: Vec<ShootingAsset>,
    #[state(artifact)]
    pub saved_cameras: Vec<ShootingSavedCamera>,
    #[state(artifact)]
    pub scene: ShootingSceneLighting,
    #[state(artifact)]
    pub shots: Vec<ShootingShot>,
    #[state(artifact)]
    pub active_shot_id: String,
    #[state(artifact)]
    pub active_asset_id: String,
    /// 🕸️ Composed `s.stdio.semio.image` child mirror — see `ShootingSnapshot::emblem`'s doc comment.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub emblem: Option<ShootingEmblemChild>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for ShootingArtifact {
    fn default() -> Self {
        Self { schema: crate::SHOOTING_DOCUMENT_SCHEMA.into(), assets: Vec::new(), saved_cameras: Vec::new(), scene: ShootingSceneLighting::default(), shots: Vec::new(), active_shot_id: String::new(), active_asset_id: String::new(), emblem: None }
    }
}

impl ShootingArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> ShootingSnapshot {
        ShootingSnapshot {
            schema: self.schema.clone(),
            assets: self.assets.clone(),
            saved_cameras: self.saved_cameras.clone(),
            scene: self.scene.clone(),
            shots: self.shots.clone(),
            active_shot_id: self.active_shot_id.clone(),
            active_asset_id: self.active_asset_id.clone(),
            emblem: self.emblem.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: ShootingSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            assets: snapshot.assets,
            saved_cameras: snapshot.saved_cameras,
            scene: snapshot.scene,
            shots: snapshot.shots,
            active_shot_id: snapshot.active_shot_id,
            active_asset_id: snapshot.active_asset_id,
            emblem: snapshot.emblem,

        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: ShootingSnapshot) {
        self.schema = snapshot.schema;
        self.assets = snapshot.assets;
        self.saved_cameras = snapshot.saved_cameras;
        self.scene = snapshot.scene;
        self.shots = snapshot.shots;
        self.active_shot_id = snapshot.active_shot_id;
        self.active_asset_id = snapshot.active_asset_id;
        self.emblem = snapshot.emblem;
    }
}
//#endregion 🔖️Conversions



//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.shooting.shooting` — twenty handcrafted schema leaves.
pub fn shooting_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.shooting.shooting",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"), typescript: include_str!("🔺️diff/🟦️.ts"), graphql: include_str!("🔺️diff/🔗️.graphql"), json_schema: include_str!("🔺️diff/🔣️.json"), proto: include_str!("🔺️diff/🛰️.proto")
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
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔁️Re-exports
pub use crate::ShootingAsset;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::ShootingCamera;
pub use crate::ShootingSavedCamera;
pub use crate::ShootingSceneLighting;
pub use crate::ShootingShot;
//#endregion 🔁️Re-exports

//#region 🧩️ArtifactItems8
pub mod artifact_items_8 {
use crate::{ShootingAsset, ShootingCamera, ShootingSavedCamera, ShootingShot};
use protocol::Patchable;

//#region 🔖️CollectionSupport
/// 🎯️ An explicitly assigned optional value: wraps the value so assigning `None` stays distinct from leaving the field untouched on the wire (a bare nested `Option` collapses both to `null`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
pub struct ShootingAssigned<T> {
    pub value: T,
}

impl<T> ShootingAssigned<T> {
    /// 🏗️ Wraps the assigned value.
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

/// 🩹 Sparse asset field patch; `orientation`/`scale` are assignable so an absent optional value is restorable.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", default)]
pub struct ShootingAssetPatch {
    pub name: Option<String>,
    pub url: Option<String>,
    pub format: Option<String>,
    pub origin: Option<[f64; 3]>,
    pub orientation: Option<ShootingAssigned<Option<[f64; 4]>>>,
    pub scale: Option<ShootingAssigned<Option<[f64; 3]>>>,
}

impl Patchable<ShootingAssetPatch> for ShootingAsset {
    fn apply_patch(&mut self, patch: &ShootingAssetPatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(url) = &patch.url {
            self.url = url.clone();
        }
        if let Some(format) = &patch.format {
            self.format = format.clone();
        }
        if let Some(origin) = patch.origin {
            self.origin = origin;
        }
        if let Some(orientation) = &patch.orientation {
            self.orientation = orientation.value;
        }
        if let Some(scale) = &patch.scale {
            self.scale = scale.value;
        }
    }
}

/// 🩹 Sparse shot field patch.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", default)]
pub struct ShootingShotPatch {
    pub label: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub format: Option<String>,
    pub shape: Option<String>,
    pub background: Option<ShootingAssigned<Option<String>>>,
    pub camera_id: Option<ShootingAssigned<Option<String>>>,
}

impl Patchable<ShootingShotPatch> for ShootingShot {
    fn apply_patch(&mut self, patch: &ShootingShotPatch) {
        if let Some(label) = &patch.label {
            self.label = label.clone();
        }
        if let Some(width) = patch.width {
            self.width = width;
        }
        if let Some(height) = patch.height {
            self.height = height;
        }
        if let Some(format) = &patch.format {
            self.format = format.clone();
        }
        if let Some(shape) = &patch.shape {
            self.shape = shape.clone();
        }
        if let Some(background) = &patch.background {
            self.background = background.value.clone();
        }
        if let Some(camera_id) = &patch.camera_id {
            self.camera_id = camera_id.value.clone();
        }
    }
}

/// 🩹 Sparse saved-camera field patch.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", default)]
pub struct ShootingSavedCameraPatch {
    pub label: Option<String>,
    pub camera: Option<ShootingCamera>,
}

impl Patchable<ShootingSavedCameraPatch> for ShootingSavedCamera {
    fn apply_patch(&mut self, patch: &ShootingSavedCameraPatch) {
        if let Some(label) = &patch.label {
            self.label = label.clone();
        }
        if let Some(camera) = &patch.camera {
            self.camera = camera.clone();
        }
    }
}

/// 🩹 Sparse scene-lighting patch: one optional field per scene scalar, so a scene kind names only the value it owns.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", default)]
pub struct ShootingScenePatch {
    pub background: Option<String>,
    pub sun_enabled: Option<bool>,
    pub sun_azimuth: Option<f64>,
    pub sun_elevation: Option<f64>,
    pub sun_intensity: Option<f64>,
    pub sun_color: Option<String>,
    pub ambient_intensity: Option<f64>,
    pub ambient_color: Option<String>,
    pub shadow_enabled: Option<bool>,
    pub shadow_opacity: Option<f64>,
    pub shadow_softness: Option<f64>,
    pub material_color: Option<String>,
    pub material_metalness: Option<f64>,
    pub material_roughness: Option<f64>,
    pub material_emissive: Option<String>,
    pub material_emissive_intensity: Option<f64>,
    pub material_stroke: Option<String>,
}
//#endregion 🔖️CollectionSupport
}
//#endregion 🧩️ArtifactItems8
