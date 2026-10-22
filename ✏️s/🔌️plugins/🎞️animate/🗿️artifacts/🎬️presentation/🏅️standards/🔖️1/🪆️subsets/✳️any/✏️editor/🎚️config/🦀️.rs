//! 🧮️ Animate presentation app — view state (`PresentationConfig`) and its operation enum
//! (`PresentationConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.presentation` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so engagement edits are VCS'd exactly
//! like document content.

#[cfg(test)]
use protocol::Mutation;

//#region 🔖️Config
/// 🧮️ B1: animate presentation's real `ArtifactApp::Config` — absorbs every former
/// `AnimatePresentationPlayRuntime` field (`engagement_input`). Locale and terminology come from the
/// shared host `ViewModel` (see `crate::editor::animate::terminology`).
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "presentcfg")]
#[artifact(id = "presentation.config")]
#[dsl(layout = "lines")]
#[derive(Default)]
pub struct PresentationConfig {
    /// ⌨️ In-progress engagement-bar input draft — was `AnimatePresentationPlayRuntime::engagement_input`.
    pub engagement_input: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl pack::snapshot::ArtifactDsl for PresentationConfig {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl pack::snapshot::ArtifactPack for PresentationConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for PresentationConfig {

}

//#endregion 🔖️ArtifactCodec



impl store::ConfigRecord for PresentationConfig {}

/// 🔺️ Sparse field delta over [`PresentationConfig`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub engagement_input: Option<String>,
}

impl protocol::MutationDiff<PresentationConfig> for PresentationConfigDiff {}

impl protocol::ColdMutationDiff<PresentationConfig> for PresentationConfigDiff {
    fn absorb(&mut self, other: Self) {
        if other.engagement_input.is_some() {
            self.engagement_input = other.engagement_input;
        }
    }

    fn apply(&self, unpublished: &mut PresentationConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        *unpublished = next;
        Ok(())
    }
}

impl protocol::DiffAlgebra<PresentationConfig> for PresentationConfigDiff {
    fn is_empty(&self) -> bool {
        self.engagement_input.is_none()
    }
}

impl protocol::ColdDiffAlgebra<PresentationConfig> for PresentationConfigDiff {
    fn inverse(&self, base: &PresentationConfig) -> Self {
        Self {
            engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()),
        }
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



//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
//#region 🔖️Domain
/// 📐️ Normalized `x,y,width,height` rect — always reached through a `#[dsl(block)]` field (see
/// {@link FigureTileSource}/{@link FigureTileDraft}), so it declares no `#[dsl(keyword)]` of its own.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FigureTileFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FigureTileSource {
    pub src: String,
    pub kind: String,
    #[dsl(block)]
    pub frame: FigureTileFrame,
    #[value(skip_serializing_if = "Option::is_none")]
    pub source_aspect: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub pdf_page: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FigureTileDraft {
    pub id: String,
    pub name: String,
    #[dsl(block)]
    pub crop: FigureTileFrame,
}
//#endregion 🔖️Domain
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems13
pub mod artifact_items_13 {
use crate::{FigureTileDraft, FigureTileFrame};
use protocol::{Identified, Patchable};

//#region 🔖️CollectionSupport
impl Identified<String> for FigureTileDraft {
    fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct FigureTileDraftPatch {
    pub name: Option<String>,
    #[dsl(block)]
    pub crop: Option<FigureTileFrame>,
}

impl Patchable<FigureTileDraftPatch> for FigureTileDraft {
    fn apply_patch(&mut self, patch: &FigureTileDraftPatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(crop) = &patch.crop {
            self.crop = crop.clone();
        }
    }
}
//#endregion 🔖️CollectionSupport
}
//#endregion 🧩️ArtifactItems13
