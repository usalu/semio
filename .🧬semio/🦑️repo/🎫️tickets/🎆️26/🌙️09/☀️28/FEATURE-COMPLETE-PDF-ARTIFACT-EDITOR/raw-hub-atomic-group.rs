//! 🧾️ Defines the Hub atomic transport identity and its exact persisted group descriptor.

use semio_framework_hash::Sha256;
use semio_framework_value::paged::ByteReadSource;
use super::atomic_group::{ATOMIC_BACKBONE_HEADER_BYTES, ATOMIC_BACKBONE_PART_BYTES, AtomicBackboneKey, AtomicBackbonePart, is_atomic_backbone_tool};
use super::atomic_group::{ATOMIC_BACKBONE_MAXIMUM_BYTES, ATOMIC_EDIT_PREFIX_BYTES, AtomicBackboneSender};
use super::{DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES, MutationEnvelope, VerifiedAtomicEdit, decode_envelopes, encode_envelope, encode_envelopes};

const CLIENT_PREFIX: [u8; 2] = [0, 8];
const SERVER_PREFIX: [u8; 2] = [0, 16];
const WAL_PREFIX: [u8; 4] = [b'S', b'A', b'E', 1];
const CHECKPOINT_PREFIX: [u8; 4] = [0x80, 0, b'G', 2];
const CHECKPOINT_MAXIMUM_BYTES: usize = 64 * 1024 * 1024;
const CHECKPOINT_MAXIMUM_COMMITS: usize = 16_384;
pub const MAX_ATOMIC_WAL_DESCRIPTOR_BYTES: usize = 8 + DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES;

/// 📥️ Selects only the distinct command-lane packet before its ordinary codec runs.
pub fn client_atomic_part(bytes: &[u8]) -> Result<Option<(u64, &[u8])>, &'static str> {
    let Some(body) = bytes.strip_prefix(&CLIENT_PREFIX) else { return Ok(None); };
    if !(8 + ATOMIC_BACKBONE_HEADER_BYTES + 1..=8 + ATOMIC_BACKBONE_HEADER_BYTES + ATOMIC_BACKBONE_PART_BYTES).contains(&body.len()) {
        return Err("atomic document packet length");
    }
    let (id, physical) = body.split_at(8);
    Ok(Some((u64::from_le_bytes(id.try_into().unwrap()), physical)))
}

/// 📚️ Retains one in-order group under its owning authenticated document socket.
#[derive(Default)]
pub struct AtomicSocketEdit {
    identity: Option<(u64, AtomicBackboneKey)>,
    scope: Option<(String, String)>,
    pages: Vec<Box<[u8]>>,
    digest: Sha256,
}

impl AtomicSocketEdit {
    pub fn push(&mut self, id: u64, physical: &[u8], document: &str, actor: &str) -> Result<Option<VerifiedAtomicEdit>, &'static str> {
        let (part, bytes) = AtomicBackbonePart::parse(physical).map_err(|_| "atomic document packet part")?;
        if part.ordinal as usize != self.pages.len() || self.identity.is_some_and(|prior| prior != (id, part.key)) ||
            self.scope.as_ref().is_some_and(|(old_document, old_actor)| old_document != document || old_actor != actor) {
            return Err("atomic document packet order or identity");
        }
        if self.scope.is_none() {
            if document.is_empty() || actor.is_empty() || document.len() > DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES ||
                actor.len() > DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES {
                return Err("atomic document packet scope extent");
            }
            self.scope = Some((document.to_owned(), actor.to_owned()));
        }
        self.identity = Some((id, part.key));
        for block in bytes.chunks(4096) { self.digest.update(block); }
        self.pages.push(bytes.into());
        if self.pages.len() < part.key.count() { return Ok(None); }
        let complete = std::mem::take(self);
        if complete.digest.clone().finalize() != part.key.digest { return Err("atomic document packet digest"); }
        super::decode_atomic_backbone(&complete, document, Some(actor)).map(Some).map_err(|_| "atomic document packet author or payload")
    }
}

