//! ☀️ Shooting mutation payloads — the `scene` document-level facet's semantic verbs, one per
//! independently-set field (the play app's `☀️scene` commands each set exactly one of these; there
//! is no editor gesture that sets sun `enabled`/`azimuth`/`elevation`/`intensity` together, so this
//! is `change-scene-<field>`, not a bundled `update-scene-sun` facet — the previous option-bag
//! `PatchScene { patch: ShootingScenePatch }` payload was exactly the forbidden "raw option-bag
//! Patch struct used AS a mutation payload" pattern).

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region ☀️ChangeSceneSunEnabled
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneSunEnabled {
    pub new_enabled: bool,
}
impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunEnabled {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-enabled", kind: "change-scene-sun-enabled", record: "ChangedSceneSunEnabled" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_scene_sun_enabled(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_scene_sun_enabled(self, base)
    }
    fn label(&self) -> String {
        format!("{} sun", if self.new_enabled { "Enable" } else { "Disable" })
    }
}
//#endregion ☀️ChangeSceneSunEnabled

//#region 🧭️ChangeSceneSunAzimuth
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneSunAzimuth {
    pub new_azimuth: f64,
}
impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunAzimuth {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-azimuth", kind: "change-scene-sun-azimuth", record: "ChangedSceneSunAzimuth" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_scene_sun_azimuth(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_scene_sun_azimuth(self, base)
    }
    fn label(&self) -> String {
        format!("Change sun azimuth to {}", self.new_azimuth)
    }
}
//#endregion 🧭️ChangeSceneSunAzimuth

//#region 🌅️ChangeSceneSunElevation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneSunElevation {
    pub new_elevation: f64,
}
impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunElevation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-elevation", kind: "change-scene-sun-elevation", record: "ChangedSceneSunElevation" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_scene_sun_elevation(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_scene_sun_elevation(self, base)
    }
    fn label(&self) -> String {
        format!("Change sun elevation to {}", self.new_elevation)
    }
}
//#endregion 🌅️ChangeSceneSunElevation

//#region 💡️ChangeSceneSunIntensity
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneSunIntensity {
    pub new_intensity: f64,
}
impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunIntensity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-intensity", kind: "change-scene-sun-intensity", record: "ChangedSceneSunIntensity" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_scene_sun_intensity(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_scene_sun_intensity(self, base)
    }
    fn label(&self) -> String {
        format!("Change sun intensity to {}", self.new_intensity)
    }
}
//#endregion 💡️ChangeSceneSunIntensity

//#region 🔅️ChangeSceneAmbientIntensity
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneAmbientIntensity {
    pub new_intensity: f64,
}
impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneAmbientIntensity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-ambient-intensity", kind: "change-scene-ambient-intensity", record: "ChangedSceneAmbientIntensity" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_scene_ambient_intensity(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_scene_ambient_intensity(self, base)
    }
    fn label(&self) -> String {
        format!("Change ambient intensity to {}", self.new_intensity)
    }
}
//#endregion 🔅️ChangeSceneAmbientIntensity

//#region 🌑️ChangeSceneShadowEnabled
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneShadowEnabled {
    pub new_enabled: bool,
}
impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneShadowEnabled {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-shadow-enabled", kind: "change-scene-shadow-enabled", record: "ChangedSceneShadowEnabled" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_scene_shadow_enabled(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_scene_shadow_enabled(self, base)
    }
    fn label(&self) -> String {
        format!("{} shadows", if self.new_enabled { "Enable" } else { "Disable" })
    }
}
//#endregion 🌑️ChangeSceneShadowEnabled

//#region 🪨️ChangeSceneMaterialRoughness
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneMaterialRoughness {
    pub new_roughness: f64,
}
impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneMaterialRoughness {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-material-roughness", kind: "change-scene-material-roughness", record: "ChangedSceneMaterialRoughness" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_scene_material_roughness(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_scene_material_roughness(self, base)
    }
    fn label(&self) -> String {
        format!("Change material roughness to {}", self.new_roughness)
    }
}
//#endregion 🪨️ChangeSceneMaterialRoughness
