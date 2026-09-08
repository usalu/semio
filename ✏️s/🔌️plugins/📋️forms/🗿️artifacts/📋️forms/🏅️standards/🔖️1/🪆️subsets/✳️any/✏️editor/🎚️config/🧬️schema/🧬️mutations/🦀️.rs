//! 🧬️ Forms configuration mutation collection.

use super::*;
#[path = "📸️replace-config/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "👣️set-step-index/🦀️.rs"]
mod set_step_index;
pub use set_step_index::SetStepIndex;
#[path = "📦️stage-try-value-chunk/🦀️.rs"]
mod stage_try_value_chunk;
pub use stage_try_value_chunk::StageTryValueChunk;
#[path = "🗑️discard-try-value-staging/🦀️.rs"]
mod discard_try_value_staging;
pub use discard_try_value_staging::DiscardTryValueStaging;
#[path = "🔎️verify-try-value-chunk/🦀️.rs"]
mod verify_try_value_chunk;
pub use verify_try_value_chunk::VerifyTryValueChunk;
#[path = "🎯️commit-try-value/🦀️.rs"]
mod commit_try_value;
pub use commit_try_value::CommitTryValue;
#[path = "🗃️stage-try-values-entry/🦀️.rs"]
mod stage_try_values_entry;
pub use stage_try_values_entry::StageTryValuesEntry;
#[path = "🧹️discard-try-values-batch/🦀️.rs"]
mod discard_try_values_batch;
pub use discard_try_values_batch::DiscardTryValuesBatch;
#[path = "✅️commit-try-values-batch/🦀️.rs"]
mod commit_try_values_batch;
pub use commit_try_values_batch::CommitTryValuesBatch;
#[path = "🧽️clear-try-values/🦀️.rs"]
mod clear_try_values;
pub use clear_try_values::ClearTryValues;
#[path = "🗣️set-locale/🦀️.rs"]
mod set_locale;
pub use set_locale::SetLocale;
#[path = "🧩️set-contributions/🦀️.rs"]
mod set_contributions;
pub use set_contributions::SetContributions;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = FormsConfig, diff = FormsConfig, schema = "forms.config")]
pub enum FormsConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
    #[dsl(key = "set-step-index")]
    SetStepIndex(SetStepIndex),
    #[dsl(key = "stage-try-value-chunk")]
    StageTryValueChunk(StageTryValueChunk),
    #[dsl(key = "discard-try-value-staging")]
    DiscardTryValueStaging(DiscardTryValueStaging),
    #[dsl(key = "verify-try-value-chunk")]
    VerifyTryValueChunk(VerifyTryValueChunk),
    #[dsl(key = "commit-try-value")]
    CommitTryValue(CommitTryValue),
    #[dsl(key = "stage-try-values-entry")]
    StageTryValuesEntry(StageTryValuesEntry),
    #[dsl(key = "discard-try-values-batch")]
    DiscardTryValuesBatch(DiscardTryValuesBatch),
    #[dsl(key = "commit-try-values-batch")]
    CommitTryValuesBatch(CommitTryValuesBatch),
    #[dsl(key = "clear-try-values")]
    ClearTryValues(ClearTryValues),
    #[dsl(key = "set-locale")]
    SetLocale(SetLocale),
    #[dsl(key = "set-contributions")]
    SetContributions(SetContributions),
}

impl protocol::OpText for FormsConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for FormsConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let operation = <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })?;
        if let FormsConfigMutation::StageTryValueChunk(StageTryValueChunk { staging_id, chunk, .. }) = &operation {
            if staging_id.len() > MAX_TRY_VALUE_OPERATION_ID_BYTES || chunk.len() > MAX_STAGED_TRY_VALUE_CHUNK_BYTES {
                return Err(protocol::ProtocolError::Malformed { what: "forms try-value chunk", offset: reader.position() as u64, detail: "staging id or chunk exceeds the bounded operation limit".into() });
            }
        }
        Ok(operation)
    }
}

//#endregion 🔖️OpCodec

