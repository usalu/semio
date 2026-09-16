//! 🖥️ Fem2d play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, view state in concrete window config, shared compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `Fem2dCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one passthrough per node (fem2d's mode/window declarations stay
//! scalar/inline — no `mode_def`/`window_kind_def` object is built anywhere in the pre-migration code).

use crate::app_surface::ResultDisplay;
use crate::editor::fem2d::commands::{
    add_area_load, add_bar, add_beam, add_combination, add_load_case, add_material, add_member_udl, add_nodal_load, add_node, add_region, add_section, add_support, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, focus_entity, patch_combination,
    patch_element, patch_load, patch_load_case, patch_material, patch_node, patch_region, patch_section, patch_support, remove_selection, result_animation_tick, set_active_example, set_analysis_settings, set_camera, set_result_animation, set_result_display,
    set_self_weight,
};
use crate::editor::fem2d::commands::gumball::{rotate_selection, scale_selection, set_transform_gumball_flag, translate_selection};
use crate::editor::fem2d::interaction::{fem2d_interaction_definition, Fem2dInteractionSnapshot, FEM2D_INTERACTION_DOMAIN};
use crate::editor::fem2d::panels::{artifact as artifact_panel, inspection as inspection_panel, results as results_panel};
use crate::editor::fem2d::terminology::fem2d_labels;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::fem2d::modes::edit;
use crate::editor::fem2d::modes::edit::windows::model as model_window;
use crate::editor::fem2d::modes::edit::windows::results as results_window;
use crate::model::{Dof, ElementResult};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::Fem2dSnapshot;
use dsl::json::Value;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{
    built_text_node, create_default_layout, ActionArgDef, ActionArgOption, ActionDescriptor, AppIo, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract,
    ArtifactToolPublicationLane, ArtifactView, ConfigSpec, ConfigView, DraftView, Editor, EditorApp, Emit, Fault, InteractionRef, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, NoDraft, NoDraftMutation,
    PluginAssemblyError, PluginCloseStep, UtilityCategory, UtilityDefinition, ViewModel, WindowMeasure,
};
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const FEM2D_APP_ID: &str = "fem2d-play";
/// 🎛️ Stable controller/action-factory tag every panel row and control binds its actions under
/// (the `ActionFactory` id, distinct from the derived surface app id) — the same string as the app id.
pub const FEM2D_PLAY_CONTROLLER_ID: &str = FEM2D_APP_ID;

/// 📦️ The `fem2d-play` "default" example — read directly by the `setActiveExample` handler
/// (`crate::editor::fem2d::commands::set_active_example`) and every test fixture (`EditorBuilder` has
/// no `.example(...)` registration — see the SDK-gap doc comment on `create_fem2d_app` below).
pub const FEM2D_EXAMPLE_DSL: &str = crate::standards::v1::subsets::any::schema::snapshot::text::FEM2D_EXAMPLE_TEXT;
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Fem2dPlayApp::Command` — the SOLE dispatch surface for fem2d's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the codec uses) — genuinely different vocabularies; `"setActiveExample" as
    /// "active-example"` and `"setCamera" as "camera"` are two of the rows that prove it. **Row order is
    /// the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum Fem2dCommand for Fem2dSnapshot, Fem2dMutation, NoConfig, NoConfigMutation {
        "addNode" as "add-node" => add_node::AddNode,
        "addBar" as "add-bar" => add_bar::AddBar,
        "addBeam" as "add-beam" => add_beam::AddBeam,
        "addMaterial" as "add-material" => add_material::AddMaterial,
        "addSection" as "add-section" => add_section::AddSection,
        "addSupport" as "add-support" => add_support::AddSupport,
        "addNodalLoad" as "add-nodal-load" => add_nodal_load::AddNodalLoad,
        "addMemberUdl" as "add-member-udl" => add_member_udl::AddMemberUdl,
        "addAreaLoad" as "add-area-load" => add_area_load::AddAreaLoad,
        "addRegion" as "add-region" => add_region::AddRegion,
        "addLoadCase" as "add-load-case" => add_load_case::AddLoadCase,
        "addCombination" as "add-combination" => add_combination::AddCombination,
        "setSelfWeight" as "set-self-weight" => set_self_weight::SetSelfWeight,
        "setAnalysisSettings" as "set-analysis-settings" => set_analysis_settings::SetAnalysisSettings,
        "removeSelection" as "remove-selection" => remove_selection::RemoveSelection,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setResultDisplay" as "result-display" => set_result_display::SetResultDisplay,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "patchNode" as "patch-node" => patch_node::PatchNode,
        "patchElement" as "patch-element" => patch_element::PatchElement,
        "patchMaterial" as "patch-material" => patch_material::PatchMaterial,
        "patchSection" as "patch-section" => patch_section::PatchSection,
        "patchSupport" as "patch-support" => patch_support::PatchSupport,
        "patchRegion" as "patch-region" => patch_region::PatchRegion,
        "patchLoad" as "patch-load" => patch_load::PatchLoad,
        "patchLoadCase" as "patch-load-case" => patch_load_case::PatchLoadCase,
        "patchCombination" as "patch-combination" => patch_combination::PatchCombination,
        "setResultAnimation" as "result-animation" => set_result_animation::SetResultAnimation,
        "resultAnimationTick" as "result-animation-tick" => result_animation_tick::ResultAnimationTick,
        "focusEntity" as "focus-entity" => focus_entity::FocusEntity,
        "translateSelection" as "translate-selection" => translate_selection::TranslateSelection,
        "rotateSelection" as "rotate-selection" => rotate_selection::RotateSelection,
        "scaleSelection" as "scale-selection" => scale_selection::ScaleSelection,
        "setTransformGumballFlag" as "set-transform-gumball-flag" => set_transform_gumball_flag::SetTransformGumballFlag,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
/// 🧵️ Every `Fem2dCommand` row, without exception — fem2d declares no host-only verb, so the retained
/// route table and `create_fem2d_app`'s `Migrated` classification list are the same nineteen ids
/// (pinned by `retained_routes_cover_every_command_exactly_once`).
const FEM2D_RETAINED_TOOL_IDS: &[&str] = &[
    "addNode",
    "addBar",
    "addBeam",
    "addMaterial",
    "addSection",
    "addSupport",
    "addNodalLoad",
    "addMemberUdl",
    "addAreaLoad",
    "addRegion",
    "addLoadCase",
    "addCombination",
    "setSelfWeight",
    "setAnalysisSettings",
    "removeSelection",
    "setActiveExample",
    "setCamera",
    "setResultDisplay",
    "canvasPointerDown",
    "canvasPointerMove",
    "canvasPointerUp",
    "patchNode",
    "patchElement",
    "patchMaterial",
    "patchSection",
    "patchSupport",
    "patchRegion",
    "patchLoad",
    "patchLoadCase",
    "patchCombination",
    "setResultAnimation",
    "resultAnimationTick",
    "focusEntity",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "setTransformGumballFlag",
];
const FEM2D_RETAINED_PAYLOAD_SCHEMA: &str = "fem.2d.tool-command.v1";
const FEM2D_RETAINED_RAW_BYTES: usize = 65_536;
const FEM2D_RETAINED_WORK_ITEMS: usize = 4_096;
/// 🧮️ The largest document a retained fem2d route will reduce over in one bounded first step — the
/// same number the artifact-lane store preparation admits, so a document too large for the reducer is
/// rejected before any authority is claimed rather than mid-publication.
const FEM2D_MAXIMUM_DOCUMENT_ITEMS: usize = 4_096;
/// 🛣️ Structural commands publish artifact mutations; example loading emits a host effect.
/// Camera and result display publish only to the addressed window's configuration.
const FEM2D_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addBar", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addBeam", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addMaterial", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSupport", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addNodalLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addMemberUdl", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addAreaLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addLoadCase", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addCombination", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setSelfWeight", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setAnalysisSettings", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setResultDisplay", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "patchNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchElement", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchMaterial", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchSection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchSupport", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchLoadCase", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchCombination", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setResultAnimation", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "resultAnimationTick", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "focusEntity", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setTransformGumballFlag", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