impl ByteReadSource for AtomicSocketEdit {
    fn byte_len(&self) -> usize { self.identity.map_or(0, |(_, key)| key.extent as usize) }
    fn byte_at(&self, index: usize) -> Option<u8> {
        self.pages.get(index / ATOMIC_BACKBONE_PART_BYTES)?.get(index % ATOMIC_BACKBONE_PART_BYTES).copied()
    }
    fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize {
        let mut copied = 0;
        while copied < output.len() {
            let Some(index) = offset.checked_add(copied) else { break; };
            let Some(page) = self.pages.get(index / ATOMIC_BACKBONE_PART_BYTES) else { break; };
            let start = index % ATOMIC_BACKBONE_PART_BYTES;
            let count = (output.len() - copied).min(page.len().saturating_sub(start));
            if count == 0 { break; }
            output[copied..copied + count].copy_from_slice(&page[start..start + count]);
            copied += count;
        }
        copied
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtomicWalEdit {
    pub edit_id: String,
    pub rows: usize,
}

impl AtomicWalEdit {
    /// ✍️ Writes the existing Store identity beside all causal rows in their single WAL transaction.
    pub fn encode(&self) -> Result<Vec<u8>, &'static str> {
        if self.edit_id.is_empty() || self.edit_id.len() > DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES ||
            self.rows == 0 || self.rows > DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES {
            return Err("atomic WAL group descriptor extent");
        }
        let mut bytes = Vec::with_capacity(8 + self.edit_id.len());
        bytes.extend_from_slice(&WAL_PREFIX);
        bytes.extend_from_slice(&(self.rows as u16).to_le_bytes());
        bytes.extend_from_slice(&(self.edit_id.len() as u16).to_le_bytes());
        bytes.extend_from_slice(self.edit_id.as_bytes());
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Option<Self>, &'static str> {
        let Some(body) = bytes.strip_prefix(&WAL_PREFIX) else { return Ok(None); };
        let rows = u16::from_le_bytes(body.get(..2).ok_or("atomic WAL row count")?.try_into().unwrap()) as usize;
        let length = u16::from_le_bytes(body.get(2..4).ok_or("atomic WAL edit identity width")?.try_into().unwrap()) as usize;
        if length == 0 || length > DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES || body.len() != 4 + length ||
            rows == 0 || rows > DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES {
            return Err("atomic WAL group descriptor extent");
        }
        let edit_id = std::str::from_utf8(&body[4..]).map_err(|_| "atomic WAL edit identity UTF-8")?.to_owned();
        Ok(Some(Self { edit_id, rows }))
    }

    pub fn admits<'a>(&self, rows: impl IntoIterator<Item = &'a MutationEnvelope>) -> bool {
        let mut rows = rows.into_iter();
        let Some(first) = rows.next() else { return false; };
        let Some(transaction) = first.transaction.as_ref() else { return false; };
        let mut count = 1;
        !transaction.id.is_empty() && is_atomic_backbone_tool(&transaction.tool) && rows.all(|row| {
            count += 1;
            row.document_id == first.document_id && row.actor == first.actor && row.transaction.as_ref() == Some(transaction)
        }) && self.rows == count
    }

    fn prepare(&self, rows: &[MutationEnvelope]) -> Result<Vec<u8>, &'static str> {
        if !self.admits(rows) { return Err("atomic relay author or row count"); }
        let mut prefix = [0; ATOMIC_EDIT_PREFIX_BYTES];
        let width = AtomicBackboneSender::edit_prefix(&self.edit_id, &mut prefix).map_err(|_| "atomic relay edit identity")?;
        let mut prepared = Vec::from(&prefix[..width]);
        let mut count = rows.len();
        while count >= 128 {
            prepared.push((count as u8 & 127) | 128);
            count >>= 7;
        }
        prepared.push(count as u8);
        for row in rows {
            encode_envelope(row, &mut prepared);
            if prepared.len() > ATOMIC_BACKBONE_MAXIMUM_BYTES { return Err("atomic relay extent"); }
        }
        Ok(prepared)
    }

    /// 📡️ Reconstitutes canonical physical frames from the exact durable grouped rows.
    pub fn relay(&self, rows: &[MutationEnvelope], frontier: &[u8]) -> Result<Vec<Vec<u8>>, &'static str> {
        if frontier.len() > DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES * 2 + 128 {
            return Err("atomic relay frontier");
        }
        let prepared = self.prepare(rows)?;
        let mut digest = Sha256::new();
        for bytes in prepared.chunks(4096) { digest.update(bytes); }
        let key = AtomicBackboneKey { digest: digest.finalize(), extent: prepared.len() as u32 };
        prepared.chunks(ATOMIC_BACKBONE_PART_BYTES).enumerate().map(|(ordinal, body)| {
            let meta = if ordinal == 0 { frontier } else { &[] };
            let header = (AtomicBackbonePart { key, ordinal: ordinal as u16 }).encode_header().map_err(|_| "atomic relay frame extent")?;
            let mut message = Vec::with_capacity(4 + meta.len() + header.len() + body.len());
            message.extend_from_slice(&SERVER_PREFIX);
            message.extend_from_slice(&(meta.len() as u16).to_le_bytes());
            message.extend_from_slice(meta);
            message.extend_from_slice(&header);
            message.extend_from_slice(body);
            Ok(message)
        }).collect()
    }
}

