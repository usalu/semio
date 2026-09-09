//! 👥️ Generation3d viewer presence — the closed semantic mutation aggregate.
//!
//! Two authored leaf directories, one per shareable live facet. Both are immediate children of this
//! aggregate's own mutation root, so `dsl::Mutations`'s leaf-ownership contract holds.

use super::Generation3dViewPresence;
use crate::viewer::generation3d::config::Generation3dViewCamera;

#[path = "📷️set-preview-camera/🦀️.rs"]
mod set_preview_camera;
#[path = "👁️set-show-mode/🦀️.rs"]
mod set_show_mode;

pub use set_preview_camera::SetPreviewCamera;
pub use set_show_mode::SetShowMode;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = Generation3dViewPresence, diff = Generation3dViewPresence, schema = "generation3dview.presence")]
pub enum Generation3dViewPresenceMutation {
    #[dsl(key = "preview-camera")]
    SetPreviewCamera(SetPreviewCamera),
    #[dsl(key = "show-mode")]
    SetShowMode(SetShowMode),
}

impl protocol::OpText for Generation3dViewPresenceMutation {
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

impl protocol::OpBinary for Generation3dViewPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
