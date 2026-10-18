//! 👯️ Puzzle 5d play app — the plugin's unified 2d+3d play app: its `ArtifactApp` impl
//! (dispatch-only), the structural-twin document model its command/panel/window nodes mutate and
//! render, the shared scene/engine/brush helpers those nodes reach for, and the manifest that
//! stitches them together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/{◻2d,🧊️3d}`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle5dPlaySnapshot` newtype over a bare
//! `Value` document (see `crate::standards::v1::subsets::any::io::text::mutations`'s `🔖️ValueBridge`), not the
//! typed `Puzzle5dSnapshot` — the `Puzzle5dDocument` model below is this app's own structural twin
//! of it, and each action emits the concrete typed mutation kinds of the gesture it carries
//! (`🎮️commands/<group>`), never a delta of two documents.
use crate::editor::puzzle5d::entities::puzzle5d_grip_full_id;

pub use semio_s_artifact_puzzle_5d_mut_any_5::editor::puzzle5d::snapshot;


use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;

use crate::standards::v1::subsets::any::schema::mutations::{Puzzle5dMutation};

use crate::Puzzle5dSnapshot;
use crate::editor::puzzle5d::commands::{
    apply_sun, cycle_brush_candidate, delete_selection, duplicate_selection, engagement_abort, engagement_control_select, engagement_input,
    engagement_repeat_last, engagement_submit, register_brush_mesh, rotate_selection, scale_selection, select_same_kind, set_brush_placement_contact_tolerance,
    set_camera, set_camera_2d, set_camera_3d, set_fill_count, set_grid_factor, set_grid_snap_enabled, set_kind_weight, set_lod_mode, set_selection_flag, set_suggestion_offset, target_brush_suggestions, translate_selection, world_relocate,
};
use crate::editor::puzzle5d::commands::{
    set_grid_spacing, set_grid_visible, set_grip_direction, set_grip_show, set_lod_automatic, set_lod_depth_variable, set_lod_manual, set_projection, set_selectable_kind, set_transform_gumball_flag,
};
use crate::editor::puzzle5d::commands::{add_target_volume, delete_target_volume, relocate_target_volume, set_target_volume_flag, set_voxel_dims};
use crate::editor::puzzle5d::commands::{set_chunk_size, set_proximity_radius};
use crate::editor::puzzle5d::commands::{accept_suggestion, close_vortex_suggestions, hover_suggestion, open_vortex_suggestions};
use crate::editor::puzzle5d::commands::{export_snapshot, import_snapshot, open_add_part_dialog, open_import_snapshot};
use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dConfig, Puzzle5dConfigMutation, Puzzle5dRuntime, Puzzle5dConfigSetObjectKindWeights, Puzzle5dConfigSetVortexKindWeights};
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
use crate::editor::puzzle5d::panels::{catalogue, artifact as artifact_panel, inspection, settings as settings_panel};
use semio_s_artifact_puzzle_3d_editor::Puzzle3dInstanceOperationOwner;
use crate::editor::puzzle5d::presence::{Puzzle5dPresence, Puzzle5dPresenceMutation};
use crate::editor::puzzle5d::terminology::{puzzle5d_labels, puzzle5d_localized, Puzzle5dLabels};
use crate::editor::puzzle5d::window as window_ownership;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PasteAnchor, PastePlacement, UiDirtyScope};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactPresentation;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::ArtifactToolCompletion;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::DialogDefinition;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::InteractionWrite;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPortDirection;
use semio_framework_plugin::MediaPortSpec;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PortMultiplicity;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
// 🕹️ `InteractionView` — see 🧊️3d/🦀️.rs's identical import comment (missing top-level
// re-export from `semio_framework_plugin`, flagged to the coordinator, not fixed here).
use semio_framework_job::{InteractiveJob, Operation, StepContext};
use semio_framework_plugin::app::InteractionView;
use semio_framework_value::{FromValue, ToValue};
use semio_framework_pack_json::{parse, Value};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use semio_framework_2d_compute::EngineHandles;

pub use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities;
pub use entities::*;

pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::view;
pub use view::*;

pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::catalog;
pub use catalog::*;

pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction;
pub use interaction::*;

#[path = "🖌️session/🦀️.rs"]
pub mod session;
use session::*;

pub use semio_s_artifact_puzzle_5d_retained::editor::puzzle5d::retained;
use retained::*;

pub use semio_s_artifact_puzzle_5d_mut_any_6::editor::puzzle5d::gestures;
use gestures::*;

pub use semio_s_artifact_puzzle_5d_retained::editor::puzzle5d::kit;
use kit::*;

pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::dispatch;
use dispatch::puzzle5d_handle_action;

use crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::puzzle5d_inspector_nudge;

#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;
#[cfg(any(test, feature = "testkit"))]
pub use testkit::puzzle5d_app_manifest_for_tests;

//#region 🔖️PlayApp
// 🧩️ B1: Puzzle-5d play app. Stateless: the persisted document (bare `Puzzle5dDocument` json) lives in
// the wrapping `VcsArtifactApp`'s document store, the ephemeral view state in the wrapping store's real,
// VCS-tracked `Puzzle5dConfig` artifact (see `🦀️config.rs`) — every read comes from `cfg.snapshot`, every write
// flows out as a `Puzzle5dConfigMutation` in the returned `Emit` — and what an instance retains across turns
// (the brush suggestions link) in its instance operation owner.
// Each action mutates a transient {@link Puzzle5dScene}, then emits the granular operation delta.
// Undo/redo/checkpoints are handled by the wrapper.
#[derive(Default)]
pub struct Puzzle5dPlayApp;

impl Puzzle5dPlayApp {
    /// 🖱️ The ONE context-menu implementation — `ArtifactEditor::context_menu` and
    /// `context_menu_with_request_context` funnel here, differing only in whether `interaction` carries a live
    /// `vortex`-domain read or the empty default.
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &Puzzle5dInteractionSnapshot,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let Some(labels) = puzzle5d_labels(view_state) else { return Vec::new() };
        let window_id = view_state.window_id.as_deref().unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let mut selection = Puzzle5dContextSelection::from_surface(request.surface.as_ref());
        selection.fill_from_interaction(interaction);
        puzzle5d_context_menu_items(&envelope, &selection, labels, view_state, registry)
    }
}

//#region 🧵️RetainedCommands

pub struct Puzzle5dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
    contract: ToolExecutionContract,
}

impl Puzzle5dRetainedCommandJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self {
            keys: PUZZLE5D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(),
            contract: ToolExecutionContract::resumable(PUZZLE5D_RETAINED_RAW_BYTES, PUZZLE5D_RETAINED_DECODED_ITEMS, 1, crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES, crate::retained_command::PUZZLE_COMMAND_STEP_MICROS, 1, 1),
        }
    }
}

