//! 📬 Authenticated Shell crossing for one bounded read of an unchanged folder record.
use super::archive_ingress::{DocumentArchiveIngressSeal, DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES};

const REQUEST: &[u8; 8] = b"SEMIOL1I";
const RESPONSE: &[u8; 8] = b"SEMIOL1O";
const KEY_BYTES: usize = 36;
const HEADER: usize = KEY_BYTES + 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FolderArchiveLoadKey {
    pub instance: u32,
    pub activation: u64,
    pub lifetime: u64,
    pub sequence: u64,
}

impl FolderArchiveLoadKey {
    pub fn operation(self) -> Option<u64> {
        (self.sequence != 0 && self.sequence < 1u64 << 62).then_some(1u64 << 62 | self.sequence)
    }

    fn header(self, magic: &[u8; 8], action: u8) -> [u8; HEADER] {
        let mut result = [0; HEADER];
        result[..8].copy_from_slice(magic);
        result[8..12].copy_from_slice(&self.instance.to_le_bytes());
        result[12..20].copy_from_slice(&self.activation.to_le_bytes());
        result[20..28].copy_from_slice(&self.lifetime.to_le_bytes());
        result[28..36].copy_from_slice(&self.sequence.to_le_bytes());
        result[36] = action;
        result
    }

