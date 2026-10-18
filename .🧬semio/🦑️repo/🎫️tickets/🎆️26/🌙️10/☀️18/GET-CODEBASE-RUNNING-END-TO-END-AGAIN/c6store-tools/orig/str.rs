//! 🗃️ Local-first, non-blocking, client-side, in-memory document store — hot-swappable
//! backbones (`temp://`/`file://`/`folder://`/`remote://`) layered on `vcs`'s version-graph
//! algebra. Store application capabilities and erased registration live in OS IO.
//! Neutral snapshot representation contracts belong to framework IO, packaged by Pack;
//! concrete Store snapshot and mutation codecs belong to their physical IO owners.
//!
//! Mutation of document state is sealed: public writes go through [`ArtifactStore::dispatch`]
//! (`Apply`/`IngestRemote`/`PruneDrafts`/…) or [`ArtifactStore::reset`]. Envelope fields stay
//! `pub` for serde/plugins; treat them as read-mostly and prefer [`ArtifactEnvelopeView`].

// 🚫️async: R7 — `async fn` in a public trait warns because auto trait bounds (e.g. `Send`) cannot
// be named on the method. Answered structurally per R3: every former `dyn` seam in this crate is a
// concrete enum, so `Send` is derived at each spawn site from the concrete type, never from a bound
// on the trait method's returned future. This crate is guest-reachable, so its futures are
// deliberately `?Send` — do not "fix" this warning by adding `-> impl Future<..> + Send` (contradicts
// R3) or by making a trait method sync (contradicts O1/R1).
#![allow(async_fn_in_trait)]

use semio_framework_composition_admission::{ChildRestoreProjection,ChildRestoreProjectionError,CHILD_RESTORE_MAXIMUM_FIELD_BYTES};
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use genesis_pack_admission::ArtifactGenesisPackCapture;
use semio_framework_os_store_owned_schema_decode::genesis_pack_admission;
use semio_framework_os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisProducer};

mod retirement_integration {
    pub use semio_framework_os_store_retirement::*;
    pub use semio_framework_os_store_space_schema::lifted_retirement_integration::*;
}
pub use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
pub use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use protocol::codec::HistoryReadLimits;

#[path = "📦️codec/🎮️operation/🦀️.rs"]
mod retained_codec_catalog;
pub use retained_codec_catalog::{ArtifactCodecCatalogStep, RetainedArtifactCodecCatalog};

pub use semio_framework_os_store_snapshot_clone as snapshot_clone_preparation;

use semio_framework_os_store_replay::replay_preparation;
pub use replay_preparation::{ArtifactReplayPreparation, ArtifactReplayPreparationContext, ArtifactReplayPreparationFactory, ArtifactReplayPreparationMode, ArtifactReplayPreparationProgress, ArtifactReplayPreparationRetirement, ArtifactReplayPreparationStep, ArtifactReplayPreparationTask, ArtifactReplayPrepared, admit_replay_preparation_step};
pub use semio_framework_os_spr::command::MessageCopyCursor;

use semio_framework_os_store_ledger::edit_message_clamp;

pub use semio_framework_os_spr::command::{ArtifactMessageComposeCursor, ArtifactMessageComposeRefusal, ArtifactMessageFragment, ArtifactMessageSource};

use semio_framework_os_store_one_item_batch::ephemeral_transfer;
pub use ephemeral_transfer::{ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES, ArtifactEphemeralTransferPreparationFactory};

use semio_framework_os_store_one_item_batch::ephemeral_preparation;
pub use ephemeral_preparation::{ArtifactEphemeralPreparationTask, ArtifactEphemeralPreparationTaskStep, ArtifactEphemeralTaskPreparationFactory};

use semio_framework_os_store_engine::initial_child_identity;

use semio_framework_os_store_composition::owned_document_closure;
pub use semio_framework_os_store_composition::owned_document_closure::{OWNED_DOCUMENT_MAXIMUM_MEMBERS, OwnedDocumentClosure, OwnedDocumentClosureDiagnostic, OwnedDocumentClosureProgress, OwnedDocumentClosureSource, OwnedDocumentClosureStep};

pub mod member_open {
    pub use semio_framework_os_store_member_open::*;
    pub use semio_framework_os_store_member_open_store::lifted_member_open::*;
    pub use semio_framework_os_store_member_open_store::admit_member_input_buffer;
    pub use semio_framework_os_store_engine_owners::lifted_operation::MemberSnapshotOpenProgress;
    pub use semio_framework_os_store_member_open_store::InitialMemberStoreOpen;
    pub use semio_framework_os_store_engine_owners::lifted_operation::MemberSnapshotOpenOperation;
    pub use semio_framework_os_store_engine_owners::lifted_operation::MemberSnapshotOpenStep;
    pub use semio_framework_os_store_member_open_store::ProducedMemberSnapshotOpen;
    pub use semio_framework_os_store_space_member::lifted_operation::UnsupportedMemberFactoryOpen;
    pub use semio_framework_os_store_space_member::lifted_operation::UnsupportedMemberSnapshotOpen;
}
pub use semio_framework_os_store_member_open::genesis::{MemberGenesisEnvelopeEncoder, MemberGenesisEnvelopeProgress, MemberGenesisEnvelopeSource};
pub use semio_framework_os_store_member_open_store::{InitialMemberStoreOpen, admit_member_input_buffer, ProducedMemberSnapshotOpen};
pub use semio_framework_os_store_member_open::{MEMBER_OPEN_IDENTITY_BYTES, MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenFrame, MemberOpenInputStep, MemberOpenOperation, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, MemberOpenStep};
pub use semio_framework_os_store_space_member::factory::MemberOpenDeclaration;
pub use semio_framework_os_store_member_open::source_admission::MemberOpenSourceAdmission;
pub use semio_framework_os_store_engine_owners::lifted_operation::{MemberSnapshotOpenProgress, MemberSnapshotOpenOperation, MemberSnapshotOpenStep};
pub use semio_framework_os_store_member_open::decoder::{MemberSnapshotDecoder, ArtifactPackSnapshotDecoder};
pub use semio_framework_os_store_space_member::lifted_operation::{UnsupportedMemberFactoryOpen, UnsupportedMemberSnapshotOpen};

use semio_framework_os_store_document::persisted_document_hydration;
pub use semio_framework_os_store_document::persisted_document_hydration::{PersistedDocumentHydrationOutput, PersistedDocumentHydrationProgress, PersistedDocumentHydrationStep, PersistedDocumentStoreHydrationStep, PersistedDocumentHydrationTarget, RetainedPersistedDocumentHydration};

use semio_framework_os_store_document::mounted_pack_session;

use semio_framework_os_store_config::retained_config_hydration;
pub use semio_framework_os_store_config::retained_config_hydration::{ConfigStoreHydrationDiagnostic, ConfigStoreHydrationProgress, ConfigStoreHydrationStep, RetainedConfigStoreHydration};

pub mod durable_group {
    pub use semio_framework_os_store_durable_group::*;
    pub use semio_framework_os_store_engine_stage::lifted_durable_group::*;
    pub use semio_framework_os_store_durable_group_wire::lifted_durable_group::*;
}

use semio_framework_os_store_engine::member_group;

use semio_framework_os_store_one_item_batch as one_item_owners;
pub use one_item_owners::OneItemOwners;

use semio_framework_os_store_one_item_batch::paged_one_item;
pub use paged_one_item::{mutation_apply_preparation_factory, paged_one_item_factory_birth_bytes, MutationApplyEdit, PagedOneItemEdit, PagedOneItemEditStep, PagedOneItemPreparationFactory};

// The `crate::os_dsl::DslArtifact`/`crate::os_dsl::DslOps` derive macros emit `::pack::snapshot::ArtifactDsl`/`::crate::os_store::OpText`
// paths (see `dsl/derive/rs/lib.rs`), which only resolve for crates that depend on `store` as an
// external crate — every real consumer, INCLUDING this crate's own `.ops` header grammar
// (`OpsHeaderLine` in `🔖️TextFormat` below, derived on the engine directly) as well as its in-crate
// test fixtures (a crate is never its own dependency otherwise). `// extern crate self removed after merge` is
// the same fix `vcs`/`dsl` use for their own in-crate derive usage: it makes `::store` resolve to
// this crate even when the derive is exercised in-crate.
// extern crate self removed after merge