impl ToolJobFactory for Puzzle5dRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Puzzle5dPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Puzzle5dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PUZZLE5D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        self.contract
    }

    fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > payload.maximum_raw_bytes || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Puzzle 5d retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl ArtifactOwnedToolJobFactory for Puzzle5dRetainedCommandJobFactory {
    type Owner = EditorApp<Puzzle5dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PUZZLE5D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE5D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidate", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidateBack", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "duplicateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementControlSelect", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementRepeatLast", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "exportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importSnapshot", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "openAddPartDialog", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "openImportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "selectSameKindSelection", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "setFillCount", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSelectionFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "targetBrushSuggestions", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "openVortexSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "closeVortexSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "hoverSuggestion", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "acceptSuggestion", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "focusSelection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "addBrushPart", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "addPartKind", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "applyBoardEvents", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "createFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "editFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchGrip", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchPart", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "proximityConnect", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "registerBrushMesh", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "retargetFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "worldRelocate", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setCamera2d", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setCamera3d", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridFactor", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSnapEnabled", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSuggestionOffset", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setBrushPlacementContactTolerance", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setProximityRadius", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setChunkSize", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setPartKindWeight", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setGripKindWeight", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSpacing", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setProjection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setProjectionParam", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGripShow", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGripDirection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSelectableKind", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodAutomatic", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodDepthVariable", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodManual", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setTransformGumballFlag", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "addTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "relocateTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetVolumeFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetVolumeHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetVolumeLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setVoxelDims", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📜️ToolProofs
/// 📜️ The retained command catalog's own bounded first-step proofs — one per
/// `PUZZLE5D_RETAINED_TOOL_IDS` entry, all joined to the single concrete
/// `Puzzle5dRetainedCommandJobFactory`.
struct Puzzle5dRetainedCommandProofs;

impl Puzzle5dRetainedCommandProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle5dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle5d@1/*#editor",
        artifact_schema: "puzzle.5d",
        factory: "Puzzle5dRetainedCommandJobFactory",
        factory_type: Puzzle5dRetainedCommandJobFactory,
        contract: ToolExecutionContract::resumable(PUZZLE5D_RETAINED_RAW_BYTES, PUZZLE5D_RETAINED_DECODED_ITEMS, 1, 262_144, 7_500, 1, 1),
        tools: [
            "canvasPointerDown",
            "cycleBrushCandidate",
            "cycleBrushCandidateBack",
            "worldPointerDown",
            "deleteSelection",
            "duplicateSelection",
            "engagementAbort",
            "engagementControlSelect",
            "engagementInput",
            "engagementRepeatLast",
            "engagementSubmit",
            "exportSnapshot",
            "importSnapshot",
            "openAddPartDialog",
            "openImportSnapshot",
            "selectSameKindSelection",
            "setFillCount",
            "setSelectionFlag",
            "setSelectionHidden",
            "setSelectionLocked",
            "targetBrushSuggestions",
            "openVortexSuggestions",
            "closeVortexSuggestions",
            "hoverSuggestion",
            "acceptSuggestion",
            "focusSelection",
            "addBrushPart",
            "addNode",
            "addPartKind",
            "applyBoardEvents",
            "createFastener",
            "deleteFastener",
            "editFastener",
            "patchFastener",
            "patchGrip",
            "patchPart",
            "proximityConnect",
            "registerBrushMesh",
            "retargetFastener",
            "rotateSelection",
            "scaleSelection",
            "setActiveExample",
            "translateSelection",
            "worldRelocate",
            "setCamera",
            "setCamera2d",
            "setCamera3d",
            "setGridFactor",
            "setGridSnapEnabled",
            "setLodMode",
            "setSuggestionOffset",
            "toggleSun",
            "setSunAzimuth",
            "setSunElevation",
            "setSunIntensity",
            "setBrushPlacementContactTolerance",
            "setProximityRadius",
            "setChunkSize",
            "setPartKindWeight",
            "setGripKindWeight",
            "setGridVisible",
            "setGridSpacing",
            "setProjection",
            "setProjectionParam",
            "setGripShow",
            "setGripDirection",
            "setSelectableKind",
            "setLodAutomatic",
            "setLodDepthVariable",
            "setLodManual",
            "setTransformGumballFlag",
            "addTargetVolume",
            "deleteTargetVolume",
            "relocateTargetVolume",
            "setTargetVolumeFlag",
            "setTargetVolumeHidden",
            "setTargetVolumeLocked",
            "setVoxelDims"
        ]
    }
}

/// 📜️ The two framework-injected host-configuration verbs. Deliberately GENERIC proofs (no
/// `factory_type`): they are not app-owned retained tools — `Puzzle5dRetainedCommandJobFactory` never
/// claims them — they only need the wire admission and output budget `dispatch_action`'s
/// host-configuration branch asks for before it applies `host_configuration_mutation`. Without them the
/// shell's `setActiveTool`/`setActiveUtility` fall through to `admit_command_json` and fail closed with
/// `interactive-job.missing-factory`, so the fill TOOL could not even be armed.
struct Puzzle5dHostConfigurationProofs;

impl Puzzle5dHostConfigurationProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle5dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle5d@1/*#editor",
        artifact_schema: "puzzle.5d",
        factory: "BoundedFirstStepCommandJobFactory",
        contract: ToolExecutionContract::resumable(8_192, 8, 1, 8_192, 7_500, 1, 1),
        tools: ["setActiveTool", "setActiveUtility"]
    }
}
//#endregion 📜️ToolProofs

impl ArtifactEditor for Puzzle5dPlayApp {
    const DIALECT: semio_framework_plugin::app::Dialect = crate::PUZZLE5D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE5D_SCHEMA;
    type Snapshot = Puzzle5dPlaySnapshot;
    type Mutation = Puzzle5dMutation;
    type Config = Puzzle5dConfig;
    type ConfigMutation = Puzzle5dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Puzzle5dPresence;
    type PresenceMutation = Puzzle5dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = Puzzle5dCommand;

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_config(registry)
    }

    /// 🪧️ A history-edit reference chip names its board/world entity as the outliner does ([`puzzle5d_entity_label`]).
    fn entity_label(snapshot: &Puzzle5dPlaySnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        puzzle5d_entity_label(snapshot.typed(), kinds, id)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_transient(registry)
    }



    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }


    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    /// 👥️ Puzzle 5d presence is inline camera scalars, so the default root is its exact empty terminal.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |_| true).expect("default Puzzle5d presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Puzzle5dStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Puzzle5dConfigStorePreparationFactory))
    }

    /// ⏯️ The fill utility's run and revalidate jobs and the brush utility's suggestions run: the puzzle 3d planners
    /// behind the 5d translation.
    fn build_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
        match request.tool_id {
            board2d::utilities::brush::UTILITY_ID => crate::editor::puzzle5d::precompute::brush::build_run_job(request),
            fill_tool::TOOL_ID => crate::editor::puzzle5d::precompute::fill::build_run_job(request),
            _ => Ok(None),
        }
    }

    /// 📜️ The retained command catalog's proofs plus the two host-configuration verbs' generic ones.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Puzzle5dRetainedCommandProofs::bounded_first_step_tool_proofs();
        proofs.extend(Puzzle5dHostConfigurationProofs::bounded_first_step_tool_proofs());
        proofs
    }

    /// 🧠️ What one document instance retains across turns: puzzle 3d's brush suggestions link, which the brush run
    /// job follows and publishes to.
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Puzzle3dInstanceOperationOwner::default())
    }

    /// 🚦️ Starts, wakes or aborts the brush suggestions run for the grip the link points at.
    fn pending_effects(owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, _view: Option<&semio_framework_plugin::ViewModel>) -> Vec<Effect> {
        owner.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities::brush::run_effects(&mut owner.brush_suggestions, doc.tool_run()))).unwrap_or_default()
    }


    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(Puzzle5dRetainedCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !PUZZLE5D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "puzzle5d-command-tool-mismatch"));
        }
        let tool_id = request.command.action_id();
        let nonce = puzzle5d_operation_nonce(&request.operation);
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Self>>> = match tool_id {
            // ⏯️ The verbs that start, retarget or abort a run read the instance's live run as of admission.
            "engagementAbort" | "openVortexSuggestions" | "closeVortexSuggestions" => Box::new(Puzzle5dWindowCommandWork::new(tool_id, request.authoring_seed.clone()).bound(request.instance_operation_owner.clone()).with_tool_run(request.context.tool_run().cloned())),
            window if PUZZLE5D_WINDOW_TOOL_IDS.contains(&window) => Box::new(Puzzle5dWindowCommandWork::new(window, request.authoring_seed.clone()).bound(request.instance_operation_owner.clone())),
            "addBrushPart" | "addPartKind" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id).bound(nonce)),
            "applyBoardEvents" => Box::new(Puzzle5dBoardEventsWork::new(request.authoring_seed.clone()).bound(nonce)),
            "translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => Box::new(Puzzle5dTransformWork::new(tool_id, request.authoring_seed.clone())),
            "patchPart" if puzzle5d_inspector_nudge(request.command.args()).is_some() => Box::new(Puzzle5dTransformWork::new(tool_id, request.authoring_seed.clone())),
            "focusSelection" => Box::new(Puzzle5dFocusSelectionWork::default()),
            "patchPart" => Box::new(Puzzle5dPatchPartWork::default()),
            "patchFastener" => Box::new(Puzzle5dPatchFastenerWork::default()),
            "editFastener" => Box::new(Puzzle5dEditFastenerWork::default()),
            "retargetFastener" => Box::new(Puzzle5dRetargetFastenerWork::default()),
            "proximityConnect" => Box::new(Puzzle5dProximityConnectWork::default().bound(nonce)),
            "patchGrip" => Box::new(Puzzle5dPatchGripWork::default()),
            "deleteFastener" => Box::new(Puzzle5dDeleteFastenerWork::default()),
            "addNode" => Box::new(Puzzle5dAddNodeWork::default().bound(nonce)),
            "createFastener" => Box::new(Puzzle5dCreateFastenerWork::default().bound(nonce)),
            "exportSnapshot" => Box::new(Puzzle5dExportWork::default()),
            "setActiveExample" => Box::new(Puzzle5dSetActiveExampleWork::default()),
            "registerBrushMesh" => Box::new(Puzzle5dRegisterBrushMeshWork::default()),
            "setPartKindWeight" | "setGripKindWeight" => Box::new(Puzzle5dKindWeightWork::new(tool_id)),
            "worldPointerDown" | "canvasPointerDown" => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(tool_id)),
            _ => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce::<Puzzle5dPlayApp>, puzzle5d_retained_extent)),
        };
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
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
            Puzzle5dCommand::action_id,
            PUZZLE5D_RETAINED_RAW_BYTES,
            crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_reserved_tool_job(mut request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if !["copy", "cut", "paste", "import-media"].contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        let raw = std::mem::take(&mut request.raw_wire);
        request.raw_wire = match puzzle5d_preflight_reserved_wire(raw, request.contract.max_raw_wire_bytes) {
            Ok(raw) => raw,
            Err((fault, rejected)) => {
                drop(rejected);
                return Err(fault);
            }
        };
        let job = match request.tool_id.as_str() {
            "copy" => {
                let interaction = match &request.input {
                    ArtifactReservedToolInput::Action { interaction, .. } => interaction.clone(),
                    _ => return Err(Fault::from("puzzle5d copy requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dClipboardJob::new(Puzzle5dClipboardVerb::Copy, request, &interaction))
            }
            "cut" => {
                let interaction = match &request.input {
                    ArtifactReservedToolInput::Action { interaction, .. } => interaction.clone(),
                    _ => return Err(Fault::from("puzzle5d cut requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dClipboardJob::new(Puzzle5dClipboardVerb::Cut, request, &interaction))
            }
            "paste" => {
                let args = match &request.input {
                    ArtifactReservedToolInput::Action { args, .. } => args.clone(),
                    _ => return Err(Fault::from("puzzle5d paste requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dPasteJob::new(request, args.map(|value| semio_framework_pack_json::from_dsl_value(&value))))
            }
            "import-media" => {
                let (port, media) = match &request.input {
                    ArtifactReservedToolInput::Media { port, media } => (port.clone(), media.clone()),
                    _ => return Err(Fault::from("puzzle5d import-media requires media input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dImportJob::new(request, port, media))
            }
            _ => return Ok(None),
        };
        Ok(Some(job))
    }

    /// 📎 Ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d: replaces the old
    /// `crate::editor::puzzle5d::config::schema::register_app_schema()` self-registering call, which
    /// puzzle's plugin root used to reach `.setup()` for — `register_document_app`/`document_app`
    /// now call this automatically the moment `Puzzle5dPlayApp` is bound to a plugin, exactly like
    /// `🗒️note`'s own `app_schema` override.
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::puzzle5d::config::schema::app_schema_descriptor())
    }

    /// 🚀️ Boots on `default_document()` and on NOTHING else.
    ///
    /// 🐛️ This used to pre-warm three statics before returning: `NAKAGIN_EXAMPLE_DOCUMENT` (168 355 B
    /// of DSL), `CAPSULE_DREAM_EXAMPLE_DOCUMENT` (3 035 200 B of DSL → ~3.5 MB of JSON → a typed
    /// `Puzzle5dDocument` of 2 880 parts) and `PUZZLE5D_EXAMPLE_OPERATIONS` — the 4×4 matrix of
    /// PAIRWISE semantic diffs between all four example documents, i.e. sixteen diffs, several of them
    /// between a 2 880-part and a 180-part document. Every mount of this app paid all of it before its
    /// first frame, which is why a pane took seconds to boot and why this crate's own example-switch
    /// and retained-import laws ran for over a minute each (the 2026-09-21 test binary was killed by
    /// the 10-minute watchdog). None of it is needed to produce the boot document, and nothing is lost:
    /// each static is a `LazyLock`, forced only when its example is switched to. That matrix has since been
    /// deleted outright.
    fn initial_snapshot() -> Puzzle5dPlaySnapshot {
        Puzzle5dPlaySnapshot::new(puzzle5d_snapshot_from_document(&default_document()).expect("authored initial document admits"))
    }

    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::Kit, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        let (part_ids, fastener_ids) = puzzle5d_interaction_part_and_fastener_ids(interaction);
        puzzle5d_copy_fragment(doc.snapshot, &part_ids, &fastener_ids)
    }

    /// ✂️ B1: `ArtifactApp::cut_operations`'s signature carries no config output channel (it
    /// returns a bare `Vec<Self::Mutation>`, not an `Emit`), so this can only emit the document
    /// removal; clearing the selection is left to the framework's own post-cut selection reconciliation
    /// (the cut parts/fasteners are gone from the document either way, so a stale selection referencing
    /// them is inert until the next real selection action overwrites it).
    fn cut_operations(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, interaction: &InteractionView<'_>) -> Vec<Puzzle5dMutation> {
        let (part_ids, fastener_ids) = puzzle5d_interaction_part_and_fastener_ids(interaction);
        puzzle5d_cut_operations(doc.snapshot, &part_ids, &fastener_ids)
    }

    /// 📋️ B1: `ArtifactApp::paste_operations` carries no `ConfigView` at all (only `doc`/
    /// `fragment`/`placement`), so the new selection can't be threaded through this call; a following
    /// `setSelection` command (which the host already issues after a paste in practice) is what
    /// actually selects the pasted parts now.
    fn paste_operations(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle5dMutation>, ClipboardError> {
        let expected = Self::clipboard_media_type().unwrap_or(MediaType { class: MediaClass::Kit, form: MediaForm::Design });
        if fragment.media_type != expected {
            return Err(ClipboardError::IncompatibleMediaType(fragment.media_type));
        }
        let fragment_value: Value = parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_parts: Vec<Puzzle5dPart> = puzzle5d_record_from_projection(fragment_value.get("parts").cloned().unwrap_or_else(|| semio_framework_pack_json::json!([]))).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_fasteners: Vec<Puzzle5dFastener> = puzzle5d_record_from_projection(fragment_value.get("fasteners").cloned().unwrap_or_else(|| semio_framework_pack_json::json!([]))).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let before = puzzle5d_editor_projection(&doc.snapshot);
        let document: Puzzle5dDocument = <Puzzle5dDocument as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&before)).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let delta = paste_delta_2d(&fragment_parts, &document.parts, placement);
        let (fresh_parts, fresh_fasteners) = paste_selection_local(&document, &fragment_parts, &fragment_fasteners, delta);
        puzzle5d_paste_mutations(fresh_parts, fresh_fasteners).map_err(|error| ClipboardError::ParseFailed(error.to_string()))
    }

    /// 🏷️ Maps each `Puzzle5dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle5dCommand) -> &'static str {
        command.action_id()
    }

    /// 📢️ The localized notices of this guest's refusal codes ([`puzzle5d_fault_notices`]).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        puzzle5d_fault_notices()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        if let Some(flag) = puzzle5d_flag_value_argument(action) {
            args.and_then(|value| value.get(flag)).and_then(semio_framework_value::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("puzzle5d.action.flag-value-required"), format!("action '{action}' requires the boolean '{flag}' it sets")))?;
        }
        let args = args.map(semio_framework_pack_json::from_dsl_value);
        let window_id = args.as_ref().and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(Value::as_str).map(str::to_string);
        Puzzle5dCommand::try_from_action(action, args, window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 5D action '{action}'")))
    }

    /// 🧩️ Thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple `handle_action_impl` expects from the typed `Puzzle5dCommand`.
    fn handle(
        command: &Puzzle5dCommand,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle5dMutation, Puzzle5dConfigMutation, Self::DraftMutation>, Fault> {
        let selection = interaction.selection(PUZZLE5D_INTERACTION_DOMAIN);
        let window_id = view_state.and_then(|view| view.window_id.as_deref()).or_else(|| command.window_id()).unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
        Ok(puzzle5d_handle_action::<Self>(authoring_seed, command.action_id(), command.args(), command.window_id(), doc.snapshot, &runtime, view_state, selection, None, None).0)
    }

    /// 🕹️ `vortex` domain topology (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    /// parts and fasteners as flat roots, grips nested under their owning part (mirrors puzzle3d's
    /// object→vortex-marker nesting).
    fn interaction_topology(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let document: Puzzle5dDocument = puzzle5d_document_from_snapshot(doc.snapshot.typed()).expect("admitted snapshot projects to native editor document");
        let mut ordered = Vec::new();
        for part in &document.parts {
            ordered.push(semio_framework_plugin::TopologyNode { id: part.id.clone(), granularity: PUZZLE5D_GRANULARITY_PART.into(), parent: None });
            for grip in &part.grips {
                ordered.push(semio_framework_plugin::TopologyNode { id: puzzle5d_grip_full_id(&part.id, &grip.id), granularity: PUZZLE5D_GRANULARITY_GRIP.into(), parent: Some(part.id.clone()) });
            }
        }
        for fastener in &document.fasteners {
            ordered.push(semio_framework_plugin::TopologyNode { id: fastener.id.clone(), granularity: PUZZLE5D_GRANULARITY_FASTENER.into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(PUZZLE5D_INTERACTION_DOMAIN.to_string(), semio_framework_plugin::DomainTopology { ordered });
        semio_framework_plugin::InteractionTopology { domains }
    
})())
}

    /// 🔌️ Declares puzzle5d's typed media I/O surface: the implicit document ports (from
    /// `.document([...])`/`.artifact_kind(...)` in `create_puzzle5d_app`) plus `kit:in` (accepting a
    /// `kit.catalog` fragment shaped like block3d's `puzzle3d_catalog_fragment`, fanning IN from
    /// potentially many producers) and `design:out` (this app's own `5d.puzzle` design artifact, fanning
    /// OUT to potentially many consumers).
    fn io() -> Option<AppIo> {
        let io = ::semio_framework_async::poll::resolve_ready(AppIo::from_artifact(
            "puzzle.5d",
            MediaType { class: MediaClass::Kit, form: MediaForm::Design },
            ArtifactPresentation { id: "5d.puzzle".into(), name: "5D Puzzle".into(), dimension: "5d".into(), component_kind: "puzzle5d".into() },
        ));
        Some(::semio_framework_async::poll::resolve_ready(io.with_ports(vec![
            MediaPortSpec {
                id: "kit:in".into(),
                label: "Kit Catalog".into(),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                kind_id: Some("kit.catalog".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
            MediaPortSpec {
                id: "design:out".into(),
                label: "5D Puzzle Design".into(),
                direction: MediaPortDirection::Out,
                // 🔁️ Reuses the exact `id`/`media_type` already declared on the artifact's own
                // `artifact_kind()` — the same design artifact this app's document already
                // publishes, just exposed as an explicit workflow output port.
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
                kind_id: Some("5d.puzzle".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
        ])))
    }

    /// 🧵️ The synchronous editor callback is deliberately closed: production import enters only
    /// remains batch-only until an artifact-lane preparation owner can retire its publication roots.
    fn import_media(_port: &str, _media: &Media, _doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>) -> Result<Emit<Puzzle5dMutation, Puzzle5dConfigMutation, Self::DraftMutation>, MediaError> {
        Err(MediaError::NotImplemented)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let window_for_body = if body_key == board2d::BODY_KEY { crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID } else { crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID };
        let window_id = view_state.window_id.as_deref().unwrap_or(window_for_body);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let labels = puzzle5d_labels(view_state).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle5d has no authored label set for the host's locale/terminology axes"))?;
        // 🪟️ One `TreeWindows` per render, read off the host's `ViewModel::tree_windows` for exactly
        // the body being rendered — every panel container below shares its first-paint row budget.
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            board2d::BODY_KEY => board2d::render(&envelope, labels, None),
            world3d::BODY_KEY => world3d::render(&envelope, labels, doc.tool_run(), &crate::editor::puzzle5d::precompute::puzzle5d_mesh_lane(doc.snapshot, &envelope.document), None),
            artifact_panel::BODY_KEY => artifact_panel::render(&envelope, labels, &windows),
            catalogue::BODY_KEY => catalogue::render(&envelope, labels, &windows),
            inspection::BODY_KEY => inspection::render(&envelope, labels, &windows),
            settings_panel::BODY_KEY => settings_panel::render(&envelope, labels, settings_panel::panel_window_id(view_state)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d unknown-body label admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or_else(|| if body_key == board2d::BODY_KEY { crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID } else { crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID });
        let window_id = view_state.window_id.as_deref().unwrap_or(window_kind);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::transient_from_view(transient), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        // 🕹️ The ONE live read of the framework-owned `vortex` domain: both panes' paint, the gumball
        // and the inspection panel's per-entity field groups all render against this snapshot.
        let envelope = scene_from_projection_with_interaction(&projection, runtime, &active_utility, Puzzle5dInteractionSnapshot::from_interaction(interaction));
        let labels = puzzle5d_labels(view_state).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle5d has no authored label set for the host's locale/terminology axes"))?;
        // 🪟️ One `TreeWindows` per render, read off the host's `ViewModel::tree_windows` for exactly
        // the body being rendered — every panel container below shares its first-paint row budget.
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        // 🎣️ What the brush suggestions run resolved for the open menu's grip — the instance owner holds it, and both
        // panes list it.
        let menu_target = envelope.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.as_str());
        let suggestions = menu_target.and_then(|target| owner.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(owner.brush_suggestions.found(target).cloned())).ok().flatten());
        let node = match body_key {
            board2d::BODY_KEY => board2d::render(&envelope, labels, suggestions.as_ref()),
            world3d::BODY_KEY => world3d::render(&envelope, labels, doc.tool_run(), &crate::editor::puzzle5d::precompute::puzzle5d_mesh_lane(doc.snapshot, &envelope.document), suggestions.as_ref()),
            artifact_panel::BODY_KEY => artifact_panel::render(&envelope, labels, &windows),
            catalogue::BODY_KEY => catalogue::render(&envelope, labels, &windows),
            inspection::BODY_KEY => inspection::render(&envelope, labels, &windows),
            settings_panel::BODY_KEY => settings_panel::render(&envelope, labels, settings_panel::panel_window_id(view_state)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d unknown-body label admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        HashMap::from([(window_id.to_string(), edit::puzzle5d_engagement(&envelope, window_kind, labels, doc.tool_run()))])
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::transient_from_view(transient), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&puzzle5d_editor_projection(&doc.snapshot), runtime, &active_utility);
        HashMap::from([(window_id.to_string(), edit::puzzle5d_engagement(&envelope, window_kind, labels, doc.tool_run()))])
    }

    fn window_measures(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let measures = if window_kind == crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID { board2d::window_measures(&envelope, labels) } else { world3d::window_measures(&envelope, labels) };
        HashMap::from([(window_id.to_string(), measures)])
    }

    /// 🛠️ The mode-level tool options rail: the fill tool's count entry and distribution trees, live against
    /// this instance's run so the count reads `loading` while the run is working.
    fn tool_measures(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let window_id = view_state.window_id.as_deref().unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&puzzle5d_editor_projection(&doc.snapshot), runtime, &active_utility);
        HashMap::from([(fill_tool::TOOL_ID.to_string(), fill_tool::measures(&envelope, labels, doc.tool_run()))])
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle5dInteractionSnapshot::default(), registry)
    }

    /// 🕹️ The live-`vortex`-domain twin: a grip or fastener selected in EITHER pane reaches the menu even though
    /// the world host's surface only reports its painted part ids.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle5dInteractionSnapshot::from_interaction(interaction), registry)
    }
}
//#endregion 🔖️PlayApp

//#region 🔖️Manifest




/// 📢️ The localized notice of every refusal code the puzzle5d guest names (design §20.12): a set-verb's missing flag value.
fn puzzle5d_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: LazyLock<[(&str, LocalizedLabel); 1]> = LazyLock::new(|| {
        [
            ("puzzle5d.action.flag-value-required", LocalizedLabel::native("Choose on or off for this setting.", "Für diese Einstellung Ein oder Aus wählen.")),
        ]
    });
    &*NOTICES
}

/// 🗨️ The one `partKind` select both the standalone `addPartKind` arg form and the "Add Part" dialog
/// declare — built twice from the same catalog so the two forms can never drift apart.

/// 🙈️ The flag a set-verb sets to exactly the boolean its arguments carry (`setSelectionHidden{hidden}`,
/// `setTargetVolumeLocked{locked}`, …) — the row target's explicit next state, so a stale view sets a value and never flips one.
fn puzzle5d_flag_value_argument(action: &str) -> Option<&'static str> {
    match action {
        "setSelectionHidden" | "setTargetVolumeHidden" => Some("hidden"),
        "setSelectionLocked" | "setTargetVolumeLocked" => Some("locked"),
        _ => None,
    }
}

/// 🙈️ A set-verb's definition: the explicit identity a row names (or, left empty, the live selection) and the REQUIRED
/// boolean `flag` it sets — a missing value is refused, never defaulted.
fn puzzle5d_flag_value_action(id: &str, identity: Vec<ActionArgDef>, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    let mut args = identity;
    args.push(ActionArgDef::toggle(flag, value).required());
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation).with_category("settings").with_args(args) }
}

fn puzzle5d_part_kind_arg() -> ActionArgDef {
    let options = puzzle5d_part_kind_options();
    let default = puzzle5d_default_part_kind(&options);
    ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), options).default_value(&default)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain declaration —
/// one granularity per previously-distinct `Puzzle5dSelection` bag (part/grip/fastener). `Topology`
/// hierarchy (see `Puzzle5dPlayApp::interaction_topology`) exposes the part→grip nesting.
fn puzzle5d_interaction_definition() -> InteractionDefinition {
    let granularity = |id: &str, label: LocalizedLabel, icon: &str| GranularityDefinition { id: id.into(), label, icon_id: icon.into() };
    InteractionDefinition {
        id: PUZZLE5D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Vortex", "Vortex"),
        granularities: vec![
            granularity(PUZZLE5D_GRANULARITY_PART, puzzle5d_localized(|l| l.part), "box"),
            granularity(PUZZLE5D_GRANULARITY_GRIP, puzzle5d_localized(|l| l.grip), "circle-dot"),
            granularity(PUZZLE5D_GRANULARITY_FASTENER, LocalizedLabel::native("Fastener", "Verbinder"), "link"),
            granularity(PUZZLE5D_GRANULARITY_TARGET_VOLUME, LocalizedLabel::native("Target Volume", "Zielvolumen"), "box-select"),
        ],
        hierarchy: HierarchyProvider::Topology,
        hover: HoverSpec { enabled: true, transitive: false, channels: vec![PUZZLE5D_HOVER_CHANNEL.into()], broadcast: true },
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}

/// 🚧️ SDK note (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4): `EditorBuilder`
/// has no `.example(...)`/`.workflow(...)` methods — `.editor::<E>(def: AppDefinition)` only takes the
/// bare definition, `App.examples` is discarded by the plugin-root builder. The three examples this
/// app used to register (`PUZZLE5D_EXAMPLE_CONCRETE_FOREST`/`_NAKAGIN`/`_CAPSULE_DREAM`) and the
/// `"puzzle5d"` workflow tag are DROPPED here, not silently ported — the constants/JSON statics stay
/// live for `setActiveExample`'s own dispatch path (`🎮️commands/🛍️set-active-example`), only the
/// manifest-level registration is gone.
pub fn create_puzzle5d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(Puzzle5dPlayApp::DIALECT)
            .document(["semio", "puzzle", "5d"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "5d"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE5D_PLAY_MODE_EDIT)
            .window_kind_def(board2d::definition())
            .window_kind_def(world3d::definition())
            .interaction(puzzle5d_interaction_definition())
            .window_kind_interactions(crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE5D_INTERACTION_DOMAIN)])
            .window_kind_interactions(crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE5D_INTERACTION_DOMAIN)])
            .window_kind_action_refs(crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID, vec![board2d::actions::apply_board_events::reference(), board2d::actions::set_camera::reference()])
            .window_kind_action_refs(
                crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID,
                vec![
                    world3d::actions::translate_selection::reference(),
                    world3d::actions::rotate_selection::reference(),
                    world3d::actions::scale_selection::reference(),
                    world3d::actions::world_relocate::reference(),
                    world3d::actions::set_camera::reference(),
                ],
            )
            // 🏗️ 2D-first 40/60 split — board pane left (diagram 40%), world pane right (scene 60%).
            .default_layout(edit::layout())
            .panel_tab_def(artifact_panel::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            .panel_tab_def(settings_panel::definition())
            .keybinding("escape", "engagementAbort")
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateSelection")
            .keybinding("tab", "cycleBrushCandidate")
            .keybinding("shift+tab", "cycleBrushCandidateBack")
            .keybinding("f", "focusSelection")
            // 🔧️ Document-mutating operations (emit VCS operations through the before/after document delta).
            // 📤️📥️ Document IO. `exportSnapshot`/`openImportSnapshot` are SHELL verbs (a download, a file
            // picker — no document mutation of their own); `importSnapshot` is the mutation the picker
            // re-dispatches once per wire page and never a menu row of its own, so the user-facing
            // "Import" row is the one that actually opens a picker.
            .action_with(ActionDefinition::bounded_catalog("exportSnapshot", puzzle5d_localized(|l| l.export), ActionKind::Shell).with_category("file"))
            .action_with(ActionDefinition::bounded_catalog("openImportSnapshot", puzzle5d_localized(|l| l.import), ActionKind::Shell).with_category("file"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importSnapshot", puzzle5d_localized(|l| l.import), ActionKind::Mutation) })
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            // 🗨️ Shell-only effect: opens the declared "addPart" dialog, whose submit dispatches
            // `addPartKind`. The parametrized verb itself is NOT a palette row — its `partKind` select IS
            // the dialog, and offering both published two rows carrying the same "Add Part" label, the
            // second one opening a bare action pane (puzzle 3d's own measured defect).
            .action_with(ActionDefinition::bounded_catalog("openAddPartDialog", puzzle5d_localized(|l| l.add_part_prompt), ActionKind::Shell).with_category("create"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("addPartKind", puzzle5d_localized(|l| l.add_part), ActionKind::Mutation).with_category("create") })
            .mutation("addBrushPart", LocalizedLabel::native("Add Brush Part", "Pinselteil hinzufügen"))
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteSelection")
            .action_with(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).with_category("create"))
            .action_with(ActionDefinition::bounded_catalog("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(puzzle5d_flag_value_action("setSelectionHidden", vec![ActionArgDef::text("entity", LocalizedLabel::native("Entity", "Entität")), ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs"))], "hidden", LocalizedLabel::native("Set Hidden", "Verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle5d_flag_value_action("setSelectionLocked", vec![ActionArgDef::text("entity", LocalizedLabel::native("Entity", "Entität")), ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs"))], "locked", LocalizedLabel::native("Set Locked", "Gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .action_with(ActionDefinition::bounded_catalog("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"), ActionKind::Mutation).with_category("view"))
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .mutation("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"))
            .mutation("patchPart", LocalizedLabel::native("Patch Part", "Teil aktualisieren"))
            .mutation("patchGrip", LocalizedLabel::native("Patch Grip", "Griff aktualisieren"))
            .mutation("patchFastener", LocalizedLabel::native("Patch Fastener", "Verbinder aktualisieren"))
            .mutation("createFastener", LocalizedLabel::native("Create Fastener", "Verbinder erstellen"))
            .mutation("deleteFastener", LocalizedLabel::native("Delete Fastener", "Verbinder löschen"))
            .action_destructive("deleteFastener")
            .mutation("retargetFastener", LocalizedLabel::native("Retarget Fastener", "Verbinder umhängen"))
            .mutation("editFastener", LocalizedLabel::native("Edit Fastener", "Verbinder bearbeiten"))
            .mutation("proximityConnect", LocalizedLabel::native("Proximity Connect", "Näherungsverbinden"))
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .mutation("worldRelocate", LocalizedLabel::native("Relocate Part", "Teil verlagern"))
            .mutation("addTargetVolume", LocalizedLabel::native("Add Target Volume", "Zielvolumen hinzufügen"))
            .action_with(ActionDefinition::bounded_catalog("deleteTargetVolume", LocalizedLabel::native("Delete Target Volume", "Zielvolumen löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteTargetVolume")
            .action_with(ActionDefinition::bounded_catalog("setTargetVolumeFlag", LocalizedLabel::native("Set Target Volume Flag", "Zielvolumen-Markierung festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(puzzle5d_flag_value_action("setTargetVolumeHidden", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "ID"))], "hidden", LocalizedLabel::native("Set Target Volume Hidden", "Zielvolumen verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle5d_flag_value_action("setTargetVolumeLocked", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "ID"))], "locked", LocalizedLabel::native("Set Target Volume Locked", "Zielvolumen gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .mutation("relocateTargetVolume", LocalizedLabel::native("Relocate Target Volume", "Zielvolumen verlagern"))
            .mutation("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"))
            // 👁️ Ephemeral view state — selection, hover, utility parameters, brush cycling, camera pose.
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setCamera2d", LocalizedLabel::native("Set Camera 2D", "Kamera 2D festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setCamera3d", LocalizedLabel::native("Set Camera 3D", "Kamera 3D festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::bounded_catalog("selectSameKindSelection", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection"))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand"))
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand"))
            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View, "hand"))
            .view_action("cycleBrushCandidate", LocalizedLabel::native("Cycle Brush Candidate", "Pinselkandidat wechseln"))
            .view_action("cycleBrushCandidateBack", LocalizedLabel::native("Cycle Brush Candidate Back", "Pinselkandidat rückwärts wechseln"))
            .view_action("targetBrushSuggestions", LocalizedLabel::native("Target Brush Suggestions", "Pinselvorschläge ausrichten"))
            .view_action("openVortexSuggestions", LocalizedLabel::native("Open Grip Suggestions", "Griff-Vorschläge öffnen"))
            .view_action("closeVortexSuggestions", LocalizedLabel::native("Close Grip Suggestions", "Griff-Vorschläge schließen"))
            .view_action("hoverSuggestion", LocalizedLabel::native("Hover Suggestion", "Vorschlag überfahren"))
            .action_audience("hoverSuggestion", semio_framework_plugin::CapabilityAudience::Input)
            .mutation("acceptSuggestion", LocalizedLabel::native("Accept Suggestion", "Vorschlag annehmen"))
            .view_action("registerBrushMesh", LocalizedLabel::native("Register Brush Mesh", "Pinsel-Mesh registrieren"))
            .view_action("setBrushPlacementContactTolerance", LocalizedLabel::native("Set Brush Placement Contact Tolerance", "Pinsel-Kontakttoleranz festlegen"))
            .view_action("setProximityRadius", LocalizedLabel::native("Set Proximity Radius", "Näherungsradius festlegen"))
            .view_action("setChunkSize", LocalizedLabel::native("Set Chunk Size", "Blockgröße festlegen"))
            .view_action("setPartKindWeight", LocalizedLabel::native("Set Part Kind Weight", "Teileart-Gewicht festlegen"))
            .view_action("setGripKindWeight", LocalizedLabel::native("Set Grip Kind Weight", "Griffart-Gewicht festlegen"))
            .action_with(ActionDefinition::new("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"), ActionKind::View, "layers"))
            .view_action("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"))
            .view_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"))
            .action_with(ActionDefinition::new("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View, "grid-3x3"))
            .action_with(ActionDefinition::new("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View, "grid-3x3"))
            .action_with(ActionDefinition::new("setVoxelDims", LocalizedLabel::native("Set Voxel Dims", "Voxelmaße festlegen"), ActionKind::View, "box"))
            .action_with(ActionDefinition::new("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster anzeigen"), ActionKind::View, "layout-grid"))
            .action_with(ActionDefinition::new("setGridSpacing", LocalizedLabel::native("Set Grid Spacing", "Rasterabstand festlegen"), ActionKind::View, "grid-3x3"))
            .action_with(ActionDefinition::new("setProjection", LocalizedLabel::native("Set Projection", "Projektion festlegen"), ActionKind::View, "video"))
            .action_with(ActionDefinition::new("setProjectionParam", LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"), ActionKind::View, "video"))
            .action_with(ActionDefinition::new("setGripShow", LocalizedLabel::native("Set Grip Markers", "Griffmarken festlegen"), ActionKind::View, "circle-dot"))
            .action_with(ActionDefinition::new("setGripDirection", LocalizedLabel::native("Set Grip Direction", "Griffrichtung festlegen"), ActionKind::View, "compass"))
            .action_with(ActionDefinition::new("setSelectableKind", LocalizedLabel::native("Set Selectable Kind", "Auswählbare Art festlegen"), ActionKind::View, "mouse-pointer").with_category("selection"))
            .action_with(ActionDefinition::new("setLodAutomatic", LocalizedLabel::native("Set Lod Automatic", "LOD automatisch"), ActionKind::View, "zoom-in"))
            .action_with(ActionDefinition::new("setLodDepthVariable", LocalizedLabel::native("Set Lod Depth Variable", "LOD tiefenabhängig"), ActionKind::View, "layers"))
            .action_with(ActionDefinition::new("setLodManual", LocalizedLabel::native("Set Lod Manual", "LOD manuell festlegen"), ActionKind::View, "layers"))
            .action_with(ActionDefinition::new("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transformationsgriff umschalten"), ActionKind::View, "move-3d"))
            .action_audience("setTransformGumballFlag", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_with(ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("worldPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("addBrushPart", InteractiveJobClassification::Migrated)
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("addPartKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("applyBoardEvents", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("createFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidate", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidateBack", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("editFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementControlSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportSnapshot", InteractiveJobClassification::Migrated)
            .action_destructive("exportSnapshot")
            .action_interactive_job("importSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("openAddPartDialog", InteractiveJobClassification::Migrated)
            .action_interactive_job("openImportSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchGrip", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchPart", InteractiveJobClassification::Migrated)
            .action_interactive_job("proximityConnect", InteractiveJobClassification::Migrated)
            .action_interactive_job("registerBrushMesh", InteractiveJobClassification::Migrated)
            .action_interactive_job("retargetFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectSameKindSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementContactTolerance", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProximityRadius", InteractiveJobClassification::Migrated)
            .action_interactive_job("setChunkSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera2d", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera3d", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillCount", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridFactor", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSnapEnabled", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridVisible", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSpacing", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjectionParam", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGripShow", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGripDirection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectableKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodAutomatic", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodDepthVariable", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodManual", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("setPartKindWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSuggestionOffset", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGripKindWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("targetBrushSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("openVortexSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("closeVortexSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("hoverSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("acceptSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("addTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("relocateTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("setVoxelDims", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldRelocate", InteractiveJobClassification::Migrated)
            // 📝️ Staged argument forms for the brush create actions (P1).
            .action_args("addPartKind", vec![puzzle5d_part_kind_arg()])
            // 🗨️ The dialog `openAddPartDialog` opens, driving the SAME `addPartKind` operation and the
            // SAME live `partKind` select the arg form declares.
            .dialog(
                DialogDefinition::new(open_add_part_dialog::PUZZLE5D_ADD_PART_DIALOG, puzzle5d_localized(|l| l.add_part), semio_framework_plugin::ActionRef::new("addPartKind"))
                    .body(puzzle5d_localized(|l| l.add_part_body))
                    .args(vec![puzzle5d_part_kind_arg().required()])
                    .submit_label(puzzle5d_localized(|l| l.add)),
            )
            .action_args("addBrushPart", vec![
                ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), vec![ActionArgOption::new("Part", puzzle5d_localized(|l| l.part))]).default_value(&"Part"),
            ])
            // 🧰️ Flat per-window set of utilities; `select` is the default. Each `🪛️utilities/*` node
            // owns its own id/definition; a utility bound by BOTH windows is declared once (under the
            // 2D window) and referenced by the 3D window's `definition()`.
            .utility(board2d::utilities::select::definition(puzzle5d_localized(|l| l.select)))
            .utility(crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::definition())
            .utility(board2d::utilities::brush::definition(puzzle5d_localized(|l| l.brush)))
            .utility(world3d::utilities::volume_brush::definition(puzzle5d_localized(|l| l.volume_brush)))
            .utility(world3d::utilities::world_relocate::definition())
            // 🛠️ Fill is a mode-level TOOL (a whole-document generator over both projections), not a window
            // utility — it keeps its viewport interaction in both panes through the host's `active_tool_id`.
            .tool(fill_tool::definition(puzzle5d_localized(|l| l.fill)))
            .mode_tools(edit::PUZZLE5D_PLAY_MODE_EDIT, vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(fill_tool::TOOL_ID))])
    .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 5D puzzle with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte 5D-Puzzle durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
    .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected part from the 5D puzzle together with its connections.", "Löscht alle ausgewählten Teile samt ihrer Verbindungen aus dem 5D-Puzzle."))
    .action_describe("duplicateSelection", LocalizedLabel::native("Adds a copy of every selected part next to the original; nothing happens without a selection.", "Fügt neben jedem ausgewählten Teil eine Kopie hinzu; ohne Auswahl geschieht nichts."))
    .action_describe("focusSelection", LocalizedLabel::native("Frames the camera on the selected parts; only the view changes.", "Richtet die Kamera auf die ausgewählten Teile aus; nur die Ansicht ändert sich."))
    .action_describe("translateSelection", LocalizedLabel::native("Moves the selected parts by dx and dy.", "Verschiebt die ausgewählten Teile um dx und dy."))
    .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected parts by the given angle.", "Dreht die ausgewählten Teile um den angegebenen Winkel."))
    .action_describe("scaleSelection", LocalizedLabel::native("Scales the selected parts by the given factor.", "Skaliert die ausgewählten Teile um den angegebenen Faktor."))
    .action_describe("exportSnapshot", LocalizedLabel::native("Writes the whole 5D puzzle as JSON to a downloaded file named after the active example on the user's machine.", "Schreibt das gesamte 5D-Puzzle als JSON in eine heruntergeladene, nach dem aktiven Beispiel benannte Datei auf dem Rechner des Nutzers."))
    .action_describe("openImportSnapshot", LocalizedLabel::native("Opens the host's file picker for a 5D puzzle JSON file; the chosen file then replaces the whole puzzle.", "Öffnet die Dateiauswahl des Hosts für eine 5D-Puzzle-JSON-Datei; die gewählte Datei ersetzt dann das gesamte Puzzle."))
    .action_describe("importSnapshot", LocalizedLabel::native("Replaces the whole 5D puzzle with one read from an imported JSON file; the previous puzzle is discarded.", "Ersetzt das gesamte 5D-Puzzle durch eines aus einer importierten JSON-Datei; das bisherige Puzzle wird verworfen."))
    .action_describe("setSelectionFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given or selected parts.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen oder ausgewählten Teile."))
    .action_describe("setSelectionHidden", LocalizedLabel::native("Sets the given or selected parts hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt die angegebenen oder ausgewählten Teile oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("setSelectionLocked", LocalizedLabel::native("Sets the given or selected parts locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt die angegebenen oder ausgewählten Teile oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("acceptSuggestion", LocalizedLabel::native("Places the suggested piece chosen from the suggestion list (by index, or the highlighted one) at its connection point.", "Setzt das aus der Vorschlagsliste gewählte Teil (per Index oder das hervorgehobene) an seinem Anschlusspunkt."))
    .action_describe("selectSameKindSelection", LocalizedLabel::native("Extends the selection to every piece of the same kind as the selected one.", "Erweitert die Auswahl auf alle Teile derselben Art wie das ausgewählte."))
    .action_describe("toggleSun", LocalizedLabel::native("Switches the 3D view's sun light on or off; only the view changes.", "Schaltet das Sonnenlicht der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the compass direction the 3D view's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne der 3D-Ansicht scheint; nur die Ansicht ändert sich."))
    .action_describe("setSunElevation", LocalizedLabel::native("Sets how high the 3D view's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne der 3D-Ansicht über dem Horizont steht; nur die Ansicht ändert sich."))
    .action_describe("setSunIntensity", LocalizedLabel::native("Sets the brightness of the 3D view's sun; only the view changes.", "Legt die Helligkeit der Sonne der 3D-Ansicht fest; nur die Ansicht ändert sich."))
    .action_describe("setLodAutomatic", LocalizedLabel::native("Turns automatic level of detail in the 3D view on or off; only the view changes.", "Schaltet die automatische Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setLodDepthVariable", LocalizedLabel::native("Turns depth-dependent level of detail in the 3D view on or off; only the view changes.", "Schaltet die tiefenabhängige Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setLodManual", LocalizedLabel::native("Sets a fixed level of detail for the 3D view; only the view changes.", "Legt eine feste Detailstufe für die 3D-Ansicht fest; nur die Ansicht ändert sich."))
    .action_describe("setGridVisible", LocalizedLabel::native("Shows or hides the placement grid in the 3D view; only the view changes.", "Blendet das Platzierungsraster in der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setGridSnapEnabled", LocalizedLabel::native("Turns snapping to the placement grid on or off for moves in the 3D view.", "Schaltet das Fangen am Platzierungsraster für Verschiebungen in der 3D-Ansicht ein oder aus."))
    .action_describe("setGridSpacing", LocalizedLabel::native("Sets the spacing of the placement grid.", "Legt den Abstand des Platzierungsrasters fest."))
    .action_describe("setProximityRadius", LocalizedLabel::native("Sets how close two pieces must come for a move or Connect Nearby to join their connection points.", "Legt fest, wie nahe sich zwei Teile kommen müssen, damit ein Verschieben oder In der Nähe verbinden ihre Anschlusspunkte verbindet."))
    .action_describe("setChunkSize", LocalizedLabel::native("Sets the size of the spatial chunks the 3D view loads and draws pieces in; only the view changes.", "Legt die Größe der räumlichen Blöcke fest, in denen die 3D-Ansicht Teile lädt und zeichnet; nur die Ansicht ändert sich."))
    .action_describe("setSelectableKind", LocalizedLabel::native("Sets whether pieces, connection points or attractions can be picked in the 3D view.", "Legt fest, ob Teile, Anschlusspunkte oder Anziehungen in der 3D-Ansicht gewählt werden können."))
    .action_describe("setTransformGumballFlag", LocalizedLabel::native("Switches one option of the transform gumball (such as translate, rotate or scale handles) on or off.", "Schaltet eine Option des Transformationsgriffs (etwa Verschiebe-, Dreh- oder Skaliergriffe) ein oder aus."))
    .action_describe("setVoxelDims", LocalizedLabel::native("Sets one dimension (width, depth or height) of the voxel volume the fill tool packs pieces into.", "Legt eine Abmessung (Breite, Tiefe oder Höhe) des Voxelvolumens fest, in das das Füllwerkzeug Teile packt."))
    .action_describe("relocateTargetVolume", LocalizedLabel::native("Moves one target volume to a new position.", "Verschiebt ein Zielvolumen an eine neue Position."))
    .action_describe("setFillCount", LocalizedLabel::native("Sets how many pieces the fill tool places into the target volume.", "Legt fest, wie viele Teile das Füllwerkzeug in das Zielvolumen setzt."))
    .action_describe("setBrushPlacementContactTolerance", LocalizedLabel::native("Sets how much overlap (0 to 1) the placement brush tolerates between a suggested piece and existing ones.", "Legt fest, wie viel Überlappung (0 bis 1) der Platzierungspinsel zwischen einem vorgeschlagenen und vorhandenen Teilen zulässt."))
    .action_describe("cycleBrushCandidate", LocalizedLabel::native("Switches the placement brush to the next suggested piece.", "Schaltet den Platzierungspinsel zum nächsten vorgeschlagenen Teil."))
    .action_describe("cycleBrushCandidateBack", LocalizedLabel::native("Switches the placement brush to the previous suggested piece.", "Schaltet den Platzierungspinsel zum vorherigen vorgeschlagenen Teil."))
    .action_describe("openVortexSuggestions", LocalizedLabel::native("Opens the list of pieces that fit at one connection point.", "Öffnet die Liste der Teile, die an einen Anschlusspunkt passen."))
    .action_describe("closeVortexSuggestions", LocalizedLabel::native("Closes the list of suggested pieces.", "Schließt die Liste der vorgeschlagenen Teile."))
    .action_describe("addTargetVolume", LocalizedLabel::native("Adds a target volume, a box region the fill tool packs pieces into.", "Fügt ein Zielvolumen hinzu, einen Quaderbereich, in den das Füllwerkzeug Teile packt."))
    .action_describe("deleteTargetVolume", LocalizedLabel::native("Deletes one target volume by id.", "Löscht ein Zielvolumen anhand seiner Id."))
    .action_describe("setTargetVolumeFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given target volumes.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen Zielvolumen."))
    .action_describe("setTargetVolumeHidden", LocalizedLabel::native("Sets the given target volume hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt das angegebene Zielvolumen oder zeigt es, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("setTargetVolumeLocked", LocalizedLabel::native("Sets the given target volume locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt das angegebene Zielvolumen oder entsperrt es, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("addNode", LocalizedLabel::native("Adds a part of the given kind (a 5D block kind) to the puzzle at x, y.", "Fügt dem Puzzle an x, y ein Teil der angegebenen Art (einer 5D-Blockart) hinzu."))
    .action_describe("proximityConnect", LocalizedLabel::native("Fastens every pair of compatible grips that lie within the proximity radius of each other.", "Verbindet jedes Paar verträglicher Griffe, die innerhalb des Näheradius beieinander liegen, mit Verbindern."))
    .action_describe("worldRelocate", LocalizedLabel::native("Moves one part to a new position and fastens it to nearby compatible grips.", "Verschiebt ein Teil an eine neue Position und verbindet es mit verträglichen Griffen in der Nähe."))
    .action_describe("openAddPartDialog", LocalizedLabel::native("Opens the Add Part dialog to pick a part kind to place.", "Öffnet den Dialog Teil hinzufügen, um eine zu setzende Teileart zu wählen."))
    .action_describe("addPartKind", LocalizedLabel::native("Adds a part of the given kind to the puzzle.", "Fügt dem Puzzle ein Teil der angegebenen Art hinzu."))
    .action_describe("addBrushPart", LocalizedLabel::native("Places a part of the given kind with the placement brush where the brush suggests.", "Setzt mit dem Platzierungspinsel ein Teil der angegebenen Art dort, wo der Pinsel es vorschlägt."))
    .action_describe("patchPart", LocalizedLabel::native("Sets one field on the given or selected parts.", "Setzt ein Feld auf den angegebenen oder ausgewählten Teilen."))
    .action_describe("patchGrip", LocalizedLabel::native("Sets one field on the given or selected grips.", "Setzt ein Feld auf den angegebenen oder ausgewählten Griffen."))
    .action_describe("patchFastener", LocalizedLabel::native("Sets one field on the given or selected fasteners.", "Setzt ein Feld auf den angegebenen oder ausgewählten Verbindern."))
    .action_describe("createFastener", LocalizedLabel::native("Creates a fastener joining two compatible grips of different parts.", "Erstellt einen Verbinder, der zwei verträgliche Griffe verschiedener Teile verbindet."))
    .action_describe("deleteFastener", LocalizedLabel::native("Deletes one fastener by id, releasing its two grips.", "Löscht einen Verbinder anhand seiner Id und gibt seine zwei Griffe frei."))
    .action_describe("retargetFastener", LocalizedLabel::native("Moves one end of a fastener to another compatible grip.", "Hängt ein Ende eines Verbinders an einen anderen verträglichen Griff um."))
    .action_describe("editFastener", LocalizedLabel::native("Sets the gap, shift or rise of one fastener, absolutely or by a delta.", "Setzt Abstand, Versatz oder Anhebung eines Verbinders, absolut oder um eine Differenz."))
    .action_describe("setPartKindWeight", LocalizedLabel::native("Sets how often the fill tool and brush pick one part kind relative to the others.", "Legt fest, wie oft Füllwerkzeug und Pinsel eine Teileart im Verhältnis zu den anderen wählen."))
    .action_describe("setGripKindWeight", LocalizedLabel::native("Sets how strongly one grip kind is preferred when suggesting placements.", "Legt fest, wie stark eine Griffart bei Platzierungsvorschlägen bevorzugt wird."))
    .action_describe("setLodMode", LocalizedLabel::native("Sets the level-of-detail mode of the views; only the view changes.", "Legt den Detailstufenmodus der Ansichten fest; nur die Ansicht ändert sich."))
    .action_describe("setSuggestionOffset", LocalizedLabel::native("Sets how far suggested parts are offset from the grip they would attach to.", "Legt fest, wie weit vorgeschlagene Teile vom Griff, an den sie ansetzen würden, versetzt werden."))
    .action_describe("setGridFactor", LocalizedLabel::native("Sets the spacing factor of the 2D view's snap grid; only the view setting changes.", "Legt den Abstandsfaktor des Fangrasters der 2D-Ansicht fest; nur die Ansichtseinstellung ändert sich."))
    .action_describe("setGripShow", LocalizedLabel::native("Sets whether grip markers are shown always or only on selected parts; only the view changes.", "Legt fest, ob Griffmarken immer oder nur an ausgewählten Teilen gezeigt werden; nur die Ansicht ändert sich."))
    .action_describe("setGripDirection", LocalizedLabel::native("Sets how grip directions are drawn; only the view changes.", "Legt fest, wie Griffrichtungen gezeichnet werden; nur die Ansicht ändert sich."))
    .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("setProjection", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("setProjectionParam", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("engagementRepeatLast", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("engagementControlSelect", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("targetBrushSuggestions", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("registerBrushMesh", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("setCamera2d", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("setCamera3d", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("applyBoardEvents", semio_framework_plugin::CapabilityAudience::Input)
    .action_destructive("importSnapshot")
    .build_definition()
}

// 🗂️ `Puzzle5dPlaySnapshot`'s pack<->dsl codec (so `framework/sync`'s `FolderEndpoint::Pack` can
// print/parse puzzle-5d play documents without depending on this crate's concrete
// `Projection`/`Mutation` types) is now declared via `.document_codec::<Puzzle5dPlayApp>()` on
// `crate::declaration()` (ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`
// M1) — the old side-effecting `register_puzzle5d_exports()` wrapper (this app file's only caller of
// `register_document_codec_for_app`) is gone. The 5d mesh export/import OS-host registration
// (`register_mesh_io()`/`puzzle5d_document_from_mesh`) was never rewired to a real `.setup()` caller
// after the artifacts-only-plugin-architecture migration (`🧩️puzzle/🦀️.rs`'s `plugin()`
// builder chain has no `.setup()` call at all) and was deleted as dead code (ticket
// 26/08/17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS) — same fate as puzzle3d's
// sibling mesh bridge. Mesh export/import should be re-derived from `io_dispatch`'s real
// `ComposerEntry` chain if/when this bridge is needed again.
//#endregion 🔖️Manifest

//#region 🧪️UnitTests





//#endregion 🧪️UnitTests







//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{PUZZLE_5D_SCHEMA};

//#region 🔖️ArtifactKind
/// 🗿️ The `5d.puzzle` artifact kind — lifted out of the pre-consolidation manifest builder chain so
/// the artifact, not the app, owns its own identity.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "5d.puzzle".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("5D Puzzle", "5D-Puzzle"),
        source_format: "puzzle.5d".into(),
        component_kind: "puzzle5d".into(),
        dimension: "5d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Design },
        schema: PUZZLE_5D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.png".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.zip".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems4
