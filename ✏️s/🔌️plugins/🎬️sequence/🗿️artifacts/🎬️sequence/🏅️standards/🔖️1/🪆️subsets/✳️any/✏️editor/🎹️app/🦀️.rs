//! 🎹️ Sequence play app: the proof catalogs, the `ArtifactEditor` impl and the manifest stitch.

use crate::editor::sequence::panels::catalogue as catalogue_panel;
use crate::editor::sequence::panels::document as document_panel;
use crate::editor::sequence::panels::inspection as inspection_panel;
use semio_framework_plugin::{ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, AppActionRegistry, AppDefinition, AppIo, ArtifactEditor, ArtifactView, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DomainTopology, DraftView, DslValue, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Media, MediaError, MediaPayload, MergeMode, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode};
use std::collections::BTreeMap;
use semio_framework_artifact_reference::Dialect;
use semio_framework_2d_compute::EngineHandles;
use semio_framework_plugin::app::InteractionView;
use semio_framework_ui_locale::{Label, LocalizedLabel};
use crate::{SEQUENCE_DOCUMENT_SCHEMA, SequenceSnapshot, SequenceStep, SequenceWorkingScene, StepParams};
use crate::mutations::SequenceMutation;
use crate::editor::sequence::modes::edit::windows::script::transient::SequenceScriptWindowTransient;
use crate::editor::sequence::commands::step::{add_step, add_step_dropped, add_step_to_slot, delete_selection, move_step, remove_step, set_step_collapsed, set_step_params};
use crate::editor::sequence::modes::edit::windows::{compiled, main, script};
use crate::editor::sequence::commands::connection::{connect_steps, disconnect_steps};
use crate::editor::sequence::modes::edit;
use crate::editor::sequence::edit_rules;
use semio_framework_pack_json::json;
use crate::editor::sequence::commands::node_graph::{node_graph_edit, set_viewport};
use crate::editor::sequence::commands::layout::{reorganize, set_orientation};
use crate::editor::sequence::commands::playback::{run_command, stop_command};
use crate::editor::sequence::terminology::sequence_play_labels;
use crate::editor::sequence::commands::example::set_active_example;
use crate::editor::sequence::panels::document::SEQUENCE_PLAY_BODY_ARTIFACT;
use crate::editor::sequence::panels::catalogue::SEQUENCE_PLAY_BODY_CATALOGUE;
use crate::editor::sequence::modes::edit::windows::compiled::SEQUENCE_PLAY_BODY_COMPILED;
use crate::editor::sequence::panels::inspection::SEQUENCE_PLAY_BODY_INSPECTOR;
use crate::editor::sequence::modes::edit::windows::main::SEQUENCE_PLAY_BODY_MAIN;
use crate::editor::sequence::modes::edit::windows::script::SEQUENCE_PLAY_BODY_SCRIPT;
use crate::editor::sequence::import::{SEQUENCE_IMPORT_TOOL_ID, SequenceImportJob};
use crate::editor::sequence::editor_vocabulary::{SEQUENCE_INTERACTION_GRANULARITY, SEQUENCE_INTERACTION_STEPS, sequence_child_leaves_emit, sequence_fault, sequence_fault_notices, sequence_host_snapshot_from_children, sequence_io, sequence_working_scene_from_children};
use crate::editor::sequence::persistent::{SEQUENCE_PERSISTENT_MAXIMUM_UNITS, SEQUENCE_PERSISTENT_TOOL_IDS, SequencePersistentJobFactory, SequencePersistentWork};
use crate::editor::sequence::retained::{SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS, SEQUENCE_RETAINED_CONFIG_TOOL_IDS, SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS, SEQUENCE_RETAINED_EXAMPLE_TOOL_IDS, SEQUENCE_RETAINED_MAXIMUM_UNITS, SEQUENCE_RETAINED_RAW_BYTES, SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS, SequenceCommand, SequenceRetainedArtifactJobFactory, SequenceRetainedArtifactWork, SequenceRetainedConfigJobFactory, SequenceRetainedConfigWork, SequenceRetainedExampleJobFactory, SequenceRetainedExampleWork, sequence_retained_artifact_command_admitted, sequence_retained_config_command_admitted, sequence_retained_example_command_admitted};
use crate::editor::sequence::host::max_serial_in_snapshot;