use semio_framework_dsl_record_derive::DslEnum;
use semio_framework_dsl_record_derive::DslRecord;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId, UndoPolicy};
use semio_framework_os_spr::{DiffRegions, Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText, TouchedPaths};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
//#region 🧬️OpaqueSnapshotRead
pub mod snapshot_read {
    pub use semio_framework_os_store_read::*;
    pub use semio_framework_os_store_ledger::lifted_snapshot_read::*;
    pub use semio_framework_os_store_replay::lifted_snapshot_read::*;
}
pub use semio_framework_os_store_engine_owners as document_store_owners;
use snapshot_read::{ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY, ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ArtifactStoreDisplacedSnapshotRetirement, ErasedSnapshotRead, SnapshotRead, SnapshotReadRef, SnapshotReadReturn, admit_original_snapshot_alias, artifact_retirement_admit_owned, artifact_retirement_admit_shared_lease, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owned_birth_demands, retirement_owner_close, retirement_owner_demands, artifact_retirement_shared_lease_birth_demands, retire_document_envelope, retire_document_envelope_birth_bytes};
use semio_framework_os_store_engine_owners::{DocumentStoreOwners, DocumentStoreOwnersAdmissionError, complete_unscheduled_catalog, document_store_owners_constructor_birth_bytes, funded_member_store_owners};
use semio_framework_os_store_engine_owners::member_producer::MemberStoreProducer;
use snapshot_read::{ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY, ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY, ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS, ArtifactEnvelopeMetadata, ArtifactGenesisRetirement, ArtifactHistoryReadRetirement, ArtifactStoreAlternativeRetirementFactory, ArtifactStoreCheckpointRetirementFactory, ArtifactStoreCursorDisposerPhase, ArtifactStoreCursorRetirement, ArtifactStoreDisplacedOwnerReservation, ArtifactStoreDisplacedRetirements, ArtifactStoreEditMetadata, ArtifactStoreEditRetirement, ArtifactStoreEditRetirementState, ArtifactStoreEnvelopeRetirement, ArtifactStoreHistoryLedgerRetirement, ArtifactStoreHistoryMetadataRetirement, ArtifactStoreHistoryMetadataRetirementState, ArtifactStoreMutationDagRetirement, ArtifactStoreOperationRowOwners, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement, ArtifactStoreVcsRetirement, ArtifactStoreVcsRetirementPhase, DISPLACED_RELIEF_STEPS, HistoryReadPlan, SNAPSHOT_READ_LEASE_CAPACITY, SnapshotReadLease, SnapshotReadLeaseRefusal, SnapshotReadLeaseRegistry, SnapshotReadLeaseRegistryOwner, SnapshotReadLeaseRegistryState, SnapshotReadLeaseSlot, SnapshotReadLeaseStateGuard, SnapshotReadLeaseStateLock, advance_returned_snapshot_read, advance_returned_snapshot_read_root, artifact_history_ledger_birth, artifact_history_ledger_handoff, artifact_retirement_box_depth, retain_completed_history_plan, return_snapshot_read, snapshot_registry_frame_bytes, try_return_original_snapshot_read};
use semio_framework_os_store_engine_owners::{DocumentCatalogOwner, UNSCHEDULED_CATALOG_STEPS, UninstalledDocumentEnvelopeRetirement, close_unscheduled_catalog};

use semio_framework_os_store_read::history_plan_retirement;

use semio_framework_os_store_read::history_read_admission;
pub use history_read_admission::PreparedHistoryReadRetirement;

use semio_framework_os_store_retirement::resident_backing;


pub(crate) use semio_framework_value_snapshot_read::{SnapshotReadRegistryHandle, SnapshotReadRegistryAliasRetirement, snapshot_registry_alias_demands, snapshot_registry_alias_close_step};
pub use semio_framework_value_snapshot_read::snapshot_read_projection;
pub use semio_framework_value_snapshot_read::SnapshotReadProjection;

pub(crate) use semio_framework_os_store_ledger::edit_retirement;
use edit_retirement::{ArtifactStoreChangeRetirementFactory, ArtifactStoreDecodedEditRetirement, ArtifactStoreDecodedEditRetirementFactory, ArtifactStoreOperationRowsRetirement};

use semio_framework_os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;

use semio_framework_os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;

use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;


use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;


//#endregion 🧬️OpaqueSnapshotRead
use semio_framework_os_store_text_io::operation_identity;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::io::binary::entity_identity::{content_addressed_entity_id, edit_scoped_id, mint_alternative_id, mint_change_id, mint_edit_id, mint_mutation_id, mint_child_id, mint_invocation_id, operation_list_fingerprint, mint_space_checkpoint_id, mint_space_alternative_id};
use semio_framework_os_vcs::io::binary::checkpoint_identity::{content_addressed_checkpoint_id, content_addressed_checkpoint_id_with_pending_change};
pub use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Alternative, ArtifactHistoryKey, ArtifactHistoryLedger, ArtifactHistoryReservation, ArtifactVcs, Change, Checkpoint, CollectionDiff, Identified, ItemPatch, Patchable, apply_mutation};
//#region 🔖️Schemas
pub mod store_schema {
    pub use semio_framework_os_store_space_schema::*;
    pub use semio_framework_os_store_replay::ReplayTurnBudget;
}
use store_schema::{AcceptedArtifactProjection, ArtifactBackboneRef, ArtifactCommand, ArtifactCursor, ArtifactCursorOwners, ArtifactEnvelope, ArtifactEnvelopeOwners, ArtifactEnvelopeRead, ArtifactProjectionCacheMode, ArtifactProjectionCause, ArtifactProjectionEvent, ArtifactProjectionInvalidation, ArtifactProjectionResult, ArtifactProjectionStamp, ArtifactRevision, HistoryLane, MigrationProvenance, OpenToolTransaction, ReplayTurnBudget, StaleArtifactProjection, SupersedeInput, document_backbone_ref};
use replay::{PendingLocalStep,PendingReprojection,ReprojectionAdvance,TailStep};
//#region 🔖️HistoryLane
//#endregion 🔖️HistoryLane
//#endregion 🔖️Schemas
//#region 🔖️Composition


use semio_framework_os_store_child::artifact_child_retained_clone;

/// 🏠️ The ownership stamp placed on the CHILD's own `ArtifactEnvelope.owner` (not only on
/// the parent's `ArtifactChild` handle), so ownership is queryable directly from the child side —
/// e.g. "is this document embeddable standalone, or does deleting it require going through its
/// owner". `child_id` matches the owning `ArtifactChild<S>.child_id`/`ChildRef.child_id` exactly.
use semio_framework_os_store_envelope::owner as artifact_child_owner_schema;
pub use artifact_child_owner_schema::OwnerRef;

pub use semio_framework_os_store_link_schema::{ArtifactLink,LinkPin};


//#region 🔖️CompositionDsl
pub use semio_framework_os_store_child as artifact_child;
use artifact_child::{ArtifactChild, ArtifactChildMaterializationError, ArtifactChildRetainedParts, ChildRef};
use artifact_child::{ArtifactChildLocalText, artifact_child_from_record, artifact_child_spec, artifact_child_spec_controlled, artifact_child_spec_producer, artifact_child_to_record, owner_ref_from_record, owner_ref_spec, owner_ref_spec_controlled, owner_ref_spec_producer, owner_ref_to_record};
use semio_framework_os_store_link_schema::{artifact_link_from_record, artifact_link_from_record_controlled, artifact_link_spec, artifact_link_spec_controlled, artifact_link_spec_producer, artifact_link_to_record, artifact_link_to_record_controlled, link_pin_borrowed_spec, link_pin_from_record, link_pin_from_record_controlled, link_pin_spec, link_pin_spec_controlled, link_pin_spec_producer, link_pin_to_record, link_pin_to_record_controlled, validate_link_pin_record};


//#endregion 🔖️CompositionDsl
//#endregion 🔖️Composition
//#region 🔖️Authority

//#region 🔮️Projection
//#region 🧭️Stamps
//#endregion 🧭️Stamps
//#region 📡️Events
//#endregion 📡️Events
//#endregion 🔮️Projection


//#region 🔖️EphemeralLanes

pub use semio_framework_os_store_ephemeral::*;

//#region 🔖️Text
//#region 🔖️Text
pub mod serialization {
    pub use semio_framework_os_store_text_io::*;
    pub use semio_framework_os_store_space_history::lifted_serialization::*;
}
use serialization::{ArtifactCodec, ArtifactNativeSnapshotIdentity, ArtifactPackFiles, ArtifactSqliteSnapshotHost, ArtifactSqliteSnapshotCodec, decode_document_pack_bytes, encode_document_pack_bytes, text_error_to_pack_error};
#[cfg(not(target_arch = "wasm32"))]
use serialization::{ArtifactCodecApplyFuture};
#[cfg(target_arch = "wasm32")]
use serialization::{ArtifactCodecApplyFuture};
/// 📍️ 1-based line/column position inside DSL or op-log source text. Lives in `dsl_core`
/// (the token-native DSL engine's foundation crate, which sits below `vcs`); re-exported here so
/// every existing `crate::os_store::TextSpan`/`crate::os_store::TextError` import across the workspace keeps compiling.
pub use semio_framework_diagnostic::TextError;
pub use semio_framework_diagnostic::TextSpan;

pub use semio_framework_os_semio as semio_format;

// 🎞️ CW3 kernel cut-over: `OpText` moved (method order flipped, behavior unchanged) to
// `protocol_command`, re-exported via the `🚧️TEMPORARY protocol shim` near the top of this file.

//#endregion 🔖️Text

//#region 🔖️Pack
//#region 🔖️Pack
/// 📦️ Binary counterpart of `🔖️Text` above — see the wave-1 design at
/// `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/` for the full container-format
/// contract. `pack`'s own `EncodeOptions`/`DecodeOptions`/`VerificationLevel` are re-exported under
/// a `Pack`-prefixed name (not a plain re-export — `dsl_derive`'s emitted `ArtifactPack` impl and
/// every downstream caller spell them `crate::os_store::PackEncodeOptions`/`crate::os_store::PackDecodeOptions`/
/// `crate::os_store::PackVerificationLevel`, so there is exactly one spelling repo-wide).
pub use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackRefusal, VerificationLevel as PackVerificationLevel};

use semio_framework_os_pack::pack_rt;

