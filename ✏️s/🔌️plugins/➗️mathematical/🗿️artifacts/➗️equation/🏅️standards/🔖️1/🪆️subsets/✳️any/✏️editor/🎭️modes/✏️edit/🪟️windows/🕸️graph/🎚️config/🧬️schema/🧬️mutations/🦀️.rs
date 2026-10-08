//! 🧬️ Equation configuration mutations with explicit source descriptors.

use super::{EquationCamera, EquationGraphWindowConfig, EquationGraphWindowConfigDiff};
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
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





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
