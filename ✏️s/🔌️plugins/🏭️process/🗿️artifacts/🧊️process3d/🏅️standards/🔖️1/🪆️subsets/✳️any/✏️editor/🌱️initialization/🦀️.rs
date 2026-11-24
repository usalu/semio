//! 🌱️ Process3d editor native declarations: the config genesis, the presence and window-transient lanes and the original registrar.

use crate::placement_hover::WorkpiecePlacementOwner;
use crate::Process3dPlayApp;
use semio_framework_dsl_record::{native_encoding::FieldProjectionSource, BorrowedDslRecord, BorrowedRecordSpec};
use semio_framework_value::{retained_clone::RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_plugin::original_ephemeral::{self, FixedStoreInitial as EphemeralFixedStoreInitial, OriginalEphemeralDeclaration, OriginalFixedStoreInitial as OriginalEphemeralFixedStoreInitial, OriginalPresenceOwners, OriginalTransientOwners};
use semio_plugin::original_instance_registration::{OriginalInstanceDeclarations, OriginalNativeRegistration, OriginalNoWindows, OriginalWindowSequence};
use semio_plugin::{ArtifactEditor, EditorApp, OriginalWindowTransientOwnerRegistration, WindowConfigOwnerRegistry, WindowTransientOwnerRegistry};
use semio_plugin_ownership::original_store_genesis::{FixedStoreInitial, OriginalFixedStoreInitial, OriginalStoreGenesisOwner};
use semio_s_artifact_process_process3d_config::{Process3dConfig, Process3dConfigMutation};
use semio_s_artifact_process_process3d_host::Process3dDocumentGenesis;
use semio_s_artifact_process_process3d_presence::{process3d_presence_is_terminal_empty, Process3dPresence};
use std::{mem::size_of, sync::Arc};
use store::os_store::document_store_owners::{DocumentStoreOwnerProducer, DocumentStoreOwners, DocumentStoreOwnersAdmissionError};

type ConfigOwners = DocumentStoreOwners<Process3dConfig, Process3dConfigMutation>;
type ConfigRetirement<T> = store::os_store::document_store_owners::BoundedArtifactRetirementFactory<T>;
type ConfigDisposer = store::os_store::ArtifactStoreCursorDisposer<Process3dConfig, Process3dConfigMutation>;
type ConfigPreparation = store::ColdMutationApplyFactory<Process3dConfig, Process3dConfigMutation>;

const NO_CHILDREN: RetainedCloneBirthDemand = RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 };

/// 🎚️ Config publication and replay share one cold semantic issuer over the config mutation vocabulary.
pub struct Process3dConfigCatalog;

impl DocumentStoreOwnerProducer<Process3dConfig, Process3dConfigMutation> for Process3dConfigCatalog {
    fn source_demands() -> Result<RetirementDemand, ValueError> {
        ConfigOwners::shared_semantic_source_demands::<ConfigRetirement<Process3dConfig>, ConfigRetirement<Process3dConfig>, ConfigRetirement<Process3dConfigMutation>, ConfigDisposer, ConfigPreparation>(NO_CHILDREN, 0)
    }

    fn admit_sources(grant: RetainedCloneGrant) -> Result<(ConfigOwners, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<Process3dConfig, Process3dConfigMutation>> {
        ConfigOwners::admit_source_constructor_with_shared_semantic_preparation(grant, NO_CHILDREN, 0, || (ConfigRetirement::new(), ConfigRetirement::new(), ConfigRetirement::new(), ConfigDisposer::new(), ConfigPreparation::default())).map(|(mut owners, progress)| {
            owners.operation_wire = semio_os_store_operation_wire::original_edit::with_original_edit_message(store::ArtifactOperationWire::canonical().with_original_receiving().with_original_target(store::whole_document_target));
            (owners.with_batch_digest(), progress)
        })
    }
}

/// 🎚️ The authored default view state of one Process3d editor instance.
pub struct Process3dConfigGenesis;

impl OriginalStoreGenesisOwner for Process3dConfigGenesis {
    type State = Process3dConfig;
    type Mutation = Process3dConfigMutation;
    type Catalog = Process3dConfigCatalog;
    type Initial = FixedStoreInitial<Self>;
    const SCHEMA: &'static str = "process3d.config";
    const MAXIMUM_INITIAL_PACK_BYTES: usize = 4096;

    fn pack_envelope_id() -> Option<&'static str> {
        Some(<Process3dConfig as pack::snapshot::ArtifactDsl>::envelope_id())
    }

    fn identity_parts<'a>(label: &'a str) -> [&'a str; 4] {
        ["", "", "", label]
    }

    fn history_shape() -> store::HistoryShape {
        store::HistoryShape::Config
    }

    fn initial_state_pack_source(state: &Process3dConfig) -> Result<(&dyn FieldProjectionSource, BorrowedRecordSpec), ValueError> {
        Ok((state, <Process3dConfig as BorrowedDslRecord>::RECORD))
    }
}