/// 🧵️ Mounted canonical-pack capability surface. It intentionally exports only retained
/// cursors and wire constants: batch `decode_document`, `decode_record_body`, `RecordValue`, and
/// their allocation-oriented options are absent from this module's type-level reachability graph.
pub mod mounted_pack_rt {
    pub use semio_framework_os_store_document::mounted_pack_session::{RetainedTypedPackCloseStep, RetainedTypedPackOwner, RetainedTypedPackSession};
    use semio_framework_dsl_record::{DslField,FieldValue,RecordLayout,RecordSpec,RecordSpecProducer,RecordValue,Shape};
use semio_framework_value::DslValue;
    pub use semio_framework_os_pack::PackLimits;
    pub use pack::record::{RetainedRecordBodyCursor, RetainedRecordBodyToken, RetainedValueContainer, RetainedValueCursor, RetainedValueRole, RetainedValueToken};
    pub use pack::{
        MAGIC, RETAINED_PACK_MAXIMUM_PAGES, RETAINED_PACK_PAGE_BYTES, RetainedPackAnchorCursor, RetainedPackCatalog, RetainedPackCatalogAllocationError, RetainedPackCatalogAllocationStep, RetainedPackCatalogCursor, RetainedPackCatalogEvent,
        RetainedPackCatalogFault, RetainedPackCatalogProgress, RetainedPackCloseStep, RetainedPackPage, RetainedPackSegmentCursor, RetainedPackSegmentEvent, RetainedPackSourceAllocationError, RetainedPackSourceAllocationStep,
        RetainedPackSourceCursor, RetainedPackSourceEvent, RetainedPackSourceProgress,
    };
    pub use pack::format::RetainedPackSourceFault;
}
//#region 🧬️OwnedSchemaDecode
pub mod owned_schema_decode {
    pub use semio_framework_os_store_owned_schema_decode::*;
    pub use semio_framework_os_store_owned_schema_fields::lifted_owned_schema_decode::*;
}
use owned_schema_decode::{ARTIFACT_ENVELOPE_ACTIVE_ALTERNATIVE_FIELD, ARTIFACT_ENVELOPE_BACKBONE_FIELD, ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY, ARTIFACT_ENVELOPE_CONFLICTS_FIELD, ARTIFACT_ENVELOPE_CURSOR_FIELD, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, ARTIFACT_ENVELOPE_DIALECT_FIELD, ARTIFACT_ENVELOPE_EDIT_MESSAGES_FIELD, ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY, ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES, ARTIFACT_ENVELOPE_HISTORY_ITEMS, ARTIFACT_ENVELOPE_ID_FIELD, ARTIFACT_ENVELOPE_LANES_FIELD, ARTIFACT_ENVELOPE_MIGRATED_FROM_FIELD, ARTIFACT_ENVELOPE_OWNER_FIELD, ARTIFACT_ENVELOPE_SCHEMA_FIELD, ARTIFACT_ENVELOPE_VCS_FIELD, ArtifactEnvelopeCompletedRecord, ArtifactEnvelopeCompletedRecordFault, ArtifactEnvelopeCompletedRecordOwner, ArtifactEnvelopeCompletedRecordRegistry, ArtifactEnvelopeCompletedRecordTarget, ArtifactEnvelopeCompletedRecordTicket, ArtifactEnvelopeDecodeAuthority, ArtifactEnvelopeDecodeCompletion, ArtifactEnvelopeDecodeOwnerBundle, ArtifactEnvelopeDecodePage, ArtifactEnvelopeDecodeRejected, ArtifactEnvelopeDecodedRecordTarget, ArtifactEnvelopeFieldDecodeStep, ArtifactEnvelopeFieldDecoder, ArtifactEnvelopeFieldDecoderLease, ArtifactEnvelopeFieldDecoderRegistry, ArtifactEnvelopeFieldDecoderRegistryFault, ArtifactEnvelopeFieldDecoderTicket, ArtifactEnvelopeFieldReservation, ArtifactEnvelopeFreshFieldDecoder, ArtifactEnvelopeFreshVcsAuthority, ArtifactEnvelopeMutationFieldAuthority, ArtifactEnvelopeMutationFieldTarget, ArtifactEnvelopeOwnedFieldCatalog, ArtifactEnvelopeReturnedFieldDecoder, ArtifactEnvelopeSnapshotFieldAuthority, ArtifactEnvelopeSnapshotFieldTarget, ArtifactEnvelopeSprConflictAuthority, ArtifactEnvelopeUnadmittedDecodeRejected, ArtifactEnvelopeVcsFieldAuthority, ArtifactEnvelopeWholeBufferIngressError, ArtifactOwnedHistoryEntryAuthority, ArtifactOwnedHistoryEntryDecoder, OWNED_SCHEMA_DECODE_MAXIMUM_PAGES, OWNED_SCHEMA_DECODE_PAGE_BYTES, OwnedSchemaDecodeAdmissionFault, OwnedSchemaDecodeCredits, OwnedSchemaDecodeDiagnostic, OwnedSchemaDecodePage, OwnedSchemaDecodePageFault, OwnedSchemaDecodePages, OwnedSchemaEmptyArrayAuthority, OwnedSchemaFieldSpec, OwnedSchemaHexAuthority, OwnedSchemaHexStep, OwnedSchemaNestedRecordCursor, OwnedSchemaNestedRecordStep, OwnedSchemaPath, OwnedSchemaRecordCursor, OwnedSchemaRecordSpec, OwnedSchemaRecordStep, OwnedSchemaStringAuthority, OwnedSchemaStringStep, OwnedSchemaToken, OwnedSchemaTokenCursor, OwnedSchemaTokenKind, OwnedSchemaTokenStep, artifact_bounded_history_entry_decoder, artifact_envelope_decode_record, artifact_envelope_owned_schema, artifact_owned_spr_edit_history_decoder, reject_whole_buffer_artifact_envelope_ingress};
use owned_schema_decode::{ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS, ARTIFACT_ENVELOPE_OWNED_SCHEMA_FIELDS, ARTIFACT_OWNED_SPR_EDIT_FIELDS, ArtifactEnvelopeCompletedRecordRegistryState, ArtifactEnvelopeCompletedRecordSlot, ArtifactEnvelopeDecodeState, ArtifactEnvelopeFieldDecoderRegistryState, ArtifactEnvelopeFieldDecoderSlot, ArtifactEnvelopeFreshRecordActive, ArtifactEnvelopeFreshRecordTarget, ArtifactEnvelopeFreshSnapshotTarget, ArtifactEnvelopeFreshVcsActive, ArtifactOwnedSprEditActive, ArtifactOwnedSprEditAuthority, ArtifactOwnedSprEditDecoder, ArtifactOwnedSprMutationArrayAuthority, ArtifactOwnedSprMutationArrayState, ArtifactOwnedSprMutationTarget, ArtifactRepositoryHistoryEntryAuthority, ArtifactRepositoryHistoryEntryDecoder, OwnedSchemaBoundedArrayAuthority, OwnedSchemaBoundedArrayState, OwnedSchemaLexState, OwnedSchemaRecordState, OwnedSchemaStringState, artifact_envelope_diagnostic_error, artifact_envelope_field_demands, artifact_retirement_grant_covers, artifact_retirement_self_grant};

pub use semio_framework_pack_json_retained_schema::schema_diagnostic;
//#endregion 🧬️OwnedSchemaDecode

pub use semio_framework_dsl_record::{RecordSpecProducer,BorrowedRecordSpecProducer};
pub use pack::snapshot::{ArtifactDsl,ArtifactPack};
pub use serialization::ArtifactStoreSnapshot;

//#endregion 🔖️Pack
//#region 🔖️OpRt
//#endregion 🔖️OpRt
//#region 🔖️CodecRegistry
//#region 🔖️CodecRegistry
pub use semio_framework_os_store_codec_registry as codec_registry;
use codec_registry::{ArtifactAssemblyStoreRegistryError, ArtifactAssemblyStoreRegistryGuards, ComponentDocumentCodec, DialectMigration, DialectMigrationError, DialectMigrationRegistryError, DocumentCodecRegistryConflict, DocumentCodecRegistryError, DocumentKindCodec, acquire_artifact_assembly_store_registry_guards, canonical_check_in_line, commit_artifact_assembly_store_registry_guards, document_codec, document_kind_codec, lane_schema_from_spr, migrate_document, preflight_artifact_assembly_store_registry_guards, preflight_dialect_migrations, preflight_dialect_migrations_in_assembly, preflight_document_codecs, preflight_document_codecs_in_assembly, register_component_document_codec, register_dialect_migration, register_dialect_migrations, register_dialect_migrations_in_assembly, register_document_codec, register_document_codecs, register_document_codecs_in_assembly, validate_owned_sqlite_snapshot_subset};
use semio_framework_os_store_engine_owners::{bounded_artifact_store_owners, bounded_artifact_store_owners_birth_bytes, bounded_artifact_store_owners_birth_demand, bounded_artifact_store_owners_source_demands, bounded_artifact_store_owners_with_one_item_preparation, fund_document_store_owners, funded_bounded_artifact_store_owners, funded_bounded_artifact_store_owners_with_one_item_preparation};
#[cfg(not(target_arch = "wasm32"))]
use codec_registry::{ComponentDocumentCodecFuture};
#[cfg(target_arch = "wasm32")]
use codec_registry::{ComponentDocumentCodecFuture};
use codec_registry::{COMPONENT_DOCUMENT_CODEC_REGISTRY, DIALECT_MIGRATION_REGISTRY, DOCUMENT_CODEC_REGISTRY, component_document_codec_registry, dialect_migration_registry, document_codec_registry, same_dialect_migration, same_document_codec, validate_dialect_migrations, validate_document_codecs};
use semio_framework_os_store_engine_owners::BoundedArtifactRetirementFactory;

use semio_framework_os_store_codec_registry::native_snapshot;
pub use semio_framework_os_io::control::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner, NativeSnapshotBodyWallet};
pub use native_snapshot::{ArtifactPackReceiving,decode_native_snapshot_record,encode_native_snapshot_record};
pub use pack::snapshot::{ArtifactNativeSnapshot,NativeSnapshotEncoding,NativeSnapshotInput};

use semio_framework_os_store_codec_registry::sqlite_snapshot_retirement;

use semio_framework_os_io::sqlite_snapshot_operations;
pub use sqlite_snapshot_operations::ArtifactSqliteSnapshotOperations;

mod sqlite_snapshot_workspace {
    pub use semio_framework_os_store_space_history::sqlite_snapshot_workspace::*;
    pub use semio_framework_os_store_codec_registry::lifted_sqlite_snapshot_workspace::*;
}

use semio_framework_os_store_codec_registry::sqlite_snapshot_native_decoding;
pub use sqlite_snapshot_native_decoding::decode_sqlite_snapshot_record_native;

use semio_framework_os_store_codec_registry::sqlite_snapshot_native_encoding;
pub use sqlite_snapshot_native_encoding::encode_sqlite_snapshot_record_native;

