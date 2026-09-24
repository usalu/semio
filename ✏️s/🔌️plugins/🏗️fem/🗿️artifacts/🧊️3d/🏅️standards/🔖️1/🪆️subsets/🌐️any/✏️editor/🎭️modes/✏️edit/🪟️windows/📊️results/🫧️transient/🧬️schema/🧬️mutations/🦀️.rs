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
        dsl::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        dsl::variants_text::print_op(self)
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

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `Fem3dResultsWindowTransient`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn fem3d_results_window_transient_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<Fem3dResultsWindowTransient, Fem3dResultsWindowTransientMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
