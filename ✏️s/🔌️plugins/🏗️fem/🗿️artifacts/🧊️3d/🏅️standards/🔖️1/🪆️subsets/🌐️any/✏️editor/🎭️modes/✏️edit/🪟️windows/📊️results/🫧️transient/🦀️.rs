//! 🫧️ The running playback CLOCK of one concrete FEM 3D results window — the per-frame phase the
//! tick chain advances at ~30 fps.
//!
//! 🕰️ Why a transient and not the window config: a window-config partition is a history-ledgered
//! store. A coalesced amend per frame keeps ONE edit, but that edit accumulates every frame's
//! operation and the store re-digests the whole edit on each amend, so the tick cost grows with
//! the number of frames played (the browser's tick period tripled inside a minute), and each amend
//! displaces three owners into the partition's fixed retirement queue. A transient partition is one
//! root behind an `Arc` with no history at all — the exact tier for an unbounded stream of frames.
//! The transport SETTINGS (speed, loop mode, waveform, play/pause) and the RESTING phase the slider
//! seeks stay in [`super::config::Fem3dResultsAnimation`]; the clock exists only while the window
//! plays, and the tick that finds the transport stopped parks the clock's phase back into the
//! config and clears it.

use super::config::{Fem3dLoopMode, Fem3dResultsAnimation, ANIMATION_SPEED_MAXIMUM, ANIMATION_SPEED_MINIMUM};

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

pub const WINDOW_KIND_ID: &str = super::FEM3D_WINDOW_RESULTS;

//#region 🔖️Clock
impl Fem3dPlaybackClock {
    /// ▶️ The clock a run starts from: the transport's resting phase and direction.
    pub fn from_settings(settings: &Fem3dResultsAnimation) -> Self {
        Self { phase: settings.phase, reverse: settings.reverse }
    }

    /// ⏱️ The clock one fixed frame later under the transport's loop mode, and whether the run goes
    /// on: `Once` parks at 1 and ends the run; `PingPong` bounces off both ends by flipping `reverse`
    /// (`speed` never goes negative); `Loop` wraps.
    pub fn advanced(&self, settings: &Fem3dResultsAnimation, seconds: f64) -> (Self, bool) {
        let step = settings.speed.clamp(ANIMATION_SPEED_MINIMUM, ANIMATION_SPEED_MAXIMUM) * seconds;
        match settings.loop_mode {
            Fem3dLoopMode::Loop => (Self { phase: (self.phase + step).rem_euclid(1.0), reverse: self.reverse }, true),
            Fem3dLoopMode::Once => {
                let phase = self.phase + step;
                (Self { phase: phase.min(1.0), reverse: self.reverse }, phase < 1.0)
            }
            Fem3dLoopMode::PingPong => {
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
    pub fn parked_into(&self, settings: &Fem3dResultsAnimation) -> Fem3dResultsAnimation {
        Fem3dResultsAnimation { phase: self.phase, reverse: self.reverse, ..*settings }
    }
}
//#endregion 🔖️Clock

impl store::ArtifactDsl for Fem3dResultsWindowTransient {
    const EXTENSION: &'static str = "fem3dresultswindowtransient";
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid FEM results-window transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Fem3dResultsWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("FEM results-window transient pack envelope mismatch".into()));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl protocol::MutationDiff<Fem3dResultsWindowTransient> for Fem3dResultsWindowTransient {
    fn apply(&self, _base: &Fem3dResultsWindowTransient) -> protocol::MutationApplyResult<Fem3dResultsWindowTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

impl store::retirement::RetireOwned for Fem3dPlaybackClock {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        store::retirement::sequence(vec![store::retirement::leaf(self.phase), store::retirement::leaf(self.reverse)])
    }
}

impl store::retirement::RetireOwned for Fem3dResultsWindowTransient {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        store::retirement::RetireOwned::retirement(self.clock)
    }
}

impl store::retirement::RetireOwned for Fem3dResultsWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        let Self::SetPlaybackClock(mutation) = self;
        store::retirement::RetireOwned::retirement(mutation.clock)
    }
}

#[expect(clippy::unnecessary_wraps, reason = "ArtifactEphemeralTransferPreparationFactory requires a fallible footprint callback")]
fn clock_footprint(_: &Fem3dResultsWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: size_of::<Fem3dResultsWindowTransientMutation>() })
}

fn clock_transfer(mutation: Fem3dResultsWindowTransientMutation) -> Fem3dResultsWindowTransient {
    let Fem3dResultsWindowTransientMutation::SetPlaybackClock(mutation) = mutation;
    Fem3dResultsWindowTransient { clock: mutation.clock }
}

pub struct Fem3dResultsWindowTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for Fem3dResultsWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = WINDOW_KIND_ID;
    type State = Fem3dResultsWindowTransient;
    type Mutation = Fem3dResultsWindowTransientMutation;

    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::State>> = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::Mutation>> = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(clock_footprint, clock_transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

//#region 🔖️Address
/// 🫧️ The clock publication into one exact results window's transient partition — `None` clears it.
pub fn addressed_to(window_id: &str, clock: Option<Fem3dPlaybackClock>) -> semio_framework_plugin::WindowTransientMutation {
    semio_framework_plugin::WindowTransientMutation::of::<Fem3dResultsWindowTransientOwner>(window_id, SetPlaybackClock { clock }.into())
}

/// 🫧️ The clock a captured results-window transient carries, if the capture is that window's.
pub fn captured_clock(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>, window_id: &str) -> Option<Fem3dPlaybackClock> {
    snapshot.filter(|snapshot| snapshot.window_id() == window_id && snapshot.window_kind_id() == WINDOW_KIND_ID).and_then(|snapshot| snapshot.get::<Fem3dResultsWindowTransientOwner>()).and_then(|state| state.clock)
}
//#endregion 🔖️Address

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
