//! 📋️ Persisted local selection for one exact Architect Register window.

use semio_framework_value_derive::{FromValue, ToValue};

/// 📋️ Selects the document register rendered by one concrete Register window.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArchitectRegisterWindowConfig {
    pub active_register: String,
}

impl Default for ArchitectRegisterWindowConfig {
    fn default() -> Self {
        Self { active_register: "elements".into() }
    }
}

/// 🔁️ Changes the selected register of one addressed Register window.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum ArchitectRegisterWindowConfigMutation {
    SetActiveRegister { active_register: String },
}

impl protocol::Mutation<ArchitectRegisterWindowConfig> for ArchitectRegisterWindowConfigMutation {
    type Diff = ArchitectRegisterWindowConfig;
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
        outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::Warning],
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
            return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Register selection is unchanged.");
        }
        protocol::MutationOutcome::new(ArchitectRegisterWindowConfig { active_register: active_register.clone() })
    }

    fn inverse(&self, base: &ArchitectRegisterWindowConfig) -> Vec<Self> {
        let Self::SetActiveRegister { active_register } = self;
        (base.active_register != *active_register)
            .then(|| Self::SetActiveRegister { active_register: base.active_register.clone() })
            .into_iter()
            .collect()
    }
}

impl store::ArtifactDsl for ArchitectRegisterWindowConfig {
    const EXTENSION: &'static str = "architectregisterwindowcfg";

    fn envelope_id() -> &'static str {
        "s.architect.program.register-window.config"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let json: serde_json::Value = serde_json::from_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_string_pretty(&value).expect("Architect Register window config JSON");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Architect Register window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ArchitectRegisterWindowConfig {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }

    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("Architect Register window config pack envelope mismatch".into()));
        }
        let json: serde_json::Value = serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }

    fn record_spec() -> Option<dsl::RecordSpec> {
        None
    }
}

store::impl_whole_record_config!(ArchitectRegisterWindowConfig);

impl protocol::OpText for ArchitectRegisterWindowConfigMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }

    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for ArchitectRegisterWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
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
