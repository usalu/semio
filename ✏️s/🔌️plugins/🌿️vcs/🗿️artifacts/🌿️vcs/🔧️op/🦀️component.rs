//! ⚡️ VCS artifact — operation enum + laws (was: constitutional `op`).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::vcs::diff::VcsDemoDiff;
use crate::artifacts::vcs::VcsDemoProjection;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum VcsDemoOperation {
    SetCounter { counter: i64 },
    SetTitle { title: String },
    SetNotes { notes: String },
    SetStatus { status: String },
    AddTag { tag: String },
    RemoveTag { tag: String },
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl protocol::OpText for VcsDemoOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
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
impl protocol::OpBinary for VcsDemoOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
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
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec


impl Operation<VcsDemoProjection> for VcsDemoOperation {
    type Diff = VcsDemoDiff;

    fn diff(&self, _projection: &VcsDemoProjection) -> Self::Diff {
        match self {
            VcsDemoOperation::SetCounter { counter } => VcsDemoDiff::SetCounter { counter: *counter },
            VcsDemoOperation::SetTitle { title } => VcsDemoDiff::SetTitle { title: title.clone() },
            VcsDemoOperation::SetNotes { notes } => VcsDemoDiff::SetNotes { notes: notes.clone() },
            VcsDemoOperation::SetStatus { status } => VcsDemoDiff::SetStatus { status: status.clone() },
            VcsDemoOperation::AddTag { tag } => VcsDemoDiff::AddTag { tag: tag.clone() },
            VcsDemoOperation::RemoveTag { tag } => VcsDemoDiff::RemoveTag { tag: tag.clone() },
        }
    }

    fn backwards(&self, projection: &VcsDemoProjection) -> Vec<Self> {
        match self {
            VcsDemoOperation::SetCounter { .. } => vec![VcsDemoOperation::SetCounter { counter: projection.counter }],
            VcsDemoOperation::SetTitle { .. } => vec![VcsDemoOperation::SetTitle { title: projection.title.clone() }],
            VcsDemoOperation::SetNotes { .. } => vec![VcsDemoOperation::SetNotes { notes: projection.notes.clone() }],
            VcsDemoOperation::SetStatus { .. } => vec![VcsDemoOperation::SetStatus { status: projection.status.clone() }],
            VcsDemoOperation::AddTag { tag } => vec![VcsDemoOperation::RemoveTag { tag: tag.clone() }],
            VcsDemoOperation::RemoveTag { tag } => vec![VcsDemoOperation::AddTag { tag: tag.clone() }],
        }
    }
}
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
/// 🔺️ Shared by [`VcsDemoDiff::apply`] (the diff is a thin wrapper around the same field write).
pub fn apply_vcs_demo_operation(projection: &VcsDemoProjection, operation: &VcsDemoOperation) -> VcsDemoProjection {
    let mut next = projection.clone();
    match operation {
        VcsDemoOperation::SetCounter { counter } => next.counter = *counter,
        VcsDemoOperation::SetTitle { title } => next.title = title.clone(),
        VcsDemoOperation::SetNotes { notes } => next.notes = notes.clone(),
        VcsDemoOperation::SetStatus { status } => next.status = status.clone(),
        VcsDemoOperation::AddTag { tag } => {
            if !next.tags.contains(tag) {
                next.tags.push(tag.clone());
            }
        }
        VcsDemoOperation::RemoveTag { tag } => next.tags.retain(|entry| entry != tag),
    }
    next
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcs_demo_operation_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&VcsDemoOperation::SetCounter { counter: 3 });
        store::test_support::assert_op_line_round_trip(&VcsDemoOperation::SetTitle { title: "Untitled".into() });
        store::test_support::assert_op_line_round_trip(&VcsDemoOperation::AddTag { tag: "draft".into() });
    }
}
//#endregion 🧪️Tests
