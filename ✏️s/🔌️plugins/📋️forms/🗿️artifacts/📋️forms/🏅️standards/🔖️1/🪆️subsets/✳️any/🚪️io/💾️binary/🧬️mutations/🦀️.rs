//! ⚖️ Forms artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpBinary for FormMutation` is implemented directly in the shared `playbook` kernel crate;
//! see `🗿️artifacts/📋️forms/🦀️.rs` for why. This component only adds the thin artifact-facing
//! `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law.
//!
//! The app's typed `FormsCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is an APP concern, not an artifact one: it now lives in the `✏️editor` surface's own root
//! `🦀️.rs`, assembled from
//! the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::op::FormMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `FormMutation` to its binary state-patch form.
pub fn encode_op(operation: &FormMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `FormMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<FormMutation, protocol::ProtocolError> {
    FormMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
pub use crate::mutations::FormMutation;
use crate::mutations::{
    change_form_title::mutation::ChangeFormTitle, change_step_description::mutation::ChangeStepDescription, create_block::mutation::CreateBlock, create_step::mutation::CreateStep, delete_block::mutation::DeleteBlock,
    delete_step::mutation::DeleteStep, move_block_to_step::mutation::MoveBlockToStep, rename_step::mutation::RenameStep, reorder_step::mutation::ReorderStep, replace_block::mutation::ReplaceBlock,
    change_block_field::mutation::{BlockField, ChangeBlockField},
};
use crate::{FormQuestion, FormStep};
use crate::mutations::{commit_response::mutation::CommitResponse, discard_response::mutation::DiscardResponse};
pub use mutations_wire_codec::*;
use crate::standards::v1::subsets::any::io::text::mutations::{enc_step,dec_step,enc_block,dec_block};
fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
}

fn write_opt_str_bin(out: &mut Vec<u8>, s: &Option<String>) {
    match s {
        Some(v) => {
            out.push(1);
            write_str_bin(out, v);
        }
        None => out.push(0),
    }
}

fn read_opt_str_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_bin(reader)?)),
        other => Err(format!("bad option tag {other}")),
    }
}

fn write_opt_usize_bin(out: &mut Vec<u8>, v: &Option<usize>) {
    match v {
        Some(x) => {
            out.push(1);
            store::pack_rt::write_varint_u64(out, *x as u64);
        }
        None => out.push(0),
    }
}

fn read_opt_usize_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<usize>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(reader.read_varint_u64().map_err(|e| e.to_string())? as usize)),
        other => Err(format!("bad option tag {other}")),
    }
}
impl protocol::OpBinary for FormMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            FormMutation::CreateStep(_) => 0,
            FormMutation::DeleteStep(_) => 1,
            FormMutation::ReorderStep(_) => 2,
            FormMutation::RenameStep(_) => 3,
            FormMutation::ChangeStepDescription(_) => 4,
            FormMutation::CreateBlock(_) => 5,
            FormMutation::DeleteBlock(_) => 6,
            FormMutation::MoveBlockToStep(_) => 7,
            FormMutation::ReplaceBlock(_) => 8,
            FormMutation::ChangeFormTitle(_) => 9,
            FormMutation::CommitResponse(_) => 10,
            FormMutation::DiscardResponse(_) => 11,
            FormMutation::ChangeBlockField(_) => 12,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            FormMutation::CreateStep(p) => {
                write_str_bin(&mut out, &enc_step(&p.step));
                write_opt_usize_bin(&mut out, &p.index);
            }
            FormMutation::DeleteStep(p) => write_str_bin(&mut out, &p.id),
            FormMutation::ReorderStep(p) => {
                write_str_bin(&mut out, &p.id);
                store::pack_rt::write_varint_u64(&mut out, p.to_index as u64);
            }
            FormMutation::RenameStep(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.new_title);
            }
            FormMutation::ChangeStepDescription(p) => {
                write_str_bin(&mut out, &p.id);
                write_opt_str_bin(&mut out, &p.new_description);
            }
            FormMutation::CreateBlock(p) => {
                write_str_bin(&mut out, &p.step_id);
                write_str_bin(&mut out, &enc_block(&p.block));
                write_opt_usize_bin(&mut out, &p.index);
            }
            FormMutation::DeleteBlock(p) => {
                write_str_bin(&mut out, &p.step_id);
                write_str_bin(&mut out, &p.id);
            }
            FormMutation::MoveBlockToStep(p) => {
                write_str_bin(&mut out, &p.step_id);
                write_str_bin(&mut out, &p.block_id);
                write_str_bin(&mut out, &p.to_step_id);
                store::pack_rt::write_varint_u64(&mut out, p.index as u64);
            }
            FormMutation::ReplaceBlock(p) => {
                write_str_bin(&mut out, &p.step_id);
                write_str_bin(&mut out, &enc_block(&p.block));
            }
            FormMutation::CommitResponse(p) => {
                write_str_bin(&mut out, &semio_framework_pack_json::to_json_string(&p.response));
                write_opt_usize_bin(&mut out, &p.index);
            }
            FormMutation::DiscardResponse(p) => write_str_bin(&mut out, &p.id),
            FormMutation::ChangeFormTitle(p) => write_opt_str_bin(&mut out, &p.new_title),
            FormMutation::ChangeBlockField(p) => {
                write_str_bin(&mut out, &p.block_id);
                write_str_bin(&mut out, &semio_framework_pack_json::to_json_string(&p.change));
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => {
                let step_text = read_str_bin(&mut reader).map_err(|e| malformed("step", reader.position(), e))?;
                let step = dec_step(&step_text).map_err(|e| malformed("step", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(FormMutation::CreateStep(CreateStep { step, index }))
            }
            1 => Ok(FormMutation::DeleteStep(DeleteStep { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            2 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let to_index = reader.read_varint_u64().map_err(|e| malformed("to_index", reader.position(), e.to_string()))? as usize;
                Ok(FormMutation::ReorderStep(ReorderStep { id, to_index }))
            }
            3 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_title = read_str_bin(&mut reader).map_err(|e| malformed("new_title", reader.position(), e))?;
                Ok(FormMutation::RenameStep(RenameStep { id, new_title }))
            }
            4 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_description = read_opt_str_bin(&mut reader).map_err(|e| malformed("new_description", reader.position(), e))?;
                Ok(FormMutation::ChangeStepDescription(ChangeStepDescription { id, new_description }))
            }
            5 => {
                let step_id = read_str_bin(&mut reader).map_err(|e| malformed("step_id", reader.position(), e))?;
                let block_text = read_str_bin(&mut reader).map_err(|e| malformed("block", reader.position(), e))?;
                let block = dec_block(&block_text).map_err(|e| malformed("block", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(FormMutation::CreateBlock(CreateBlock { step_id, block, index }))
            }
            6 => {
                let step_id = read_str_bin(&mut reader).map_err(|e| malformed("step_id", reader.position(), e))?;
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                Ok(FormMutation::DeleteBlock(DeleteBlock { step_id, id }))
            }
            7 => {
                let step_id = read_str_bin(&mut reader).map_err(|e| malformed("step_id", reader.position(), e))?;
                let block_id = read_str_bin(&mut reader).map_err(|e| malformed("block_id", reader.position(), e))?;
                let to_step_id = read_str_bin(&mut reader).map_err(|e| malformed("to_step_id", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("index", reader.position(), e.to_string()))? as usize;
                Ok(FormMutation::MoveBlockToStep(MoveBlockToStep { step_id, block_id, to_step_id, index }))
            }
            8 => {
                let step_id = read_str_bin(&mut reader).map_err(|e| malformed("step_id", reader.position(), e))?;
                let block_text = read_str_bin(&mut reader).map_err(|e| malformed("block", reader.position(), e))?;
                let block = dec_block(&block_text).map_err(|e| malformed("block", reader.position(), e))?;
                Ok(FormMutation::ReplaceBlock(ReplaceBlock { step_id, block }))
            }
            9 => Ok(FormMutation::ChangeFormTitle(ChangeFormTitle { new_title: read_opt_str_bin(&mut reader).map_err(|e| malformed("new_title", reader.position(), e))? })),
            10 => {
                let text = read_str_bin(&mut reader).map_err(|e| malformed("response", reader.position(), e))?;
                let response = semio_framework_pack_json::from_json_str(&text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| malformed("response", reader.position(), e.into_message()))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(FormMutation::CommitResponse(CommitResponse { response, index }))
            }
            11 => Ok(FormMutation::DiscardResponse(DiscardResponse { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            12 => {
                let block_id = read_str_bin(&mut reader).map_err(|e| malformed("block_id", reader.position(), e))?;
                let text = read_str_bin(&mut reader).map_err(|e| malformed("change", reader.position(), e))?;
                let change = semio_framework_pack_json::from_json_str(&text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| malformed("change", reader.position(), e.into_message()))?;
                Ok(FormMutation::ChangeBlockField(ChangeBlockField { block_id, change }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
}
