//! 🎚️ Persisted local state for one exact FEM 3D model window.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

impl Default for Fem3dGumballConfig {
    /// 🧭️ Every handle a FEM entity can honestly take: moves along axes and planes, a rotation, and
    /// scaling per axis or uniformly — the options rail turns the ones a gesture does not need off.
    fn default() -> Self {
        Self { move_axes: true, move_planes: true, rotate: true, scale_axes: true, scale_uniform: true }
    }
}

impl Fem3dGumballConfig {
    /// 🎚️ Sets, or with `pressed = None` toggles, one named handle flag; an unknown flag is refused.
    pub fn set_flag(&mut self, flag: &str, pressed: Option<bool>) -> Result<(), semio_framework_plugin::Fault> {
        let slot = match flag {
            "move" | "moveAxes" => &mut self.move_axes,
            "movePlanes" => &mut self.move_planes,
            "rotate" => &mut self.rotate,
            "scaleAxes" => &mut self.scale_axes,
            "scaleUniform" => &mut self.scale_uniform,
            other => return Err(semio_framework_plugin::Fault::from(format!("fem3d.gumball-flag.unknown: '{other}' is not a gumball handle flag"))),
        };
        *slot = pressed.unwrap_or(!*slot);
        Ok(())
    }

    /// 🫥️ An all-off gumball would draw nothing to grab.
    pub fn any(&self) -> bool {
        self.move_axes || self.move_planes || self.rotate || self.scale_axes || self.scale_uniform
    }
}

impl Default for Fem3dModelWindowConfig {
    fn default() -> Self {
        Self { camera: crate::viewport::INITIAL, gumball: Fem3dGumballConfig::default() }
    }
}

impl store::ArtifactDsl for Fem3dModelWindowConfig {
    const EXTENSION: &'static str = "fem3dmodelwindowcfg";
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

impl store::ArtifactPack for Fem3dModelWindowConfig {
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

store::impl_whole_record_config!(Fem3dModelWindowConfig);

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslOps)]
pub enum Fem3dModelWindowConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Box<Fem3dModelWindowConfig>,
    },
}

impl protocol::OpText for Fem3dModelWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        dsl::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for Fem3dModelWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

impl protocol::Mutation<Fem3dModelWindowConfig> for Fem3dModelWindowConfigMutation {
    type Diff = Fem3dModelWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set FEM 3D Model Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Snapshot",
        payload_schema: "fem.3d.modelwindowconfig",
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
    fn diff(&self, base: &Fem3dModelWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { config } if config.as_ref() == base => protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "FEM window configuration is already current."),
            Self::Snapshot { config } => protocol::MutationOutcome::new(config.as_ref().clone()),
        }
    }
    fn inverse(&self, base: &Fem3dModelWindowConfig) -> Vec<Self> {
        vec![Self::Snapshot { config: Box::new(base.clone()) }]
    }
}

pub struct Fem3dModelWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for Fem3dModelWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::FEM3D_WINDOW_MODEL;
    const SCHEMA: &'static str = "fem.3d.modelwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 16_384;
    type State = Fem3dModelWindowConfig;
    type Mutation = Fem3dModelWindowConfigMutation;
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

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> Fem3dModelWindowConfig {
    view.window::<Fem3dModelWindowConfigOwner>().cloned().unwrap_or_default()
}

/// 🪟️ The captured model-window partition, or `None` when the projection captured another window.
pub fn captured<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> Option<Fem3dModelWindowConfig> {
    view.window::<Fem3dModelWindowConfigOwner>().cloned()
}

/// 🎚️ The whole-record publication into one exact model-window partition.
pub fn addressed_to(window_id: &str, config: Fem3dModelWindowConfig) -> semio_framework_plugin::WindowConfigMutation {
    semio_framework_plugin::WindowConfigMutation::of::<Fem3dModelWindowConfigOwner>(window_id, Fem3dModelWindowConfigMutation::Snapshot { config: Box::new(config) })
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, config: Fem3dModelWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("fem.window.required: command has no addressed window instance"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("fem.window.stale: addressed window instance is not open"))?;
    if kind != <Fem3dModelWindowConfigOwner as semio_framework_plugin::WindowConfigOwner>::WINDOW_KIND_ID {
        return Err(semio_framework_plugin::Fault::from("fem.window.kind: addressed window has the wrong kind"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<Fem3dModelWindowConfigOwner>(id, Fem3dModelWindowConfigMutation::Snapshot { config: Box::new(config) }))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
