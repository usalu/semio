//! 📋️ Persisted local selection for one exact Architect Register window.

use semio_framework_value_derive::{FromValue, ToValue};

/// 📋️ Selects the document register rendered by one concrete Register window.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "s.architect.program.register-window.config", extension = "architectregisterwindowcfg")]
pub struct ArchitectRegisterWindowConfig {
    pub active_register: String,
}

impl Default for ArchitectRegisterWindowConfig {
    fn default() -> Self {
        Self { active_register: "elements".into() }
    }
}

/// 🔺️ Sparse field diff of one register selection window config: a present field is written, the rest of the config is untouched.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct ArchitectRegisterWindowConfigDiff {
    pub active_register: Option<String>,
}

impl protocol::MutationDiff<ArchitectRegisterWindowConfig> for ArchitectRegisterWindowConfigDiff {
    fn apply(&self, base: &ArchitectRegisterWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<ArchitectRegisterWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.active_register {
            next.active_register = value.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.active_register.is_some() {
            self.active_register = other.active_register;
        }
    }
}

impl protocol::DiffAlgebra<ArchitectRegisterWindowConfig> for ArchitectRegisterWindowConfigDiff {
    fn inverse(&self, base: &ArchitectRegisterWindowConfig) -> Self {
        Self { active_register: self.active_register.as_ref().map(|_| base.active_register.clone()) }
    }

    fn between(base: &ArchitectRegisterWindowConfig, other: &ArchitectRegisterWindowConfig) -> Self {
        Self { active_register: (base.active_register != other.active_register).then(|| other.active_register.clone()) }
    }

    fn is_empty(&self) -> bool {
        self.active_register.is_none()
    }
}

/// 🔁️ Changes the selected register of one addressed Register window.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum ArchitectRegisterWindowConfigMutation {
    SetActiveRegister { active_register: String },
}

impl protocol::Mutation<ArchitectRegisterWindowConfig> for ArchitectRegisterWindowConfigMutation {
    type Diff = ArchitectRegisterWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config",
        semantic_kind: "set-active-register",
        display_name: "Set Architect Register Window Selection",
        emoji: "📋️",
        aggregate_variant: "SetActiveRegister",
        payload_schema: "architect.register-window.config",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[
            protocol::MutationLanguageSurface::Rust,
            protocol::MutationLanguageSurface::Typescript,
            protocol::MutationLanguageSurface::JsonSchema,
            protocol::MutationLanguageSurface::Graphql,
            protocol::MutationLanguageSurface::Protobuf,
            protocol::MutationLanguageSurface::Text,
            protocol::MutationLanguageSurface::Binary,
        ],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }

    fn diff(&self, base: &ArchitectRegisterWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        let Self::SetActiveRegister { active_register } = self;
        if base.active_register == *active_register {
            return protocol::MutationOutcome::new(ArchitectRegisterWindowConfigDiff::default()).warning("mutation.no-op", "Register selection is unchanged.");
        }
        protocol::MutationOutcome::new(ArchitectRegisterWindowConfigDiff { active_register: Some(active_register.clone()) })
    }

    fn inverse(&self, base: &ArchitectRegisterWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        let Self::SetActiveRegister { active_register } = self;
        (base.active_register != *active_register)
            .then(|| Self::SetActiveRegister { active_register: base.active_register.clone() })
            .into_iter()
            .collect()
    
    })())
}
}

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for ArchitectRegisterWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Architect register window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// this window kind with `WindowConfigPackLoadDiagnostic::TypedState`.
impl store::ArtifactPack for ArchitectRegisterWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ConfigRecord for ArchitectRegisterWindowConfig {}

impl protocol::OpText for ArchitectRegisterWindowConfigMutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for ArchitectRegisterWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

pub struct ArchitectRegisterWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for ArchitectRegisterWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::ARCHITECT_WINDOW_REGISTER;
    const SCHEMA: &'static str = "architect.register-window.config";
    const MAXIMUM_PUBLICATION_BYTES: usize = 1_024;
    type State = ArchitectRegisterWindowConfig;
    type Mutation = ArchitectRegisterWindowConfigMutation;

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

/// 🎯️ Reads the caller's exact Register-window configuration or its initial value.
pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> ArchitectRegisterWindowConfig {
    view.window::<ArchitectRegisterWindowConfigOwner>().cloned().unwrap_or_default()
}

/// 📬️ Addresses a selection change to the caller's concrete Register window.
pub fn addressed(view: &semio_framework_plugin::ViewModel, active_register: String) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("architect-register-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("architect-register-window-stale"))?;
    if kind != super::ARCHITECT_WINDOW_REGISTER {
        return Err(semio_framework_plugin::Fault::from("architect-register-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<ArchitectRegisterWindowConfigOwner>(
        id,
        ArchitectRegisterWindowConfigMutation::SetActiveRegister { active_register },
    ))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window-ownership/🦀️.rs"]
mod window_ownership_tests;
