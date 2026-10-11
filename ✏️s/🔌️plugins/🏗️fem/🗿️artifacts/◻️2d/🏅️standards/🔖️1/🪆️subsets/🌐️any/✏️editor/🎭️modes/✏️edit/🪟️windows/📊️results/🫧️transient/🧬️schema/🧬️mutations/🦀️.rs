//! 🫧️ FEM results window-transient mutation aggregate (fem 2d and fem 3d share it).

use super::{FemPlaybackClockChange, FemResultsWindowTransient, FemResultsWindowTransientDiff};

#[path = "⏱️set-playback-clock/🦀️.rs"]
mod set_playback_clock;
pub use set_playback_clock::SetPlaybackClock;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = FemResultsWindowTransient, diff = FemResultsWindowTransientDiff, schema = "fem.resultswindowtransient")]
pub enum FemResultsWindowTransientMutation {
    #[dsl(key = "set-playback-clock")]
    SetPlaybackClock(SetPlaybackClock),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