//#region 🗃️BoundedArtifactStoreOwners
//#endregion 🗃️BoundedArtifactStoreOwners


//#endregion 🔖️CodecRegistry
//#region 🔖️DialectMigration
//#endregion 🔖️DialectMigration
//#region 🔖️MergeHelpers


//#endregion 🔖️MergeHelpers
//#region 🔖️Config


pub use semio_framework_mutation_diff::{KeyedRow,KeyedRows,keyed_rows_apply,keyed_rows_inverse,keyed_rows_changing,keyed_rows_absorb};

/// 🎚️ Generates `$set`, the per-field set mutations of `$record`: one `Set…(value)` variant per field whose diff sets that one
/// slot of `$diff` (empty when the base already holds the value) and whose inverse is the same variant carrying the base value.
/// There is deliberately no whole-record variant: a caller holding a whole next record dispatches `$set::setting(&base, &next)`, one set
/// mutation per changed field. (The one exception is an ephemeral transient root, whose transfer moves a whole root — see
/// `transient_root!` in the plugin crate.) The enum owns its leaf descriptors (`owner/kind`), its `{"kind": …, "value": …}` wire shape
/// and semantic value lowering. Representation owners explicitly declare their text and binary codecs.
#[macro_export]
macro_rules! field_set_mutations {
    (
        record: $record:ident,
        diff: $diff:ident,
        set: $set:ident,
        owner: $owner:literal,
        payload_schema: $schema:literal,
        emoji: $emoji:literal,
        fields: { $($field:ident : $ty:ty => $variant:ident $kind:literal),+ $(,)? } $(,)?
    ) => {
        #[doc = concat!("🎚️ The set mutations of [`", stringify!($record), "`].")]
        #[derive(Clone, Debug, PartialEq, $crate::__value::RetainedClone)]
        pub enum $set {
            $($variant($ty)),+
        }

        impl $crate::snapshot_clone_preparation::ConfigApplyMutation<$record> for $set {
            fn exchange(self, post: &mut $record) -> Result<Self, ($crate::__value::ValueError, Self)> {
                Ok(match self {
                    $(Self::$variant(value) => Self::$variant(std::mem::replace(&mut post.$field, value))),+
                })
            }
        }

        impl $crate::__value::retirement::RetireOwned for $set {
            fn retirement(self)->Box<dyn $crate::__value::retirement::RetirementCursor>{match self{$(Self::$variant(original)=>$crate::__value::retirement::RetireOwned::retirement(original)),+}}
            fn retirement_birth_bytes(&self)->Option<usize>{match self{$(Self::$variant(original)=>$crate::__value::retirement::RetireOwned::retirement_birth_bytes(original)),+}}
            fn controlled_retirement_supported()->bool{true}
        }

        impl $crate::__value::ToValue for $set {
            fn to_value(&self) -> $crate::__value::DslValue {
                match self {
                    $(Self::$variant(value) => $crate::__value::DslValue::object(vec![
                        ("kind".to_string(), $crate::__value::DslValue::String($kind.to_string())),
                        ("value".to_string(), $crate::__value::ToValue::to_value(value)),
                    ]),)+
                }
            }
        }

        impl $crate::__value::FromValue for $set {
            fn from_value(value: $crate::__value::DslValue) -> Result<Self, $crate::__value::ValueError> {
                let entries = value.into_object()?;
                if let Some((member, _)) = entries.iter().find(|(key, _)| key != "kind" && key != "value") {
                    return Err($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, format!("unknown {} member {member}", stringify!($set))));
                }
                let kind = match entries.iter().find(|(key, _)| key == "kind") {
                    Some((_, $crate::__value::DslValue::String(kind))) => kind.clone(),
                    _ => return Err($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, concat!(stringify!($set), " requires a string kind"))),
                };
                let entry = |name: &str| {
                    entries
                        .iter()
                        .find(|(key, _)| key == name)
                        .map(|(_, value)| value.clone())
                        .ok_or_else(|| $crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, format!("{} {kind} requires {name}", stringify!($set))))
                };
                $(if kind == $kind {
                    return Ok(Self::$variant(<$ty as $crate::__value::FromValue>::from_value(entry("value")?).map_err(|error| error.under("value"))?));
                })+
                Err($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, format!("unknown {} kind {kind}", stringify!($set))))
            }
        }

        impl $crate::Mutation<$record> for $set {
            type Diff = $diff;

            const DESCRIPTORS: &'static [$crate::MutationLeafDescriptor] = &[
                $($crate::MutationLeafDescriptor {
                    schema_version: 1,
                    owner: concat!($owner, "/", $kind),
                    semantic_kind: $kind,
                    display_name: $kind,
                    emoji: $emoji,
                    aggregate_variant: stringify!($variant),
                    payload_schema: $schema,
                    text_opcode: None,
                    binary_tag: None,
                    invertibility: $crate::MutationInvertibility::ExplicitMutation,
                    diff_participation: $crate::MutationDiffParticipation::Detect,
                    outcome_classes: &[$crate::MutationOutcomeClass::Applied],
                    composition: $crate::MutationComposition::Atomic,
                    required_language_surfaces: &[$crate::MutationLanguageSurface::Rust, $crate::MutationLanguageSurface::JsonSchema],
                }),+
            ];

            fn descriptor(&self) -> &'static $crate::MutationLeafDescriptor {
                let variant = match self {
                    $(Self::$variant(_) => stringify!($variant)),+
                };
                Self::DESCRIPTORS.iter().find(|descriptor| descriptor.aggregate_variant == variant).expect("every set mutation variant declares its descriptor")
            }

            fn diff(&self, base: &$record) -> $crate::MutationOutcome<$diff> {
                match self {
                    $(Self::$variant(value) => {
                        if base.$field == *value {
                            return $crate::MutationOutcome::empty();
                        }
                        $crate::MutationOutcome::new($diff { $field: Some(value.clone()), ..<$diff as Default>::default() })
                    })+
                }
            }

            fn inverse(&self, base: &$record) -> Result<Vec<Self>, $crate::__value::ValueError> {
                Ok(vec![match self {
                    $(Self::$variant(_) => Self::$variant(base.$field.clone())),+
                }])
            }
        }

        impl $set {
            #[doc = concat!("🎚️ The set mutations that turn `base` into `next`: one per field where they differ, in field order.")]
            pub fn setting(base: &$record, next: &$record) -> Vec<Self> {
                let mut mutations = Vec::new();
                $(if base.$field != next.$field {
                    mutations.push(Self::$variant(next.$field.clone()));
                })+
                mutations
            }
        }

    };
}

/// 🎯️ Declares a config record's sparse diff ([`sparse_record_diff!`]), its per-field set mutations ([`field_set_mutations!`]; no
/// whole-record variant, callers dispatch `$set::setting(&base, &next)`) and its `ConfigRecord` mark in one invocation: `config_record! { record: X, diff: XDiff, set: XSet, owner: "…", payload_schema:
/// "…", emoji: "…", fields: { field: Type => SetField "set-field", … } }`. Configs that keep a hand-written mutation enum use
/// `config_diff!` for the diff alone.
#[macro_export]
macro_rules! config_record {
    (
        record: $record:ident,
        diff: $diff:ident,
        set: $set:ident,
        owner: $owner:literal,
        payload_schema: $schema:literal,
        emoji: $emoji:literal,
        fields: { $($field:ident : $ty:ty => $variant:ident $kind:literal),+ $(,)? } $(,)?
    ) => {
        impl $crate::ConfigRecord for $record {}
        $crate::sparse_record_diff! { record: $record, diff: $diff, fields: { $($field: $ty),+ } }
        $crate::field_set_mutations! {
            record: $record, diff: $diff, set: $set, owner: $owner, payload_schema: $schema, emoji: $emoji,
            fields: { $($field: $ty => $variant $kind),+ }
        }
    };
}

/// 🎯️ Declares a config record's `ConfigRecord` mark and sparse diff ([`sparse_record_diff!`]) for a config whose mutation enum is
/// hand-written: `config_diff! { record: X, diff: XDiff, fields: { field: Type, … } [, keyed: { map_field: ValueType, … }] }`.
#[macro_export]
macro_rules! config_diff {
    (record: $record:ident, diff: $diff:ident, fields: { $($field:ident : $ty:ty),+ $(,)? }, keyed: { $($kf:ident : $kv:ty),* $(,)? } $(,)?) => {
        impl $crate::ConfigRecord for $record {}
        $crate::sparse_record_diff! { record: $record, diff: $diff, fields: { $($field: $ty),+ }, keyed: { $($kf: $kv),* } }
    };
    (record: $record:ident, diff: $diff:ident, fields: { $($field:ident : $ty:ty),+ $(,)? } $(,)?) => {
        impl $crate::ConfigRecord for $record {}
        $crate::sparse_record_diff! { record: $record, diff: $diff, fields: { $($field: $ty),+ } }
    };
}
//#endregion 🔖️Config
//#region 🔖️Materialize
pub mod document_materialize {
    pub use semio_framework_os_store_document::*;
    pub use semio_framework_os_store_engine_live::lifted_document_materialize::*;
    pub use semio_framework_os_store_engine_core::lifted_document_materialize::*;
    pub use semio_framework_os_store_initialization_runtime::lifted_document_materialize::*;
    pub use semio_framework_os_store_space_member::lifted_document_materialize::*;
}
use semio_framework_os_store_engine_core::lifted_document_materialize::{EMPTY_CHECKPOINT_MESSAGE, now_iso};
use semio_framework_os_store_document::{create_document_envelope, create_document_envelope_from_genesis, create_document_envelope_from_genesis_owners, empty_document_envelope_from_genesis_owners};
use semio_framework_os_store_space_member::lifted_document_materialize::{edit_ids_for_changes, materialize_document_snapshot};
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::now_ms;
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::fresh_replica_clock;
use semio_framework_os_store_engine_live::lifted_document_materialize::stamp_primary_operation_identity;
use semio_framework_os_store_engine_core::lifted_document_materialize::uncommitted_edit_ids;
//#endregion 🔖️Materialize
//#region 🔖️TextFormat
//#region 🔖️TextFormat
pub use semio_framework_os_store_text_io::text_format;
use text_format::{ArtifactTextFiles, ParsedDocumentText, append_history_events_to_spr, empty_document_spr, genesis_member_envelope_pack, parse_decoded_document_spr, parse_document_pack, parse_document_spr, parse_document_text, print_document_pack, print_document_spr, print_document_text, print_edit_lines, stamp_document_spr_identity};
use text_format::{adopt_loaded_replay, apply_history_composition, canonical_conflict_actors, common_address, conflict_from_history_conflict, conflict_messages_for_edits, discard_parsed_envelope, expected_mutation_message_level, fill_supersession_targets, history_composition_from_envelope, history_conflict_from_conflict, history_edit_from_edit, history_message_from_mutation_message, history_op_meta_from_operation_meta, history_op_payloads, loaded_history_replay, mutation_ids, mutation_message_from_history_message, mutation_meta_from_history_op_meta, operation_ids, ops_line_from_transition, parse_clock, print_clock, print_ops_log, project_envelope_history, protocol_undo_policy_from_ordinal, protocol_undo_policy_ordinal, replay_loaded_history, replay_ops, settle_parsed_envelope, stable_mutation_ids_for_edit, supersession_address, take_loaded_replay, transition_from_ops_line, validate_durable_history, validate_history_lanes, validate_persisted_conflicts, validate_persisted_message};
//#region 🔖️OpsHeaderGrammar

