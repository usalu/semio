//! 🎛️ Edits one control on a durable, per-shot camera.

use crate::diff::ShootingDiff;
use crate::{ShootingCamera, ShootingSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PatchShotCamera {
    pub shot_id: String,
    pub field: String,
    pub camera: ShootingCamera,
    pub fallback: ShootingCamera,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<ShotCameraSlot>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ShotCameraSlot {
    pub seed: ShootingCamera,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera_id: Option<String>,
}

impl<Op> MutationKind<ShootingSnapshot, Op> for PatchShotCamera
where
    Op: protocol::Mutation<ShootingSnapshot, Diff = ShootingDiff> + From<PatchShotCamera>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "shot-camera", kind: "patch-shot-camera", record: "PatchedShotCamera" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Edit camera of shot \"{}\"", self.shot_id), &format!("Kamera von Aufnahme \"{}\" bearbeiten", self.shot_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.shot_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ShootingSnapshot, Op> for PatchShotCamera
where
    Op: protocol::ColdMutation<ShootingSnapshot, ReferenceDiff = ShootingDiff> + From<PatchShotCamera>,
{
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}

pub(super) fn slot_id(shot_id: &str) -> String {
    format!("shot-camera:{shot_id}")
}

pub(super) fn apply_field(camera: &mut ShootingCamera, field: &str, source: &ShootingCamera) -> bool {
    match field {
        "projection" => camera.projection = source.projection.clone(),
        "zoom" => camera.zoom = source.zoom,
        "fov" => camera.fov = source.fov,
        "eyeX" => camera.position[0] = source.position[0],
        "eyeY" => camera.position[1] = source.position[1],
        "eyeZ" => camera.position[2] = source.position[2],
        "targetX" => camera.target[0] = source.target[0],
        "targetY" => camera.target[1] = source.target[1],
        "targetZ" => camera.target[2] = source.target[2],
        "upX" | "upY" | "upZ" => {
            let defaults = [0.0, 0.0, 1.0];
            let axis = match field { "upX" => 0, "upY" => 1, _ => 2 };
            let mut up = camera.up.unwrap_or(defaults);
            up[axis] = source.up.unwrap_or(defaults)[axis];
            camera.up = (source.up.is_some() || up != defaults).then_some(up);
        }
        _ => return false,
    }
    true
}

fn normalized(vector: [f64; 3]) -> Option<[f64; 3]> {
    let scale = vector.into_iter().map(f64::abs).fold(0.0, f64::max);
    if scale == 0.0 || !vector.into_iter().all(f64::is_finite) { return None; }
    Some(vector.map(|component| component / scale))
}

impl PatchShotCamera {
/// 📐️ Checks the complete pose for finite scalars and nondegenerate view vectors.
pub fn valid_camera(camera: &ShootingCamera) -> bool {
    let Some(eye) = normalized(std::array::from_fn(|i| camera.position[i] - camera.target[i])) else { return false; };
    if !camera.zoom.is_finite() || camera.zoom <= 0.0 || !(0.0 < camera.fov && camera.fov < 180.0) {
        return false;
    }
    if !matches!(camera.projection.as_deref(), None | Some("orthographic" | "perspective")) { return false; }
    let Some(up) = camera.up else { return true; };
    let Some(up) = normalized(up) else { return false; };
    (0..3).any(|axis| {
        let a = (axis + 1) % 3;
        let b = (axis + 2) % 3;
        (eye[a] * up[b] - eye[b] * up[a]).abs() > 1.0e-12
    })
}
}
