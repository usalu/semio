//! 🧬️ Sequence configuration mutation collection.

use super::*;
#[path = "🏃️set-last-run/🦀️.rs"]
mod set_last_run;
pub use set_last_run::SetLastRun;
#[path = "🧭️set-orientation/🦀️.rs"]
mod set_orientation;
pub use set_orientation::SetOrientation;
#[path = "📷️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = SequenceConfig, diff = SequenceConfig, schema = "sequence.config")]
pub enum SequenceConfigMutation {
    #[dsl(key = "set-last-run")]
    SetLastRun(SetLastRun),
    #[dsl(key = "set-orientation")]
    SetOrientation(SetOrientation),
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
}

impl protocol::OpText for SequenceConfigMutation {
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

impl protocol::OpBinary for SequenceConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}