//#region 🔖️OpCodec
//#endregion 🔖️OpCodec
//#endregion 🔖️OpsHeaderGrammar
//#endregion 🔖️TextFormat

//#region 🔖️CommandFormat

pub use semio_framework_os_store_command as command_format;
use command_format::{COMMAND_BINARY_FORMAT, CommandDecodeError, parse_command, print_command};
use command_format::{CommandHeaderLine, OpsAuthor, OpsHeaderLine, OpsPin, OpsSupersededInput, SupersedeInputLine, conflict_resolution_from_ordinal, conflict_resolution_ordinal, conflict_resolution_to_token, dedent_command_lines, history_lane_from_ordinal, history_lane_ordinal, history_lane_to_token, merge_policy_to_token, ops_line_provenance, parse_conflict_resolution_token, parse_history_lane_token, parse_indented_ops, parse_merge_policy_token, parse_supersede_input_lines, parse_supersede_inputs, parse_undo_policy_token, print_indented_ops, print_supersede_inputs, read_command_op, read_command_ops, read_command_str, read_command_transaction, read_supersede_input, read_supersede_inputs, retire_command, retire_operations, transaction_from_text, transaction_text, undo_policy_from_ordinal, undo_policy_ordinal, undo_policy_to_token, write_command_ops, write_command_str, write_command_transaction, write_supersede_inputs};
//#region 🔖️OpCodec
//#endregion 🔖️OpCodec

semio_framework_diagnostic::fault_from_error!(CommandDecodeError, semio_framework_diagnostic::FaultOrigin::Module, "module.store.command");

//#endregion 🔖️CommandFormat
//#region 🔖️History
//#region 🔖️History


//#endregion 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
pub use semio_framework_os_store_ledger as ledger;
use ledger::{ArtifactEditMessageLedger, ArtifactEditMessageLedgerIter, ArtifactEditMessageLedgerRejected, edit_from_operation_envelope, fold_envelope_history, fold_event_log};
use ledger::{ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES, ARTIFACT_EDIT_MESSAGE_ID_BYTES, ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY, ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY, ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE, ARTIFACT_EDIT_MESSAGE_LEDGER_BYTES, ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE, ARTIFACT_EDIT_MESSAGE_OWNER_BYTES, ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS, ARTIFACT_EDIT_MESSAGE_SLOT_PAGES, ARTIFACT_EDIT_MESSAGE_TARGET_BYTES, ArtifactEditMessageLedgerRetirement, ArtifactEditMessageTicket, ArtifactStoreHistoryCommitReservation, ArtifactStorePendingReportRetirement, CursorRevisionAccumulator, CursorRevisionRecord, EditDigestChains, PendingCommandReport, ReplayProjection, StackMirrorDirt, edit_timestamp, envelope_viewer_head, operation_semantic_kind, retire_replayed_projection, retire_scratch_edits, retire_scratch_operations, retire_supersede_inputs, stamp_edit_semantics, validate_composition_pins};


use semio_framework_os_store_initialization_runtime::initialization_catalog_close;

use semio_framework_os_store_initialization_replay::initialization_authority;
pub use semio_framework_os_store_initialization_replay::initialization_authority::ArtifactStoreInitializationAuthority;

mod initialization_replay {
    pub use semio_framework_os_store_initialization_replay::*;
    pub use semio_framework_os_store_initialization_runtime::lifted_initialization_replay::*;
}
pub use semio_framework_os_store_initialization_runtime::lifted_initialization_replay::ArtifactStoreInitializationNext;
pub use semio_framework_os_store_initialization_replay::{ArtifactStoreReplayInitializer, ARTIFACT_STORE_REPLAY_FIELD_BYTES};


//#region 📬️OneItemPublication
pub mod one_item_publication {
    pub use semio_framework_os_store_one_item_publication::*;
    pub use semio_framework_os_store_one_item_publication::lifted_publication::*;
    pub use semio_framework_os_store_one_item_batch::lifted_one_item_publication::*;
}
use one_item_publication::{ArtifactStoreBatchAdmissionRejected, ArtifactStoreBatchPublication, ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemPublicationPhase, ErasedMemberStoreOneItemPublication, MemberStoreOneItemWire, MemberStoreOneItemWirePreparationFactory, MemberStoreOneItemWireRequest};
use one_item_publication::{ArtifactStoreBatchItemAuthority, ArtifactStoreBatchItemRequest, ArtifactStoreBatchSource, ArtifactStoreBatchSourceOf, ArtifactStoreBatchStage, ArtifactStoreOneItemSeal, ArtifactStoreWireRetirement, ArtifactTypedBatchItemAuthority, MemberStoreOneItemPublication};
mod canonical_edit {
    pub use semio_framework_os_store_canonical_json::*;
    pub use semio_framework_os_store_one_item_publication::lifted_canonical_edit::*;
}
use semio_framework_os_store_one_item_publication::prepared_retirement;
pub use prepared_retirement::ArtifactStorePreparedRetirement;
use semio_framework_os_store_one_item_publication::batch_digest;
pub use batch_digest::{ArtifactStoreBatchDigest,admit_artifact_batch_digest,artifact_batch_digest_birth_demand};
pub use canonical_edit::{
    ARTIFACT_CANONICAL_JSON_CHUNK_BYTES, ARTIFACT_CANONICAL_JSON_DEPTH, ArtifactCanonicalJson, ArtifactCanonicalJsonArray, ArtifactCanonicalJsonCursor, ArtifactCanonicalJsonEncodeError, ArtifactCanonicalJsonNode, ArtifactCanonicalJsonObject, ArtifactCanonicalJsonText, ArtifactCanonicalJsonTree, ArtifactCanonicalJsonTreeCursor, ArtifactCanonicalJsonTreeStep,
    ArtifactCanonicalJsonReader, ArtifactCanonicalJsonValue, ArtifactCanonicalValue, ArtifactCanonicalValueAdmission, ArtifactCanonicalValueCheckpoint, ArtifactCanonicalValueCloseStep, ArtifactCanonicalValueGrant, ArtifactCanonicalValueLimits,
    ArtifactCanonicalValueStep, ArtifactStoreOneItemSealCheckpoint, ArtifactStoreOneItemSealer,
};

mod prepared_operation_wire {
    pub use semio_framework_os_store_operation_wire::*;
    pub use semio_framework_os_store_one_item_publication::lifted_prepared_operation_wire::*;
}
pub use prepared_operation_wire::{ArtifactCanonicalTreeSource, ArtifactOperationWire, ArtifactOperationText, ArtifactOperationTextNode, ArtifactOperationTextCursor, ArtifactPreparedOperationSource, ArtifactPreparedOperationCursor, ArtifactPreparedOperationProgress, ArtifactPreparedOperationError, ArtifactPreparedOperationOutput, ArtifactPreparedOperations, ArtifactPreparedOperationsOwner, operation_wire_preparation_factory, operation_wire_preparation_factory_birth_bytes, operation_wire_preparation_factory_source_birth_demand};

pub use semio_framework_os_store_one_item as one_item;
use one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_RETRIES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, mutation_inverse_rows_declaration_failures, validate_original_preparation_step};

//#region 🧺️BatchSource

//#endregion 🧺️BatchSource
//#region 🧩️MemberOneItemPublication
mod member_owned_batch {
    pub use semio_framework_os_store_one_item_batch::member_owned_batch::*;
    pub use semio_framework_os_store_one_item_publication::lifted_member_owned_batch::*;
}
pub use member_owned_batch::{MemberStoreOwnedBatch, MemberStoreOwnedBatchRequest};
//#endregion 🧩️MemberOneItemPublication
//#endregion 📬️OneItemPublication
use semio_framework_os_store_initialization_replay::original_store_genesis_factory;
pub use semio_framework_os_store_initialization_replay::original_store_genesis_factory::OriginalStoreGenesisFactory;

use semio_framework_os_store_initialization_runtime::native_publication_producer;
pub use semio_framework_os_store_initialization_runtime::native_publication_producer::{ArtifactStorePublication, ArtifactSnapshotPublication};


