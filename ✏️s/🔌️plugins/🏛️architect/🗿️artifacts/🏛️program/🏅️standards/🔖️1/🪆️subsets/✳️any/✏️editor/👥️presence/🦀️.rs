//! 👥️ Architect presence — shareable live ephemeral state + mutations.
//!
//! 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: peer selection no longer lives
//! here — it broadcasts automatically via the framework's typed `PresenceInteraction` (assembled
//! from the "program" domain's `InteractionState`, zero app code).

use crate::registers::AdjacencyKind;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of architect view state (active register, adjacency filter, graph camera).
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "architect.presence")]
#[dsl(layout = "lines")]
pub struct ArchitectPresence {
    pub active_register: String,
    pub adjacency_kind_filter: Option<AdjacencyKind>,
    pub graph_camera_x: f64,
    pub graph_camera_y: f64,
    pub graph_camera_zoom: f64,
}

impl Default for ArchitectPresence {
    fn default() -> Self {
        Self { active_register: "elements".into(), adjacency_kind_filter: None, graph_camera_x: 0.0, graph_camera_y: 0.0, graph_camera_zoom: 1.0 }
    }
}

/// 🎯️ Sets the optional adjacency-kind filter to `value`, `None` clearing it — a present edit is how a diff writes an optional field,
/// so clearing the filter stays distinguishable from leaving it alone.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct AdjacencyKindFilterSet {
    pub value: Option<AdjacencyKind>,
}

/// 🔺️ Sparse field diff of the architect presence: each present field is written, the rest of the presence is untouched.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ArchitectPresenceDiff {
    pub active_register: Option<String>,
    pub adjacency_kind_filter: Option<AdjacencyKindFilterSet>,
    pub graph_camera_x: Option<f64>,
    pub graph_camera_y: Option<f64>,
    pub graph_camera_zoom: Option<f64>,
}

impl protocol::MutationDiff<ArchitectPresence> for ArchitectPresenceDiff {
    fn apply(&self, base: &ArchitectPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<ArchitectPresence> {
        let mut next = base.clone();
        if let Some(value) = &self.active_register {
            next.active_register = value.clone();
        }
        if let Some(filter) = &self.adjacency_kind_filter {
            next.adjacency_kind_filter = filter.value.clone();
        }
        if let Some(value) = self.graph_camera_x {
            next.graph_camera_x = value;
        }
        if let Some(value) = self.graph_camera_y {
            next.graph_camera_y = value;
        }
        if let Some(value) = self.graph_camera_zoom {
            next.graph_camera_zoom = value;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.active_register.is_some() {
            self.active_register = other.active_register;
        }
        if other.adjacency_kind_filter.is_some() {
            self.adjacency_kind_filter = other.adjacency_kind_filter;
        }
        if other.graph_camera_x.is_some() {
            self.graph_camera_x = other.graph_camera_x;
        }
        if other.graph_camera_y.is_some() {
            self.graph_camera_y = other.graph_camera_y;
        }
        if other.graph_camera_zoom.is_some() {
            self.graph_camera_zoom = other.graph_camera_zoom;
        }
    }
}

impl protocol::DiffAlgebra<ArchitectPresence> for ArchitectPresenceDiff {
    fn inverse(&self, base: &ArchitectPresence) -> Self {
        Self {
            active_register: self.active_register.as_ref().map(|_| base.active_register.clone()),
            adjacency_kind_filter: self.adjacency_kind_filter.as_ref().map(|_| AdjacencyKindFilterSet { value: base.adjacency_kind_filter.clone() }),
            graph_camera_x: self.graph_camera_x.map(|_| base.graph_camera_x),
            graph_camera_y: self.graph_camera_y.map(|_| base.graph_camera_y),
            graph_camera_zoom: self.graph_camera_zoom.map(|_| base.graph_camera_zoom),
        }
    }

    fn between(base: &ArchitectPresence, other: &ArchitectPresence) -> Self {
        Self {
            active_register: (base.active_register != other.active_register).then(|| other.active_register.clone()),
            adjacency_kind_filter: (base.adjacency_kind_filter != other.adjacency_kind_filter).then(|| AdjacencyKindFilterSet { value: other.adjacency_kind_filter.clone() }),
            graph_camera_x: (base.graph_camera_x != other.graph_camera_x).then_some(other.graph_camera_x),
            graph_camera_y: (base.graph_camera_y != other.graph_camera_y).then_some(other.graph_camera_y),
            graph_camera_zoom: (base.graph_camera_zoom != other.graph_camera_zoom).then_some(other.graph_camera_zoom),
        }
    }

    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

impl store::ArtifactDsl for ArchitectPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for ArchitectPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
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
//#endregion 🔖️Presence

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️contract-vectors/🦀️.rs"]
mod contract_vectors;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
