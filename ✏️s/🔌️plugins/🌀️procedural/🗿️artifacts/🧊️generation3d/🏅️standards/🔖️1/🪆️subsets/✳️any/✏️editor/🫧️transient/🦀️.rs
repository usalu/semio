//! 🫧️ Generation3d app-local computed preview state shared by every generation window.

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_os_kernel::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "generation.3dtransient")]
#[dsl(layout = "lines")]
pub struct Generation3dTransient {
    pub generation_preview_text: Option<String>,
}

impl store::ArtifactDsl for Generation3dTransient {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Generation3d transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Generation3dTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}


#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path = "🚪️io/🦀️.rs"]
pub mod io;

/// 🩹 Owned-field diff of [`Generation3dTransient`]: exactly the fields a leaf sets.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dTransientPatch {
    pub generation_preview_text: Option<Generation3dPreviewTextChange>,
}

/// 🔺️ One change of the nullable `generation_preview_text`: the inner `None` clears it.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Generation3dPreviewTextChange {
    pub text: Option<String>,
}

impl protocol::MutationDiff<Generation3dTransient> for Generation3dTransientPatch {
    fn apply(&self, base: &Generation3dTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Generation3dTransient> {
        Ok(Generation3dTransient {
            generation_preview_text: self.generation_preview_text.clone().map_or_else(|| base.generation_preview_text.clone(), |change| change.text),
            ..base.clone()
        })
    }
    fn absorb(&mut self, other: Self) {
        self.generation_preview_text = other.generation_preview_text.or_else(|| self.generation_preview_text.take());
    }
}

impl protocol::DiffAlgebra<Generation3dTransient> for Generation3dTransientPatch {
    fn inverse(&self, base: &Generation3dTransient) -> Self {
        Self {
            generation_preview_text: self.generation_preview_text.as_ref().map(|_| Generation3dPreviewTextChange { text: base.generation_preview_text.clone() }),
        }
    }
    fn between(base: &Generation3dTransient, other: &Generation3dTransient) -> Self {
        Self {
            generation_preview_text: (base.generation_preview_text != other.generation_preview_text).then(|| Generation3dPreviewTextChange { text: other.generation_preview_text.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        self.generation_preview_text.is_none()
    }
}