/// 📑️ Preserves a durable transaction boundary across the trusted Check In codec.
pub enum CheckpointReplayGroup {
    Ordinary(Vec<MutationEnvelope>),
    Original(VerifiedAtomicEdit),
}

/// 📦️ Carries grouped WAL transactions through the existing opaque Check In operation input.
pub fn encode_checkpoint_replay<'a>(
    commits: impl IntoIterator<Item = (Option<&'a AtomicWalEdit>, &'a [MutationEnvelope])>,
) -> Result<Vec<u8>, &'static str> {
    let mut result = Vec::from(CHECKPOINT_PREFIX);
    for (index, (group, rows)) in commits.into_iter().enumerate() {
        if index >= CHECKPOINT_MAXIMUM_COMMITS || rows.is_empty() { return Err("checkpoint grouped replay row count"); }
        let (tag, bytes) = if let Some(group) = group {
            (1, group.prepare(rows)?)
        } else {
            if rows.iter().any(|row| row.transaction.as_ref().is_some_and(|tx| is_atomic_backbone_tool(&tx.tool))) {
                return Err("checkpoint original edit lost its grouping");
            }
            (0, encode_envelopes(rows))
        };
        if bytes.len() > ATOMIC_BACKBONE_MAXIMUM_BYTES ||
           result.len().saturating_add(bytes.len()).saturating_add(5) > CHECKPOINT_MAXIMUM_BYTES {
            return Err("checkpoint grouped replay extent");
        }
        result.push(tag);
        result.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        result.extend(bytes);
    }
    if result.len() == CHECKPOINT_PREFIX.len() { return Err("checkpoint grouped replay is empty"); }
    Ok(result)
}

/// 🔐️ Opens grouped checkpoint operations only for the exact document the trusted codec loaded.
pub fn decode_checkpoint_replay(bytes: &[u8], document: &str) -> Result<Option<Vec<CheckpointReplayGroup>>, &'static str> {
    let Some(mut tail) = bytes.strip_prefix(&CHECKPOINT_PREFIX) else { return Ok(None); };
    if tail.is_empty() || bytes.len() > CHECKPOINT_MAXIMUM_BYTES || document.is_empty() {
        return Err("checkpoint grouped replay extent or document");
    }
    let mut commits = Vec::new();
    while !tail.is_empty() {
        if commits.len() >= CHECKPOINT_MAXIMUM_COMMITS { return Err("checkpoint grouped replay transaction count"); }
        let header = tail.get(..5).ok_or("checkpoint replay is missing a record header")?;
        let extent = u32::from_le_bytes(header[1..].try_into().unwrap()) as usize;
        if extent == 0 || extent > ATOMIC_BACKBONE_MAXIMUM_BYTES { return Err("checkpoint replay record extent"); }
        let body = tail.get(5..5 + extent).ok_or("checkpoint replay record is truncated")?;
        let commit = match header[0] {
            0 => {
                let rows = decode_envelopes(body).map_err(|_| "checkpoint replay malformed ordinary rows")?;
                if rows.is_empty() || rows.iter().any(|row| row.document_id.0 != document ||
                    row.transaction.as_ref().is_some_and(|tx| is_atomic_backbone_tool(&tx.tool))) {
                    return Err("checkpoint replay ordinary row binding");
                }
                CheckpointReplayGroup::Ordinary(rows)
            }
            1 => CheckpointReplayGroup::Original(super::decode_atomic_backbone(body, document, None)
                .map_err(|_| "checkpoint replay malformed original edit")?),
            _ => return Err("checkpoint replay record kind"),
        };
        commits.push(commit);
        tail = &tail[5 + extent..];
    }
    Ok(Some(commits))
}