//#endregion 🔖️ArtifactStore
//#region 🔖️PairMerge
pub mod replay {
    pub use semio_framework_os_store_replay::*;
    pub use semio_framework_os_store_ledger::lifted_replay::*;
}
use replay::{DRAFT_SUPERSESSION_ID, EditReplay, EditReplayResult, EffectiveOperation, EffectiveSupersessions, PairMerge, ReplayEdits, ReplayMode, ReplayOwnedState, ReplayProgress, ReplayResultOwnedState, ReplayStep, admit_replacement, apply_operation, apply_outcome, draft_supersessions, effective_forwards, effective_operation, fold_operation};
use replay::{FoldProjection, IndexedAppliedReplayEdits, PREFIX_SNAPSHOT_RING_CAPACITY, PREFIX_SNAPSHOT_RING_FLOOR, PrefixSnapshot, REPROJECTION_OWNER_SLOTS, ReplayConvergence, edit_supersession_digest, effective_operation_with_supersession, fold_effective_edit, fold_recorded, forward_prefix_digests, forward_prefix_genesis, forward_prefix_step, mirror_id_suffix, order_ledger_as, retire_shared_projection, same_effective_inputs, shared_prefix_len, superseded_positions, supersession_digest};


//#endregion 🔖️PairMerge
//#region 🔖️EffectiveForwards
//#endregion 🔖️EffectiveForwards
//#region 🔖️EditReplay
use semio_framework_os_store_replay::raw_replay_retirement;
//#endregion 🔖️EditReplay
//#region 🔖️Backbone
//#region 🔖️Backbone
pub use semio_framework_os_store_backbone as backbone;
use backbone::{ActorBackboneChannelOwner, ActorBackboneChannelPort, BACKBONE_CHANNEL_MAXIMUM_BYTES, BACKBONE_CHANNEL_MAXIMUM_MESSAGES, BACKBONE_GENESIS_MESSAGE_MAXIMUM_BYTES, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, Backbone, BackboneChannelPort, BackboneChannelPorts, BackboneMessage, BackbonePort, BackbonePorts, Backbones, ChannelBackbone, ChannelBackboneRemote, LocalStorageBackbonePort, MemoryBackbone, MemoryBackbonePort, PortBackbone, decode_backbone_message_exact, decode_hot_backbone_message_exact, set_host_backbone_port};
#[cfg(target_arch = "wasm32")]
use backbone::resolve_backbone;
use backbone::{ActorBackboneChannelState, HOST_BACKBONE_PORT, host_backbone_port, push_actor_backbone_outbound};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use backbone::local_storage_backbone_key;
//#region 🔖️OpCodec
//#endregion 🔖️OpCodec
use semio_framework_os_store_backbone::backbone_retirement;
use backbone_retirement::ArtifactStoreBackboneRetirement;
//#endregion 🔖️Backbone

//#region 🔖️BlobStore
//#region 🔖️BlobStore
pub use semio_framework_os_store_blob_identity::BlobRef;


//#endregion 🔖️BlobStore
//#region 🔖️Space
//#region SpaceMember


