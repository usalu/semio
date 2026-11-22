//! 🧾 Exact first-genesis archive offer, page receipt and durable digest scope.
use super::{AppDocumentIdentity, DOCUMENT_ARCHIVE_MAXIMUM_BYTES};
use protocol::wire::command_ingress::{
    ActorInstanceOpenPurpose, NATURAL_SOURCE_COMMAND_KIND, NaturalSourceWireAck,
    NaturalSourceWireLoan, NaturalSourceWirePhase, NaturalSourceWireResult,
};

pub const FIRST_GENESIS_ARCHIVE_PAGE_BYTES: usize = 4096;

/// 🎟️ Authorizes only the initial archive of the unpublished kind-241 result.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FirstGenesisArchiveScope {
    loan: NaturalSourceWireLoan,
    #[serde(with = "u64_decimal")]
    seq: u64,
    document: AppDocumentIdentity,
    #[serde(with = "u64_decimal")]
    store_generation: u64,
    store_revision: [u8; 32],
}

impl FirstGenesisArchiveScope {
    /// 🪪️ Binds the accepted natural input to the exact mounted Store frontier.
    pub fn from_completed_source(
        purpose: &ActorInstanceOpenPurpose, expected_instance: u32, expected_seq: u64,
        expected_loan: &NaturalSourceWireLoan, ack: &NaturalSourceWireAck, document: AppDocumentIdentity,
        store_generation: u64, store_revision: [u8; 32],
    ) -> Option<Self> {
        let ActorInstanceOpenPurpose::NaturalSource { declaration, extent } = purpose else { return None; };
        let loan = ack.loan.as_ref()?;
        let id = document.parent_document_id.as_deref()?;
        if expected_instance == 0 || expected_seq == 0 || *extent == 0 || !declaration.binary || loan != expected_loan
            || !canonical_hex(&loan.operation_id) || !canonical_hex(&loan.generation)
            || ack.owner != u64::from(expected_instance) || ack.seq != expected_seq
            || ack.kind != NATURAL_SOURCE_COMMAND_KIND || ack.extent != *extent
            || ack.phase != NaturalSourceWirePhase::Complete || ack.page_index.is_some()
            || ack.result != NaturalSourceWireResult::Accepted || loan.instance_id != expected_instance
            || loan.extent != *extent || loan.declaration != *declaration
            || document.app_instance_id != expected_instance || !(1..=512).contains(&id.chars().count())
            || document.base_revision != u64::from_be_bytes(store_revision[..8].try_into().unwrap())
        { return None; }
        Some(Self { loan: loan.clone(), seq: expected_seq, document, store_generation, store_revision })
    }

    pub fn document(&self) -> &AppDocumentIdentity { &self.document }
    pub fn frontier(&self) -> (u64, [u8; 32]) { (self.store_generation, self.store_revision) }
    pub fn matches_key(&self, key: super::FirstArchiveKey) -> bool {
        super::FirstArchiveKey::from_loan(&self.loan, self.seq) == Some(key)
    }
    pub fn matches_current(&self, document: &AppDocumentIdentity, generation: u64, revision: &[u8; 32]) -> bool {
        self.document == *document && self.store_generation == generation && self.store_revision == *revision
    }
}

/// 📐 Binds the encoded archive extent to its validated genesis frontier.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FirstGenesisArchiveOffer {
    pub scope: FirstGenesisArchiveScope,
    pub extent: u32,
}

impl FirstGenesisArchiveOffer {
    pub fn new(scope: FirstGenesisArchiveScope, extent: usize) -> Option<Self> {
        (extent != 0 && extent <= DOCUMENT_ARCHIVE_MAXIMUM_BYTES)
            .then_some(Self { scope, extent: extent as u32 })
    }

    pub fn page_length(&self, offset: u32) -> Option<usize> {
        let offset = offset as usize;
        let extent = self.extent as usize;
        (extent <= DOCUMENT_ARCHIVE_MAXIMUM_BYTES && offset < extent && offset.is_multiple_of(FIRST_GENESIS_ARCHIVE_PAGE_BYTES))
            .then(|| (extent - offset).min(FIRST_GENESIS_ARCHIVE_PAGE_BYTES))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FirstGenesisArchivePageReceipt {
    pub offer: FirstGenesisArchiveOffer,
    pub offset: u32,
    pub length: u32,
}

impl FirstGenesisArchivePageReceipt {
    pub fn matches(&self, offer: &FirstGenesisArchiveOffer, offset: u32, length: usize) -> bool {
        self.offer == *offer && self.offset == offset && self.length as usize == length
            && offer.page_length(offset) == Some(length)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FirstGenesisArchiveSeal {
    pub offer: FirstGenesisArchiveOffer,
    pub sha256: [u8; 32],
}

fn canonical_hex(value: &str) -> bool {
    value.len() == 16 && value != "0000000000000000"
        && value.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

mod u64_decimal {
    pub fn serialize<S: serde::Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        let integer = value.parse::<u64>().map_err(serde::de::Error::custom)?;
        if integer.to_string() != value { return Err(serde::de::Error::custom("noncanonical archive identity")); }
        Ok(integer)
    }
}
