//! 🫧️ Generation3dViewTransient — the read-only surface's ephemeral LOCAL-ONLY state.
//!
//! One item: the evaluated flow output the preview window paints from. It is local (never shared —
//! that is `👥️presence`), ephemeral (never persisted — that is `🎚️config`) and derived (a pure
//! function of the document), which is exactly the lane definition. Populating it is what stops the
//! preview window from re-evaluating the whole fixture on every single repaint.
//!
//! 🕹️ Hover and selection are deliberately NOT here. They are the framework's own interaction
//! mechanism (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): hover lives in the
//! ephemeral `InteractionHoverState` and selection in the persisted `protocol::InteractionState`,
//! both read through `InteractionView` in `render_with_request_context`. A surface that copied them
//! into its own transient would own two sources of truth for one fact.

use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Transient
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "generation3dview.transient")]
#[dsl(layout = "lines")]
pub struct Generation3dViewTransient {
    /// 🧮️ `FlowEvalSession::eval_json()` for the current document — `None` until the first command
    /// evaluated it, in which case the preview window evaluates once inline instead.
    pub preview_eval_text: Option<String>,
}

impl store::ArtifactDsl for Generation3dViewTransient {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Generation3d viewer transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Generation3dViewTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
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

impl protocol::MutationDiff<Generation3dViewTransient> for Generation3dViewTransient {
    fn apply(&self, _base: &Generation3dViewTransient) -> protocol::MutationApplyResult<Generation3dViewTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

// 🧹️ The explicit retirement ladder both this surface's APP-level transient lane and its preview
// WINDOW-level one publish through (`🪟️windows/👁️preview/🫧️transient`): a published evaluation is
// the largest ephemeral value this viewer owns, so the store retires its bytes under a grant
// rather than dropping them (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
store::artifact_retire_struct!(Generation3dViewTransient { preview_eval_text });
//#endregion 🔖️Transient

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;