    fn decode(bytes: &[u8], magic: &[u8; 8]) -> Option<Self> {
        if bytes.len() < HEADER || &bytes[..8] != magic { return None; }
        let key = Self {
            instance: u32::from_le_bytes(bytes[8..12].try_into().ok()?),
            activation: u64::from_le_bytes(bytes[12..20].try_into().ok()?),
            lifetime: u64::from_le_bytes(bytes[20..28].try_into().ok()?),
            sequence: u64::from_le_bytes(bytes[28..36].try_into().ok()?),
        };
        let beginning = bytes[KEY_BYTES] == 1;
        (key.instance != 0 && key.operation().is_some()
            && if beginning { (key.activation, key.lifetime) == (0, 0) } else { key.activation != 0 && key.lifetime != 0 }).then_some(key)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FolderArchiveLoadAction<'a> {
    Begin { seal: DocumentArchiveIngressSeal, document: &'a str, schema: &'a str },
    Page { offset: u32, bytes: &'a [u8] },
    Finish,
    Publish,
    Cancel,
}

impl FolderArchiveLoadAction<'_> {
    fn tag(self) -> u8 {
        match self { Self::Begin { .. } => 1, Self::Page { .. } => 2, Self::Finish => 3, Self::Publish => 4, Self::Cancel => 5 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FolderArchiveLoadRequest<'a> {
    pub key: FolderArchiveLoadKey,
    pub action: FolderArchiveLoadAction<'a>,
}

impl<'a> FolderArchiveLoadRequest<'a> {
    pub fn is_wire(bytes: &[u8]) -> bool { bytes.starts_with(REQUEST) }

    pub fn encode(self) -> Option<Vec<u8>> {
        use FolderArchiveLoadAction::*;
        let body = match self.action {
            Begin { seal, document, schema } if (6..=u32::MAX as usize).contains(&seal.bytes) && label(document) && label(schema) => 40 + document.len() + schema.len(),
            Page { bytes, .. } if (1..=DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES).contains(&bytes.len()) => 4 + bytes.len(),
            Finish | Publish | Cancel => 0,
            _ => return None,
        };
        let mut result = Vec::new();
        result.try_reserve_exact(HEADER + body).ok()?;
        result.extend_from_slice(&self.key.header(REQUEST, self.action.tag()));
        match self.action {
            Begin { seal, document, schema } => {
                result.extend_from_slice(&(seal.bytes as u32).to_le_bytes());
                result.extend_from_slice(&seal.sha256);
                result.extend_from_slice(&(document.len() as u16).to_le_bytes());
                result.extend_from_slice(&(schema.len() as u16).to_le_bytes());
                result.extend_from_slice(document.as_bytes());
                result.extend_from_slice(schema.as_bytes());
            }
            Page { offset, bytes } => { result.extend_from_slice(&offset.to_le_bytes()); result.extend_from_slice(bytes); }
            _ => {}
        }
        Some(result)
    }

    pub fn decode(bytes: &'a [u8]) -> Option<Self> {
        use FolderArchiveLoadAction::*;
        let key = FolderArchiveLoadKey::decode(bytes, REQUEST)?;
        let tail = &bytes[HEADER..];
        let read_u32 = |offset| tail.get(offset..offset + 4).and_then(|bytes| bytes.try_into().ok()).map(u32::from_le_bytes);
        let action = match bytes[KEY_BYTES] {
            1 if tail.len() >= 40 => {
                let extent = read_u32(0)? as usize;
                let document_bytes = u16::from_le_bytes(tail[36..38].try_into().ok()?) as usize;
                let schema_bytes = u16::from_le_bytes(tail[38..40].try_into().ok()?) as usize;
                let document = std::str::from_utf8(tail.get(40..40 + document_bytes)?).ok()?;
                let schema = std::str::from_utf8(&tail[40 + document_bytes..]).ok()?;
                if extent < 6 || schema.len() != schema_bytes || !label(document) || !label(schema) { return None; }
                Begin { seal: DocumentArchiveIngressSeal { bytes: extent, sha256: tail[4..36].try_into().ok()? }, document, schema }
            }
            2 if (5..=4 + DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES).contains(&tail.len()) => Page { offset: read_u32(0)?, bytes: &tail[4..] },
            3 if tail.is_empty() => Finish,
            4 if tail.is_empty() => Publish,
            5 if tail.is_empty() => Cancel,
            _ => return None,
        };
        Some(Self { key, action })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FolderArchiveLoadReply {
    Pending,
    Admitted { activation: u64, lifetime: u64 },
    Received { through: u32 },
    Verified,
    Submitted,
    Closed,
    Refused,
}

impl FolderArchiveLoadReply {
    pub const MAXIMUM_BYTES: usize = HEADER + 17;
    pub fn is_wire(bytes: &[u8]) -> bool { bytes.starts_with(RESPONSE) }

    pub fn encode_into(self, request: FolderArchiveLoadRequest<'_>, output: &mut Vec<u8>) -> bool {
        if !output.is_empty() || output.capacity() < Self::MAXIMUM_BYTES { return false; }
        output.extend_from_slice(&request.key.header(RESPONSE, request.action.tag()));
        match self {
            Self::Pending => output.push(0),
            Self::Admitted { activation, lifetime } => {
                output.push(1);
                output.extend_from_slice(&activation.to_le_bytes());
                output.extend_from_slice(&lifetime.to_le_bytes());
            }
            Self::Received { through } => { output.push(2); output.extend_from_slice(&through.to_le_bytes()); }
            Self::Verified => output.push(3),
            Self::Submitted => output.push(4),
            Self::Closed => output.push(5),
            Self::Refused => output.push(6),
        }
        true
    }

    pub fn decode(request: FolderArchiveLoadRequest<'_>, bytes: &[u8]) -> Option<Self> {
        if FolderArchiveLoadKey::decode(bytes, RESPONSE)? != request.key || bytes[KEY_BYTES] != request.action.tag() { return None; }
        let (&tag, tail) = bytes[HEADER..].split_first()?;
        use FolderArchiveLoadAction::*;
        match (tag, request.action) {
            (0, _) if tail.is_empty() => Some(Self::Pending),
            (1, Begin { .. }) if tail.len() == 16 => {
                let activation = u64::from_le_bytes(tail[..8].try_into().ok()?);
                let lifetime = u64::from_le_bytes(tail[8..].try_into().ok()?);
                (activation != 0 && lifetime != 0).then_some(Self::Admitted { activation, lifetime })
            },
            (2, Page { offset, bytes }) if tail.len() == 4 => {
                let through = u32::from_le_bytes(tail.try_into().ok()?);
                (through > offset && through <= offset.checked_add(bytes.len() as u32)?).then_some(Self::Received { through })
            }
            (3, Finish) if tail.is_empty() => Some(Self::Verified),
            (4, Publish) if tail.is_empty() => Some(Self::Submitted),
            (5, Cancel) if tail.is_empty() => Some(Self::Closed),
            (6, _) if tail.is_empty() => Some(Self::Refused),
            _ => None,
        }
    }
}

fn label(value: &str) -> bool {
    !value.is_empty() && value.len() <= 2048 && value.chars().count() <= 512 && !value.chars().any(char::is_control)
}