/// 🧬️ Generates a per-plugin `SpaceMember`+`MemberFactory` enum spanning several document
/// kinds composed under one `SpaceHost`/`CompositionCoordinator` — the O1 replacement for a
/// `Box<dyn SpaceMember>` heterogeneous registry. Delegation arms are written ONCE here, next to
/// `trait SpaceMember` itself, so drift between the trait and the macro is a compile error, never a
/// silent bug. Usage:
/// ```ignore
/// space_members! {
///     native_producer:&semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
///     history_limits: HistoryReadLimits { file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64 },
///     maximum_history_allocation_bytes:8388608,
///     pub enum NoteMembers, NoteMembersOpen {
///         Text("s.note.text", "1", "*", "note.text/v1") => (TextSnapshot, TextMutation, TextMemberStoreProducer, TextMemberStoreProducer),
///         Sketch("s.note.sketch", "1", "*", "note.sketch/v1") => (SketchSnapshot, SketchMutation, SketchMemberStoreProducer, SketchMemberStoreProducer),
///     }
/// }
/// ```
/// expands to the enum, `impl SpaceMember for NoteMembers` (match-delegation over every required
/// method), and `impl MemberFactory for NoteMembers` with an exact coordinate per typed store.
#[macro_export]
macro_rules! space_members {
    (native_producer:$native_producer:expr,history_limits:$history_limits:expr,maximum_history_allocation_bytes:$history_allocation_bytes:expr,pub enum $enum_name:ident, $open_name:ident { $($variant:ident($kind:literal, $standard:literal, $subset:literal, $schema:literal) => ($snapshot:ty, $mutation:ty, $catalog:ty, $catalog_value:expr)),+ $(,)? }) => {
        pub enum $enum_name {
            $($variant(Box<$crate::os_store::ArtifactStore<$snapshot, $mutation>>)),+
        }

        pub enum $open_name {
            $($variant(Box<$crate::os_store::InitialMemberStoreOpen<$enum_name, $snapshot, $mutation,$catalog>>)),+
            , ReturningSource(::semio_framework_job::OriginalCallerSourceLoan), Closed
        }

        impl $crate::os_store::MemberOpenOperation for $open_name {
            type Member = $enum_name;

            fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<$crate::os_store::MemberOpenStep<Self::Member>,::semio_framework_job::OriginalTurnFailure> {
                Ok(match self {
                    $(Self::$variant(open) => match open.step_store(cx, grant)? {
                        $crate::os_store::MemberOpenStep::Pending(progress) => $crate::os_store::MemberOpenStep::Pending(progress),
                        $crate::os_store::MemberOpenStep::Ready(member) => $crate::os_store::MemberOpenStep::Ready($enum_name::$variant(member)),
                        $crate::os_store::MemberOpenStep::Rejected(diagnostic) => $crate::os_store::MemberOpenStep::Rejected(diagnostic),
                    }),+,
                    Self::ReturningSource(_) | Self::Closed => $crate::os_store::MemberOpenStep::Rejected($crate::os_store::MemberOpenDiagnostic::Stale),
                })
            }

            fn close_step(&mut self,parent:&::semio_framework_job::OriginalCallerSourceParent,cx:&mut ::semio_framework_job::StepContext<'_>)->Result<::semio_framework_value::retained_clone::RetainedCloneStep,::semio_framework_job::OriginalTurnFailure>{
                if let Self::ReturningSource(loan)=self{let step=loan.close_step(parent,cx)?;let progress=step.progress();if loan.terminal_is_empty(){*self=Self::Closed}return Ok(if matches!(self,Self::Closed){::semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)}else{::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)})}
                let extent=match self{$(Self::$variant(open)if open.payload_terminal_is_empty()=>{open.validate_source_context(cx)?;Some(::std::mem::size_of_val(open.as_ref()))},)+Self::Closed=>return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())),_=>None};
                if let Some(extent)=extent{let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<extent{return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()))}let loan=match self{$(Self::$variant(open)=>open.take_source_before_shell_return(cx)?,)+_=>unreachable!()};let original=::std::mem::replace(self,Self::ReturningSource(loan));drop(original);return $crate::os_store::member_open::receive_member_close(cx,Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(::semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,released_bytes:extent,..Default::default()})))}
                match self{$(Self::$variant(open)=>$crate::os_store::MemberOpenOperation::close_step(open.as_mut(),parent,cx).map(|step|::semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress())),)+Self::ReturningSource(_)|Self::Closed=>unreachable!()}
            }
            fn next_copy_byte_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) if !open.payload_terminal_is_empty() => $crate::os_store::MemberOpenOperation::next_copy_byte_demand(open.as_ref()),)+ _ => Ok(0) }
            }
            fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) if !open.payload_terminal_is_empty() => $crate::os_store::MemberOpenOperation::next_capacity_byte_demand(open.as_ref(), body),)+ _ => Ok(0) }
            }
            fn next_release_byte_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) => if open.payload_terminal_is_empty() { Ok(::std::mem::size_of_val(open.as_ref())) } else { $crate::os_store::MemberOpenOperation::next_release_byte_demand(open.as_ref()) },)+ Self::ReturningSource(_)|Self::Closed => Ok(0) }
            }
            fn next_depth_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) => if open.payload_terminal_is_empty() { Ok(1) } else { $crate::os_store::MemberOpenOperation::next_depth_demand(open.as_ref()) },)+ Self::ReturningSource(_) => Ok(1),Self::Closed => Ok(0) }
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self, Self::Closed)
            }
            fn terminal_drop_byte_demand(&self) -> Option<usize> {
                matches!(self, Self::Closed).then_some(0)
            }
        }

        impl $crate::os_store::SpaceMember for $enum_name {
            fn local_actor_id(&self) -> &$crate::os_spr::ActorId {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::local_actor_id(m.as_ref())),+ }
            }
            async fn document_id(&self) -> &str {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::document_id(m.as_ref()).await),+ }
            }
            fn artifact_ref(&self) -> Option<::semio_framework_artifact_reference::ArtifactRef> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::artifact_ref(m.as_ref())),+ }
            }
            fn owner_ref(&self) -> Option<$crate::os_store::OwnerRef> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::owner_ref(m.as_ref())),+ }
            }
            fn child_restore_projection(&self) -> Result<$crate::semio_framework_composition_admission::ChildRestoreProjection<'_>, $crate::semio_framework_composition_admission::ChildRestoreProjectionError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::child_restore_projection(m.as_ref())),+ }
            }
            fn child_restore_projection_for_read<'read>(&self,read:&'read $crate::os_store::snapshot_read::ErasedSnapshotRead)->Result<$crate::semio_framework_composition_admission::ChildRestoreProjection<'read>,$crate::semio_framework_composition_admission::ChildRestoreProjectionError>{
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::child_restore_projection_for_read(m.as_ref(),read)),+ }
            }
            fn one_item_publication_identity(&self) -> (u64, [u8; 32]) {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::one_item_publication_identity(m.as_ref())),+ }
            }
            fn one_item_wire_publication_supported(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::one_item_wire_publication_supported(m.as_ref())),+ }
            }
            fn begin_one_item_wire_publication(&self, request: $crate::os_store::one_item_publication::MemberStoreOneItemWireRequest) -> Result<Box<dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication>, $crate::os_store::one_item_publication::ArtifactStoreBatchAdmissionRejected<$crate::os_store::one_item_publication::MemberStoreOneItemWire>> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::begin_one_item_wire_publication(m.as_ref(), request)),+ }
            }
            fn owned_publication_birth_bytes(&self, request: &$crate::os_store::MemberStoreOwnedBatchRequest) -> Result<usize, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::owned_publication_birth_bytes(m.as_ref(), request)),+ }
            }
            fn begin_one_item_owned_publication(&self, request: &mut $crate::os_store::MemberStoreOwnedBatchRequest, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<Option<Box<dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication>>, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::begin_one_item_owned_publication(m.as_ref(), request, grant)),+ }
            }
            fn advance_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::one_item_publication::ArtifactStoreOneItemAdvance, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::advance_one_item_publication(m.as_mut(), publication, grant)),+ }
            }
            fn prepare_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::prepare_one_item_publication(m.as_mut(), publication, grant)),+ }
            }
            fn stage_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::stage_one_item_publication(m.as_mut(), publication, visibility, grant)),+ }
            }
            fn adopt_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::adopt_one_item_publication(m.as_mut(), publication, visibility, grant)),+ }
            }
            fn snapshot_read_erased_for_publication(&self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], $crate::os_store::snapshot_read::ErasedSnapshotRead, ::semio_framework_value::retained_clone::RetainedCloneProgress)>, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_erased_for_publication(m.as_ref(), publication, visibility, grant)),+ }
            }
            fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, snapshot: $crate::os_store::snapshot_read::ErasedSnapshotRead, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::RetainedCloneStep, (::semio_framework_value::ValueError, $crate::os_store::snapshot_read::ErasedSnapshotRead)> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::return_prepared_snapshot_read_erased(m.as_ref(), publication, visibility, snapshot, grant)),+ }
            }
            fn abort_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<::semio_framework_value::retained_clone::RetainedCloneStep, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::abort_one_item_publication(m.as_mut(), publication, grant)),+ }
            }
            fn snapshot_read_erased_now(&self) -> Result<$crate::os_store::snapshot_read::ErasedSnapshotRead, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_erased_now(m.as_ref())),+ }
            }
            async fn snapshot_read_erased(&self) -> Result<$crate::os_store::snapshot_read::ErasedSnapshotRead, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_erased(m.as_ref()).await),+ }
            }
            fn retire_snapshot_read_erased(&mut self, snapshot: &mut Option<$crate::os_store::snapshot_read::ErasedSnapshotRead>, grant: $crate::os_store::RetainedCloneGrant) -> Result<(Option<Box<dyn $crate::os_store::ErasedSnapshotRetirement>>, $crate::os_store::RetainedCloneProgress), ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::retire_snapshot_read_erased(m.as_mut(), snapshot, grant)),+ }
            }
            fn take_returned_snapshot_read_retirement(&mut self, grant: $crate::os_store::RetainedCloneGrant) -> Result<(Option<Box<dyn $crate::os_store::ErasedSnapshotRetirement>>, $crate::os_store::RetainedCloneProgress), ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::take_returned_snapshot_read_retirement(m.as_mut(), grant)),+ }
            }
            fn snapshot_read_retirement_demand(&self) -> ::semio_framework_value::RetirementDemand {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_retirement_demand(m.as_ref())),+ }
            }
            fn returned_snapshot_read_retirement_demand(&self) -> Result<::semio_framework_value::RetirementDemand, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::returned_snapshot_read_retirement_demand(m.as_ref())),+ }
            }
            fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_leases_terminal_is_empty(m.as_ref())),+ }
            }
            fn close_owned_step(&mut self, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<::semio_framework_value::retained_clone::RetainedCloneStep, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::close_owned_step(m.as_mut(), grant)),+ }
            }
            fn close_owned_terminal_is_empty(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::close_owned_terminal_is_empty(m.as_ref())),+ }
            }
            fn close_owned_demands(&self, maximum_body_bytes: usize) -> Result<::semio_framework_value::RetirementDemand, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::close_owned_demands(m.as_ref(), maximum_body_bytes)),+ }
            }
            fn content_revision_now(&self) -> [u8; 32] {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::content_revision_now(m.as_ref())),+ }
            }
            async fn content_revision(&self) -> [u8; 32] {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::content_revision(m.as_ref()).await),+ }
            }
            async fn is_dirty(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::is_dirty(m.as_ref()).await),+ }
            }
            async fn commit_checkpoint(&mut self, message: String, authors: Vec<$crate::os_store::Author>, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::commit_checkpoint(m.as_mut(), message, authors, identity).await),+ }
            }
            async fn current_checkpoint_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::current_checkpoint_id(m.as_ref()).await),+ }
            }
            async fn current_alternative_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::current_alternative_id(m.as_ref()).await),+ }
            }
            async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::checkout(m.as_mut(), checkpoint_id, alternative_id, identity).await),+ }
            }
            async fn create_alternative(&mut self, name: String, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::create_alternative(m.as_mut(), name, identity).await),+ }
            }
            async fn last_local_edit_timestamp(&self) -> Option<$crate::os_spr::HybridLogicalTimestamp> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::last_local_edit_timestamp(m.as_ref()).await),+ }
            }
            async fn last_undone_local_edit_timestamp(&self) -> Option<$crate::os_spr::HybridLogicalTimestamp> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::last_undone_local_edit_timestamp(m.as_ref()).await),+ }
            }
            async fn undo(&mut self, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::undo(m.as_mut(), identity).await),+ }
            }
            async fn redo(&mut self, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::redo(m.as_mut(), identity).await),+ }
            }
            async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::as_any_mut(m.as_mut()).await),+ }
            }
            async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<$crate::os_spr::MutationMessage> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::preview_wire(m.as_ref(), ops).await),+ }
            }
            async fn dispatch_wire(&mut self, cmd_bytes: &[u8], identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<$crate::os_store::CommandReceipt, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::dispatch_wire(m.as_mut(), cmd_bytes, identity).await),+ }
            }
            async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: $crate::os_spr::MergePolicy, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<$crate::os_store::CommandReceipt, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::dispatch_wire_with_policy(m.as_mut(), cmd_bytes, policy, identity).await),+ }
            }
            async fn event_log_payload(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::event_log_payload(m.as_ref()).await),+ }
            }
            async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::announce_tail_edit_payload(m.as_mut()).await),+ }
            }
            async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::ingest_remote_payload(m.as_mut(), envelopes).await),+ }
            }
            async fn tail_group_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::tail_group_id(m.as_ref()).await),+ }
            }
            async fn tail_edit_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::tail_edit_id(m.as_ref()).await),+ }
            }
            async fn redo_tail(&self) -> Option<(String, Option<String>)> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::redo_tail(m.as_ref()).await),+ }
            }
            async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::stamp_tail_group_id(m.as_mut(), group_id).await),+ }
            }
            async fn stamp_tail_origin(&mut self, origin: $crate::os_spr::MutationOrigin) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::stamp_tail_origin(m.as_mut(), origin).await),+ }
            }
            async fn set_owner(&mut self, owner: Option<$crate::os_store::OwnerRef>) {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::set_owner(m.as_mut(), owner).await),+ }
            }
            async fn document_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::document_pack_bytes(m.as_ref()).await),+ }
            }
            async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::envelope_pack_bytes(m.as_ref()).await),+ }
            }
            async fn merge_persisted_envelope(&mut self, envelope_pack: &[u8]) -> Result<$crate::os_store::replay::PairMerge, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::merge_persisted_envelope(m.as_mut(), envelope_pack).await),+ }
            }
            async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::pack_at_checkpoint(m.as_ref(), checkpoint_id).await),+ }
            }
            async fn merge_policy(&self) -> $crate::os_spr::MergePolicy {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::merge_policy(m.as_ref()).await),+ }
            }
        }

        impl $crate::os_store::MemberFactory for $enum_name {
            const OPEN_DECLARATIONS: &'static [$crate::os_store::MemberOpenDeclaration] = &[
                $($crate::os_store::MemberOpenDeclaration { kind: $kind, standard: $standard, subset: $subset, schema: $schema,history_limits:$history_limits,maximum_history_allocation_bytes:$history_allocation_bytes }),+
            ];

            type Open = $open_name;

            fn open_birth_demand(request: &$crate::os_store::MemberOpenRequest) -> Result<$crate::os_store::RetainedCloneBirthDemand, $crate::os_store::MemberOpenDiagnostic> {
                let dialect = &request.admitted_expected()?.dialect;
                match (dialect.artifact_kind.as_str(), dialect.standard.as_str(), dialect.subset.as_str()) {
                    $(($kind, $standard, $subset) => {
                        let child = $crate::os_store::InitialMemberStoreOpen::<Self, $snapshot, $mutation,$catalog>::begin_birth_demand(request,&$catalog_value)?;
                        let capacity_bytes = child.capacity_bytes.checked_add(::std::mem::size_of::<$crate::os_store::InitialMemberStoreOpen<Self, $snapshot, $mutation,$catalog>>()).ok_or($crate::os_store::MemberOpenDiagnostic::Capacity)?;
                        let depth = child.depth.checked_add(1).ok_or($crate::os_store::MemberOpenDiagnostic::Capacity)?;
                        Ok($crate::os_store::RetainedCloneBirthDemand { capacity_bytes, depth })
                    },)+
                    _ => Err($crate::os_store::MemberOpenDiagnostic::Identity),
                }
            }

            fn member_frame_bytes(&self) -> usize {
                match self { $(Self::$variant(member) => ::std::mem::size_of_val(member.as_ref())),+ }
            }

            fn begin_open(admission:&mut $crate::os_store::MemberOpenSourceAdmission,opened:&mut Option<Self::Open>,cx:&mut ::semio_framework_job::StepContext<'_>,parent:&::semio_framework_job::OriginalCallerSourceParent)->Result<bool,::semio_framework_job::OriginalTurnFailure>{
                if opened.is_some(){return Err($crate::os_store::member_open::member_admission_diagnostic($crate::os_store::MemberOpenDiagnostic::Stale).into())}
                if !admission.admit_source(parent,cx)?||cx.should_yield(){return Ok(false)}
                let retained=admission.request().ok_or_else(||$crate::os_store::member_open::member_admission_diagnostic($crate::os_store::MemberOpenDiagnostic::Stale))?;
                let required=Self::open_birth_demand(retained).map_err($crate::os_store::member_open::member_admission_diagnostic)?;
                let grant=cx.retained_grant();
                if grant.maximum_items==0||grant.maximum_depth<required.depth||grant.maximum_capacity_bytes<required.capacity_bytes{return Ok(false)}
                let dialect=&retained.admitted_expected().map_err($crate::os_store::member_open::member_admission_diagnostic)?.dialect;
                match(dialect.artifact_kind.as_str(),dialect.standard.as_str(),dialect.subset.as_str()){
                    $(($kind,$standard,$subset)=>{let(open,mut progress)=$crate::os_store::InitialMemberStoreOpen::begin(admission,cx,::semio_framework_value::retained_clone::RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes-::std::mem::size_of::<$crate::os_store::InitialMemberStoreOpen<Self,$snapshot,$mutation,$catalog>>(),maximum_depth:grant.maximum_depth-1,..grant},$catalog_value)?;progress.retained_capacity_bytes+=::std::mem::size_of_val(&open);*opened=Some($open_name::$variant(Box::new(open)));$crate::os_store::member_open::receive_member_close(cx,Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)))?;Ok(true)},)+
                    _=>Err($crate::os_store::member_open::member_admission_diagnostic($crate::os_store::MemberOpenDiagnostic::Identity).into()),
                }
            }

            async fn create(id: &str, dialect: &::semio_framework_artifact_reference::ArtifactDialect, initial_pack: &[u8], actor: $crate::os_spr::ActorId) -> Result<Self, $crate::os_store::VcsError> {
                match (dialect.artifact_kind.as_str(), dialect.standard.as_str(), dialect.subset.as_str()) {
                    $(($kind, $standard, $subset) => Ok(Self::$variant(Box::new($crate::os_store::create_member_store($schema, id, dialect, initial_pack, actor, $native_producer,&$catalog_value).await?))),)+
                    _ => Err($crate::os_store::VcsError::ValidationFailed(format!("no member dialect '{}' registered in {}", ::semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText::to_coordinate(dialect), stringify!($enum_name)))),
                }
            }
            async fn open(expected: &::semio_framework_artifact_reference::ArtifactRef, owner: Option<&$crate::os_store::OwnerRef>, envelope_pack: &[u8], actor: $crate::os_spr::ActorId) -> Result<Self, $crate::os_store::VcsError> {
                match (expected.dialect.artifact_kind.as_str(), expected.dialect.standard.as_str(), expected.dialect.subset.as_str()) {
                    $(($kind, $standard, $subset) => Ok(Self::$variant(Box::new($crate::os_store::open_member_store($schema, expected, owner, envelope_pack, actor, $native_producer,&$catalog_value).await?))),)+
                    _ => Err($crate::os_store::VcsError::ValidationFailed(format!("no member dialect '{}' registered in {}", ::semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText::to_coordinate(&expected.dialect), stringify!($enum_name)))),
                }
            }
        }

        impl $crate::os_store::MemberVisit for $enum_name {
            fn visit_member<V: $crate::os_store::MemberStoreVisitor>(&self, visitor: V) -> V::Output {
                match self { $(Self::$variant(m) => visitor.visit(m.as_ref())),+ }
            }
            async fn visit_member_mut<V: $crate::os_store::MemberStoreVisitorMut>(&mut self, visitor: V) -> V::Output {
                match self { $(Self::$variant(m) => visitor.visit_mut(m.as_mut()).await),+ }
            }
        }
    };
}
//#endregion SpaceMember
//#region SpaceHistoryDocument
pub use semio_framework_os_store_space_schema::space_history_document;
use space_history_document::{S_SPACE_HISTORY_SCHEMA, SpaceAlternative, SpaceCheckpoint, SpaceHistoryDiff, SpaceHistorySnapshot, SpaceHistoryStep, SpaceMemberPin};

