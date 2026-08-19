//! 👥️ VCS presence — shareable live ephemeral state + mutations.
//!
//! Empty: the VCS play demo keeps all view state in [`crate::editor::vcs::config::VcsDemoConfig`]; there is
//! no separate shareable live surface (history selection and locale are local config).

use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🔖️Presence
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VcsDemoPresence {}

impl protocol::MutationDiff<VcsDemoPresence> for VcsDemoPresence {
    async fn apply(&self, base: &VcsDemoPresence) -> protocol::MutationApplyResult<VcsDemoPresence> {
        Ok({
            base.clone()
        })
    }
    async fn absorb(&mut self, _other: Self) {}
}

impl store::ArtifactDsl for VcsDemoPresence {
    const EXTENSION: &'static str = "vcs.presence";
    async fn envelope_id() -> &'static str {
        "vcs.presence"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(store::TextError::new("vcs presence", store::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for VcsDemoPresence {
    async fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    async fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        if !inner.is_empty() {
            return Err(store::PackError::Schema("vcs presence pack must be empty".into()));
        }
        Ok(Self::default())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(rename_all = "camelCase")]
pub enum VcsDemoPresenceMutation {
    Noop,
}

impl Mutation<VcsDemoPresence> for VcsDemoPresenceMutation {
    type Diff = VcsDemoPresence;

    async fn diff(&self, _base: &VcsDemoPresence) -> protocol::MutationOutcome<VcsDemoPresence> {
        protocol::MutationOutcome::new(VcsDemoPresence::default())
    }

    async fn inverse(&self, _base: &VcsDemoPresence) -> Vec<Self> {
        vec![VcsDemoPresenceMutation::Noop]
    }
}

impl protocol::OpText for VcsDemoPresenceMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() {
                    line[keyword.len()..].trim_start()
                } else {
                    ""
                };
                let record = dsl::parse(
                    body,
                    &spec_fn(),
                    &dsl::ParseOptions {
                        limits: dsl::Limits::default(),
                        mode: dsl::SourceMode::Inline,
                    },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants
            .iter()
            .find(|(k, _)| k == &keyword)
            .map(|(_, s)| *s)
            .expect("variant spec must exist for its own keyword");
        let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for VcsDemoPresenceMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
