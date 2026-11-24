//! 🌱️ Process3d viewer native declarations: the shared document genesis, empty config and draft lanes and the original registrar.

use crate::Process3dViewer;
use semio_plugin::original_ephemeral::OriginalEmptyEphemeralDeclaration;
use semio_plugin::original_instance_registration::{OriginalInstanceDeclarations, OriginalNativeRegistration, OriginalNoWindows};
use semio_plugin::{ArtifactViewer, ViewerApp, WindowConfigOwnerRegistry, WindowTransientOwnerRegistry};
use semio_s_artifact_process_process3d_host::Process3dDocumentGenesis;

/// 👁️ A read-only viewer persists no presence, transient or window configuration.
pub struct Process3dViewerDeclarations;

impl OriginalInstanceDeclarations<ViewerApp<Process3dViewer>> for Process3dViewerDeclarations {
    type Ephemeral = OriginalEmptyEphemeralDeclaration;
    type ConfigWindows = OriginalNoWindows<WindowConfigOwnerRegistry>;
    type TransientWindows = OriginalNoWindows<WindowTransientOwnerRegistry>;
}

/// 🎟️ The native builder selects this original registrar with the unchanged mounted owner policy.
pub type Process3dViewerNativeRegistration = OriginalNativeRegistration<ViewerApp<Process3dViewer>, <Process3dViewer as ArtifactViewer>::Members, Process3dViewerDeclarations>;

impl semio_plugin::OriginalArtifactViewerBirth for Process3dViewer {
    type DocumentGenesis = Process3dDocumentGenesis;
    type ConfigGenesis = semio_plugin::NoConfigGenesisOwner;
    type DraftGenesis = semio_plugin::NoDraftGenesisOwner;
}
