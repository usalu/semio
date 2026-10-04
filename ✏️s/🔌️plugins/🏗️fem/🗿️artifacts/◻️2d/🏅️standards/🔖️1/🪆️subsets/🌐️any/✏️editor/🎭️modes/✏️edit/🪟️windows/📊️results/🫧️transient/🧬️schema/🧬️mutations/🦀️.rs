//! 🫧️ FEM results window-transient mutation aggregate (fem 2d and fem 3d share it).

use super::FemResultsWindowTransient;

#[path = "⏱️set-playback-clock/🦀️.rs"]
mod set_playback_clock;
pub use set_playback_clock::SetPlaybackClock;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = FemResultsWindowTransient, diff = FemResultsWindowTransient, schema = "fem.resultswindowtransient")]
pub enum FemResultsWindowTransientMutation {
    #[dsl(key = "set-playback-clock")]
    SetPlaybackClock(SetPlaybackClock),
}

impl protocol::OpText for FemResultsWindowTransientMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for FemResultsWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `FemResultsWindowTransient`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn fem_results_window_transient_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<FemResultsWindowTransient, FemResultsWindowTransientMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
