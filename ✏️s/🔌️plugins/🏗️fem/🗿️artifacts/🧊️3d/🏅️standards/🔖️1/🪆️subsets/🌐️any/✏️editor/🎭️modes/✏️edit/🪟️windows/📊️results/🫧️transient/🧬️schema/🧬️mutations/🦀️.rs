//! 🫧️ FEM 3D results window-transient mutation aggregate.

use super::Fem3dResultsWindowTransient;

#[path = "⏱️set-playback-clock/🦀️.rs"]
mod set_playback_clock;
pub use set_playback_clock::SetPlaybackClock;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = Fem3dResultsWindowTransient, diff = Fem3dResultsWindowTransient, schema = "fem.3d.resultswindowtransient")]
pub enum Fem3dResultsWindowTransientMutation {
    #[dsl(key = "set-playback-clock")]
    SetPlaybackClock(SetPlaybackClock),
}

impl protocol::OpText for Fem3dResultsWindowTransientMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            if line == keyword.as_str() || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown FEM results-window transient mutation '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let spec = <Self as dsl::DslVariants>::variants().iter().find(|(key, _)| key == &keyword).map(|(_, spec)| spec()).expect("FEM results-window transient mutation variant");
        dsl::print(&record, &spec, dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Fem3dResultsWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
