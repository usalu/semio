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
    /// 🕸️ Composed `stdio.semio.image` child mirror — see `ShootingSnapshot::emblem`'s doc comment.
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
pub use crate::ShootingDrawing;
pub use crate::ShootingSavedCamera;
pub use crate::ShootingSceneLighting;
pub use crate::ShootingShot;
//#endregion 🔁️Re-exports

//#region 🧩️ArtifactItems8
pub mod artifact_items_8 {
use crate::{ShootingAsset, ShootingCamera, ShootingDrawing, ShootingSavedCamera, ShootingShot};
use protocol::Patchable;

//#region 🔖️CollectionSupport
/// 🎯️ An explicitly assigned optional value: wraps the value so assigning `None` stays distinct from leaving the field untouched on the wire (a bare nested `Option` collapses both to `null`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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

/// ✒️ Independent edits to the controls of one drawing shot.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct ShootingDrawingPatch {
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<ShootingAssigned<Option<String>>>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<f64>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub visible_lines: Option<bool>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub hidden_lines: Option<bool>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub visible_color: Option<String>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub hidden_color: Option<String>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub visible_width: Option<f64>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub hidden_width: Option<f64>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub line_easing: Option<f64>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub merge_lines: Option<bool>,
    #[cfg_attr(test, serde(skip_serializing_if = "is_false"))]
    #[value(skip_serializing_if = "is_false")]
    pub restore_absent: bool,
}

fn is_false(value: &bool) -> bool {
    !value
}

impl ShootingDrawingPatch {
    /// 🖌️ Applies only the supplied controls to the currently retained settings.
    pub fn apply(&self, base: Option<&ShootingDrawing>) -> Option<ShootingDrawing> {
        let defaults = ShootingDrawing::default();
        let current = base.unwrap_or(&defaults);
        let drawing = ShootingDrawing {
            background: self.background.unwrap_or(current.background),
            fill: self.fill.unwrap_or(current.fill),
            fill_color: self.fill_color.as_ref().map_or_else(|| current.fill_color.clone(), |patch| patch.value.clone()),
            fill_opacity: self.fill_opacity.unwrap_or(current.fill_opacity),
            visible_lines: self.visible_lines.unwrap_or(current.visible_lines),
            hidden_lines: self.hidden_lines.unwrap_or(current.hidden_lines),
            visible_color: self.visible_color.clone().unwrap_or_else(|| current.visible_color.clone()),
            hidden_color: self.hidden_color.clone().unwrap_or_else(|| current.hidden_color.clone()),
            visible_width: self.visible_width.unwrap_or(current.visible_width),
            hidden_width: self.hidden_width.unwrap_or(current.hidden_width),
            line_easing: self.line_easing.unwrap_or(current.line_easing),
            merge_lines: self.merge_lines.unwrap_or(current.merge_lines),
        };
        (!self.restore_absent || drawing != defaults).then_some(drawing)
    }

    /// ↩️ Restores only the supplied controls from the pre-edit settings.
    pub fn inverse(&self, base: Option<&ShootingDrawing>) -> Self {
        let defaults = ShootingDrawing::default();
        let current = base.unwrap_or(&defaults);
        Self {
            background: self.background.as_ref().map(|_| current.background),
            fill: self.fill.as_ref().map(|_| current.fill),
            fill_color: self.fill_color.as_ref().map(|_| ShootingAssigned::new(current.fill_color.clone())),
            fill_opacity: self.fill_opacity.as_ref().map(|_| current.fill_opacity),
            visible_lines: self.visible_lines.as_ref().map(|_| current.visible_lines),
            hidden_lines: self.hidden_lines.as_ref().map(|_| current.hidden_lines),
            visible_color: self.visible_color.as_ref().map(|_| current.visible_color.clone()),
            hidden_color: self.hidden_color.as_ref().map(|_| current.hidden_color.clone()),
            visible_width: self.visible_width.as_ref().map(|_| current.visible_width),
            hidden_width: self.hidden_width.as_ref().map(|_| current.hidden_width),
            line_easing: self.line_easing.as_ref().map(|_| current.line_easing),
            merge_lines: self.merge_lines.as_ref().map(|_| current.merge_lines),
            restore_absent: base.is_none(),
        }
    }

    /// 🔀️ Combines successive edits without replacing unrelated controls.
    pub fn merge(&mut self, later: Self) {
        *self = Self {
            background: later.background.or_else(|| self.background.take()),
            fill: later.fill.or_else(|| self.fill.take()),
            fill_color: later.fill_color.or_else(|| self.fill_color.take()),
            fill_opacity: later.fill_opacity.or_else(|| self.fill_opacity.take()),
            visible_lines: later.visible_lines.or_else(|| self.visible_lines.take()),
            hidden_lines: later.hidden_lines.or_else(|| self.hidden_lines.take()),
            visible_color: later.visible_color.or_else(|| self.visible_color.take()),
            hidden_color: later.hidden_color.or_else(|| self.hidden_color.take()),
            visible_width: later.visible_width.or_else(|| self.visible_width.take()),
            hidden_width: later.hidden_width.or_else(|| self.hidden_width.take()),
            line_easing: later.line_easing.or_else(|| self.line_easing.take()),
            merge_lines: later.merge_lines.or_else(|| self.merge_lines.take()),
            restore_absent: later.restore_absent,
        };
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
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub drawing: Option<ShootingDrawingPatch>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub auto_fit: Option<ShootingAssigned<Option<bool>>>,
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
        if let Some(drawing) = &patch.drawing {
            self.drawing = drawing.apply(self.drawing.as_ref());
        }
        if let Some(auto_fit) = &patch.auto_fit {
            self.auto_fit = auto_fit.value;
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
