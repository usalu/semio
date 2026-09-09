//! 🧬️ Layout layout.config mutation collection.

use super::*;
#[path = "📑️set-active-page/🦀️.rs"]
mod set_active_page;
pub use set_active_page::SetActivePage;
#[path = "👻️set-drop-preview/🦀️.rs"]
mod set_drop_preview;
pub use set_drop_preview::SetDropPreview;
#[path = "💬️set-engagement-input/🦀️.rs"]
mod set_engagement_input;
pub use set_engagement_input::SetEngagementInput;
#[path = "📷️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
#[path = "🪞️set-preview-camera/🦀️.rs"]
mod set_preview_camera;
pub use set_preview_camera::SetPreviewCamera;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = LayoutConfig, diff = LayoutConfig, schema = "layout.config")]
pub enum LayoutConfigMutation {
    #[dsl(key = "set-active-page")]
    SetActivePage(SetActivePage),
    #[dsl(key = "set-drop-preview")]
    SetDropPreview(SetDropPreview),
    #[dsl(key = "set-engagement-input")]
    SetEngagementInput(SetEngagementInput),
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
    #[dsl(key = "set-preview-camera")]
    SetPreviewCamera(SetPreviewCamera),
}

impl protocol::OpText for LayoutConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        for (keyword, spec_fn) in <Self as dsl::DslVariants>::variants() {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(&keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec = variants.iter().find(|(key, _)| key == &keyword).expect("declared variant").1();
        dsl::print(&record, &spec, dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for LayoutConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
