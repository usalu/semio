//! 🎚️ Generation3d editor config — the closed semantic mutation aggregate.
//!
//! Seven authored leaf directories, one per settled view-state interaction. Every leaf is an
//! immediate child of this aggregate's own mutation root, so `dsl::Mutations`'s leaf-ownership
//! contract holds and not one descriptor is provisional.

use super::{CameraJson, Generation3dConfig, Generation3dPreviewCamera};

#[path = "⚙️set-snapshot/🦀️.rs"]
mod set_snapshot;
#[path = "🔬️set-lod-mode/🦀️.rs"]
mod set_lod_mode;
#[path = "👁️set-show-mode/🦀️.rs"]
mod set_show_mode;
#[path = "🕸️set-camera/🦀️.rs"]
mod set_camera;
#[path = "📷️set-preview-camera/🦀️.rs"]
mod set_preview_camera;
#[path = "🌞️set-sun/🦀️.rs"]
mod set_sun;
#[path = "🧬️set-selected/🦀️.rs"]
mod set_selected_generation;

pub use set_camera::SetCamera;
pub use set_lod_mode::SetLodMode;
pub use set_preview_camera::SetPreviewCamera;
pub use set_selected_generation::SetSelectedGeneration;
pub use set_show_mode::SetShowMode;
pub use set_snapshot::SetSnapshot;
pub use set_sun::SetSun;

/// 🧮️ [`Generation3dConfig`]'s operation enum — one newtype variant per authored leaf, in binary-tag
/// order. Appending is safe, reordering is a wire-format break.
// 🧯️ `SetSnapshot` genuinely carries the whole config by value (it IS the inverse of every bulk
// change); boxing it would only relocate the allocation for an enum that is never stored in bulk.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = Generation3dConfig, diff = Generation3dConfig, schema = "generation3dcfg")]
pub enum Generation3dConfigMutation {
    #[dsl(key = "snapshot")]
    SetSnapshot(SetSnapshot),
    #[dsl(key = "lod-mode")]
    SetLodMode(SetLodMode),
    #[dsl(key = "show-mode")]
    SetShowMode(SetShowMode),
    #[dsl(key = "camera")]
    SetCamera(SetCamera),
    #[dsl(key = "preview-camera")]
    SetPreviewCamera(SetPreviewCamera),
    #[dsl(key = "sun")]
    SetSun(SetSun),
    #[dsl(key = "selected-generation")]
    SetSelectedGeneration(SetSelectedGeneration),
}

impl protocol::OpText for Generation3dConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown mutation line '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Generation3dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `Generation3dConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn generation3d_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<Generation3dConfig, Generation3dConfigMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
