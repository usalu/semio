//! 🎚️ Persisted local preferences for one exact CAD world window instance.

use crate::editor::cad::config::{CadDislocateOptions, CadSunConfig};
use crate::editor::cad::modes::edit::windows::{building, energy, shape, structure_classic};
use crate::CadCamera;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(extension = "cadworldwindowcfg")]
#[dsl(id = "cad.worldwindowconfig")]
#[dsl(layout = "lines")]
pub struct CadWorldWindowConfig {
    #[dsl(block)]
    pub camera: CadCamera,
    #[dsl(block)]
    pub sun: CadSunConfig,
    #[dsl(block)]
    pub dislocate_options: CadDislocateOptions,
}

impl Default for CadWorldWindowConfig {
    fn default() -> Self {
        Self { camera: CadCamera::default(), sun: CadSunConfig::default(), dislocate_options: CadDislocateOptions::default() }
    }
}

impl store::ArtifactDsl for CadWorldWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid CAD world-window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for CadWorldWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("CAD world-window pack envelope mismatch".into()));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}

store::impl_whole_record_config!(CadWorldWindowConfig);

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
pub enum CadWorldWindowConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Box<CadWorldWindowConfig>,
    },
}

impl protocol::OpText for CadWorldWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown CAD world-window mutation '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let spec = <Self as dsl::DslVariants>::variants().iter().find(|(key, _)| key == &keyword).map(|(_, spec)| spec()).expect("world-window mutation variant");
        dsl::print(&record, &spec, dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for CadWorldWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}

impl Mutation<CadWorldWindowConfig> for CadWorldWindowConfigMutation {
    type Diff = CadWorldWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set CAD World Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Snapshot",
        payload_schema: "cad.worldwindowconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, base: &CadWorldWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { config } if config.as_ref() == base => protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "CAD world-window configuration is already up to date."),
            Self::Snapshot { config } => protocol::MutationOutcome::new(config.as_ref().clone()),
        }
    }
    fn inverse(&self, base: &CadWorldWindowConfig) -> Vec<Self> { vec![Self::Snapshot { config: Box::new(base.clone()) }] }
}

macro_rules! cad_world_window_config_owner {
    ($owner:ident, $kind:path) => {
        pub struct $owner;
        impl semio_framework_plugin::WindowConfigOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $kind;
            const SCHEMA: &'static str = "cad.worldwindowconfig";
            const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
            type State = crate::editor::cad::modes::edit::windows::config::CadWorldWindowConfig;
            type Mutation = crate::editor::cad::modes::edit::windows::config::CadWorldWindowConfigMutation;
            fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
            fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
        }
    };
}
pub(crate) use cad_world_window_config_owner;

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> CadWorldWindowConfig {
    let Some(snapshot) = view.window else { return CadWorldWindowConfig::default() };
    match snapshot.window_kind_id() {
        shape::WINDOW_KIND_ID => snapshot.get::<shape::config::CadShapeWindowConfigOwner>(),
        building::WINDOW_KIND_ID => snapshot.get::<building::config::CadBuildingWindowConfigOwner>(),
        energy::WINDOW_KIND_ID => snapshot.get::<energy::config::CadEnergyWindowConfigOwner>(),
        structure_classic::WINDOW_KIND_ID => snapshot.get::<structure_classic::config::CadStructureClassicWindowConfigOwner>(),
        _ => None,
    }
    .cloned()
    .unwrap_or_default()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, config: CadWorldWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("cad.window.required: command has no addressed window instance"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("cad.window.stale: addressed window instance is not open"))?;
    let mutation = CadWorldWindowConfigMutation::Snapshot { config: Box::new(config) };
    match kind {
        shape::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<shape::config::CadShapeWindowConfigOwner>(id, mutation)),
        building::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<building::config::CadBuildingWindowConfigOwner>(id, mutation)),
        energy::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<energy::config::CadEnergyWindowConfigOwner>(id, mutation)),
        structure_classic::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<structure_classic::config::CadStructureClassicWindowConfigOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::from("cad.window.kind: addressed window is not a CAD world window")),
    }
}

pub fn addressed_from_context(ctx: &crate::editor::cad::CadDispatchCtx, config: CadWorldWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    addressed(ctx.view_state.as_ref().ok_or_else(|| semio_framework_plugin::Fault::from("cad.window.required: command has no view state"))?, config)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window-ownership/🦀️.rs"]
mod tests;