//#region 🔖️SequencePlayApp
/// 🧪️ Stateless app shell; exact window owners hold graph preferences and run output.
#[derive(Default)]
pub struct SequencePlayApp;

//#region 🧾️ProofCatalogs
struct SequenceArtifactProofs;
impl SequenceArtifactProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SequencePlayApp>,
        owner_file: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎹️app/🦀️.rs",
        controller: "s.sequence.sequence@1/*#editor",
        artifact_schema: "sequence.sequence",
        factory: "SequenceRetainedArtifactJobFactory",
        factory_type: SequenceRetainedArtifactJobFactory,
        tools: {
            "addStep" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "addStepToSlot" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "addStepDropped" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "removeStep" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "deleteSelection" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "moveStep" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "connectSteps" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "disconnectSteps" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "setStepParams" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "setStepCollapsed" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
        }
    }
}

struct SequencePersistentProofs;
impl SequencePersistentProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SequencePlayApp>,
        owner_file: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎹️app/🦀️.rs",
        controller: "s.sequence.sequence@1/*#editor",
        artifact_schema: "sequence.sequence",
        factory: "SequencePersistentJobFactory",
        factory_type: SequencePersistentJobFactory,
        tools: {
            "reorganize" => semio_framework::ToolExecutionContract::resumable(4_096, 66_049, 1, 65_536, 7_500, 1, 1),
            "nodeGraphEdit" => semio_framework::ToolExecutionContract::resumable(4_096, 66_049, 1, 65_536, 7_500, 1, 1),
            "run" => semio_framework::ToolExecutionContract::resumable(4_096, 66_049, 1, 65_536, 7_500, 1, 1),
        }
    }
}

struct SequenceConfigProofs;
impl SequenceConfigProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SequencePlayApp>,
        owner_file: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎹️app/🦀️.rs",
        controller: "s.sequence.sequence@1/*#editor",
        artifact_schema: "sequence.sequence",
        factory: "SequenceRetainedConfigJobFactory",
        factory_type: SequenceRetainedConfigJobFactory,
        tools: {
            "setViewport" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 4_096, 2_000, 1, 1),
            "setOrientation" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 4_096, 2_000, 1, 1),
            "stop" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 4_096, 2_000, 1, 1),
        }
    }
}

struct SequenceExampleProofs;
impl SequenceExampleProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SequencePlayApp>,
        owner_file: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎹️app/🦀️.rs",
        controller: "s.sequence.sequence@1/*#editor",
        artifact_schema: "sequence.sequence",
        factory: "SequenceRetainedExampleJobFactory",
        factory_type: SequenceRetainedExampleJobFactory,
        tools: {
            "setActiveExample" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 7_500, 1, 1),
        }
    }
}
//#endregion 🧾️ProofCatalogs

impl ArtifactEditor for SequencePlayApp {
    /// 🧩️ Composes `s.stdio.semio@v1/*` children, so every bundle of this surface opens them through the same roster.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = SequenceSnapshot;
    type Mutation = SequenceMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = SequenceCommand;