fn fem2d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(FEM2D_RETAINED_RAW_BYTES, FEM2D_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 🧮️ Every fem2d route reduces in exactly one bounded step, admitted only while the live document
/// stays inside `FEM2D_MAXIMUM_DOCUMENT_ITEMS` — an oversized document faults as "exceeds semantic
/// work capacity" instead of silently blowing the step budget.
fn fem2d_document_items(snapshot: &Fem2dSnapshot) -> Option<usize> {
    [snapshot.nodes.len(), snapshot.elements.len(), snapshot.regions.len(), snapshot.materials.len(), snapshot.sections.len(), snapshot.supports.len(), snapshot.load_cases.len(), snapshot.combinations.len()]
        .into_iter()
        .try_fold(1usize, |total, count| total.checked_add(count))
}

fn fem2d_interaction_selection_ids(interaction: &protocol::InteractionState) -> Vec<String> {
    interaction.selection.get(FEM2D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default()
}

fn fem2d_retained_extent(command: &Fem2dCommand, snapshot: &Fem2dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !FEM2D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    fem2d_document_items(snapshot).filter(|items| *items <= FEM2D_MAXIMUM_DOCUMENT_ITEMS).map(|_| 1)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn fem2d_retained_reduce(
    command: &Fem2dCommand,
    snapshot: &Fem2dSnapshot,
    config: &NoConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Fem2dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Fem2dMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    if !FEM2D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("fem2d-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: context.and_then(|context| context.window_config.as_ref()) };
    let view = context.and_then(|context| context.view_state.as_ref());
    match command {
        Fem2dCommand::SetCamera(payload) => set_camera::handle_window(payload, &cfg, view.ok_or_else(|| Fault::from("fem2d.camera.window-context-required"))?),
        Fem2dCommand::SetResultDisplay(payload) => set_result_display::handle_window(payload, &cfg, view.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?),
        Fem2dCommand::SetResultAnimation(payload) => set_result_animation::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?),
        Fem2dCommand::ResultAnimationTick(payload) => result_animation_tick::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?),
        Fem2dCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
        Fem2dCommand::CanvasPointerMove(payload) => canvas_pointer_move::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
        Fem2dCommand::CanvasPointerUp(payload) => canvas_pointer_up::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
        Fem2dCommand::FocusEntity(payload) => focus_entity::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.camera.window-context-required"))?),
        Fem2dCommand::TranslateSelection(payload) => {
            let payload = if payload.ids.is_empty() {
                translate_selection::TranslateSelection { ids: fem2d_interaction_selection_ids(interaction), dx: payload.dx, dy: payload.dy, dz: payload.dz }
            } else {
                payload.clone()
            };
            translate_selection::handle(&payload, &doc, &cfg)
        }
        Fem2dCommand::RotateSelection(payload) => {
            let payload = if payload.ids.is_empty() {
                rotate_selection::RotateSelection { ids: fem2d_interaction_selection_ids(interaction), ax: payload.ax, ay: payload.ay, az: payload.az, angle: payload.angle }
            } else {
                payload.clone()
            };
            rotate_selection::handle(&payload, &doc, &cfg)
        }
        Fem2dCommand::ScaleSelection(payload) => {
            let payload = if payload.ids.is_empty() {
                scale_selection::ScaleSelection { ids: fem2d_interaction_selection_ids(interaction), sx: payload.sx, sy: payload.sy, sz: payload.sz }
            } else {
                payload.clone()
            };
            scale_selection::handle(&payload, &doc, &cfg)
        }
        Fem2dCommand::SetTransformGumballFlag(payload) => set_transform_gumball_flag::handle_window(payload, view.ok_or_else(|| Fault::from("fem2d.gumball-flag.window-context-required"))?),
        _ => command.dispatch(&doc, &cfg),
    }
}

struct Fem2dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Fem2dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: FEM2D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Fem2dRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Fem2dPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Fem2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FEM2D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        fem2d_retained_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > FEM2D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("FEM2D retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Fem2dRetainedCommandJobFactory {
    type Owner = EditorApp<Fem2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FEM2D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = crate::FEM_2D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = FEM2D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️ArtifactStorePreparation
/// 📬️ Retains the document lane's one-item publication authority for structural edits.
struct Fem2dArtifactPreparationFactory;

struct Fem2dArtifactPreparation {
    base: Option<store::SnapshotRead<Fem2dSnapshot>>,
    mutation: Option<Fem2dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Fem2dSnapshot, Fem2dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Fem2dSnapshot, Fem2dMutation> for Fem2dArtifactPreparationFactory {
    fn preflight(&self, _mutation: &Fem2dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("fem2d-artifact-lane-or-description-envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_one_invertible_item(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Fem2dSnapshot, Fem2dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Fem2dSnapshot, Fem2dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Fem2dSnapshot, Fem2dMutation>> {
        let admitted = fem2d_document_items(request.base.get()).is_some_and(|items| items <= FEM2D_MAXIMUM_DOCUMENT_ITEMS);
        if !admitted
            || request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()
        {
            return Err(request);
        }
        Ok(Box::new(Fem2dArtifactPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Fem2dSnapshot, Fem2dMutation> for Fem2dArtifactPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "fem2d-artifact-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "fem2d-artifact-mutation-owner-missing".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "fem2d-artifact-authority-missing".to_string())?;
        let id = format!("fem2d-retained-{}", authority.next_sequence_number());
        let edit = protocol::Edit {
            id: id.clone(),
            actor: Some(authority.actor().to_string()),
            forwards: vec![mutation],
            inverse,
            mutation_meta: vec![protocol::MutationMeta {
                mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
                dependencies: Vec::new(),
                base_version: authority.base_applied_edit_count() as u64,
                author_id: Some(protocol::ActorId(authority.actor().to_string())),
                timestamp: authority.next_clock(),
                undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: None,
                origin: Default::default(),
            }],
            description: self.description.take(),
            coalesce_key: None,
            sequence_number: authority.next_sequence_number(),
            started_at: String::new(),
            finished_at: None,
        };
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Fem2dSnapshot, Fem2dMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Fem2dSnapshot, Fem2dMutation>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("fem2d-artifact-base-retirement-rejected".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

//#region 🧪️ArtifactPreparationLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️fem2d-artifact-preparation-laws/🦀️.rs"]
mod fem2d_artifact_preparation_laws;
//#endregion 🧪️ArtifactPreparationLaws
//#endregion 📬️ArtifactStorePreparation

//#region 🔖️ExportImportHelpers
/// 👁️ Project persisted configuration from the addressed results window for rendering.
fn config_result_display(cfg: &results_window::config::Fem2dResultsWindowConfig) -> ResultDisplay {
    let mode = cfg.result_mode.display(cfg.result_mode_index);
    ResultDisplay { source_id: cfg.result_source_id.clone(), mode }
}

/// 🎨️ Manual `crate::model::StaticResult` -> JSON bridge for `"results:out"` (see `export_media` below)
/// — `crate::model::StaticResult`/`ElementResult`/`Dof` don't derive `Serialize` (out of this ticket's
/// scope: `🫀️core` is a shared crate), so this hand-rolls the same shape `dsl::json::to_json_string` would
/// have produced, using `Dof`'s existing `{:?}` formatting (already used for the reaction-label layers
/// in the results window's render).
fn dof_json(dof: Dof) -> Value {
    dsl::json!(format!("{dof:?}"))
}

fn element_result_json(result: &ElementResult) -> Value {
    match result {
        ElementResult::Bar { n } => dsl::json!({ "kind": "bar", "n": n }),
        ElementResult::Beam { stations } => {
            dsl::json!({ "kind": "beam", "stations": stations.iter().map(|s| dsl::json!({ "x": s.x, "n": s.n, "v": s.v, "m": s.m })).collect::<Vec<_>>() })
        }
        ElementResult::Plane { gauss } => {
            dsl::json!({ "kind": "plane", "gauss": gauss.iter().map(|g| dsl::json!({ "sxx": g.sxx, "syy": g.syy, "sxy": g.sxy, "vonMises": g.von_mises })).collect::<Vec<_>>() })
        }
        ElementResult::Plate { gauss } => {
            dsl::json!({ "kind": "plate", "gauss": gauss.iter().map(|g| dsl::json!({ "mx": g.mx, "my": g.my, "mxy": g.mxy })).collect::<Vec<_>>() })
        }
        ElementResult::Solid { gauss } => dsl::json!({
            "kind": "solid",
            "gauss": gauss.iter().map(|g| dsl::json!({ "sxx": g.sxx, "syy": g.syy, "szz": g.szz, "sxy": g.sxy, "syz": g.syz, "sxz": g.sxz, "vonMises": g.von_mises })).collect::<Vec<_>>(),
        }),
        ElementResult::Shell { gauss } => dsl::json!({
            "kind": "shell",
            "gauss": gauss.iter().map(|g| dsl::json!({ "nxx": g.nxx, "nyy": g.nyy, "nxy": g.nxy, "mxx": g.mxx, "myy": g.myy, "mxy": g.mxy, "vonMisesTop": g.von_mises_top, "vonMisesBottom": g.von_mises_bottom })).collect::<Vec<_>>(),
        }),
    }
}

fn static_result_json(result: &crate::model::StaticResult) -> Value {
    dsl::json!({
        "displacements": result.displacements.iter().map(|d| dsl::json!({ "nodeId": d.node_id, "values": d.values })).collect::<Vec<_>>(),
        "reactions": result.reactions.iter().map(|r| dsl::json!({ "nodeId": r.node_id, "dof": dof_json(r.dof), "value": r.value })).collect::<Vec<_>>(),
        "elements": result.elements.iter().map(|(id, element_result)| dsl::json!({ "id": id, "result": element_result_json(element_result) })).collect::<Vec<_>>(),
        "checks": { "residualNorm": result.checks.residual_norm, "reactionSum": result.checks.reaction_sum },
    })
}

fn results_map_json(results: &HashMap<String, crate::model::StaticResult>) -> Value {
    Value::Object(results.iter().map(|(id, result)| (id.clone(), static_result_json(result))).collect())
}
//#endregion 🔖️ExportImportHelpers

//#region 🔌️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document port pair
/// (`fem.2d` × 2D-Vector) plus `geometry:in` (importing an externally authored 2D outline as a new
/// `FemRegion` — see `import_media` above) and `results:out` (every load case/combination's solved
/// `crate::model::StaticResult`, pinned to the `computation.fem2d` artifact kind declared in
/// `crate::computation_artifact_kind` — see `export_media` above). Moved out of the
/// (now deleted) artifact `⚙️engine`: it returns `AppIo`, an app type, so it belongs here.
pub fn fem2d_io() -> AppIo {
    AppIo {
        artifact_schema: crate::FEM_2D_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        ports: vec![fem2d_geometry_in_port(), fem2d_results_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.fem".into(), name: "FEM 2D".into(), dimension: "2d".into(), component_kind: "fem2d".into() },
    }
}

/// 🔌️ `geometry:in` — an externally authored 2D polygon-with-holes outline, imported as a new
/// `FemRegion`.
pub fn fem2d_geometry_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "geometry:in".into(),
        label: "Geometry".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: None,
        required: true,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}

/// 🔌️ `results:out` — every load case/combination's solved `crate::model::StaticResult`, pinned to the
/// `computation.fem2d` artifact kind.
pub fn fem2d_results_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "results:out".into(),
        label: "Results".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        kind_id: Some("computation.fem2d".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}
//#endregion 🔌️Io

//#region 🔖️ActionArgHelpers
/// 🛡️ Reads one `FemDof` out of an action arg's free text (`"tx"`, `"Ty"`, `"rz"`, …) — the shells
/// stage every declared arg as a JSON scalar, so the typed enum is resolved here rather than by the
/// stringly action wire.
fn fem2d_dof(value: Option<&str>) -> Option<crate::FemDof> {
    use crate::FemDof;
    match value?.trim().to_ascii_lowercase().as_str() {
        "tx" => Some(FemDof::Tx),
        "ty" => Some(FemDof::Ty),
        "tz" => Some(FemDof::Tz),
        "rx" => Some(FemDof::Rx),
        "ry" => Some(FemDof::Ry),
        "rz" => Some(FemDof::Rz),
        _ => None,
    }
}

/// 🛡️ Reads `addSupport`'s separator-delimited `fixed` list, defaulting to the pinned support
/// (`tx,ty`) every fem2d fixture starts from when nothing is staged.
fn fem2d_dofs(value: Option<&str>) -> Vec<crate::FemDof> {
    use crate::FemDof;
    let parsed: Vec<FemDof> = value.map(|text| text.split([',', ' ', ';']).filter_map(|token| fem2d_dof(Some(token))).collect()).unwrap_or_default();
    if parsed.is_empty() {
        vec![FemDof::Tx, FemDof::Ty]
    } else {
        parsed
    }
}
//#endregion 🔖️ActionArgHelpers

//#region 🔖️Fem2dPlayApp
/// 🪟️ FEM application commands and rendering use the concrete Model and Results window owners.
#[derive(Default)]
pub struct Fem2dPlayApp;

impl ArtifactEditor for Fem2dPlayApp {
    type Snapshot = Fem2dSnapshot;
    type Mutation = Fem2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Fem2dCommand;

    const DIALECT: Dialect = crate::FEM2D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::FEM_2D_SCHEMA;

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 🏗️ Admits the whole-document replacement every `Effect::LoadDocument` this editor emits
    /// (`reset_document_effect`: example switch, `artifact:in` import) drives through the host's
    /// persisted-envelope replacement — the trait default refuses the envelope, which makes every
    /// fem2d document swap fault at the archive-load boundary.
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, crate::FEM_2D_SCHEMA, operation, generation))
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

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<model_window::config::Fem2dModelWindowConfigOwner>()?;
        registry.register::<results_window::config::Fem2dResultsWindowConfigOwner>()
    }



    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Fem2dArtifactPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Fem2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs",
        controller: "s.fem.fem2d@1/*#editor",
        artifact_schema: "fem.2d",
        factory: "Fem2dRetainedCommandJobFactory",
        factory_type: Fem2dRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "addNode",
            "addBar",
            "addBeam",
            "addMaterial",
            "addSection",
            "addSupport",
            "addNodalLoad",
            "addMemberUdl",
            "addAreaLoad",
            "addRegion",
            "addLoadCase",
            "addCombination",
            "setSelfWeight",
            "setAnalysisSettings",
            "removeSelection",
            "setActiveExample",
            "setCamera",
            "setResultDisplay",
            "canvasPointerDown",
            "canvasPointerMove",
            "canvasPointerUp",
            "patchNode",
            "patchElement",
            "patchMaterial",
            "patchSection",
            "patchSupport",
            "patchRegion",
            "patchLoad",
            "patchLoadCase",
            "patchCombination",
            "setResultAnimation",
            "resultAnimationTick",
            "focusEntity",
            "translateSelection",
            "rotateSelection",
            "scaleSelection",
            "setTransformGumballFlag",
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Fem2dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !FEM2D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("fem2d-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, fem2d_retained_reduce, fem2d_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            Fem2dCommand::command_id,
            FEM2D_RETAINED_RAW_BYTES,
            FEM2D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        None
    }

    /// 🌱️ Boots on the bundled `📚️examples/🎬️demo` document so the playground paints a real structure
    /// at first frame instead of an empty canvas — the same snapshot `Fem2dViewer` already booted on.
    fn initial_snapshot() -> Fem2dSnapshot {
        crate::standards::v1::subsets::any::schema::default_fem2d_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(fem2d_io())
    }

    fn mounted_job_maintenance_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(crate::editor::fem2d::session::maintenance_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_job_close_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(crate::editor::fem2d::session::close_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        crate::editor::fem2d::session::terminal_is_empty(instance_id)
    }

    fn mounted_job_prepare_snapshot_read(operation: semio_framework_plugin::AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
        crate::editor::fem2d::session::prepare_snapshot_read(operation, snapshot)
    }

    /// 🎞️ `"artifact:out"` reproduces the trait's default whole-document pack (overriding `export_media`
    /// shadows the trait's provided body for every port on this app, not just the new one). `"results:out"`
    /// runs every load case/combination's analysis fresh and returns them as plain JSON text in a
    /// `Structured` payload — `MediaPayload::Structured.json` doesn't require a `pack`-encoded value. A
    /// document with no load cases, or a solve failure, is reported as `MediaError::Payload` rather than
    /// an empty/panicking export.
    fn export_media(port: &str, doc: &ArtifactView<'_, Fem2dSnapshot>) -> Result<Media, MediaError> {
        match port {
            "artifact:out" => {
                let media_type = fem2d_io().artifact_media_type;
                let bytes = <Fem2dSnapshot as store::ArtifactPack>::encode_pack(doc.snapshot);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "results:out" => {
                if doc.snapshot.load_cases.is_empty() {
                    return Err(MediaError::Payload("results:out".into(), "no load cases defined".into()));
                }
                let results = crate::fem2d_engine::fem2d_solve_all(doc.snapshot).map_err(|error| MediaError::Payload("results:out".into(), error.to_string()))?;
                let json = results_map_json(&results).to_string();
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.fem2d".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧬️ No `whole_document_operation` override on this impl — per `📓️taxonomy.md`, whole-document
    /// replace (`SetSnapshot`) is banned outright with NO replacement mutation, so this falls back to
    /// the trait's own default (`None`).
    ///
    /// 🎞️ `"artifact:in"` swaps the whole live document via `reset_document_effect` (a
    /// `Effect::LoadDocument`, the sanctioned non-history whole-doc-replace path — see
    /// `reset_document_effect`'s own doc comment) instead of routing through `whole_document_operation`.
    /// `"geometry:in"` decodes a minimal, app-owned `{"outline": [[f64;2]...], "holes": [[[f64;2]...]...]}`
    /// polygon-with-holes contract into a new `FemRegion` via `create-region`, defaulted to the
    /// document's first existing material if any, else an `"unassigned"` placeholder id.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Fem2dSnapshot>) -> Result<Emit<Fem2dMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "artifact:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <Fem2dSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
            }
            "geometry:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "geometry:in only accepts a Structured JSON payload".into()));
                };
                let value = dsl::json::parse(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let outline: Vec<[f64; 2]> = dsl::FromValue::from_value(dsl::json::to_dsl_value(&value.get("outline").cloned().unwrap_or(Value::Null))).map_err(|error| MediaError::Payload(port.to_string(), format!("outline: {error}")))?;
                let holes: Vec<Vec<[f64; 2]>> = match value.get("holes").cloned() {
                    Some(holes_value) => dsl::FromValue::from_value(dsl::json::to_dsl_value(&holes_value)).map_err(|error| MediaError::Payload(port.to_string(), format!("holes: {error}")))?,
                    None => Vec::new(),
                };
                let material_id = doc.snapshot.materials.first().map_or_else(|| "unassigned".into(), |material| material.id.clone());
                let id = crate::app_surface::next_id(doc.snapshot.regions.iter().map(|r| r.id.clone()), "r");
                let region = crate::FemRegion { id, name: "Imported Geometry".into(), outline, holes, thickness: 0.02, material_id, mesh_size: 0.25 };
                Ok(Emit::mutations(vec![Fem2dMutation::CreateRegion(crate::standards::v1::subsets::any::schema::mutations::create_region::CreateRegion { region })]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &Fem2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + staged args onto `Fem2dCommand` — the React/wgpu shells still speak the
    /// stringly `{action, args}` wire (`ShellHost`'s action pane and its example switcher both do), and
    /// the trait's default rejects every app action outright, so without this bridge none of fem2d's
    /// nineteen declared actions can reach `dispatch`. Every key here is the `ActionArgDef.id` declared
    /// for that action in `🔖️Manifest` below.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let text = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_str).map(str::to_string);
        let number = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_f64);
        let flag = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_bool);
        let list = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_array).map(|items| items.iter().filter_map(dsl::DslValue::as_str).map(str::to_string).collect::<Vec<_>>());
        // 🧵️ `samples: [[x, y], …]` — every well-formed 2-number pair in order; malformed entries are skipped.
        let point_list = |key: &str| {
            args.and_then(|value| value.get(key))
                .and_then(dsl::DslValue::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|item| {
                            let pair = item.as_array()?;
                            Some([pair.first()?.as_f64()?, pair.get(1)?.as_f64()?])
                        })
                        .collect::<Vec<[f64; 2]>>()
                })
                .unwrap_or_default()
        };
        // 🩹️ A control's `Trigger::Change` value arrives typed (number for sliders/number inputs,
        // bool for toggles, text for selects/inputs); every patch command carries it as text.
        let scalar_text = |key: &str| {
            args.and_then(|value| value.get(key)).and_then(|value| match value {
                dsl::DslValue::String(text) => Some(text.clone()),
                dsl::DslValue::Bool(flag) => Some(flag.to_string()),
                other => other.as_f64().map(|number| number.to_string()),
            })
        };
        match action {
            "addNode" => Ok(Fem2dCommand::AddNode(add_node::AddNode { x: number("x").unwrap_or_default(), y: number("y").unwrap_or_default() })),
            "addBar" => {
                Ok(Fem2dCommand::AddBar(add_bar::AddBar { start: text("start").unwrap_or_default(), end: text("end").unwrap_or_default(), material_id: text("materialId").unwrap_or_default(), section_id: text("sectionId").unwrap_or_default() }))
            }
            "addBeam" => {
                Ok(Fem2dCommand::AddBeam(add_beam::AddBeam { start: text("start").unwrap_or_default(), end: text("end").unwrap_or_default(), material_id: text("materialId").unwrap_or_default(), section_id: text("sectionId").unwrap_or_default() }))
            }
            "addMaterial" => Ok(Fem2dCommand::AddMaterial(add_material::AddMaterial { name: text("name").unwrap_or_default(), e: number("e").unwrap_or(2.1e11) })),
            "addSection" => Ok(Fem2dCommand::AddSection(add_section::AddSection { name: text("name").unwrap_or_default(), area: number("area").unwrap_or_default(), iy: number("iy").unwrap_or_default() })),
            "addSupport" => Ok(Fem2dCommand::AddSupport(add_support::AddSupport { node_id: text("nodeId").unwrap_or_default(), fixed: fem2d_dofs(text("fixed").as_deref()) })),
            "addNodalLoad" => Ok(Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad {
                node_id: text("nodeId").unwrap_or_default(),
                dof: fem2d_dof(text("dof").as_deref()).unwrap_or(crate::FemDof::Ty),
                value: number("value").unwrap_or_default(),
                case_id: text("caseId").filter(|id| !id.is_empty()),
            })),
            "addMemberUdl" => Ok(Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl {
                element_id: text("elementId").unwrap_or_default(),
                wx: number("wx").unwrap_or_default(),
                wy: number("wy").unwrap_or_default(),
                case_id: text("caseId").filter(|id| !id.is_empty()),
            })),
            "addAreaLoad" => Ok(Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: text("regionId").unwrap_or_default(), pressure: number("pressure").unwrap_or_default(), case_id: text("caseId").filter(|id| !id.is_empty()) })),
            "addRegion" => Ok(Fem2dCommand::AddRegion(add_region::AddRegion {
                x: number("x").unwrap_or_default(),
                y: number("y").unwrap_or_default(),
                width: number("width").unwrap_or_default(),
                height: number("height").unwrap_or_default(),
                material_id: text("materialId").unwrap_or_default(),
                thickness: number("thickness"),
                mesh_size: number("meshSize"),
            })),
            "addLoadCase" => Ok(Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: text("name").unwrap_or_default(), self_weight: flag("selfWeight").unwrap_or(false) })),
            "addCombination" => Ok(Fem2dCommand::AddCombination(add_combination::AddCombination { name: text("name").unwrap_or_default(), terms: Vec::new() })),
            "setSelfWeight" => Ok(Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: text("caseId").unwrap_or_default(), enabled: flag("enabled").unwrap_or(false) })),
            "setAnalysisSettings" => Ok(Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings {
                modal_count: number("modalCount").map(|value| value.max(0.0) as u32),
                buckling_count: number("bucklingCount").map(|value| value.max(0.0) as u32),
                deformation_scale: number("deformationScale"),
                field: text("field"),
                value: scalar_text("value"),
                window_id: text("windowId"),
            })),
            "removeSelection" => Ok(Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: list("ids").unwrap_or_default() })),
            "setActiveExample" => Ok(Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text("exampleId").or_else(|| text("id")).unwrap_or_default() })),
            "setCamera" => Ok(Fem2dCommand::SetCamera(set_camera::SetCamera { x: number("x").unwrap_or_default(), y: number("y").unwrap_or_default(), zoom: number("zoom").unwrap_or(1.0) })),
            "setResultDisplay" => Ok(Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay {
                source_id: text("sourceId").filter(|id| !id.is_empty()),
                mode: text("mode").unwrap_or_else(|| "static".into()),
                mode_index: number("modeIndex").map(|value| value.max(0.0) as u32).unwrap_or_default(),
                field: text("field"),
                value: scalar_text("value"),
                window_id: text("windowId"),
            })),
            "canvasPointerDown" => Ok(Fem2dCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
                x: number("x").unwrap_or_default(),
                y: number("y").unwrap_or_default(),
                width: number("width").unwrap_or_default(),
                height: number("height").unwrap_or_default(),
                button: number("button").map(|value| value.max(0.0) as u32).unwrap_or_default(),
                shift: flag("shift").unwrap_or(false),
                ctrl: flag("ctrl").unwrap_or(false),
                meta: flag("meta").unwrap_or(false),
                alt: flag("alt").unwrap_or(false),
            })),
            "canvasPointerMove" => {
                let x = number("x").unwrap_or_default();
                let y = number("y").unwrap_or_default();
                let mut samples = point_list("samples");
                if samples.is_empty() {
                    samples.push([x, y]);
                }
                Ok(Fem2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x, y, width: number("width").unwrap_or_default(), height: number("height").unwrap_or_default(), samples }))
            }
            "canvasPointerUp" => Ok(Fem2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {
                x: number("x").unwrap_or_default(),
                y: number("y").unwrap_or_default(),
                width: number("width").unwrap_or_default(),
                height: number("height").unwrap_or_default(),
                shift: flag("shift").unwrap_or(false),
                ctrl: flag("ctrl").unwrap_or(false),
                meta: flag("meta").unwrap_or(false),
                alt: flag("alt").unwrap_or(false),
                cancelled: flag("cancelled").unwrap_or(false),
            })),
            "patchNode" => Ok(Fem2dCommand::PatchNode(patch_node::PatchNode { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchElement" => Ok(Fem2dCommand::PatchElement(patch_element::PatchElement { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchMaterial" => Ok(Fem2dCommand::PatchMaterial(patch_material::PatchMaterial { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchSection" => Ok(Fem2dCommand::PatchSection(patch_section::PatchSection { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchSupport" => Ok(Fem2dCommand::PatchSupport(patch_support::PatchSupport { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchRegion" => Ok(Fem2dCommand::PatchRegion(patch_region::PatchRegion { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchLoad" => Ok(Fem2dCommand::PatchLoad(patch_load::PatchLoad { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchLoadCase" => Ok(Fem2dCommand::PatchLoadCase(patch_load_case::PatchLoadCase { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchCombination" => Ok(Fem2dCommand::PatchCombination(patch_combination::PatchCombination { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "setResultAnimation" => Ok(Fem2dCommand::SetResultAnimation(set_result_animation::SetResultAnimation { phase: number("phase"), playing: flag("playing"), speed: number("speed"), loop_mode: text("loopMode"), waveform: text("waveform"), field: text("field"), value: scalar_text("value"), window_id: text("windowId") })),
            "resultAnimationTick" => Ok(Fem2dCommand::ResultAnimationTick(result_animation_tick::ResultAnimationTick {})),
            "focusEntity" => Ok(Fem2dCommand::FocusEntity(focus_entity::FocusEntity { id: text("id").unwrap_or_default() })),
            "translateSelection" => Ok(Fem2dCommand::TranslateSelection(translate_selection::TranslateSelection {
                ids: list("ids").unwrap_or_default(),
                dx: number("dx").unwrap_or_default(),
                dy: number("dy").unwrap_or_default(),
                dz: number("dz").unwrap_or_default(),
            })),
            "rotateSelection" => Ok(Fem2dCommand::RotateSelection(rotate_selection::RotateSelection {
                ids: list("ids").unwrap_or_default(),
                ax: number("ax").unwrap_or_default(),
                ay: number("ay").unwrap_or_default(),
                az: number("az").unwrap_or(1.0),
                angle: number("angle").unwrap_or_default(),
            })),
            "scaleSelection" => Ok(Fem2dCommand::ScaleSelection(scale_selection::ScaleSelection {
                ids: list("ids").unwrap_or_default(),
                sx: number("sx").unwrap_or(1.0),
                sy: number("sy").unwrap_or(1.0),
                sz: number("sz").unwrap_or(1.0),
            })),
            "setTransformGumballFlag" => Ok(Fem2dCommand::SetTransformGumballFlag(set_transform_gumball_flag::SetTransformGumballFlag { flag: text("flag").unwrap_or_default(), pressed: flag("pressed") })),
            other => Err(Fault::from(format!("action '{other}' is not a declared fem2d action — every app action is dispatched through the typed command channel (see `dispatch_typed_command`)"))),
        }
    }

    fn handle(
        command: &Fem2dCommand,
        doc: &ArtifactView<'_, Fem2dSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Fem2dMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            // 🗑️ The delete keybinding dispatches `removeSelection` without ids: the live framework-owned
            // `"fem2d"` selection is what gets removed.
            Fem2dCommand::RemoveSelection(payload) if payload.ids.is_empty() => remove_selection::handle(&remove_selection::RemoveSelection { ids: interaction.selection(FEM2D_INTERACTION_DOMAIN).ids.clone() }, doc, cfg),
            Fem2dCommand::TranslateSelection(payload) => {
                let payload = if payload.ids.is_empty() {
                    translate_selection::TranslateSelection { ids: interaction.selection(FEM2D_INTERACTION_DOMAIN).ids.clone(), dx: payload.dx, dy: payload.dy, dz: payload.dz }
                } else {
                    payload.clone()
                };
                translate_selection::handle(&payload, doc, cfg)
            }
            Fem2dCommand::RotateSelection(payload) => {
                let payload = if payload.ids.is_empty() {
                    rotate_selection::RotateSelection { ids: interaction.selection(FEM2D_INTERACTION_DOMAIN).ids.clone(), ax: payload.ax, ay: payload.ay, az: payload.az, angle: payload.angle }
                } else {
                    payload.clone()
                };
                rotate_selection::handle(&payload, doc, cfg)
            }
            Fem2dCommand::ScaleSelection(payload) => {
                let payload = if payload.ids.is_empty() {
                    scale_selection::ScaleSelection { ids: interaction.selection(FEM2D_INTERACTION_DOMAIN).ids.clone(), sx: payload.sx, sy: payload.sy, sz: payload.sz }
                } else {
                    payload.clone()
                };
                scale_selection::handle(&payload, doc, cfg)
            }
            Fem2dCommand::SetTransformGumballFlag(payload) => set_transform_gumball_flag::handle_window(payload, view_state.ok_or_else(|| Fault::from("fem2d.gumball-flag.window-context-required"))?),
            Fem2dCommand::SetCamera(payload) => set_camera::handle_window(payload, cfg, view_state.ok_or_else(|| Fault::from("fem2d.camera.window-context-required"))?),
            Fem2dCommand::SetResultDisplay(payload) => set_result_display::handle_window(payload, cfg, view_state.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?),
            Fem2dCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
            Fem2dCommand::CanvasPointerMove(payload) => canvas_pointer_move::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
            Fem2dCommand::CanvasPointerUp(payload) => canvas_pointer_up::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
            Fem2dCommand::SetResultAnimation(payload) => set_result_animation::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?),
            Fem2dCommand::ResultAnimationTick(payload) => result_animation_tick::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?),
            Fem2dCommand::FocusEntity(payload) => focus_entity::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.camera.window-context-required"))?),
            _ => command.dispatch(doc, cfg),
        }
    }

    fn pending_effects(_owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view: Option<&semio_framework_plugin::ViewModel>) -> Vec<semio_framework::kernel::Effect> {
        crate::editor::fem2d::session::reconcile(doc)
    }

    /// 🎯️ Fem2d has no user-visible config defaults to expose (all of `addRegion`'s
    /// `thickness`/`meshSize` defaults are baked directly into its handler, not user-configurable
    /// settings) — declaring `ConfigSpec::empty()` explicitly keeps the typed channel surface
    /// consistent with the sibling apps' convention.
    fn config_spec() -> ConfigSpec {
        ConfigSpec::default()
    }

    /// 🕹️ Interaction-less twin of [`Self::render_with_request_context`] — an empty `"fem2d"` domain,
    /// so nothing paints selected and the inspector shows the document summary.
    fn render(body_key: &str, doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, Fem2dInteractionSnapshot::default())
    }

    /// 🕹️ Reads the framework-owned `"fem2d"` selection/hover once per render and threads it through
    /// every body — the windows paint it, the artifact tree marks it, the inspector edits it.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Fem2dSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, Fem2dInteractionSnapshot::from_interaction(interaction))
    }

    fn window_measures(_doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = fem2d_labels(view_state);
        let measures = crate::editor::fem2d::window_measures::fem2d_window_measures(view_state, labels);
        HashMap::from([(model_window::WINDOW_KIND_ID.into(), measures.clone()), (results_window::WINDOW_KIND_ID.into(), measures)])
    }
}

impl Fem2dPlayApp {
    /// 🖼️ Body-key routing table shared by both render entry points: two Canvas2d windows and the
    /// three dock panels.
    fn render_body(body_key: &str, doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel, interaction: Fem2dInteractionSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let labels = fem2d_labels(view_state);
        match body_key {
            model_window::BODY_KEY => {
                let window = model_window::config::current(cfg);
                let window_id = crate::editor::fem2d::interaction::canvas_gesture::fem2d_gesture_window_id_for_render(view_state, model_window::BODY_KEY);
                let active_utility = crate::editor::fem2d::interaction::canvas_gesture::fem2d_active_utility(view_state);
                crate::editor::fem2d::session::with_live_visual(doc.render_operation(), |visual| {
                    model_window::render_with_progress(doc.snapshot, &window.camera, visual, &interaction, window_id.as_deref(), active_utility)
                })
            }
            results_window::BODY_KEY => {
                let window = results_window::config::current(cfg);
                let window_id = crate::editor::fem2d::interaction::canvas_gesture::fem2d_gesture_window_id_for_render(view_state, results_window::BODY_KEY);
                let active_utility = crate::editor::fem2d::interaction::canvas_gesture::fem2d_active_utility(view_state);
                results_window::render(
                    doc.snapshot,
                    &config_result_display(&window),
                    &window.camera,
                    &window,
                    &interaction,
                    doc.render_operation(),
                    window_id.as_deref(),
                    active_utility,
                )
            }
            artifact_panel::BODY_KEY => artifact_panel::render(doc.snapshot, &interaction, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, artifact_panel::BODY_KEY)),
            inspection_panel::BODY_KEY => inspection_panel::render(doc.snapshot, &interaction, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection_panel::BODY_KEY)),
            results_panel::BODY_KEY => results_panel::render(doc.snapshot, results_window::config::captured(cfg).as_ref(), &results_window_instance_id(view_state).unwrap_or_default(), labels),
            _ => built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "fem2d unknown-body label admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }
}

//#region 🔖️UiHelpers
fn fem2d_utility(id: &str, label: LocalizedLabel, icon: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some("Select".into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

const FEM2D_CANVAS_UTILITIES: &[&str] = &["selectDirect", "selectMarquee", "selectLasso", "transformMove", "transform"];

/// 🏷️ Admits resolved fem2d text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "fem2d UI label admission failed"))
}

/// 🎛️ Mints one fem2d-controller action for a panel row or control binding.
pub fn fem2d_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(FEM2D_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🎛️ Window-measure action binding — same controller scope as [`fem2d_action`], without the UI assembly envelope.
pub fn fem2d_measure_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: FEM2D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: args.map(|value| dsl::json::to_dsl_value(&value)) }
}

/// 🪟️ The first results-window instance of the layout — the one the results panel addresses when
/// the panel itself has no window context.
pub fn results_window_instance_id(view_state: &ViewModel) -> Option<String> {
    let is_results = |id: &str| view_state.window_instances.iter().any(|window| window.id == id && window.window_kind_id == results_window::WINDOW_KIND_ID);
    view_state
        .window_id
        .as_deref()
        .filter(|id| is_results(id))
        .or_else(|| view_state.focused_window_id.as_deref().filter(|id| is_results(id)))
        .map(str::to_string)
        .or_else(|| view_state.window_instances.iter().find(|window| window.window_kind_id == results_window::WINDOW_KIND_ID).map(|window| window.id.clone()))
}
//#endregion 🔖️UiHelpers
//#endregion 🔖️Fem2dPlayApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (file import,
/// load-example). Per `📓️taxonomy.md`, `SetSnapshot` is banned outright with NO replacement
/// mutation: whole-document replace is not expressible as an in-history `Mutation` at all. Every
/// former "replace the whole document" gesture in this package (`import_media`'s `"artifact:in"`,
/// `commands::set_active_example`) builds this effect instead of an `Emit::mutations([...])`.
/// The spr is a fresh, edit-free op-log for `scene` — a genesis envelope with no history to encode.
pub fn reset_document_effect(scene: &Fem2dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Fem2dSnapshot as store::ArtifactPack>::encode_pack(scene);
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("fem2d", crate::FEM_2D_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🩹️ One inspector patch action — internal (not in the palette; its arguments are authored by the
/// inspector row that binds it), a document mutation on one entity.
fn fem2d_patch_action(id: &str, label: LocalizedLabel) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition {
        in_palette: false,
        args: vec![ActionArgDef::text("id", LocalizedLabel::native("Entity", "Element")).required(), ActionArgDef::text("field", LocalizedLabel::native("Field", "Feld")).required(), ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).required()],
        ..semio_framework_plugin::ActionDefinition::new(id, label, semio_framework_plugin::ActionKind::Mutation, "pencil")
    }
}

/// 📚️ `AppBuilder` carries no `.example(...)`: an example is declared ONCE as a definition leaf
/// (`📚️examples/🎬️demo`'s `ExampleSource`) and reaches `PluginManifest.examples` through the subset
/// root's `SubsetDeclaration.examples` — see `🪆️subsets/🌐️any/🦀️.rs`. The shell's navbar switcher
/// reads that list and dispatches `setActiveExample { exampleId }` back into this app, which is why
/// `setActiveExample`'s select option below is that same `demo::ID` and why the action is `Migrated`
/// (a `BatchOnlyPendingRewrite` classification would make the switcher a dead control). `.workflow(...)`
/// stays dropped: `WorkflowDefinition` was deleted from framework-core, with no replacement surface.
pub fn create_fem2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::FEM2D_DIALECT)
            .document(["semio", "fem", "fem2d"])
            // 🔌️ The computed-results output artifact (`results:out`'s `kind_id`, see
            // `fem2d_io` above) — deliberately a different `media_type`
            // (`Computation`×`Value`) than the PORT's wire-level `Data`×`Value`.
            .artifact_kind(crate::computation_artifact_kind())
            .icon_id("fem-app")
            .mode(edit::MODE_ID, LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id(edit::MODE_ID)
            .window_kind(model_window::WINDOW_KIND_ID, LocalizedLabel::native("Model", "Modell"), model_window::BODY_KEY, semio_framework_ui_contract::SurfaceKind::Canvas2d, "fem-model")
            .window_kind(results_window::WINDOW_KIND_ID, LocalizedLabel::native("Results", "Ergebnisse"), results_window::BODY_KEY, semio_framework_ui_contract::SurfaceKind::Canvas2d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[model_window::WINDOW_KIND_ID.into(), results_window::WINDOW_KIND_ID.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_args("addNode", vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required(),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required(),
            ])
            .mutation("addBar", LocalizedLabel::native("Add Bar", "Stab hinzufügen"))
            .action_args("addBar", vec![
                ActionArgDef::text("start", LocalizedLabel::native("Start Node", "Startknoten")).required(),
                ActionArgDef::text("end", LocalizedLabel::native("End Node", "Endknoten")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::native("Material", "Material")).required(),
                ActionArgDef::text("sectionId", LocalizedLabel::native("Section", "Querschnitt")).required(),
            ])
            .mutation("addBeam", LocalizedLabel::native("Add Beam", "Balken hinzufügen"))
            .action_args("addBeam", vec![
                ActionArgDef::text("start", LocalizedLabel::native("Start Node", "Startknoten")).required(),
                ActionArgDef::text("end", LocalizedLabel::native("End Node", "Endknoten")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::native("Material", "Material")).required(),
                ActionArgDef::text("sectionId", LocalizedLabel::native("Section", "Querschnitt")).required(),
            ])
            .mutation("addMaterial", LocalizedLabel::native("Add Material", "Material hinzufügen"))
            .action_args("addMaterial", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::number("e", LocalizedLabel::native("Young's Modulus", "Elastizitätsmodul")).default_value(&2.1e11),
            ])
            .mutation("addSection", LocalizedLabel::native("Add Section", "Querschnitt hinzufügen"))
            .action_args("addSection", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::number("area", LocalizedLabel::native("Area", "Fläche")).required(),
                ActionArgDef::number("iy", LocalizedLabel::native("Second Moment of Area", "Flächenträgheitsmoment")).required(),
            ])
            .mutation("addSupport", LocalizedLabel::native("Add Support", "Lager hinzufügen"))
            // 🛡️ `fixed` is `Vec<FemDof>`; no `ActionArgDef` control maps to a typed enum list, so the
            // staged form takes the separator-delimited spelling `fem2d_dofs` reads (`"tx,ty"`).
            .action_args("addSupport", vec![
                ActionArgDef::text("nodeId", LocalizedLabel::native("Node", "Knoten")).required(),
                ActionArgDef::text("fixed", LocalizedLabel::native("Fixed Degrees of Freedom", "Gesperrte Freiheitsgrade")).default_value(&"tx,ty"),
            ])
            .mutation("addNodalLoad", LocalizedLabel::native("Add Nodal Load", "Knotenlast hinzufügen"))
            .action_args("addNodalLoad", vec![
                ActionArgDef::text("nodeId", LocalizedLabel::native("Node", "Knoten")).required(),
                ActionArgDef::select("dof", LocalizedLabel::native("Degree of Freedom", "Freiheitsgrad"), vec![
                    ActionArgOption::new("tx", LocalizedLabel::native("Tx", "Tx")),
                    ActionArgOption::new("ty", LocalizedLabel::native("Ty", "Ty")),
                    ActionArgOption::new("rz", LocalizedLabel::native("Rz", "Rz")),
                ])
                .default_value(&"ty"),
                ActionArgDef::number("value", LocalizedLabel::native("Value", "Wert")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addMemberUdl", LocalizedLabel::native("Add Member UDL", "Streckenlast hinzufügen"))
            .action_args("addMemberUdl", vec![
                ActionArgDef::text("elementId", LocalizedLabel::native("Element", "Element")).required(),
                ActionArgDef::number("wx", LocalizedLabel::native("Wx", "Wx")).default_value(&0.0),
                ActionArgDef::number("wy", LocalizedLabel::native("Wy", "Wy")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addAreaLoad", LocalizedLabel::native("Add Area Load", "Flächenlast hinzufügen"))
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("regionId", LocalizedLabel::native("Region", "Bereich")).required(),
                ActionArgDef::number("pressure", LocalizedLabel::native("Pressure", "Druck")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addRegion", LocalizedLabel::native("Add Region", "Bereich hinzufügen"))
            .action_args("addRegion", vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required(),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required(),
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required(),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::native("Material", "Material")).required(),
                ActionArgDef::number("thickness", LocalizedLabel::native("Thickness", "Dicke")).default_value(&0.02),
                ActionArgDef::number("meshSize", LocalizedLabel::native("Mesh Size", "Netzgröße")).default_value(&0.25),
            ])
            .mutation("addLoadCase", LocalizedLabel::native("Add Load Case", "Lastfall hinzufügen"))
            .action_args("addLoadCase", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::toggle("selfWeight", LocalizedLabel::native("Self Weight", "Eigengewicht")).default_value(&false),
            ])
            // 🎯️ `terms` is `Fem2dCommand::AddCombination`'s typed `Vec<FemCombinationTerm>` — no single
            // `ActionArgDef` control maps to that shape, so the staged form declares `name` only and the
            // action bridge opens an empty combination the term rows are added to afterwards.
            .mutation("addCombination", LocalizedLabel::native("Add Combination", "Kombination hinzufügen"))
            .action_args("addCombination", vec![ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required()])
            .mutation("setSelfWeight", LocalizedLabel::native("Set Self Weight", "Eigengewicht festlegen"))
            .action_args("setSelfWeight", vec![
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")).required(),
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),
            ])
            .mutation("setAnalysisSettings", LocalizedLabel::native("Set Analysis Settings", "Analyseeinstellungen festlegen"))
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", LocalizedLabel::native("Modal Count", "Anzahl Eigenformen")),
                ActionArgDef::number("bucklingCount", LocalizedLabel::native("Buckling Count", "Anzahl Knickformen")),
                ActionArgDef::number("deformationScale", LocalizedLabel::native("Deformation Scale", "Verformungsmaßstab")),
            ])
            .mutation("removeSelection", LocalizedLabel::native("Remove Selection", "Auswahl entfernen"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), semio_framework_plugin::ActionKind::View, "camera"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), semio_framework_plugin::ActionKind::Mutation, "panel-left"))
            // 📚️ The option id is the bundled example's own `ExampleSource` id, because the shell's
            // navbar switcher dispatches `setActiveExample { exampleId }` straight from
            // `PluginManifest.examples` (`ShellHost`'s `dispatchActiveExample`) — a select option that
            // did not match that id could never be reached from the switcher.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(
                    crate::examples::demo::ID,
                    crate::examples::demo::label(),
                )])
                .default_value(&crate::examples::demo::ID),
            ])
            .view_action("setResultDisplay", LocalizedLabel::native("Set Result Display", "Ergebnisanzeige festlegen"))
            .action_args("setResultDisplay", crate::app_surface::result_display_action_args())
            // 🖱️ Viewport gesture vocabulary — the Canvas2d host emits these; picks become framework
            // `interactionSelect`/`interactionHover` requests, never a document mutation.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), semio_framework_plugin::ActionKind::View, "mouse-pointer-click") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegen"), semio_framework_plugin::ActionKind::View, "mouse-pointer") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), semio_framework_plugin::ActionKind::View, "mouse-pointer") })
            // 🩹️ Inspector field edits — one bounded edit per dispatch on one entity.
            .action_with(fem2d_patch_action("patchNode", LocalizedLabel::native("Patch Node", "Knoten ändern")))
            .action_with(fem2d_patch_action("patchElement", LocalizedLabel::native("Patch Element", "Element ändern")))
            .action_with(fem2d_patch_action("patchMaterial", LocalizedLabel::native("Patch Material", "Material ändern")))
            .action_with(fem2d_patch_action("patchSection", LocalizedLabel::native("Patch Section", "Querschnitt ändern")))
            .action_with(fem2d_patch_action("patchSupport", LocalizedLabel::native("Patch Support", "Lager ändern")))
            .action_with(fem2d_patch_action("patchRegion", LocalizedLabel::native("Patch Region", "Bereich ändern")))
            .action_with(fem2d_patch_action("patchLoad", LocalizedLabel::native("Patch Load", "Last ändern")))
            .action_with(fem2d_patch_action("patchLoadCase", LocalizedLabel::native("Patch Load Case", "Lastfall ändern")))
            .action_with(fem2d_patch_action("patchCombination", LocalizedLabel::native("Patch Combination", "Kombination ändern")))
            // ⏯️ Deformation playback — results-window view state, self re-armed through `DispatchAction`.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("setResultAnimation", LocalizedLabel::native("Set Result Animation", "Ergebnisanimation festlegen"), semio_framework_plugin::ActionKind::View, "play") })
            .action_args("setResultAnimation", vec![
                ActionArgDef::slider("phase", LocalizedLabel::native("Phase", "Phase"), 0.0, 1.0),
                ActionArgDef::toggle("playing", LocalizedLabel::native("Playing", "Läuft")),
                ActionArgDef::number("speed", LocalizedLabel::native("Speed", "Geschwindigkeit")),
                ActionArgDef::select("loopMode", LocalizedLabel::native("Loop", "Schleife"), vec![
                    ActionArgOption::new("loop", LocalizedLabel::native("Loop", "Schleife")),
                    ActionArgOption::new("pingPong", LocalizedLabel::native("Ping-Pong", "Ping-Pong")),
                    ActionArgOption::new("once", LocalizedLabel::native("Once", "Einmal")),
                ]),
                ActionArgDef::select("waveform", LocalizedLabel::native("Waveform", "Wellenform"), vec![ActionArgOption::new("ramp", LocalizedLabel::native("Ramp", "Rampe")), ActionArgOption::new("sine", LocalizedLabel::native("Sine", "Sinus"))]),
            ])
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("resultAnimationTick", LocalizedLabel::native("Result Animation Tick", "Ergebnisanimation Takt"), semio_framework_plugin::ActionKind::View, "timer") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("focusEntity", LocalizedLabel::native("Focus Entity", "Element fokussieren"), semio_framework_plugin::ActionKind::View, "focus") })
            .action_args("focusEntity", vec![ActionArgDef::text("id", LocalizedLabel::native("Entity", "Element")).required()])
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("addBar", InteractiveJobClassification::Migrated)
            .action_interactive_job("addBeam", InteractiveJobClassification::Migrated)
            .action_interactive_job("addMaterial", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSection", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSupport", InteractiveJobClassification::Migrated)
            .action_interactive_job("addNodalLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("addMemberUdl", InteractiveJobClassification::Migrated)
            .action_interactive_job("addAreaLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("addRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("addLoadCase", InteractiveJobClassification::Migrated)
            .action_interactive_job("addCombination", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelfWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAnalysisSettings", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setResultDisplay", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchElement", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchMaterial", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchSection", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchSupport", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchLoadCase", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchCombination", InteractiveJobClassification::Migrated)
            .action_interactive_job("setResultAnimation", InteractiveJobClassification::Migrated)
            .action_interactive_job("resultAnimationTick", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusEntity", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", InteractiveJobClassification::Migrated)
            // 🧰️ Canvas selection utilities — rectangle/lasso marquee and direct pick, shared with draw/note.
            .utility(fem2d_utility("selectDirect", LocalizedLabel::native("Direct Select", "Direktauswahl"), "mouse-pointer-2", UtilityCategory::Selection))
            .utility(fem2d_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", UtilityCategory::Selection))
            .utility(fem2d_utility("selectLasso", LocalizedLabel::native("Lasso Select", "Lasso-Auswahl"), "lasso", UtilityCategory::Selection))
            .utility(fem2d_utility("transformMove", LocalizedLabel::native("Pan", "Schwenken"), "move", UtilityCategory::Utilities))
            .utility(UtilityDefinition { group: Some("Transform".into()), category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("transform", LocalizedLabel::native("Transform", "Transformieren"), "move-3d") })
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .action_args("translateSelection", vec![
                ActionArgDef::number("dx", LocalizedLabel::native("Dx", "Dx")).required(),
                ActionArgDef::number("dy", LocalizedLabel::native("Dy", "Dy")).required(),
            ])
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .action_args("rotateSelection", vec![ActionArgDef::number("angle", LocalizedLabel::native("Angle", "Winkel")).required()])
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .action_args("scaleSelection", vec![
                ActionArgDef::number("sx", LocalizedLabel::native("Sx", "Sx")).required(),
                ActionArgDef::number("sy", LocalizedLabel::native("Sy", "Sy")).required(),
            ])
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transform-Griff festlegen"), semio_framework_plugin::ActionKind::View, "settings-2") })
            .action_args("setTransformGumballFlag", vec![
                ActionArgDef::text("flag", LocalizedLabel::native("Flag", "Flagge")).required(),
                ActionArgDef::toggle("pressed", LocalizedLabel::native("Pressed", "Gedrückt")),
            ])
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", InteractiveJobClassification::Migrated)
            .window_kind_utilities(model_window::WINDOW_KIND_ID, FEM2D_CANVAS_UTILITIES.iter().map(|id| (*id).into()).collect())
            .window_kind_utilities(results_window::WINDOW_KIND_ID, FEM2D_CANVAS_UTILITIES.iter().map(|id| (*id).into()).collect())
            // 🕹️ Framework-owned hover/selection: one domain over every document entity kind, bound to
            // both Canvas2d windows; interactionSelect/interactionHover/clearSelection/selectAll auto-inject.
            .interaction(fem2d_interaction_definition())
            .window_kind_interactions(model_window::WINDOW_KIND_ID, vec![InteractionRef::new(FEM2D_INTERACTION_DOMAIN)])
            .window_kind_interactions(results_window::WINDOW_KIND_ID, vec![InteractionRef::new(FEM2D_INTERACTION_DOMAIN)])
            .panel_tab_def(artifact_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .panel_tab_def(results_panel::definition())
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("delete", "removeSelection")
            .keybinding("backspace", "removeSelection")
            .keybinding("space", "setResultAnimation")
            // 🎯️ Typed channel surface — `config_spec()`/`fem2d_io()` are this same information's single
            // source of truth, reused here rather than duplicated.
            .config(Fem2dPlayApp::config_spec())
            .io(fem2d_io())
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
#[cfg(test)]
#[path = "🧪️tests/🪟️window-config-ownership/🦀️.rs"]
mod window_config_ownership;
//#endregion 🧪️UnitTests
