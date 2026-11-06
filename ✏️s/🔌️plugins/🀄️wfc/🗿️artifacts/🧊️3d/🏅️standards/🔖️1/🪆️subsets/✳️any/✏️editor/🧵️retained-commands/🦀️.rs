//! 🧵️ Retained command route of the WFC 3D editor: the bounded publication contracts, the command work and the job factories. See [`super`].

use super::*;

//#region 🧵️Retained
/// 🎨️ The action id the shell's own navbar example picker dispatches — fixed by
/// `SET_ACTIVE_EXAMPLE_ACTION_ID` in `🛠️ShellHelpers`, so it is camelCase where every wfc verb is
/// kebab-case.
pub const WFC_3D_SET_ACTIVE_EXAMPLE: &str = "setActiveExample";

/// 🕹️ The action id the NodeGraph canvas host dispatches every node drag and every completed wire
/// under (`nodeGraphActions.edit`) — likewise fixed by the renderer, likewise camelCase.
pub const WFC_3D_NODE_GRAPH_EDIT: &str = "nodeGraphEdit";

/// 🎥️ The action id the NodeGraph canvas host dispatches its SETTLED camera under — once per camera
/// gesture and once when the canvas first fits the document, never per wheel tick.
pub const WFC_3D_NODE_GRAPH_VIEWPORT: &str = "nodeGraphViewport";

/// 🧵️ Every verb a pane may dispatch. An id absent here carries NO app-owned retained factory, and a
/// bare bounded proof faults with `interactive-job.missing-owned-reducer` — the verb is then dead in
/// the running app however it is classified.
pub const WFC_3D_RETAINED_TOOL_IDS: &[&str] = &[
    "change-seed",
    "create-slot",
    "delete-slot",
    "move-slot",
    "resize-slot",
    "connect-slots",
    "disconnect-slots",
    "pin-slot",
    "unpin-slot",
    "create-tile",
    "delete-tile",
    "change-tile-weight",
    "change-tile-media",
    "create-rule",
    "delete-rule",
    "change-camera",
    "change-active-tile",
    "solve",
    fill::COMMIT_FILL_ACTION_ID,
    WFC_3D_SET_ACTIVE_EXAMPLE,
    WFC_3D_NODE_GRAPH_EDIT,
    WFC_3D_NODE_GRAPH_VIEWPORT,
];

const WFC_3D_RETAINED_PAYLOAD_SCHEMA: &str = "wfc.wfc3d.tool-command.v1";

pub(super) const WFC_3D_RETAINED_RAW_BYTES: usize = 2 * semio_framework_tool_run::TOOL_RUN_TICK_BYTES_MAX + 16_384;

/// 📬️ Two complete solve records and their sparse diff fit within the admitted tick carrier.
pub(super) const WFC_3D_RETAINED_OUTPUT_BYTES: usize = 4 * semio_framework_tool_run::TOOL_RUN_TICK_BYTES_MAX + 16_384;

pub(super) const WFC_3D_RETAINED_WORK_ITEMS: usize = 4_096;

/// 📄️ One document-lane publication row.
const fn artifact_route(tool_id: &'static str) -> semio_plugin::ArtifactToolPublicationContract {
    semio_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_plugin::ArtifactToolPublicationLane::Artifact] }
}

/// 🧮️ One pane-config publication row — camera and armed tile are view state, never document state.
const fn config_route(tool_id: &'static str) -> semio_plugin::ArtifactToolPublicationContract {
    semio_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_plugin::ArtifactToolPublicationLane::Config] }
}

