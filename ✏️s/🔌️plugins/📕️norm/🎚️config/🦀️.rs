//! 🧮️ Norm plugin — the ONE view-state config artifact every one of the fifteen norm apps uses.
//!
//! 📌️ Deliberately NOT a per-surface `✏️editor/🎚️config/🦀️.rs`: all fifteen compliance apps have the
//! identical config shape (one field — which `CheckReport::checks` row the inspection panel points at),
//! so unlike `shooting`'s per-app `ShootingConfig` this is ONE type reused by every app rather than
//! fifteen byte-identical copies. It lives in `🫀️core` (the cross-artifact/cross-app kernel) because
//! that is the shallowest taxonomy node common to every consumer — the same "put shared declarations at
//! the shallowest common ancestor" rule the migration template states for shared window options.

pub use crate::document::NormHost;
pub use super::mutations::{change_selected_check_index::ChangeSelectedCheckIndex, NormConfigMutation};
pub use super::schema::NormConfig;

//#region 🔖️Config
//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for NormConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for NormConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec

impl store::ConfigRecord for NormConfig {}

/// 🧮️ Resolved one-field configuration projection produced by its semantic mutation.
impl protocol::MutationDiff<NormConfig> for NormConfig {
    fn apply(&self, _base: &NormConfig) -> protocol::MutationApplyResult<NormConfig> {
        Ok({ self.clone() })
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Config

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn norm_config_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&NormConfig::default());
        store::os_store::test_support::assert_dsl_round_trip(&NormConfig { selected_check_index: Some(3) });
    }

    #[semio_framework_async_macros::async_test]
    fn norm_config_dsl_pack_equivalence() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&NormConfig::default());
        store::os_store::test_support::assert_dsl_pack_equivalence(&NormConfig { selected_check_index: Some(7) });
    }

}
//#endregion 🧪️Tests
