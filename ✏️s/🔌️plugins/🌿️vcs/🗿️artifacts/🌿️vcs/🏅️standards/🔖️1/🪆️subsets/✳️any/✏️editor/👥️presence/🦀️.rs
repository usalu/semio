//! 👥️ VCS presence — shareable live ephemeral state + mutations.
//!
//! Empty: the VCS play demo keeps all view state in [`crate::editor::vcs::config::VcsDemoConfig`]; there is
//! no separate shareable live surface (history selection is framework interaction state).

use protocol::Mutation;
use store::ArtifactPack;

//#region 🔖️Presence
#[derive(Clone, Debug, PartialEq, Default, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct VcsDemoPresence {}

/// 🔺️ Sparse delta over [`VcsDemoPresence`]: the presence record carries no field, so its delta names nothing.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct VcsDemoPresenceDiff {}

impl protocol::MutationDiff<VcsDemoPresence> for VcsDemoPresenceDiff {
    fn apply(&self, base: &VcsDemoPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<VcsDemoPresence> {
        Ok(base.clone())
    }
    fn absorb(&mut self, _other: Self) {}
}

impl protocol::DiffAlgebra<VcsDemoPresence> for VcsDemoPresenceDiff {
    fn inverse(&self, _base: &VcsDemoPresence) -> Self {
        Self {}
    }
    fn is_empty(&self) -> bool {
        true
    }
}

impl store::ArtifactDsl for VcsDemoPresence {
    const EXTENSION: &'static str = "vcs.presence";
    fn envelope_id() -> &'static str {
        "vcs.presence"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "vcs presence", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for VcsDemoPresence {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        if !inner.is_empty() {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"vcs presence pack must be empty")));
        }
        Ok(Self::default())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum VcsDemoPresenceMutation {
    Noop,
}

impl Mutation<VcsDemoPresence> for VcsDemoPresenceMutation {
    type Diff = VcsDemoPresenceDiff;

    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/⏸️noop",
        semantic_kind: "noop",
        display_name: "Noop",
        emoji: "⏸️",
        aggregate_variant: "Noop",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::SelfInvertible,
        diff_participation: protocol::MutationDiffParticipation::None,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::Noop => &Self::DESCRIPTORS[0],
        }
    }

    fn diff(&self, _base: &VcsDemoPresence) -> protocol::MutationOutcome<VcsDemoPresenceDiff> {
        protocol::MutationOutcome::new(VcsDemoPresenceDiff {})
    }

    fn inverse(&self, _base: &VcsDemoPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![VcsDemoPresenceMutation::Noop]
    
    })())
}
}

impl protocol::OpText for VcsDemoPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = semio_framework_dsl_record::parse(body, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for VcsDemoPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