/// 🛣️ One lane per route, read off each arm of `command_emit`. The example picker is `HostOnly`: it
/// answers a whole-document `Effect::LoadDocument`, which is not an edit, so re-picking the boot
/// example mints no phantom undo entry.
const WFC_3D_PUBLICATION_CONTRACTS: &[semio_plugin::ArtifactToolPublicationContract] = &[
    artifact_route("change-seed"),
    artifact_route("create-slot"),
    artifact_route("delete-slot"),
    artifact_route("move-slot"),
    artifact_route("resize-slot"),
    artifact_route("connect-slots"),
    artifact_route("disconnect-slots"),
    artifact_route("pin-slot"),
    artifact_route("unpin-slot"),
    artifact_route("create-tile"),
    artifact_route("delete-tile"),
    artifact_route("change-tile-weight"),
    artifact_route("change-tile-media"),
    artifact_route("create-rule"),
    artifact_route("delete-rule"),
    config_route("change-camera"),
    config_route("change-active-tile"),
    semio_plugin::ArtifactToolPublicationContract { tool_id: "solve", lanes: &[semio_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_plugin::ArtifactToolPublicationContract { tool_id: fill::COMMIT_FILL_ACTION_ID, lanes: &[semio_plugin::ArtifactToolPublicationLane::Transient] },
    semio_plugin::ArtifactToolPublicationContract { tool_id: WFC_3D_SET_ACTIVE_EXAMPLE, lanes: &[semio_plugin::ArtifactToolPublicationLane::HostOnly] },
    artifact_route(WFC_3D_NODE_GRAPH_EDIT),
    config_route(WFC_3D_NODE_GRAPH_VIEWPORT),
];

fn wfc3d_retained_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(WFC_3D_RETAINED_RAW_BYTES, WFC_3D_RETAINED_WORK_ITEMS, 1, WFC_3D_RETAINED_OUTPUT_BYTES, 7_500)
}

/// 📏️ One bounded first step per route, admitted only while the WHOLE addressable document plus this
/// edit fit inside `WFC_3D_RETAINED_WORK_ITEMS`.
pub(super) fn wfc3d_retained_extent(command: &Wfc3dEditorCommand, snapshot: &Wfc3dSnapshot) -> Option<usize> {
    if !WFC_3D_RETAINED_TOOL_IDS.contains(&wfc3d_command_id(command)) {
        return None;
    }
    let items = snapshot.slots.len().checked_add(snapshot.edges.len())?.checked_add(snapshot.tiles.len())?.checked_add(snapshot.rules.len())?.checked_add(1)?;
    (items <= WFC_3D_RETAINED_WORK_ITEMS).then_some(1)
}

/// 🧵️ The ONE bounded work step every wfc3d route runs — the pure `command_emit` reducer.
pub(super) struct Wfc3dCommandWork {
    pub(super) tool_id: &'static str,
    completed: bool,
}

impl Wfc3dCommandWork {
    pub(super) fn new(tool_id: &'static str) -> Self {
        Self { tool_id, completed: false }
    }
}

impl semio_plugin::retained_command::ArtifactCommandWork<EditorApp<Wfc3dEditor>> for Wfc3dCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Wfc3dEditorCommand,
        snapshot: &Wfc3dSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Wfc3dEditor>>>,
    ) -> Option<usize> {
        (!self.completed && wfc3d_command_id(command) == self.tool_id).then(|| wfc3d_retained_extent(command, snapshot)).flatten()
    }

    fn step(
        &mut self,
        input: &semio_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Wfc3dEditor>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Wfc3dEditor>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.completed || wfc3d_command_id(input.command) != self.tool_id {
            return Err(Fault::new(
                semio_plugin::FaultOrigin::App,
                semio_plugin::FaultCode::new("wfc3d.retained.route"),
                "the bounded WFC 3D work rejects an undeclared or completed route",
            ));
        }
        let emit = command_emit(input.command, input.snapshot, input.config, &input.operation.authoring_seed)?;
        let transient = match input.command {
            Wfc3dEditorCommand::CommitFill { payload_json } => {
                let payload = fill::decode_fill_payload(payload_json.as_bytes()).ok_or_else(|| Fault::from("wfc3d-commit-fill-payload"))?;
                if payload.completed_for(Some(&input.operation.canonical_base_revision)) { payload.set_solve_mutations() } else { Vec::new() }
            }
            _ => Vec::new(),
        };
        self.completed = true;
        Ok(semio_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit,
            ephemeral: EphemeralEmit { presence: Vec::new(), transient, window_transient: Vec::new() },
        })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

