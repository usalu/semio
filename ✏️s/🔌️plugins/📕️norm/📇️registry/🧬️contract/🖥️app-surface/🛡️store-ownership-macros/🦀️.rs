//! 🛡️ Exported store ownership macros of the norm app surface: the exact bounded store owners and the read-only viewer twin every norm editor installs. See [`super`].


/// 🧹️ Installs the exact bounded store owners and disposers shared by every Norm editor, and the shared cold replay issuer that lets its history preview, replay and archive load report.
#[macro_export]
macro_rules! norm_exact_store_ownership {
    (live) => {
        $crate::norm_exact_store_ownership!(live, store::os_store::ColdMutationApplyFactory<Self::Snapshot, Self::Mutation>);
    };
    (live, $factory:ty) => {
        fn document_store_owners_source_demands() -> Result<semio_plugin::__value::RetirementDemand, semio_plugin::__value::ValueError> {
            let children = semio_plugin::__value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 };
            store::os_store::document_store_owners::bounded_artifact_store_owners_shared_semantic_source_demands::<Self::Snapshot, Self::Mutation, $factory>(children, 0)
        }

        fn build_document_store_owners(grant: semio_plugin::__value::retained_clone::RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, semio_plugin::__value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
            let children = semio_plugin::__value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 };
            Some(store::os_store::document_store_owners::bounded_artifact_store_owners_with_shared_semantic_preparation::<Self::Snapshot, Self::Mutation, $factory>(grant, children, 0, <$factory>::default).map(|(mut owners, progress)| {
                owners.operation_wire = store::os_store::ArtifactOperationWire::semantic::<Self::Snapshot>().with_batch_digest().with_original_encoded_binary_receiving().with_original_target(store::os_store::whole_document_target::<Self::Mutation>);
                (owners, progress)
            }))
        }

        fn build_document_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            Some(semio_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            Some(semio_plugin::no_config_store_disposer())
        }

        fn build_draft_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
            Some(semio_plugin::no_draft_store_disposer())
        }

        fn build_presence_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(semio_plugin::bounded_presence_store_disposer::<Self::Presence, Self::PresenceMutation>())
        }

        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_plugin::bounded_presence_root_retirement_factory::<Self::Presence>())
        }

        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_plugin::bounded_presence_root_retirement_factory::<Self::Presence>())
        }

        fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
            Some(semio_plugin::bounded_transient_preparation_factory::<Self::Presence, Self::PresenceMutation>())
        }

        fn build_transient_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
        }

        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
            Some(semio_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
        }

        fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
            Some(semio_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
        }
    };
    () => {

        fn document_store_owners_source_demands() -> Result<semio_plugin::__value::RetirementDemand, semio_plugin::__value::ValueError> {
            semio_plugin::cold_replay_document_store_owners_source_demands::<Self::Snapshot, Self::Mutation>()
        }

        fn build_document_store_owners(grant: semio_plugin::__value::retained_clone::RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, semio_plugin::__value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
            Some(semio_plugin::cold_replay_document_store_owners::<Self::Snapshot, Self::Mutation>(grant).map(|(mut owners, progress)| {
                owners.operation_wire = store::os_store::ArtifactOperationWire::semantic::<Self::Snapshot>().with_batch_digest().with_original_encoded_binary_receiving().with_original_target(store::os_store::whole_document_target::<Self::Mutation>);
                (owners, progress)
            }))
        }

        fn build_document_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            Some(semio_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            Some(semio_plugin::no_config_store_disposer())
        }

        fn build_draft_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
            Some(semio_plugin::no_draft_store_disposer())
        }

        fn build_presence_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(semio_plugin::no_presence_store_disposer())
        }

        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_plugin::no_presence_local_root_retirement_factory())
        }

        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_plugin::no_presence_peer_retirement_factory())
        }

        fn build_transient_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_plugin::no_transient_store_disposer())
        }

        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
            Some(semio_plugin::no_transient_local_root_retirement_factory())
        }
    };
}

/// 🧹️ The read-only twin of [`norm_exact_store_ownership`]; the `(lanes)` arm keeps the ephemeral local view lane (transient store) of a stepping viewer. `ArtifactViewer` defaults every store
/// owner and disposer to `None` ("absent authority fails closed"), so a viewer that omits them
/// cannot be closed — `close_registered_fixture_app` refuses it with
/// `interactive-job.close-owned-disposer-missing`. The viewer trait carries no draft lane and no
/// local-root retirement factories, so this is the editor macro's method set minus those.
#[macro_export]
macro_rules! norm_exact_viewer_store_ownership {
    (lanes) => {

        fn build_document_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            Some(semio_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            Some(semio_plugin::no_config_store_disposer())
        }

        fn build_presence_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(semio_plugin::no_presence_store_disposer())
        }

        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_plugin::no_presence_peer_retirement_factory())
        }

        fn build_transient_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
        }

        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
            Some(semio_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
        }

        fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
            Some(semio_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
        }
    };
    () => {

        fn build_document_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            Some(semio_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            Some(semio_plugin::no_config_store_disposer())
        }

        fn build_presence_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(semio_plugin::no_presence_store_disposer())
        }

        fn build_transient_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_plugin::no_transient_store_disposer())
        }

        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_plugin::no_presence_peer_retirement_factory())
        }
    };
}
