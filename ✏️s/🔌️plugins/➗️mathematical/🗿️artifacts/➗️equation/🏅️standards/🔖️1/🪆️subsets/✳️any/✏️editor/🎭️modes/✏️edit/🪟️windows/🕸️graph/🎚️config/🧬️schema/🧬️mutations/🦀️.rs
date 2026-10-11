//! 🧬️ Equation configuration mutations with explicit source descriptors.

use super::{EquationCamera, EquationGraphWindowConfig, EquationGraphWindowConfigDiff};
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
// 🔮️ The test-only serde mirror is the INDEPENDENT oracle `🧫️fixtures/🔁️mutations.json` is read
// through (`language_neutral_mutations_match_json_oracle_and_restore_base` decodes the same vector
// twice — once with `dsl::json`, once with `serde_json` — and asserts they agree). It must therefore
// spell the same wire shape as `#[value(..)]` below: internally tagged on `kind`, kebab-case variant
// names. Without the mirror serde used its default EXTERNALLY tagged form and refused every committed
// vector with `invalid value: map, expected map with a single key`.
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "kind", rename_all = "kebab-case"))]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = EquationGraphWindowConfig, diff = EquationGraphWindowConfigDiff, schema = "mathematical.equationgraphwindowconfig")]
pub enum EquationGraphWindowConfigMutation {
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
}





impl store::snapshot_clone_preparation::ConfigApplyMutation<EquationGraphWindowConfig> for EquationGraphWindowConfigMutation {
    fn exchange(self, post: &mut EquationGraphWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::SetCamera(SetCamera { camera }) => Self::SetCamera(SetCamera { camera: std::mem::replace(&mut post.camera, camera) }),
        })
    }

    fn admissible(&self) -> bool {
        let Self::SetCamera(SetCamera { camera }) = self;
        [camera.x, camera.y, camera.zoom].into_iter().all(f64::is_finite)
    }
}

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
