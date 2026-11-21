//! 🏭️ PDF 1.7 Editor's exact owned registration for the normal native binary output.

use semio_plugin::{
    ActionDefinition, AppIo, ArtifactOwnedToolJobFactory, ArtifactPresentation, ArtifactReservedToolJob,
    ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, LocalizedLabel,
    InteractiveJobClassification, MediaPortDirection, MediaPortSpec, PortMultiplicity, ToolExecutionContract, ArtifactOutputChunks,
};
use semio_plugin::plugin_app_close_prelude::{EXPORT_MEDIA_FILE_ACTION_ID, media_file_export_action_definition};
use semio_plugin::retained_command::__action_bus::{ToolFactoryKey, ToolJobFactory};
use crate::editor::pdf17::Pdf17Editor;
use crate::standards::v1_7::subsets::base::schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA;
use super::producer::{FORMAT, MEDIA_TYPE, MIME, PORT, TOOL};
const PAYLOAD_SCHEMA: &str = "stdio.pdf.1.7.native-export.v1";
pub const CONTRACT: ToolExecutionContract = ToolExecutionContract::resumable(
    4_096, 4_096, 4_096, ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES, 2_000, 64, 1,
);

/// 📄️ Declares a native PDF document port to the revision-bound media download shell action.
pub fn app_io() -> AppIo {
    AppIo {
        artifact_schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MEDIA_TYPE,
        ports: vec![MediaPortSpec {
            id: PORT.into(),
            label: LocalizedLabel::native("PDF document", "PDF-Dokument"),
            direction: MediaPortDirection::Out,
            media_type: MEDIA_TYPE,
            kind_id: Some(FORMAT.into()),
            required: false,
            multiplicity: PortMultiplicity::Many,
        }],
        export_formats: vec![FORMAT.into()],
        import_formats: vec![FORMAT.into()],
        artifact: ArtifactPresentation {
            id: "stdio.pdf".into(),
            name: "PDF Document".into(),
            dimension: "page".into(),
            component_kind: "document".into(),
        },
    }
}

/// 🗂️ Keeps the shell's explicit and normal Save routes on the same native representation.
pub fn export_action() -> ActionDefinition {
    media_file_export_action_definition(
        EXPORT_MEDIA_FILE_ACTION_ID, LocalizedLabel::native("Export PDF File", "PDF-Datei exportieren"),
        PORT, "document.pdf", MIME, FORMAT,
    )
}

pub struct PdfNativeExportFactory {
    keys: [ToolFactoryKey; 1],
}

impl PdfNativeExportFactory {
    pub fn new(controller: &str) -> Self {
        Self { keys: [ToolFactoryKey::new(controller, TOOL)] }
    }
}

impl ToolJobFactory for PdfNativeExportFactory {
    type Payload = ArtifactReservedToolJob;
    type Job = ArtifactReservedToolJob;

    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { CONTRACT }

    semio_plugin::retained_command::reserved_tool_job_factory_admission!();
}

impl ArtifactOwnedToolJobFactory for PdfNativeExportFactory {
    type Owner = EditorApp<Pdf17Editor>;
    const TOOL_IDS: &'static [&'static str] = &[TOOL];
    const DOCUMENT_SCHEMA: &'static str = STDIO_PDF17_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract {
            tool_id: TOOL,
            lanes: &[ArtifactToolPublicationLane::HostOnly],
        },
    ];
}
