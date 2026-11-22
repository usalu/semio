//! 📬 Fixed first-archive crossings on the authenticated Shell message lane.
use super::{AppDocumentIdentity, DOCUMENT_ARCHIVE_MAXIMUM_BYTES};
use protocol::wire::command_ingress::{NaturalSourceWireAck, NaturalSourceWireLoan, NaturalSourceWirePhase, NaturalSourceWireResult, NATURAL_SOURCE_COMMAND_KIND};

const REQUEST: &[u8; 8] = b"SEMIOFA1";
const REPLY: &[u8; 8] = b"SEMIOFR1";
const REQUEST_BYTES: usize = 43;
const PAGE: usize = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FirstArchiveKey {
    pub instance: u32,
    pub operation: u64,
    pub generation: u64,
    pub sequence: u64,
}

impl FirstArchiveKey {
    pub fn completed(ack: &NaturalSourceWireAck) -> Option<Self> {
        let loan = ack.loan.as_ref()?;
        let key = Self::from_loan(loan, ack.seq)?;
        (ack.owner == u64::from(key.instance) && ack.kind == NATURAL_SOURCE_COMMAND_KIND
            && ack.extent == loan.extent && ack.phase == NaturalSourceWirePhase::Complete
            && ack.page_index.is_none() && ack.result == NaturalSourceWireResult::Accepted).then_some(key)
    }

