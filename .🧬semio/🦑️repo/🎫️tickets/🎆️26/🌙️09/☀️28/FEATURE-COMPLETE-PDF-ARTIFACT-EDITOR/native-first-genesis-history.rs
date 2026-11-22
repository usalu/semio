//! 📜 Encodes the canonical first document history from bounded borrowed fields.
use super::{HEADER_SIZE, PackSink, ProtocolError, REC_COMPOSITION, SprWriter, WriteOptions};
use semio_os_pack::PackRefusal;
use semio_framework_value::ValueRefusalKind;
use protocol::format::COMMIT_PAYLOAD_LEN;
use std::{future::Future, task::{Context, Poll, Waker}};

const MAXIMUM_BYTES: usize = 4096;

/// 📏 Quotes the exact on-disk extent before a caller reserves its SPR recipient.
pub struct FirstGenesisHistory<'a> {
    entries: [&'a str; 5],
    used: usize,
    doc: [Id; 2],
    dialect: Option<[Id; 3]>,
    doc_entries: usize,
    extent: usize,
}

#[derive(Clone, Copy)]
struct Id { index: u8, uuid: Option<[u8; 16]> }

impl Id {
    fn len(self) -> usize { if self.uuid.is_some() { 18 } else { 2 } }
}

impl<'a> FirstGenesisHistory<'a> {
    pub fn new(document: &'a str, schema: &'a str, dialect: Option<[&'a str; 3]>) -> Result<Self, ProtocolError> {
        let fields = [document, schema];
        if document.is_empty() || schema.is_empty()
            || fields.into_iter().chain(dialect.into_iter().flatten()).any(|field| field.len() > MAXIMUM_BYTES) {
            return Err(limit());
        }
        let mut entries = [""; 5];
        let mut used = 0;
        let doc = fields.map(|field| intern(field, &mut entries, &mut used));
        let doc_entries = used;
        let dialect = dialect.map(|fields| fields.map(|field| intern(field, &mut entries, &mut used)));
        let mut plan = Self { entries, used, doc, dialect, doc_entries, extent: 0 };
        let mut extent = HEADER_SIZE + frame_len(COMMIT_PAYLOAD_LEN)
            + frame_len(plan.dictionary_payload(0, doc_entries)) + frame_len(1 + ids_len(&doc));
        if let Some(fields) = dialect {
            if doc_entries != used { extent += frame_len(plan.dictionary_payload(doc_entries, used)); }
            extent += frame_len(2 + ids_len(&fields));
        }
        if extent > MAXIMUM_BYTES { return Err(limit()); }
        plan.extent = extent;
        Ok(plan)
    }

    pub fn extent(&self) -> usize { self.extent }

    /// 🖋 Writes into caller-funded exact storage using the normal SPR framing and commit.
    pub fn encode(&self, recipient: &mut [u8]) -> Result<(), ProtocolError> {
        if recipient.len() != self.extent { return Err(limit()); }
        let work = std::pin::pin!(self.write(recipient));
        let mut context = Context::from_waker(Waker::noop());
        match work.poll(&mut context) {
            Poll::Ready(result) => result,
            Poll::Pending => Err(ProtocolError::LimitExceeded("memory-only first SPR writer suspended")),
        }
    }

    async fn write(&self, recipient: &mut [u8]) -> Result<(), ProtocolError> {
        let sink = ExactSink { recipient, offset: 0 };
        let options = WriteOptions { required_flags: protocol::REQUIRED_HASH_CHAIN, optional_flags: protocol::OPTIONAL_CANONICAL };
        let mut writer = SprWriter::begin(sink, &options).await?;
        self.write_dictionary(&mut writer, 0, self.doc_entries).await?;
        self.write_ids(&mut writer, protocol::REC_DOC, true, &[1], &self.doc).await?;
        if let Some(fields) = self.dialect {
            self.write_dictionary(&mut writer, self.doc_entries, self.used).await?;
            self.write_ids(&mut writer, REC_COMPOSITION, false, &[1, 2], &fields).await?;
        }
        writer.commit().await?;
        if writer.into_sink().await.offset != self.extent { return Err(limit()); }
        Ok(())
    }

    fn dictionary_payload(&self, start: usize, end: usize) -> usize {
        3 + self.entries[start..end].iter().map(|text| varint_len(text.len()) + text.len()).sum::<usize>()
    }

    async fn write_dictionary(&self, writer: &mut SprWriter<ExactSink<'_>>, start: usize, end: usize) -> Result<(), ProtocolError> {
        if start == end { return Ok(()); }
        let mut record = writer.begin_identity_record(protocol::REC_STR_DICT, true, self.dictionary_payload(start, end)).await?;
        record.write_fragment(&[1, start as u8, (end - start) as u8]).await?;
        for text in &self.entries[start..end] {
            let mut length = [0; 2];
            let count = varint_len(text.len());
            length[0] = (text.len() as u8 & 127) | if count == 2 { 128 } else { 0 };
            length[1] = (text.len() >> 7) as u8;
            record.write_fragment(&length[..count]).await?;
            record.write_fragment(text.as_bytes()).await?;
        }
        record.finish().await?;
        Ok(())
    }

    async fn write_ids(&self, writer: &mut SprWriter<ExactSink<'_>>, kind: u8, critical: bool, prefix: &[u8], ids: &[Id]) -> Result<(), ProtocolError> {
        let mut record = writer.begin_identity_record(kind, critical, prefix.len() + ids_len(ids)).await?;
        record.write_fragment(prefix).await?;
        for id in ids {
            record.write_fragment(&[if id.uuid.is_some() { 2 } else { 1 }, id.index]).await?;
            if let Some(uuid) = &id.uuid { record.write_fragment(uuid).await?; }
        }
        record.finish().await?;
        Ok(())
    }
}

fn intern<'a>(field: &'a str, entries: &mut [&'a str; 5], used: &mut usize) -> Id {
    let (token, uuid) = protocol::scalar::split_prefix_uuid(field)
        .map_or((field, None), |(prefix, uuid)| (prefix, Some(uuid)));
    let index = entries[..*used].iter().position(|entry| *entry == token).unwrap_or_else(|| {
        let index = *used;
        entries[index] = token;
        *used += 1;
        index
    });
    Id { index: index as u8, uuid }
}

fn ids_len(ids: &[Id]) -> usize { ids.iter().map(|id| id.len()).sum() }
fn varint_len(bytes: usize) -> usize { if bytes < 128 { 1 } else { 2 } }
fn frame_len(payload: usize) -> usize { varint_len(payload + 2) + payload + 10 }
fn limit() -> ProtocolError { ProtocolError::LimitExceeded("first SPR exceeds its bounded metadata recipient") }

struct ExactSink<'a> { recipient: &'a mut [u8], offset: usize }

impl PackSink for ExactSink<'_> {
    type Error = PackRefusal;

    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        let end = self.offset + bytes.len();
        let output = self.recipient.get_mut(self.offset..end).ok_or(PackRefusal::LimitExceeded {
            kind: ValueRefusalKind::OwnershipLimit, limit: "quoted first SPR extent",
        })?;
        output.copy_from_slice(bytes);
        self.offset = end;
        Ok(())
    }

    async fn position(&self) -> u64 { self.offset as u64 }
}