impl OriginalFixedStoreInitial for Process3dConfigGenesis {
    type State = Process3dConfig;

    fn initial_state_demands() -> Result<RetirementDemand, ValueError> {
        let config = Process3dConfig::default();
        let bytes = config.engagement_input.capacity() + config.sun_color.capacity() + config.contributions_json.capacity();
        Ok(RetirementDemand { copy_bytes: 2 * size_of::<Process3dConfig>() + bytes, capacity_bytes: bytes, depth: 1, ..Default::default() })
    }

    fn initial_state(grant: RetainedCloneGrant) -> Result<(Process3dConfig, RetainedCloneProgress), ValueError> {
        let demand = Self::initial_state_demands()?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) {
            return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "Process3d original config default is unfunded"));
        }
        Ok((Process3dConfig::default(), RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() }))
    }
}

/// 👥️ The shared presence root is born from its authored default.
pub struct Process3dPresenceInitial;

impl OriginalEphemeralFixedStoreInitial for Process3dPresenceInitial {
    type State = Process3dPresence;

    fn initial_state_demands() -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { copy_bytes: 2 * size_of::<Process3dPresence>(), depth: 1, ..Default::default() })
    }

    fn initial_state(grant: RetainedCloneGrant) -> Result<(Process3dPresence, RetainedCloneProgress), ValueError> {
        let demand = Self::initial_state_demands()?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Process3d original presence requires its fixed inline source grant"));
        }
        Ok((Process3dPresence::default(), RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }
}

/// 🫧️ Presence is the shared inline root; the local transient lane is genuinely empty.
pub struct Process3dEphemeralDeclaration;

impl OriginalEphemeralDeclaration<EditorApp<Process3dPlayApp>> for Process3dEphemeralDeclaration {
    type PresenceInitial = EphemeralFixedStoreInitial<Process3dPresenceInitial>;
    type TransientInitial = EphemeralFixedStoreInitial<original_ephemeral::EmptyTransientInitial>;

    fn presence_owners_demands() -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand {
            copy_bytes: 2 * size_of::<OriginalPresenceOwners<EditorApp<Process3dPlayApp>>>(),
            capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<semio_framework_value::SharedValueRetirementFactory<Process3dPresence>>() + size_of::<semio_plugin::PresenceStoreOwnedDisposer<Process3dPresence>>(),
            depth: 2,
            ..Default::default()
        })
    }

    fn presence_owners(initial: &Arc<Process3dPresence>) -> OriginalPresenceOwners<EditorApp<Process3dPlayApp>> {
        let factory = semio_plugin::bounded_transient_root_retirement_factory::<Process3dPresence>();
        let disposer = semio_plugin::PresenceStoreOwnedDisposer::new(initial.clone(), process3d_presence_is_terminal_empty).unwrap_or_else(|_| unreachable!("Process3d presence owns only its empty engagement input at birth"));
        OriginalPresenceOwners { local: factory.clone(), peer: factory, disposer: Box::new(disposer), preparation: None }
    }

    fn transient_owners_demands() -> Result<RetirementDemand, ValueError> {
        original_ephemeral::empty_transient_owners_demands::<EditorApp<Process3dPlayApp>>()
    }

    fn transient_owners() -> OriginalTransientOwners<EditorApp<Process3dPlayApp>> {
        original_ephemeral::empty_transient_owners::<EditorApp<Process3dPlayApp>>()
    }
}

/// 🧩️ The workpiece placement hover is the sole window-transient owner; no window persists its own config.
pub struct Process3dInstanceDeclarations;

impl OriginalInstanceDeclarations<EditorApp<Process3dPlayApp>> for Process3dInstanceDeclarations {
    type Ephemeral = Process3dEphemeralDeclaration;
    type ConfigWindows = OriginalNoWindows<WindowConfigOwnerRegistry>;
    type TransientWindows = OriginalWindowSequence<OriginalWindowTransientOwnerRegistration<WorkpiecePlacementOwner>, OriginalNoWindows<WindowTransientOwnerRegistry>>;
}

/// 🎟️ The native builder selects this original registrar with the unchanged mounted owner policy.
pub type Process3dNativeRegistration = OriginalNativeRegistration<EditorApp<Process3dPlayApp>, <Process3dPlayApp as ArtifactEditor>::Members, Process3dInstanceDeclarations>;

impl semio_plugin::OriginalArtifactEditorBirth for Process3dPlayApp {
    type DocumentGenesis = Process3dDocumentGenesis;
    type ConfigGenesis = Process3dConfigGenesis;
    type DraftGenesis = semio_plugin::NoDraftGenesisOwner;
}
