//! 🏭️ Receives bounded local file selections in the Terrain editor’s retained dispatcher.

use super::{geojson, import_elevation};
use crate::editor::gis3d::{Gis3dCommand, Gis3dPlayApp};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory};
use semio_framework_value::DslValue;
use semio_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_plugin::{ArtifactOwnedToolJobFactory, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, Effect, RequestId};

type App = EditorApp<Gis3dPlayApp>;
pub const MAX_FILE_BYTES: u32 = 24 * 1024;
pub(crate) const MAX_WIRE_BYTES: usize = 2 * import_elevation::MAX_FILE_BYTES as usize + 4096;
pub(crate) const FILE_ACTIONS: [&str; 2] = [import_elevation::FILE_ACTION, geojson::FILE_ACTION];

/// 🧬️ Keeps the complete ASCII Grid and its encoded envelope inside the file tool's wire authority.
pub(crate) fn command_wire_maximum(command: &Gis3dCommand) -> usize {
    match command {
        Gis3dCommand::ImportElevationGridFile(_) => MAX_WIRE_BYTES,
        _ => semio_plugin::RUNTIME_COMMAND_WIRE_MAXIMUM,
    }
}

pub(crate) fn open(id: u64, accept: &str, import_action: &str, maximum_bytes: u32) -> Effect {
    Effect::RequestFileOpen {
        req: RequestId(id), accept: accept.into(), read_as: Some("text".into()),
        import_action: import_action.into(), args: None, multiple: false, max_file_bytes: Some(maximum_bytes),
    }
}

pub(crate) fn valid_length(input: &str, maximum_bytes: u32) -> bool {
    !input.is_empty() && input.len() <= maximum_bytes as usize
}

/// 📥️ Moves a complete singleton picker callback into a typed retained command.
pub(crate) fn receive(args: &mut Option<DslValue>, maximum_bytes: u32) -> Option<String> {
    let Some(DslValue::Object(fields)) = args else { return None };
    if fields.iter().any(|(name, value)| {
        (name == "chunkCount" && value.as_f64() != Some(1.0))
            || (name == "chunk" && value.as_f64() != Some(0.0))
    }) { return None; }
    let mut payloads = fields.iter_mut().filter(|(name, _)| name == "payload");
    let (_, DslValue::String(payload)) = payloads.next()? else { return None };
    (payloads.next().is_none() && valid_length(payload, maximum_bytes)).then(|| std::mem::take(payload))
}

pub(crate) struct LocalFileImportJobFactory {
    keys: [ToolFactoryKey; FILE_ACTIONS.len()],
}

impl LocalFileImportJobFactory {
    pub(crate) fn new(controller: &str) -> Self {
        Self { keys: FILE_ACTIONS.map(|action| ToolFactoryKey::new(controller, action)) }
    }

    pub(crate) fn proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
        Self::bounded_first_step_tool_proofs()
    }

    semio_plugin::bounded_first_step_tool_proofs! {
        owner: App,
        owner_file: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.gis.gisterrain@1/*#editor",
        artifact_schema: "gis.terrain",
        factory: "LocalFileImportJobFactory",
        factory_type: LocalFileImportJobFactory,
        contract: ToolExecutionContract::resumable(MAX_WIRE_BYTES, 32, 4096, 128 * 1024, 7_500, 64, 1),
        tools: ["importElevationGridFile", "importGeoJsonFile"]
    }
}

impl ToolJobFactory for LocalFileImportJobFactory {
    type Payload = ArtifactRetainedCommandPayload<App>;
    type Job = ArtifactRetainedCommandJob<App>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        "gis.terrain.local-file.tool-command.v1"
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(MAX_WIRE_BYTES, 32, 4096, 128 * 1024, 7_500, 64, 1)
    }

    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: MAX_WIRE_BYTES, checkpoint_resume: false);
}

impl ArtifactOwnedToolJobFactory for LocalFileImportJobFactory {
    type Owner = App;

    const TOOL_IDS: &'static [&'static str] = &FILE_ACTIONS;
    const DOCUMENT_SCHEMA: &'static str = crate::GIS_3D_TERRAIN_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: import_elevation::FILE_ACTION, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: geojson::FILE_ACTION, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ];
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