    const DIALECT: Dialect = crate::SEQUENCE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("sequence.child.projection"), error.to_string()))
    }

    /// 🔔️ The localized notices of the editor's own refusal codes (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        sequence_fault_notices()
    }

    /// 🌱️ The derivable `content` member — see `crate::genesis_sequence_child_pack`.
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::genesis_sequence_child_pack(snapshot, slot, child_id)
    
})())
}

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<main::config::SequenceMainWindowConfigOwner>()
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<script::transient::SequenceScriptWindowTransientOwner>()
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        SequenceArtifactProofs::bounded_first_step_tool_proofs()
            .into_iter()
            .chain(SequencePersistentProofs::bounded_first_step_tool_proofs())
            .chain(SequenceConfigProofs::bounded_first_step_tool_proofs())
            .chain(SequenceExampleProofs::bounded_first_step_tool_proofs())
            .collect()
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(SequenceRetainedArtifactJobFactory::new(&controller))?;
        registry.register(SequencePersistentJobFactory::new(&controller))?;
        registry.register(SequenceRetainedConfigJobFactory::new(&controller))?;
        registry.register(SequenceRetainedExampleJobFactory::new(&controller))
    }

    /// 🎞️ `import-media` is the only reserved route Sequence owns. The framework registers the
    /// reserved factory but never a concrete importer, so every inbound `steps:in` delivery is routed
    /// exclusively through this builder (`dispatch_import_media` → `build_artifact_reserved_media_job`);
    /// `copy`/`cut`/`paste` stay on the framework's own reserved factories.
    fn build_reserved_tool_job(request: semio_framework_plugin::ArtifactReservedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ArtifactReservedToolJob>, Fault> {
        if request.tool_id.as_str() != SEQUENCE_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(sequence_fault("sequence.import-media.undecoded", "sequence import-media admits a decoded media value, never a wire payload"));
        }
        let semio_framework_plugin::ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(sequence_fault("sequence.import-media.missing", "sequence import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(semio_framework_plugin::ArtifactReservedToolJob::new(SequenceImportJob::new(request, port, media))))
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        let artifact_route = SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS.contains(&request.tool_id.as_str());
        let config_route = SEQUENCE_RETAINED_CONFIG_TOOL_IDS.contains(&request.tool_id.as_str());
        let persistent_route = SEQUENCE_PERSISTENT_TOOL_IDS.contains(&request.tool_id.as_str());
        let example_route = SEQUENCE_RETAINED_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str());
        if !artifact_route && !config_route && !persistent_route && !example_route {
            return Ok(None);
        }
        let persistent_admitted = match request.command.as_ref() {
            SequenceCommand::NodeGraphEdit(payload) => payload.operations_json.len() <= SEQUENCE_RETAINED_RAW_BYTES,
            SequenceCommand::Reorganize(_) | SequenceCommand::Run(_) => true,
            _ => false,
        };
        if request.command.command_id() != request.tool_id
            || (artifact_route && !sequence_retained_artifact_command_admitted(&request.command))
            || (config_route && !sequence_retained_config_command_admitted(&request.command))
            || (persistent_route && !persistent_admitted)
            || (example_route && !sequence_retained_example_command_admitted(&request.command))
        {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "Sequence command does not match its exact retained route or payload envelope"));
        }
        let tool_id = request.command.command_id();
        let operation_context = semio_framework_plugin::AppOperationContext { retained: request.retained,
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> = if persistent_route {
            Box::new(SequencePersistentWork::new(tool_id, &operation_context))
        } else if artifact_route {
            Box::new(SequenceRetainedArtifactWork::new(tool_id, &operation_context))
        } else if example_route {
            Box::new(SequenceRetainedExampleWork::new(tool_id, &operation_context))
        } else {
            Box::new(SequenceRetainedConfigWork::new(tool_id, &operation_context))
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            SequenceCommand::command_id,
            SEQUENCE_RETAINED_RAW_BYTES,
            if persistent_route {
                SEQUENCE_PERSISTENT_MAXIMUM_UNITS
            } else if artifact_route {
                SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS
            } else if example_route {
                SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS
            } else {
                SEQUENCE_RETAINED_MAXIMUM_UNITS
            },
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn initial_snapshot() -> SequenceSnapshot {
        crate::snapshot::schema::default_persisted_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(sequence_io())
    }

    /// 🎞️ `steps:in` (Wave-2 port recipe): inserts incoming computation results as a new step at the
    /// far right of the flow — an object payload becomes that step's params verbatim, a bare
    /// scalar/array is wrapped under a single `"value"` key. Never mutates anything directly (matches
    /// every other `import_media` override): the caller (a headless runner or the UI) applies the
    /// returned `create-step` mutation through the ordinary, undoable document store.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, SequenceSnapshot>) -> Result<Emit<SequenceMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "steps:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "steps:in importer only accepts a Structured (JSON) payload".into()));
        };
        let value = json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let params_value = if value.as_object().is_some() { value } else { json!({ "value": value }) };
        let params: StepParams = semio_framework_pack_json::from_json_str(&params_value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let live = sequence_host_snapshot_from_children(doc.snapshot, &doc.children).map_err(|error| MediaError::Payload(port.to_string(), format!("{error:?}")))?;
        let id = format!("step-{}", max_serial_in_snapshot(&live).max(100) + 1);
        let x = live.steps.iter().map(|step| step.x).fold(0.0_f64, f64::max) + if live.steps.is_empty() { 0.0 } else { 280.0 };
        let mut edit = edit_rules::SceneEdit::new(SequenceWorkingScene { steps: live.steps.clone(), edges: live.edges.clone() });
        edit.insert_step(SequenceStep { id, kind: "computation.import".into(), params, x, y: 0.0, slot: None, collapsed: false });
        Ok(sequence_child_leaves_emit(doc.snapshot, edit.leaves))
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &SequenceCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps a host action id + its staged args onto `SequenceCommand`. The React/wgpu shells
    /// still dispatch `{action, args}` while the guest channel is typed-only, and the trait default
    /// refuses EVERY id (`app.command.unsupported`) — without this bridge no Actions-pane row, no
    /// palette drop and no graph gesture ever reached `handle`. Key aliases mirror the shells' own
    /// vocabularies (`value`/`id`, `nodeId`, `sourceNodeId`) rather than adding shell-side shims.
    fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<SequenceCommand, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let text_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(DslValue::as_str).map(str::to_string));
        let number_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(DslValue::as_f64)).unwrap_or_default();
        let json_arg = |key: &str, fallback: &str| args.and_then(|value| value.get(key)).map_or_else(|| fallback.to_string(), json::to_json_string);
        match action {
            "addStep" => Ok(SequenceCommand::AddStep(add_step::AddStep { kind: text_arg(&["kind", "value"]).unwrap_or_else(|| "computation.import".into()), x: number_arg(&["x"]), y: number_arg(&["y"]) })),
            "addStepToSlot" => Ok(SequenceCommand::AddStepToSlot(add_step_to_slot::AddStepToSlot {
                kind: text_arg(&["kind", "value"]).unwrap_or_else(|| "computation.import".into()),
                x: number_arg(&["x"]),
                y: number_arg(&["y"]),
                owner: text_arg(&["owner"]).unwrap_or_default(),
                slot_name: text_arg(&["slotName", "slot_name", "slot"]).unwrap_or_default(),
            })),
            "addStepDropped" => Ok(SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped {
                kind: text_arg(&["kind", "value"]).unwrap_or_else(|| "computation.import".into()),
                x: number_arg(&["x"]),
                y: number_arg(&["y"]),
                picked_step_id: text_arg(&["pickedStepId", "picked_step_id"]),
            })),
            "removeStep" => Ok(SequenceCommand::RemoveStep(remove_step::RemoveStep { id: text_arg(&["id", "stepId", "value"]).unwrap_or_default() })),
            "deleteSelection" => Ok(SequenceCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "moveStep" => Ok(SequenceCommand::MoveStep(move_step::MoveStep { node_id: text_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(), x: number_arg(&["x"]), y: number_arg(&["y"]) })),
            "connectSteps" => Ok(SequenceCommand::ConnectSteps(connect_steps::ConnectSteps { source_node_id: text_arg(&["sourceNodeId", "source_node_id", "from"]).unwrap_or_default(), target_node_id: text_arg(&["targetNodeId", "target_node_id", "to"]).unwrap_or_default() })),
            "disconnectSteps" => Ok(SequenceCommand::DisconnectSteps(disconnect_steps::DisconnectSteps { from_id: text_arg(&["fromId", "from_id", "from"]).unwrap_or_default(), to_id: text_arg(&["toId", "to_id", "to"]).unwrap_or_default() })),
            "setStepParams" => Ok(SequenceCommand::SetStepParams(set_step_params::SetStepParams { id: text_arg(&["id", "stepId"]).unwrap_or_default(), params_json: json_arg("params", "null") })),
            "setStepCollapsed" => Ok(SequenceCommand::SetStepCollapsed(set_step_collapsed::SetStepCollapsed { id: text_arg(&["id", "stepId", "value"]).unwrap_or_default() })),
            "reorganize" => Ok(SequenceCommand::Reorganize(reorganize::Reorganize {})),
            "nodeGraphEdit" => Ok(SequenceCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: json_arg("operations", "[]") })),
            "setOrientation" => Ok(SequenceCommand::SetOrientation(set_orientation::SetOrientation { value: text_arg(&["value", "orientation"]).unwrap_or_else(|| "horizontal".into()) })),
            "run" => Ok(SequenceCommand::Run(run_command::Run {})),
            "stop" => Ok(SequenceCommand::Stop(stop_command::Stop {})),
            "setViewport" => {
                let value = args.and_then(|value| value.get("camera")).or_else(|| args.and_then(|value| value.get("viewport"))).cloned().ok_or_else(|| sequence_fault("sequence.viewport.camera", "sequence setViewport requires a camera"))?;
                Ok(SequenceCommand::SetViewport(set_viewport::SetViewport { camera: semio_framework_value::FromValue::from_value(value).map_err(|error| sequence_fault("sequence.viewport.camera", format!("invalid sequence setViewport camera: {error}")))? }))
            }
            "setActiveExample" => Ok(SequenceCommand::SetActiveExample(set_active_example::SetActiveExample {
                example_id: text_arg(&["exampleId", "example_id", "id", "value"]).unwrap_or_else(|| crate::examples::demo::ID.into()),
            })),
            other => Err(sequence_fault("sequence.action.unhandled", format!("sequence: unhandled action id {other}"))),
        }
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit` read the "steps" interaction domain directly (bypassing
    /// the `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg)`
    /// signature is framework-fixed and has no `interaction` slot) — ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(
        command: &SequenceCommand,
        doc: &ArtifactView<'_, SequenceSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<SequenceMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            SequenceCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction),
            _ => command.dispatch(doc, cfg),
        }
    }

    /// 🕹️ `steps`'s `HierarchyProvider::Topology` — every step is registered at the "step"
    /// granularity, parented to its control-flow slot owner (`SlotRef.owner`) when nested inside a
    /// `then`/`else`/`body` slot, or as a root otherwise — mirrors the document panel's own nesting
    /// (`build_step_tree_item`) so a deleted step's id auto-prunes out of the live selection.
    fn interaction_topology(doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let ordered = sequence_working_scene_from_children(doc.snapshot, &doc.children)
            .map(|scene| scene.steps.iter().map(|step| TopologyNode { id: step.id.clone(), granularity: "step".into(), parent: step.slot.as_ref().map(|slot| slot.owner.clone()) }).collect())
            .unwrap_or_default();
        let mut domains = BTreeMap::new();
        domains.insert(SEQUENCE_INTERACTION_STEPS.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    
})())
}

    fn render(body_key: &str, doc: &ArtifactView<'_, SequenceSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let live = sequence_host_snapshot_from_children(doc.snapshot, &doc.children)
            .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("sequence.child-content", format!("{error:?}")))?;
        let config = main::config::current(cfg);
        let transient = SequenceScriptWindowTransient::default();
        let labels = sequence_play_labels(view_state);
        match body_key {
            SEQUENCE_PLAY_BODY_MAIN => main::render(&live, &config),
            SEQUENCE_PLAY_BODY_SCRIPT => script::render(&live, &transient),
            SEQUENCE_PLAY_BODY_COMPILED => compiled::render(&live),
            SEQUENCE_PLAY_BODY_ARTIFACT => document_panel::render(&live, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SEQUENCE_PLAY_BODY_ARTIFACT)),
            SEQUENCE_PLAY_BODY_CATALOGUE => catalogue_panel::render(&live, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SEQUENCE_PLAY_BODY_CATALOGUE)),
            // 🕹️ `render` carries no `InteractionView` (same gap as `context_menu` below — see ticket
            // 26/08/14's w3b-summary.md), so this always takes the "nothing selected" branch rather
            // than reading a stale/wrong selection.
            SEQUENCE_PLAY_BODY_INSPECTOR => inspection_panel::render(&live, &[], labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "sequence diagnostic admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }

    /// 🕹️ `context_menu` carries no `InteractionView` (same gap as `render` above — see ticket
    /// 26/08/14's w3b-summary.md), so the selection-dependent rows built by
    /// `sequence_context_menu_items` below always take the "nothing selected" branch here rather than
    /// reading a stale/wrong selection.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, SequenceSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        _interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        let live = sequence_host_snapshot_from_children(doc.snapshot, &doc.children)
            .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("sequence.child-content", format!("{error:?}")))?;
        let config = main::config::current(cfg);
        let transient = script::transient::current(transient);
        let labels = sequence_play_labels(view_state);
        match body_key {
            SEQUENCE_PLAY_BODY_MAIN => main::render(&live, &config),
            SEQUENCE_PLAY_BODY_SCRIPT => script::render(&live, &transient),
            SEQUENCE_PLAY_BODY_COMPILED => compiled::render(&live),
            SEQUENCE_PLAY_BODY_ARTIFACT => document_panel::render(&live, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SEQUENCE_PLAY_BODY_ARTIFACT)),
            SEQUENCE_PLAY_BODY_CATALOGUE => catalogue_panel::render(&live, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SEQUENCE_PLAY_BODY_CATALOGUE)),
            SEQUENCE_PLAY_BODY_INSPECTOR => inspection_panel::render(&live, &[], labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "sequence diagnostic admission failed")),
        }.map(semio_framework_plugin::built_to_component_tree)

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        sequence_context_menu_items(registry, view_state, request.surface.as_ref(), &[])
    }
}

/// 🗂️ Grouped disclosure: `run`/`stop`/`addStep` stay top-level (the most frequent verbs);
/// `reorganize` folds into the `transform` group and a single-node hit's `setStepCollapsed` folds
/// into the `selection` group; `deleteSelection` stays a direct destructive item last —
/// `organize_context_menu` (applied automatically at the `VcsArtifactApp::context_menu` funnel)
/// sorts the groups into `RIBBON_PARENT_CATEGORIES` order and inserts the pre-destructive separator
/// itself. Factored out of `ArtifactApp::context_menu` (which carries no `InteractionView` — ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) so a test can exercise the selection-dependent
/// rows directly with a real `selected` slice, matching `space`'s own precedent.
pub fn sequence_context_menu_items(registry: &AppActionRegistry, view_state: &semio_framework_plugin::ViewModel, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>, selected: &[String]) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};
    let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;

    let (nodes, edges) = selection_domains_from_surface(surface, selected, &[]);

    let mut menu = Menu::of(registry, view_state).action("run").action("stop").action("addStep").group("transform", |m| m.action("reorganize"));

    if nodes.len() == 1 {
        let id = nodes[0].clone();
        menu = menu.group("selection", |m| {
            m.item(ContextMenuItemSpec {
                id: "setStepCollapsed".into(),
                label: Some(if is_de { "Schritt einklappen".into() } else { "Toggle Collapsed".into() }),
                icon: Some("chevrons-up-down".into()),
                action: Some("setStepCollapsed".into()),
                args: Some(json::to_dsl_value(&json!({ "id": id }))),
                ..Default::default()
            })
        });
    }

    menu.item(node_graph_delete_selection_spec(semio_framework_plugin::delete_selection().resolve(view_state.terminology, view_state.locale), view_state, &nodes, &edges, NodeGraphDeleteDispatch::Direct)).build()
}
//#endregion 🔖️SequencePlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own
/// `definition()`. Only the leaf action/keybinding declarations (which have no dedicated `_def`
/// passthrough) are written out inline.
pub fn create_sequence_app() -> AppDefinition {
    Editor::builder(crate::SEQUENCE_DIALECT)
            .document(["semio", "sequence"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("sequence")
            .mode_def(edit::definition())
            .default_mode_id(edit::SEQUENCE_PLAY_MODE_EDIT)
            .window_kind_def(main::definition())
            .window_kind_def(script::definition())
            .window_kind_def(compiled::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .action_with(ActionDefinition::bounded_catalog("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"), ActionKind::Mutation).with_category("create"))
            .mutation("addStepToSlot", LocalizedLabel::native("Add Step To Slot", "Schritt zu Slot hinzufügen"))
            .mutation("addStepDropped", LocalizedLabel::native("Add Step Dropped", "Schritt per Ablegen hinzufügen"))
            .action_audience("addStepDropped", semio_framework_plugin::CapabilityAudience::Input)
            .mutation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
            .action_destructive("removeStep")
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteSelection")
            .mutation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .mutation("connectSteps", LocalizedLabel::native("Connect Steps", "Schritte verbinden"))
            .mutation("disconnectSteps", LocalizedLabel::native("Disconnect Steps", "Schritte trennen"))
            .mutation("setStepParams", LocalizedLabel::native("Set Step Params", "Schrittparameter festlegen"))
            .action_with(ActionDefinition::bounded_catalog("setStepCollapsed", LocalizedLabel::native("Set Step Collapsed", "Schritt einklappen"), ActionKind::Mutation).with_category("selection"))
            .action_with(ActionDefinition::new("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "rotate-cw").with_category("transform"))
            .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .view_action("setViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
            // 👁️ Main-window orientation is persisted by its exact config owner; run output is
            // ephemeral on the invoking Script window.
            .view_action("setOrientation", LocalizedLabel::native("Set Orientation", "Ausrichtung festlegen"))
            .action_with(ActionDefinition::new("run", LocalizedLabel::native("Run", "Ausführen"), ActionKind::View, "play").with_category("actions"))
            .action_with(ActionDefinition::new("stop", LocalizedLabel::native("Stop", "Stopp"), ActionKind::View, "play").with_category("actions"))
            // 📚️ The playground navbar dispatches `setActiveExample` on boot for its example
            // combobox, and the shell offers that combobox only to an app that declares the verb.
            // Undeclared, it was dropped before dispatch — this app's console ERROR at boot, and the
            // reason its committed example never reached the document store at all.
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            // 📝️ Staged argument forms for the panel-visible create + layout actions.
            .action_args("addStep", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("state.set", LocalizedLabel::native("Set State", "Zustand setzen")),
                    ActionArgOption::new("log.print", LocalizedLabel::native("Print", "Ausgeben")),
                    ActionArgOption::new("control.if", LocalizedLabel::native("If", "Wenn")),
                    ActionArgOption::new("control.while", LocalizedLabel::native("While", "Solange")),
                    ActionArgOption::new("math.add", LocalizedLabel::native("Add", "Addieren")),
                ]).default_value(&"log.print"),
            ])
            .action_args("setOrientation", vec![
                ActionArgDef::select("orientation", LocalizedLabel::native("Orientation", "Ausrichtung"), vec![
                    ActionArgOption::new("leftRight", LocalizedLabel::native("Left to Right", "Links nach rechts")),
                    ActionArgOption::new("topBottom", LocalizedLabel::native("Top to Bottom", "Oben nach unten")),
                ]).required(),
            ])
            .action_interactive_job("addStep", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addStepToSlot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addStepDropped", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("removeStep", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("moveStep", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("connectSteps", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("disconnectSteps", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setStepParams", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setStepCollapsed", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphEdit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setOrientation", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("run", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("stop", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setViewport", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label())])
                    .required()
                    .default_value(&crate::examples::demo::ID),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
            // one domain over the step graph, `HierarchyProvider::Topology` (see
            // `SequencePlayApp::interaction_topology` above) from each step's own control-flow slot
            // nesting — `transitive: false` matches the pre-migration behavior exactly (deleting a
            // selected control step never cascaded into its `then`/`else`/`body` children).
            .interaction(InteractionDefinition {
                id: SEQUENCE_INTERACTION_STEPS.into(),
                label: LocalizedLabel::native("Steps", "Schritte"),
                granularities: vec![GranularityDefinition { id: SEQUENCE_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Step", "Schritt"), icon_id: "box".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(main::SEQUENCE_PLAY_WINDOW_MAIN, vec![InteractionRef::new(SEQUENCE_INTERACTION_STEPS)])
            .window_kind_action_refs(main::SEQUENCE_PLAY_WINDOW_MAIN, vec![
                "reorganize".into(), "nodeGraphEdit".into(), "setOrientation".into(), "setViewport".into(),
            ])
            .window_kind_action_refs(script::SEQUENCE_PLAY_WINDOW_SCRIPT, vec!["run".into(), "stop".into()])
            .io(sequence_io())
            // 🕳️ SDK gap (contract §2.4, confirmed absent on `EditorBuilder` as of this packet): the
            // pre-migration chain's trailing `.example_source(art_sequence_demo::source())` /
            // `.workflow("sequence", "Sequence", "graph")` calls are dropped here, not silently ported
            // — `Editor::builder(...)` has no such methods, and `PluginBuilder::editor::<E>(def)`
            // wraps `def` in `App { definition: def, examples: Vec::new() }`, discarding `App.examples`
            // even if it were populated. The subset's own `📚️examples/🎬️demo` facet (mounted at the
            // plugin root as `artifacts::sequence::examples::demo`) is the closest surviving carrier of this
            // content today.
            .action_describe("reorganize", LocalizedLabel::native("Lays out every step of the sequence automatically along the window's flow direction, overwriting their manual positions.", "Ordnet alle Schritte der Sequenz automatisch entlang der Flussrichtung des Fensters an und überschreibt ihre manuellen Positionen."))
            .action_describe("setOrientation", LocalizedLabel::native("Sets the flow direction (such as left to right or top to bottom) that Reorganize lays the steps out along; only the window's setting changes.", "Legt die Flussrichtung fest (etwa links nach rechts oder oben nach unten), entlang der Neu anordnen die Schritte auslegt; nur die Einstellung des Fensters ändert sich."))
            .action_describe("run", LocalizedLabel::native("Runs the sequence's compiled path once from its first step; the document is not changed.", "Führt den kompilierten Pfad der Sequenz einmal ab dem ersten Schritt aus; das Dokument ändert sich nicht."))
            .action_describe("stop", LocalizedLabel::native("Stops a running sequence; the document is not changed.", "Hält eine laufende Sequenz an; das Dokument ändert sich nicht."))
            .action_describe("addStep", LocalizedLabel::native("Adds a new step of the given kind to the sequence canvas at x, y.", "Fügt der Sequenzfläche an x, y einen neuen Schritt der angegebenen Art hinzu."))
            .action_describe("addStepToSlot", LocalizedLabel::native("Adds a new step of the given kind into a named slot of an owner step (such as the body of a loop) at x, y.", "Fügt einen neuen Schritt der angegebenen Art an x, y in einen benannten Slot eines Besitzerschritts ein (etwa den Rumpf einer Schleife)."))
            .action_describe("removeStep", LocalizedLabel::native("Removes one step by id from the sequence together with the flow edges attached to it.", "Entfernt einen Schritt anhand seiner Id samt der angeschlossenen Flusskanten aus der Sequenz."))
            .action_describe("deleteSelection", LocalizedLabel::native("Removes every currently selected step from the sequence together with their flow edges.", "Entfernt alle aktuell ausgewählten Schritte samt ihrer Flusskanten aus der Sequenz."))
            .action_describe("moveStep", LocalizedLabel::native("Moves one step to an absolute position x, y on the sequence canvas.", "Verschiebt einen Schritt an die absolute Position x, y der Sequenzfläche."))
            .action_describe("connectSteps", LocalizedLabel::native("Adds a flow edge from one step to another, so the second runs after the first.", "Fügt eine Flusskante von einem Schritt zu einem anderen hinzu, sodass der zweite nach dem ersten läuft."))
            .action_describe("disconnectSteps", LocalizedLabel::native("Removes the flow edge between two steps, so they no longer run in that order.", "Entfernt die Flusskante zwischen zwei Schritten, sodass sie nicht mehr in dieser Reihenfolge laufen."))
            .action_describe("setStepParams", LocalizedLabel::native("Replaces the parameters of one step with the given name-to-value map.", "Ersetzt die Parameter eines Schritts durch die angegebene Zuordnung von Namen zu Werten."))
            .action_describe("setStepCollapsed", LocalizedLabel::native("Collapses or expands one step on the canvas, hiding or showing its nested steps.", "Klappt einen Schritt auf der Fläche ein oder aus und verbirgt oder zeigt seine verschachtelten Schritte."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole sequence with the bundled demo sequence; any other example id changes nothing.", "Ersetzt die gesamte Sequenz durch die mitgelieferte Demo-Sequenz; jede andere Beispiel-Id ändert nichts."))
            .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("setViewport", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("reorganize")
            .build_definition()
}
//#endregion 🔖️Manifest
