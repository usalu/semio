//! 🎚️ Persisted local state for one exact FEM 3D results window.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

//#region 🔖️Playback
/// ⏱️ The playback frame delta, in milliseconds. A wasm guest has no monotonic clock it may read
/// inside a command, so the tick advances the phase by a FIXED delta and the host's
/// `Effect::DispatchAction { delay_ms }` is what keeps that delta honest (~30 fps).
pub const ANIMATION_TICK_MS: u64 = 33;

/// ⏱️ [`ANIMATION_TICK_MS`] in seconds — `speed` is stated in cycles per second.
pub const ANIMATION_TICK_SECONDS: f64 = ANIMATION_TICK_MS as f64 / 1_000.0;

/// 🐢️ Slowest and fastest playback the transport admits, in cycles per second.
pub const ANIMATION_SPEED_MINIMUM: f64 = 0.05;
pub const ANIMATION_SPEED_MAXIMUM: f64 = 4.0;

/// ⏭️ One transport step — a twenty-fourth of a cycle, the classic film frame.
pub const ANIMATION_PHASE_STEP: f64 = 1.0 / 24.0;

impl Default for Fem3dResultsAnimation {
    /// 🎞️ A still results window sits at phase 1 — the FULL deformed shape, exactly what the window
    /// drew before playback existed. Phase 0 would open every results window on an undeformed
    /// structure, so the resting pose is the end of the ramp, not its start.
    fn default() -> Self {
        Self { phase: 1.0, playing: false, speed: 0.5, loop_mode: Fem3dLoopMode::Loop, waveform: Fem3dWaveform::Ramp, reverse: false }
    }
}

impl Fem3dResultsAnimation {
    /// 〰️ The signed factor the solved displacement field is scaled by this frame. `Sine` returns
    /// negative values on purpose: the structure swings through both signs instead of only growing.
    pub fn amplitude(&self) -> f64 {
        match self.waveform {
            Fem3dWaveform::Ramp => self.phase,
            Fem3dWaveform::Sine => (std::f64::consts::TAU * self.phase).sin(),
        }
    }

    /// ▶️ Arms playback: a `Once` run that already sits at its end rewinds, so the play button is
    /// never a control that visibly does nothing.
    pub fn start(&mut self) {
        if self.loop_mode == Fem3dLoopMode::Once && self.phase >= 1.0 {
            self.phase = 0.0;
        }
        self.playing = true;
    }
}
//#endregion 🔖️Playback

impl Default for Fem3dResultsWindowConfig {
    fn default() -> Self {
        Self { camera: crate::viewport::INITIAL, result_source_id: None, result_mode: crate::app_surface::ResultMode::Static, result_mode_index: 0, animation: Fem3dResultsAnimation::default() }
    }
}

impl store::ArtifactDsl for Fem3dResultsWindowConfig {
    const EXTENSION: &'static str = "fem3dresultswindowcfg";
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid FEM window-config envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Fem3dResultsWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("FEM window-config pack envelope mismatch".into()));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

store::impl_whole_record_config!(Fem3dResultsWindowConfig);

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslOps)]
pub enum Fem3dResultsWindowConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Box<Fem3dResultsWindowConfig>,
    },
}

impl protocol::OpText for Fem3dResultsWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        dsl::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for Fem3dResultsWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

impl protocol::Mutation<Fem3dResultsWindowConfig> for Fem3dResultsWindowConfigMutation {
    type Diff = Fem3dResultsWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set FEM 3D Results Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Snapshot",
        payload_schema: "fem.3d.resultswindowconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }
    fn diff(&self, base: &Fem3dResultsWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { config } if config.as_ref() == base => protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "FEM window configuration is already current."),
            Self::Snapshot { config } => protocol::MutationOutcome::new(config.as_ref().clone()),
        }
    }
    fn inverse(&self, base: &Fem3dResultsWindowConfig) -> Vec<Self> {
        vec![Self::Snapshot { config: Box::new(base.clone()) }]
    }
}

