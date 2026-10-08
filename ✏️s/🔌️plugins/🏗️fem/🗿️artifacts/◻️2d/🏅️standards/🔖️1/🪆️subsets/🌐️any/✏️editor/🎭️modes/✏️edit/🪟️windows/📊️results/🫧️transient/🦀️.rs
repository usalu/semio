//! 🫧️ The running playback CLOCK of one concrete FEM results window — the per-frame phase the tick chain
//! advances at ~30 fps. ONE clock for every FEM editor: fem 2d and fem 3d register this transient for their own
//! results-window kind ([`Fem2dResultsWindowTransientOwner`] here, fem 3d's owner beside its window).
//!
//! 🕰️ Why a transient and not the window config: a window-config partition is a history-ledgered
//! store. A coalesced amend per frame keeps ONE edit, but that edit accumulates every frame's
//! operation and the store re-digests the whole edit on each amend, so the tick cost grows with
//! the number of frames played (the browser's tick period tripled inside a minute), and each amend
//! displaces three owners into the partition's fixed retirement queue. A transient partition is one
//! root behind an `Arc` with no history at all — the exact tier for an unbounded stream of frames.
//! The transport SETTINGS (speed, loop mode, waveform, play/pause) and the RESTING phase the slider
//! seeks stay in [`crate::app_surface::FemResultsAnimation`] inside each window config; the clock exists
//! only while the window plays, and the tick that finds the transport stopped parks the clock's phase back
//! into the config and clears it.

use crate::app_surface::{FemLoopMode, FemResultsAnimation, ANIMATION_SPEED_MAXIMUM, ANIMATION_SPEED_MINIMUM};
use semio_framework_plugin::{Fault, WindowTransientMutation, WindowTransientOwner, WindowTransientOwnerBundle, WindowTransientSnapshot};

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

pub const WINDOW_KIND_ID: &str = super::WINDOW_KIND_ID;

//#region 🔖️Clock
impl FemPlaybackClock {
    /// ▶️ The clock a run starts from: the transport's resting phase and direction.
    pub fn from_settings(settings: &FemResultsAnimation) -> Self {
        Self { phase: settings.phase, reverse: settings.reverse }
    }

    /// ⏱️ The clock one fixed frame later under the transport's loop mode, and whether the run goes
    /// on: `Once` parks at 1 and ends the run; `PingPong` bounces off both ends by flipping `reverse`
    /// (`speed` never goes negative); `Loop` wraps.
    pub fn advanced(&self, settings: &FemResultsAnimation, seconds: f64) -> (Self, bool) {
        let step = settings.speed.clamp(ANIMATION_SPEED_MINIMUM, ANIMATION_SPEED_MAXIMUM) * seconds;
        match settings.loop_mode {
            FemLoopMode::Loop => (Self { phase: (self.phase + step).rem_euclid(1.0), reverse: self.reverse }, true),
            FemLoopMode::Once => {
                let phase = self.phase + step;
                (Self { phase: phase.min(1.0), reverse: self.reverse }, phase < 1.0)
            }
            FemLoopMode::PingPong => {
                let phase = if self.reverse { self.phase - step } else { self.phase + step };
                let next = if phase > 1.0 {
                    Self { phase: (2.0 - phase).clamp(0.0, 1.0), reverse: true }
                } else if phase < 0.0 {
                    Self { phase: (-phase).clamp(0.0, 1.0), reverse: false }
                } else {
                    Self { phase, reverse: self.reverse }
                };
                (next, true)
            }
        }
    }

    /// 🎞️ The transport settings with this clock's phase and direction folded in — what the results
    /// window DRAWS while the clock runs, and what the parking tick writes back as the resting state.
    pub fn parked_into(&self, settings: &FemResultsAnimation) -> FemResultsAnimation {
        FemResultsAnimation { phase: self.phase, reverse: self.reverse, ..*settings }
    }
}
//#endregion 🔖️Clock

impl store::ArtifactDsl for FemResultsWindowTransient {
    const EXTENSION: &'static str = "femresultswindowtransient";
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid FEM results-window transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for FemResultsWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "FEM results-window transient pack envelope mismatch")));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl protocol::MutationDiff<FemResultsWindowTransient> for FemResultsWindowTransientDiff {
    fn apply(&self, base: &FemResultsWindowTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<FemResultsWindowTransient> {
        Ok(FemResultsWindowTransient { clock: self.clock.map_or(base.clock, |change| change.clock) })
    }
    fn absorb(&mut self, other: Self) {
        self.clock = other.clock.or(self.clock);
    }
}

impl protocol::DiffAlgebra<FemResultsWindowTransient> for FemResultsWindowTransientDiff {
    fn inverse(&self, base: &FemResultsWindowTransient) -> Self {
        Self { clock: self.clock.map(|_| FemPlaybackClockChange { clock: base.clock }) }
    }
    fn is_empty(&self) -> bool {
        self.clock.is_none()
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

impl semio_framework_value::retirement::RetireOwned for FemPlaybackClock {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(self.phase), semio_framework_value::retirement::leaf(self.reverse)])
    }
}