    pub fn from_loan(loan: &NaturalSourceWireLoan, sequence: u64) -> Option<Self> {
        fn token(value: &str) -> Option<u64> {
            if value.len() != 16 || !value.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')) { return None; }
            u64::from_str_radix(value, 16).ok().filter(|value| *value != 0)
        }
        (loan.instance_id != 0 && sequence != 0 && loan.extent != 0).then_some(Self {
            instance: loan.instance_id, operation: token(&loan.operation_id)?, generation: token(&loan.generation)?, sequence,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirstArchiveAction {
    Open,
    Preview { offset: u32 },
    Acknowledge { offset: u32, length: u16 },
    Seal,
    Cancel,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FirstArchiveRequest {
    pub key: FirstArchiveKey,
    pub action: FirstArchiveAction,
}

impl FirstArchiveRequest {
    pub fn encode(self) -> [u8; REQUEST_BYTES] {
        let mut bytes = [0; REQUEST_BYTES];
        bytes[..8].copy_from_slice(REQUEST);
        bytes[8..12].copy_from_slice(&self.key.instance.to_le_bytes());
        bytes[12..20].copy_from_slice(&self.key.operation.to_le_bytes());
        bytes[20..28].copy_from_slice(&self.key.generation.to_le_bytes());
        bytes[28..36].copy_from_slice(&self.key.sequence.to_le_bytes());
        let (action, offset, length) = match self.action {
            FirstArchiveAction::Open => (1, 0u32, 0u16),
            FirstArchiveAction::Preview { offset } => (2, offset, 0),
            FirstArchiveAction::Acknowledge { offset, length } => (3, offset, length),
            FirstArchiveAction::Seal => (4, 0, 0),
            FirstArchiveAction::Cancel => (5, 0, 0),
            FirstArchiveAction::Close => (6, 0, 0),
        };
        bytes[36] = action;
        bytes[37..41].copy_from_slice(&offset.to_le_bytes());
        bytes[41..43].copy_from_slice(&length.to_le_bytes());
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != REQUEST_BYTES || &bytes[..8] != REQUEST { return None; }
        let read64 = |offset| u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap());
        let key = FirstArchiveKey {
            instance: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            operation: read64(12), generation: read64(20), sequence: read64(28),
        };
        if key.instance == 0 || key.operation == 0 || key.generation == 0 || key.sequence == 0 { return None; }
        let offset = u32::from_le_bytes(bytes[37..41].try_into().unwrap());
        let page = (offset as usize) < DOCUMENT_ARCHIVE_MAXIMUM_BYTES && (offset as usize).is_multiple_of(PAGE);
        let length = u16::from_le_bytes(bytes[41..43].try_into().unwrap());
        let action = match (bytes[36], length) {
            (1, 0) if offset == 0 => FirstArchiveAction::Open,
            (2, 0) if page => FirstArchiveAction::Preview { offset },
            (3, 1..=4096) if page => FirstArchiveAction::Acknowledge { offset, length },
            (4, 0) if offset == 0 => FirstArchiveAction::Seal,
            (5, 0) if offset == 0 => FirstArchiveAction::Cancel,
            (6, 0) if offset == 0 => FirstArchiveAction::Close,
            _ => return None,
        };
        Some(Self { key, action })
    }

    pub fn is_wire(bytes: &[u8]) -> bool { bytes.starts_with(REQUEST) }
}

pub enum FirstArchiveReply<'a> {
    Pending,
    Opened { extent: u32, document: &'a str, generation: u64, revision: [u8; 32] },
    Page(&'a [u8]),
    Acknowledged,
    Sealed([u8; 32]),
    Closed,
    Refused,
}

impl<'a> FirstArchiveReply<'a> {
    pub fn wire_length(&self) -> usize {
        REQUEST_BYTES + 1 + match self {
            Self::Opened { document, .. } => 44 + document.len(),
            Self::Page(bytes) => bytes.len(),
            Self::Sealed(_) => 32,
            _ => 0,
        }
    }

    pub fn encode_into(&self, request: FirstArchiveRequest, output: &mut Vec<u8>) -> bool {
        if !output.is_empty() || self.wire_length() > output.capacity() { return false; }
        output.extend_from_slice(REPLY);
        output.extend_from_slice(&request.encode()[8..]);
        match self {
            Self::Pending => output.push(0),
            Self::Opened { extent, document, generation, revision } => {
                output.push(1);
                output.extend_from_slice(&extent.to_le_bytes());
                output.extend_from_slice(&generation.to_le_bytes());
                output.extend_from_slice(revision);
                output.extend_from_slice(document.as_bytes());
            }
            Self::Page(bytes) => { output.push(2); output.extend_from_slice(bytes); }
            Self::Acknowledged => output.push(3),
            Self::Sealed(digest) => { output.push(4); output.extend_from_slice(digest); }
            Self::Closed => output.push(5),
            Self::Refused => output.push(6),
        }
        true
    }

    pub fn decode(request: FirstArchiveRequest, bytes: &'a [u8]) -> Option<Self> {
        if bytes.len() <= REQUEST_BYTES || &bytes[..8] != REPLY || bytes[8..REQUEST_BYTES] != request.encode()[8..] { return None; }
        let tail = &bytes[REQUEST_BYTES + 1..];
        match (bytes[REQUEST_BYTES], request.action) {
            (0, _) if tail.is_empty() => Some(Self::Pending),
            (1, FirstArchiveAction::Open) if (45..=2092).contains(&tail.len()) => {
                let extent = u32::from_le_bytes(tail[..4].try_into().unwrap());
                if extent == 0 || extent as usize > DOCUMENT_ARCHIVE_MAXIMUM_BYTES { return None; }
                let document = std::str::from_utf8(&tail[44..]).ok()?;
                if document.chars().take(513).count() > 512 { return None; }
                Some(Self::Opened { extent, document, generation: u64::from_le_bytes(tail[4..12].try_into().unwrap()), revision: tail[12..44].try_into().unwrap() })
            }
            (2, FirstArchiveAction::Preview { .. }) if (1..=PAGE).contains(&tail.len()) => Some(Self::Page(tail)),
            (3, FirstArchiveAction::Acknowledge { .. }) if tail.is_empty() => Some(Self::Acknowledged),
            (4, FirstArchiveAction::Seal) if tail.len() == 32 => Some(Self::Sealed(tail.try_into().unwrap())),
            (5, FirstArchiveAction::Cancel | FirstArchiveAction::Close) if tail.is_empty() => Some(Self::Closed),
            (6, _) if tail.is_empty() => Some(Self::Refused),
            _ => None,
        }
    }

    pub fn is_wire(bytes: &[u8]) -> bool { bytes.starts_with(REPLY) }

    pub fn opened_identity(instance: u32, document: &str, revision: &[u8; 32]) -> AppDocumentIdentity {
        AppDocumentIdentity { app_instance_id: instance, parent_document_id: Some(document.into()), base_revision: u64::from_be_bytes(revision[..8].try_into().unwrap()) }
    }
}