pub struct Fem3dResultsWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for Fem3dResultsWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::FEM3D_WINDOW_RESULTS;
    const SCHEMA: &'static str = "fem.3d.resultswindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 16_384;
    type State = Fem3dResultsWindowConfig;
    type Mutation = Fem3dResultsWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> {
        semio_framework_plugin::bounded_window_config_store_owners::<Self>()
    }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
    }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
    }
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> Fem3dResultsWindowConfig {
    view.window::<Fem3dResultsWindowConfigOwner>().cloned().unwrap_or_default()
}

/// 🎞️ The configuration the results window DRAWS: the persisted transport with the running clock's
/// phase and direction folded in while the window plays, the resting phase otherwise.
pub fn effective(config: &Fem3dResultsWindowConfig, clock: Option<&super::transient::Fem3dPlaybackClock>) -> Fem3dResultsWindowConfig {
    match clock {
        Some(clock) => Fem3dResultsWindowConfig { animation: clock.parked_into(&config.animation), ..config.clone() },
        None => config.clone(),
    }
}

/// 🪟️ The captured results-window partition, or `None` when the projection captured another window
/// (a panel projection binds to the FOCUSED pane) — the read side that lets a panel tell "live state"
/// from "the defaults of a pane it cannot see".
pub fn captured<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> Option<Fem3dResultsWindowConfig> {
    view.window::<Fem3dResultsWindowConfigOwner>().cloned()
}

/// 🪟️ The results-window instance a command both READS through [`current`] and writes back.
///
/// `requested` is the `windowId` a panel control tagged onto its own arguments (the puzzle3d
/// Settings-panel rule): a panel projection carries no `window_id` of its own, so without the tag a
/// split layout would retune whichever pane happened to be focused. An explicit tag therefore wins —
/// but only over the ROSTER, never over the captured partition: `WindowConfigOwnerRegistry::capture`
/// binds `ConfigView::window` to `window_id`, or for a panel to `focused_window_id`, and that
/// snapshot is the state [`current`] hands the caller. Writing a DIFFERENT window than the one just
/// read would publish one pane's state into another's, so that pair is refused by name instead.
pub fn addressed_window_id<C>(cfg: &semio_framework_plugin::ConfigView<'_, C>, view: &semio_framework_plugin::ViewModel, requested: Option<&str>) -> Result<String, semio_framework_plugin::Fault> {
    let kind_id = <Fem3dResultsWindowConfigOwner as semio_framework_plugin::WindowConfigOwner>::WINDOW_KIND_ID;
    let requested = requested.filter(|id| !id.is_empty());
    let captured = cfg.window.filter(|snapshot| snapshot.window_kind_id() == kind_id).map(|snapshot| snapshot.window_id().to_string());
    if let Some(requested) = requested {
        if view.window_instances.iter().any(|window| window.id == requested && window.window_kind_id == kind_id) {
            return match captured {
                Some(captured) if captured != requested => Err(semio_framework_plugin::Fault::from(format!("fem.window.mismatch: control asked for results window '{requested}' but the captured configuration is '{captured}'"))),
                _ => Ok(requested.to_string()),
            };
        }
        return Err(semio_framework_plugin::Fault::from(format!("fem.window.stale: '{requested}' is not an open FEM 3D results window")));
    }
    if let Some(captured) = captured {
        return Ok(captured);
    }
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("fem.window.required: command has no addressed window instance"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("fem.window.stale: addressed window instance is not open"))?;
    if kind != kind_id {
        return Err(semio_framework_plugin::Fault::from("fem.window.kind: addressed window has the wrong kind"));
    }
    Ok(id.to_string())
}

/// 🎚️ The whole-record publication into one exact results-window partition.
pub fn addressed_to(window_id: &str, config: Fem3dResultsWindowConfig) -> semio_framework_plugin::WindowConfigMutation {
    semio_framework_plugin::WindowConfigMutation::of::<Fem3dResultsWindowConfigOwner>(window_id, Fem3dResultsWindowConfigMutation::Snapshot { config: Box::new(config) })
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, config: Fem3dResultsWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("fem.window.required: command has no addressed window instance"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("fem.window.stale: addressed window instance is not open"))?;
    if kind != <Fem3dResultsWindowConfigOwner as semio_framework_plugin::WindowConfigOwner>::WINDOW_KIND_ID {
        return Err(semio_framework_plugin::Fault::from("fem.window.kind: addressed window has the wrong kind"));
    }
    Ok(addressed_to(id, config))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
