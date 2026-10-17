//! 🪢️ Canonical checkpoint identity, named refusals and pure admission laws.

use super::{ArtifactFrontier, ArtifactHash, CheckpointId, DocumentScope, PublishedArtifactBlob, RebootstrapRequired};

/// 🚫️ Every way a pair body or its admission is refused, named exactly as the TypeScript twin throws it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalCheckpointPairRefusalV1 {
    Oversized,
    Truncated,
    FrameLength,
    Header,
    OversizedText,
    Text,
    Capacity,
    ZeroDigest,
    HeaderTrailingBytes,
    BaselineFrontier,
    PairLength,
    RecordCount,
    RecordTag,
    RecordOrdinal,
    RecordLength,
    RecordTrailingBytes,
    RecordOffset,
    RecordPart,
    Terminal,
    Incomplete,
    PackDigest,
    SprDigest,
    AggregateDigest,
    Scope,
    Checkpoint,
    Descriptor,
    Baseline,
    Aggregate,
}

impl CanonicalCheckpointPairRefusalV1 {
    /// 🏷️ The refusal's one spelling on every implementation.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Oversized => "canonical-checkpoint-pair.oversized",
            Self::Truncated => "canonical-checkpoint-pair.truncated",
            Self::FrameLength => "canonical-checkpoint-pair.frame-length",
            Self::Header => "canonical-checkpoint-pair.header",
            Self::OversizedText => "canonical-checkpoint-pair.oversized-text",
            Self::Text => "canonical-checkpoint-pair.text",
            Self::Capacity => "canonical-checkpoint-pair.capacity",
            Self::ZeroDigest => "canonical-checkpoint-pair.zero-digest",
            Self::HeaderTrailingBytes => "canonical-checkpoint-pair.header-trailing-bytes",
            Self::BaselineFrontier => "canonical-checkpoint-pair.baseline-frontier",
            Self::PairLength => "canonical-checkpoint-pair.pair-length",
            Self::RecordCount => "canonical-checkpoint-pair.record-count",
            Self::RecordTag => "canonical-checkpoint-pair.record-tag",
            Self::RecordOrdinal => "canonical-checkpoint-pair.record-ordinal",
            Self::RecordLength => "canonical-checkpoint-pair.record-length",
            Self::RecordTrailingBytes => "canonical-checkpoint-pair.record-trailing-bytes",
            Self::RecordOffset => "canonical-checkpoint-pair.record-offset",
            Self::RecordPart => "canonical-checkpoint-pair.record-part",
            Self::Terminal => "canonical-checkpoint-pair.terminal",
            Self::Incomplete => "canonical-checkpoint-pair.incomplete",
            Self::PackDigest => "canonical-checkpoint-pair.pack-digest",
            Self::SprDigest => "canonical-checkpoint-pair.spr-digest",
            Self::AggregateDigest => "canonical-checkpoint-pair.aggregate-digest",
            Self::Scope => "canonical-checkpoint-pair.scope",
            Self::Checkpoint => "canonical-checkpoint-pair.checkpoint",
            Self::Descriptor => "canonical-checkpoint-pair.descriptor",
            Self::Baseline => "canonical-checkpoint-pair.baseline",
            Self::Aggregate => "canonical-checkpoint-pair.aggregate",
        }
    }
}

impl std::fmt::Display for CanonicalCheckpointPairRefusalV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for CanonicalCheckpointPairRefusalV1 {}

/// 🎫️ Native-admitted checkpoint selection; semantic gates compare typed identities only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmittedCheckpointSelectionV1 {
    pub checkpoint_id: CheckpointId,
    pub descriptor_digest_v1: ArtifactHash,
    pub baseline_frontier: ArtifactFrontier,
    pub aggregate_sha256: ArtifactHash,
}

/// 🪢️ One decoded canonical pair whose pack, SPR and aggregate digests match its own selection header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalCheckpointPairV1 {
    pub scope: DocumentScope,
    pub descriptor_digest_v1: ArtifactHash,
    pub active_checkpoint_id: CheckpointId,
    pub baseline_frontier: ArtifactFrontier,
    pub pack: PublishedArtifactBlob,
    pub spr: PublishedArtifactBlob,
    pub aggregate_sha256: ArtifactHash,
    pub pack_bytes: Vec<u8>,
    pub spr_bytes: Vec<u8>,
}

impl CanonicalCheckpointPairV1 {
    /// 🎫️ Admits the pair only as exactly the checkpoint the hub authorized for this open (the execution-target
    /// lease's or the open plan's `checkpoint`): a checkpoint that moved, a changed descriptor or a foreign scope
    /// is refused, never mounted.
    pub fn admit(&self, scope: &DocumentScope, expected: &AdmittedCheckpointSelectionV1) -> Result<(), CanonicalCheckpointPairRefusalV1> {
        if self.scope != *scope {
            return Err(CanonicalCheckpointPairRefusalV1::Scope);
        }
        if self.active_checkpoint_id != expected.checkpoint_id {
            return Err(CanonicalCheckpointPairRefusalV1::Checkpoint);
        }
        if self.descriptor_digest_v1 != expected.descriptor_digest_v1 {
            return Err(CanonicalCheckpointPairRefusalV1::Descriptor);
        }
        if self.baseline_frontier != expected.baseline_frontier {
            return Err(CanonicalCheckpointPairRefusalV1::Baseline);
        }
        if self.aggregate_sha256 != expected.aggregate_sha256 {
            return Err(CanonicalCheckpointPairRefusalV1::Aggregate);
        }
        Ok(())
    }

    /// 🛟️ Admits the pair only as exactly the checkpoint a hub `RebootstrapRequired` control names (scope, checkpoint id,
    /// descriptor digest, baseline frontier) — the control carries no aggregate, which the decoder already proved against the
    /// pair's own bytes.
    pub fn admit_rebootstrap(&self, control: &RebootstrapRequired) -> Result<(), CanonicalCheckpointPairRefusalV1> {
        if self.scope != control.scope {
            return Err(CanonicalCheckpointPairRefusalV1::Scope);
        }
        if self.active_checkpoint_id != control.checkpoint_id {
            return Err(CanonicalCheckpointPairRefusalV1::Checkpoint);
        }
        if self.descriptor_digest_v1 != control.descriptor_digest_v1 {
            return Err(CanonicalCheckpointPairRefusalV1::Descriptor);
        }
        if self.baseline_frontier != control.baseline_frontier {
            return Err(CanonicalCheckpointPairRefusalV1::Baseline);
        }
        Ok(())
    }
}

