//! 🧮️ Playbook play app — view state (`PlaybookConfig`) and its operation enum (`PlaybookConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.playbook` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`). 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the former app-struct `RefCell<Vec<String>>`
//! selection state that B1 had absorbed here as `selected_ids` moved OUT again, into the framework's own
//! `InteractionState` (the "blocks" domain, declared on `PlaybookPlayApp`'s manifest) — this config no
//! longer carries any selection.

#[cfg(test)]
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Config
/// 🧮️ `PlaybookPlayApp::Config` — the pure-trait `ArtifactEditor::Config` for the playbook app.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "playbookcfg")]
#[dsl(layout = "lines")]
pub struct PlaybookConfig {
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `playbook.blockKind` hot-swap installs.
    #[value(default = "default_contributions_json")]
    pub contributions_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for PlaybookConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        "playbook.config"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for PlaybookConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec

fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for PlaybookConfig {
    fn default() -> Self {
        Self { contributions_json: default_contributions_json() }
    }
}

impl store::ConfigRecord for PlaybookConfig {}

/// 🔺️ Sparse field delta over [`PlaybookConfig`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct PlaybookConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub contributions_json: Option<String>,
}

impl protocol::MutationDiff<PlaybookConfig> for PlaybookConfigDiff {
    fn apply(&self, base: &PlaybookConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<PlaybookConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.contributions_json {
            next.contributions_json = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.contributions_json.is_some() {
            self.contributions_json = other.contributions_json;
        }
    }
}

impl protocol::DiffAlgebra<PlaybookConfig> for PlaybookConfigDiff {
    fn inverse(&self, base: &PlaybookConfig) -> Self {
        Self {
            contributions_json: self.contributions_json.as_ref().map(|_| base.contributions_json.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.contributions_json.is_none()
    }
}

//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️contract-vectors/🦀️.rs"]
mod contract_vectors;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
