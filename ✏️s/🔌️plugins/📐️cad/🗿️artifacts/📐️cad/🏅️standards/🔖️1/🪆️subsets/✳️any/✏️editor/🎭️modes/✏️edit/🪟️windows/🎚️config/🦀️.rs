//! 🎚️ Persisted local preferences for one exact CAD world window instance.

use crate::editor::cad::config::{CadDislocateOptions, CadSunConfig};
use crate::editor::cad::modes::edit::windows::{building, energy, shape, structure_classic};
use crate::CadCamera;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(extension = "cadworldwindowcfg")]
#[artifact(id = "cad.worldwindowconfig")]
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
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid CAD world-window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for CadWorldWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "CAD world-window pack envelope mismatch")));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { Some(Self::__dsl_spec()) }
}

impl store::ConfigRecord for CadWorldWindowConfig {}

/// 🔺️ Sparse delta of one world window's preferences: only the sub-records a mutation actually changes.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CadWorldWindowConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera: Option<CadCamera>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub sun: Option<CadSunConfig>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub dislocate_options: Option<CadDislocateOptions>,
}

impl protocol::MutationDiff<CadWorldWindowConfig> for CadWorldWindowConfigDiff {
    fn apply(&self, base: &CadWorldWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<CadWorldWindowConfig> {
        Ok(CadWorldWindowConfig { camera: self.camera.clone().unwrap_or_else(|| base.camera.clone()), sun: self.sun.clone().unwrap_or_else(|| base.sun.clone()), dislocate_options: self.dislocate_options.unwrap_or(base.dislocate_options) })
    }
    fn absorb(&mut self, other: Self) {
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        if other.sun.is_some() {
            self.sun = other.sun;
        }
        if other.dislocate_options.is_some() {
            self.dislocate_options = other.dislocate_options;
        }
    }
}

impl protocol::DiffAlgebra<CadWorldWindowConfig> for CadWorldWindowConfigDiff {
    fn inverse(&self, base: &CadWorldWindowConfig) -> Self {
        Self { camera: self.camera.as_ref().map(|_| base.camera.clone()), sun: self.sun.as_ref().map(|_| base.sun.clone()), dislocate_options: self.dislocate_options.map(|_| base.dislocate_options) }
    }
    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

/// 🎚️ The whole-record payload of [`CadWorldWindowConfigMutation::Set`]; its wire is `{"config": …}`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set")]
pub struct CadWorldWindowConfigSet {
    #[dsl(block)]
    pub config: Box<CadWorldWindowConfig>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum CadWorldWindowConfigMutation {
    #[dsl(key = "set")]
    Set(CadWorldWindowConfigSet),
}

impl store::snapshot_clone_preparation::ConfigApplyMutation<CadWorldWindowConfig> for CadWorldWindowConfigMutation {
    fn exchange(self, post: &mut CadWorldWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        let Self::Set(CadWorldWindowConfigSet { mut config }) = self;
        std::mem::swap(config.as_mut(), post);
        Ok(Self::Set(CadWorldWindowConfigSet { config }))
    }
}

impl protocol::OpText for CadWorldWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown CAD world-window mutation '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let spec = <Self as semio_framework_dsl_record::DslVariants>::variants().iter().find(|(key, _)| key == &keyword).map(|(_, spec)| (spec.ordinary)()).expect("world-window mutation variant");
        semio_framework_dsl_record::print(&record, &spec, semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for CadWorldWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}

impl Mutation<CadWorldWindowConfig> for CadWorldWindowConfigMutation {
    type Diff = CadWorldWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set CAD World Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Set",
        payload_schema: "cad.worldwindowconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, base: &CadWorldWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        let Self::Set(CadWorldWindowConfigSet { config }) = self;
        let diff = CadWorldWindowConfigDiff {
            camera: (base.camera != config.camera).then(|| config.camera.clone()),
            sun: (base.sun != config.sun).then(|| config.sun.clone()),
            dislocate_options: (base.dislocate_options != config.dislocate_options).then_some(config.dislocate_options),
        };
        if diff == CadWorldWindowConfigDiff::default() {
            return protocol::MutationOutcome::new(diff).warning("mutation.no-op", "CAD world-window configuration is already up to date.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &CadWorldWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::Set(CadWorldWindowConfigSet { config: Box::new(base.clone()) })])
    }
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
            type Edit = store::snapshot_clone_preparation::ConfigApplyEdit<crate::editor::cad::modes::edit::windows::config::CadWorldWindowConfig, crate::editor::cad::modes::edit::windows::config::CadWorldWindowConfigMutation>;
            const MAXIMUM_PREPARATION_DEPTH: usize = 64;
            fn build_retained_edit() -> std::sync::Arc<Self::Edit> {
                std::sync::Arc::new(store::snapshot_clone_preparation::ConfigApplyEdit::new())
            }
            fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
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
    let mutation = CadWorldWindowConfigMutation::Set(CadWorldWindowConfigSet { config: Box::new(config) });
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
#[path = "🧪️tests/🔬️window/🦀️.rs"]
mod tests;

//#region 🪢️TaxonomyMounts
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
//#endregion 🪢️TaxonomyMounts
