//! 👥️ Fem2dPresence — empty shareable live state (selection is command-transient; camera/results live in config).

use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Fem2dPresence has no shareable live fields beyond config: selection is transient command payload,
/// and camera / result display already live on `Fem2dConfig` as `local-ui`.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dPresence {}

impl protocol::MutationDiff<Fem2dPresence> for Fem2dPresence {
    fn apply(&self, base: &Fem2dPresence) -> protocol::MutationApplyResult<Fem2dPresence> {
        Ok({
            base.clone()
        })
    }
    fn absorb(&mut self, _other: Self) {}
}

impl store::ArtifactDsl for Fem2dPresence {
    const EXTENSION: &'static str = "fem2d.presence";
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(store::TextError::new("no presence fields", store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for Fem2dPresence {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    fn decode_pack_with(_bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        Ok(Self::default())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(rename_all = "camelCase")]
pub enum Fem2dPresenceMutation {
    Noop,
}

impl Mutation<Fem2dPresence> for Fem2dPresenceMutation {
    type Diff = Fem2dPresence;

    fn diff(&self, _base: &Fem2dPresence) -> protocol::MutationOutcome<Fem2dPresence> {
        protocol::MutationOutcome::new(Fem2dPresence::default())
    }

    fn inverse(&self, _base: &Fem2dPresence) -> Vec<Self> {
        vec![Fem2dPresenceMutation::Noop]
    }
}

impl protocol::OpText for Fem2dPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
    fn print_op(&self) -> String {
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

impl protocol::OpBinary for Fem2dPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
