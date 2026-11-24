//! 🌱️ Process3d native document genesis: the authored empty document, its canonical Pack source and the shared semantic Store catalog.

use super::*;
use semio_framework_artifact_reference::Dialect;
use semio_framework_dsl_record::{native_encoding::FieldProjectionSource, BorrowedDslRecord, BorrowedRecordSpec};
use semio_framework_value::{retained_clone::RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_plugin_ownership::original_store_genesis::{FixedStoreInitial, OriginalFixedStoreInitial, OriginalStoreGenesisOwner};
use semio_s_artifact_process_process3d_model::snapshot::{empty_process3d_snapshot, PROCESS3D_DIALECT, PROCESS_3D_SCHEMA};
use std::mem::size_of;
use store::os_store::document_store_owners::{BoundedArtifactRetirementFactory, DocumentStoreOwnerProducer, DocumentStoreOwners, DocumentStoreOwnersAdmissionError};

type Owners = DocumentStoreOwners<Process3dSnapshot, Process3dMutation>;
type SnapshotRetirement = BoundedArtifactRetirementFactory<Process3dSnapshot>;
type MutationRetirement = BoundedArtifactRetirementFactory<Process3dMutation>;
type Disposer = store::os_store::ArtifactStoreCursorDisposer<Process3dSnapshot, Process3dMutation>;
type Preparation = store::ColdMutationApplyFactory<Process3dSnapshot, Process3dMutation>;

const NO_CHILDREN: RetainedCloneBirthDemand = RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 };

/// 🏭️ Publication and replay share one cold semantic issuer over the Process3d mutation vocabulary.
pub struct Process3dDocumentCatalog;

impl DocumentStoreOwnerProducer<Process3dSnapshot, Process3dMutation> for Process3dDocumentCatalog {
    fn source_demands() -> Result<RetirementDemand, ValueError> {
        Owners::shared_semantic_source_demands::<SnapshotRetirement, SnapshotRetirement, MutationRetirement, Disposer, Preparation>(NO_CHILDREN, 0)
    }

    fn admit_sources(grant: RetainedCloneGrant) -> Result<(Owners, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<Process3dSnapshot, Process3dMutation>> {
        Owners::admit_source_constructor_with_shared_semantic_preparation(grant, NO_CHILDREN, 0, || (SnapshotRetirement::new(), SnapshotRetirement::new(), MutationRetirement::new(), Disposer::new(), Preparation::default())).map(|(mut owners, progress)| {
            owners.operation_wire = semio_os_store_operation_wire::original_edit::with_original_edit_message(store::ArtifactOperationWire::canonical().with_original_receiving().with_original_target(store::whole_document_target));
            (owners.with_batch_digest(), progress)
        })
    }
}

/// 🧬️ The authored empty Process3d document: default workshop and stock, no steps, minted composed children.
pub struct Process3dDocumentGenesis;

impl OriginalStoreGenesisOwner for Process3dDocumentGenesis {
    type State = Process3dSnapshot;
    type Mutation = Process3dMutation;
    type Catalog = Process3dDocumentCatalog;
    type Initial = FixedStoreInitial<Self>;
    const SCHEMA: &'static str = PROCESS_3D_SCHEMA;
    const MAXIMUM_INITIAL_PACK_BYTES: usize = 262_144;

    fn pack_envelope_id() -> Option<&'static str> {
        Some(<Process3dSnapshot as store::snapshot::ArtifactDsl>::envelope_id())
    }

    fn artifact_dialect() -> Option<Dialect> {
        Some(PROCESS3D_DIALECT)
    }

    fn identity_parts<'a>(label: &'a str) -> [&'a str; 4] {
        [PROCESS3D_DIALECT.artifact_kind, PROCESS3D_DIALECT.standard.0, PROCESS3D_DIALECT.subset.0, label]
    }

    fn initial_state_pack_source(state: &Process3dSnapshot) -> Result<(&dyn FieldProjectionSource, BorrowedRecordSpec), ValueError> {
        Ok((state, <Process3dSnapshot as BorrowedDslRecord>::RECORD))
    }
}

impl OriginalFixedStoreInitial for Process3dDocumentGenesis {
    type State = Process3dSnapshot;

    fn initial_state_demands() -> Result<RetirementDemand, ValueError> {
        static BYTES: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&empty_process3d_snapshot()).unwrap_or(0));
        Ok(RetirementDemand { copy_bytes: 2 * size_of::<Process3dSnapshot>() + *BYTES, capacity_bytes: *BYTES, depth: 1, ..Default::default() })
    }

    fn initial_state(grant: RetainedCloneGrant) -> Result<(Process3dSnapshot, RetainedCloneProgress), ValueError> {
        let demand = Self::initial_state_demands()?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) {
            return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "Process3d authored initial state is unfunded"));
        }
        Ok((empty_process3d_snapshot(), RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() }))
    }
}
