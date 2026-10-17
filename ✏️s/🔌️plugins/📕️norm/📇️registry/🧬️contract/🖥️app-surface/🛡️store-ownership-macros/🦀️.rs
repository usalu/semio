//! 🛡️ Exported store ownership macros of the norm app surface: the exact bounded store owners and the read-only viewer twin every norm editor installs. See [`super`].


/// 🧹️ Installs the exact bounded store owners and disposers shared by every Norm editor.
#[macro_export]
macro_rules! norm_exact_store_ownership {
    () => {

        fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            Some(semio_framework_plugin::no_config_store_disposer())
        }

        fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
            Some(semio_framework_plugin::no_draft_store_disposer())
        }

        fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(semio_framework_plugin::no_presence_store_disposer())
        }

        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
        }

        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin::no_presence_peer_retirement_factory())
        }

        fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_framework_plugin::no_transient_store_disposer())
        }

        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
            Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
        }
    };
}

/// 🧹️ The read-only twin of [`norm_exact_store_ownership`]. `ArtifactViewer` defaults every store
/// owner and disposer to `None` ("absent authority fails closed"), so a viewer that omits them
/// cannot be closed — `close_registered_fixture_app` refuses it with
/// `interactive-job.close-owned-disposer-missing`. The viewer trait carries no draft lane and no
/// local-root retirement factories, so this is the editor macro's method set minus those.
#[macro_export]
macro_rules! norm_exact_viewer_store_ownership {
    () => {

        fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            Some(semio_framework_plugin::no_config_store_disposer())
        }

        fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(semio_framework_plugin::no_presence_store_disposer())
        }

        fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_framework_plugin::no_transient_store_disposer())
        }

        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin::no_presence_peer_retirement_factory())
        }
    };
}