impl semio_framework_value::retirement::RetireOwned for FemResultsWindowTransient {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.clock)
    }
}

impl semio_framework_value::retirement::RetireOwned for FemResultsWindowTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self::SetPlaybackClock(mutation) = self;
        semio_framework_value::retirement::RetireOwned::retirement(mutation.clock)
    }
}

#[expect(clippy::unnecessary_wraps, reason = "ArtifactEphemeralTransferPreparationFactory requires a fallible footprint callback")]
fn clock_footprint(_: &FemResultsWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    Ok(store::ArtifactStoreOneItemFootprint::for_ephemeral_item(size_of::<FemResultsWindowTransientMutation>()))
}

fn clock_transfer(mutation: FemResultsWindowTransientMutation) -> FemResultsWindowTransient {
    let FemResultsWindowTransientMutation::SetPlaybackClock(mutation) = mutation;
    FemResultsWindowTransient { clock: mutation.clock }
}

//#region 🧰️Owners
/// 🪪️ A results-window kind's owner of THIS transient — every FEM editor declares one for its own window kind.
pub trait FemResultsWindowTransientOwner: WindowTransientOwner<State = FemResultsWindowTransient, Mutation = FemResultsWindowTransientMutation> {}

impl<O: WindowTransientOwner<State = FemResultsWindowTransient, Mutation = FemResultsWindowTransientMutation>> FemResultsWindowTransientOwner for O {}

/// 🧰️ The bounded preparation and retirement owners every FEM results-window kind registers for the clock.
pub fn owners() -> WindowTransientOwnerBundle<FemResultsWindowTransient, FemResultsWindowTransientMutation> {
    let state: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<FemResultsWindowTransient>> = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<FemResultsWindowTransient>::default());
    let mutation: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<FemResultsWindowTransientMutation>> = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<FemResultsWindowTransientMutation>::default());
    let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(clock_footprint, clock_transfer, state.clone(), mutation.clone()));
    WindowTransientOwnerBundle::new(preparation, state, mutation)
}

/// 🪟️ The fem 2d results window's owner of the clock.
pub struct Fem2dResultsWindowTransientOwner;

impl WindowTransientOwner for Fem2dResultsWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = WINDOW_KIND_ID;
    type State = FemResultsWindowTransient;
    type Mutation = FemResultsWindowTransientMutation;

    fn build_owners() -> WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        owners()
    }
}
//#endregion 🧰️Owners

//#region 🔖️Address
/// 📮️ The clock publication into one exact results window's transient partition — `None` clears it.
pub fn addressed_to<O: FemResultsWindowTransientOwner>(window_id: &str, clock: Option<FemPlaybackClock>) -> WindowTransientMutation {
    WindowTransientMutation::of::<O>(window_id, SetPlaybackClock { clock }.into())
}

/// 📥️ The clock a captured results-window transient carries, if the capture is that window's.
pub fn captured_clock<O: FemResultsWindowTransientOwner>(snapshot: Option<&WindowTransientSnapshot>, window_id: &str) -> Option<FemPlaybackClock> {
    snapshot.filter(|snapshot| snapshot.window_id() == window_id && snapshot.window_kind_id() == O::WINDOW_KIND_ID).and_then(|snapshot| snapshot.get::<O>()).and_then(|state| state.clock)
}

/// 🔐️ The clock the runtime captured for a tick — refused when the tick's declared window authority is missing or
/// belongs to another window.
pub fn required_clock<O: FemResultsWindowTransientOwner>(snapshot: Option<&WindowTransientSnapshot>, window_id: &str) -> Result<Option<FemPlaybackClock>, Fault> {
    let snapshot = snapshot.ok_or_else(|| Fault::from("fem.result-animation-tick.window-transient-required"))?;
    if snapshot.window_id() != window_id || snapshot.window_kind_id() != O::WINDOW_KIND_ID {
        return Err(Fault::from("fem.result-animation-tick.window-transient-mismatch"));
    }
    Ok(captured_clock::<O>(Some(snapshot), window_id))
}
//#endregion 🔖️Address

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
