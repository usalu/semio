//! 🎚️ Generation3d viewer config — the closed semantic mutation aggregate.
//!
//! Five authored leaf directories, one per settled read-only interaction. Every leaf is an
//! immediate child of this aggregate's own mutation root, so `dsl::Mutations`'s leaf-ownership
//! contract holds without a single provisional descriptor.

use super::{Generation3dViewCamera, Generation3dViewConfig};

#[path = "👁️set-show-mode/🦀️.rs"]
mod set_show_mode;
#[path = "🔬️set-lod-mode/🦀️.rs"]
mod set_lod_mode;
#[path = "📷️set-preview-camera/🦀️.rs"]
mod set_preview_camera;
#[path = "🌞️set-sun/🦀️.rs"]
mod set_sun;
#[path = "🎨️set-active-example/🦀️.rs"]
mod set_active_example;

pub use set_active_example::SetActiveExample;
pub use set_lod_mode::SetLodMode;
pub use set_preview_camera::SetPreviewCamera;
pub use set_show_mode::SetShowMode;
pub use set_sun::SetSun;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = Generation3dViewConfig, diff = Generation3dViewConfig, schema = "generation3dviewcfg")]
pub enum Generation3dViewConfigMutation {
    #[dsl(key = "show-mode")]
    SetShowMode(SetShowMode),
    #[dsl(key = "lod-mode")]
    SetLodMode(SetLodMode),
    #[dsl(key = "preview-camera")]
    SetPreviewCamera(SetPreviewCamera),
    #[dsl(key = "sun")]
    SetSun(SetSun),
    /// 🎨️ Which bundled example this read-only surface is looking at — a config leaf, because a
    /// viewer opens a document rather than rewriting one.
    #[dsl(key = "active-example")]
    SetActiveExample(SetActiveExample),
}

impl protocol::OpText for Generation3dViewConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Generation3dViewConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `Generation3dViewConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn generation3d_view_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<Generation3dViewConfig, Generation3dViewConfigMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
