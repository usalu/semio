//! 🎚️ Persisted local state for one exact FEM 2D results window.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

impl Default for Fem2dResultsWindowConfig {
    fn default() -> Self {
        Self {
            camera: crate::Viewport2d::default(),
            result_source_id: None,
            result_mode: crate::app_surface::ResultMode::Static,
            result_mode_index: 0,
            animation: crate::app_surface::FemResultsAnimation::default(),
        }
    }
}

impl store::ArtifactDsl for Fem2dResultsWindowConfig {
    const EXTENSION: &'static str = "fem2dresultswindowcfg";
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid FEM window-config envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Fem2dResultsWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "FEM window-config pack envelope mismatch")));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ConfigRecord for Fem2dResultsWindowConfig {}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum Fem2dResultsWindowConfigMutation {
    #[dsl(key = "update")]
    Update {
        #[dsl(block)]
        patch: Box<Fem2dResultsWindowConfigPatch>,
    },
}

impl protocol::OpText for Fem2dResultsWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for Fem2dResultsWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

impl protocol::Mutation<Fem2dResultsWindowConfig> for Fem2dResultsWindowConfigMutation {
    type Diff = Fem2dResultsWindowConfigPatch;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set FEM 2D Results Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Update",
        payload_schema: "fem.2d.resultswindowconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }
    fn diff(&self, base: &Fem2dResultsWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        let Self::Update { patch } = self;
        let changed = patch.against(base);
        if changed == Fem2dResultsWindowConfigPatch::default() {
            protocol::MutationOutcome::new(changed).warning("mutation.no-op", "Window configuration is already current.")
        } else {
            protocol::MutationOutcome::new(changed)
        }
    }
    fn inverse(&self, base: &Fem2dResultsWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        let Self::Update { patch } = self;
        let changed = patch.against(base);
        Ok(if changed == Fem2dResultsWindowConfigPatch::default() { Vec::new() } else { vec![Self::Update { patch: Box::new(protocol::DiffAlgebra::inverse(&changed, base)) }] })
    }
}

pub struct Fem2dResultsWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for Fem2dResultsWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::WINDOW_KIND_ID;
    const SCHEMA: &'static str = "fem.2d.resultswindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 16_384;
    type State = Fem2dResultsWindowConfig;
    type Mutation = Fem2dResultsWindowConfigMutation;
    fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> {
        semio_framework_plugin::bounded_window_config_store_owners::<Self>()
    }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
    }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
    }
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> Fem2dResultsWindowConfig {
    view.window::<Fem2dResultsWindowConfigOwner>().cloned().unwrap_or_default()
}

/// 🎞️ The configuration the results window DRAWS: the persisted transport with the running clock's
/// phase and direction folded in while the window plays, the resting phase otherwise.
pub fn effective(config: &Fem2dResultsWindowConfig, clock: Option<&super::transient::FemPlaybackClock>) -> Fem2dResultsWindowConfig {
    match clock {
        Some(clock) => Fem2dResultsWindowConfig { animation: clock.parked_into(&config.animation), ..config.clone() },
        None => config.clone(),
    }
}

/// 🪟️ The captured results-window partition, or `None` when the projection captured another window
/// (a panel projection binds to the FOCUSED pane) — the read side that lets a panel tell "live state"
/// from "the defaults of a pane it cannot see".
pub fn captured<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> Option<Fem2dResultsWindowConfig> {
    view.window::<Fem2dResultsWindowConfigOwner>().cloned()
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
///
/// With no captured results-window config at all (a bare `ViewModel`, as the publication-lane law
/// builds) the tag, then the roster, decides; a roster that names no results window refuses outright
/// rather than clobbering a partition with defaults.
pub fn addressed_window_id<C>(
    cfg: &semio_framework_plugin::ConfigView<'_, C>,
    view: &semio_framework_plugin::ViewModel,
    requested: Option<&str>,
) -> Result<String, semio_framework_plugin::Fault> {
    let kind_id = <Fem2dResultsWindowConfigOwner as semio_framework_plugin::WindowConfigOwner>::WINDOW_KIND_ID;
    let requested = requested.filter(|id| !id.is_empty());
    let captured = cfg.window.filter(|snapshot| snapshot.window_kind_id() == kind_id).map(|snapshot| snapshot.window_id().to_string());
    if let Some(requested) = requested {
        if view.window_instances.iter().any(|window| window.id == requested && window.window_kind_id == kind_id) {
            return match captured {
                Some(captured) if captured != requested => {
                    Err(semio_framework_plugin::Fault::from(format!("fem.window.mismatch: control asked for results window '{requested}' but the captured configuration is '{captured}'")))
                }
                _ => Ok(requested.to_string()),
            };
        }
        return Err(semio_framework_plugin::Fault::from(format!("fem.window.stale: '{requested}' is not an open FEM 2D results window")));
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
pub fn addressed_to(window_id: &str, config: Fem2dResultsWindowConfig) -> semio_framework_plugin::WindowConfigMutation {
    semio_framework_plugin::WindowConfigMutation::of::<Fem2dResultsWindowConfigOwner>(window_id, Fem2dResultsWindowConfigMutation::Update { patch: Box::new(Fem2dResultsWindowConfigPatch::replacing(&config)) })
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, config: Fem2dResultsWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("fem.window.required: command has no addressed window instance"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("fem.window.stale: addressed window instance is not open"))?;
    if kind != <Fem2dResultsWindowConfigOwner as semio_framework_plugin::WindowConfigOwner>::WINDOW_KIND_ID {
        return Err(semio_framework_plugin::Fault::from("fem.window.kind: addressed window has the wrong kind"));
    }
    Ok(addressed_to(id, config))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