pub use semio_framework_os_store_space_history as space_history;
pub use semio_framework_os_store_space::space_history_registration;

use semio_framework_os_store_space_schema::space_history_mutations;
pub use space_history_mutations::{CommitSpaceCheckpoint, CreateSpaceAlternative, RemoveSpaceAlternative, RemoveSpaceCheckpoint, SetActiveSpaceAlternative, SpaceHistoryMutation, SwitchSpaceAlternative};

//#endregion SpaceHistoryDocument
//#region SpaceHost
//#endregion SpaceHost
//#endregion 🔖️Space
//#region 🔖️CompositionCoordinator
pub use semio_framework_os_store_composition as composition_coordinator;
use semio_framework_os_store_composition::{ChildDispatch, ChildGenesis, CompositionCoordinator, CompositionGraph, GroupMeta, GroupReceipt, GroupUndoReport, MemberRelation, OwnsAdmission, TransactionCoordinator};
use semio_framework_os_store_composition::{build_apply_command_bytes, fold_compensation_error, prefix_message_target, reject_if_policy_rejects};

use semio_framework_os_store_composition::composition_group;
pub use semio_framework_os_store_composition::composition_group::{GroupOwnsError, GroupOwnsPreparation, GroupOwnsStep};
pub use semio_framework_os_store_engine_owners::lifted_store::{ArtifactStoreCloseHost, ArtifactStoreCloseView, ArtifactStoreCursorDisposer, ArtifactStoreOwnedDisposer};
pub use semio_framework_os_store_space_member::lifted_store::{ArtifactRefs, BlobStore, LinkResolver, LinkState, MemberDirectory, MemberFactory, MemberLinkResolver, MemberStoreVisitor, MemberStoreVisitorMut, MemberVisit, NoBlobStore, NoMembers, SpaceMember, content_id};
pub use semio_framework_os_store_space::lifted_store::{SpaceHost, create_member_store, install_funded_member_store_owners, install_unscheduled_catalog, open_member_store, space_history_store_owners};
pub(crate) use semio_framework_os_store_space::lifted_store::{validate_member_history_identity};
pub use semio_framework_os_store_engine_core::lifted_store::{ArtifactEnvelopeView, CommandReceipt, HistoryColumn, StoreCursor, StoreLog, StoreMintToken, StoreOwners, StoreWire, build_history_columns};
pub use semio_framework_os_store_engine::lifted_store::{ArtifactStore, ConfigStore, DraftStore, HistoryFinalization, config_store_from_initialized_runtime_with_owners, merge_base, reconcile_alternative, retract_history_transitions_from_spr};
pub use semio_framework_os_store_codec_factory::lifted_store::{ARTIFACT_CODEC_APPLY_CLOSE_MAXIMUM_STEPS, artifact_codec, bare_artifact_codec, replay_envelopes_onto_pair};
pub(crate) use semio_framework_os_store_codec_factory::lifted_store::{close_codec_reduction_store};
pub(crate) use semio_framework_os_store_engine::lifted_store::{SupersedeScope, checkpoint_ancestors, checkpoint_identity, merge_by_id, preflight_merge_by_id};
pub use semio_framework_os_store_config::lifted_store::{ConfigEnvelope, ConfigRecord, create_config_envelope};
pub(crate) use semio_framework_os_store_engine_core::lifted_store::{MIRROR_ORACLE_EVERY_HISTORY, MIRROR_ORACLE_HISTORY, MIRROR_ORACLE_STRIDE, ScratchOwner, assign_history_checkpoint_lanes, branched_alternatives, checkpoint_alternatives, fold_history, has_main_only_descendant, is_checkpoint_main_only};
pub use semio_framework_os_store_initialization_runtime::lifted_store::{ArtifactStoreInitializationDigest, ArtifactStoreInitializationEditAdmission, ArtifactStoreInitializationEditIndex, ArtifactStoreInitializationForward, ArtifactStoreInitializationOwnerCatalog, ArtifactStoreInitializationRuntime, artifact_initial_digest, artifact_initial_digest_of_pack, bound_edit_messages};
pub use semio_framework_os_store_engine_live::lifted_store::{AppliedMutation, StoreLive, StoreParts, StorePartsRef, StoreReplay, transition_retraction_closure};
pub(crate) use semio_framework_os_store_engine_live::lifted_store::{ArtifactStoreDocumentRootCommitAuthority, document_backbone_batches, edit_actor_from_meta};
pub(crate) use semio_framework_os_store_test_support::lifted_store::{STORE_SCHEMA_EXPORTS};
pub use semio_framework_os_store_test_support::lifted_store::{register_store_schema_exports};

use semio_framework_os_store_composition::composition_links;

//#endregion 🔖️CompositionCoordinator
//#region 🔖️TestSupport
pub use semio_framework_os_store_test_support as test_support;
//#endregion 🧪️Tests
//#endregion 🧪️Tests

//#region 🔖️InteractionStatePack
//#endregion 🔖️TestSupport

//#region 🧪️Tests


//#endregion 🔖️InteractionStatePack
//#region 🔖️SchemaExports


//#endregion 🔖️SchemaExports


                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    


// ♻️ `HistoryColumn` belongs to the engine core, so its retirement impl and the command custody test live beside it until the engine crates land.
semio_framework_value::artifact_retire_struct!(HistoryColumn { checkpoint_id, timestamp, labels, authors, parent_checkpoint_id, description, lane, alternative_ids });