/// 🏭️ The app-owned retained command job factory — `factory_type:` in the proof block binds this exact
/// Rust type to `EditorApp<Wfc3dEditor>`, which is what turns a bare (and therefore dispatch-dead)
/// bounded proof into an exact-owner proof.
pub struct Wfc3dRetainedCommandJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl Wfc3dRetainedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: WFC_3D_RETAINED_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Wfc3dRetainedCommandJobFactory {
    type Payload = semio_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Wfc3dEditor>>;
    type Job = semio_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Wfc3dEditor>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        WFC_3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        wfc3d_retained_contract()
    }

    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: WFC_3D_RETAINED_RAW_BYTES, checkpoint_resume: false);

}

impl semio_plugin::ArtifactOwnedToolJobFactory for Wfc3dRetainedCommandJobFactory {
    type Owner = EditorApp<Wfc3dEditor>;
    const TOOL_IDS: &'static [&'static str] = WFC_3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WFC3D_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_plugin::ArtifactToolPublicationContract] = WFC_3D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️Retained

#[cfg(test)]
mod completion_source_tests {
    use super::*;
    use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandSnapshotRead, ArtifactCommandWork, ArtifactCommandWorkStep};
    use crate::editor::wfc3d::transient::{Wfc3dAssignment, Wfc3dTransient};

    /// 📨️ Real retained completion work preserves an existing complete cache when a late callback is stale.
    #[test]
    fn stale_completion_emits_no_rows_and_cannot_overwrite_a_newer_cache() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🧪️tests/🧫️preview-revisions/🔣️.json")).unwrap();
        let snapshot = crate::examples::two_room_corridor::snapshot();
        let config = crate::editor::wfc3d::config::Wfc3dConfig::default();
        let history = semio_plugin::HistoryView::empty();
        let interaction = protocol::InteractionState::default();
        let hover = Default::default();
        let grant = semio_plugin::artifact_app_laws::fixture_retained_turn().grant;
        for case in fixture["cases"].as_array().unwrap() {
            let source_revision: Vec<u8> = serde_json::from_value(case["sourceRevision"].clone()).unwrap();
            let current: Option<[u8;32]> = serde_json::from_value(case["currentRevision"].clone()).unwrap();
            let payload = fill::Wfc3dFillTickPayload { source_revision, done: true, assignments: [(snapshot.slots[0].id.clone(), Some(snapshot.tiles.last().unwrap().id.clone()))].into_iter().collect(), ..Default::default() };
            let Some(revision) = current else { assert!(!payload.completed_for(None)); continue; };
            let operation = semio_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "wfc3d".into(), operation_id: 1, generation: 1, canonical_base_revision: revision, retained: grant, authoring_seed: "completion".into() };
            let command = Wfc3dEditorCommand::CommitFill { payload_json: String::from_utf8(fill::encode_fill_payload(&payload)).unwrap() };
            let input = ArtifactCommandInputs::<semio_plugin::EditorApp<Wfc3dEditor>> { command: &command, snapshot: &snapshot, snapshot_owner: None, snapshot_read: ArtifactCommandSnapshotRead::unavailable(&operation), config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
            let mut cache = Wfc3dTransient { source_revision: revision.to_vec(), assignments: vec![Wfc3dAssignment { slot_id: snapshot.slots[0].id.clone(), tile_id: snapshot.tiles[0].id.clone() }], contradiction: false };
            let before = cache.clone();
            let mut work = Wfc3dCommandWork::new(fill::COMMIT_FILL_ACTION_ID);
            let mut caller = semio_plugin::artifact_app_laws::OriginalWorkFixture::new();
            let step = caller.work_turn(grant, 1, |cx| { cx.consume_fuel(1); work.step(&input, cx) }).unwrap();
            let rows = match step { ArtifactCommandWorkStep::CompleteWithEphemeral { ephemeral, .. } => ephemeral.transient, ArtifactCommandWorkStep::Complete(_) => Vec::new(), _ => panic!("completion must settle") };
            let visible = case["visible"].as_bool().unwrap();
            assert_eq!(rows.len(), usize::from(visible), "{}", case["id"]);
            for row in rows { cache = protocol::apply_mutation(&cache, &row).unwrap().0; }
            assert_eq!(cache != before, visible, "{}", case["id"]);
            if !visible { assert_eq!(cache, before); }
            assert!(work.terminal_is_empty());
            caller.finish();
        }
    }
}
