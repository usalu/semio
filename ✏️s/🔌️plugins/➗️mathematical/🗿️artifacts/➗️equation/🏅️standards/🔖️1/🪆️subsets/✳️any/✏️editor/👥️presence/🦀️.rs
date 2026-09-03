//! 👥️ Equation presence — shareable live ephemeral state + mutations.

use protocol::Mutation;
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ No shareable live surface state yet — graph edits are document mutations and viewport/locale live in config.
#[derive(Clone, Debug, PartialEq, Default, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase")]
pub struct EquationPresence {}

impl protocol::MutationDiff<EquationPresence> for EquationPresence {
    async fn apply(&self, base: &EquationPresence) -> protocol::MutationApplyResult<EquationPresence> {
        Ok({ base.clone() })
    }
    async fn absorb(&mut self, _other: Self) {}
}

impl store::ArtifactDsl for EquationPresence {
    const EXTENSION: &'static str = "equation.presence";
    async fn envelope_id() -> &'static str {
        "mathematical.equation.presence"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(store::TextError::new("equation presence is empty", store::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for EquationPresence {
    async fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    async fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        Err(store::PackError::Schema("equation presence pack must be empty".into()))
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslOps)]
#[value(rename_all = "camelCase")]
pub enum EquationPresenceMutation {
    Noop,
}

impl Mutation<EquationPresence> for EquationPresenceMutation {
    type Diff = EquationPresence;

    async fn diff(&self, _base: &EquationPresence) -> protocol::MutationOutcome<EquationPresence> {
        protocol::MutationOutcome::new(EquationPresence::default())
    }

    async fn inverse(&self, _base: &EquationPresence) -> Vec<Self> {
        vec![EquationPresenceMutation::Noop]
    }
}

impl protocol::OpText for EquationPresenceMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for EquationPresenceMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
